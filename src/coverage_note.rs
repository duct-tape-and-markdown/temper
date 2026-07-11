//! The wedge's advisory coverage note — silence about an unchecked surface must
//! never read as "checked".
//!
//! Fail-loud delivery — the invariant. The
//! gate checks each built-in kind's members and stays silent about everything else,
//! but silence about a surface temper carries no kind for (`settings.json`, an
//! `.mcp.json`) is indistinguishable from "checked and clean".
//! This module makes the coverage explicit: one advisory note stating which kinds
//! checked how many members, one finding per known Claude Code surface present on
//! disk that no in-scope kind governs, and one finding per stray entry directly under
//! `.claude/` that neither a kind nor a known surface claims. Every finding is `warn`
//! — the note narrates coverage, it never gates, and the session-start reporter
//! ignores it.
//!
//! The known-surface list is an **external fact** (.claude/rules/collaboration.md,
//! "External facts are cited"): each entry carries its Claude Code docs citation at
//! the point of claim.

use std::collections::BTreeMap;
use std::path::Path;

use ignore::WalkBuilder;

use crate::check::Diagnostic;
use crate::drift;
use crate::kind::{CustomKind, compile_glob};

/// The advisory rule id for the per-kind member-count summary.
const CHECKED_RULE: &str = "coverage.checked";

/// The advisory rule id for a known surface present on disk that no kind governs.
const UNMODELED_RULE: &str = "coverage.unmodeled-surface";

/// The advisory rule id for a `.claude/` entry that no in-scope kind governs and no
/// [`KNOWN_SURFACES`] row already names — a stray this module's own richer
/// `UNMODELED_RULE` finding never covers.
const UNCLAIMED_RULE: &str = "coverage.unclaimed-entry";

/// The workspace directory holding the committed lock this module reads its own
/// custom-kind rows from — mirrors `install.rs`'s own copy of the same literal.
const TEMPER_DIR: &str = ".temper";

/// A known Claude Code harness surface temper's built-in kinds do not govern — an
/// external fact carrying its citation at the point of claim
/// (.claude/rules/collaboration.md, "External facts are cited").
struct KnownSurface {
    /// The surface's path relative to the harness root (slash-separated).
    path: &'static str,
    /// Whether the path is a directory (`.claude/agents/`) or a single file
    /// (`.claude/settings.json`) — fixes both the on-disk probe and the governance test.
    is_dir: bool,
    /// A one-line description of what the surface holds, for the advisory message.
    holds: &'static str,
    /// The Claude Code docs the surface's existence and locus are claimed from.
    source: &'static str,
}

/// The Claude Code settings docs, retrieved 2026-07-02 — the shared citation for the
/// curated surfaces below, each of which is documented there.
const SETTINGS_DOC: &str = "code.claude.com/docs/en/settings (retrieved 2026-07-02)";

/// The curated known-surface list. Every entry is a documented Claude Code surface
/// (verified against the settings docs, [`SETTINGS_DOC`]) that **no built-in kind
/// governs**: skills live under `.claude/skills/` (the `skill` kind), commands under
/// `.claude/commands/` (the `command` kind), subagents under `.claude/agents/` (the
/// `agent` kind), rules under `.claude/rules/` (the `rule` kind), and memory under
/// `CLAUDE.md` (the `memory` kind), so those loci are deliberately absent —
/// governance already covers them.
/// Hooks are **not** a directory: they are configured inside
/// `settings.json`, so the settings entry covers them and no invented `.claude/hooks/`
/// locus appears (a false locus would be the exact uncited guess collaboration.md
/// forbids). A `specs/` corpus is likewise absent: it is not a Claude Code surface,
/// and temper models it with *custom* kinds (`intent`/`architecture`/`process`), so
/// hardcoding it as ungoverned would fire a false positive on temper's own harness.
const KNOWN_SURFACES: &[KnownSurface] = &[
    KnownSurface {
        path: ".claude/settings.json",
        is_dir: false,
        holds: "Claude Code project settings — permissions, env, and hooks",
        source: SETTINGS_DOC,
    },
    KnownSurface {
        path: ".mcp.json",
        is_dir: false,
        holds: "Claude Code project MCP server configuration",
        source: SETTINGS_DOC,
    },
];

