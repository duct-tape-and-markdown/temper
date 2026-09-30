//! `temper check` — the diagnostic surface.
//!
//! Carries the [`Diagnostic`] value the generic engine emits. The clauses
//! themselves are validated by the generic engine over the closed algebra —
//! there is no per-rule code here.
//!
//! A [`Diagnostic`] is a value the engine *collects*, not a thrown error — one
//! `error`-severity finding drives `check`'s non-zero exit ([`any_error`]), and
//! [`render`] presents the set with `miette`.

use std::fmt;

use miette::GraphicalReportHandler;

/// The severity of a [`Diagnostic`] — the *reported* level, distinct from the
/// author-declared [`contract::Severity`](crate::contract::Severity) a clause carries
/// (`required`/`advisory`, mapped by [`engine::severity_of`](crate::engine::severity_of)).
///
/// `Error` and `Warn` are the two levels a violation is reported at: `Error` raises the
/// process exit code, `Warn` is the advisory a corpus can escalate with
/// `--deny-advisories`. `Note` is the third and is not a violation at all — it is
/// **disclosure**, what the gate checked, which no contract declares and no dial can
/// tune, so nothing promotes it to blocking.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// A correctness/contract violation. Any `Error` makes `check` exit non-zero.
    Error,
    /// A best-practice advisory that does not fail the run on its own. Escalated to
    /// blocking by `--deny-advisories` — it is a violation, so a corpus may demand it.
    Warn,
    /// A disclosure note: a statement of what was checked, never a violation. Every
    /// reporter prints it (invariant 6 — the gate's silence must never read as
    /// "checked"), and no flag promotes it to blocking, because there is no contract
    /// clause behind it to escalate.
    Note,
}

/// A single lint finding: which rule fired, on which artifact, with what message.
/// A finding the engine collects — never a thrown error.
///
/// It implements [`miette::Diagnostic`] so it renders through the same graphical
/// handler as the crate's hard errors: [`Severity`] maps to miette's severity,
/// [`Diagnostic::rule`] becomes the diagnostic `code`, and [`Diagnostic::artifact`]
/// surfaces as the help line.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error("{message}")]
pub struct Diagnostic {
    /// Whether this finding fails the run or is merely advisory.
    pub severity: Severity,
    /// The rule id — stable, the diagnostic `code`, and the one name this finding is
    /// addressed by.
    ///
    /// A finding a clause produced prints that clause's **address**: the label emit
    /// stamped on its lock row, dot-joined `<owner>.<predicate>[.<field>]` — the kind
    /// whose contract carries it (or `requirement.<name>` for a requirement's own
    /// demand), the predicate key, and the field it names when it names one:
    /// `skill.max_len.description`, `requirement.approved-model.count`. It is the whole
    /// identity — there is no second, friendlier name beside it — so an author who reads
    /// a finding can spell the clause back to the dial without a lookup, and the
    /// clause's own guidance is what teaches the *why*.
    ///
    /// A finding no clause produced — well-formedness, the graph's fixed checks — reports
    /// under its own `<area>.<check>` id (`kind.governs-collision`, `graph.acyclic`)
    /// instead: those are preconditions of judging, never dialable, so they have no
    /// clause to be addressed by.
    pub rule: String,
    /// The artifact the finding is about, e.g. the skill name or its path.
    pub artifact: String,
    /// The human-readable finding, the diagnostic's `Display`.
    pub message: String,
    /// The **colocated guidance** of the clause that produced this finding, if it
    /// carried any: the hover-sized *why*,
    /// delivered just-in-time on the violation — the failure is the teaching
    /// moment. Advisory-only prose that never gates (it played no part in deciding
    /// this finding, only in explaining it), surfaced on the rendered help line
    /// below the artifact. `None` when the clause carried no guidance.
    pub guidance: Option<String>,
}

impl Diagnostic {
    /// An `error`-severity finding.
    pub fn error(
        rule: impl Into<String>,
        artifact: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(Severity::Error, rule, artifact, message)
    }

    /// A `warn`-severity finding.
    pub fn warn(
        rule: impl Into<String>,
        artifact: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(Severity::Warn, rule, artifact, message)
    }

