//! Harness discovery — the sole member extractor the gate and `emit` both ride.
//!
//! The discovery walk (`discover_kind_units`/`discover_builtin`) is the sole member
//! extractor the gate and `emit` both ride.
//! The `init`/`lift` on-ramp verbs that once wrote
//! an in-place `[[member]]` table over members in place retired with the `[[member]]`
//! codec (`CODEC-RETIRE`) — `install` is the
//! on-ramp going forward; a trunk gap between the two is an
//! accepted clean-slate cost (John, 2026-07-06).
//!
//! Keystone invariant (`.claude/rules/rust.md`): idempotence. It holds because
//! every write is content-derived, name-sorted, and overwrites in place.

use std::cell::{Cell, OnceCell};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use ignore::WalkBuilder;

use crate::kind::{Commitment, CustomKind, Governs, UnitShape};

/// Whether a walk lets a committed local-locus kind's `governs` declaration override
/// discovery's two presumptions — the repository's ignore rules and the workspace skip.
/// The declaration is reviewed while the documents under it are not, so it is itself the
/// authorship claim over them; a walk that presumed otherwise would find a real
/// per-machine document only by accident.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalOverride {
    /// The declaration governs: a local-locus kind's documents are discovered though the
    /// repo ignores them or they sit under the workspace. Every read-side walk — the
    /// gate's and the manifest face's — takes this, so a local member's rows derive
    /// rather than silently failing to.
    Honored,
    /// The presumptions stay whole whatever a kind declares. Adoption's walk takes this:
    /// it converts what it finds into a committed member module, and a local document is
    /// never that.
    Withheld,
}

/// A run's shared discovery cache over one harness. The ignore-honoring walk
/// ([`discoverable_paths`]) is a pure function of the harness and the `local_governs`
/// flavor, and only two flavors exist, so it is walked at most once per flavor here and
/// reused across every kind and nested-file host a run discovers. Without it a run
/// re-walks the consumer's whole tree per kind and twice per nested-file host — the
/// dominant cost at consumer scale. The harness disk is stable across a run, so the
/// memoized set is the same one a re-walk would produce.
///
/// Similarly, the claimed-path set a declared kind's own `governs` locus claims is a
/// pure function of the kind set and the `local_governs` preference, computed once
/// per run and cached here to avoid recomputation across every nested-file kind's
/// discovery pass.
pub struct Discovery<'a> {
    harness: &'a Path,
    /// The `local_governs = true` flavor: a local-locus kind's own walk, with the ignore
    /// rules and the workspace skip waived.
    local: OnceCell<Discoverable>,
    /// The `local_governs = false` flavor: every committed kind's walk, both presumptions
    /// standing.
    standard: OnceCell<Discoverable>,
    /// The claimed paths for the `LocalOverride::Honored` flavor.
    claimed_honored: OnceCell<BTreeSet<PathBuf>>,
    /// The claimed paths for the `LocalOverride::Withheld` flavor.
    claimed_withheld: OnceCell<BTreeSet<PathBuf>>,
}

impl<'a> Discovery<'a> {
    /// A fresh cache over `harness` — no walk runs until a flavor is first consulted.
    #[must_use]
    pub fn new(harness: &'a Path) -> Self {
        Self {
            harness,
            local: OnceCell::new(),
            standard: OnceCell::new(),
            claimed_honored: OnceCell::new(),
            claimed_withheld: OnceCell::new(),
        }
    }

    /// The harness root this cache walks — the base a `governs` root and the bare-skill
    /// path join against.
    #[must_use]
    pub fn harness(&self) -> &Path {
        self.harness
    }

    /// The discoverable tree for the `local_governs` flavor, walking it on first use
    /// and returning the memoized index thereafter.
    fn discoverable(&self, local_governs: bool) -> &Discoverable {
        let cell = if local_governs {
            &self.local
        } else {
            &self.standard
        };
        cell.get_or_init(|| discoverable_paths(self.harness, local_governs))
    }

    /// The claimed-path set a declared kind's own `governs` locus claims, for the given
    /// override flavor. Computed on first use across the `kinds` set and cached here to
    /// avoid recomputation across every nested-file kind's discovery pass.
    fn declared_governed_paths(
        &self,
        kinds: &BTreeMap<String, CustomKind>,
        over: LocalOverride,
    ) -> &BTreeSet<PathBuf> {
        let cell = match over {
            LocalOverride::Honored => &self.claimed_honored,
            LocalOverride::Withheld => &self.claimed_withheld,
        };
        cell.get_or_init(|| {
            let mut claimed = BTreeSet::new();
            for kind in kinds.values() {
                if let Some(governs) = kind.governs.as_ref() {
                    claimed.extend(discover_kind_files(self, kind, governs, over));
                }
            }
            claimed
        })
    }

    /// How many of the two flavors have actually been walked — the single-walk-per-flavor
    /// property a shared-cache test asserts against (`<= 2`, one per flavor consulted).
    #[cfg(test)]
    fn flavors_walked(&self) -> usize {
        usize::from(self.local.get().is_some()) + usize::from(self.standard.get().is_some())
    }
}