/// Compute the wedge's advisory coverage note over the harness at `root`.
///
/// `member_counts` is the per-kind checked-member count the gate already loaded,
/// keyed by each kind's bare row label; `kinds` is the built-in kind set. The
/// gap check additionally reads `root`'s own committed lock for any kind it
/// declares beyond those built-ins, so a locked custom kind's `governs` suppresses
/// a known surface exactly as a built-in's does. Returns `warn`-severity
/// diagnostics only (never `error`, never a session-start verdict): a summary of
/// what was checked, then one finding per known Claude Code surface present on
/// disk that no in-scope kind governs — so the gate's silence about an unmodeled
/// surface never reads as "checked".
///
/// # Errors
///
/// Returns an error when `root`'s committed lock cannot be read or parsed, or when a
/// lock-declared kind row cannot be lifted — a corrupt lock is loud here, never a
/// silent degrade to built-ins-only suppression.
pub fn check(
    root: &Path,
    kinds: &BTreeMap<String, CustomKind>,
    member_counts: &BTreeMap<String, usize>,
) -> miette::Result<Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();

    // (1) State what WAS checked: each kind's member count, so a clean run reads as
    // "checked N members", never bare silence. Iteration is over the name-sorted
    // `BTreeMap`, so the summary is stable. `member_counts` already folds in every
    // locked custom kind's members alongside the built-ins, so the message names no
    // "built-in" qualifier that would misdescribe a custom-kind count.
    let total: usize = member_counts.values().sum();
    let per_kind: Vec<String> = member_counts
        .iter()
        .map(|(kind, count)| format!("{kind} ({count})"))
        .collect();
    diagnostics.push(Diagnostic::warn(
        CHECKED_RULE,
        "harness",
        format!(
            "checked {total} member{} across {} kind{}: {}",
            crate::display::plural(total),
            member_counts.len(),
            crate::display::plural(member_counts.len()),
            per_kind.join(", "),
        ),
    ));

    // (2) Name the gaps: a known Claude Code surface present on disk that no in-scope
    // kind governs is checked by nothing — flag it so silence never reads as "checked".
    // The governing set is the built-ins plus every kind the committed lock declares,
    // so a locked custom kind (e.g. a `widget` kind rooted at `.claude`, selecting
    // `settings.json`) suppresses the surface it governs exactly as a built-in does.
    let governing_kinds = with_locked_kinds(root, kinds)?;
    for surface in KNOWN_SURFACES {
        if present(root, surface) && !governed_by_any(&governing_kinds, surface) {
            diagnostics.push(Diagnostic::warn(
                UNMODELED_RULE,
                surface.path,
                format!(
                    "`{}` ({}) is present but no kind governs it — temper checks none of its members [source: {}]",
                    surface.path, surface.holds, surface.source
                ),
            ));
        }
    }

    // (3) Name the strays: an entry directly under `.claude/` that no in-scope kind
    // governs AND no `KNOWN_SURFACES` row already names is examined by nothing —
    // the known-surface exclusion keeps this disjoint from (2)'s richer, per-surface
    // `UNMODELED_RULE` message, so neither ever double-reports the same path.
    for (path, is_dir) in claude_entries(root) {
        if governed_by_any_path(&governing_kinds, &path, is_dir)
            || KNOWN_SURFACES.iter().any(|surface| surface.path == path)
        {
            continue;
        }
        diagnostics.push(Diagnostic::warn(
            UNCLAIMED_RULE,
            &path,
            format!("`{path}` is present under `.claude/` but no kind or known surface covers it"),
        ));
    }

    Ok(diagnostics)
}

