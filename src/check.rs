//! `temper check` — the workspace-load and diagnostic surface.
//!
//! Implements the loading half of the `check` gate: reconstruct
//! the typed config surface into a [`Workspace`] IR, and carry the [`Diagnostic`]
//! value the generic engine emits. The clauses themselves are validated by the
//! generic engine over the closed algebra — there is no per-rule code
//! here; the heuristic rule registry it replaced is retired (kill the heuristic
//! rule registry).
//!
//! A [`Diagnostic`] is a value the engine *collects*, not a thrown error — one
//! `error`-severity finding drives `check`'s non-zero exit ([`any_error`]), and
//! [`render`] presents the set with `miette`. It is distinct from a
//! [`WorkspaceError`] (a hard failure that aborts the load).

use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use miette::GraphicalReportHandler;

use crate::frontmatter::{FrontmatterError, Member};
use crate::kind::{CustomKind, KindError};

/// The loaded config surface: every built-in artifact `check` lints. Carries each
/// built-in kind's members as generic frontmatter [`Member`]s, keyed by the kind's bare name in a per-kind map so
/// every built-in kind's members — not just skills and rules — reach the readers that
/// range over the workspace (drift/bundle/explain), and a cross-artifact clause can
/// reach the whole harness at once.
///
/// Custom project kinds (temper's own `spec`, ADRs, …) are not built-ins: they are read
/// generically through [`Unit::from_surface_dir`](crate::kind::Unit::from_surface_dir),
/// not materialized here.
#[derive(Debug, Clone, PartialEq)]
pub struct Workspace {
    /// Each built-in kind's members, keyed by the kind's bare name (`skill`, `rule`, …).
    /// A kind with no surface members is present with an empty vector; each vector is
    /// name-sorted by its load. Private so the readers reach it through the accessors
    /// below, which fix the kind key at one call site.
    members: BTreeMap<String, Vec<Member>>,
}

impl Workspace {
    /// Load a workspace from its surface directory by reconstructing every built-in
    /// kind's members through the one generic frontmatter adapter
    /// ([`Member::from_surface`]) — no per-kind IR. The kind set is the embedded
    /// built-in std-lib ([`builtin_kind::definitions`](crate::builtin_kind::definitions)),
    /// so adding a built-in kind extends the workspace with no change here.
    ///
    /// A child is treated as an artifact surface only when it holds its kind's member
    /// document (`SKILL.md`, `RULE.md`, …), so stray files and partial directories are
    /// skipped rather than erroring. Each kind is returned name-sorted (the directory
    /// listing order is unspecified) so the diagnostic set is stable across runs.
    ///
    /// # Errors
    ///
    /// Returns a [`WorkspaceError`] if the built-in kind set fails to parse, a surface
    /// directory cannot be enumerated, or a member document is unreadable or malformed.
    pub fn load(dir: &Path) -> Result<Self, WorkspaceError> {
        Self::load_kinds(dir, crate::builtin_kind::definitions()?.values())
    }

