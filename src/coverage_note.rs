//! Fail-loud delivery — the invariant. The gate checks each built-in kind's members and
//! stays silent about everything else, so two finding classes say what silence cannot:
//! the disclosure of what was checked, and each entry directly under `.claude/` that no
//! in-scope kind governs and no known Claude Code surface names.
//!
//! The known-surface exclusion list is an **external fact**
//! (.claude/rules/collaboration.md, "External facts are cited"), cited at its point of
//! claim.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::builtin_kind::{CLAUDE_ROOT, KNOWN_SURFACES};
use crate::check::Diagnostic;
use crate::kind::CustomKind;
use crate::path::HarnessRelativePath;

/// The rule id for the per-kind member-count summary — a **disclosure note**
/// ([`Severity::Note`](crate::check::Severity::Note)), not an advisory: it states what
/// was checked, so there is no clause behind it a corpus could dial and nothing for
/// `--deny-advisories` to promote.
const CHECKED_RULE: &str = "coverage.checked";

/// The advisory rule id for a `.claude/` entry that no in-scope kind governs and
/// [`KNOWN_SURFACES`] does not list.
const UNCLAIMED_RULE: &str = "coverage.unclaimed-entry";

/// The disclosure rule id for the supporting docs no typed reference reaches on an
/// unrepresented harness — a note, because there the reference is unfillable: such a
/// harness declares no edges, so a per-doc advisory would ask for what it cannot hold.
const UNREACHED_DOCS_RULE: &str = "coverage.unreached-supporting-docs";

/// The disclosure rule id for an unrepresented harness whose settings run a command —
/// a note, because such a harness declares no tools, so the `{ file }` tool a script
/// would become is unfillable there.
const SCRIPTS_RULE: &str = "coverage.unrepresented-scripts";

/// The built-in kinds whose documents are Claude Code settings files, where a hook
/// handler or the status line names a command.
const SETTINGS_KINDS: [&str; 2] = ["settings", "settings-local"];
/// The disclosure rule id for the references naming a target the tree does not show and a
/// scope `temper` does not read may answer — a note, because the verdict is undecidable in
/// a check of the project.
const OFF_TREE_RULE: &str = "coverage.off-tree-targets";

/// Fold every off-tree reference into one disclosure naming each reference and the scopes
/// `temper` does not read — `None` when there is none.
#[must_use]
pub fn off_tree_targets(references: &[crate::graph::OffTreeReference]) -> Option<Diagnostic> {
    if references.is_empty() {
        return None;
    }
    let named: Vec<String> = references
        .iter()
        .map(|reference| {
            format!(
                "`{}` `{}` → `{}`",
                reference.from.1, reference.field, reference.target
            )
        })
        .collect();
    let scopes: BTreeSet<&str> = references
        .iter()
        .flat_map(|reference| reference.scopes.iter().copied())
        .collect();
    let scopes: Vec<&str> = scopes.into_iter().collect();
    Some(Diagnostic::note(
        OFF_TREE_RULE,
        "harness",
        format!(
            "{} reference{} name{} a target the tree does not show ({}); temper does not read \
             {}, which may answer {}, so none is judged dangling",
            references.len(),
            crate::display::plural(references.len()),
            if references.len() == 1 { "s" } else { "" },
            named.join(", "),
            scopes.join(", "),
            if references.len() == 1 { "it" } else { "them" },
        ),
    ))
}

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
/// weight and never the count printed here; `declared_kinds` is the run's one effective
/// kind set ([`crate::compose::LockFamily::declared_kinds`]) — every built-in under the
/// committed lock's overlay plus every kind that lock declares — so a locked custom
/// kind's `governs` claims a `.claude/` entry exactly as a built-in's does, and a
/// relocated built-in claims the locus it was relocated *to*. Nothing here is ever
/// `error` and none of it is a session-start verdict: the summary of what was checked is
/// a `note`-severity **disclosure** (no clause declares it, so `--deny-advisories` never
/// promotes it), and each stray `.claude/` entry nothing claims is a `warn`-severity
/// advisory — a real gap the corpus closes by modelling it — so the gate's silence about
/// an unexamined entry never reads as "checked". `claimed_files` is the set of
/// non-member files the roster reaches, spelled under `root`: each one a memory `@path`
/// import reaches ([`crate::graph::DirectiveClassing::reached_files`]), claimed by its
/// importer through the import edge, and each declared input (a `{ file }` tool's
/// script among them), claimed by the member declaring it — so neither is a stray.
/// `unreached_supporting_docs` counts the supporting docs no typed reference reaches on
/// an unrepresented harness; a non-zero count adds one disclosure line in place of a
/// finding per doc.
pub fn check(
    root: &Path,
    declared_kinds: &BTreeMap<String, CustomKind>,
    member_counts: &BTreeMap<String, usize>,
    nested_member_counts: &BTreeMap<String, usize>,
    undeclared_counts: &BTreeMap<String, usize>,
    claimed_files: &BTreeSet<PathBuf>,
    unreached_supporting_docs: usize,
) -> Vec<Diagnostic> {
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

    if unreached_supporting_docs > 0 {
        diagnostics.push(Diagnostic::note(
            UNREACHED_DOCS_RULE,
            "harness",
            format!(
                "{unreached_supporting_docs} supporting doc{} no typed reference reaches: an \
                 unrepresented harness declares no edges, so a prose link is not one. \
                 Represent the harness and reference each doc with `mentionOf` or its path \
                 mention to judge them per doc",
                crate::display::plural(unreached_supporting_docs),
            ),
        ));
    }

    // (2) Name the strays: an entry directly under `.claude/` that no in-scope kind
    // governs AND `KNOWN_SURFACES` does not list is examined by nothing, and silence
    // about it is indistinguishable from "checked and clean". The governing set is the
    // run's effective one, so a locked custom kind claims its own locus and a relocated
    // built-in claims its new one — the raw embedded definitions would read a relocated
    // built-in's pre-relocation locus and name its real one a stray.
    let fully_reached = fully_reached_dirs(root, claimed_files);
    for (path, is_dir) in claude_entries(root) {
        let reached = if is_dir {
            fully_reached.contains(&path)
        } else {
            claimed_files.contains(&crate::path::normalize_path(&root.join(&*path)))
        };
        if reached
            || governed_by_any_path(declared_kinds, &path, is_dir)
            || KNOWN_SURFACES.contains(&&*path)
        {
            continue;
        }
        diagnostics.push(
            Diagnostic::warn(
                UNCLAIMED_RULE,
                &*path,
                format!(
                    "`{path}` is present under `.claude/` but no kind or known surface covers it"
                ),
            )
            .with_guidance(cited_remedy(&path, is_dir).map(str::to_string)),
        );
    }

    diagnostics
}