/// Every entry directly under `<root>/.claude` (not recursive), as a
/// (`.claude/`-relative slash path, is-directory) pair — the unclaimed-entry scan's
/// input. Honors the repository's ignore rules (`.gitignore`, `.git/info/exclude`)
/// exactly as harness discovery does ([`crate::import`]), so a gitignored stray is by
/// declaration not authored here and never fires. A missing `.claude/` yields no
/// entries rather than an error — the same absent-harness tolerance [`present`] takes.
fn claude_entries(root: &Path) -> Vec<(String, bool)> {
    let claude_dir = root.join(".claude");
    if !claude_dir.is_dir() {
        return Vec::new();
    }
    let walk = WalkBuilder::new(&claude_dir)
        .max_depth(Some(1))
        .hidden(false) // a dotfile stray (`.clauignore`) must not hide from itself.
        .parents(false)
        .ignore(false)
        .git_global(false)
        .git_ignore(true)
        .git_exclude(true)
        .require_git(false)
        .build();
    walk.flatten()
        .filter(|entry| entry.path() != claude_dir)
        .filter_map(|entry| {
            let rel = entry.path().strip_prefix(root).ok()?;
            let is_dir = entry.file_type().is_some_and(|ft| ft.is_dir());
            Some((drift::to_lock_path(rel), is_dir))
        })
        .collect()
}

/// `kinds` plus every kind `root`'s committed lock declares that is not already in
/// `kinds` — so a locked custom kind's `governs` locus joins the built-ins for the
/// unmodeled-surface suppression below. A missing lock declares no kinds and degrades to
/// `kinds` alone; a present kind row outside its closed vocabulary rejects loud.
///
/// # Errors
///
/// Returns an error when the committed lock cannot be read or parsed, or when a
/// declared kind row cannot be lifted — a corrupt lock never silently reads as "no
/// kinds declared", which would drop the locked-kind suppression.
fn with_locked_kinds(
    root: &Path,
    kinds: &BTreeMap<String, CustomKind>,
) -> miette::Result<BTreeMap<String, CustomKind>> {
    let mut merged = kinds.clone();
    let locked = drift::read_declarations(&root.join(TEMPER_DIR))?;
    for row in &locked.kinds {
        if !merged.contains_key(&row.name) {
            merged.insert(row.name.clone(), CustomKind::from_kind_fact_row(row)?);
        }
    }
    Ok(merged)
}

/// Whether a known surface exists on disk at `root`, probed as its declared shape — a
/// directory for a tree, a file for a single-file surface — so a same-named file where
/// a directory is expected (or vice versa) is not mistaken for the surface.
fn present(root: &Path, surface: &KnownSurface) -> bool {
    let path = root.join(surface.path);
    if surface.is_dir {
        path.is_dir()
    } else {
        path.is_file()
    }
}

/// Whether any in-scope kind governs `surface` — the suppression that keeps the note
/// truthful to its inputs: a surface a kind actually covers is checked, not a gap.
fn governed_by_any(kinds: &BTreeMap<String, CustomKind>, surface: &KnownSurface) -> bool {
    governed_by_any_path(kinds, surface.path, surface.is_dir)
}

/// Whether any in-scope kind governs a harness-relative `path`, generalized off
/// [`KnownSurface`] so the unclaimed-entry scan reuses the same governance test on an
/// arbitrary on-disk path — see [`governs`] for the directory/file distinction.
fn governed_by_any_path(kinds: &BTreeMap<String, CustomKind>, path: &str, is_dir: bool) -> bool {
    kinds.values().any(|kind| governs(kind, path, is_dir))
}