thread_local! {
    /// Per-thread count of discovery tree-walks: every [`discoverable_paths`] call bumps
    /// it. The walk is single-threaded on its caller's thread, so this counts one run's
    /// walks in isolation — a concurrent run on another test thread cannot perturb it. A
    /// whole run shares one [`Discovery`], so its walks memoize per flavor and the count's
    /// delta across a run is exactly the number of flavors that run consulted, each walked
    /// once. A second `Discovery` built mid-run walks off its own cache rather than the
    /// run's threaded one, so the delta overshoots: the run-level count-pin's observable,
    /// decidable and machine-independent, never a wall-clock threshold.
    static WALKS: Cell<usize> = const { Cell::new(0) };
}

/// This thread's discovery walk count. Read before and after a check run and compare the
/// delta to the flavors that run consults, pinning that whole-input discovery work
/// computes once per flavor per run rather than per kind or per call site.
#[must_use]
pub fn walk_count() -> usize {
    WALKS.with(Cell::get)
}

/// Discover a built-in `kind`'s source files at whichever locus it declares — the same
/// data-driven scan a custom kind would get, so `skill`/`rule` are no longer hardwired
/// paths (the emit face's locus is the read face's scan root).
/// The `skill` locus (`.claude/skills` + `*/SKILL.md`) resolves through the generalized
/// subdir glob; `rule`'s (`.claude/rules` + `*.md`) is flat. Yields the member source
/// *files* — for a skill the `SKILL.md`, not its directory.
///
/// The parsed `kind` is threaded in from the caller's `definitions()` set, never
/// re-resolved by bare `name` — the scan reads whatever locus the caller hands it,
/// independent of the embedded set's own lookup.
///
/// The bare-harness-is-a-skill case — a `<harness>/SKILL.md`, a project root that is
/// itself a skill — is Claude Code's own convention, outside the `.claude/skills`
/// locus the `governs` scan covers, so it is layered on for the `skill` kind only.
///
/// A kind governing no locus — a **nested file** kind, whose members sit under their
/// host's unit — is discovered off `kinds` instead ([`discover_nested_file`]): the two
/// halves of its locus are the host's, so the declared set the host lives in is what the
/// scan keys on.
pub(crate) fn discover_builtin(
    disc: &Discovery,
    kind: &CustomKind,
    kinds: &BTreeMap<String, CustomKind>,
    over: LocalOverride,
) -> Vec<PathBuf> {
    match &kind.governs {
        Some(governs) => discover_kind_files(disc, kind, governs, over),
        None => discover_nested_file(disc, kind, kinds, over)
            .into_iter()
            .map(|unit| unit.file)
            .collect(),
    }
}

/// One discovered nested file member: the child's source file, plus the host unit
/// directory its path composed under. The file alone cannot name that directory — a
/// `*`-free pattern may seat the child levels below it — and the id a `file` unit shape
/// folds is the file's placement under it, so both halves travel together.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NestedFileUnit {
    /// The host member's unit directory.
    pub host_unit: PathBuf,
    /// The child's source file, under `host_unit` at the host template's pattern.
    pub file: PathBuf,
}

/// Discover a nested file `kind`'s members on `harness`: under each host member's unit,
/// every file matching the host template's pattern for this kind. The child kind carries
/// neither half — the pattern is the host kind's declared `templates` fact and the units
/// are the host kind's own `governs` scan — so `kinds`, the declared set keyed by bare
/// name, is what the host is read out of. This is the read side of the composition emit
/// writes a child's projection at: host unit joined with pattern, and nothing else.
///
/// A host qualifies only where it owns a directory unit at a `governs` locus: a template's
/// pattern is relative to its host's unit, and a lone file has no interior to seat a child
/// in. A host's entry file is that host's own member and never its own child, so a pattern
/// matching it collects nothing.
///
/// A child sits inside its host's unit, so the host's commitment class is what decides
/// whether `over` lets discovery's presumptions be overridden here: the child kind
/// governs no locus of its own and so declares no class of its own.
pub fn discover_nested_file(
    disc: &Discovery,
    kind: &CustomKind,
    kinds: &BTreeMap<String, CustomKind>,
    over: LocalOverride,
) -> Vec<NestedFileUnit> {
    // A path a declared kind's own `governs` locus claims has one home — that kind's
    // member — so it is carved out of every host template's discovery here, at the
    // single point one path is decided. Without the carve a declared exact-path kind and
    // a host template both materialize the path: a phantom twin the coverage, `explain`,
    // and `degree` consumers would each then have to un-see. Position stays decidable at
    // this one seam instead.
    let claimed = disc.declared_governed_paths(kinds, over);
    let mut found = Vec::new();
    for host in kinds.values() {
        let (Some(pattern), Some(governs)) =
            (file_template(host, &kind.name), host.governs.as_ref())
        else {
            continue;
        };
        if host.unit_shape != Some(UnitShape::Directory) {
            continue;
        }
        let discoverable = disc.discoverable(local_governs(host, over));
        let root = crate::path::normalize_path(&disc.harness().join(&governs.root));
        for entry in discover_kind_files(disc, host, governs, over) {
            let Some(host_unit) = unit_dir(&root, &entry) else {
                continue;
            };
            for file in scan_locus(&host_unit, pattern, discoverable) {
                if file != entry && !claimed.contains(&file) {
                    found.push(NestedFileUnit {
                        host_unit: host_unit.clone(),
                        file,
                    });
                }
            }
        }
    }
    found.sort_by(|a, b| a.file.cmp(&b.file));
    found
}

