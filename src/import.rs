//! `temper init` — scan a Claude Code harness into the typed config surface.
//!
//! specs/architecture/20-surface.md, "Decision: `init` is the on-ramp, and adoption is a
//! gradient"; custom kinds specs/architecture/40-composition.md.
//!
//! [`init`] is the on-ramp: it scans the built-in-kind harness members at their real
//! Claude Code locus under `<harness>/.claude/` and writes a manifest over them **in
//! place** — a `[[member]]` table per member naming its landscape file, zero file moves,
//! zero copy tree (`specs/architecture/20-surface.md`, the gradient's `init` on-ramp).
//! [`lift`] migrates one in-place member into document carriage, a one-time per-member
//! projection into `<harness>/.temper/` (`specs/architecture/15-kinds.md`, the generic
//! frontmatter adapter). The discovery walk (`discover_kind_units`/`discover_builtin`) is
//! the sole member extractor the gate, `lift`, and `emit`'s lock-writer ([`write_rollup`])
//! all ride.
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
use crate::compose::{self, AuthorLayer, InPlaceMember, ManifestMember};
use crate::drift::Declarations;
use crate::frontmatter::{FrontmatterError, Member};
use crate::kind::{CustomKind, Governs, KindError, Unit};

/// Filename of the generated roll-up index — the contents' state-of-record —
/// written at the workspace root (`specs/architecture/20-surface.md`, "Topology").
const LOCK_FILENAME: &str = "lock.toml";