/// Whether `kind`'s member locus covers `path`. A directory path is governed when the
/// kind roots at or below it (its members live inside); a file path is governed when
/// the kind roots at the file's parent and its glob leaf selects the filename. Roots
/// are normalized (`./` prefix and trailing `/` stripped, a bare `.` treated as the
/// harness root) so `governs.root = "."` compares against a top-level file's empty
/// parent.
///
/// A **manifest kind** (one carrying a collection address) is the exception: it represents
/// only its own segment of the host manifest (`hooks.<Event>` of `settings.json`), never
/// the whole file — the container's other segments stay unmodeled until the manifest is a
/// represented member. So it governs no whole-path surface here, and a manifest present on
/// disk keeps its unmodeled-surface finding until every segment is modeled.
fn governs(kind: &CustomKind, path: &str, is_dir: bool) -> bool {
    if kind.collection_address.is_some() {
        return false;
    }
    let root = normalize_root(&kind.governs.root);
    if is_dir {
        root == path || root.starts_with(&format!("{path}/"))
    } else {
        let (parent, leaf) = split_file(path);
        root == parent
            && compile_glob(kind.governs.glob_leaf()).is_some_and(|matcher| matcher.is_match(leaf))
    }
}

/// A `governs.root` reduced to a comparable relative path: leading `./` and any
/// trailing `/` stripped, and a bare `.` (the harness root itself, the `memory`
/// kind's locus) folded to the empty string so it matches a top-level file's parent.
fn normalize_root(root: &str) -> &str {
    let root = root.trim_start_matches("./").trim_end_matches('/');
    if root == "." { "" } else { root }
}