/// The path pattern `host` templates `child`'s file layer at, if it declares one — the
/// child's half of the locus, owned by the host. A template carrying no path is an
/// embedded layer, whose children own no file to find.
fn file_template<'a>(host: &'a CustomKind, child: &str) -> Option<&'a str> {
    host.templates
        .iter()
        .find(|template| template.kind == child)
        .and_then(|template| template.path.as_deref())
}

/// The unit directory a discovered entry file sits in: the one level below the kind's
/// `governs` root, where a directory-unit member's `<root>/<name>/` is composed. [`None`]
/// for a file the root does not contain (a bare harness that is itself a skill) or one
/// lying loose at the root, neither of which owns an interior a template addresses.
fn unit_dir(root: &Path, entry: &Path) -> Option<PathBuf> {
    let relative = entry.strip_prefix(root).ok()?;
    let mut components = relative.components();
    let name = components.next()?;
    components.next()?;
    Some(root.join(name))
}

/// Discover a `kind`'s member source files under `harness`, matching an explicit
/// `governs` locus — the generalized scan [`discover_kind_units`] runs, plus any
/// bare-root file declared in the kind's `bare_root_file` field (e.g., a
/// `<harness>/SKILL.md`, a harness that is itself a skill). Decoupled from the kind's
/// own [`CustomKind::governs`] so a caller can walk a *different* declared locus for the
/// same kind — the committed lock's own kind-fact row on an adopted harness, the
/// kind's embedded default otherwise (the built-in lock) — while the bare-root fact
/// is a declared property of the kind itself and applies wherever its locus is walked
/// from. [`discover_builtin`] is the thin caller that always walks the kind's own
/// governs.
///
/// `over` decides whether the kind's own commitment class may override discovery's
/// presumptions for this walk; the `kind` is what carries that class, which is why the
/// generalized scan cannot decide it off `governs` alone.
pub fn discover_kind_files(
    disc: &Discovery,
    kind: &CustomKind,
    governs: &Governs,
    over: LocalOverride,
) -> Vec<PathBuf> {
    let mut files = discover_kind_units(disc, governs, local_governs(kind, over));
    if let Some(bare_file) = &kind.bare_root_file {
        let bare = disc.harness().join(bare_file);
        if bare.is_file() {
            files.push(bare);
            // Re-sort so the bare root file lands in name order beside the children.
            files.sort();
        }
    }
    files
}

/// Whether `kind`'s walk lets its `governs` declaration override discovery's
/// presumptions — a local locus under a walk that honors the override, and nothing else.
/// The `governs` locus alone cannot answer it: the commitment class is the kind's column.
fn local_governs(kind: &CustomKind, over: LocalOverride) -> bool {
    over == LocalOverride::Honored && kind.commitment == Some(Commitment::Local)
}

/// Discover a kind's units under `<harness>/<governs.root>/` by matching the
/// `governs.glob` against paths beneath the root. The glob may be **flat** (`*.md` —
/// immediate files), carry a **fixed subdirectory** segment (`*/SKILL.md` — a file
/// inside each matching immediate child), or open with the **any-depth** wildcard
/// `**` (`**/AGENTS.md` — the named file at every level of a nested hierarchy); the
/// one scanner resolves all three, so it serves every custom kind and the built-in
/// `skill`/`rule` loci alike. Non-matching entries are skipped, and a missing root
/// yields an empty list (a declared kind whose corpus does not exist on this
/// harness). Data-driven discovery — the locus is the kind's own `governs`
/// declaration, never a hardwired path.
fn discover_kind_units(disc: &Discovery, governs: &Governs, local_governs: bool) -> Vec<PathBuf> {
    // A member is authored content; an ignored file is by declaration not authored here,
    // so discovery sees only what the repo's ignore rules leave in — else a `**` glob
    // would import a vendored dep's memory file. A local-locus kind's own walk is the one
    // exception, and its `governs` says so: `local_governs` carries that scope in.
    // Resolved off the harness (repo) root so a root `.gitignore` governs every kind's
    // walk, whatever its `governs.root` depth. Shared per flavor across the run so N kinds
    // cost one walk, not N ([`Discovery`]).
    let discoverable = disc.discoverable(local_governs);
    scan_locus(
        &disc.harness().join(&governs.root),
        &governs.glob,
        discoverable,
    )
}

/// The scan itself: every file under `root` matching `glob`, deterministically ordered.
/// Split from [`discover_kind_units`] so a nested file child's scan under each host unit
/// rides the same matcher and the same already-computed `discoverable` index — one scanner
/// serves every kind's locus, host and child alike.
fn scan_locus(root: &Path, glob: &str, discoverable: &Discoverable) -> Vec<PathBuf> {
    // A glob is a `/`-separated segment list: the final segment matches files, each
    // earlier one a subdirectory to descend into — a `**` segment descending any
    // number of levels. `split` always yields at least one segment.
    let segments: Vec<&str> = glob.split('/').collect();
    // The index is keyed by normalized paths, so a `.`-rooted locus (`root = "."`) resolves
    // to the harness root the walk keyed its top-level entries under.
    let root = crate::path::normalize_path(root);
    let mut files = Vec::new();
    collect_glob(&root, &segments, discoverable, &mut files);
    // A `**` reaches one file by exactly one path, but the index yields children in walk
    // order; sort for deterministic processing.
    files.sort();
    files
}