/// The one script note for an unrepresented harness whose settings run a command: a
/// `command` hook handler under `hooks.<Event>[].hooks[]` or a `command` `statusLine`
/// (code.claude.com/docs/en/hooks, "Common fields", retrieved 2026-09-25;
/// code.claude.com/docs/en/statusline, retrieved 2026-11-25). The note names and counts
/// no script, since telling a script path from an inline command is mining the command
/// string. A settings file absent or unparseable is read as running nothing — the
/// settings kind's own read reports a malformed one.
pub fn script_note(
    root: &Path,
    declared_kinds: &BTreeMap<String, CustomKind>,
) -> Option<Diagnostic> {
    let runs_command = SETTINGS_KINDS
        .iter()
        .filter_map(|name| declared_kinds.get(*name)?.governs.as_ref())
        .filter_map(|governs| {
            let path = root.join(normalize_root(&governs.root)).join(&governs.glob);
            serde_json::from_str::<serde_json::Value>(&std::fs::read_to_string(path).ok()?).ok()
        })
        .any(|settings| settings_run_a_command(&settings));
    runs_command.then(|| {
        Diagnostic::note(
            SCRIPTS_RULE,
            "harness",
            "settings run a hook or status-line command: an unrepresented harness declares \
             no tools, so its scripts are not checked. Represent the harness and declare \
             each committed script as a `{ file }` tool the command runs",
        )
    })
}

fn settings_run_a_command(settings: &serde_json::Value) -> bool {
    let is_command = |handler: &serde_json::Value| {
        handler.get("type").and_then(serde_json::Value::as_str) == Some("command")
            && handler
                .get("command")
                .is_some_and(serde_json::Value::is_string)
    };
    let hook_runs = settings
        .get("hooks")
        .and_then(serde_json::Value::as_object)
        .into_iter()
        .flat_map(|events| events.values())
        .filter_map(serde_json::Value::as_array)
        .flatten()
        .filter_map(|group| group.get("hooks")?.as_array())
        .flatten()
        .any(is_command);
    hook_runs || settings.get("statusLine").is_some_and(is_command)
}

