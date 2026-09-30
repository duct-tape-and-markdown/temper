//! Fail-loud delivery — the invariant. The gate checks each built-in kind's members and
//! stays silent about everything else, so two finding classes say what silence cannot:
//! the disclosure of what was checked, and each entry directly under `.claude/` that no
//! in-scope kind governs and no known Claude Code surface names.
//!
//! The known-surface exclusion list is an **external fact**
//! (.claude/rules/collaboration.md, "External facts are cited"), cited at its point of
//! claim.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use ignore::WalkBuilder;

use crate::builtin_kind::{CLAUDE_ROOT, KNOWN_SURFACES};
use crate::check::Diagnostic;
use crate::drift;
use crate::glob::compile_glob;
use crate::kind::CustomKind;

/// The rule id for the per-kind member-count summary — a **disclosure note**
/// ([`Severity::Note`](crate::check::Severity::Note)), not an advisory: it states what
/// was checked, so there is no clause behind it a corpus could dial and nothing for
/// `--deny-advisories` to promote.
const CHECKED_RULE: &str = "coverage.checked";

/// The advisory rule id for a `.claude/` entry that no in-scope kind governs and
/// [`KNOWN_SURFACES`] does not list.
const UNCLAIMED_RULE: &str = "coverage.unclaimed-entry";