/// Collect every discoverable file whose path matches the remaining glob `segments`,
/// reading the tree from the shared `discoverable` index rather than the filesystem. The
/// head segment selects children at this level; if it is the last, matching **files** are
/// collected, otherwise matching **subdirectories** are descended. A `**` head is the
/// any-depth wildcard — it matches zero or more directory levels, so a nested nearest-wins
/// hierarchy (the `CLAUDE.md` memory nesting) is discovered at every level, not just the
/// fixed glob depth. A `dir` the index holds no children for contributes nothing — a
/// subdir glob whose intermediate level is absent, or a locus that does not exist on this
/// harness, both resolve to no units.
///
/// Every child the index yields is already discoverable (`.git/` excluded, `.gitignore`
/// respected, nested governed roots fenced), so no membership filter is applied here — the
/// shared walk enforced it once, and the per-child file-vs-directory tag it recorded is
/// what decides collect-vs-descend.
fn collect_glob(
    dir: &Path,
    segments: &[&str],
    discoverable: &Discoverable,
    out: &mut Vec<PathBuf>,
) {
    let Some((segment, rest)) = segments.split_first() else {
        // `**` recurses with the same segments, so it can bottom out at an empty list
        // (a trailing `**` with nothing left to match): nothing more to collect here.
        return;
    };
    if *segment == "**" {
        // Zero levels: match the remaining segments right at this level, so
        // `**/CLAUDE.md` picks up a `CLAUDE.md` directly under the root too.
        collect_glob(dir, rest, discoverable, out);
        // One-or-more levels: descend into every subdirectory carrying the `**`, so
        // each nested file is reached by exactly one path (no double-collection).
        for child in discoverable.children(dir) {
            if child.is_dir {
                collect_glob(&child.path, segments, discoverable, out);
            }
        }
        return;
    }
    for child in discoverable.children(dir) {
        let Some(name) = child.path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if !crate::glob::compile_glob(segment).is_some_and(|matcher| matcher.is_match(name)) {
            continue;
        }
        if rest.is_empty() {
            if !child.is_dir {
                out.push(child.path.clone());
            }
        } else if child.is_dir {
            collect_glob(&child.path, rest, discoverable, out);
        }
    }
}

/// The tree of paths under `harness` a walk for a kind whose locus `local_governs` may
/// see, indexed as each discoverable directory's direct children. Two presumptions prune
/// it, and the flag waives **both, together**: the
/// repo's ignore rules — an ignored file is by declaration not authored here — and the
/// surface workspace (`.temper/`), which holds temper's own modules and lock and, being
/// committed rather than gitignored, would otherwise enter the set on its own. A
/// local-locus kind's `governs` is the reviewed claim over its unreviewed documents, so
/// for its walk neither presumption stands: a real per-machine document is always
/// gitignored, and one may sit under the workspace. The waiver needs no path scoping —
/// the set is only ever consulted under the walking kind's own locus.
///
/// Two fences are not presumptions and hold for every walk: `.git/`, and a **nested
/// governed root** — a subdirectory below the harness root carrying its own
/// `.temper/lock.toml` is its own corpus, so the walk never descends it. The harness
/// root's own lock must not self-fence, so that skip keys off walk depth, not the name.
///
/// Built with ripgrep's `ignore` engine so nested `.gitignore` files, negation, and
/// precedence are honored rather than hand-rolled. Only git's own declaration counts:
/// the machine-global and ripgrep-specific (`.ignore`) sources are off, and parent
/// directories above the harness are not consulted — the harness is the per-project
/// boundary. `require_git` is off so a `.gitignore` is honored even when the harness is
/// not itself a git checkout (a sub-tree, or a test fixture). Paths are normalized so a
/// `.`-rooted `governs` (`root = "."`) compares equal to the walk's harness-relative
/// entries.
fn discoverable_paths(harness: &Path, local_governs: bool) -> Discoverable {
    WALKS.with(|w| w.set(w.get() + 1));
    let mut children: BTreeMap<PathBuf, Vec<DiscoverableChild>> = BTreeMap::new();
    let walk = WalkBuilder::new(harness)
        .hidden(false) // `.claude/` is a dotdir the harness lives in — never hide it.
        .parents(false)
        .ignore(false)
        .git_global(false)
        .git_ignore(!local_governs)
        .git_exclude(!local_governs)
        .require_git(false)
        .filter_entry(move |entry| {
            if entry.file_name() == OsStr::new(".git") {
                return false;
            }
            if !local_governs && entry.file_name() == OsStr::new(crate::WORKSPACE_DIR) {
                return false;
            }
            // Depth 0 is the harness root itself: its own `.temper/lock.toml` governs
            // this walk and must never fence it. Any deeper directory carrying one is a
            // nested governed root — its members belong to its own corpus, so the walk
            // stops here rather than collecting them for the parent.
            !(entry.depth() > 0
                && entry
                    .file_type()
                    .is_some_and(|file_type| file_type.is_dir())
                && is_governed_root(entry.path()))
        })
        .build();
    // A walk error (an unreadable entry) drops that entry rather than aborting discovery.
    // The file-vs-directory tag rides `entry.file_type()` — the one fact the scan needs
    // that a bare path set lacks — recorded here so the scan never re-`stat`s a candidate.
    for entry in walk.flatten() {
        let path = crate::path::normalize_path(entry.path());
        let is_dir = entry
            .file_type()
            .is_some_and(|file_type| file_type.is_dir());
        if let Some(parent) = path.parent().map(Path::to_path_buf) {
            children
                .entry(parent)
                .or_default()
                .push(DiscoverableChild { path, is_dir });
        }
    }
    Discoverable { children }
}

