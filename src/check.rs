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

/// The severity of a [`Diagnostic`]. Only `error` raises the process exit code;
/// `warn` is advisory. (The slice-1 rule table in the spec uses exactly these
/// two levels.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// A correctness/contract violation. Any `Error` makes `check` exit non-zero.
    Error,
    /// A best-practice advisory that does not fail the run.
    Warn,
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

impl miette::Diagnostic for Diagnostic {
    fn severity(&self) -> Option<miette::Severity> {
        Some(match self.severity {
            Severity::Error => miette::Severity::Error,
            Severity::Warn => miette::Severity::Warning,
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

/// The inputs that judged a run beyond the committed harness — the three uncommitted-or-
/// joined families, named so a verdict can never rest on something content review never
/// saw without saying so.
///
/// Assembled once by the gate and rendered by every reporter. Empty is the ordinary case:
/// a harness with no local member, no dial entry that reached a clause, and no joined
/// lock was judged by its committed lock alone, and there is nothing to announce.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Announcement {
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

/// The family label of an announced [`Announcement::local_members`] entry.
const LOCAL_MEMBER: &str = "local member";

/// The family label of an announced [`Announcement::dialed_clauses`] entry.
const DIALED_CLAUSE: &str = "dialed clause";

/// The family label of an announced [`Announcement::joined_locks`] entry.
const JOINED_LOCK: &str = "joined lock";

/// The sentence that leads a rendered announcement.
const ANNOUNCEMENT_HEADING: &str = "judged by inputs the committed harness does not carry:";

impl Announcement {
    /// Whether nothing was announced — the run was judged by the committed harness alone.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.local_members.is_empty()
            && self.dialed_clauses.is_empty()
            && self.joined_locks.is_empty()
    }

    /// Every announced input as a `(family, name)` pair, in layer-stack order: the members
    /// this machine holds, the clauses its dial re-weighed, then the locks the invocation
    /// joined on top. The one vocabulary every reporter names these inputs by — each
    /// reporter chooses the envelope, never the words.
    #[must_use]
    pub fn entries(&self) -> Vec<(&'static str, &str)> {
        [
            (LOCAL_MEMBER, &self.local_members),
            (DIALED_CLAUSE, &self.dialed_clauses),
            (JOINED_LOCK, &self.joined_locks),
        ]
        .into_iter()
        .flat_map(|(family, names)| names.iter().map(move |name| (family, name.as_str())))
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
/// non-zero process exit. Warn-only runs return `false`.
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
            local_members: vec!["dial:workstation".to_string()],
            dialed_clauses: vec!["skill.extent".to_string()],
            joined_locks: vec!["/org/lock.toml".to_string()],
        };
        let rendered = render(&[], &announcement);

        assert!(rendered.starts_with(ANNOUNCEMENT_HEADING));
        assert!(rendered.contains("local member: dial:workstation"));
        assert!(rendered.contains("dialed clause: skill.extent"));
        assert!(rendered.contains("joined lock: /org/lock.toml"));
    }

    #[test]
    fn an_announcement_is_empty_only_when_every_family_is() {
        assert!(Announcement::default().is_empty());
        for announcement in [
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
        }
    }
}
