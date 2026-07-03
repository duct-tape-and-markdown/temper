//! The wedge's advisory coverage note — silence about an unchecked surface must
//! never read as "checked".
//!
//! specs/architecture/50-distribution.md, "Fail-loud delivery — the invariant". The
//! gate checks each built-in kind's members and stays silent about everything else,
//! but silence about a surface temper carries no kind for (a `.claude/agents/` tree,
//! `settings.json`, an `.mcp.json`) is indistinguishable from "checked and clean".
//! This module makes the coverage explicit: one advisory note stating which kinds
//! checked how many members, plus one finding per known Claude Code surface present
//! on disk that no in-scope kind governs. Every finding is `warn` — the note narrates
//! coverage, it never gates, and the session-start reporter ignores it.
//!
//! The known-surface list is an **external fact** (.claude/rules/collaboration.md,
//! "External facts are cited"): each entry carries its Claude Code docs citation at
//! the point of claim.

use std::collections::BTreeMap;
use std::path::Path;

use crate::check::Diagnostic;
use crate::kind::{CustomKind, glob_matches};

/// The advisory rule id for the per-kind member-count summary.
const CHECKED_RULE: &str = "coverage.checked";

/// The advisory rule id for a known surface present on disk that no kind governs.
const UNMODELED_RULE: &str = "coverage.unmodeled-surface";

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
/// governs**: skills live under `.claude/skills/` (the `skill` kind), rules under
/// `.claude/rules/` (the `rule` kind), and memory under `CLAUDE.md`/`AGENTS.md` (the
/// two `memory` kinds), so those loci are deliberately absent — governance already
/// covers them. Hooks are **not** a directory: they are configured inside
/// `settings.json`, so the settings entry covers them and no invented `.claude/hooks/`
/// locus appears (a false locus would be the exact uncited guess collaboration.md
/// forbids). A `specs/` corpus is likewise absent: it is not a Claude Code surface,
/// and temper models it with *custom* kinds (`intent`/`architecture`/`process`), so
/// hardcoding it as ungoverned would fire a false positive on temper's own harness.
const KNOWN_SURFACES: &[KnownSurface] = &[
    KnownSurface {
        path: ".claude/agents",
        is_dir: true,
        holds: "Claude Code subagent definitions",
        source: SETTINGS_DOC,
    },
    KnownSurface {
        path: ".claude/commands",
        is_dir: true,
        holds: "Claude Code custom slash commands",
        source: SETTINGS_DOC,
    },
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
/// keyed by each kind's qualified identity; `kinds` is the in-scope built-in kind
/// set, consulted only to suppress a known surface a kind actually governs. Returns
/// `warn`-severity diagnostics only (never `error`, never a session-start verdict): a
/// summary of what was checked, then one finding per known Claude Code surface present
/// on disk that no in-scope kind governs — so the gate's silence about an unmodeled
/// surface never reads as "checked" (`specs/architecture/50-distribution.md`).
#[must_use]
pub fn check(
    root: &Path,
    kinds: &BTreeMap<String, CustomKind>,
    member_counts: &BTreeMap<String, usize>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // (1) State what WAS checked: each kind's member count, so a clean run reads as
    // "checked N members", never bare silence. Iteration is over the name-sorted
    // `BTreeMap`, so the summary is stable.
    let total: usize = member_counts.values().sum();
    let per_kind: Vec<String> = member_counts
        .iter()
        .map(|(kind, count)| format!("{kind} ({count})"))
        .collect();
    diagnostics.push(Diagnostic::warn(
        CHECKED_RULE,
        "harness",
        format!(
            "checked {total} member{} across {} built-in kind{}: {}",
            plural(total),
            member_counts.len(),
            plural(member_counts.len()),
            per_kind.join(", "),
        ),
    ));

    // (2) Name the gaps: a known Claude Code surface present on disk that no in-scope
    // kind governs is checked by nothing — flag it so silence never reads as "checked".
    for surface in KNOWN_SURFACES {
        if present(root, surface) && !governed_by_any(kinds, surface) {
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

    diagnostics
}

/// The plural suffix for a count — `""` for one, `"s"` otherwise.
fn plural(n: usize) -> &'static str {
    if n == 1 { "" } else { "s" }
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
    kinds.values().any(|kind| governs(kind, surface))
}

/// Whether `kind`'s member locus covers `surface`. A directory surface is governed
/// when the kind roots at or below it (its members live inside); a file surface is
/// governed when the kind roots at the file's parent and its glob leaf selects the
/// filename. Roots are normalized (`./` prefix and trailing `/` stripped, a bare `.`
/// treated as the harness root) so `governs.root = "."` compares against a top-level
/// file's empty parent.
fn governs(kind: &CustomKind, surface: &KnownSurface) -> bool {
    let root = normalize_root(&kind.governs.root);
    if surface.is_dir {
        root == surface.path || root.starts_with(&format!("{}/", surface.path))
    } else {
        let (parent, leaf) = split_file(surface.path);
        root == parent && glob_matches(kind.governs.glob_leaf(), leaf)
    }
}

/// A `governs.root` reduced to a comparable relative path: leading `./` and any
/// trailing `/` stripped, and a bare `.` (the harness root itself, the two `memory`
/// kinds' locus) folded to the empty string so it matches a top-level file's parent.
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
    use crate::kind::CustomKind;
    use std::path::Path;
    use toml_edit::DocumentMut;

    /// Parse a minimal `KIND.md` header into a [`CustomKind`] with the given `governs`
    /// locus — enough for the governance-suppression tests, which read only `governs`.
    fn kind_governing(name: &str, root: &str, glob: &str) -> CustomKind {
        let src = format!("governs = {{ root = \"{root}\", glob = \"{glob}\" }}\n");
        let doc = src.parse::<DocumentMut>().unwrap();
        CustomKind::from_header(doc.as_table(), name, Path::new("kinds/x/KIND.md")).unwrap()
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
        let counts = BTreeMap::from([
            ("claude-code.skill".to_string(), 2usize),
            ("claude-code.rule".to_string(), 3usize),
        ]);
        let diagnostics = check(
            Path::new("/nonexistent-harness-root"),
            &builtin_set(),
            &counts,
        );
        let summary = diagnostics
            .iter()
            .find(|d| d.rule == CHECKED_RULE)
            .expect("a checked-summary diagnostic");
        assert_eq!(summary.severity, Severity::Warn);
        assert!(summary.message.contains("claude-code.skill (2)"));
        assert!(summary.message.contains("claude-code.rule (3)"));
        // The total pluralizes and names both kinds.
        assert!(
            summary
                .message
                .contains("checked 5 members across 2 built-in kinds")
        );
    }

    #[test]
    fn a_single_member_and_kind_do_not_pluralize() {
        let counts = BTreeMap::from([("claude-code.skill".to_string(), 1usize)]);
        let diagnostics = check(
            Path::new("/nonexistent-harness-root"),
            &builtin_set(),
            &counts,
        );
        let summary = diagnostics.iter().find(|d| d.rule == CHECKED_RULE).unwrap();
        assert!(
            summary
                .message
                .contains("checked 1 member across 1 built-in kind:")
        );
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
}
