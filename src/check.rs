//! `temper check` — the diagnostic surface.
//!
//! Carries the [`Diagnostic`] value the generic engine emits. The clauses
//! themselves are validated by the generic engine over the closed algebra —
//! there is no per-rule code here; the heuristic rule registry it replaced is
//! retired (kill the heuristic rule registry).
//!
//! A [`Diagnostic`] is a value the engine *collects*, not a thrown error — one
//! `error`-severity finding drives `check`'s non-zero exit ([`any_error`]), and
//! [`render`] presents the set with `miette`.

use std::fmt;
use std::path::Path;

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
    /// The rule id, e.g. `skill.name-format` — stable, the diagnostic `code`.
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

/// Render diagnostics for the terminal with miette's graphical handler — the same
/// presentation the crate's hard errors use.
pub fn render(diagnostics: &[Diagnostic]) -> String {
    let handler = GraphicalReportHandler::new();
    let mut out = String::new();
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

/// The rule id for [`empty_assembly_incoherence`]'s fail-loud tripwire.
pub const EMPTY_ASSEMBLY_RULE: &str = "coverage.empty-assembly";

/// Fail loud when the committed assembly declares requirements but the gate
/// resolved none of them and the lock carries no declaration rows either — the
/// silent "checked 0 members … exit 0" class the wave-end confirmation caught.
/// Takes
/// primitives rather than `Declarations` so the predicate stays a pure, unit-testable
/// tripwire: `declared` is whether the lock declares ≥1 `[requirement.*]`;
/// `resolved_members` is the sum of every built-in kind's checked-member count;
/// `declarations_empty` is whether the lock's declaration-row family carries nothing.
///
/// A workspace that resolves ≥1 member never fires, and neither does a genuinely empty harness
/// (`declared` false) — zero members is legitimate there.
#[must_use]
pub fn empty_assembly_incoherence(
    root: &Path,
    declared: bool,
    resolved_members: usize,
    declarations_empty: bool,
) -> Option<Diagnostic> {
    if declared && resolved_members == 0 && declarations_empty {
        let root = root.display();
        Some(Diagnostic::error(
            EMPTY_ASSEMBLY_RULE,
            root.to_string(),
            format!(
                "{root} declares members/requirements but the gate resolved none of them, \
                 and the lock carries no declaration rows — likely a stale or absent \
                 lock.toml, or a mis-rooted workspace (the harness root vs `./.temper`, or \
                 `emit` not run)"
            ),
        ))
    } else {
        None
    }
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
        let rendered = render(std::slice::from_ref(&diagnostic));

        assert!(rendered.contains("skill.name-format"));
        assert!(rendered.contains("name has uppercase"));
        // The artifact rides along on the help line.
        assert!(rendered.contains("demo"));
    }

    #[test]
    fn empty_assembly_incoherence_fires_when_declared_but_nothing_resolved() {
        let root = Path::new("/harness/root");
        let diagnostic = empty_assembly_incoherence(root, true, 0, true).unwrap();

        assert_eq!(diagnostic.severity, Severity::Error);
        assert_eq!(diagnostic.rule, EMPTY_ASSEMBLY_RULE);
        assert!(diagnostic.message.contains("/harness/root"));
    }

    #[test]
    fn empty_assembly_incoherence_stays_silent_when_members_resolved() {
        // A correctly-rooted workspace that resolves at least one member never fires,
        // even though the lock carries no declaration rows (e.g. the lock predates the
        // declaration recut): no false block on a clean gate.
        let root = Path::new("/harness/root");
        assert!(empty_assembly_incoherence(root, true, 1, true).is_none());
    }

    #[test]
    fn empty_assembly_incoherence_stays_silent_when_lock_carries_declarations() {
        // Declared and zero resolved, but the lock's declaration rows are non-empty:
        // not the silent-skip class this guards against.
        let root = Path::new("/harness/root");
        assert!(empty_assembly_incoherence(root, true, 0, false).is_none());
    }

    #[test]
    fn empty_assembly_incoherence_stays_silent_on_a_genuinely_empty_harness() {
        // The lock declares nothing: `declared` is false, so zero resolved
        // members is legitimate and the guard never fires.
        let root = Path::new("/harness/root");
        assert!(empty_assembly_incoherence(root, false, 0, true).is_none());
    }
}