    /// A `note`-severity disclosure — what the gate checked, never a violation, so
    /// `--deny-advisories` never promotes it.
    pub fn note(
        rule: impl Into<String>,
        artifact: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(Severity::Note, rule, artifact, message)
    }

    /// A finding at an explicit [`Severity`].
    pub fn new(
        severity: Severity,
        rule: impl Into<String>,
        artifact: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            rule: rule.into(),
            artifact: artifact.into(),
            message: message.into(),
            guidance: None,
        }
    }

    /// A finding **a clause produced**: the one home that reads all three of a
    /// [`Clause`](crate::contract::Clause)'s channels together — the author's declared
    /// [`severity`](crate::contract::Clause::severity) (through
    /// [`engine::severity_of`](crate::engine::severity_of)), the
    /// [`label`](crate::contract::Clause::label) the finding reports under, and the
    /// [`guidance`](crate::contract::Clause::guidance) the gate teaches through at the
    /// moment of failure. A judge hands the clause, the indicted artifact and the
    /// message; it can no more thread two channels of three than it can invent a
    /// fourth.
    ///
    /// [`with_guidance`](Self::with_guidance) stays public beside it: a finding that
    /// carries guidance from somewhere other than its filing clause — the `when` body
    /// whose own prose wins over the enclosing guard's
    /// ([`engine`](crate::engine)) — still needs the builder.
    #[must_use]
    pub fn from_clause(
        clause: &crate::contract::Clause,
        artifact: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::new(
            crate::engine::severity_of(clause.severity),
            &clause.label,
            artifact,
            message,
        )
        .with_guidance(clause.guidance.clone())
    }

    /// Attach a clause's [`guidance`](crate::contract::Clause::guidance) to this
    /// finding — the just-in-time delivery of the hover-sized *why* on the
    /// violation. A builder so the base
    /// constructors stay guidance-free (most findings carry none); a `None`
    /// argument is a no-op, leaving the finding unguided.
    #[must_use]
    pub fn with_guidance(mut self, guidance: Option<String>) -> Self {
        self.guidance = guidance;
        self
    }
}

/// The rule a lowered load fault is addressed by when the failure it lowers carries no
/// `miette` code of its own — every other one keeps the code it already has.
const LOAD_FAULT_RULE: &str = "gate.load-fault";

/// The sentence a lowered load fault leads with: what the run did *not* do, before the
/// detail of why.
const LOAD_FAULT_MESSAGE: &str =
    "the harness could not be loaded, so the contract gate did not run";

/// The sentence a lowered load fault **leads** with when the lock names an engine other
/// than the one running — the one place the engine-skew fact can be said over a harness
/// that never loaded, because the
/// [`engine-matches`](crate::contract::Predicate::EngineMatches) clause that ordinarily
/// says it is judged inside a gate this run never reached (`specs/model/pipeline.md`,
/// "The lock": a gate run by a different engine says so).
///
/// Names **both** stamps and no ordering between them: nothing in the sanctioned crate
/// set does semver, so the sentence never claims which is newer — honest whichever
/// direction the skew runs. The remedy differs from the clause's for a reason: where the
/// gate ran, the rows read fine and one `emit` rewrites the lock in this engine's
/// canonical form, but here *this* engine could not read them at all, so the only engine
/// known to be able to is the one the lock names.
fn engine_skew_lead(lock_engine: &str) -> String {
    format!(
        "the committed lock was written by engine version {lock_engine}, and this gate is \
         running engine version {} — read it with the engine it names",
        crate::VERSION
    )
}

