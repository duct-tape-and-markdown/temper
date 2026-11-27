//! Harness discovery — the sole member extractor the gate and `emit` both ride
//! (`discover_kind_units`/`discover_builtin`).

use std::borrow::Cow;
use std::cell::{Cell, OnceCell, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use ignore::WalkBuilder;

use crate::kind::{Commitment, CustomKind, Governs, UnitShape};
use crate::member_address;

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

/// A run's one owner of the harness tree: the fenced file index every discovery reader
/// shares, built by one ignore-honoring walk on first use ([`tree_index`]) and read by
/// every kind's locus scan, every nested-file host, the `.claude/` stray scan and the
/// guard's locus enumeration. The harness disk is stable across a run, so the memoized
/// index is the one a re-walk would produce.
///
/// A local-locus kind's scan reads the same index, reaching the ignored documents its own
/// locus names off it ([`Reach`]) rather than re-walking the tree with the rules waived.
///
/// The claimed-path set a declared kind's own `governs` locus claims is a pure function of
/// the kind set and the override, computed once per run and cached here too.
///
/// So is every locus scan: the files one `(root, glob, flavor)` locus matches are a pure
/// function of the stable tree, so the index is scanned for a locus once per run
/// ([`Discovery::scan`]) and every later reader of that locus — the gate's per-kind loop,
/// the claimed-path set, a host's unit enumeration, composition, admissibility — shares
/// the one result rather than re-scanning per call site.
pub struct Discovery<'a> {
    harness: &'a Path,
    index: OnceCell<Discoverable>,
    /// Each locus scan's result, keyed by its normalized root, its glob and whether it
    /// reads in the local flavor.
    scans: RefCell<HashMap<LocusKey, Rc<[PathBuf]>>>,
    /// The claimed paths for the `LocalOverride::Honored` flavor.
    claimed_honored: OnceCell<BTreeSet<PathBuf>>,
    /// The claimed paths for the `LocalOverride::Withheld` flavor.
    claimed_withheld: OnceCell<BTreeSet<PathBuf>>,
    /// The run's owner of file bytes, shared by every reader the index is threaded to.
    bytes: crate::bytes::Bytes,
}

/// One locus scan's identity: its normalized root, its glob, and whether it reads in the
/// local flavor.
type LocusKey = (PathBuf, String, bool);

impl<'a> Discovery<'a> {
    /// A fresh owner over `harness` — no walk runs until the index is first consulted.
    #[must_use]
    pub fn new(harness: &'a Path) -> Self {
        Self {
            harness,
            index: OnceCell::new(),
            scans: RefCell::new(HashMap::new()),
            claimed_honored: OnceCell::new(),
            claimed_withheld: OnceCell::new(),
            bytes: crate::bytes::Bytes::new(),
        }
    }

    /// The run's owner of file bytes: a reader holding this index reads a file through
    /// it, so the run opens each file once whichever reader reaches it first.
    #[must_use]
    pub fn bytes(&self) -> &crate::bytes::Bytes {
        &self.bytes
    }

    /// The harness root this cache walks — the base a `governs` root and the bare-skill
    /// path join against. The `governs` join itself has its own home one method down
    /// (`locus_root`); a caller asking "where does this locus sit on disk" wants that,
    /// not this.
    #[must_use]
    pub fn harness(&self) -> &Path {
        self.harness
    }

    /// The on-disk root a `governs` locus names — the harness root joined with the
    /// locus's declared `root`, normalized so a `.`-rooted locus (`memory`'s
    /// `.` + `**/CLAUDE.md`) resolves to the harness root itself rather than a
    /// `<harness>/.` spelling that no `strip_prefix` downstream matches.
    ///
    /// The one join-and-normalize: the unit-directory fold ([`host_units`]), the locus
    /// scan ([`discover_kind_units`]) and `compose`'s file-unit fold base each used to
    /// pair the join with the normalize by hand — two normalizing on the spot, the third
    /// leaving it to [`scan_locus`] — and now all three read it here. This is the only
    /// normalize on the way into [`scan_locus`], whose other root — a host unit's
    /// directory — is derived from this one ([`unit_dir`]) or from the normalized index
    /// ([`tree_index`]).
    ///
    /// `pub(crate)` because the only consumers are in-crate — this module and `compose`;
    /// the walk the root feeds is not itself public surface.
    #[must_use]
    pub(crate) fn locus_root(&self, governs: &Governs) -> PathBuf {
        crate::path::normalize_path(&self.harness().join(&governs.root))
    }

    /// The fenced tree index, walked on first use and memoized thereafter.
    fn index(&self) -> &Discoverable {
        self.index
            .get_or_init(|| tree_index(&self.bytes, self.harness))
    }

    /// How a scan in the `local_governs` flavor reads the tree: off the index alone, or
    /// off the index plus the ignored documents a local locus names.
    fn reach(&self, local_governs: bool) -> Reach<'_> {
        Reach {
            index: self.index(),
            harness: self.harness,
            bytes: &self.bytes,
            local: local_governs,
        }
    }

    /// Every file under the normalized `root` matching `glob` in the `local_governs` flavor,
    /// deterministically ordered — scanned off the index on a locus's first request this
    /// run and shared thereafter ([`scan_locus`] is the scan itself).
    fn scan(&self, root: &Path, glob: &str, local_governs: bool) -> Rc<[PathBuf]> {
        let key = (root.to_path_buf(), glob.to_string(), local_governs);
        if let Some(hit) = self.scans.borrow().get(&key) {
            return Rc::clone(hit);
        }
        LOCUS_SCANS.with(|scans| scans.set(scans.get() + 1));
        let found: Rc<[PathBuf]> = scan_locus(root, glob, &self.reach(local_governs)).into();
        self.scans.borrow_mut().insert(key, Rc::clone(&found));
        found
    }

    /// The index's direct children of the normalized `dir`, each with whether it is a
    /// directory, in name order — empty where the index holds nothing under `dir` (an
    /// absent, ignored or fenced directory).
    pub(crate) fn tree_children(&self, dir: &Path) -> impl Iterator<Item = (&Path, bool)> {
        self.index()
            .children(dir)
            .iter()
            .map(|child| (child.path.as_path(), child.is_dir))
    }

    /// Every file the index holds below the normalized `dir`, depth first in name order
    /// within each directory.
    pub(crate) fn tree_files_under(&self, dir: &Path) -> Vec<PathBuf> {
        fn descend(index: &Discoverable, dir: &Path, out: &mut Vec<PathBuf>) {
            for child in index.children(dir) {
                if child.is_dir {
                    descend(index, &child.path, out);
                } else {
                    out.push(child.path.clone());
                }
            }
        }
        let mut files = Vec::new();
        descend(self.index(), dir, &mut files);
        files
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

    /// Whether the index has been built.
    #[cfg(test)]
    fn indexed(&self) -> bool {
        self.index.get().is_some()
    }
}

