//! `temper check` — the workspace-load and diagnostic surface.
//!
//! Implements the loading half of the `check` gate (`specs/20-surface.md`, "CLI
//! surface" — the verb that validates against the active contract): reconstruct
//! the typed config surface into a [`Workspace`] IR, and carry the [`Diagnostic`]
//! value the generic engine emits. The clauses themselves are validated by the
//! generic engine over the closed algebra ([`crate::engine`], `specs/10-contracts.md`,
//! "The engine is generic; everything is an instance") — there is no per-rule code
//! here; the heuristic rule registry it replaced is retired ("Decision: kill the
//! heuristic rule registry").
//!
//! A [`Diagnostic`] is a value the engine *collects*, not a thrown error — one
//! `error`-severity finding drives `check`'s non-zero exit ([`any_error`]), and
//! [`render`] presents the set with `miette`. It is distinct from a
//! [`WorkspaceError`] (a hard failure that aborts the load).

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use miette::GraphicalReportHandler;

use crate::rule::{Rule as RuleArtifact, RuleError};
use crate::skill::{Skill, SkillError};

/// The loaded config surface: every built-in artifact `check` lints. Carries the
/// skills and rules; later built-in artifact kinds (hooks, agents, …) extend this
/// struct so a cross-artifact clause can reach the whole harness at once. Custom
/// project kinds (temper's own `spec`, ADRs, …) are not built-ins: they are read
/// generically through [`Unit::from_surface_dir`](crate::kind::Unit::from_surface_dir),
/// not materialized as a field here.
#[derive(Debug, Clone, PartialEq)]
pub struct Workspace {
    /// The skills reconstructed from `<workspace>/skills/<name>/`.
    pub skills: Vec<Skill>,
    /// The rules reconstructed from `<workspace>/rules/<name>/`.
    pub rules: Vec<RuleArtifact>,
}

impl Workspace {
    /// Load a workspace from its surface directory by reconstructing every skill
    /// under `<dir>/skills/*` via [`Skill::from_surface_dir`] and every rule under
    /// `<dir>/rules/*` via [`RuleArtifact::from_surface_dir`].
    ///
    /// A child is treated as an artifact surface only when it holds a `meta.toml`,
    /// so stray files and partial directories are skipped rather than erroring.
    /// Each kind is returned name-sorted (the directory listing order is
    /// unspecified) so the diagnostic set is stable across runs.
    pub fn load(dir: &Path) -> Result<Self, WorkspaceError> {
        let mut skills = Vec::new();
        for skill_dir in &surface_dirs(&dir.join("skills"))? {
            skills.push(Skill::from_surface_dir(skill_dir)?);
        }

        let mut rules = Vec::new();
        for rule_dir in &surface_dirs(&dir.join("rules"))? {
            rules.push(RuleArtifact::from_surface_dir(rule_dir)?);
        }

        Ok(Self { skills, rules })
    }
}