/// Whether `dir` carries its own `.temper/lock.toml` — the mark of an independently
/// governed harness whose members are its own corpus, not the enclosing walk's.
fn is_governed_root(dir: &Path) -> bool {
    dir.join(crate::WORKSPACE_DIR)
        .join(crate::LOCK_FILENAME)
        .is_file()
}

/// The shared discoverable tree for one flavor: the direct children of every discoverable
/// directory, keyed by the parent's normalized path. The ignore-honoring walk records it
/// once ([`discoverable_paths`]) and every kind's glob scan reads its matches from it, so N
/// kinds cost one walk rather than N filesystem re-walks. Each child carries the
/// file-vs-directory tag the walk already knew (`entry.file_type()`): a glob's final
/// segment collects a child only if it is a file and descends it only if a directory, so
/// the tag is load-bearing and read here rather than re-`stat`ed per candidate.
struct Discoverable {
    children: BTreeMap<PathBuf, Vec<DiscoverableChild>>,
}

impl Discoverable {
    /// The direct children the walk recorded under `dir` — empty for a path the index holds
    /// none for (a missing locus, or a leaf that seats nothing).
    fn children(&self, dir: &Path) -> &[DiscoverableChild] {
        self.children.get(dir).map_or(&[], Vec::as_slice)
    }
}

/// One entry in a directory's child list: its normalized path and whether it is a
/// directory — the two facts the glob scan needs to match a name and decide
/// collect-vs-descend, both known at walk time.
struct DiscoverableChild {
    path: PathBuf,
    is_dir: bool,
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    use crate::builtin_kind;
    use crate::drift::TemplateRow;
    use crate::kind::Extraction;
    use crate::test_support::tmpdir;

    /// A kind set declaring no template at all: every `governs` scan below is keyed on its
    /// own kind's locus, and the set the nested file arm reads a host out of plays no part.
    fn no_hosts() -> BTreeMap<String, CustomKind> {
        BTreeMap::new()
    }

    const COORDINATE: &str = "---\n\
name: coordinate\n\
description: Use when driving a complex task across a team of agents.\n\
license: \"MIT\"\n\
allowed-tools: [\"Task\", \"Read\"]\n\
---\n\
# Coordinate\n\
\n\
See PLAYBOOK.md for the full reference.   \n\
No trailing newline here.";

    const DEMO: &str = "---\n\
name: demo\n\
description: A second skill so the roll-up carries more than one entry.\n\
---\n\
# Demo body\n";

    const PLAYBOOK: &[u8] = b"# Playbook\n\nStep one.\n\x00binary-ish\xff tail\n";
    const SCRIPT: &[u8] = b"#!/bin/sh\necho coordinating\n";

    /// A rule with `paths:` frontmatter and an unknown Cursor key, plus a body
    /// whose trailing bytes must survive intact.
    const RUST_RULE: &str = "---\n\
paths:\n\
  - \"src/**/*.rs\"\n\
description: A Cursor key Claude Code ignores — preserved, not dropped.\n\
---\n\
# Rust conventions\n\
\n\
Prefer a clone over a lifetime fight.   \n\
Last line, no newline.";

    /// A rule with no frontmatter at all — the `collaboration.md` shape.
    const COLLAB_RULE: &str = "# Collaboration\n\nPushback is the point.\n";

    /// Build a harness with two skills under `.claude/skills/` and two rules under
    /// `.claude/rules/`; `coordinate` carries a companion markdown file and a
    /// nested script. The two kinds coexist so one import covers both.
    fn write_fixture_harness(root: &Path) {
        let coordinate = root.join(".claude").join("skills").join("coordinate");
        fs::create_dir_all(coordinate.join("scripts")).unwrap();
        fs::write(coordinate.join("SKILL.md"), COORDINATE).unwrap();
        fs::write(coordinate.join("PLAYBOOK.md"), PLAYBOOK).unwrap();
        fs::write(coordinate.join("scripts").join("run.sh"), SCRIPT).unwrap();

        let demo = root.join(".claude").join("skills").join("demo");
        fs::create_dir_all(&demo).unwrap();
        fs::write(demo.join("SKILL.md"), DEMO).unwrap();

        let rules = root.join(".claude").join("rules");
        fs::create_dir_all(&rules).unwrap();
        fs::write(rules.join("rust.md"), RUST_RULE).unwrap();
        fs::write(rules.join("collaboration.md"), COLLAB_RULE).unwrap();
    }