thread_local! {
    /// Per-thread count of tree index passes: every walk [`ignore_walk`] builds bumps it,
    /// so a reader that walks the tree by any mechanism riding the shared ignore policy is
    /// counted, not only the discovery index. Walks run on their caller's thread, so a
    /// concurrent run on another test thread cannot perturb it.
    static INDEX_PASSES: Cell<usize> = const { Cell::new(0) };
    /// Per-thread count of locus scans over the index — one per distinct
    /// `(root, glob, flavor)` a [`Discovery`] is first asked for, never one per call site
    /// that asks again.
    static LOCUS_SCANS: Cell<usize> = const { Cell::new(0) };
}

/// This thread's locus scan count. Read before and after a verb run and compare the delta
/// to the distinct loci the run discovers, pinning that the index is scanned once per
/// locus rather than once per kind or reader asking for it.
#[must_use]
pub fn locus_scan_count() -> usize {
    LOCUS_SCANS.with(Cell::get)
}

/// This thread's tree index pass count. Read before and after a verb run and compare the
/// delta to one, pinning that whole-tree work computes once per run rather than per kind,
/// locus or call site.
#[must_use]
pub fn index_pass_count() -> usize {
    INDEX_PASSES.with(Cell::get)
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

/// One discovered nested file member: the child's source file, the host unit directory
/// its path composed under, and the **host member's own identity**. The file alone cannot
/// name that directory — a `*`-free pattern may seat the child levels below it — and the
/// id a `file` unit shape folds is the file's placement under it, so the halves travel
/// together.
///
/// The host template's pattern travels with them for the same reason the unit does: the
/// child's key folds against the locus the pattern's literal segments spell, and only this
/// scan knows which pattern found the file.
///
/// The host's identity travels beside them because a nested member's identity *is* its
/// `<host-address>/<kind>/<key>` address, and this scan is the one place that knows which
/// host a child composed under: the child kind governs no locus of its own, so nothing
/// downstream could recover the host from the file's path alone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NestedFileUnit {
    /// The host member's unit directory.
    pub host_unit: PathBuf,
    /// The host member's kind — the kind whose template placed this child.
    pub host_kind: String,
    /// The host member's **identity**, in the one form
    /// [`crate::member_address::address_of`] reads: its bare name when the host is a
    /// top-level member, and its whole `<host-address>/<kind>/<key>` address when the host
    /// is itself a nested file child. A host qualifies only at a **directory** unit shape
    /// (the guard below), whose key is its unit directory's name, so the final segment is
    /// always [`host_unit`](NestedFileUnit::host_unit)'s final component — what runs ahead
    /// of it is the chain of hosts above, carried so a child two layers down is addressed
    /// under both of them rather than under the nearer one alone.
    pub host_name: String,
    /// The host template's path pattern this child's file matched — the host kind's own
    /// declared fact, carried beside the unit because the pair is what composes the path:
    /// the literal segments the pattern spells are locus, so the key a read folds is
    /// derived against [`host_unit`](NestedFileUnit::host_unit) descended through them
    /// ([`crate::drift::fold_base`]), never against the bare unit.
    pub pattern: String,
    /// The child's source file, under `host_unit` at the host template's pattern.
    pub file: PathBuf,
}

/// One unit a host kind owns, as the per-host scan below needs it: the directory a child's
/// pattern is walked from, the host member's own identity ([`NestedFileUnit::host_name`]),
/// and the host's entry file — which is that host's own member and never its own child, so
/// a pattern matching it collects nothing.
struct HostUnit {
    dir: PathBuf,
    identity: String,
    entry: PathBuf,
}

impl HostUnit {
    /// The child at `file` this unit hosts, placed by `host`'s template `pattern`.
    fn child(&self, host: &CustomKind, pattern: &str, file: PathBuf) -> NestedFileUnit {
        NestedFileUnit {
            host_unit: self.dir.clone(),
            host_kind: host.name.clone(),
            host_name: self.identity.clone(),
            pattern: pattern.to_string(),
            file,
        }
    }
}

/// Discover a nested file `kind`'s members on `harness`: under each host member's unit,
/// every file matching the host template's pattern for this kind. The child kind carries
/// neither half — the pattern is the host kind's declared `templates` fact and the units
/// are the host kind's own — so `kinds`, the declared set keyed by bare name, is what the
/// host is read out of. This is the read side of the composition emit writes a child's
/// projection at: host unit joined with pattern, and nothing else.
///
/// The host's own units are **discovered through this same scan**, not read off a
/// `governs` locus: a host at an `at` locus supplies them from its own glob walk, and a
/// host that is *itself* a nested file child supplies them from its own host's unit, one
/// layer further up ([`host_units`]). So a child two layers down is discovered rather than
/// invisible, and nesting reads back at the arbitrary depth the model declares
/// (`specs/model/representation.md`, "nesting").
///
/// A host qualifies only where it owns a **directory** unit: a template's pattern is
/// relative to its host's unit, and a lone file has no interior to seat a child in.
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
    nested_file_members(disc, kind, kinds, over, claimed, &mut Vec::new())
}