/// Lower a load failure — the [`miette::Report`] raised while resolving the harness root
/// or gating it — to the single `error` [`Diagnostic`] the run reports instead of
/// aborting on.
///
/// Aborting is harmless for a hard placement, which exits non-zero and renders the error
/// either way. It is not harmless for the advisory session-start placement, whose stdout
/// payload is the only thing the session ever sees: a run that never reaches a reporter
/// prints nothing, and an empty payload is indistinguishable from a pass — the one
/// outcome `specs/distribution.md` ("The placements and their enforcement modes") forbids
/// it, the gate that "never silently passes". Lowered here, every reporter renders the
/// fault, and the exit-code verdict is unchanged: one `error` diagnostic keeps
/// [`any_error`] true.
///
/// Three mechanics, none cosmetic:
///
/// - The **rule** is the report's own [`code`](miette::Diagnostic::code) when it carries
///   one, so a load fault keeps the single name it is already addressed by
///   (`temper::toml_document::malformed`) rather than growing a second one, and falls
///   back to `gate.load-fault` only for a report that declares none.
/// - The **message** renders the whole [`chain`](miette::Report::chain), not the bare
///   `Display`. The crate's error vocabulary puts its detail in `#[source]` fields —
///   `{path} is not valid UTF-8` names the file and nothing about the decode — and
///   `Display` alone drops every one of them.
/// - `lock_engine` is the committed lock's engine stamp as the caller read it, and where
///   it disagrees with [`crate::VERSION`] the message leads with
///   [`engine_skew_lead`]: a rejection of rows a *different* compiler wrote is a
///   different diagnosis from a rejection of this engine's own, and the author cannot
///   tell them apart from a closed-vocabulary complaint alone. `None` is **unknown**, never
///   a verdict — an absent lock, a stamp-less older lock, and a workspace that would not
///   resolve all reach it, and each keeps the bare detail chain, exactly as the
///   `engine-matches` clause stays silent on absent evidence. A same-stamp lock is the
///   ordinary case and reads identically to `None`.
///
/// The peer of `compose::frontmatter_fault_diagnostic`, which lowers three *named*
/// frontmatter faults to `member.load-fault` and re-raises the rest. That one is a
/// read-site judgement about which faults a discovery walk absorbs, so it lives beside
/// the read that raises them. This one absorbs whatever the run raises, from wherever it
/// was raised, and its whole purpose is to have a reporter to hand it to — so it lives
/// here, beside [`Diagnostic`] and the reporters that render it.
#[must_use]
pub fn load_fault(
    report: &miette::Report,
    artifact: impl Into<String>,
    lock_engine: Option<&str>,
) -> Diagnostic {
    let rule = report
        .code()
        .map_or_else(|| LOAD_FAULT_RULE.to_string(), |code| code.to_string());
    let detail: Vec<String> = report.chain().map(ToString::to_string).collect();
    let fault = format!("{LOAD_FAULT_MESSAGE}: {}", detail.join(": "));
    let message = match lock_engine.filter(|stamp| *stamp != crate::VERSION) {
        Some(stamp) => format!("{}: {fault}", engine_skew_lead(stamp)),
        None => fault,
    };
    Diagnostic::error(rule, artifact, message)
}

impl miette::Diagnostic for Diagnostic {
    fn severity(&self) -> Option<miette::Severity> {
        Some(match self.severity {
            Severity::Error => miette::Severity::Error,
            Severity::Warn => miette::Severity::Warning,
            Severity::Note => miette::Severity::Advice,
        })
    }

    fn code(&self) -> Option<Box<dyn fmt::Display + '_>> {
        Some(Box::new(self.rule.clone()))
    }

    fn help(&self) -> Option<Box<dyn fmt::Display + '_>> {
        // The colocated guidance rides the help line beneath the artifact — the
        // violation is the teaching moment.
        Some(Box::new(match &self.guidance {
            Some(guidance) => format!("artifact: {}\n{guidance}", self.artifact),
            None => format!("artifact: {}", self.artifact),
        }))
    }
}