/// The documented remedy for an unclaimed entry Claude Code itself writes, where its
/// docs name one — the finding's guidance, so the advisory says what to do rather than
/// only that nothing covers the entry. Each remedy is an **external fact**, quoted and
/// cited at its point of claim:
///
/// - `.claude/worktrees/` is where `claude --worktree` checks out its worktrees, and
///   the docs advise "Add `.claude/worktrees/` to your `.gitignore`"
///   (code.claude.com/docs/en/worktrees, retrieved 2026-11-25). The remedy works where
///   the finding is read: the scan honors the harness root's `.gitignore`
///   ([`claude_walk`]), so following it silences the finding.
fn cited_remedy(path: &HarnessRelativePath, is_dir: bool) -> Option<&'static str> {
    (is_dir && &**path == ".claude/worktrees").then_some(
        "Claude Code checks its worktrees out here, and its docs advise: \"Add \
         `.claude/worktrees/` to your `.gitignore`\" (code.claude.com/docs/en/worktrees)",
    )
}

/// A walk over `<root>/.claude` under the shared ignore policy
/// ([`crate::import::ignore_walk`]), rooted at the harness root and pruned to the
/// `.claude` subtree, so the root's `.gitignore` and the repository's exclude govern
/// `.claude/` entries exactly as git reads them — a walk rooted at `.claude` itself
/// would read neither. `max_depth` is counted from the harness root.
fn claude_walk(root: &Path, max_depth: Option<usize>) -> ignore::Walk {
    crate::import::ignore_walk(root, true)
        .max_depth(max_depth)
        .filter_entry(|entry| entry.depth() != 1 || entry.file_name() == CLAUDE_ROOT)
        .build()
}

/// Every entry directly under `<root>/.claude` (not recursive), as a
/// (path, is-directory) pair — the unclaimed-entry scan's input. The path keeps its
/// `.claude/` prefix, because a governing claim is spelled against the harness root and
/// the finding names the entry the author sees. The ignore sources are the shared
/// policy's ([`crate::import::ignore_walk`]), so a gitignored stray is by declaration
/// not authored here and never fires. The whole-tree walk's three membership
/// fences (`.git/`, the surface workspace, a nested governed root) are deliberately not
/// taken: each prunes *descent* into a subtree the
/// enclosing corpus does not own, and this scan descends nothing — it names the direct
/// children of one directory. A missing `.claude/` yields no entries rather than an
/// error: an unadopted tree is not a finding.
fn claude_entries(root: &Path) -> Vec<(HarnessRelativePath, bool)> {
    let claude_dir = root.join(CLAUDE_ROOT);
    if !claude_dir.is_dir() {
        return Vec::new();
    }
    claude_walk(root, Some(2))
        .flatten()
        .filter(|entry| entry.depth() == 2)
        .filter_map(|entry| {
            let rel = entry.path().strip_prefix(root).ok()?;
            let is_dir = entry.file_type().is_some_and(|ft| ft.is_dir());
            Some((HarnessRelativePath::from_path(rel), is_dir))
        })
        .collect()
}

/// The directories directly under `<root>/.claude` holding at least one file, every one
/// of which the roster claims (an import reaches it, or a member declares it its input)
/// — claimed whole, as each of its files is. `every`, never `some`: the scan names direct children only, so a partly reached directory
/// claimed whole would hide its unreached file. An empty directory reaches nothing and
/// stays unclaimed. One walk under the scan's own ignore policy, so a gitignored file
/// neither claims nor blocks a claim.
fn fully_reached_dirs(
    root: &Path,
    claimed_files: &BTreeSet<PathBuf>,
) -> BTreeSet<HarnessRelativePath> {
    let claude_dir = root.join(CLAUDE_ROOT);
    if !claude_dir.is_dir() {
        return BTreeSet::new();
    }
    // Per top-level directory: does every file beneath it sit in `claimed_files`?
    let mut all_reached: BTreeMap<PathBuf, bool> = BTreeMap::new();
    for entry in claude_walk(root, None).flatten() {
        if !entry.file_type().is_some_and(|ft| ft.is_file()) {
            continue;
        }
        let Ok(rel) = entry.path().strip_prefix(&claude_dir) else {
            continue;
        };
        let mut components = rel.components();
        let (Some(top), Some(_)) = (components.next(), components.next()) else {
            continue;
        };
        let reached = claimed_files.contains(&crate::path::normalize_path(entry.path()));
        *all_reached
            .entry(Path::new(CLAUDE_ROOT).join(top))
            .or_insert(true) &= reached;
    }
    all_reached
        .into_iter()
        .filter(|(_, reached)| *reached)
        .map(|(dir, _)| HarnessRelativePath::from_path(&dir))
        .collect()
}