/// [`discover_nested_file`] with the descent path it recurses along — the kinds whose
/// units are already being resolved further down the stack.
///
/// A kind met twice on one descent is a template cycle, and a cycle composes no locus at
/// all: the unit its members would sit in is their own descendant's. `emit` refuses such
/// a declaration outright (`crate::drift`'s `TemplateCycle`), so the lock a read walks
/// cannot carry one; the trail is what keeps *this* walk total regardless of what it is
/// handed, since a `Vec` return has no channel to refuse down.
fn nested_file_members(
    disc: &Discovery,
    kind: &CustomKind,
    kinds: &BTreeMap<String, CustomKind>,
    over: LocalOverride,
    claimed: &BTreeSet<PathBuf>,
    trail: &mut Vec<String>,
) -> Vec<NestedFileUnit> {
    let mut found = Vec::new();
    for (host, pattern, unit) in kind_host_units(disc, kind, kinds, over, claimed, trail) {
        for file in disc
            .scan(&unit.dir, pattern, local_governs(host, over))
            .iter()
        {
            if *file != unit.entry && !claimed.contains(file) {
                found.push(unit.child(host, pattern, file.clone()));
            }
        }
    }
    found.sort_by(|a, b| a.file.cmp(&b.file));
    found
}

/// The nested file placement `check`'s scan would give a `kind` document at `file`, an
/// absolute path that need not exist yet — a pending write, which no walk can admit. The
/// single-path answer to [`discover_nested_file`], over the same host units: `file` is
/// placed where it sits under a host unit at the host template's pattern, is not that
/// host's own entry, is claimed by no declared `governs` locus, and discovery would reach
/// it in the host's flavor ([`discovers_path`]). [`None`] where any of those fails.
pub fn nested_file_placement(
    disc: &Discovery,
    kind: &CustomKind,
    kinds: &BTreeMap<String, CustomKind>,
    over: LocalOverride,
    file: &Path,
) -> Option<NestedFileUnit> {
    let file = crate::path::normalize_path(file);
    let harness = crate::path::normalize_path(disc.harness());
    let relative = file.strip_prefix(&harness).ok()?;
    let claimed = disc.declared_governed_paths(kinds, over);
    if claimed.contains(&file) {
        return None;
    }
    kind_host_units(disc, kind, kinds, over, claimed, &mut Vec::new())
        .into_iter()
        .find_map(|(host, pattern, unit)| {
            let under = crate::path::to_canonical_spelling(file.strip_prefix(&unit.dir).ok()?);
            let placed = file != unit.entry
                && crate::glob::compile_glob(pattern).is_some_and(|m| m.is_match(&under))
                && discovers_path(disc.bytes(), &harness, relative, local_governs(host, over));
            placed.then(|| unit.child(host, pattern, file.clone()))
        })
}

/// Every host unit a `kind` child composes under, with the host kind and the template
/// pattern it places the child at — the host loop [`nested_file_members`] scans and
/// [`nested_file_placement`] matches against. Only a host owning a **directory** unit
/// qualifies: a lone file has no interior to seat a child in. A `kind` already on `trail`
/// is a template cycle and yields nothing.
fn kind_host_units<'k>(
    disc: &Discovery,
    kind: &CustomKind,
    kinds: &'k BTreeMap<String, CustomKind>,
    over: LocalOverride,
    claimed: &BTreeSet<PathBuf>,
    trail: &mut Vec<String>,
) -> Vec<(&'k CustomKind, &'k str, HostUnit)> {
    let mut units = Vec::new();
    if trail.contains(&kind.name) {
        return units;
    }
    trail.push(kind.name.clone());
    for host in kinds.values() {
        let Some(pattern) = file_template(host, &kind.name) else {
            continue;
        };
        if host.unit_shape != Some(UnitShape::Directory) {
            continue;
        }
        for unit in host_units(disc, host, kinds, over, claimed, trail) {
            units.push((host, pattern, unit));
        }
    }
    trail.pop();
    units
}