/// What judged a run beyond the committed harness the invocation named — the three
/// uncommitted-or-joined families, plus the harness root when that is not the path the
/// invocation gave. Named so a verdict can never rest on something content review never
/// saw, nor on a tree other than the one it was asked about, without saying so.
///
/// Assembled once by the gate and rendered by every reporter. Empty is the ordinary case:
/// a harness with no local member, no dial entry that reached a clause, and no joined
/// lock, addressed by the root it was given, was judged by its committed lock alone, and
/// there is nothing to announce.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Announcement {
    /// The harness root the run resolved, when that is **not** the path the invocation
    /// gave — a workspace argument gates against the root enclosing it, so the verdict
    /// came from a tree the argument never spelled. `None` is the ordinary case: the
    /// path given *is* the root read, and a root nobody re-rooted is nothing to say.
    pub harness_root: Option<String>,
    /// Every active local member, by the `<kind>:<id>` address its findings name it by.
    pub local_members: Vec<String>,
    /// Every clause the dial re-weighed, by the address the dial entry spelled. An entry
    /// that reached no clause is a dial refusal, not an announcement — nothing was judged
    /// through it.
    pub dialed_clauses: Vec<String>,
    /// Every lock this invocation joined, as it was spelled at `--layer`. One entry per
    /// lock, whatever number of clauses it carried: the lock is what was joined.
    pub joined_locks: Vec<String>,
}

/// The family label of the announced [`Announcement::harness_root`].
const HARNESS_ROOT: &str = "harness root";

/// The family label of an announced [`Announcement::local_members`] entry.
const LOCAL_MEMBER: &str = "local member";

/// The family label of an announced [`Announcement::dialed_clauses`] entry.
const DIALED_CLAUSE: &str = "dialed clause";

/// The family label of an announced [`Announcement::joined_locks`] entry.
const JOINED_LOCK: &str = "joined lock";

/// The sentence that leads a rendered announcement.
const ANNOUNCEMENT_HEADING: &str =
    "judged this run by more than the committed harness the invocation named:";

impl Announcement {
    /// Whether nothing was announced — the run was judged by the committed harness alone.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.harness_root.is_none()
            && self.local_members.is_empty()
            && self.dialed_clauses.is_empty()
            && self.joined_locks.is_empty()
    }

    /// Every announcement as a `(family, name)` pair: the resolved root first — not a
    /// layer, but the tree every line under it was read from — then layer-stack order,
    /// the members this machine holds, the clauses its dial re-weighed, and the locks the
    /// invocation joined on top. The one vocabulary every reporter names these by — each
    /// reporter chooses the envelope, never the words.
    #[must_use]
    pub fn entries(&self) -> Vec<(&'static str, &str)> {
        self.harness_root
            .iter()
            .map(|root| (HARNESS_ROOT, root.as_str()))
            .chain(
                [
                    (LOCAL_MEMBER, &self.local_members),
                    (DIALED_CLAUSE, &self.dialed_clauses),
                    (JOINED_LOCK, &self.joined_locks),
                ]
                .into_iter()
                .flat_map(|(family, names)| names.iter().map(move |name| (family, name.as_str()))),
            )
            .collect()
    }

    /// The plain-text block: [`ANNOUNCEMENT_HEADING`], then one indented `<family>: <name>`
    /// line per input. The empty string when there is nothing to announce, so a caller
    /// concatenates it unconditionally.
    #[must_use]
    pub fn render(&self) -> String {
        if self.is_empty() {
            return String::new();
        }
        let mut out = format!("{ANNOUNCEMENT_HEADING}\n");
        for (family, name) in self.entries() {
            out.push_str(&format!("  {family}: {name}\n"));
        }
        out
    }
}

/// Render diagnostics for the terminal with miette's graphical handler — the same
/// presentation the crate's hard errors use, led by the [`Announcement`] so what judged
/// the run is read before its findings are.
pub fn render(diagnostics: &[Diagnostic], announcement: &Announcement) -> String {
    let handler = GraphicalReportHandler::new();
    let mut out = announcement.render();
    if !out.is_empty() {
        out.push('\n');
    }
    for diagnostic in diagnostics {
        // Writing to a `String` never fails; fall back to the bare message if a
        // future handler ever does, so a render hiccup can't swallow a finding.
        if handler
            .render_report(&mut out, diagnostic as &dyn miette::Diagnostic)
            .is_err()
        {
            out.push_str(&diagnostic.message);
        }
        out.push('\n');
    }
    out
}