/// A file path split into its parent directory and filename leaf; a path with no `/`
/// has an empty parent (a harness-root file).
fn split_file(path: &str) -> (&str, &str) {
    path.rsplit_once('/').unwrap_or(("", path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::Severity;
    use crate::kind::{CustomKind, Extraction, Governs};
    use crate::test_support::tmpdir;

    /// A minimal [`CustomKind`] with the given `governs` locus — enough for the
    /// governance-suppression tests, which read only `governs`.
    fn kind_governing(name: &str, root: &str, glob: &str) -> CustomKind {
        CustomKind::new(
            name,
            Governs {
                root: root.to_string(),
                glob: glob.to_string(),
            },
            Extraction::new(Vec::new()),
        )
    }

    fn skill_kind() -> CustomKind {
        kind_governing("skill", ".claude/skills", "*/SKILL.md")
    }

    /// The two built-in-shaped kinds keyed by name — the set the note is handed.
    fn builtin_set() -> BTreeMap<String, CustomKind> {
        BTreeMap::from([
            ("skill".to_string(), skill_kind()),
            (
                "rule".to_string(),
                kind_governing("rule", ".claude/rules", "*.md"),
            ),
        ])
    }

    #[test]
    fn the_checked_summary_reports_each_kind_count_and_is_warn() {
        let counts = BTreeMap::from([("skill".to_string(), 2usize), ("rule".to_string(), 3usize)]);
        let diagnostics = check(
            Path::new("/nonexistent-harness-root"),
            &builtin_set(),
            &counts,
        )
        .unwrap();
        let summary = diagnostics
            .iter()
            .find(|d| d.rule == CHECKED_RULE)
            .expect("a checked-summary diagnostic");
        assert_eq!(summary.severity, Severity::Warn);
        assert!(summary.message.contains("skill (2)"));
        assert!(summary.message.contains("rule (3)"));
        // The total pluralizes and names both kinds, with no "built-in" qualifier —
        // `member_counts` folds in locked custom-kind members alongside built-ins.
        assert!(summary.message.contains("checked 5 members across 2 kinds"));
        assert!(!summary.message.contains("built-in"));
    }

    #[test]
    fn a_single_member_and_kind_do_not_pluralize() {
        let counts = BTreeMap::from([("skill".to_string(), 1usize)]);
        let diagnostics = check(
            Path::new("/nonexistent-harness-root"),
            &builtin_set(),
            &counts,
        )
        .unwrap();
        let summary = diagnostics.iter().find(|d| d.rule == CHECKED_RULE).unwrap();
        assert!(summary.message.contains("checked 1 member across 1 kind:"));
    }

    #[test]
    fn the_checked_summary_names_no_built_in_qualifier_when_a_custom_kind_is_counted() {
        // A custom kind's members ride the same `member_counts` map as built-ins —
        // the summary must not misdescribe them as "built-in".
        let counts = BTreeMap::from([
            ("skill".to_string(), 1usize),
            ("command".to_string(), 2usize),
        ]);
        let diagnostics = check(
            Path::new("/nonexistent-harness-root"),
            &builtin_set(),
            &counts,
        )
        .unwrap();
        let summary = diagnostics.iter().find(|d| d.rule == CHECKED_RULE).unwrap();
        assert!(summary.message.contains("command (2)"));
        assert!(!summary.message.contains("built-in"));
    }

    #[test]
    fn a_directory_surface_is_governed_only_by_a_kind_rooted_at_or_below_it() {
        let agents = KnownSurface {
            path: ".claude/agents",
            is_dir: true,
            holds: "agents",
            source: "x",
        };
        // No built-in kind roots under `.claude/agents`, so it is ungoverned.
        assert!(!governed_by_any(&builtin_set(), &agents));
        // A custom kind rooted exactly there governs it.
        let with_agent = BTreeMap::from([(
            "agent".to_string(),
            kind_governing("agent", ".claude/agents", "*.md"),
        )]);
        assert!(governed_by_any(&with_agent, &agents));
    }

    #[test]
    fn a_file_surface_is_governed_by_a_kind_whose_glob_leaf_selects_it() {
        let settings = KnownSurface {
            path: ".claude/settings.json",
            is_dir: false,
            holds: "settings",
            source: "x",
        };
        assert!(!governed_by_any(&builtin_set(), &settings));
        // A kind rooted at `.claude` selecting `settings.json` governs the file surface.
        let with_settings = BTreeMap::from([(
            "settings".to_string(),
            kind_governing("settings", ".claude", "settings.json"),
        )]);
        assert!(governed_by_any(&with_settings, &settings));
    }

    #[test]
    fn a_root_memory_locus_governs_a_top_level_file_surface() {
        // `governs.root = "."` (the memory kinds' locus) normalizes to the empty parent,
        // so a top-level file surface is governed when the glob leaf selects it — the
        // reason `CLAUDE.md`/`AGENTS.md` are never flagged.
        let claude_md = KnownSurface {
            path: "CLAUDE.md",
            is_dir: false,
            holds: "memory",
            source: "x",
        };
        let memory = BTreeMap::from([(
            "memory".to_string(),
            kind_governing("memory", ".", "CLAUDE.md"),
        )]);
        assert!(governed_by_any(&memory, &claude_md));
    }

    /// Commit a lock at `<root>/.temper/lock.toml` declaring one `widget` kind
    /// rooted at `.claude` selecting `settings.json` — a locked custom kind the
    /// built-in set (`builtin_set`) carries no row for. `widget` stands in for "some
    /// not-yet-shipped custom kind" here: `agent` no longer fits (AGENT-KIND
    /// graduated it to a real built-in, so `.claude/agents` is unconditionally
    /// governed and off [`KNOWN_SURFACES`], mirroring `command`'s own graduation).
    fn lock_widget_kind(root: &std::path::Path) {
        let payload = crate::drift::Payload {
            version: crate::drift::SEAM_VERSION,
            declarations: crate::drift::Declarations {
                kinds: vec![crate::drift::KindFactRow {
                    name: "widget".to_string(),
                    provider: None,
                    governs_root: ".claude".to_string(),
                    governs_glob: "settings.json".to_string(),
                    format: None,
                    unit_shape: Some("file".to_string()),
                    registration: Vec::new(),
                    templates: Vec::new(),
                    content: None,
                    shape: None,
                    collection_address: None,
                }],
                ..crate::drift::Declarations::default()
            },
            members: Vec::new(),
        };
        crate::drift::emit(
            &payload,
            &root.join(TEMPER_DIR),
            crate::drift::EmitOptions::default(),
        )
        .unwrap();
    }

    #[test]
    fn a_locked_custom_kind_suppresses_the_surface_it_governs() {
        let root = tmpdir("locked-widget-kind");
        lock_widget_kind(&root);
        std::fs::create_dir_all(root.join(".claude")).unwrap();
        std::fs::write(root.join(".claude/settings.json"), "{}").unwrap();

        let counts = BTreeMap::from([("widget".to_string(), 0usize)]);
        let diagnostics = check(&root, &builtin_set(), &counts).unwrap();

        assert!(
            diagnostics
                .iter()
                .all(|d| !(d.rule == UNMODELED_RULE && d.artifact == ".claude/settings.json")),
            "a locked custom kind governing .claude/settings.json must suppress the finding, got: {diagnostics:#?}"
        );
    }

    #[test]
    fn a_present_surface_with_no_locked_or_builtin_governor_is_still_flagged() {
        let root = tmpdir("no-lock");
        std::fs::write(root.join(".mcp.json"), "{}").unwrap();

        let counts = BTreeMap::new();
        let diagnostics = check(&root, &builtin_set(), &counts).unwrap();

        assert!(
            diagnostics
                .iter()
                .any(|d| d.rule == UNMODELED_RULE && d.artifact == ".mcp.json"),
            "an ungoverned present surface must still be flagged, got: {diagnostics:#?}"
        );
    }

    #[test]
    fn a_stray_claude_entry_no_kind_or_surface_covers_fires_unclaimed_entry() {
        let root = tmpdir("stray-entry");
        std::fs::create_dir_all(root.join(".claude")).unwrap();
        std::fs::write(root.join(".claude/.clauignore"), "").unwrap();

        let diagnostics = check(&root, &BTreeMap::new(), &BTreeMap::new()).unwrap();

        let matches: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.rule == UNCLAIMED_RULE)
            .collect();
        assert_eq!(matches.len(), 1, "{diagnostics:#?}");
        assert_eq!(matches[0].artifact, ".claude/.clauignore");
        assert_eq!(matches[0].severity, Severity::Warn);
    }

    #[test]
    fn a_governed_locus_under_claude_never_fires_unclaimed_entry() {
        let root = tmpdir("governed-locus");
        std::fs::create_dir_all(root.join(".claude/skills")).unwrap();

        let diagnostics = check(&root, &builtin_set(), &BTreeMap::new()).unwrap();

        assert!(
            diagnostics.iter().all(|d| d.rule != UNCLAIMED_RULE),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn a_known_surface_under_claude_does_not_double_report() {
        let root = tmpdir("known-surface-no-double");
        std::fs::create_dir_all(root.join(".claude")).unwrap();
        std::fs::write(root.join(".claude/settings.json"), "{}").unwrap();
        std::fs::write(root.join(".mcp.json"), "{}").unwrap();

        let diagnostics = check(&root, &BTreeMap::new(), &BTreeMap::new()).unwrap();

        assert!(
            diagnostics.iter().all(|d| d.rule != UNCLAIMED_RULE),
            "a known surface must never also fire coverage.unclaimed-entry, got: {diagnostics:#?}"
        );
        let unmodeled: Vec<_> = diagnostics
            .iter()
            .filter(|d| d.rule == UNMODELED_RULE)
            .collect();
        assert_eq!(
            unmodeled.len(),
            2,
            "each known surface still fires its own unmodeled-surface finding exactly once: {diagnostics:#?}"
        );
    }

    #[test]
    fn a_gitignored_stray_under_claude_never_fires() {
        let root = tmpdir("gitignored-stray");
        std::fs::create_dir_all(root.join(".claude")).unwrap();
        std::fs::write(root.join(".claude/.gitignore"), "ignored-stray.md\n").unwrap();
        std::fs::write(root.join(".claude/ignored-stray.md"), "").unwrap();

        let diagnostics = check(&root, &BTreeMap::new(), &BTreeMap::new()).unwrap();

        assert!(
            diagnostics
                .iter()
                .all(|d| !(d.rule == UNCLAIMED_RULE && d.artifact == ".claude/ignored-stray.md")),
            "a gitignored stray must never fire, got: {diagnostics:#?}"
        );
    }
}