/// Every unit `host` owns on disk, with the host member's own identity — the recursion's
/// one step, and the read-side twin of the way emit derives a host's unit from the host's
/// own projection (`crate::drift`'s `nested_file_path`).
///
/// A host at an `at` locus walks its own `governs` glob and keys each unit by its
/// directory's name. A host that governs nothing is **itself** a nested file child: its
/// own members are discovered one layer up, each entry file sitting inside the unit it
/// owns, and its identity is its whole `<host-address>/<kind>/<key>` address — composed
/// through [`member_address::address_of`], so the chain above it survives into every
/// child it in turn hosts.
///
/// Only a **directory** unit shape reaches here (the caller's guard), so a host's key is
/// its unit directory's final component either way — the id that shape folds.
fn host_units(
    disc: &Discovery,
    host: &CustomKind,
    kinds: &BTreeMap<String, CustomKind>,
    over: LocalOverride,
    claimed: &BTreeSet<PathBuf>,
    trail: &mut Vec<String>,
) -> Vec<HostUnit> {
    let Some(governs) = host.governs.as_ref() else {
        return nested_file_members(disc, host, kinds, over, claimed, trail)
            .into_iter()
            .filter_map(|found| {
                let dir = found.file.parent()?.to_path_buf();
                let key = dir.file_name().and_then(OsStr::to_str)?;
                let above = member_address::address_of(&found.host_kind, &found.host_name);
                Some(HostUnit {
                    identity: member_address::nested_address(&above, &host.name, key),
                    dir,
                    entry: found.file,
                })
            })
            .collect();
    };
    let root = disc.locus_root(governs);
    discover_kind_files(disc, host, governs, over)
        .into_iter()
        .filter_map(|entry| {
            let dir = unit_dir(&root, &entry)?;
            let glob = governs.glob_at(&root, &entry);
            let base = crate::drift::fold_base(&root, glob);
            // The host's own name: the id a `directory` unit shape derives, its unit
            // directory's final component behind any `**`-spanned prefix.
            let identity = crate::extract::unit_shape_id(
                Some(&UnitShape::Directory),
                &entry,
                &base,
                Some(glob),
                |_| None,
            )
            .ok()?;
            Some(HostUnit {
                dir,
                identity,
                entry,
            })
        })
        .collect()
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

/// The unit directory a discovered entry file sits in, below the kind's
/// `governs` root, where a directory-unit member's `<root>/<name>/` is composed. [`None`]
/// for a file the root does not contain (a bare harness that is itself a skill) or one
/// lying loose at the root, neither of which owns an interior a template addresses.
fn unit_dir(root: &Path, entry: &Path) -> Option<PathBuf> {
    let relative = entry.strip_prefix(root).ok()?;
    relative
        .parent()
        .filter(|dir| !dir.as_os_str().is_empty())?;
    entry.parent().map(Path::to_path_buf)
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
/// The kind's declared excluded paths are subtracted from the walk through the one locus
/// membership predicate ([`crate::kind::locus_admits`]): no member owns one. So are
/// its subkinds' ceded loci, whose documents are the subkinds' members alone.
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
    if !kind.excluded.is_empty() || !kind.ceded.is_empty() {
        // The walk's files hang off the normalized locus root, so they are read back to
        // harness-relative against the harness normalized the same way.
        let harness = crate::path::normalize_path(disc.harness());
        let patterns = governs.patterns();
        files.retain(|file| {
            file.strip_prefix(&harness).is_ok_and(|relative| {
                let relative = crate::path::HarnessRelativePath::from_path(relative);
                crate::kind::locus_admits_any(&patterns, &kind.excluded, &kind.ceded, &relative)
            })
        });
    }
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
pub(crate) fn local_governs(kind: &CustomKind, over: LocalOverride) -> bool {
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
    let root = disc.locus_root(governs);
    // Each template is walked; a document two templates both match is one member.
    let mut files: Vec<PathBuf> = governs
        .globs
        .iter()
        .flat_map(|glob| disc.scan(&root, glob, local_governs).to_vec())
        .collect();
    if governs.globs.len() > 1 {
        files.sort();
        files.dedup();
    }
    files
}

/// The scan itself: every file under `root` matching `glob`, deterministically ordered —
/// run once per locus per run, behind [`Discovery::scan`]'s memo.
/// Split from [`discover_kind_units`] so a nested file child's scan under each host unit
/// rides the same matcher and the same shared index — one scanner serves every kind's
/// locus, host and child alike.
///
/// `root` must already be normalized: matching reads the index, every key of which is
/// normalized ([`tree_index`]), so a `<harness>/.` spelling keys nothing. Normalizing is
/// the producer's job, not the scan's — a locus root comes from [`Discovery::locus_root`],
/// a host unit's directory from [`unit_dir`] off that root or from an index key's parent.
fn scan_locus(root: &Path, glob: &str, reach: &Reach) -> Vec<PathBuf> {
    // A glob is a `/`-separated segment list: the final segment matches files, each
    // earlier one a subdirectory to descend into — a `**` segment descending any
    // number of levels. `split` always yields at least one segment.
    //
    // Each segment's matcher is resolved once here, before the walk, rather than per child
    // the walk tests against it.
    let segments: Vec<Segment> = glob.split('/').map(Segment::new).collect();
    let mut files = Vec::new();
    if reach.admits_root(root) {
        collect_glob(root, &segments, reach, &mut files);
    }
    // A lone `**` reaches one file by exactly one path, but an interior one beside it can
    // reach a file nested in two matching folders twice, so the walk dedups.
    files.sort();
    files.dedup();
    files
}

/// Collect every reachable file whose path matches the remaining glob `segments`. The
/// head segment selects children at this level; if it is the last, matching **files** are
/// collected, otherwise matching **subdirectories** are descended. A `**` head is the
/// any-depth wildcard — it matches zero or more directory levels, so a nested nearest-wins
/// hierarchy (the `CLAUDE.md` memory nesting) is discovered at every level, not just the
/// fixed glob depth. A `dir` holding no children contributes nothing — a subdir glob
/// whose intermediate level is absent, or a locus that does not exist on this harness,
/// both resolve to no units.
fn collect_glob(dir: &Path, segments: &[Segment], reach: &Reach, out: &mut Vec<PathBuf>) {
    let Some((segment, rest)) = segments.split_first() else {
        // `**` recurses with the same segments, so it can bottom out at an empty list
        // (a trailing `**` with nothing left to match): nothing more to collect here.
        return;
    };
    let Segment::Name { text, matcher } = segment else {
        // Zero levels: match the remaining segments right at this level, so
        // `**/CLAUDE.md` picks up a `CLAUDE.md` directly under the root too.
        collect_glob(dir, rest, reach, out);
        // One-or-more levels: descend into every subdirectory carrying the `**`, so
        // each nested file is reached by exactly one path (no double-collection).
        for child in reach.children(dir, None).iter() {
            if child.is_dir {
                collect_glob(&child.path, segments, reach, out);
            }
        }
        return;
    };
    // A segment `globset` cannot compile matches nothing.
    let Some(matcher) = matcher else {
        return;
    };
    let want_dir = !rest.is_empty();
    for child in reach.children(dir, Some((text, want_dir))).iter() {
        if child.is_dir != want_dir {
            continue;
        }
        let Some(name) = child.path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if !matcher.is_match(name) {
            continue;
        }
        if want_dir {
            collect_glob(&child.path, rest, reach, out);
        } else {
            out.push(child.path.clone());
        }
    }
}

/// One `/`-separated glob segment as [`collect_glob`] walks it: the any-depth `**`, or a
/// name pattern carrying its matcher, resolved once per scan.
enum Segment<'g> {
    AnyDepth,
    Name {
        text: &'g str,
        matcher: Option<Rc<globset::GlobMatcher>>,
    },
}

impl<'g> Segment<'g> {
    fn new(text: &'g str) -> Self {
        if text == "**" {
            Self::AnyDepth
        } else {
            Self::Name {
                text,
                matcher: crate::glob::compile_glob(text),
            }
        }
    }
}

/// How one scan reads the tree. The committed flavor reads the fenced index alone: every
/// child it yields is already discoverable (`.git/` excluded, the ignore rules and the
/// workspace skip applied, nested governed roots fenced). A local-locus kind's scan reads
/// the same index for every directory a wildcard descends, and reaches the documents its
/// `governs` claims past the ignore rules and the workspace skip only where its own locus
/// names them: a literal segment is stat'ed rather than looked up, a directory the index
/// never descended (ignored or the workspace) is listed raw, and a wildcard leaf lists its
/// directory raw so an ignored file there is seen. A wildcard never descends an ignored
/// directory the index skipped — the local waiver is the locus's own, never a whole-tree
/// walk with the rules waived. `.git/` and a nested governed root stay fenced either way.
struct Reach<'d> {
    index: &'d Discoverable,
    harness: &'d Path,
    bytes: &'d crate::bytes::Bytes,
    local: bool,
}