/// Compute the wedge's coverage note over the harness at `root`.
///
/// `member_counts` is the per-kind count of members discovered at a `governs` locus,
/// keyed by each kind's bare row label; `nested_member_counts` is the per-kind
/// embedded-member count over both sources a nested member reaches the run through —
/// the lock's own `nested_member` rows and the members a host kind's read composed
/// ([`crate::gate`]). Both maps are checked members, so both are counted;
/// `undeclared_counts` is how many of each kind's discovered members no lock row
/// declares, disclosed apart so the one line stating what was checked cannot silently
/// absorb a document the program does not declare — a **disclosure**, never the
/// `locus-declared` clause's finding, so dialing that clause changes the finding's
/// weight and never the count printed here; `kinds` is the built-in kind set.
/// `locked_kinds` are the kind-fact rows from the committed lock
/// (an empty slice for an unadopted harness), so a locked custom kind's `governs` claims
/// a `.claude/` entry exactly as a built-in's does. Nothing here is ever
/// `error` and none of it is a session-start verdict: the summary of what was checked is
/// a `note`-severity **disclosure** (no clause declares it, so `--deny-advisories` never
/// promotes it), and each stray `.claude/` entry nothing claims is a `warn`-severity
/// advisory — a real gap the corpus closes by modelling it — so the gate's silence about
/// an unexamined entry never reads as "checked".
///
/// # Errors
///
/// Returns an error when a lock-declared kind row cannot be lifted — a corrupt lock
/// is loud here, never a silent degrade to built-ins-only exclusion.
pub fn check(
    root: &Path,
    kinds: &BTreeMap<String, CustomKind>,
    member_counts: &BTreeMap<String, usize>,
    nested_member_counts: &BTreeMap<String, usize>,
    undeclared_counts: &BTreeMap<String, usize>,
    locked_kinds: &[crate::drift::KindFactRow],
) -> miette::Result<Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();

    // (1) State what WAS checked: each kind's member count, so a clean run reads as
    // "checked N members", never bare silence. Iteration is over the name-sorted
    // `BTreeMap`, so the summary is stable. `member_counts` already folds in every
    // locked custom kind's members alongside the built-ins, so the message names no
    // "built-in" qualifier that would misdescribe a custom-kind count.
    // The total sums BOTH maps, because an embedded member is a checked member: the
    // embedded dispatcher runs it through the same two greens an at-locus member takes.
    // Count and enumeration are one fact, so a kind carrying both a discovered and an
    // embedded count renders both parts below rather than hiding one from the arithmetic.
    let total: usize =
        member_counts.values().sum::<usize>() + nested_member_counts.values().sum::<usize>();
    let mut all_kinds: BTreeSet<&String> = member_counts.keys().collect();
    all_kinds.extend(nested_member_counts.keys());
    let per_kind: Vec<String> = all_kinds
        .iter()
        .map(|kind| {
            let discovered_count = member_counts.get(*kind).copied().unwrap_or(0);
            let embedded_count = nested_member_counts.get(*kind).copied().unwrap_or(0);
            let undeclared = undeclared_counts.get(*kind).copied().unwrap_or(0);
            let mut parts = Vec::new();
            // A purely embedded kind contributes no discovered segment at all — a bare
            // `(0)` beside its embedded count would read as dead.
            if discovered_count > 0 || embedded_count == 0 {
                // The undeclared split appends only where there IS an undeclared member,
                // so a kind whose members are all declared renders exactly as it always
                // has.
                parts.push(if undeclared > 0 {
                    format!(
                        "{}: {} declared, {} undeclared",
                        discovered_count,
                        discovered_count.saturating_sub(undeclared),
                        undeclared
                    )
                } else {
                    discovered_count.to_string()
                });
            }
            if embedded_count > 0 {
                parts.push(format!("{embedded_count} embedded"));
            }
            format!("{} ({})", kind, parts.join(", "))
        })
        .collect();
    let kind_count = all_kinds.len();
    diagnostics.push(Diagnostic::note(
        CHECKED_RULE,
        "harness",
        format!(
            "checked {total} member{} across {} kind{}: {}",
            crate::display::plural(total),
            kind_count,
            crate::display::plural(kind_count),
            per_kind.join(", "),
        ),
    ));

    // (2) Name the strays: an entry directly under `.claude/` that no in-scope kind
    // governs AND `KNOWN_SURFACES` does not list is examined by nothing, and silence
    // about it is indistinguishable from "checked and clean". The governing set is the
    // built-ins plus every kind the committed lock declares, so a locked custom kind
    // claims its own locus exactly as a built-in does.
    let governing_kinds = with_locked_kinds(kinds, locked_kinds)?;
    for (path, is_dir) in claude_entries(root) {
        if governed_by_any_path(&governing_kinds, &path, is_dir)
            || KNOWN_SURFACES.contains(&path.as_str())
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
/// entries rather than an error: an unadopted tree is not a finding.
fn claude_entries(root: &Path) -> Vec<(String, bool)> {
    let claude_dir = root.join(CLAUDE_ROOT);
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

/// `kinds` plus every kind row in `locked_kinds` that is not already in `kinds` — so
/// a locked custom kind's `governs` locus joins the built-ins for the stray-entry
/// exclusion. An empty `locked_kinds` slice degrades to `kinds` alone; a
/// kind row outside its closed vocabulary rejects loud.
///
/// # Errors
///
/// Returns an error when a declared kind row cannot be lifted — a corrupt row is loud
/// here, never a silent degrade to built-ins-only exclusion.
fn with_locked_kinds(
    kinds: &BTreeMap<String, CustomKind>,
    locked_kinds: &[crate::drift::KindFactRow],
) -> miette::Result<BTreeMap<String, CustomKind>> {
    let mut merged = kinds.clone();
    for row in locked_kinds {
        if !merged.contains_key(&row.name) {
            merged.insert(row.name.clone(), CustomKind::from_kind_fact_row(row)?);
        }
    }
    Ok(merged)
}

/// Whether any in-scope kind governs a harness-relative `path` — the exclusion that keeps
/// the stray scan truthful to its inputs: an entry a kind actually covers is checked, not
/// a gap. See [`governs`] for the directory/file distinction.
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
/// A **manifest kind** (one carrying a collection address) governs its host file only when
/// its collection spans the whole manifest: `.mcp.json` is wholly its `mcpServers` map, so
/// the `mcp-server` kind covers the file outright. A segment kind (`hooks.<Event>` of
/// `settings.json`) represents only its own slice, so it governs no path — whole-file
/// governance there is the `settings` container's. A manifest kind never governs a
/// directory (its members live in a file, not a tree). A kind governing no locus at all —
/// a nested file kind, whose members compose their paths under their host's unit — covers
/// no path of its own.
fn governs(kind: &CustomKind, path: &str, is_dir: bool) -> bool {
    let Some(governs) = &kind.governs else {
        return false;
    };
    if let Some(address) = &kind.collection_address {
        if is_dir || !address.key_path.spans_whole_manifest() {
            return false;
        }
        return governs_file_leaf(governs, path);
    }
    let root = normalize_root(&governs.root);
    if is_dir {
        root == path || root.starts_with(&format!("{path}/"))
    } else {
        governs_file_leaf(governs, path)
    }
}

/// Whether a file path matches a `governs` locus's root and glob-leaf filter.
fn governs_file_leaf(governs: &crate::kind::Governs, path: &str) -> bool {
    let (parent, leaf) = split_file(path);
    normalize_root(&governs.root) == parent
        && compile_glob(governs.glob_leaf()).is_some_and(|matcher| matcher.is_match(leaf))
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
    /// governance-exclusion tests, which read only `governs`.
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
    fn the_checked_summary_reports_each_kind_count_and_is_a_disclosure_note() {
        let counts = BTreeMap::from([("skill".to_string(), 2usize), ("rule".to_string(), 3usize)]);
        let diagnostics = check(
            Path::new("/nonexistent-harness-root"),
            &builtin_set(),
            &counts,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
        )
        .unwrap();
        let summary = diagnostics
            .iter()
            .find(|d| d.rule == CHECKED_RULE)
            .expect("a checked-summary diagnostic");
        // Disclosure, never a violation: nothing a corpus declared, so nothing
        // `--deny-advisories` may promote to blocking.
        assert_eq!(summary.severity, Severity::Note);
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
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
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
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
        )
        .unwrap();
        let summary = diagnostics.iter().find(|d| d.rule == CHECKED_RULE).unwrap();
        assert!(summary.message.contains("command (2)"));
        assert!(!summary.message.contains("built-in"));
    }

    #[test]
    fn the_checked_total_counts_embedded_members_alongside_discovered_ones() {
        // An embedded member is checked — the embedded dispatcher runs it through the same
        // two greens — so the one line stating what was checked counts it.
        let counts = BTreeMap::from([("skill".to_string(), 1usize)]);
        let nested = BTreeMap::from([("supporting-doc".to_string(), 2usize)]);
        let diagnostics = check(
            Path::new("/nonexistent-harness-root"),
            &builtin_set(),
            &counts,
            &nested,
            &BTreeMap::new(),
            &[],
        )
        .unwrap();
        let summary = diagnostics.iter().find(|d| d.rule == CHECKED_RULE).unwrap();
        assert!(
            summary.message.contains("checked 3 members across 2 kinds"),
            "the total must sum both maps, got: {}",
            summary.message
        );
        // Non-vacuity: the embedded half is populated and enumerated, so the 3 is the
        // 1 + 2 the sentence goes on to name.
        assert!(summary.message.contains("skill (1)"), "{}", summary.message);
        assert!(
            summary.message.contains("supporting-doc (2 embedded)"),
            "{}",
            summary.message
        );
    }

    #[test]
    fn a_kind_carrying_both_a_discovered_and_an_embedded_count_renders_both() {
        // The total sums both maps unconditionally, so a kind appearing in both must
        // disclose both segments — a hidden embedded half is the count and the
        // enumeration disagreeing.
        let counts = BTreeMap::from([("skill".to_string(), 2usize)]);
        let nested = BTreeMap::from([("skill".to_string(), 1usize)]);
        let diagnostics = check(
            Path::new("/nonexistent-harness-root"),
            &builtin_set(),
            &counts,
            &nested,
            &BTreeMap::new(),
            &[],
        )
        .unwrap();
        let summary = diagnostics.iter().find(|d| d.rule == CHECKED_RULE).unwrap();
        assert!(
            summary.message.contains("skill (2, 1 embedded)"),
            "{}",
            summary.message
        );
        assert!(
            summary.message.contains("checked 3 members across 1 kind"),
            "{}",
            summary.message
        );
    }

    #[test]
    fn a_directory_entry_is_governed_only_by_a_kind_rooted_at_or_below_it() {
        // No built-in-shaped kind roots under `.claude/agents`, so it is ungoverned.
        assert!(!governed_by_any_path(
            &builtin_set(),
            ".claude/agents",
            true
        ));
        // A custom kind rooted exactly there governs it.
        let with_agent = BTreeMap::from([(
            "agent".to_string(),
            kind_governing("agent", ".claude/agents", "*.md"),
        )]);
        assert!(governed_by_any_path(&with_agent, ".claude/agents", true));
    }

    #[test]
    fn a_file_entry_is_governed_by_a_kind_whose_glob_leaf_selects_it() {
        assert!(!governed_by_any_path(
            &builtin_set(),
            ".claude/settings.json",
            false
        ));
        // A kind rooted at `.claude` selecting `settings.json` governs the file.
        let with_settings = BTreeMap::from([(
            "settings".to_string(),
            kind_governing("settings", ".claude", "settings.json"),
        )]);
        assert!(governed_by_any_path(
            &with_settings,
            ".claude/settings.json",
            false
        ));
    }

    #[test]
    fn a_stray_claude_entry_no_kind_or_surface_covers_fires_unclaimed_entry() {
        let root = tmpdir("stray-entry");
        std::fs::create_dir_all(root.join(".claude")).unwrap();
        std::fs::write(root.join(".claude/.clauignore"), "").unwrap();

        let diagnostics = check(
            &root,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
        )
        .unwrap();

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

        let diagnostics = check(
            &root,
            &builtin_set(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
        )
        .unwrap();

        assert!(
            diagnostics.iter().all(|d| d.rule != UNCLAIMED_RULE),
            "{diagnostics:#?}"
        );
    }

    #[test]
    fn a_known_surface_under_claude_never_fires_unclaimed_entry() {
        // The exclusion list's whole remaining job, driven with nothing in scope to govern
        // the file: a documented Claude Code surface is not a stray, so the scan passes over
        // it on the registry row alone.
        let root = tmpdir("known-surface-excluded");
        std::fs::create_dir_all(root.join(".claude")).unwrap();
        std::fs::write(root.join(".claude/settings.json"), "{}").unwrap();

        let diagnostics = check(
            &root,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
        )
        .unwrap();

        assert!(
            diagnostics.iter().all(|d| d.rule != UNCLAIMED_RULE),
            "a known surface must never fire coverage.unclaimed-entry, got: {diagnostics:#?}"
        );
    }

    #[test]
    fn a_gitignored_stray_under_claude_never_fires() {
        let root = tmpdir("gitignored-stray");
        std::fs::create_dir_all(root.join(".claude")).unwrap();
        std::fs::write(root.join(".claude/.gitignore"), "ignored-stray.md\n").unwrap();
        std::fs::write(root.join(".claude/ignored-stray.md"), "").unwrap();

        let diagnostics = check(
            &root,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
        )
        .unwrap();

        assert!(
            diagnostics
                .iter()
                .all(|d| !(d.rule == UNCLAIMED_RULE && d.artifact == ".claude/ignored-stray.md")),
            "a gitignored stray must never fire, got: {diagnostics:#?}"
        );
    }
}