    #[test]
    fn builtin_discovery_keys_off_the_embedded_kind_governs() {
        // Discovery is driven by the embedded `skill`/`rule` kinds' declared `governs`,
        // not a hardwired path: the skill `*/SKILL.md` subdir glob and the rule `*.md`
        // flat glob both resolve through the one generalized scanner.
        let harness = tmpdir("gov-src");
        write_fixture_harness(&harness);

        let skill_kind = builtin_kind::definition("skill").unwrap();
        let rule_kind = builtin_kind::definition("rule").unwrap();

        // The skill locus (`.claude/skills` + `*/SKILL.md`) yields the `SKILL.md`
        // files themselves — the subdir glob descended one level.
        let skills = discover_builtin(
            &Discovery::new(&harness),
            &skill_kind,
            &no_hosts(),
            LocalOverride::Honored,
        );
        assert_eq!(
            skills,
            vec![
                harness.join(".claude/skills/coordinate").join("SKILL.md"),
                harness.join(".claude/skills/demo").join("SKILL.md"),
            ]
        );

        // The rule locus (`.claude/rules` + `*.md`) is flat — immediate `*.md` files.
        let rules = discover_builtin(
            &Discovery::new(&harness),
            &rule_kind,
            &no_hosts(),
            LocalOverride::Honored,
        );
        assert_eq!(
            rules,
            vec![
                harness.join(".claude/rules/collaboration.md"),
                harness.join(".claude/rules/rust.md"),
            ]
        );
    }

    #[test]
    fn discover_builtin_scans_the_passed_kind_never_re_resolving_by_name() {
        // Discovery reads the `governs` of the kind it is *handed*, never re-resolving
        // its bare `name` against the embedded set. Proven with a synthetic `memory`
        // kind carrying a *different* locus than the real embedded `memory` kind
        // (`mem/*.md` here vs. `**/CLAUDE.md`): a by-name re-resolution would scan the
        // embedded locus instead, so finding the member at this kind's own locus proves
        // the parsed kind is threaded through untouched.
        let harness = tmpdir("threaded-discovery");
        fs::create_dir_all(harness.join("mem")).unwrap();
        fs::write(harness.join("mem").join("CLAUDE.md"), "# root\n").unwrap();

        let memory = CustomKind::new(
            "memory",
            Governs {
                root: "mem".to_string(),
                glob: "*.md".to_string(),
            },
            Extraction::new(Vec::new()),
        );

        let found = discover_builtin(
            &Discovery::new(&harness),
            &memory,
            &no_hosts(),
            LocalOverride::Honored,
        );
        assert_eq!(found, vec![harness.join("mem").join("CLAUDE.md")]);
    }

    #[test]
    fn a_subdir_glob_descends_one_level_and_skips_a_dir_without_the_file() {
        // The generalized `governs` scanner resolves a `*/FILE.md` subdir glob for any
        // kind, not just the built-in skill: it descends each immediate child and
        // collects the named file, skipping a child that lacks it and a loose file at
        // the root (which matches no subdirectory).
        let harness = tmpdir("subdir-glob-src");
        let root = harness.join("things");
        fs::create_dir_all(root.join("alpha")).unwrap();
        fs::create_dir_all(root.join("beta")).unwrap();
        fs::create_dir_all(root.join("empty")).unwrap();
        fs::write(root.join("alpha").join("THING.md"), "a\n").unwrap();
        fs::write(root.join("beta").join("THING.md"), "b\n").unwrap();
        // Noise: a subdir without the file, and a loose root file the glob can't reach.
        fs::write(root.join("empty").join("other.md"), "skip\n").unwrap();
        fs::write(root.join("THING.md"), "root, unreachable via */\n").unwrap();

        let governs = Governs {
            root: "things".to_string(),
            glob: "*/THING.md".to_string(),
        };
        let found = discover_kind_units(&Discovery::new(&harness), &governs, false);
        assert_eq!(
            found,
            vec![
                root.join("alpha").join("THING.md"),
                root.join("beta").join("THING.md"),
            ]
        );
    }

    /// A `dial` kind governing `.claude/local/*.md`, `local` where `commitment` says so —
    /// the synthetic stand-in the two walks below are driven with. No embedded kind
    /// declares the class yet, so the `Withheld` fence install passes is only falsifiable
    /// against a kind built here.
    fn dial_kind(commitment: Option<Commitment>) -> CustomKind {
        let mut kind = CustomKind::new(
            "dial",
            Governs {
                root: ".claude/local".to_string(),
                glob: "*.md".to_string(),
            },
            Extraction::new(Vec::new()),
        );
        kind.commitment = commitment;
        kind
    }

    /// A harness whose `.gitignore` names the dial locus, carrying one document under it —
    /// a real per-machine document's shape, which is always an ignored one.
    fn ignored_dial_harness(slug: &str) -> PathBuf {
        let harness = tmpdir(slug);
        fs::create_dir_all(harness.join(".claude").join("local")).unwrap();
        fs::write(harness.join(".gitignore"), ".claude/local/\n").unwrap();
        fs::write(
            harness.join(".claude").join("local").join("dial.md"),
            "mode: advisory\n",
        )
        .unwrap();
        harness
    }