impl Reach<'_> {
    /// Whether a scan may start at the normalized `root`: always for the committed flavor
    /// (the index answers), and for a local scan wherever the fences admit it.
    fn admits_root(&self, root: &Path) -> bool {
        if !self.local || self.index.descended(root) {
            return true;
        }
        let harness = crate::path::normalize_path(self.harness);
        root.strip_prefix(&harness)
            .is_ok_and(|relative| discovers_path(self.bytes, self.harness, relative, true))
    }

    /// The children of `dir` a scan considers for the next segment: `next` is that
    /// segment and whether it selects a directory, [`None`] for a `**` descent.
    /// The committed flavor borrows the index's own list rather than copying it.
    fn children(&self, dir: &Path, next: Option<(&str, bool)>) -> Cow<'_, [DiscoverableChild]> {
        let indexed = self.index.children(dir);
        if !self.local {
            return Cow::Borrowed(indexed);
        }
        let descended = self.index.descended(dir);
        match next {
            Some((segment, want_dir)) if is_literal(segment) => {
                let path = dir.join(segment);
                let found = if want_dir {
                    on_disk(&path).is_dir()
                } else {
                    on_disk(&path).is_file()
                };
                if found && !fenced(&path, 1, want_dir, true) {
                    Cow::Owned(vec![DiscoverableChild {
                        path,
                        is_dir: want_dir,
                    }])
                } else {
                    Cow::Owned(Vec::new())
                }
            }
            Some((_, false)) => Cow::Owned(raw_children(dir)),
            _ if descended => Cow::Borrowed(indexed),
            _ => Cow::Owned(raw_children(dir)),
        }
    }
}

/// Whether a glob segment names one entry exactly — no wildcard, class or alternation.
fn is_literal(segment: &str) -> bool {
    !segment.contains(['*', '?', '[', '{'])
}

/// The spelling a filesystem call takes for a normalized path: the empty path a `.`
/// harness normalizes to reads as `.`.
fn on_disk(path: &Path) -> &Path {
    if path.as_os_str().is_empty() {
        Path::new(".")
    } else {
        path
    }
}

/// The direct children of `dir` on raw disk, ignore rules and the workspace skip waived,
/// `.git/` and a nested governed root still fenced — a local scan's one listing of a
/// directory its own locus names. An unreadable directory lists nothing.
fn raw_children(dir: &Path) -> Vec<DiscoverableChild> {
    let Ok(entries) = std::fs::read_dir(on_disk(dir)) else {
        return Vec::new();
    };
    entries
        .flatten()
        .filter_map(|entry| {
            let path = dir.join(entry.file_name());
            let is_dir = entry.file_type().ok()?.is_dir();
            (!fenced(&path, 1, is_dir, true)).then_some(DiscoverableChild { path, is_dir })
        })
        .collect()
}

/// A walk over `root` carrying the ignore-source policy every walk in this tree rides —
/// which declarations of non-authorship a walk consults, spelled once so two consumers
/// cannot drift apart on it.
///
/// Built with ripgrep's `ignore` engine so nested `.gitignore` files, negation, and
/// precedence are honored rather than hand-rolled. Only git's own declaration counts:
/// the machine-global and ripgrep-specific (`.ignore`) sources are off, and parent
/// directories above `root` are not consulted — the harness is the per-project
/// boundary. `require_git` is off so a `.gitignore` is honored even when the tree is not
/// itself a git checkout (a sub-tree, or a test fixture). Dotfiles never hide: `.claude/`
/// is a dotdir the harness lives in.
///
/// `git_declarations` off drops git's two sources with the rest, for a walk whose
/// kind declares authorship over what the repo ignores.
pub(crate) fn ignore_walk(
    bytes: &crate::bytes::Bytes,
    root: &Path,
    git_declarations: bool,
) -> WalkBuilder {
    INDEX_PASSES.with(|passes| passes.set(passes.get() + 1));
    let mut builder = WalkBuilder::new(root);
    builder
        .hidden(false)
        .parents(false)
        .ignore(false)
        .git_global(false)
        .git_ignore(git_declarations)
        .git_exclude(git_declarations)
        .require_git(false);
    if git_declarations && let Some(exclude) = common_exclude(bytes, root) {
        // Explicit ignores anchor at the builder's current dir and rank below every
        // per-directory source — git's own precedence for `info/exclude`.
        builder.current_dir(root);
        // A partial parse error still contributes the globs that parsed.
        let _partial = builder.add_ignore(exclude);
    }
    builder
}

/// The git directories of the repository whose `.git` sits at `root`: the per-worktree
/// git dir and the common dir that holds `info/`, refs, and objects. [`None`] where
/// `root` carries no `.git`, or its pointer file does not resolve.
///
/// A `.git` directory is both. A `.git` file is a pointer (`gitdir: <path>`, relative to
/// `root`) to a per-worktree admin dir, whose `commondir` file (relative to the admin
/// dir) names the common dir; an admin dir with no `commondir` is its own common dir.
pub(crate) fn git_dirs(bytes: &crate::bytes::Bytes, root: &Path) -> Option<GitDirs> {
    let dot_git = root.join(".git");
    if dot_git.is_dir() {
        return Some(GitDirs {
            git_dir: dot_git.clone(),
            common_dir: dot_git,
        });
    }
    let pointer = bytes.read_to_string(&dot_git).ok()?;
    let gitdir = pointer
        .lines()
        .find_map(|line| line.strip_prefix("gitdir: "))?
        .trim();
    let git_dir = crate::path::normalize_path(&root.join(gitdir));
    if !git_dir.is_dir() {
        return None;
    }
    let common_dir = match bytes.read_to_string(&git_dir.join("commondir")) {
        Ok(commondir) => crate::path::normalize_path(&git_dir.join(commondir.trim())),
        Err(_) => git_dir.clone(),
    };
    Some(GitDirs {
        git_dir,
        common_dir,
    })
}

/// A repository's two git directories, as [`git_dirs`] resolves them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct GitDirs {
    /// The per-worktree git dir (`HEAD`, the index).
    pub(crate) git_dir: PathBuf,
    /// The dir shared by every worktree (`info/`, refs, objects).
    pub(crate) common_dir: PathBuf,
}

