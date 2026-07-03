//! `temper check` — the workspace-load and diagnostic surface.
//!
//! Implements the loading half of the `check` gate (`specs/architecture/20-surface.md`, "CLI
//! surface" — the verb that validates against the active contract): reconstruct
//! the typed config surface into a [`Workspace`] IR, and carry the [`Diagnostic`]
//! value the generic engine emits. The clauses themselves are validated by the
//! generic engine over the closed algebra ([`crate::engine`], `specs/architecture/10-contracts.md`,
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

use crate::frontmatter::{FrontmatterError, Member};
use crate::kind::{KindError, Unit};

/// The loaded config surface: every built-in artifact `check` lints. Carries the
/// skills and rules as generic frontmatter [`Member`]s (`specs/architecture/15-kinds.md`, "A
/// built-in kind is an adapter"); later built-in artifact kinds (hooks, agents, …)
/// extend this struct so a cross-artifact clause can reach the whole harness at once.
/// Custom project kinds (temper's own `spec`, ADRs, …) are not built-ins: they are
/// read generically through [`Unit::from_surface_dir`](crate::kind::Unit::from_surface_dir),
/// not materialized as a field here.
#[derive(Debug, Clone, PartialEq)]
pub struct Workspace {
    /// The skill members reconstructed from `<workspace>/skills/<name>/`.
    pub skills: Vec<Member>,
    /// The rule members reconstructed from `<workspace>/rules/<name>/`.
    pub rules: Vec<Member>,
}

impl Workspace {
    /// Load a workspace from its surface directory by reconstructing every skill
    /// under `<dir>/skills/*` and every rule under `<dir>/rules/*` through the one
    /// generic frontmatter adapter ([`Member::from_surface`]) — no per-kind IR.
    ///
    /// A child is treated as an artifact surface only when it holds its kind's member
    /// document (`SKILL.md`, `RULE.md`), so stray files and partial directories are
    /// skipped rather than erroring. Each kind is returned name-sorted (the directory
    /// listing order is unspecified) so the diagnostic set is stable across runs.
    pub fn load(dir: &Path) -> Result<Self, WorkspaceError> {
        let mut skills = Vec::new();
        for skill_dir in &surface_dirs(&dir.join("skills"), "SKILL.md")? {
            skills.push(Member::from_surface(skill_dir, "SKILL.md")?);
        }

        let mut rules = Vec::new();
        for rule_dir in &surface_dirs(&dir.join("rules"), "RULE.md")? {
            rules.push(Member::from_surface(rule_dir, "RULE.md")?);
        }

        Ok(Self { skills, rules })
    }
}

/// Load a built-in kind's surface members as generic [`Unit`]s — the read the gate's
/// feature extraction ranges over, the built-in counterpart to `main::custom_units`.
/// Each member directory under `<dir>/<subdir>/*` that holds its kind's member
/// document `member_doc` (`SKILL.md`, `RULE.md`) is reloaded through the same
/// [`Unit::from_member_document`] a custom kind's [`Unit::from_surface_dir`] uses — so
/// built-in and custom kinds read the surface through **one loader**, with no IR→Unit
/// adapter on the check path (`specs/architecture/15-kinds.md`, "A built-in kind is an adapter").
///
/// The member document is targeted by the built-in's own name, not the lone-`.md`
/// heuristic, so a skill's markdown companion (a `PLAYBOOK.md`) never confuses the
/// read. Name-sorted for a stable diagnostic set; a missing `subdir` yields an empty
/// list. The typed [`Workspace`] survives for the adapter faces (drift/bundle/apply);
/// this is the check read alone.
///
/// # Errors
///
/// Returns a [`WorkspaceError`] if a surface directory cannot be enumerated, or a
/// member document is unreadable or malformed.
pub fn surface_units(
    dir: &Path,
    subdir: &str,
    member_doc: &str,
) -> Result<Vec<Unit>, WorkspaceError> {
    let mut units = Vec::new();
    for member_dir in surface_dirs(&dir.join(subdir), member_doc)? {
        let doc_path = member_dir.join(member_doc);
        units.push(Unit::from_member_document(&member_dir, &doc_path)?);
    }
    Ok(units)
}