    #[test]
    fn a_withheld_walk_keeps_the_presumptions_whole_for_a_local_kind() {
        // The seam install's adoption walk rides: it converts what it finds into a
        // committed member module, so the override the read side honors is withheld there
        // and the ignore rules stand whatever the kind declares. The `Honored` half is the
        // falsifier — without it the assertion below would hold for a walk that had simply
        // failed to find anything.
        let harness = ignored_dial_harness("dial-withheld");
        let local = dial_kind(Some(Commitment::Local));

        let adopted = discover_builtin(
            &Discovery::new(&harness),
            &local,
            &no_hosts(),
            LocalOverride::Withheld,
        );
        assert_eq!(adopted, Vec::<PathBuf>::new());

        let read = discover_builtin(
            &Discovery::new(&harness),
            &local,
            &no_hosts(),
            LocalOverride::Honored,
        );
        assert_eq!(
            read,
            vec![harness.join(".claude").join("local").join("dial.md")]
        );
    }

    #[test]
    fn an_honored_walk_overrides_the_presumptions_for_a_local_kind_and_no_other() {
        // The override is the kind's own commitment class, not the walk's mode: the same
        // ignored document under the same locus stays invisible to a committed kind, whose
        // members' bytes are reviewed and so are never ignored ones.
        let harness = ignored_dial_harness("dial-committed");

        let committed = discover_builtin(
            &Discovery::new(&harness),
            &dial_kind(None),
            &no_hosts(),
            LocalOverride::Honored,
        );
        assert_eq!(committed, Vec::<PathBuf>::new());
    }

    #[test]
    fn discover_builtin_finds_a_bare_harness_that_is_itself_a_skill() {
        // A `<harness>` whose own SKILL.md makes it a skill dir, with no skills/ — the
        // real bare-skill-repo shape, not a tmpdir artifact.
        let harness = tmpdir("bare-src").join("demo");
        fs::create_dir_all(&harness).unwrap();
        fs::write(harness.join("SKILL.md"), DEMO).unwrap();

        let skill_kind = builtin_kind::definition("skill").unwrap();
        let found = discover_builtin(
            &Discovery::new(&harness),
            &skill_kind,
            &no_hosts(),
            LocalOverride::Honored,
        );
        assert_eq!(found, vec![harness.join("SKILL.md")]);
    }

    #[test]
    fn a_synthetic_non_skill_kind_declaring_bare_root_file_discovers_it() {
        // The bare-root fact is a declared field on any kind, not a hardwired mechanism
        // for `skill` alone: a synthetic `reference` kind declaring `bare_root_file =
        // Some("REF.md")` discovers a `<harness>/REF.md` exactly as skill discovers its
        // `SKILL.md`, proving the mechanism is generalized to any kind that declares it.
        let harness = tmpdir("synthetic-bare-root");
        fs::create_dir_all(&harness).unwrap();
        fs::write(harness.join("REF.md"), "# Reference\n").unwrap();

        let mut reference_kind = CustomKind::new(
            "reference",
            Governs {
                root: "refs".to_string(),
                glob: "*.md".to_string(),
            },
            Extraction::new(Vec::new()),
        );
        reference_kind.bare_root_file = Some("REF.md".to_string());

        let found = discover_builtin(
            &Discovery::new(&harness),
            &reference_kind,
            &no_hosts(),
            LocalOverride::Honored,
        );
        assert_eq!(found, vec![harness.join("REF.md")]);
    }

    #[test]
    fn discover_fences_a_nested_governed_root_but_not_the_harness_root() {
        // The memory kind's `**/CLAUDE.md` root=`.` walk collects the harness root's own
        // memory file but stops at a vendored sub-harness carrying its own
        // `.temper/lock.toml`: that subdir is its own corpus, never the parent's. The
        // harness root's own lock must not self-fence — its member is still discovered.
        let harness = tmpdir("nested-governed-root");

        // The parent harness: its own `.temper/lock.toml` (must not self-fence) plus a
        // root memory file.
        fs::create_dir_all(harness.join(crate::WORKSPACE_DIR)).unwrap();
        fs::write(
            harness
                .join(crate::WORKSPACE_DIR)
                .join(crate::LOCK_FILENAME),
            "",
        )
        .unwrap();
        fs::write(harness.join("CLAUDE.md"), "# root memory\n").unwrap();

        // A vendored sub-harness with its own governed root and its own memory file —
        // fenced from the parent's walk.
        let vendored = harness.join("examples").join("sub-harness");
        fs::create_dir_all(vendored.join(crate::WORKSPACE_DIR)).unwrap();
        fs::write(
            vendored
                .join(crate::WORKSPACE_DIR)
                .join(crate::LOCK_FILENAME),
            "",
        )
        .unwrap();
        fs::write(vendored.join("CLAUDE.md"), "# vendored memory\n").unwrap();

        let memory = CustomKind::new(
            "memory",
            Governs {
                root: ".".to_string(),
                glob: "**/CLAUDE.md".to_string(),
            },
            Extraction::new(Vec::new()),
        );

        let found = discover_builtin(
            &Discovery::new(&harness),
            &memory,
            &no_hosts(),
            LocalOverride::Honored,
        );
        assert_eq!(found, vec![harness.join("CLAUDE.md")]);
    }