/// The `info/exclude` a walk over `root` must add beyond what the `ignore` engine reads
/// itself: the common dir's, when `root`'s `.git` is a pointer file — the engine only
/// reads `<dir>/.git/info/exclude`, which a pointer file never has.
fn common_exclude(bytes: &crate::bytes::Bytes, root: &Path) -> Option<PathBuf> {
    if root.join(".git").is_dir() {
        return None;
    }
    let exclude = git_dirs(bytes, root)?
        .common_dir
        .join("info")
        .join("exclude");
    exclude.is_file().then_some(exclude)
}

/// The fenced tree index over `harness`, each discoverable directory's direct children.
/// Two presumptions prune it: the repo's ignore rules — an ignored file is by declaration
/// not authored here — and the surface workspace (`.temper/`), which holds temper's own
/// modules and lock and, being committed rather than gitignored, would otherwise enter the
/// set on its own. A local-locus kind's scan reaches past both under its own locus
/// ([`Reach`]), never by a second walk.
///
/// Two fences are not presumptions and hold for every reader: `.git/`, and a **nested
/// governed root** — a subdirectory below the harness root carrying its own
/// `.temper/lock.toml` is its own corpus, so the walk never descends it. The harness
/// root's own lock must not self-fence, so that skip keys off walk depth, not the name.
///
/// Paths are normalized so a `.`-rooted `governs` (`root = "."`) compares equal to the
/// walk's harness-relative entries; children sort by name, so every reader enumerates in
/// one order on every platform.
fn tree_index(bytes: &crate::bytes::Bytes, harness: &Path) -> Discoverable {
    let mut children: BTreeMap<PathBuf, Vec<DiscoverableChild>> = BTreeMap::new();
    let mut descended = BTreeSet::new();
    let walk = ignore_walk(bytes, harness, true)
        .sort_by_file_name(std::cmp::Ord::cmp)
        .filter_entry(discovery_fence(false))
        .build();
    // A walk error (an unreadable entry) drops that entry rather than aborting discovery.
    // The file-vs-directory tag rides `entry.file_type()` — the one fact the scan needs
    // that a bare path set lacks — recorded here so the scan never re-`stat`s a candidate.
    for entry in walk.flatten() {
        let path = crate::path::normalize_path(entry.path());
        let is_dir = entry
            .file_type()
            .is_some_and(|file_type| file_type.is_dir());
        if is_dir {
            descended.insert(path.clone());
        }
        if entry.depth() == 0 {
            continue;
        }
        if let Some(parent) = path.parent().map(Path::to_path_buf) {
            children
                .entry(parent)
                .or_default()
                .push(DiscoverableChild { path, is_dir });
        }
    }
    Discoverable {
        children,
        descended,
    }
}

/// The entry filter every discovery walk prunes by: [`fenced`] over a walk's entry, so a
/// walk that adds its own bound composes this rather than respelling the fences.
pub(crate) fn discovery_fence(
    local_governs: bool,
) -> impl Fn(&ignore::DirEntry) -> bool + Send + Sync + 'static {
    move |entry| {
        let is_dir = entry
            .file_type()
            .is_some_and(|file_type| file_type.is_dir());
        !fenced(entry.path(), entry.depth(), is_dir, local_governs)
    }
}

/// Whether a walk stops at `path` regardless of the repo's ignore rules: `.git/`, the
/// surface workspace unless `local_governs` waives it, and a nested governed root. Depth 0
/// is the harness root itself, whose own lock governs the walk and must never fence it.
fn fenced(path: &Path, depth: usize, is_dir: bool, local_governs: bool) -> bool {
    let name = path.file_name();
    name == Some(OsStr::new(".git"))
        || (!local_governs && name == Some(OsStr::new(crate::WORKSPACE_DIR)))
        || (depth > 0 && is_dir && is_governed_root(path))
}

/// Whether a discovery walk over `harness` in the `local_governs` flavor would reach the
/// file at the harness-relative `relative` — the single-path answer to the question
/// [`tree_index`] answers for a whole tree, for a path that may not exist yet (a
/// pending write) and so cannot be admitted by a walk. The flavor waives what the walk's
/// does: the workspace fence and every git ignore declaration.
///
/// Each prefix of `relative` is put to the [`fenced`] fences, then the git ignore
/// declarations [`ignore_walk`] consults ([`admitted`]).
pub(crate) fn discovers_path(
    bytes: &crate::bytes::Bytes,
    harness: &Path,
    relative: &Path,
    local_governs: bool,
) -> bool {
    admitted(
        bytes,
        harness,
        relative,
        !local_governs,
        |path, depth, is_dir| fenced(path, depth, is_dir, local_governs),
    )
}

/// Whether the repository holding the real, absolute `path` leaves it unignored — the
/// git ignore declarations a discovery walk honours (every `.gitignore`, each level's
/// `.git/info/exclude`, a linked worktree's common exclude), judged from the nearest
/// ancestor carrying a `.git`, or from `fallback_root` when no ancestor does and `path`
/// sits under it. A path in no repository and under no fallback root has no rules to
/// exclude it, so it is kept.
pub(crate) fn repository_keeps(path: &Path, fallback_root: &Path) -> bool {
    let path = crate::path::normalize_path(path);
    let root = path
        .ancestors()
        .skip(1)
        .find(|dir| dir.join(".git").exists())
        .map(Path::to_path_buf)
        .or_else(|| {
            let fallback = fallback_root
                .canonicalize()
                .unwrap_or_else(|_| fallback_root.to_path_buf());
            path.starts_with(&fallback).then_some(fallback)
        });
    let Some(root) = root else {
        return true;
    };
    let Ok(relative) = path.strip_prefix(&root) else {
        return true;
    };
    admitted(
        &crate::bytes::Bytes::new(),
        &root,
        relative,
        true,
        |path, _depth, _is_dir| path.file_name() == Some(OsStr::new(".git")),
    )
}

