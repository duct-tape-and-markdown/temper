//! `temper import` — scan a Claude Code harness into the typed config surface.
//!
//! specs/architecture/20-surface.md, "Artifact kinds & contract selection"; custom kinds
//! specs/architecture/40-composition.md.
//!
//! Built-in kinds scan at their real Claude Code locus under `<harness>/.claude/`,
//! so one project-root `harness_path` captures the whole harness; each is projected
//! as one authored document through the generic frontmatter adapter
//! ([`import_frontmatter_member`]). Custom kinds are
//! discovered data-driven off the [`governs`](crate::kind::Governs) locus their
//! authored `.temper/kinds/<name>/KIND.md` declares — spec discovery is a custom
//! kind, not a hardwired scan, so absent a `temper.toml` registration the built-ins
//! import alone. A `<into>/lock.toml` roll-up records one row per artifact.
//!
//! Keystone invariant (`.claude/rules/rust.md`): idempotence. It holds because
//! every write is content-derived, name-sorted, and overwrites in place.

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::fs;
use std::path::{Component, Path, PathBuf};

use ignore::WalkBuilder;
use toml_edit::{ArrayOfTables, DocumentMut, Item, Table, value};

use crate::builtin_kind;
use crate::compose::AuthorLayer;
use crate::document::{self, Document};
use crate::frontmatter::{self, FrontmatterError, Member};
use crate::kind::{CustomKind, Format, Governs, KindError, UnitShape};

/// Filename of the generated roll-up index — the contents' state-of-record —
/// written at the workspace root (`specs/architecture/20-surface.md`, "Topology").
const LOCK_FILENAME: &str = "lock.toml";