    /// Load a workspace over an explicit built-in `kinds` set — the generic core of
    /// [`load`](Self::load), factored out so a third built-in kind's members can be
    /// exercised without waiting on the embedded set to grow. Each kind's members land
    /// under its bare name, read from `<dir>/<subdir>/*` by the kind's own
    /// [`surface_subdir`](CustomKind::surface_subdir) and
    /// [`member_document`](CustomKind::member_document). Two kinds sharing a bare name
    /// would silently overwrite each other's entry here — the embedded built-in set is
    /// guaranteed unique, so a collision is a malformed kind set, not
    /// a case this loader resolves.
    fn load_kinds<'a>(
        dir: &Path,
        kinds: impl IntoIterator<Item = &'a CustomKind>,
    ) -> Result<Self, WorkspaceError> {
        let mut members = BTreeMap::new();
        for kind in kinds {
            let doc = kind.member_document();
            let mut kind_members = Vec::new();
            for member_dir in &surface_dirs(&dir.join(kind.surface_subdir()), &doc)? {
                kind_members.push(Member::from_surface(member_dir, &doc)?);
            }
            members.insert(kind.name.clone(), kind_members);
        }
        Ok(Self { members })
    }

    /// This kind's loaded members, name-sorted, or an empty slice when the workspace
    /// carries none — the generic accessor the readers range over.
    #[must_use]
    pub fn members(&self, kind: &str) -> &[Member] {
        self.members.get(kind).map_or(&[], Vec::as_slice)
    }

    /// The skill members — the built-in `skill` kind's slice off the generic map.
    #[must_use]
    pub fn skills(&self) -> &[Member] {
        self.members("skill")
    }

    /// The rule members — the built-in `rule` kind's slice off the generic map.
    #[must_use]
    pub fn rules(&self) -> &[Member] {
        self.members("rule")
    }
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
    /// generic frontmatter adapter.
    #[error(transparent)]
    #[diagnostic(transparent)]
    Frontmatter(#[from] FrontmatterError),

    /// A built-in kind's surface member document could not be read generically
    /// — the check read's
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

    /// A fresh, empty temp directory, uniquely named via the sanctioned `tempfile`
    /// crate rather than a hand-rolled counter+pid scheme.
    fn tmpdir(label: &str) -> PathBuf {
        tempfile::Builder::new()
            .prefix(label)
            .tempdir()
            .expect("failed to create temp dir")
            .keep()
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

        assert_eq!(loaded.skills().len(), 1);
        assert_eq!(loaded.skills()[0].id, "demo");
        assert_eq!(
            loaded.skills()[0].field("version").and_then(|v| v.as_str()),
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
        assert_eq!(loaded.skills().len(), 1);
        let rule_names: Vec<&str> = loaded.rules().iter().map(|r| r.id.as_str()).collect();
        assert_eq!(rule_names, vec!["collaboration", "rust"]);
        assert_eq!(
            loaded.rules()[1].field("paths"),
            Some(&serde_json::json!(["src/**/*.rs"]))
        );
        // The no-frontmatter rule carries no `paths`.
        assert!(!loaded.rules()[0].has_field("paths"));
    }

    #[test]
    fn load_skips_non_skill_dirs_and_returns_skills_sorted() {
        let ws = tmpdir("sorted");
        write_surface_skill(&ws, "bravo", DEMO);
        write_surface_skill(&ws, "alpha", DEMO);
        // Noise under skills/: a dir with no meta.toml must be skipped.
        fs::create_dir_all(ws.join("skills").join("empty")).unwrap();

        let loaded = Workspace::load(&ws).unwrap();
        let names: Vec<&str> = loaded.skills().iter().map(|s| s.id.as_str()).collect();
        // The member id is the surface directory name (`directory` shape), so the two
        // surfaces are `alpha` and `bravo`, name-sorted, and the stray dir was skipped.
        assert_eq!(names, vec!["alpha", "bravo"]);
    }

    #[test]
    fn load_of_a_workspace_without_skills_dir_is_empty() {
        let ws = tmpdir("nodir");
        let loaded = Workspace::load(&ws).unwrap();
        assert!(loaded.skills().is_empty());
    }

    #[test]
    fn load_places_a_third_built_in_kinds_members_in_the_generic_map() {
        use crate::kind::{CustomKind, Extraction, Governs, Primitive, UnitShape};

        // A synthetic third built-in kind — an `agent` under `.claude/agents/<name>/
        // AGENT.md`, the shape the embedded set gains when the next kind ships. Its
        // member document is `AGENT.md`, its surface subdir `agents`, and it extracts a
        // `model` field — the field a `membership` clause over agents' satisfier set
        // would range over. With only skill/rule hardwired, this kind's members loaded
        // nowhere.
        let agent = CustomKind {
            unit_shape: Some(UnitShape::Directory),
            ..CustomKind::new(
                "agent",
                Governs {
                    root: ".claude/agents".to_string(),
                    glob: "*/AGENT.md".to_string(),
                },
                Extraction::new(vec![Primitive::Field {
                    key: "model".to_string(),
                }]),
            )
        };

        // Project one agent member onto the surface exactly as `import`/`emit` would.
        let ws = tmpdir("third-kind");
        let src = tmpdir("agent-src");
        fs::create_dir_all(src.join("reviewer")).unwrap();
        fs::write(
            src.join("reviewer").join("AGENT.md"),
            "---\nmodel: opus\n---\n# Reviewer\n\nBody.\n",
        )
        .unwrap();
        let member = Member::from_source(&agent, &src.join("reviewer").join("AGENT.md")).unwrap();
        let dir = ws.join("agents").join("reviewer");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("AGENT.md"), member.to_document().emit()).unwrap();

        // Load over skill + rule + the third kind: the agent members land under the bare
        // kind name in the generic map, and their reconstructed clause value — what a
        // `require` over the `agent` kind's clauses would run over — is present, no
        // longer stranded.
        let kinds = vec![builtin("skill"), builtin("rule"), agent];
        let loaded = Workspace::load_kinds(&ws, &kinds).unwrap();

        let agents = loaded.members("agent");
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].id, "reviewer");
        assert_eq!(
            agents[0].field("model").and_then(|v| v.as_str()),
            Some("opus")
        );
        // The third kind is additive: skill/rule stay reachable through the same map.
        assert!(loaded.skills().is_empty());
        assert!(loaded.rules().is_empty());
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