/// The walk-order judgment [`discovers_path`] and [`repository_keeps`] share: each prefix
/// of `relative` under `root` is put to `fence`, then — when `git_declarations` — to
/// every `.gitignore` from `root` down to the prefix's parent, deepest first, then each
/// level's `.git/info/exclude` the same way, then a linked worktree's common exclude, the
/// precedence the `ignore` engine applies. An ignored prefix prunes everything below it,
/// exactly as the walk never descends an ignored directory. A `.gitignore` that fails to
/// parse contributes what it could parse, as the walk's own builder does.
fn admitted(
    bytes: &crate::bytes::Bytes,
    root: &Path,
    relative: &Path,
    git_declarations: bool,
    fence: impl Fn(&Path, usize, bool) -> bool,
) -> bool {
    use ignore::gitignore::{Gitignore, GitignoreBuilder};

    let matcher = |dir: &Path, file: PathBuf| {
        let mut builder = GitignoreBuilder::new(dir);
        if file.is_file() {
            // A partial parse error still yields the globs that parsed.
            let _partial = builder.add(file);
        }
        builder.build().unwrap_or_else(|_| Gitignore::empty())
    };
    let common = if git_declarations {
        common_exclude(bytes, root).map(|exclude| matcher(root, exclude))
    } else {
        None
    };
    let components: Vec<_> = relative.components().collect();
    let mut gitignores = Vec::new();
    let mut excludes = Vec::new();
    let mut dir = root.to_path_buf();
    for (index, component) in components.iter().enumerate() {
        let path = dir.join(component);
        let last = index + 1 == components.len();
        let is_dir = !last || path.is_dir();
        if fence(&path, index + 1, is_dir) {
            return false;
        }
        if !git_declarations {
            dir = path;
            continue;
        }
        gitignores.push(matcher(&dir, dir.join(".gitignore")));
        excludes.push(matcher(&dir, dir.join(".git").join("info").join("exclude")));
        let decision = gitignores
            .iter()
            .rev()
            .chain(excludes.iter().rev())
            .chain(common.iter())
            .map(|ignores| ignores.matched(&path, is_dir))
            .find(|matched| !matched.is_none());
        if decision.is_some_and(|matched| matched.is_ignore()) {
            return false;
        }
        dir = path;
    }
    true
}

/// Whether `dir` carries its own `.temper/lock.toml` — the mark of an independently
/// governed harness whose members are its own corpus, not the enclosing walk's.
fn is_governed_root(dir: &Path) -> bool {
    dir.join(crate::WORKSPACE_DIR)
        .join(crate::LOCK_FILENAME)
        .is_file()
}

/// The fenced tree index: the direct children of every discoverable directory, keyed by
/// the parent's normalized path, and the set of directories the walk descended. Each child
/// carries the file-vs-directory tag the walk already knew (`entry.file_type()`): a glob's
/// final segment collects a child only if it is a file and descends it only if a
/// directory, so the tag is read here rather than re-`stat`ed per candidate.
struct Discoverable {
    children: BTreeMap<PathBuf, Vec<DiscoverableChild>>,
    descended: BTreeSet<PathBuf>,
}

impl Discoverable {
    /// The direct children the walk recorded under `dir` — empty for a path the index holds
    /// none for (a missing locus, or a leaf that seats nothing).
    fn children(&self, dir: &Path) -> &[DiscoverableChild] {
        self.children.get(dir).map_or(&[], Vec::as_slice)
    }

    /// Whether the walk descended `dir` — false for a directory the ignore rules, the
    /// workspace skip or a fence kept it out of, and for one that does not exist.
    fn descended(&self, dir: &Path) -> bool {
        self.descended.contains(dir)
    }
}