/// Whether any diagnostic is `error` severity — the signal that drives `check`'s
/// non-zero process exit. Warn-only and note-only runs return `false`.
pub fn any_error(diagnostics: &[Diagnostic]) -> bool {
    diagnostics
        .iter()
        .any(|diagnostic| diagnostic.severity == Severity::Error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_error_is_true_for_an_error_and_false_for_warn_only() {
        let error = Diagnostic::error("skill.name-format", "demo", "name has uppercase");
        let warn = Diagnostic::warn("skill.body-length", "demo", "body is long");

        assert!(any_error(std::slice::from_ref(&error)));
        assert!(!any_error(std::slice::from_ref(&warn)));
        // A warn alongside an error still fails the run.
        assert!(any_error(&[warn, error]));
    }

    #[test]
    fn a_load_fault_leads_with_the_skew_only_when_the_stamps_disagree() {
        let report = miette::miette!("lock clause row names predicate `not_a_predicate`");
        let bare = load_fault(&report, "/repo", None);
        // A same-stamp lock is the ordinary case, and reads identically to unknown: the
        // lead is the whole difference between the two messages, never a reshuffle of
        // the detail chain.
        assert_eq!(load_fault(&report, "/repo", Some(crate::VERSION)), bare);
        assert!(bare.message.starts_with(LOAD_FAULT_MESSAGE));

        let skewed = load_fault(&report, "/repo", Some("0.0.0-elsewhere"));
        // Both stamps, and the detail the bare fault already carried underneath.
        assert!(skewed.message.contains("0.0.0-elsewhere"));
        assert!(skewed.message.contains(crate::VERSION));
        assert!(skewed.message.ends_with(&bare.message));
        // One diagnostic either way, under the same rule and severity: the skew is a
        // diagnosis the fault leads with, never a second finding.
        assert_eq!((skewed.rule, skewed.severity), (bare.rule, bare.severity));
    }

    #[test]
    fn render_surfaces_the_rule_code_and_message() {
        let diagnostic = Diagnostic::error("skill.name-format", "demo", "name has uppercase");
        let rendered = render(std::slice::from_ref(&diagnostic), &Announcement::default());

        assert!(rendered.contains("skill.name-format"));
        assert!(rendered.contains("name has uppercase"));
        // The artifact rides along on the help line.
        assert!(rendered.contains("demo"));
        // Nothing beyond the committed harness judged this run, so the render says
        // nothing extra.
        assert!(!rendered.contains(ANNOUNCEMENT_HEADING));
    }

    #[test]
    fn render_leads_with_the_announced_inputs() {
        let announcement = Announcement {
            harness_root: Some("/repo".to_string()),
            local_members: vec!["dial:workstation".to_string()],
            dialed_clauses: vec!["skill.extent".to_string()],
            joined_locks: vec!["/org/lock.toml".to_string()],
        };
        let rendered = render(&[], &announcement);

        assert!(rendered.starts_with(ANNOUNCEMENT_HEADING));
        // The root leads: every line under it was read from that tree.
        assert!(
            rendered
                .lines()
                .nth(1)
                .is_some_and(|line| line.trim() == "harness root: /repo")
        );
        assert!(rendered.contains("local member: dial:workstation"));
        assert!(rendered.contains("dialed clause: skill.extent"));
        assert!(rendered.contains("joined lock: /org/lock.toml"));
    }

    #[test]
    fn an_announcement_is_empty_only_when_every_family_is() {
        assert!(Announcement::default().is_empty());
        for announcement in [
            // A re-rooted run announces that and nothing else, and still renders: the
            // root is a family of its own, not a decoration on the other three.
            Announcement {
                harness_root: Some("/repo".to_string()),
                ..Default::default()
            },
            Announcement {
                local_members: vec!["dial:workstation".to_string()],
                ..Default::default()
            },
            Announcement {
                dialed_clauses: vec!["skill.extent".to_string()],
                ..Default::default()
            },
            Announcement {
                joined_locks: vec!["/org/lock.toml".to_string()],
                ..Default::default()
            },
        ] {
            assert!(!announcement.is_empty());
            assert_eq!(announcement.entries().len(), 1);
            assert!(announcement.render().starts_with(ANNOUNCEMENT_HEADING));
        }
    }
}