/// Filename of the generated-canonical **manifest** — the assembly + its emitted member
/// features — written beside the `.temper/` workspace at the project root
/// (`specs/architecture/20-surface.md`, "Topology": the only thing the gate reads).
const MANIFEST_FILENAME: &str = "temper.toml";

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

    /// A written surface member document failed to reload
    /// ([`Unit::from_member_document`]) — a malformed `+++`-fenced document or a
    /// missing required part, surfaced rather than panicked so re-reading a prior
    /// surface stays panic-free (`.claude/rules/rust.md`).
    #[error(transparent)]
    #[diagnostic(transparent)]
    Kind(#[from] KindError),

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

    /// The existing manifest could not be read to patch it format-preserving
    /// (`specs/architecture/20-surface.md`, "a hand-written manifest is patched format-preserving").
    #[error("failed to read manifest {path}")]
    #[diagnostic(code(temper::import::manifest_read))]
    ManifestRead {
        /// The manifest path that failed to read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// The existing manifest is not valid TOML, so it cannot be patched
    /// format-preserving — a hand-authored manifest must round-trip through `toml_edit`.
    #[error("failed to parse manifest {path} as TOML")]
    #[diagnostic(code(temper::import::manifest_toml))]
    ManifestToml {
        /// The manifest that failed to parse.
        path: PathBuf,
        /// The TOML parse error.
        #[source]
        source: toml_edit::TomlError,
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

/// The on-ramp (`specs/architecture/20-surface.md`, "Decision: `init` is the on-ramp"): scan
/// `harness_path` for its built-in-kind members and write a manifest over them **in
/// place** — zero file moves, zero copy tree, zero reformatting. Each member lands as a
/// `source`-bearing `[[member]]` table naming its landscape file; a 40-artifact harness
/// is governed by the floor day one, byte-identical to the harness it was the day
/// before. Members arrive **unrecognized** (no `satisfies`, no published requirements);
/// recognition accrues member-by-member from the author's own declared requirements
/// failing coverage, never from on-ramp ceremony.
///
/// The `<harness>/temper.toml` is patched **format-preserving** — the hand-authored
/// bindings, requirements, and comments survive; only the generated-canonical `member`
/// root is re-emitted whole. Any **already-lifted** member (a document/module-carried
/// `[[member]]` a prior [`lift`] wrote) is preserved and **not** re-scanned as in-place,
/// so `init` composes with the gradient rather than clobbering a climbed member. In-place
/// carriage is built-in-kind only — a custom project kind is an authored `.temper/`
/// artifact (document/module carriage), not one of the floor's harness members
/// (`specs/architecture/20-surface.md`, "In-place — the floor's harness members").
///
/// # Errors
///
/// Returns an error if the harness cannot be scanned, a member source cannot be read, or
/// the existing manifest cannot be read/parsed for patching.
pub fn init(harness_path: &Path) -> miette::Result<()> {
    let manifest_path = harness_path.join(MANIFEST_FILENAME);
    let mut doc = load_manifest(&manifest_path)?;

    // Preserve any already-lifted (document/module-carried) members, and skip re-scanning
    // them as in-place — the gradient climbs per member, so init must not knock a lifted
    // member back down to in-place carriage.
    let extracted: Vec<ManifestMember> = AuthorLayer::load(&manifest_path)?
        .map(|layer| layer.members().to_vec())
        .unwrap_or_default();
    let lifted: BTreeSet<(String, String)> = extracted
        .iter()
        .map(|member| (member.kind.clone(), member.features.id.clone()))
        .collect();

    let inplace = scan_inplace_members(harness_path, &lifted)?;
    compose::write_members(&mut doc, &extracted, &inplace);
    write_bytes(&manifest_path, doc.to_string().as_bytes())?;
    Ok(())
}

/// Scan every built-in kind's members under `harness` and model each as an
/// [`InPlaceMember`] naming its landscape file (a slash path relative to `harness`),
/// name-sorted per kind for a byte-stable manifest. `lifted` names the `(kind, id)`
/// members a prior [`lift`] already climbed to document/module carriage; those are
/// skipped so a member is never carried twice. Built-in-kind only — a custom kind's units
/// are authored `.temper/` artifacts, not in-place harness members.
fn scan_inplace_members(
    harness: &Path,
    lifted: &BTreeSet<(String, String)>,
) -> miette::Result<Vec<InPlaceMember>> {
    let builtins = builtin_kind::definitions()?;
    let mut members = Vec::new();
    for kind in builtins.values() {
        for file in discover_builtin(harness, kind)? {
            let member =
                Member::from_source_rooted(kind, &file, &harness.join(&kind.governs.root))?;
            if lifted.contains(&(kind.name.clone(), member.id.clone())) {
                continue;
            }
            members.push(InPlaceMember {
                kind: kind.name.clone(),
                name: member.id,
                source: rel_slash(harness, &file),
                satisfies: Vec::new(),
                published: Vec::new(),
            });
        }
    }
    members.sort_by(|a, b| a.kind.cmp(&b.kind).then_with(|| a.name.cmp(&b.name)));
    Ok(members)
}

/// `file` as a slash-separated path relative to the harness root — the form an in-place
/// `[[member]]` records its `source` as, so the gate resolves `harness_root.join(source)`
/// on every platform. Falls back to the whole path when `file` is not under `harness`.
fn rel_slash(harness: &Path, file: &Path) -> String {
    file.strip_prefix(harness)
        .unwrap_or(file)
        .to_string_lossy()
        .replace('\\', "/")
}

/// The per-member migration into a richer carriage (in-place → document → module)
/// (`specs/architecture/20-surface.md`, "adoption is a gradient"; "`--lift` … normalizes framing,
/// never content"): lift the in-place member
/// `member_name` into **document carriage** — project it into `<harness>/.temper/` as a
/// `+++`-headed member document (via [`import_frontmatter_member`], body byte-identical),
/// then rewrite its `[[member]]` from the `source`-bearing in-place form to the
/// pre-extracted document form. The rest of the manifest — every other in-place member,
/// the hand-authored bindings and requirements — is preserved. The member's declared
/// joins carry across the lift, since framing normalizes but the recognition it earned
/// must not be dropped.
///
/// Lift to **module carriage** (the altitude) needs the parked TypeScript SDK, so this is
/// the reachable carriage today: the document form is the same data hand-spellable
/// (`specs/architecture/20-surface.md`, "the document form is the same data hand-spelled").
/// Built-in-kind only, matching [`init`]'s in-place scan.
///
/// # Errors
///
/// Returns an error if the harness carries no manifest, names no in-place member
/// `member_name`, names an unknown built-in kind, or the projection/manifest write fails.
pub fn lift(harness_path: &Path, member_name: &str) -> miette::Result<()> {
    let manifest_path = harness_path.join(MANIFEST_FILENAME);
    let layer = AuthorLayer::load(&manifest_path)?.ok_or_else(|| {
        miette::miette!(
            "no {MANIFEST_FILENAME} at {} — run `temper init` before lifting a member",
            harness_path.display()
        )
    })?;

    let target = layer
        .inplace_members()
        .iter()
        .find(|member| member.name == member_name)
        .ok_or_else(|| miette::miette!("no in-place member `{member_name}` in the manifest"))?
        .clone();

    let builtins = builtin_kind::definitions()?;
    let kind = builtins
        .values()
        .find(|kind| kind.name == target.kind)
        .ok_or_else(|| {
            miette::miette!(
                "in-place member `{member_name}` names unknown built-in kind `{}`",
                target.kind
            )
        })?;

    // Project the member into document carriage under `<harness>/.temper/` — the body
    // rides byte-identical, only the `+++` framing is added.
    let into = harness_path.join(".temper");
    let source_file = harness_path.join(&target.source);
    let row = import_frontmatter_member(kind, harness_path, &source_file, &into)?;

    // Re-extract the now-document-carried member's features for the pre-extracted manifest
    // form, reading the surface through the same loader the gate uses; the recognition the
    // in-place member earned rides across the lift, not the empty joins a fresh projection
    // would carry.
    let member_doc = kind.member_document();
    let out_dir = into.join(kind.surface_subdir()).join(&row.name);
    let unit = Unit::from_member_document(&out_dir, &out_dir.join(&member_doc))?;
    let mut features = builtin_kind::features(kind, &unit);
    features.satisfies = target.satisfies.clone();
    features.published_requirements = target.published.clone();

    let mut extracted: Vec<ManifestMember> = layer.members().to_vec();
    extracted.push(ManifestMember {
        kind: target.kind.clone(),
        features,
    });
    extracted.sort_by(|a, b| {
        a.kind
            .cmp(&b.kind)
            .then_with(|| a.features.id.cmp(&b.features.id))
    });
    let mut inplace: Vec<InPlaceMember> = layer
        .inplace_members()
        .iter()
        .filter(|member| member.name != member_name)
        .cloned()
        .collect();
    inplace.sort_by(|a, b| a.kind.cmp(&b.kind).then_with(|| a.name.cmp(&b.name)));

    let mut doc = load_manifest(&manifest_path)?;
    compose::write_members(&mut doc, &extracted, &inplace);
    write_bytes(&manifest_path, doc.to_string().as_bytes())?;
    Ok(())
}

/// Load an existing manifest at `path` as a format-preserving [`DocumentMut`], or a fresh
/// empty document when none is there (a first import creates the manifest). A malformed
/// existing manifest is a hard error, never silently overwritten — patch-preserving
/// requires a parseable base.
fn load_manifest(path: &Path) -> Result<DocumentMut, ImportError> {
    match fs::read_to_string(path) {
        Ok(src) => src
            .parse::<DocumentMut>()
            .map_err(|source| ImportError::ManifestToml {
                path: path.to_path_buf(),
                source,
            }),
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(DocumentMut::new()),
        Err(source) => Err(ImportError::ManifestRead {
            path: path.to_path_buf(),
            source,
        }),
    }
}

/// Discover a built-in `kind`'s source files, keying off its declared `governs`
/// locus — the same data-driven scan a custom kind would get, so `skill`/`rule`
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
    discover_kind_files(harness, kind, &kind.governs)
}

/// Discover a `kind`'s member source files under `harness`, matching an explicit
/// `governs` locus — the generalized scan [`discover_kind_units`] runs, plus `skill`'s
/// bare-root special case (a `<harness>/SKILL.md`, a harness that is itself a skill).
/// Decoupled from the kind's own [`CustomKind::governs`] so a caller can walk a
/// *different* declared locus for the same kind — the committed lock's own kind-fact
/// row (`specs/architecture/20-surface.md`, "The lock and drift") on an adopted
/// harness, the kind's embedded default otherwise (the built-in lock) — while the
/// bare-root-skill convention still applies wherever `skill`'s locus is walked from.
/// [`discover_builtin`] is the thin caller that always walks the kind's own governs.
///
/// # Errors
///
/// Returns an [`ImportError`] if a directory under `governs.root` cannot be
/// enumerated.
pub fn discover_kind_files(
    harness: &Path,
    kind: &CustomKind,
    governs: &Governs,
) -> Result<Vec<PathBuf>, ImportError> {
    let mut files = discover_kind_units(harness, governs)?;
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

/// Read one source member of a frontmatter `kind` and project it into document carriage
/// under `<into>/<subdir>/<id>/`, returning the roll-up row for the index — [`lift`]'s
/// one-time, per-member write (`specs/architecture/20-surface.md`, "the lift is one-time,
/// per-member, byte-stable on content"): the member document via [`Member::to_document`],
/// plus the copied companions of a directory-shaped unit. The surface subdir is the
/// kind's declared `governs` leaf, the member document its declared name (`SKILL.md`,
/// `RULE.md`).
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
    // (`docs/adr`).
    let member_doc = kind.member_document();
    let out_dir = into.join(kind.surface_subdir()).join(&member.id);
    create_dir_all(&out_dir)?;

    // The member is ONE document: the `+++`-fenced clause-module header over the
    // byte-faithful body, written format-preserving — never a lossy re-serialize.
    write_bytes(
        &out_dir.join(&member_doc),
        member.to_document().emit().as_bytes(),
    )?;

    // A directory-shaped unit's companions ride beside the member document, copied
    // byte-for-byte from the source directory (the member file's parent).
    if let Some(source_dir) = source_file.parent() {
        for companion in &member.companions {
            let from = source_dir.join(companion);
            let to = out_dir.join(companion);
            if let Some(parent) = to.parent() {
                create_dir_all(parent)?;
            }
            fs::copy(&from, &to).map_err(|source| ImportError::Write { path: to, source })?;
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

/// Write the `<into>/lock.toml` roll-up: one `[[<kind>]]` table per emitted member —
/// the built-in kinds first (key-sorted) then the custom kinds (name-sorted) — each with
/// `name`, `source_path`, `source_hash`, and the `emit_hash` fingerprint. Both maps are
/// key-sorted, so the emitted order is deterministic. `drift::emit` is the sole caller
/// (`specs/architecture/20-surface.md`, "The lock and drift"): a kind with no emitted
/// member simply has no entry, matching the toml round-trip reality — an empty
/// `ArrayOfTables` emits nothing, so a written-then-vanished section would break
/// idempotence against a re-parse that never sees it.
///
/// After the per-member sections come the program's **declaration rows** — kind facts,
/// clauses, requirements, assembly facts under an implicit `[declaration]` table
/// (`specs/architecture/20-surface.md`, "The lock and drift"); the drift/gate side reads them
/// through [`crate::drift::read_declarations`].
pub(crate) fn write_rollup(
    into: &Path,
    builtins: &BTreeMap<String, Vec<RollupEntry>>,
    custom: &BTreeMap<String, Vec<RollupEntry>>,
    declarations: &Declarations,
) -> Result<(), ImportError> {
    let mut doc = DocumentMut::new();
    for (kind, rows) in builtins {
        doc[kind.as_str()] = Item::ArrayOfTables(rollup_tables(rows));
    }
    for (kind, units) in custom {
        doc[kind.as_str()] = Item::ArrayOfTables(rollup_tables(units));
    }
    declarations.write_into(&mut doc);

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
    use std::sync::atomic::{AtomicU32, Ordering};

    use crate::kind::Extraction;

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
    fn import_frontmatter_member_writes_the_document_and_its_companions() {
        // The per-member write [`lift`] drives: one `+++`-fenced member document over the
        // byte-faithful body, plus a directory-shaped unit's companions copied
        // byte-for-byte — no meta.toml, and the document reloads through the generic
        // adapter.
        let harness = tmpdir("member-src");
        write_fixture_harness(&harness);
        let into = tmpdir("member-into");

        let skill_kind = builtin_kind::definition("skill").unwrap().unwrap();
        let source = harness
            .join(".claude")
            .join("skills")
            .join("coordinate")
            .join("SKILL.md");
        let row = import_frontmatter_member(&skill_kind, &harness, &source, &into).unwrap();
        assert_eq!(row.name, "coordinate");
        assert_eq!(row.emit_hash, row.source_hash);

        let coord = into.join("skills").join("coordinate");
        assert!(coord.join("SKILL.md").is_file());
        assert!(!coord.join("meta.toml").exists());
        assert_eq!(fs::read(coord.join("PLAYBOOK.md")).unwrap(), PLAYBOOK);
        assert_eq!(
            fs::read(coord.join("scripts").join("run.sh")).unwrap(),
            SCRIPT
        );

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
    fn builtin_discovery_keys_off_the_embedded_kind_governs() {
        // Discovery is driven by the embedded `skill`/`rule` kinds' declared `governs`,
        // not a hardwired path: the skill `*/SKILL.md` subdir glob and the rule `*.md`
        // flat glob both resolve through the one generalized scanner.
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

        let memory = CustomKind {
            qualified: Some("claude-code.memory".to_string()),
            ..CustomKind::new(
                "memory",
                Governs {
                    root: "mem".to_string(),
                    glob: "*.md".to_string(),
                },
                Extraction::new(Vec::new()),
            )
        };

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
    fn discover_builtin_finds_a_bare_harness_that_is_itself_a_skill() {
        // A `<harness>` whose own SKILL.md makes it a skill dir, with no skills/ — the
        // real bare-skill-repo shape, not a tmpdir artifact.
        let harness = tmpdir("bare-src").join("demo");
        fs::create_dir_all(&harness).unwrap();
        fs::write(harness.join("SKILL.md"), DEMO).unwrap();

        let skill_kind = builtin_kind::definition("skill").unwrap().unwrap();
        let found = discover_builtin(&harness, &skill_kind).unwrap();
        assert_eq!(found, vec![harness.join("SKILL.md")]);
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

        let skill_kind = builtin_kind::definition("skill").unwrap().unwrap();
        let found = discover_builtin(&harness, &skill_kind).unwrap();
        assert_eq!(
            found,
            vec![
                harness.join(".claude/skills/coordinate").join("SKILL.md"),
                harness.join(".claude/skills/demo").join("SKILL.md"),
            ]
        );
    }
}