/// One entry in a directory's child list: its normalized path and whether it is a
/// directory — the two facts the glob scan needs to match a name and decide
/// collect-vs-descend, both known at walk time.
#[derive(Clone)]
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

    /// The single-path query answers exactly as the walk does, for every fence and for
    /// nested ignore precedence: a deeper `.gitignore` re-including what a shallower one
    /// ignores, an ignored directory pruning its re-included child, a nested governed
    /// root, and the workspace.
    #[test]
    fn the_single_path_query_agrees_with_the_discovery_walk() {
        let root = tmpdir("discovers-path-agrees");
        let write = |relative: &str, body: &str| {
            let path = root.join(relative);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, body).unwrap();
        };
        write(".gitignore", "*.local.md\nbuild/\n");
        write(".claude/rules/.gitignore", "!keep.local.md\n");
        write(".claude/rules/a.md", "");
        write(".claude/rules/b.local.md", "");
        write(".claude/rules/keep.local.md", "");
        write("build/.gitignore", "!x.md\n");
        write("build/x.md", "");
        write("vendor/sub/.temper/lock.toml", "");
        write("vendor/sub/y.md", "");
        write(".temper/z.md", "");
        write(".git/info/exclude", "excluded.md\n");
        write("excluded.md", "");

        let walked: BTreeSet<PathBuf> = ignore_walk(&crate::bytes::Bytes::new(), &root, true)
            .filter_entry(discovery_fence(false))
            .build()
            .flatten()
            .filter(|entry| entry.file_type().is_some_and(|kind| kind.is_file()))
            .map(|entry| entry.path().strip_prefix(&root).unwrap().to_path_buf())
            .collect();
        for candidate in [
            ".claude/rules/a.md",
            ".claude/rules/b.local.md",
            ".claude/rules/keep.local.md",
            ".claude/rules/pending.md",
            ".claude/rules/pending.local.md",
            "build/x.md",
            "vendor/sub/y.md",
            ".temper/z.md",
            "excluded.md",
        ] {
            let candidate = Path::new(candidate);
            let exists = root.join(candidate).is_file();
            let queried = discovers_path(&crate::bytes::Bytes::new(), &root, candidate, false);
            if exists {
                assert_eq!(
                    queried,
                    walked.contains(candidate),
                    "{}",
                    candidate.display()
                );
            }
        }
        assert!(discovers_path(
            &crate::bytes::Bytes::new(),
            &root,
            Path::new(".claude/rules/pending.md"),
            false
        ));
        assert!(!discovers_path(
            &crate::bytes::Bytes::new(),
            &root,
            Path::new(".claude/rules/pending.local.md"),
            false
        ));
        assert!(walked.contains(Path::new(".claude/rules/keep.local.md")));
        assert!(!walked.contains(Path::new("build/x.md")));
    }

    /// In a linked worktree the common git dir's `info/exclude` governs both surfaces
    /// alike: its patterns anchor at the worktree root, a `.gitignore` negation outranks
    /// it, and an excluded directory prunes its children.
    #[test]
    fn a_linked_worktrees_common_exclude_anchors_at_its_root_below_a_gitignore_negation() {
        let tmp = tmpdir("discovers-path-worktree");
        let write = |relative: &str, body: &str| {
            let path = tmp.join(relative);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, body).unwrap();
        };
        // The layout `git worktree add` writes: a pointer file, an admin dir under the
        // common dir's `worktrees/`, and a `commondir` relative to the admin dir.
        write("primary/.git/worktrees/linked/commondir", "../..\n");
        write(
            "primary/.git/info/exclude",
            "/.claude/rules/excluded.md\nscratch/\n*.keep.md\n",
        );
        write("linked/.git", "gitdir: ../primary/.git/worktrees/linked\n");
        write("linked/.gitignore", "!a.keep.md\n");
        write("linked/.claude/rules/a.md", "");
        write("linked/.claude/rules/excluded.md", "");
        write("linked/.claude/rules/a.keep.md", "");
        write("linked/.claude/rules/b.keep.md", "");
        write("linked/scratch/x.md", "");
        write("linked/nested/.claude/rules/excluded.md", "");
        let root = tmp.join("linked");

        let walked: BTreeSet<PathBuf> = ignore_walk(&crate::bytes::Bytes::new(), &root, true)
            .filter_entry(discovery_fence(false))
            .build()
            .flatten()
            .filter(|entry| entry.file_type().is_some_and(|kind| kind.is_file()))
            .map(|entry| entry.path().strip_prefix(&root).unwrap().to_path_buf())
            .collect();
        let expected = [
            (".claude/rules/a.md", true),
            (".claude/rules/excluded.md", false),
            (".claude/rules/a.keep.md", true),
            (".claude/rules/b.keep.md", false),
            ("scratch/x.md", false),
            ("nested/.claude/rules/excluded.md", true),
        ];
        for (candidate, discoverable) in expected {
            let candidate = Path::new(candidate);
            assert_eq!(
                walked.contains(candidate),
                discoverable,
                "walk: {}",
                candidate.display()
            );
            assert_eq!(
                discovers_path(&crate::bytes::Bytes::new(), &root, candidate, false),
                discoverable,
                "query: {}",
                candidate.display()
            );
        }
        // The local-locus flavor waives every git declaration, the common exclude too.
        assert!(discovers_path(
            &crate::bytes::Bytes::new(),
            &root,
            Path::new(".claude/rules/excluded.md"),
            true
        ));
    }

    /// The resolver reads a `.git` directory as both dirs and chains a pointer file
    /// through its admin dir's `commondir`; an admin dir without one is its own common dir.
    #[test]
    fn git_dirs_resolves_a_directory_a_worktree_pointer_and_a_bare_pointer() {
        let tmp = tmpdir("git-dirs");
        fs::create_dir_all(tmp.join("repo/.git/worktrees/wt")).unwrap();
        fs::write(tmp.join("repo/.git/worktrees/wt/commondir"), "../..\n").unwrap();
        fs::create_dir_all(tmp.join("repo/.git/modules/sub")).unwrap();
        fs::create_dir_all(tmp.join("wt")).unwrap();
        fs::write(tmp.join("wt/.git"), "gitdir: ../repo/.git/worktrees/wt\n").unwrap();
        fs::create_dir_all(tmp.join("sub")).unwrap();
        fs::write(tmp.join("sub/.git"), "gitdir: ../repo/.git/modules/sub\n").unwrap();

        let git = tmp.join("repo/.git");
        assert_eq!(
            git_dirs(&crate::bytes::Bytes::new(), &tmp.join("repo")),
            Some(GitDirs {
                git_dir: git.clone(),
                common_dir: git.clone(),
            })
        );
        assert_eq!(
            git_dirs(&crate::bytes::Bytes::new(), &tmp.join("wt")),
            Some(GitDirs {
                git_dir: git.join("worktrees/wt"),
                common_dir: git.clone(),
            })
        );
        assert_eq!(
            git_dirs(&crate::bytes::Bytes::new(), &tmp.join("sub")),
            Some(GitDirs {
                git_dir: git.join("modules/sub"),
                common_dir: git.join("modules/sub"),
            })
        );
        assert_eq!(git_dirs(&crate::bytes::Bytes::new(), &tmp), None);
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
    fn the_locus_root_joins_the_harness_and_drops_a_root_dot() {
        // The one join-and-normalize, at the two edges each of the three former hand-
        // pairings had to get right alone: a nested locus root lands under the harness,
        // and a `.`-rooted locus resolves to the harness root itself rather than a
        // `<harness>/.` no downstream `strip_prefix` matches.
        let harness = Path::new("/tmp/gov-root");
        let disc = Discovery::new(harness);

        let nested = Governs {
            root: ".claude/skills".to_string(),
            globs: vec!["*/SKILL.md".to_string()],
        };
        assert_eq!(
            disc.locus_root(&nested),
            Path::new("/tmp/gov-root/.claude/skills")
        );

        let root_locus = Governs {
            root: ".".to_string(),
            globs: vec!["**/CLAUDE.md".to_string()],
        };
        assert_eq!(disc.locus_root(&root_locus), harness);
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
                globs: vec!["*.md".to_string()],
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
            globs: vec!["*/THING.md".to_string()],
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
                globs: vec!["*.md".to_string()],
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
                globs: vec!["*.md".to_string()],
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
                globs: vec!["**/CLAUDE.md".to_string()],
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
        let child = CustomKind::host_composed("reference-doc", Extraction::new(Vec::new()));

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
        let child = CustomKind::host_composed("reference-doc", Extraction::new(Vec::new()));

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

        // Three kinds and the nested host's per-host scan read one index.
        assert!(shared.indexed());
    }

    #[test]
    fn unit_dir_resolves_a_host_unit_with_normalized_paths() {
        // `unit_dir` speaks the shared index's vocabulary — normalized paths, no leading
        // `./` on either side — and names the one level under the locus root that owns the
        // entry: the host unit directory a nested child's template addresses an interior of.
        let root = Path::new(".claude/skills");
        let entry = Path::new(".claude/skills/coordinate/SKILL.md");
        assert_eq!(unit_dir(root, entry), Some(root.join("coordinate")));
    }
}