/// Errors raised while importing a harness. Distinct from a [`FrontmatterError`]
/// (which a malformed source member produces) by also covering the surface-write
/// side: creating the workspace tree and copying companions.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ImportError {
    /// A source member could not be read or projected through the generic
    /// frontmatter adapter (`specs/architecture/15-kinds.md`).
    #[error(transparent)]
    #[diagnostic(transparent)]
    Frontmatter(#[from] FrontmatterError),

    /// A built-in kind's embedded `KIND.md` failed to parse into its definition —
    /// the `governs` locus discovery keys off. A compiled-in invariant (`build.rs`
    /// generates the table from validated product source), surfaced rather than
    /// panicked so discovery stays panic-free (`.claude/rules/rust.md`).
    #[error(transparent)]
    #[diagnostic(transparent)]
    Kind(#[from] KindError),

    /// A custom-kind unit's source file is not valid UTF-8, so its body cannot be
    /// modelled. (A built-in kind reports this through its own IR; a custom unit is
    /// read here as a raw byte-faithful body, so the check lands in `import`.)
    #[error("{path} is not valid UTF-8")]
    #[diagnostic(code(temper::import::not_utf8))]
    NotUtf8 {
        /// The offending source file.
        path: PathBuf,
        /// The decode error.
        #[source]
        source: std::string::FromUtf8Error,
    },

    /// The harness `skills/` directory could not be enumerated.
    #[error("failed to read harness directory {path}")]
    #[diagnostic(code(temper::import::read_dir))]
    ReadDir {
        /// The directory whose listing failed.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// A surface file or directory could not be written.
    #[error("failed to write {path}")]
    #[diagnostic(code(temper::import::write))]
    Write {
        /// The destination path that failed.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },
}

/// One row of the `lock.toml` roll-up index: an artifact's identity, its source
/// provenance, and the **last-applied fingerprint** the drift/apply merge stands
/// on. Shared by every kind — a `[[skill]]`, `[[rule]]`, and every custom
/// `[[<kind>]]` row all carry the same four columns.
///
/// `pub(crate)` so the `re-add` drift direction can take the row a per-kind writer
/// produced and fold it straight into the lock — reusing `import`'s single
/// round-trip write path rather than re-deriving the fingerprints
/// (`specs/architecture/20-surface.md`, "Drift / apply — three states").
pub(crate) struct RollupEntry {
    /// Artifact name (and its `<kind>/<name>/` surface directory).
    pub(crate) name: String,
    /// Path to the original source file, as given relative to the harness arg.
    pub(crate) source_path: String,
    /// SHA-256 of the authored source bytes — the **source freshness fact**, the
    /// anchor source-drift detection compares against (`specs/architecture/20-surface.md`,
    /// "two freshness facts").
    pub(crate) source_hash: String,
    /// SHA-256 of the last emitted projection — the **emit freshness fact**, the
    /// baseline `config.stale` and projection freshness compare a committed output
    /// against. At import it provisionally equals `source_hash`: no `emit` has run
    /// yet, so the last thing projected onto the source is the source as imported
    /// (`emit` advances it once it lands).
    pub(crate) emit_hash: String,
}

/// Import every built-in artifact plus every declared custom-kind unit under
/// `harness_path` into the surface workspace `into`.
///
/// Idempotent over an unchanged harness. See the module header for the discovery
/// rules and the invariant.
pub fn run(harness_path: &Path, into: &Path) -> miette::Result<()> {
    run_with_builtins(harness_path, into, &builtin_kind::definitions()?)
}

/// Import against an explicit embedded kind set — the seam [`run`] drives with
/// [`builtin_kind::definitions`], factored out so the two-provider co-embedding a bare
/// name (the shadow and qualified-lock-key cases) is testable without waiting on the
/// curated carriers landing in the compiled-in table.
///
/// `builtins` is keyed by qualified identity (`<provider>.<name>`), so two carriers of
/// one bare name are distinct entries.
fn run_with_builtins(
    harness_path: &Path,
    into: &Path,
    builtins: &BTreeMap<String, CustomKind>,
) -> miette::Result<()> {
    // A registration owns its bare name outright (`specs/architecture/15-kinds.md`,
    // "Decision: kind identity carries a provider axis"). Resolve each registered bare
    // name against the embedded set: a UNIQUE embedded carrier is *layered* (the built-in
    // scans below; the registration is a require-side package), a name carried by two or
    // more embedded kinds is *shadowed* (the registration provides its own custom
    // definition and the ambiguous carriers drop out of the built-in scan — "embedded
    // kinds collide among themselves only over references no registration claims"), and a
    // name with no carrier is an ordinary custom kind.
    let layer = AuthorLayer::load(&harness_path.join("temper.toml"))?;
    let shadowed: BTreeSet<String> = layer
        .as_ref()
        .map(|layer| {
            layer
                .registered_kinds()
                .filter(|name| carrier_count(builtins, name) >= 2)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();

    // Built-in scan: every non-shadowed embedded kind that discovers at least one member
    // (a memberless kind writes no section — an empty `ArrayOfTables` vanishes on the toml
    // round-trip, so the skip matches that reality). The section key mirrors
    // `resolve_bare`'s policy: bare while the name is unique among member-discovering
    // kinds, qualified (`<provider>.<name>`) where two carriers of one bare name both
    // discover members, so one carrier's rows never clobber another's.
    let mut discovered: Vec<(String, String, Vec<RollupEntry>)> = Vec::new();
    for kind in builtins.values() {
        if shadowed.contains(&kind.name) {
            continue;
        }
        let rows = import_frontmatter_kind(harness_path, into, kind)?;
        if rows.is_empty() {
            continue;
        }
        discovered.push((kind.name.clone(), kind.qualified_name(), rows));
    }
    let mut bare_counts: BTreeMap<String, usize> = BTreeMap::new();
    for (bare, _, _) in &discovered {
        *bare_counts.entry(bare.clone()).or_default() += 1;
    }
    let mut builtin_rollups: BTreeMap<String, Vec<RollupEntry>> = BTreeMap::new();
    for (bare, qualified, rows) in discovered {
        let key = if bare_counts[&bare] > 1 {
            qualified
        } else {
            bare
        };
        builtin_rollups.insert(key, rows);
    }

    // A custom kind's definition — the `governs` locus discovery keys on — is the
    // authored `<harness>/.temper/kinds/<name>/KIND.md`, not an inline `temper.toml`
    // block (`specs/architecture/40-composition.md`). A registration with a *unique*
    // embedded carrier is a built-in layer, already scanned above; one with none (an
    // ordinary custom kind) or two-plus (shadowing the ambiguous carriers) loads and
    // scans its own definition. Absent a registered custom kind, only the built-ins
    // import.
    let mut custom: BTreeMap<String, Vec<RollupEntry>> = BTreeMap::new();
    if let Some(layer) = &layer {
        let kinds_dir = harness_path.join(".temper").join("kinds");
        for name in layer.registered_kinds() {
            if carrier_count(builtins, name) == 1 {
                continue;
            }
            let kind = CustomKind::load(&kinds_dir, name)?;
            let unit_files = discover_kind_units(harness_path, &kind.governs)?;
            let mut units = Vec::with_capacity(unit_files.len());
            for file in &unit_files {
                units.push(import_custom_unit(&kind, harness_path, file, into)?);
            }
            units.sort_by(|a, b| a.name.cmp(&b.name));
            custom.insert(name.to_string(), units);
        }
    }

    write_rollup(into, &builtin_rollups, &custom)?;

    Ok(())
}

/// How many embedded kinds carry the bare `name` — zero (an ordinary custom kind), one
/// (a layerable built-in), or two-plus (an ambiguous set a registration shadows). The
/// `builtins` map is keyed by qualified identity, so two providers of one bare name count
/// as two.
fn carrier_count(builtins: &BTreeMap<String, CustomKind>, name: &str) -> usize {
    builtins.values().filter(|kind| kind.name == name).count()
}

/// Import every source of one built-in frontmatter kind (`skill`, `rule`) into the
/// surface, driven by the already-parsed `kind` its caller holds. Discover the source
/// files off the kind's `governs` locus, project each through the generic frontmatter
/// adapter, and return the roll-up rows name-sorted for a stable index.
///
/// The parsed `kind` is threaded in, never re-resolved by its bare `name`: [`run`]
/// already holds it from [`builtin_kind::definitions`], so co-embedding two providers
/// of one bare name (`agents-md.memory` + `claude-code.memory`) never re-triggers the
/// `AmbiguousKind` collision on an unrelated scan (`specs/architecture/15-kinds.md`,
/// "Decision: kind identity carries a provider axis" — nobody pays a qualification tax
/// until two providers actually meet).
fn import_frontmatter_kind(
    harness: &Path,
    into: &Path,
    kind: &CustomKind,
) -> Result<Vec<RollupEntry>, ImportError> {
    let files = discover_builtin(harness, kind)?;
    let mut rows = Vec::with_capacity(files.len());
    for file in &files {
        rows.push(import_frontmatter_member(kind, harness, file, into)?);
    }
    rows.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(rows)
}

/// Discover a built-in `kind`'s source files, keying off the `governs` its embedded
/// `KIND.md` declares — the same data-driven scan a custom kind gets, so `skill`/`rule`
/// are no longer hardwired paths (`specs/architecture/15-kinds.md`, "A built-in kind is
/// an adapter": the emit face's locus is the read face's scan root). The `skill` locus
/// (`.claude/skills` + `*/SKILL.md`) resolves through the generalized subdir glob;
/// `rule`'s (`.claude/rules` + `*.md`) is flat. Yields the member source *files* — for a
/// skill the `SKILL.md`, not its directory.
///
/// The parsed `kind` is threaded in from the caller's `definitions()` set, never
/// re-resolved by bare `name`: an unrelated scan over a bare name a second provider also
/// carries must not re-trigger `AmbiguousKind` (`specs/architecture/15-kinds.md`,
/// "Decision: kind identity carries a provider axis").
///
/// The bare-harness-is-a-skill case — a `<harness>/SKILL.md`, a project root that is
/// itself a skill — is Claude Code's own convention, outside the `.claude/skills`
/// locus the `governs` scan covers, so it is layered on for the `skill` kind only.
///
/// `pub(crate)` so drift re-scans the harness, and install's modeline placement
/// targets the same set, through the identical discovery `import` used
/// (`specs/architecture/20-surface.md`, the drift "added" axis).
pub(crate) fn discover_builtin(
    harness: &Path,
    kind: &CustomKind,
) -> Result<Vec<PathBuf>, ImportError> {
    let mut files = discover_kind_units(harness, &kind.governs)?;
    if kind.name == "skill" {
        let bare = harness.join("SKILL.md");
        if bare.is_file() {
            files.push(bare);
            // Re-sort so the bare root skill lands in name order beside the children.
            files.sort();
        }
    }
    Ok(files)
}

/// Read one source member of a frontmatter `kind` and write its surface tree under
/// `<into>/<subdir>/<id>/`, returning the roll-up row for the index. The one write
/// path for every frontmatter kind (`specs/architecture/15-kinds.md`, "the adapter faces are
/// declared"): the member document via [`Member::to_document`], plus the copied
/// companions of a directory-shaped unit. The surface subdir is the kind's declared
/// `governs` leaf, the member document its declared name (`SKILL.md`, `RULE.md`).
///
/// `pub(crate)` so `re-add` reuses this exact round-trip write path when it pulls a
/// drifted or added on-disk source back into the surface, rather than re-implementing
/// the projection (`specs/architecture/20-surface.md`, "Drift / apply").
pub(crate) fn import_frontmatter_member(
    kind: &CustomKind,
    harness: &Path,
    source_file: &Path,
    into: &Path,
) -> Result<RollupEntry, ImportError> {
    // The scan root the member's placement folds against — the same locus discovery
    // scanned it from, so a nested file-shaped unit (a recursive `rule`) gets a
    // placement-folded id rather than a clobbered bare stem (`from_source_rooted`).
    let member = Member::from_source_rooted(kind, source_file, &harness.join(&kind.governs.root))?;
    // A built-in surfaces under the `governs.root` leaf (`.claude/skills` → `skills`),
    // dropping the harness-specific prefix; a custom kind under its full `governs.root`
    // (`docs/adr`). That single derivation is the only thing the two frontmatter faces
    // differ on — the write path below is shared.
    let out_dir = into.join(kind.surface_subdir()).join(&member.id);
    write_member_surface(kind, member, &out_dir, source_file.parent())
}

/// Write a projected [`Member`] to its surface directory `out_dir` and return the
/// roll-up row — the shared write path for every `yaml-frontmatter` kind, built-in or
/// custom. Carry any authored surface layer forward, write the one member document,
/// and copy a directory-shaped unit's companions. The two faces differ only in how
/// `out_dir` is derived (its leaf for a built-in, the full `governs.root` for a custom
/// kind); everything downstream of that is identical, so it rides one path rather than
/// a forked re-implementation (`specs/architecture/15-kinds.md`, "Built-in and custom kinds ride
/// the same adapter").
fn write_member_surface(
    kind: &CustomKind,
    mut member: Member,
    out_dir: &Path,
    source_dir: Option<&Path>,
) -> Result<RollupEntry, ImportError> {
    // Merge, never clobber: the source carries no authored clauses (they are
    // surface-only state), so a re-import or drifted-body `re-add` rebuilds the
    // document from source and would wipe the authored `satisfies`/`edges`. Carry
    // any existing surface layer forward before writing (`specs/architecture/20-surface.md`,
    // "three states, never two").
    let member_doc = kind.member_document();
    if let Some(existing) = existing_surface_member(out_dir, &member_doc) {
        member.carry_representation(&existing);
    }

    create_dir_all(out_dir)?;

    // The member is ONE document: the `+++`-fenced clause-module header over the
    // byte-faithful body, written format-preserving — never a lossy re-serialize.
    write_bytes(
        &out_dir.join(&member_doc),
        member.to_document().emit().as_bytes(),
    )?;

    // A directory-shaped unit's companions ride beside the member document, copied
    // byte-for-byte from the source directory (the member file's parent).
    if let Some(source_dir) = source_dir {
        for companion in &member.companions {
            copy_companion(source_dir, out_dir, companion)?;
        }
    }

    Ok(RollupEntry {
        name: member.id,
        source_path: member.provenance.source_path.to_string_lossy().into_owned(),
        // At import the emit fingerprint provisionally equals the source hash: the
        // source as it stands on disk is exactly what the surface was just derived from,
        // and no `emit` has yet advanced it.
        emit_hash: member.provenance.source_hash.clone(),
        source_hash: member.provenance.source_hash,
    })
}

/// Load an already-written surface member from `out_dir` if one is there — the carrier
/// of the authored surface layer a re-import / `re-add` must preserve. `None` on a
/// first import (the directory does not exist yet) or if the surface is unreadable, so
/// a missing or malformed prior surface degrades to "nothing to carry" rather than
/// failing the write.
fn existing_surface_member(out_dir: &Path, member_doc: &str) -> Option<Member> {
    if !out_dir.join(member_doc).is_file() {
        return None;
    }
    Member::from_surface(out_dir, member_doc).ok()
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
/// declaration (`specs/architecture/40-composition.md`), never a hardwired path.
///
/// `pub(crate)` so the drift engine re-runs the same `governs`-keyed scan against a
/// live harness — every kind's members classify through the identical discovery
/// `import` used (`specs/architecture/20-surface.md`, the drift "added" axis).
pub(crate) fn discover_kind_units(
    harness: &Path,
    governs: &Governs,
) -> Result<Vec<PathBuf>, ImportError> {
    let root = harness.join(&governs.root);
    // A glob is a `/`-separated segment list: the final segment matches files, each
    // earlier one a subdirectory to descend into — a `**` segment descending any
    // number of levels. `split` always yields at least one segment.
    let segments: Vec<&str> = governs.glob.split('/').collect();
    // A member is authored content; an ignored file is by declaration not authored
    // here, so discovery sees only what the repo's ignore rules leave in — else a
    // `**` glob would import a vendored dep's memory file (`specs/architecture/20-surface.md`,
    // "discovery respects ignore rules"). Resolved off the harness (repo) root so a
    // root `.gitignore` governs every kind's walk, whatever its `governs.root` depth.
    let discoverable = discoverable_paths(harness);
    let mut files = Vec::new();
    collect_glob(&root, &segments, &discoverable, &mut files)?;
    // A `**` reaches one file by exactly one path, but `read_dir` order across levels
    // is unspecified; sort for deterministic processing.
    files.sort();
    Ok(files)
}

/// Walk `dir` collecting every file whose path matches the remaining glob
/// `segments`. The head segment selects entries at this level; if it is the last,
/// matching **files** are collected, otherwise matching **subdirectories** are
/// descended. A `**` head is the any-depth wildcard — it matches zero or more
/// directory levels, so a nested nearest-wins hierarchy (the agents.md / `CLAUDE.md`
/// memory nesting) is discovered at every level, not just the fixed glob depth
/// (`specs/architecture/40-composition.md`). A missing or non-directory `dir`
/// contributes nothing — a subdir glob whose intermediate level is absent, or a locus
/// that does not exist on this harness, both resolve to no units rather than an error.
///
/// `discoverable` is the ignore-honoring path set (`.git/` excluded, `.gitignore`
/// respected): a file or subdirectory absent from it is skipped, so no walk descends a
/// vendored tree or collects a member the repo does not consider authored.
fn collect_glob(
    dir: &Path,
    segments: &[&str],
    discoverable: &BTreeSet<PathBuf>,
    out: &mut Vec<PathBuf>,
) -> Result<(), ImportError> {
    if !dir.is_dir() {
        return Ok(());
    }
    let Some((segment, rest)) = segments.split_first() else {
        // `**` recurses with the same segments, so it can bottom out at an empty list
        // (a trailing `**` with nothing left to match): nothing more to collect here.
        return Ok(());
    };
    if *segment == "**" {
        // Zero levels: match the remaining segments right at this level, so
        // `**/AGENTS.md` picks up an `AGENTS.md` directly under the root too.
        collect_glob(dir, rest, discoverable, out)?;
        // One-or-more levels: descend into every subdirectory carrying the `**`, so
        // each nested file is reached by exactly one path (no double-collection). An
        // ignored subdirectory (a vendored tree, `.git/`) is not descended.
        for entry in read_entries(dir)? {
            let path = entry.path();
            if path.is_dir() && discoverable.contains(&normalize(&path)) {
                collect_glob(&path, segments, discoverable, out)?;
            }
        }
        return Ok(());
    }
    for entry in read_entries(dir)? {
        let path = entry.path();
        // An ignored entry is not authored here — skip it whether it would be
        // collected as a file or descended as a subdirectory.
        if !discoverable.contains(&normalize(&path)) {
            continue;
        }
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if !crate::kind::glob_matches(segment, name) {
            continue;
        }
        if rest.is_empty() {
            if path.is_file() {
                out.push(path);
            }
        } else if path.is_dir() {
            collect_glob(&path, rest, discoverable, out)?;
        }
    }
    Ok(())
}

/// The set of paths under `harness` that discovery may see — every file and directory
/// the repo's ignore rules leave in, with `.git/` always excluded
/// (`specs/architecture/20-surface.md`, "discovery respects ignore rules"). Built with
/// ripgrep's `ignore` engine so nested `.gitignore` files, negation, and precedence are
/// honored rather than hand-rolled. Only git's own declaration counts: the machine-global
/// and ripgrep-specific (`.ignore`) sources are off, and parent directories above the
/// harness are not consulted — the harness is the per-project boundary. `require_git` is
/// off so a `.gitignore` is honored even when the harness is not itself a git checkout
/// (a sub-tree, or a test fixture). Paths are normalized so a `.`-rooted `governs`
/// (`root = "."`) compares equal to the walk's harness-relative entries.
fn discoverable_paths(harness: &Path) -> BTreeSet<PathBuf> {
    let mut allowed = BTreeSet::new();
    let walk = WalkBuilder::new(harness)
        .hidden(false) // `.claude/` is a dotdir the harness lives in — never hide it.
        .parents(false)
        .ignore(false)
        .git_global(false)
        .git_ignore(true)
        .git_exclude(true)
        .require_git(false)
        .filter_entry(|entry| entry.file_name() != OsStr::new(".git"))
        .build();
    // A walk error (an unreadable entry) drops that entry rather than aborting
    // discovery — the same tolerance the raw scan takes on `read_dir`.
    for entry in walk.flatten() {
        allowed.insert(normalize(entry.path()));
    }
    allowed
}

/// `path` with any `.` (current-dir) components dropped, so a walk entry and a
/// `harness.join(".")`-rooted discovery path denote the same key in the discoverable
/// set. Only a standalone `.` component is stripped — a dotted name (`.claude`) is a
/// normal component and survives.
fn normalize(path: &Path) -> PathBuf {
    path.components()
        .filter(|component| !matches!(component, Component::CurDir))
        .collect()
}

/// Read `dir`'s entries into a vector, mapping any failure to an
/// [`ImportError::ReadDir`]. Collected eagerly so a level can be scanned twice — the
/// `**` wildcard both matches files at a level and descends its subdirectories —
/// without re-implementing the error mapping at each read.
fn read_entries(dir: &Path) -> Result<Vec<fs::DirEntry>, ImportError> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(dir).map_err(|source| ImportError::ReadDir {
        path: dir.to_path_buf(),
        source,
    })? {
        entries.push(entry.map_err(|source| ImportError::ReadDir {
            path: dir.to_path_buf(),
            source,
        })?);
    }
    Ok(entries)
}

/// Read one discovered custom-kind unit and write its surface tree under
/// `<into>/<governs.root>/<id>/`, returning the roll-up row for the index. The
/// declared adapter faces are **load-bearing** here (`specs/architecture/15-kinds.md`, "the
/// adapter faces are declared"): the id derives per the kind's
/// [`unit_shape`](UnitShape) (the file stem for `File`, the directory name for
/// `Directory`), and the artifact is split per its [`format`](Format).
///
/// - `yaml-frontmatter` rides the same generic adapter a built-in does
///   ([`Member::from_source`]): the declared `field`s lift into `[clause.<field>]`
///   header tables, the body is byte-faithful below the frontmatter, and a
///   directory-shaped unit's companions travel along.
/// - No declared format keeps the *whole* file byte-faithful as the body under a
///   `[provenance]`-only header — the frontmatterless shape (a spec), where a unit's
///   leading `---`, if any, is prose to preserve, not frontmatter to lift.
///
/// `pub(crate)` for the same reason as [`import_frontmatter_member`]: `re-add` reuses
/// this exact generic write path to reconcile a drifted or added on-disk custom-kind
/// unit into the surface, folding the returned row straight into the lock.
pub(crate) fn import_custom_unit(
    kind: &CustomKind,
    harness: &Path,
    source_file: &Path,
    into: &Path,
) -> Result<RollupEntry, ImportError> {
    match kind.format {
        Some(Format::YamlFrontmatter) => {
            // Fold placement against the `governs` root so a nested same-named unit
            // (`sub/AGENTS.md`) keeps a distinct surface id, symmetric with the
            // whole-file path below (`from_source_rooted`, `wholefile_id`).
            let member =
                Member::from_source_rooted(kind, source_file, &harness.join(&kind.governs.root))?;
            let out_dir = into.join(&kind.governs.root).join(&member.id);
            write_member_surface(kind, member, &out_dir, source_file.parent())
        }
        None => import_wholefile_unit(kind, harness, source_file, into),
    }
}

/// Project a custom unit whose kind declares **no** format: the whole source file is
/// the byte-faithful body under a `[provenance]`-only `+++` header, so a unit's
/// frontmatter — or a leading `---` that is really prose (a spec) — survives verbatim
/// for its extractor to read at `check` time (`specs/architecture/15-kinds.md`). The id derives
/// per the kind's [`unit_shape`](UnitShape), and a directory-shaped unit's companions
/// ride beside the body, so a `Directory`-shaped frontmatterless kind is imported whole
/// rather than losing its siblings.
fn import_wholefile_unit(
    kind: &CustomKind,
    harness: &Path,
    source_file: &Path,
    into: &Path,
) -> Result<RollupEntry, ImportError> {
    let bytes = fs::read(source_file).map_err(|source| ImportError::ReadDir {
        path: source_file.to_path_buf(),
        source,
    })?;
    let source_hash = crate::hash::sha256_hex(&bytes);
    let body = String::from_utf8(bytes).map_err(|source| ImportError::NotUtf8 {
        path: source_file.to_path_buf(),
        source,
    })?;
    let name = wholefile_id(kind, &harness.join(&kind.governs.root), source_file)?;

    let out_dir = into.join(&kind.governs.root).join(&name);
    create_dir_all(&out_dir)?;

    // The member is ONE document: the `+++` header over the whole byte-faithful
    // source file as the body. Merge, never clobber (`specs/architecture/20-surface.md`,
    // "three states, never two"): a whole-file unit builds no `Member`, so the authored
    // `[requirement.*]`/`[satisfies.*]` tables live only in the existing surface's
    // header — carry them forward before writing, re-stamping only `[provenance]`,
    // exactly as the frontmatter path does via `carry_representation`.
    let body_path = out_dir.join(body_filename(&kind.name));
    let header = custom_unit_header(&body_path, &source_file.to_string_lossy(), &source_hash);
    write_bytes(&body_path, Document::new(header, body).emit().as_bytes())?;

    // A directory-shaped unit's companions ride beside the body, byte-for-byte — the
    // same treatment the frontmatter adapter gives them, so the shape is honored
    // regardless of whether the kind declares a format.
    if kind.unit_shape == Some(UnitShape::Directory)
        && let Some(source_dir) = source_file.parent()
    {
        let member_name = source_file.file_name().unwrap_or_else(|| OsStr::new(""));
        for companion in frontmatter::scan_companions(source_dir, member_name)? {
            copy_companion(source_dir, &out_dir, &companion)?;
        }
    }

    Ok(RollupEntry {
        name,
        source_path: source_file.to_string_lossy().into_owned(),
        // At import the emit fingerprint provisionally equals the source hash (see the
        // built-in frontmatter member writer).
        emit_hash: source_hash.clone(),
        source_hash,
    })
}

/// The member id for a whole-file custom unit, derived per the kind's declared
/// [`unit_shape`](UnitShape): the parent directory name for `Directory`, and for `File`
/// (or an absent shape, defaulting to a lone file) the placement-folded stem relative to
/// the `governs`-root directory `base` (`frontmatter::fold_file_id`). It shares that
/// fold with the frontmatter face ([`Member::from_source_rooted`]), so both faces name a
/// nested unit the same way — a nested `sub/AGENTS.md` gets a distinct id, not a
/// clobbered bare stem. A source path yielding no id component for its shape is a
/// [`FrontmatterError::NoId`], the same error the frontmatter face raises.
fn wholefile_id(kind: &CustomKind, base: &Path, source_file: &Path) -> Result<String, ImportError> {
    let id = match kind.unit_shape {
        Some(UnitShape::Directory) => source_file
            .parent()
            .filter(|dir| !dir.as_os_str().is_empty())
            .and_then(Path::file_name)
            .and_then(OsStr::to_str)
            .ok_or(FrontmatterError::NoId {
                path: source_file.to_path_buf(),
                shape: "directory",
            })?
            .to_string(),
        Some(UnitShape::File) | None => frontmatter::fold_file_id(base, source_file)?,
    };
    Ok(id)
}

/// The header to emit for a custom unit: carry every authored clause table from an
/// already-written surface document at `body_path` forward, re-stamping only the
/// generated `[provenance]` module with the fresh drift anchor. This makes the
/// custom-unit path **merge rather than clobber** (`specs/architecture/20-surface.md`, "three
/// states, never two") — symmetric with the frontmatter path's `carry_representation`
/// — so a re-import or drifted-body `re-add` preserves the hand-authored
/// `[requirement.*]`/`[satisfies.*]` tables instead of wiping them under a bare
/// provenance header. A first import (or an unreadable/malformed prior surface)
/// degrades to a fresh provenance-only header.
fn custom_unit_header(body_path: &Path, source_path: &str, source_hash: &str) -> DocumentMut {
    let mut header = existing_custom_header(body_path).unwrap_or_default();
    // Provenance is always freshly generated (the source freshness fact), never carried —
    // drop any carried copy so the re-stamp lands it last, below the authored tables.
    header.as_table_mut().remove("provenance");
    document::add_provenance(&mut header, source_path, source_hash);
    header
}

/// Parse the header of an already-written custom-unit surface document at `path`, or
/// `None` if it is absent, unreadable, or malformed — the carrier of the authored
/// clause tables a re-import must preserve. A missing or malformed prior surface
/// degrades to "nothing to carry" rather than failing the write, mirroring
/// [`existing_surface_member`] on the frontmatter path.
fn existing_custom_header(path: &Path) -> Option<DocumentMut> {
    let raw = fs::read_to_string(path).ok()?;
    Some(Document::parse(&raw).ok()?.header().clone())
}

/// The byte-faithful body filename for a custom kind — the kind name upper-cased
/// with a `.md` suffix (`spec` → `SPEC.md`), mirroring the built-in `SKILL.md` and
/// `RULE.md` bodies so a custom kind's surface reads uniformly with them.
fn body_filename(kind: &str) -> String {
    format!("{}.md", kind.to_uppercase())
}

/// Copy a single companion from the source dir to the surface dir, byte-for-byte,
/// creating any intermediate directories.
fn copy_companion(source_dir: &Path, out_dir: &Path, relative: &Path) -> Result<(), ImportError> {
    let from = source_dir.join(relative);
    let to = out_dir.join(relative);
    if let Some(parent) = to.parent() {
        create_dir_all(parent)?;
    }
    fs::copy(&from, &to).map_err(|source| ImportError::Write { path: to, source })?;
    Ok(())
}

/// Write the `<into>/lock.toml` roll-up: one `[[<kind>]]` table per imported member —
/// the built-in kinds first (key-sorted) then the custom kinds (name-sorted) — each with
/// `name`, `source_path`, `source_hash`, and the `emit_hash` fingerprint. Both maps
/// are key-sorted, so the emitted order is deterministic and a third embedded built-in
/// kind takes its own slot with no code change here. A built-in section's key is the bare
/// name, or the qualified `<provider>.<name>` where two carriers of one bare name both
/// discovered members ([`run_with_builtins`]).
///
/// The caller filters memberless built-in kinds before this point, matching the toml
/// round-trip reality: an empty `ArrayOfTables` emits nothing, so a written-then-vanished
/// section would break idempotence against a re-parse that never sees it.
fn write_rollup(
    into: &Path,
    builtins: &BTreeMap<String, Vec<RollupEntry>>,
    custom: &BTreeMap<String, Vec<RollupEntry>>,
) -> Result<(), ImportError> {
    let mut doc = DocumentMut::new();
    for (kind, rows) in builtins {
        doc[kind.as_str()] = Item::ArrayOfTables(rollup_tables(rows));
    }
    for (kind, units) in custom {
        doc[kind.as_str()] = Item::ArrayOfTables(rollup_tables(units));
    }

    create_dir_all(into)?;
    write_bytes(&into.join(LOCK_FILENAME), doc.to_string().as_bytes())
}

/// Build the `ArrayOfTables` for one kind's roll-up rows — the four shared columns
/// (`name`, `source_path`, `source_hash`, `emit_hash`) in a fixed order, one
/// table per entry.
fn rollup_tables(rollup: &[RollupEntry]) -> ArrayOfTables {
    let mut tables = ArrayOfTables::new();
    for entry in rollup {
        let mut table = Table::new();
        table["name"] = value(entry.name.clone());
        table["source_path"] = value(entry.source_path.clone());
        table["source_hash"] = value(entry.source_hash.clone());
        table["emit_hash"] = value(entry.emit_hash.clone());
        tables.push(table);
    }
    tables
}

/// `fs::create_dir_all`, mapping failure to an [`ImportError::Write`].
fn create_dir_all(path: &Path) -> Result<(), ImportError> {
    fs::create_dir_all(path).map_err(|source| ImportError::Write {
        path: path.to_path_buf(),
        source,
    })
}

/// `fs::write`, mapping failure to an [`ImportError::Write`].
fn write_bytes(path: &Path, bytes: &[u8]) -> Result<(), ImportError> {
    fs::write(path, bytes).map_err(|source| ImportError::Write {
        path: path.to_path_buf(),
        source,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::sync::atomic::{AtomicU32, Ordering};

    use toml_edit::DocumentMut;

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    /// A fresh, empty temp directory unique to this test run.
    fn tmpdir(label: &str) -> PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "author-import-{}-{}-{}",
            std::process::id(),
            id,
            label
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
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

    /// A spec body whose leading `---` is prose (a spec has no frontmatter) and
    /// whose missing final newline must survive intact.
    const SURFACE_SPEC: &str = "# The config surface\n\
\n\
---\n\
\n\
The surface is temper's composition write surface, no trailing newline.";

    /// A second spec so the roll-up carries more than one row and name-sorting is
    /// observable (`00-intent` sorts before `20-surface`).
    const INTENT_SPEC: &str = "# Intent\n\nThe north star.\n";

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

    /// A `temper.toml` *registering* `spec` as a custom kind — the whole require-side
    /// wiring is the package binding; the definition (the `governs` locus discovery
    /// keys on) lives in the authored `.temper/kinds/spec/KIND.md` fixture below
    /// (`specs/architecture/40-composition.md`, "Decision: a custom kind is an authored `.temper/`
    /// artifact, registered in the assembly").
    const SPEC_TEMPER_TOML: &str = "[kind.spec]\npackage = \"spec\"\n";

    /// The authored `spec` KIND.md definition (`specs/architecture/20-surface.md`, "Decision: a kind
    /// definition is `KIND.md`"): the `+++` header carries the `governs` locus and the
    /// composed extraction, the body the kind's prose. `import` reads the locus to
    /// discover units.
    const SPEC_KIND_MD: &str = "+++\n\
governs = { root = \"specs\", glob = \"*.md\" }\n\
\n\
[[extraction]]\n\
primitive = \"line_count\"\n\
\n\
[[extraction]]\n\
primitive = \"headings\"\n\
+++\n\
\n\
# The spec kind\n\
\n\
temper's own governing documents.\n";

    /// Add a `specs/` corpus to an existing harness root, plus the `temper.toml`
    /// registration and the authored `.temper/kinds/spec/KIND.md` definition so
    /// discovery finds it: two spec files plus a non-markdown loose file and a
    /// subdirectory, both of which discovery skips.
    fn write_specs(root: &Path) {
        fs::write(root.join("temper.toml"), SPEC_TEMPER_TOML).unwrap();
        let kind_dir = root.join(".temper").join("kinds").join("spec");
        fs::create_dir_all(&kind_dir).unwrap();
        fs::write(kind_dir.join("KIND.md"), SPEC_KIND_MD).unwrap();
        let specs = root.join("specs");
        fs::create_dir_all(specs.join("notes")).unwrap();
        fs::write(specs.join("20-surface.md"), SURFACE_SPEC).unwrap();
        fs::write(specs.join("00-intent.md"), INTENT_SPEC).unwrap();
        // Noise that must be ignored: a non-`.md` file and a subdirectory.
        fs::write(specs.join("README.txt"), "not a spec\n").unwrap();
        fs::write(specs.join("notes").join("scratch.md"), "nested, skipped\n").unwrap();
    }

    /// Snapshot every file under `dir` as a sorted map of relative path -> bytes,
    /// so two imports can be compared for an exact byte diff.
    fn tree_bytes(dir: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
        let mut out = BTreeMap::new();
        for entry in walkdir::WalkDir::new(dir).min_depth(1).sort_by_file_name() {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                let rel = entry.path().strip_prefix(dir).unwrap().to_path_buf();
                out.insert(rel, fs::read(entry.path()).unwrap());
            }
        }
        out
    }

    #[test]
    fn writes_the_expected_surface_tree() {
        let harness = tmpdir("tree-src");
        write_fixture_harness(&harness);
        let into = tmpdir("tree-into");

        run(&harness, &into).unwrap();

        // Per-skill surface dirs each hold ONE member document — no meta.toml.
        let coord = into.join("skills").join("coordinate");
        assert!(coord.join("SKILL.md").is_file());
        assert!(!coord.join("meta.toml").exists());
        assert!(into.join("skills").join("demo").join("SKILL.md").is_file());
        assert!(into.join("lock.toml").is_file());

        // The member document is the `+++` clause-module header over the byte-faithful
        // body, which reloads back to the source member through the generic adapter.
        let reloaded = Member::from_surface(&coord, "SKILL.md").unwrap();
        assert_eq!(reloaded.id, "coordinate");
        assert_eq!(
            reloaded.field("license").and_then(|v| v.as_str()),
            Some("MIT")
        );
        assert!(reloaded.has_field("allowed-tools"));
        assert_eq!(
            reloaded.body,
            "# Coordinate\n\nSee PLAYBOOK.md for the full reference.   \nNo trailing newline here."
        );
    }

    #[test]
    fn copies_companions_byte_for_byte() {
        let harness = tmpdir("comp-src");
        write_fixture_harness(&harness);
        let into = tmpdir("comp-into");

        run(&harness, &into).unwrap();

        let coord = into.join("skills").join("coordinate");
        assert_eq!(fs::read(coord.join("PLAYBOOK.md")).unwrap(), PLAYBOOK);
        assert_eq!(
            fs::read(coord.join("scripts").join("run.sh")).unwrap(),
            SCRIPT
        );
    }

    #[test]
    fn rollup_lists_one_entry_per_skill_with_four_columns() {
        let harness = tmpdir("roll-src");
        write_fixture_harness(&harness);
        let into = tmpdir("roll-into");

        run(&harness, &into).unwrap();

        let doc = fs::read_to_string(into.join("lock.toml"))
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();
        let skills = doc["skill"].as_array_of_tables().unwrap();

        // One entry per skill, name-sorted, each carrying the four production columns.
        let names: Vec<&str> = skills.iter().map(|t| t["name"].as_str().unwrap()).collect();
        assert_eq!(names, vec!["coordinate", "demo"]);

        for table in skills.iter() {
            let source_hash = table["source_hash"].as_str().unwrap();
            let emit_hash = table["emit_hash"].as_str().unwrap();
            assert_eq!(source_hash.len(), 64);
            assert!(table["source_path"].as_str().unwrap().ends_with("SKILL.md"));
            // The retired `body_hash` column is gone — no production reader.
            assert!(table.get("body_hash").is_none());
            // The retired pre-rename column names are gone too.
            assert!(table.get("import_hash").is_none());
            assert!(table.get("last_applied").is_none());
            // The baseline: at import the emit fingerprint provisionally equals the
            // source hash — the surface was just derived from the source as it stands,
            // and no `emit` has advanced it.
            assert_eq!(emit_hash, source_hash);
        }
    }

    #[test]
    fn import_is_idempotent() {
        let harness = tmpdir("idem-src");
        write_fixture_harness(&harness);
        let into = tmpdir("idem-into");

        run(&harness, &into).unwrap();
        let first = tree_bytes(&into);

        // A second import into the same workspace must not change a single byte.
        run(&harness, &into).unwrap();
        let second = tree_bytes(&into);

        assert_eq!(first, second);
    }

    #[test]
    fn writes_a_rule_surface_and_rollup_row() {
        let harness = tmpdir("rule-src");
        write_fixture_harness(&harness);
        let into = tmpdir("rule-into");

        run(&harness, &into).unwrap();

        // The rule surface mirrors a skill: a `rules/<name>/` dir holding ONE member
        // document `RULE.md`, no meta.toml.
        let rust = into.join("rules").join("rust");
        assert!(rust.join("RULE.md").is_file());
        assert!(!rust.join("meta.toml").exists());

        // The document round-trips back to the source rule (paths + the preserved
        // Cursor key), body byte-faithful below the header.
        let reloaded = Member::from_surface(&rust, "RULE.md").unwrap();
        assert_eq!(reloaded.id, "rust");
        assert_eq!(
            reloaded.field("paths"),
            Some(&serde_json::json!(["src/**/*.rs"]))
        );
        assert!(reloaded.has_field("description"));
        assert_eq!(
            reloaded.body,
            "# Rust conventions\n\nPrefer a clone over a lifetime fight.   \nLast line, no newline."
        );

        // A no-frontmatter rule carries its whole body byte-faithful below the header.
        let collab = into.join("rules").join("collaboration");
        assert_eq!(
            Member::from_surface(&collab, "RULE.md").unwrap().body,
            COLLAB_RULE
        );

        // The roll-up carries a `[[rule]]` row per rule, name-sorted, alongside
        // the `[[skill]]` rows — both kinds coexist in one import.
        let doc = fs::read_to_string(into.join("lock.toml"))
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();
        let skills = doc["skill"].as_array_of_tables().unwrap();
        let skill_names: Vec<&str> = skills.iter().map(|t| t["name"].as_str().unwrap()).collect();
        assert_eq!(skill_names, vec!["coordinate", "demo"]);

        let rules = doc["rule"].as_array_of_tables().unwrap();
        let rule_names: Vec<&str> = rules.iter().map(|t| t["name"].as_str().unwrap()).collect();
        assert_eq!(rule_names, vec!["collaboration", "rust"]);
        for table in rules.iter() {
            assert_eq!(table["source_hash"].as_str().unwrap().len(), 64);
            assert!(table.get("body_hash").is_none());
            assert!(table["source_path"].as_str().unwrap().ends_with(".md"));
        }
    }

    #[test]
    fn builtin_discovery_keys_off_the_embedded_kind_governs() {
        // Discovery is driven by the embedded `skill`/`rule` KIND.md `governs`, not a
        // hardwired path: the skill `*/SKILL.md` subdir glob and the rule `*.md` flat
        // glob both resolve through the one generalized scanner.
        let harness = tmpdir("gov-src");
        write_fixture_harness(&harness);

        let skill_kind = builtin_kind::definition("skill").unwrap().unwrap();
        let rule_kind = builtin_kind::definition("rule").unwrap().unwrap();

        // The skill locus (`.claude/skills` + `*/SKILL.md`) yields the `SKILL.md`
        // files themselves — the subdir glob descended one level.
        let skills = discover_builtin(&harness, &skill_kind).unwrap();
        assert_eq!(
            skills,
            vec![
                harness.join(".claude/skills/coordinate").join("SKILL.md"),
                harness.join(".claude/skills/demo").join("SKILL.md"),
            ]
        );

        // The rule locus (`.claude/rules` + `*.md`) is flat — immediate `*.md` files.
        let rules = discover_builtin(&harness, &rule_kind).unwrap();
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
        // its bare `name` against the embedded set — so a kind whose bare name a second
        // provider also carried would still scan without paying the collision
        // (`specs/architecture/15-kinds.md`, "Decision: kind identity carries a provider
        // axis"). Proven with a synthetic `memory` kind: a bare name absent from today's
        // embedded table, so a by-name re-resolution would find nothing, yet threading
        // the parsed kind through discovers its member off the kind's own locus.
        let harness = tmpdir("threaded-discovery");
        fs::create_dir_all(harness.join("mem")).unwrap();
        fs::write(harness.join("mem").join("CLAUDE.md"), "# root\n").unwrap();

        let src = "governs = { root = \"mem\", glob = \"*.md\" }\nprovider = \"claude-code\"\n";
        let doc = src.parse::<DocumentMut>().unwrap();
        let memory =
            CustomKind::from_header(doc.as_table(), "memory", Path::new("kinds/x/y/KIND.md"))
                .unwrap();

        let found = discover_builtin(&harness, &memory).unwrap();
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
        let found = discover_kind_units(&harness, &governs).unwrap();
        assert_eq!(
            found,
            vec![
                root.join("alpha").join("THING.md"),
                root.join("beta").join("THING.md"),
            ]
        );
    }

    #[test]
    fn imports_a_bare_harness_that_is_itself_a_skill() {
        // A `<harness>` whose own SKILL.md makes it a skill dir, with no skills/. Its
        // directory name is the member id (`directory` shape), so the harness dir is
        // named for the skill — the real bare-skill-repo shape, not a tmpdir artifact.
        let harness = tmpdir("bare-src").join("demo");
        fs::create_dir_all(&harness).unwrap();
        fs::write(harness.join("SKILL.md"), DEMO).unwrap();
        let into = tmpdir("bare-into");

        run(&harness, &into).unwrap();

        assert!(into.join("skills").join("demo").join("SKILL.md").is_file());
        let doc = fs::read_to_string(into.join("lock.toml"))
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();
        assert_eq!(doc["skill"].as_array_of_tables().unwrap().len(), 1);
    }

    #[test]
    fn skips_non_skill_dirs_and_files() {
        let harness = tmpdir("skip-src");
        write_fixture_harness(&harness);
        // Noise that must be ignored: a loose file and a dir without SKILL.md.
        fs::write(
            harness.join(".claude").join("skills").join("README.md"),
            "not a skill\n",
        )
        .unwrap();
        fs::create_dir_all(harness.join(".claude").join("skills").join("empty")).unwrap();

        let into = tmpdir("skip-into");
        run(&harness, &into).unwrap();

        let doc = fs::read_to_string(into.join("lock.toml"))
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();
        assert_eq!(doc["skill"].as_array_of_tables().unwrap().len(), 2);
        assert!(!into.join("skills").join("empty").exists());
    }

    #[test]
    fn writes_a_spec_surface_and_rollup_row() {
        // A harness whose `temper.toml` declares the `spec` custom kind: discovery
        // is data-driven off its `governs` locus, not a hardwired scan.
        let harness = tmpdir("spec-src");
        write_fixture_harness(&harness);
        write_specs(&harness);
        let into = tmpdir("spec-into");

        run(&harness, &into).unwrap();

        // The spec surface mirrors a rule: a `specs/<name>/` dir holding ONE member
        // document `SPEC.md` — a provenance-only `+++` header over the whole file.
        let surface = into.join("specs").join("20-surface");
        assert!(surface.join("SPEC.md").is_file());
        assert!(!surface.join("meta.toml").exists());
        // The document begins with the provenance header; the body below it is the
        // *entire* source — a spec has no frontmatter, so the leading `---` is prose
        // and the missing final newline is preserved.
        let document = fs::read_to_string(surface.join("SPEC.md")).unwrap();
        assert!(document.starts_with("+++\n[provenance]\n"));
        assert!(document.ends_with(SURFACE_SPEC));

        // The generic custom-unit surface round-trips back through the generic
        // unit loader (`crate::kind::Unit`) — a custom kind carries no bespoke IR,
        // so `import`'s output is read by the same reader `check` uses. The unit body
        // is the whole source file, below the header.
        let unit = crate::kind::Unit::from_surface_dir(&surface).unwrap();
        assert_eq!(unit.id, "20-surface");
        assert_eq!(unit.body, SURFACE_SPEC);

        // The roll-up carries a `[[spec]]` row per spec, name-sorted, alongside
        // the skill and rule rows — all three kinds coexist in one import. The
        // `notes/` subdir and `README.txt` are skipped (immediate `*.md` only).
        let doc = fs::read_to_string(into.join("lock.toml"))
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();
        let specs = doc["spec"].as_array_of_tables().unwrap();
        let spec_names: Vec<&str> = specs.iter().map(|t| t["name"].as_str().unwrap()).collect();
        assert_eq!(spec_names, vec!["00-intent", "20-surface"]);
        for table in specs.iter() {
            assert_eq!(table["source_hash"].as_str().unwrap().len(), 64);
            assert!(table.get("body_hash").is_none());
            assert!(table["source_path"].as_str().unwrap().ends_with(".md"));
        }

        // A second import into the same workspace must not change a single byte.
        let first = tree_bytes(&into);
        run(&harness, &into).unwrap();
        assert_eq!(first, tree_bytes(&into));
    }

    #[test]
    fn imports_a_generic_custom_kind_via_its_governs_locus() {
        // A custom kind that is not `spec`: an `adr` kind under a nested `docs/adr`
        // root. Discovery is generic — the surface dir is the `governs` root, the
        // body file is the kind name upper-cased, and the roll-up key is the kind
        // name — so nothing is special-cased on `spec`.
        let harness = tmpdir("adr-src");
        write_fixture_harness(&harness);
        let adr_dir = harness.join("docs").join("adr");
        fs::create_dir_all(&adr_dir).unwrap();
        let adr_body = "# ADR 0001 — adopt the surface\n\nContext, decision, no final newline.";
        fs::write(adr_dir.join("0001-surface.md"), adr_body).unwrap();
        // Noise the glob must skip: a non-`.md` sibling.
        fs::write(adr_dir.join("index.txt"), "not an adr\n").unwrap();
        // Register the `adr` kind and author its definition under `.temper/kinds/adr/`.
        fs::write(
            harness.join("temper.toml"),
            "[kind.adr]\npackage = \"adr\"\n",
        )
        .unwrap();
        let adr_kind_dir = harness.join(".temper").join("kinds").join("adr");
        fs::create_dir_all(&adr_kind_dir).unwrap();
        fs::write(
            adr_kind_dir.join("KIND.md"),
            "+++\ngoverns = { root = \"docs/adr\", glob = \"*.md\" }\n+++\n# The adr kind\n",
        )
        .unwrap();

        let into = tmpdir("adr-into");
        run(&harness, &into).unwrap();

        // The unit lands at `<into>/<root>/<name>/` as ONE member document `<KIND>.md`
        // — a provenance-only `+++` header over the whole file byte-faithful.
        let surface = into.join("docs").join("adr").join("0001-surface");
        assert!(surface.join("ADR.md").is_file());
        assert!(!surface.join("meta.toml").exists());
        let document = fs::read_to_string(surface.join("ADR.md")).unwrap();
        assert!(document.starts_with("+++\n[provenance]\n"));
        assert!(document.ends_with(adr_body));

        // The roll-up carries an `[[adr]]` row — keyed by the kind name — while the
        // non-`.md` sibling is skipped.
        let doc = fs::read_to_string(into.join("lock.toml"))
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();
        let adrs = doc["adr"].as_array_of_tables().unwrap();
        let names: Vec<&str> = adrs.iter().map(|t| t["name"].as_str().unwrap()).collect();
        assert_eq!(names, vec!["0001-surface"]);
        assert!(
            adrs.iter()
                .all(|t| t["source_hash"].as_str().unwrap().len() == 64)
        );
    }

    #[test]
    fn no_declared_custom_kind_imports_builtins_only() {
        // The base fixture carries skills and rules and a `specs/` corpus on disk,
        // but NO `temper.toml` declaring the `spec` kind — so discovery is
        // data-driven to nothing: the built-ins import, the `specs/` are ignored.
        // This is the guarantee that the old hardwired scan is gone.
        let harness = tmpdir("nospec-src");
        write_fixture_harness(&harness);
        let specs = harness.join("specs");
        fs::create_dir_all(&specs).unwrap();
        fs::write(specs.join("20-surface.md"), SURFACE_SPEC).unwrap();
        let into = tmpdir("nospec-into");

        run(&harness, &into).unwrap();

        // No spec surfaces are written — absent a `temper.toml` custom kind there
        // is no phantom `specs/` scan, even with a `specs/` corpus on disk.
        assert!(!into.join("specs").exists());
        let lock = fs::read_to_string(into.join("lock.toml")).unwrap();
        assert!(!lock.contains("[[spec]]"));
        let doc = lock.parse::<DocumentMut>().unwrap();
        assert!(doc.get("spec").is_none());
        // The built-ins are still imported — the skill and rule rows are present.
        assert_eq!(doc["skill"].as_array_of_tables().unwrap().len(), 2);
        assert_eq!(doc["rule"].as_array_of_tables().unwrap().len(), 2);
    }

    #[test]
    fn builtin_scan_is_generic_over_the_embedded_kind_set() {
        // The scan is driven off `builtin_kind::definitions()`, not the old
        // `["skill","rule"]` literal: an embedded built-in kind gets its own roll-up
        // section — keyed by its bare name while unique — so a third embedded kind would
        // be discovered here without a code change. A section is written *exactly* for a
        // kind that discovered members, never for every embedded kind: an empty
        // `ArrayOfTables` vanishes on the toml round-trip, so a memberless section could
        // not survive a re-parse anyway.
        let harness = tmpdir("generic-src");
        write_fixture_harness(&harness);
        let into = tmpdir("generic-into");

        run(&harness, &into).unwrap();

        let doc = fs::read_to_string(into.join("lock.toml"))
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();

        // The embedded set carries at least skill and rule, and the fixture gives each
        // members — so each writes a bare-keyed section (unique among member-discovering
        // kinds). No section exists for a kind that discovered nothing.
        let builtins = crate::builtin_kind::definitions().unwrap();
        let bare_names: Vec<&str> = builtins.values().map(|kind| kind.name.as_str()).collect();
        assert!(
            bare_names.contains(&"skill") && bare_names.contains(&"rule"),
            "the embedded set must carry at least skill and rule"
        );
        for name in ["skill", "rule"] {
            assert!(
                doc.get(name)
                    .and_then(|item| item.as_array_of_tables())
                    .is_some_and(|rows| !rows.is_empty()),
                "roll-up is missing the section for member-discovering kind `{name}`"
            );
        }

        // Skill and rule members are discovered exactly as the hardcoded pair did.
        let skills: Vec<&str> = doc["skill"]
            .as_array_of_tables()
            .unwrap()
            .iter()
            .map(|t| t["name"].as_str().unwrap())
            .collect();
        assert_eq!(skills, vec!["coordinate", "demo"]);
        let rules: Vec<&str> = doc["rule"]
            .as_array_of_tables()
            .unwrap()
            .iter()
            .map(|t| t["name"].as_str().unwrap())
            .collect();
        assert_eq!(rules, vec!["collaboration", "rust"]);

        // The roll-up is idempotent: a second import into the same workspace does not
        // change a single byte of its deterministic, name-sorted layout.
        let first = tree_bytes(&into);
        run(&harness, &into).unwrap();
        assert_eq!(first, tree_bytes(&into));
    }

    /// A `<provider>`-qualified embedded carrier of a given bare `name`, scanning `root`
    /// with `glob` — the shape the compiled-in table gains once the curated carriers land.
    /// A built-in kind rides the frontmatter face, so it needs no declared `format`.
    fn embedded_carrier(name: &str, provider: &str, root: &str, glob: &str) -> CustomKind {
        let src = format!(
            "governs = {{ root = \"{root}\", glob = \"{glob}\" }}\nprovider = \"{provider}\"\n"
        );
        let doc = src.parse::<DocumentMut>().unwrap();
        CustomKind::from_header(doc.as_table(), name, Path::new("kinds/x/y/KIND.md")).unwrap()
    }

    #[test]
    fn a_registration_shadows_two_embedded_carriers_of_its_bare_name() {
        // Two providers co-embed a bare `memory` kind; the project registers its own
        // `memory` custom kind over `**/MEMORY.md`. Registration owns the bare name: the
        // embedded carriers are shadowed and the project kind imports its members — never
        // silently preempted (`specs/architecture/15-kinds.md`, "Decision: kind identity
        // carries a provider axis").
        let harness = tmpdir("shadow-src");
        write_fixture_harness(&harness);
        register_custom_kind(
            &harness,
            "memory",
            "+++\n\
governs = { root = \"memory\", glob = \"**/MEMORY.md\" }\n\
unit_shape = \"file\"\n\
+++\n\
# The memory kind\n",
        );
        let mem = harness.join("memory");
        fs::create_dir_all(mem.join("api")).unwrap();
        fs::write(mem.join("MEMORY.md"), "root memory\n").unwrap();
        fs::write(mem.join("api").join("MEMORY.md"), "api memory\n").unwrap();

        // The two carriers scan the *same* `memory` locus, so an un-shadowed carrier would
        // preempt the project kind (symptom A) or clobber its lock rows (symptom C).
        let mut builtins = builtin_kind::definitions().unwrap();
        for provider in ["agents-md", "claude-code"] {
            let carrier = embedded_carrier("memory", provider, "memory", "**/MEMORY.md");
            builtins.insert(carrier.qualified_name(), carrier);
        }

        let into = tmpdir("shadow-into");
        run_with_builtins(&harness, &into, &builtins).unwrap();

        let doc = fs::read_to_string(into.join("lock.toml"))
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();
        // The project's `memory` kind imported its members — not preempted by the carriers.
        let names: Vec<&str> = doc["memory"]
            .as_array_of_tables()
            .unwrap()
            .iter()
            .map(|t| t["name"].as_str().unwrap())
            .collect();
        assert_eq!(names, vec!["MEMORY", "api-MEMORY"]);
        // The shadowed carriers wrote no section under either qualified identity.
        assert!(doc.get("agents-md.memory").is_none());
        assert!(doc.get("claude-code.memory").is_none());
        // The members are real on disk, folded to distinct placement ids.
        assert!(
            into.join("memory")
                .join("MEMORY")
                .join("MEMORY.md")
                .is_file()
        );
        assert!(
            into.join("memory")
                .join("api-MEMORY")
                .join("MEMORY.md")
                .is_file()
        );
    }

    #[test]
    fn a_memberless_embedded_kind_writes_no_lock_section() {
        // An embedded carrier whose locus holds no members discovers nothing, so it writes
        // no section — matching the toml round-trip reality (an empty `ArrayOfTables`
        // vanishes). The fixture has no `memory/` corpus, so the carrier is memberless.
        let harness = tmpdir("memberless-src");
        write_fixture_harness(&harness);

        let mut builtins = builtin_kind::definitions().unwrap();
        let carrier = embedded_carrier("memory", "claude-code", "memory", "**/MEMORY.md");
        builtins.insert(carrier.qualified_name(), carrier);

        let into = tmpdir("memberless-into");
        run_with_builtins(&harness, &into, &builtins).unwrap();

        let doc = fs::read_to_string(into.join("lock.toml"))
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();
        // No section under the bare name nor the qualified one — the carrier found nothing.
        assert!(doc.get("memory").is_none());
        assert!(doc.get("claude-code.memory").is_none());
        // The member-discovering kinds still write their bare-keyed sections.
        assert!(doc.get("skill").is_some());
        assert!(doc.get("rule").is_some());
    }

    #[test]
    fn two_co_discovering_carriers_key_their_lock_rows_by_qualified_identity() {
        // Two embedded carriers of the bare `memory` name BOTH discover members in one
        // harness, with no registration shadowing them. Each keys its roll-up rows by
        // qualified identity — `agents-md.memory` vs `claude-code.memory` — so neither
        // clobbers the other (symptom C): `resolve_bare`'s bare-while-unique,
        // qualified-where-two-meet policy, mirrored on the lock key.
        let harness = tmpdir("qualified-src");
        write_fixture_harness(&harness);
        fs::create_dir_all(harness.join("agents")).unwrap();
        fs::write(harness.join("agents").join("MEMORY.md"), "a\n").unwrap();
        fs::create_dir_all(harness.join("cc")).unwrap();
        fs::write(harness.join("cc").join("MEMORY.md"), "c\n").unwrap();

        // Distinct loci, so both carriers find exactly one member each and their surfaces
        // never overlap.
        let mut builtins = builtin_kind::definitions().unwrap();
        let a = embedded_carrier("memory", "agents-md", "agents", "*.md");
        let c = embedded_carrier("memory", "claude-code", "cc", "*.md");
        builtins.insert(a.qualified_name(), a);
        builtins.insert(c.qualified_name(), c);

        let into = tmpdir("qualified-into");
        run_with_builtins(&harness, &into, &builtins).unwrap();

        let doc = fs::read_to_string(into.join("lock.toml"))
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();
        // Bare `memory` is not a key — two carriers meet, so each qualifies.
        assert!(doc.get("memory").is_none());
        assert_eq!(
            doc["agents-md.memory"].as_array_of_tables().unwrap().len(),
            1
        );
        assert_eq!(
            doc["claude-code.memory"]
                .as_array_of_tables()
                .unwrap()
                .len(),
            1
        );
        // skill and rule stay bare — each unique among member-discovering kinds.
        assert!(doc.get("skill").is_some());
        assert!(doc.get("rule").is_some());

        // Deterministic and idempotent even with the qualified keys in play.
        let first = tree_bytes(&into);
        run_with_builtins(&harness, &into, &builtins).unwrap();
        assert_eq!(first, tree_bytes(&into));
    }

    /// A body whose trailing bytes (no final newline) must survive the frontmatter
    /// split intact.
    const PLAYBOOK_SOURCE: &str = "---\n\
title: Deploy\n\
owner: platform\n\
---\n\
# Deploy playbook\n\
\n\
Run the steps.   \n\
No final newline.";

    /// Register a custom kind under `temper.toml` and author its `KIND.md` definition
    /// (`+++`-fenced header) under `.temper/kinds/<name>/` — the discovery wiring both
    /// the frontmatter and whole-file custom paths key off.
    fn register_custom_kind(root: &Path, name: &str, kind_md: &str) {
        fs::write(
            root.join("temper.toml"),
            format!("[kind.{name}]\npackage = \"{name}\"\n"),
        )
        .unwrap();
        let kind_dir = root.join(".temper").join("kinds").join(name);
        fs::create_dir_all(&kind_dir).unwrap();
        fs::write(kind_dir.join("KIND.md"), kind_md).unwrap();
    }

    #[test]
    fn a_directory_shaped_frontmatter_custom_kind_ids_from_the_dir_and_lifts_fields() {
        // A custom kind whose declared `unit_shape`/`format` are load-bearing: a
        // directory-shaped `yaml-frontmatter` kind. The id is the directory name (not the
        // file stem), the declared `title` lifts into the header, and the companion rides
        // along — none of which the old whole-file path honored.
        let harness = tmpdir("dir-fm-src");
        write_fixture_harness(&harness);
        register_custom_kind(
            &harness,
            "playbook",
            "+++\n\
governs = { root = \"playbooks\", glob = \"*/PLAYBOOK.md\" }\n\
format = \"yaml-frontmatter\"\n\
unit_shape = \"directory\"\n\
\n\
[[extraction]]\n\
primitive = \"field\"\n\
key = \"title\"\n\
+++\n\
# The playbook kind\n",
        );
        let deploy = harness.join("playbooks").join("deploy");
        fs::create_dir_all(deploy.join("scripts")).unwrap();
        fs::write(deploy.join("PLAYBOOK.md"), PLAYBOOK_SOURCE).unwrap();
        fs::write(deploy.join("scripts").join("run.sh"), SCRIPT).unwrap();

        let into = tmpdir("dir-fm-into");
        run(&harness, &into).unwrap();

        // The surface dir is named for the *directory* (`directory` shape), not the
        // `PLAYBOOK.md` stem — the id derivation the declared `unit_shape` buys.
        let surface = into.join("playbooks").join("deploy");
        assert!(surface.join("PLAYBOOK.md").is_file());

        // The declared `title` lifts into a `[clause.*]` header table (the unknown
        // `owner` key follows it) — the frontmatter is split, not preserved whole.
        let document = fs::read_to_string(surface.join("PLAYBOOK.md")).unwrap();
        assert!(document.contains("[clause.title]\nvalue = \"Deploy\""));
        assert!(document.contains("[clause.owner]\nvalue = \"platform\""));

        // The member reloads through the generic adapter: id from the dir, declared
        // field readable, body byte-faithful below the header (trailing bytes intact).
        let member = Member::from_surface(&surface, "PLAYBOOK.md").unwrap();
        assert_eq!(member.id, "deploy");
        assert_eq!(
            member.field("title").and_then(|v| v.as_str()),
            Some("Deploy")
        );
        assert_eq!(
            member.body,
            "# Deploy playbook\n\nRun the steps.   \nNo final newline."
        );

        // The directory-shaped unit's companion rode along byte-for-byte.
        assert_eq!(
            fs::read(surface.join("scripts").join("run.sh")).unwrap(),
            SCRIPT
        );

        // The roll-up carries a `[[playbook]]` row keyed by the directory-name id.
        let doc = fs::read_to_string(into.join("lock.toml"))
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();
        let names: Vec<&str> = doc["playbook"]
            .as_array_of_tables()
            .unwrap()
            .iter()
            .map(|t| t["name"].as_str().unwrap())
            .collect();
        assert_eq!(names, vec!["deploy"]);

        // Idempotent on re-import.
        let first = tree_bytes(&into);
        run(&harness, &into).unwrap();
        assert_eq!(first, tree_bytes(&into));
    }

    #[test]
    fn an_any_depth_glob_discovers_a_nested_hierarchy_with_placement_folded_ids() {
        // A memory-format custom kind (no format ⇒ whole-file byte-faithful) whose
        // `governs.glob` opens with the any-depth `**` wildcard, modelling the nested
        // nearest-wins hierarchy of `AGENTS.md`/`CLAUDE.md`: a `MEMORY.md` at the root
        // and at two deeper levels, every one discovered — not just the fixed glob
        // depth — and each folded to a distinct surface id by its placement.
        let harness = tmpdir("recursive-src");
        write_fixture_harness(&harness);
        register_custom_kind(
            &harness,
            "memory",
            "+++\n\
governs = { root = \"memory\", glob = \"**/MEMORY.md\" }\n\
unit_shape = \"file\"\n\
+++\n\
# The memory kind\n",
        );
        let root = harness.join("memory");
        fs::create_dir_all(root.join("api").join("db")).unwrap();
        fs::write(root.join("MEMORY.md"), "root memory\n").unwrap();
        fs::write(root.join("api").join("MEMORY.md"), "api memory\n").unwrap();
        fs::write(root.join("api").join("db").join("MEMORY.md"), "db memory\n").unwrap();
        // Noise the glob must skip: a same-named file is the whole point, but a
        // differently-named sibling at depth is not a member.
        fs::write(root.join("api").join("NOTES.md"), "not a member\n").unwrap();

        let into = tmpdir("recursive-into");
        run(&harness, &into).unwrap();

        // Every level is discovered, and the root vs nested files carry distinct,
        // placement-folded ids rather than collapsing onto one clobbered `MEMORY` dir.
        let doc = fs::read_to_string(into.join("lock.toml"))
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();
        let names: Vec<&str> = doc["memory"]
            .as_array_of_tables()
            .unwrap()
            .iter()
            .map(|t| t["name"].as_str().unwrap())
            .collect();
        assert_eq!(names, vec!["MEMORY", "api-MEMORY", "api-db-MEMORY"]);

        // Each id is a real, separate surface directory carrying its own body — no
        // clobber (three same-named sources, three distinct surfaces).
        for (name, body) in [
            ("MEMORY", "root memory\n"),
            ("api-MEMORY", "api memory\n"),
            ("api-db-MEMORY", "db memory\n"),
        ] {
            let surface = into.join("memory").join(name);
            let document = fs::read_to_string(surface.join("MEMORY.md")).unwrap();
            assert!(document.starts_with("+++\n[provenance]\n"));
            assert!(document.ends_with(body));
        }

        // Re-import into the same workspace changes not a single byte.
        let first = tree_bytes(&into);
        run(&harness, &into).unwrap();
        assert_eq!(first, tree_bytes(&into));
    }

    #[test]
    fn an_any_depth_glob_folds_placement_on_the_frontmatter_face_too() {
        // The frontmatter face folds placement identically: a `yaml-frontmatter`
        // file-shaped kind with an any-depth glob imports two nested same-named files
        // to distinct surface ids, symmetric with the whole-file face above — both
        // faces name a nested unit the same way.
        let harness = tmpdir("recursive-fm-src");
        write_fixture_harness(&harness);
        register_custom_kind(
            &harness,
            "log",
            "+++\n\
governs = { root = \"logs\", glob = \"**/LOG.md\" }\n\
format = \"yaml-frontmatter\"\n\
unit_shape = \"file\"\n\
\n\
[[extraction]]\n\
primitive = \"field\"\n\
key = \"title\"\n\
+++\n\
# The log kind\n",
        );
        let logs = harness.join("logs");
        fs::create_dir_all(logs.join("2026")).unwrap();
        fs::write(logs.join("LOG.md"), "---\ntitle: Root\n---\n# Root log\n").unwrap();
        fs::write(
            logs.join("2026").join("LOG.md"),
            "---\ntitle: Yearly\n---\n# Yearly log\n",
        )
        .unwrap();

        let into = tmpdir("recursive-fm-into");
        run(&harness, &into).unwrap();

        // Distinct placement-folded surface ids, each reloading through the generic
        // adapter with its own declared field and body.
        let doc = fs::read_to_string(into.join("lock.toml"))
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();
        let names: Vec<&str> = doc["log"]
            .as_array_of_tables()
            .unwrap()
            .iter()
            .map(|t| t["name"].as_str().unwrap())
            .collect();
        assert_eq!(names, vec!["2026-LOG", "LOG"]);

        let root = Member::from_surface(&into.join("logs").join("LOG"), "LOG.md").unwrap();
        assert_eq!(root.id, "LOG");
        assert_eq!(root.field("title").and_then(|v| v.as_str()), Some("Root"));
        let nested = Member::from_surface(&into.join("logs").join("2026-LOG"), "LOG.md").unwrap();
        assert_eq!(nested.id, "2026-LOG");
        assert_eq!(
            nested.field("title").and_then(|v| v.as_str()),
            Some("Yearly")
        );

        // Idempotent on re-import.
        let first = tree_bytes(&into);
        run(&harness, &into).unwrap();
        assert_eq!(first, tree_bytes(&into));
    }

    #[test]
    fn a_file_shaped_frontmatter_custom_format_splits_declared_fields() {
        // A file-shaped `yaml-frontmatter` custom kind: the id is the file stem, and the
        // declared field splits into the header with the body byte-faithful below it —
        // the frontmatter is lifted, not carried whole as the frontmatterless path does.
        let harness = tmpdir("file-fm-src");
        write_fixture_harness(&harness);
        register_custom_kind(
            &harness,
            "note",
            "+++\n\
governs = { root = \"notes\", glob = \"*.md\" }\n\
format = \"yaml-frontmatter\"\n\
unit_shape = \"file\"\n\
\n\
[[extraction]]\n\
primitive = \"field\"\n\
key = \"title\"\n\
+++\n\
# The note kind\n",
        );
        let notes = harness.join("notes");
        fs::create_dir_all(&notes).unwrap();
        fs::write(
            notes.join("idea.md"),
            "---\ntitle: An idea\n---\n# Idea\n\nBody, no final newline.",
        )
        .unwrap();

        let into = tmpdir("file-fm-into");
        run(&harness, &into).unwrap();

        // The surface dir is named for the file stem (`file` shape).
        let surface = into.join("notes").join("idea");
        let document = fs::read_to_string(surface.join("NOTE.md")).unwrap();
        assert!(document.contains("[clause.title]\nvalue = \"An idea\""));

        let member = Member::from_surface(&surface, "NOTE.md").unwrap();
        assert_eq!(member.id, "idea");
        assert_eq!(
            member.field("title").and_then(|v| v.as_str()),
            Some("An idea")
        );
        // The body is everything below the frontmatter, byte-faithful.
        assert_eq!(member.body, "# Idea\n\nBody, no final newline.");

        // Idempotent on re-import.
        let first = tree_bytes(&into);
        run(&harness, &into).unwrap();
        assert_eq!(first, tree_bytes(&into));
    }
}