    #[test]
    fn discover_builtin_skips_non_skill_dirs_and_files() {
        let harness = tmpdir("skip-src");
        write_fixture_harness(&harness);
        // Noise that must be ignored: a loose file and a dir without SKILL.md.
        fs::write(
            harness.join(".claude").join("skills").join("README.md"),
            "not a skill\n",
        )
        .unwrap();
        fs::create_dir_all(harness.join(".claude").join("skills").join("empty")).unwrap();

        let skill_kind = builtin_kind::definition("skill").unwrap();
        let found = discover_builtin(
            &Discovery::new(&harness),
            &skill_kind,
            &no_hosts(),
            LocalOverride::Honored,
        );
        assert_eq!(
            found,
            vec![
                harness.join(".claude/skills/coordinate").join("SKILL.md"),
                harness.join(".claude/skills/demo").join("SKILL.md"),
            ]
        );
    }

    #[test]
    fn discover_builtin_routes_a_nested_file_kind_through_its_hosts_template() {
        // The locus dispatch install's own report is built on: a kind declaring no governs
        // pair is not discovered at none — it is discovered under each host member's unit,
        // at the pattern the host's `templates` column declares for it.
        let harness = tmpdir("nested-file-dispatch");
        write_fixture_harness(&harness);

        let host = builtin_kind::definition("skill")
            .unwrap()
            .overlay_templates(&[TemplateRow {
                kind: "reference-doc".to_string(),
                path: Some("*.md".to_string()),
            }]);
        let kinds = BTreeMap::from([("skill".to_string(), host)]);
        let child = CustomKind::nested_file("reference-doc", Extraction::new(Vec::new()));

        // `coordinate`'s companion doc, and nothing from `demo` (which carries none) or the
        // hosts' own `SKILL.md` entry files.
        let found = discover_builtin(
            &Discovery::new(&harness),
            &child,
            &kinds,
            LocalOverride::Honored,
        );
        assert_eq!(
            found,
            vec![
                harness
                    .join(".claude/skills/coordinate")
                    .join("PLAYBOOK.md")
            ]
        );
    }

    #[test]
    fn one_discovery_shares_a_single_walk_across_kinds_and_a_nested_host() {
        // The shared-walk contract: a multi-kind harness with a nested-file host discovers
        // the identical member set whether each kind walks the tree itself or one cache is
        // threaded through them all, and the shared cache walks each flavor at most once.
        let harness = tmpdir("shared-walk");
        write_fixture_harness(&harness);

        let skill_kind = builtin_kind::definition("skill").unwrap();
        let rule_kind = builtin_kind::definition("rule").unwrap();
        let host = builtin_kind::definition("skill")
            .unwrap()
            .overlay_templates(&[TemplateRow {
                kind: "reference-doc".to_string(),
                path: Some("*.md".to_string()),
            }]);
        let hosts = BTreeMap::from([("skill".to_string(), host)]);
        let child = CustomKind::nested_file("reference-doc", Extraction::new(Vec::new()));

        // Two governs kinds plus a nested-file host, all threaded through one cache.
        let shared = Discovery::new(&harness);
        let skills = discover_builtin(&shared, &skill_kind, &no_hosts(), LocalOverride::Honored);
        let rules = discover_builtin(&shared, &rule_kind, &no_hosts(), LocalOverride::Honored);
        let nested = discover_builtin(&shared, &child, &hosts, LocalOverride::Honored);

        // Behavior identity: the same members a fresh, unshared cache per call finds.
        assert_eq!(
            skills,
            discover_builtin(
                &Discovery::new(&harness),
                &skill_kind,
                &no_hosts(),
                LocalOverride::Honored
            )
        );
        assert_eq!(
            rules,
            discover_builtin(
                &Discovery::new(&harness),
                &rule_kind,
                &no_hosts(),
                LocalOverride::Honored
            )
        );
        assert_eq!(
            nested,
            discover_builtin(
                &Discovery::new(&harness),
                &child,
                &hosts,
                LocalOverride::Honored
            )
        );

        // Every discovery above honored the override over committed kinds, so all rode the
        // one non-local flavor: three kinds and the nested host's per-host scan cost one
        // walk, not four.
        assert_eq!(shared.flavors_walked(), 1);
    }

    #[test]
    fn unit_dir_resolves_a_host_unit_with_normalized_paths() {
        // The shared index keys entries as normalized paths (no leading `./`), so `unit_dir`
        // receives a normalized root (the fix normalizes it before calling `unit_dir`).
        // This test verifies `unit_dir` correctly extracts the host unit directory.
        let root = Path::new(".claude/skills");
        let entry = Path::new(".claude/skills/coordinate/SKILL.md");
        let result = unit_dir(root, entry);
        assert_eq!(result, Some(root.join("coordinate")));
    }

    #[test]
    fn normalize_path_strips_leading_dot_component() {
        // The fix normalizes the root before passing it to `unit_dir`. This test verifies
        // that normalize_path correctly strips the leading `.` that a `.`-rooted harness
        // would introduce (e.g., `.` joined with `.claude/skills` yields `./.claude/skills`).
        let unnormalized = Path::new("./.claude/skills");
        let normalized = crate::path::normalize_path(unnormalized);
        assert_eq!(normalized, Path::new(".claude/skills"));

        // Now `unit_dir` can correctly match the normalized root against normalized entries.
        let entry = Path::new(".claude/skills/coordinate/SKILL.md");
        let result = unit_dir(&normalized, entry);
        assert_eq!(result, Some(normalized.join("coordinate")));
    }
}