/// Enumerate the artifact surface directories under `root` — the immediate children
/// that hold their kind's member document `doc` (`SKILL.md`, `RULE.md`) — name-sorted
/// for a stable load order. A missing `root` yields an empty list (a workspace need
/// not carry every kind).
fn surface_dirs(root: &Path, doc: &str) -> Result<Vec<PathBuf>, WorkspaceError> {
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
        if path.is_dir() && path.join(doc).is_file() {
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

    /// A built-in kind's surface member could not be reconstructed through the
    /// generic frontmatter adapter (`specs/architecture/15-kinds.md`).
    #[error(transparent)]
    #[diagnostic(transparent)]
    Frontmatter(#[from] FrontmatterError),

    /// A built-in kind's surface member document could not be read generically
    /// (`specs/architecture/15-kinds.md`, "A built-in kind is an adapter") — the check read's
    /// hard failure, distinct from the typed-IR reconstruction faces above.
    #[error(transparent)]
    #[diagnostic(transparent)]
    Kind(#[from] KindError),
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
    /// The **colocated guidance** of the clause that produced this finding, if it
    /// carried any (`specs/architecture/10-contracts.md`, "Packages"): the hover-sized *why*,
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
    /// violation (`specs/architecture/10-contracts.md`, "Packages"). A builder so the base
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
        // violation is the teaching moment (`specs/architecture/10-contracts.md`, "Packages").
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

    /// The embedded built-in kind definition the generic frontmatter adapter is
    /// driven by in these tests.
    fn builtin(name: &str) -> crate::kind::CustomKind {
        crate::builtin_kind::definition(name).unwrap().unwrap()
    }

    /// Write a one-skill surface (member document + body) under `<ws>/skills/<name>/`,
    /// projecting it from a source `SKILL.md` exactly as `import` would.
    fn write_surface_skill(ws: &Path, name: &str, skill_md: &str) {
        let src = tmpdir(&format!("src-{name}"));
        fs::write(src.join("SKILL.md"), skill_md).unwrap();
        let member = Member::from_source(&builtin("skill"), &src.join("SKILL.md")).unwrap();

        let dir = ws.join("skills").join(name);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("SKILL.md"), member.to_document().emit()).unwrap();
    }

    const RULE_SRC: &str = "---\n\
paths:\n\
  - \"src/**/*.rs\"\n\
---\n\
# Rust conventions\n\
\n\
Prefer a clone.\n";

    /// Write a one-rule surface (member document + body) under `<ws>/rules/<name>/`,
    /// projecting it from a source rule file exactly as `import` would.
    fn write_surface_rule(ws: &Path, name: &str, rule_md: &str) {
        let src = tmpdir(&format!("rule-src-{name}"));
        let path = src.join(format!("{name}.md"));
        fs::write(&path, rule_md).unwrap();
        let member = Member::from_source(&builtin("rule"), &path).unwrap();

        let dir = ws.join("rules").join(name);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("RULE.md"), member.to_document().emit()).unwrap();
    }

    #[test]
    fn load_reconstructs_skills_from_a_written_surface() {
        let ws = tmpdir("load");
        write_surface_skill(&ws, "demo", DEMO);

        let loaded = Workspace::load(&ws).unwrap();

        assert_eq!(loaded.skills.len(), 1);
        assert_eq!(loaded.skills[0].id, "demo");
        assert_eq!(
            loaded.skills[0].field("version").and_then(|v| v.as_str()),
            Some("1.0.0")
        );
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
        let rule_names: Vec<&str> = loaded.rules.iter().map(|r| r.id.as_str()).collect();
        assert_eq!(rule_names, vec!["collaboration", "rust"]);
        assert_eq!(
            loaded.rules[1].field("paths"),
            Some(&serde_json::json!(["src/**/*.rs"]))
        );
        // The no-frontmatter rule carries no `paths`.
        assert!(!loaded.rules[0].has_field("paths"));
    }

    #[test]
    fn surface_units_yield_the_same_features_the_ir_adapter_produced() {
        use crate::builtin_kind;
        use crate::extract::{self, FeatureValue, Kind};

        let ws = tmpdir("surface-units");
        // A skill carrying an unknown key beside the typed fields, and a `paths` rule.
        let skill_md = "---\n\
name: demo\n\
description: Use when demonstrating the generic surface read.\n\
allowed-tools: [\"Bash\", \"Read\"]\n\
---\n\
# Demo\n\
\n\
Body.\n";
        write_surface_skill(&ws, "demo", skill_md);
        write_surface_rule(&ws, "rust", RULE_SRC);

        // The typed reconstruction (the adapter faces still use it) beside the generic
        // surface read the gate now uses — one loader, `Unit::from_member_document`.
        let typed = Workspace::load(&ws).unwrap();
        let skill_units = surface_units(&ws, "skills", "SKILL.md").unwrap();
        let rule_units = surface_units(&ws, "rules", "RULE.md").unwrap();

        let skill_features = builtin_kind::skill_features(&skill_units[0]).unwrap();
        // The id, the typed `name` field, and the unknown key all land exactly as the
        // retired IR→Unit adapter produced — the documented fields off the composed
        // `field` primitives, the unknown key folded in permissively.
        assert_eq!(skill_features.id, typed.skills[0].id);
        assert_eq!(
            skill_features.field("name"),
            Some(&FeatureValue::scalar(Kind::String, "demo"))
        );
        assert_eq!(
            skill_features.field("allowed-tools"),
            Some(&FeatureValue::List(vec![
                "Bash".to_string(),
                "Read".to_string()
            ]))
        );
        // `placement` carries through the same provenance the typed IR reloaded, so the
        // generic read's source directory equals the typed skill's.
        assert_eq!(
            skill_features.source_dir,
            extract::source_dir_name(&typed.skills[0].provenance.source_path)
        );

        let rule_features = builtin_kind::rule_features(&rule_units[0]).unwrap();
        assert_eq!(rule_features.id, typed.rules[0].id);
        assert_eq!(
            rule_features.field("paths"),
            Some(&FeatureValue::List(vec!["src/**/*.rs".to_string()]))
        );
    }

    #[test]
    fn load_skips_non_skill_dirs_and_returns_skills_sorted() {
        let ws = tmpdir("sorted");
        write_surface_skill(&ws, "bravo", DEMO);
        write_surface_skill(&ws, "alpha", DEMO);
        // Noise under skills/: a dir with no meta.toml must be skipped.
        fs::create_dir_all(ws.join("skills").join("empty")).unwrap();

        let loaded = Workspace::load(&ws).unwrap();
        let names: Vec<&str> = loaded.skills.iter().map(|s| s.id.as_str()).collect();
        // The member id is the surface directory name (`directory` shape), so the two
        // surfaces are `alpha` and `bravo`, name-sorted, and the stray dir was skipped.
        assert_eq!(names, vec!["alpha", "bravo"]);
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