/// Whether any in-scope kind governs `path` — the exclusion that keeps the stray scan
/// truthful to its inputs: an entry a kind actually covers is checked, not a gap. See
/// [`governs`] for the directory/file distinction.
fn governed_by_any_path(
    kinds: &BTreeMap<String, CustomKind>,
    path: &HarnessRelativePath,
    is_dir: bool,
) -> bool {
    kinds.values().any(|kind| governs(kind, path, is_dir))
}

/// Whether `kind`'s member locus covers `path`. A directory path is governed when the
/// kind roots at or below it (its members live inside); a file path is governed when the
/// path is a member of the kind's file locus ([`CustomKind::locus_admits`]: the whole
/// locus pattern matches it and the kind does not exclude it) — the one membership
/// predicate discovery walks and the guard watch, so the scan's "is this entry checked?"
/// is the governing claim itself and not a second, wider reading of it.
///
/// A **manifest kind** (one carrying a collection address) governs its host file only when
/// its collection spans the whole manifest: `.mcp.json` is wholly its `mcpServers` map, so
/// the `mcp-server` kind covers the file outright. A segment kind (`hooks.<Event>` of
/// `settings.json`) represents only its own slice, so it governs no path — whole-file
/// governance there is the `settings` container's. A manifest kind never governs a
/// directory (its members live in a file, not a tree). A kind governing no locus at all —
/// a nested file kind, whose members compose their paths under their host's unit — covers
/// no path of its own.
fn governs(kind: &CustomKind, path: &HarnessRelativePath, is_dir: bool) -> bool {
    let Some(governs) = &kind.governs else {
        return false;
    };
    if let Some(address) = &kind.collection_address {
        if is_dir || !address.key_path.spans_whole_manifest() {
            return false;
        }
        return kind.locus_admits(path);
    }
    if is_dir {
        let root = normalize_root(&governs.root);
        root == &**path
            || root.starts_with(&format!("{path}/"))
            || units_sit_under(&governs.root, &governs.glob, path)
    } else {
        kind.locus_admits(path)
    }
}

/// Whether the directories `root` + `glob` seats its members in match `path` — the
/// directory reading of an any-depth locus (`**/.claude/skills/*/SKILL.md` keeps its units
/// under every `.claude/skills`). The pattern less its filename and trailing wildcard
/// segments is matched against `path`; a remainder spelling no literal segment
/// (`**/CLAUDE.md` reduces to `**`) names no directory, so it covers none.
fn units_sit_under(root: &str, glob: &str, path: &HarnessRelativePath) -> bool {
    let pattern = crate::kind::locus_pattern(root, glob);
    let mut segments: Vec<&str> = pattern.split('/').collect();
    segments.pop();
    while segments.last().is_some_and(|segment| segment.contains('*')) {
        segments.pop();
    }
    if segments.iter().all(|segment| segment.contains('*')) {
        return false;
    }
    crate::glob::compile_glob(&segments.join("/")).is_some_and(|matcher| matcher.is_match(&**path))
}