/// Enumerate the artifact surface directories under `root` — the immediate
/// children that hold a `meta.toml` — name-sorted for a stable load order. A
/// missing `root` yields an empty list (a workspace need not carry every kind).
fn surface_dirs(root: &Path) -> Result<Vec<PathBuf>, WorkspaceError> {
    if !root.is_dir() {
        return Ok(Vec::new());
    }

    let listing = fs::read_dir(root).map_err(|source| WorkspaceError::ReadDir {
        path: root.to_path_buf(),
        source,
    })?;
    let mut dirs = Vec::new();
    for entry in listing {
        let entry = entry.map_err(|source| WorkspaceError::ReadDir {
            path: root.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        if path.is_dir() && path.join("meta.toml").is_file() {
            dirs.push(path);
        }
    }
    dirs.sort();
    Ok(dirs)
}

/// Errors raised while loading a [`Workspace`]. A hard failure (the surface is
/// unreadable or malformed) — not a lint finding, which is a [`Diagnostic`].
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum WorkspaceError {
    /// A workspace artifact directory (`skills/`, `rules/`) could not be
    /// enumerated.
    #[error("failed to read workspace directory {path}")]
    #[diagnostic(code(temper::check::read_dir))]
    ReadDir {
        /// The directory whose listing failed.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// A skill surface under the workspace could not be reconstructed.
    #[error(transparent)]
    #[diagnostic(transparent)]
    Skill(#[from] SkillError),

    /// A rule surface under the workspace could not be reconstructed.
    #[error(transparent)]
    #[diagnostic(transparent)]
    Rule(#[from] RuleError),
}

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
        }
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
        Some(Box::new(format!("artifact: {}", self.artifact)))
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    /// A fresh, empty temp directory unique to this test run.
    fn tmpdir(label: &str) -> PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "author-check-{}-{}-{}",
            std::process::id(),
            id,
            label
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    const DEMO: &str = "---\n\
name: demo\n\
description: Use when demonstrating the lint engine load path.\n\
version: \"1.0.0\"\n\
---\n\
# Demo\n\
\n\
Body.\n";

    /// Write a one-skill surface (`meta.toml` + body) under `<ws>/skills/<name>/`,
    /// projecting it from a source `SKILL.md` exactly as `import` would.
    fn write_surface_skill(ws: &Path, name: &str, skill_md: &str) {
        let src = tmpdir(&format!("src-{name}"));
        fs::write(src.join("SKILL.md"), skill_md).unwrap();
        let skill = Skill::from_source_dir(&src).unwrap();

        let dir = ws.join("skills").join(name);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("meta.toml"), skill.to_meta_document().to_string()).unwrap();
        fs::write(dir.join("SKILL.md"), &skill.body).unwrap();
    }

    const RULE_SRC: &str = "---\n\
paths:\n\
  - \"src/**/*.rs\"\n\
---\n\
# Rust conventions\n\
\n\
Prefer a clone.\n";

    /// Write a one-rule surface (`meta.toml` + body) under `<ws>/rules/<name>/`,
    /// projecting it from a source rule file exactly as `import` would.
    fn write_surface_rule(ws: &Path, name: &str, rule_md: &str) {
        let src = tmpdir(&format!("rule-src-{name}"));
        let path = src.join(format!("{name}.md"));
        fs::write(&path, rule_md).unwrap();
        let rule = RuleArtifact::from_source_file(&path).unwrap();

        let dir = ws.join("rules").join(name);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("meta.toml"), rule.to_meta_document().to_string()).unwrap();
        fs::write(dir.join("RULE.md"), &rule.body).unwrap();
    }

    #[test]
    fn load_reconstructs_skills_from_a_written_surface() {
        let ws = tmpdir("load");
        write_surface_skill(&ws, "demo", DEMO);

        let loaded = Workspace::load(&ws).unwrap();

        assert_eq!(loaded.skills.len(), 1);
        assert_eq!(loaded.skills[0].name, "demo");
        assert_eq!(loaded.skills[0].version.as_deref(), Some("1.0.0"));
    }

    #[test]
    fn load_reconstructs_rules_sorted_alongside_skills() {
        let ws = tmpdir("load-rules");
        write_surface_skill(&ws, "demo", DEMO);
        write_surface_rule(&ws, "rust", RULE_SRC);
        write_surface_rule(&ws, "collaboration", "# Collaboration\n\nPushback.\n");

        let loaded = Workspace::load(&ws).unwrap();

        // Skills load as before, and rules load name-sorted beside them.
        assert_eq!(loaded.skills.len(), 1);
        let rule_names: Vec<&str> = loaded.rules.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(rule_names, vec!["collaboration", "rust"]);
        assert_eq!(
            loaded.rules[1].paths.as_deref(),
            Some(&["src/**/*.rs".to_string()][..])
        );
        // The no-frontmatter rule carries no `paths`.
        assert!(loaded.rules[0].paths.is_none());
    }

    #[test]
    fn load_skips_non_skill_dirs_and_returns_skills_sorted() {
        let ws = tmpdir("sorted");
        write_surface_skill(&ws, "bravo", DEMO);
        write_surface_skill(&ws, "alpha", DEMO);
        // Noise under skills/: a dir with no meta.toml must be skipped.
        fs::create_dir_all(ws.join("skills").join("empty")).unwrap();

        let loaded = Workspace::load(&ws).unwrap();
        let names: Vec<&str> = loaded.skills.iter().map(|s| s.name.as_str()).collect();
        // Both surfaces carry `name: demo` (from the shared fixture); what we
        // assert here is the count and that the stray dir was skipped.
        assert_eq!(names, vec!["demo", "demo"]);
    }

    #[test]
    fn load_of_a_workspace_without_skills_dir_is_empty() {
        let ws = tmpdir("nodir");
        let loaded = Workspace::load(&ws).unwrap();
        assert!(loaded.skills.is_empty());
    }

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
}