/// A `governs.root` reduced to a comparable relative path: leading `./` and any trailing
/// `/` stripped, and a bare `.` (the harness root itself, the `memory` kind's locus)
/// folded to the empty string. Its one consumer is the **directory** branch of
/// [`governs`], whose question — does this kind root at or below this directory? — is a
/// prefix relation between two roots that no glob match can answer.
fn normalize_root(root: &str) -> &str {
    let root = root.trim_start_matches("./").trim_end_matches('/');
    if root == "." { "" } else { root }
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
        kind_governing("skill", ".", "**/.claude/skills/*/SKILL.md")
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
            &BTreeSet::new(),
            0,
        );
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
            &BTreeSet::new(),
            0,
        );
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
            &BTreeSet::new(),
            0,
        );
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
            &BTreeSet::new(),
            0,
        );
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
            &BTreeSet::new(),
            0,
        );
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
            &HarnessRelativePath::new(".claude/agents".to_string()),
            true
        ));
        // A custom kind rooted exactly there governs it.
        let with_agent = BTreeMap::from([(
            "agent".to_string(),
            kind_governing("agent", ".claude/agents", "*.md"),
        )]);
        assert!(governed_by_any_path(
            &with_agent,
            &HarnessRelativePath::new(".claude/agents".to_string()),
            true
        ));
    }

    #[test]
    fn a_file_entry_is_governed_by_a_kind_whose_whole_locus_matches_it() {
        // Neither built-in-shaped kind roots at `.claude` itself, so the file is
        // ungoverned.
        assert!(!governed_by_any_path(
            &builtin_set(),
            &HarnessRelativePath::new(".claude/settings.json".to_string()),
            false
        ));
        // A kind rooted at `.claude` selecting `settings.json` governs the file.
        let with_settings = BTreeMap::from([(
            "settings".to_string(),
            kind_governing("settings", ".claude", "settings.json"),
        )]);
        assert!(governed_by_any_path(
            &with_settings,
            &HarnessRelativePath::new(".claude/settings.json".to_string()),
            false
        ));
        // And a glob carrying an interior segment is matched whole: `prompts/*.md` under
        // `.claude` claims `.claude/prompts/x.md`, never `.claude/notes.md`. Read as a
        // leaf (`*.md`) against a parent that equals the root, the stray was excluded.
        let with_prompts = BTreeMap::from([(
            "prompt".to_string(),
            kind_governing("prompt", ".claude", "prompts/*.md"),
        )]);
        assert!(!governed_by_any_path(
            &with_prompts,
            &HarnessRelativePath::new(".claude/notes.md".to_string()),
            false
        ));
        assert!(governed_by_any_path(
            &with_prompts,
            &HarnessRelativePath::new(".claude/prompts/intro.md".to_string()),
            false
        ));
    }

    #[test]
    fn an_any_depth_skill_locus_governs_its_skills_folder_and_no_sibling() {
        let set = builtin_set();
        let dir = |path: &str| {
            governed_by_any_path(&set, &HarnessRelativePath::new(path.to_string()), true)
        };
        assert!(dir(".claude/skills"));
        assert!(!dir(".claude/prompts"));
        // `memory`'s `**/CLAUDE.md` names no directory, so it covers none.
        let with_memory = BTreeMap::from([(
            "memory".to_string(),
            kind_governing("memory", ".", "**/CLAUDE.md"),
        )]);
        assert!(!governed_by_any_path(
            &with_memory,
            &HarnessRelativePath::new(".claude/prompts".to_string()),
            true
        ));
    }

    #[test]
    fn the_memory_kinds_any_depth_locus_governs_the_claude_dir_copy() {
        // `memory` roots at `.` with an any-depth `**/CLAUDE.md` glob, and
        // `.claude/CLAUDE.md` is a documented location for a project memory file — so the
        // scan must read it as checked, never a stray. The leaf compare could not: the
        // root folded to the empty string and the entry's parent is `.claude`.
        let with_memory = BTreeMap::from([(
            "memory".to_string(),
            kind_governing("memory", ".", "**/CLAUDE.md"),
        )]);
        assert!(governed_by_any_path(
            &with_memory,
            &HarnessRelativePath::new(".claude/CLAUDE.md".to_string()),
            false
        ));
        assert!(!governed_by_any_path(
            &with_memory,
            &HarnessRelativePath::new(".claude/notes.md".to_string()),
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
            &BTreeSet::new(),
            0,
        );

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
            &BTreeSet::new(),
            0,
        );

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
            &BTreeSet::new(),
            0,
        );

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
            &BTreeSet::new(),
            0,
        );

        assert!(
            diagnostics
                .iter()
                .all(|d| !(d.rule == UNCLAIMED_RULE && d.artifact == ".claude/ignored-stray.md")),
            "a gitignored stray must never fire, got: {diagnostics:#?}"
        );
    }

    #[test]
    fn the_stray_scan_consults_gits_declarations_and_no_other_ignore_source() {
        // The two halves that distinguish the shared ignore-source policy from `ignore`'s
        // defaults: git's declarations are honored without a `.git` to anchor them, and
        // ripgrep's own `.ignore` source is off — a tree's preference for its own tools
        // is no declaration of non-authorship.
        let root = tmpdir("stray-scan-ignore-sources");
        std::fs::create_dir_all(root.join(".claude")).unwrap();
        std::fs::write(root.join(".claude/.gitignore"), "git-named.md\n").unwrap();
        std::fs::write(root.join(".claude/.ignore"), "rg-named.md\n").unwrap();
        std::fs::write(root.join(".claude/git-named.md"), "").unwrap();
        std::fs::write(root.join(".claude/rg-named.md"), "").unwrap();

        let diagnostics = check(
            &root,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeSet::new(),
            0,
        );

        let strays: Vec<&str> = diagnostics
            .iter()
            .filter(|d| d.rule == UNCLAIMED_RULE)
            .map(|d| d.artifact.as_str())
            .collect();
        assert!(
            strays.contains(&".claude/rg-named.md"),
            "a stray named only by `.ignore` must still fire, got: {strays:?}"
        );
        assert!(
            !strays.contains(&".claude/git-named.md"),
            "a gitignored stray must never fire, got: {strays:?}"
        );
    }
}
