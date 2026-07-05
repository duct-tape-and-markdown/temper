//! `temper init` — scan a Claude Code harness into the typed config surface.
//!
//! specs/architecture/20-surface.md, "Decision: `init` is the on-ramp, and adoption is a
//! gradient"; custom kinds specs/architecture/40-composition.md.
//!
//! [`init`] is the on-ramp: it scans the built-in-kind harness members at their real
//! Claude Code locus under `<harness>/.claude/` and writes a manifest over them **in
//! place** — a `[[member]]` table per member naming its landscape file, zero file moves,
//! zero copy tree (`specs/architecture/20-surface.md`, the gradient's `init` on-ramp). [`lift`]
//! migrates one member into a richer carriage (in-place → document → module). [`run`] is the
//! retained document-carriage projection (`specs/architecture/15-kinds.md`, the generic frontmatter
//! adapter) the one-shot gate paths, `emit`, and `diff` still ride: it copies each member into
//! `<into>/` as a `+++`-headed document and records one `<into>/lock.toml` roll-up row per artifact.
//!
//! Keystone invariant (`.claude/rules/rust.md`): idempotence. It holds because
//! every write is content-derived, name-sorted, and overwrites in place.

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::fs;
use std::path::{Component, Path, PathBuf};

use ignore::WalkBuilder;
use toml_edit::{ArrayOfTables, DocumentMut, Item, Table, value};

use crate::builtin;
use crate::builtin_kind;
use crate::compose::{self, AuthorLayer, Authority, InPlaceMember, ManifestMember, Requirement};
use crate::contract::{Clause, Severity};
use crate::drift::{
    AssemblyFactRow, ClauseRow, CountBoundRow, Declarations, DegreeBoundRow, EdgeBoundRow,
    KindFactRow, MembershipRow, RequirementRow, SatisfiesRow,
};
use crate::frontmatter::{FrontmatterError, Member};
use crate::kind::{Activation, CustomKind, Format, Governs, KindError, Unit, UnitShape};

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
    // The member-discovering built-in kinds, retained for the lock's declaration rows: a
    // kind fact and floor clauses are recorded for exactly the kinds that carry a member
    // section (`specs/architecture/20-surface.md`, "The lock and drift").
    let mut program_builtin_kinds: Vec<CustomKind> = Vec::new();
    for kind in builtins.values() {
        if shadowed.contains(&kind.name) {
            continue;
        }
        let rows = import_frontmatter_kind(harness_path, into, kind)?;
        if rows.is_empty() {
            continue;
        }
        program_builtin_kinds.push(kind.clone());
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

    // Custom kinds retire with the KIND.md file format (`specs/architecture/15-kinds.md`,
    // "Decision: field typing lives in the SDK — there is no kind file format"): a
    // project's own kind is SDK-authored, and no SDK path exists in the engine yet, so
    // a `[kind.<name>]` registration with no unique embedded carrier imports nothing.
    let custom: BTreeMap<String, Vec<RollupEntry>> = BTreeMap::new();
    let program_custom_kinds: Vec<CustomKind> = Vec::new();

    let declarations = collect_declarations(
        &program_builtin_kinds,
        &program_custom_kinds,
        layer.as_ref(),
    )?;
    write_rollup(into, &builtin_rollups, &custom, &declarations)?;

    Ok(())
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

/// Serialize the imported harness's **manifest** to `temper.toml` beside the `.temper/`
/// workspace — the generated-canonical artifact the gate reads
/// (`specs/architecture/20-surface.md`, "Topology"). Every imported member's extracted
/// [`Features`](crate::extract::Features) lands as a `[[member]]` table; a hand-authored
/// floor manifest (its bindings, requirements, relationships, and comments) is patched
/// **format-preserving** via `toml_edit` and **never clobbered** — only the
/// generated-canonical `member` root is re-emitted whole.
///
/// The manifest lives at the workspace's parent (`into.parent()`), the project root a
/// real `temper import .` targets. A throwaway one-shot import (`check --harness`,
/// session-start's surfaceless fallback) imports into a scratch surface and does **not**
/// call this, so the harness it lints is never mutated.
///
/// Write-side only: the gate reads these members at MANIFEST-GATE-READ. `check` still
/// extracts the `.temper/` copy tree until then, so the copy tree stays authoritative and
/// the manifest members ride along inert.
///
/// # Errors
///
/// Returns an error if a member surface cannot be read, an embedded kind definition is
/// malformed, or the existing manifest cannot be read/parsed for patching.
pub fn emit_manifest(into: &Path) -> miette::Result<()> {
    let members = collect_manifest_members(into)?;
    // The manifest is a *sibling* of `.temper/` at the project root, not inside it; a
    // parentless `into` (a bare relative workspace) falls back to the workspace itself.
    let manifest_path = into.parent().unwrap_or(into).join(MANIFEST_FILENAME);
    let mut doc = load_manifest(&manifest_path)?;
    compose::write_manifest_members(&mut doc, &members);
    write_bytes(&manifest_path, doc.to_string().as_bytes())?;
    Ok(())
}

/// Gather every imported member's [`ManifestMember`] — the bare kind name paired with the
/// exact [`Features`](crate::extract::Features) `check` extracts — by re-reading the
/// surface `into` through the same loader the gate uses, so a serialized member equals a
/// live extraction. Built-in kinds take the permissive extraction (`builtin_kind::features`
/// folds unknown frontmatter keys a `forbidden_keys` clause ranges over); a registered
/// custom kind takes its declared-field extraction. Sorted by kind then id, so the emitted
/// manifest is byte-stable across imports (the lock roll-up's discipline).
fn collect_manifest_members(into: &Path) -> miette::Result<Vec<ManifestMember>> {
    let builtins = builtin_kind::definitions()?;
    let mut members = Vec::new();
    for kind in builtins.values() {
        for unit in surface_units_for(into, kind.surface_subdir(), &kind.member_document())? {
            members.push(ManifestMember {
                kind: kind.name.clone(),
                features: builtin_kind::features(kind, &unit),
            });
        }
    }
    // Custom kinds retire with the KIND.md file format (`specs/architecture/15-kinds.md`,
    // "Decision: field typing lives in the SDK — there is no kind file format"): a
    // `[kind.<name>]` registration with no unique embedded carrier contributes no
    // members until a future SDK path supplies its definition.
    members.sort_by(|a, b| {
        a.kind
            .cmp(&b.kind)
            .then_with(|| a.features.id.cmp(&b.features.id))
    });
    Ok(members)
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

/// Load every surface member of one kind under `<into>/<subdir>/*/<member_doc>` as a
/// generic [`Unit`], name-sorted — the identical read `check`'s `surface_units` performs,
/// so the features this yields match the gate's exactly. A missing subdir (a kind with no
/// imported members) yields an empty list.
fn surface_units_for(
    into: &Path,
    subdir: &str,
    member_doc: &str,
) -> Result<Vec<Unit>, ImportError> {
    let dir = into.join(subdir);
    if !dir.is_dir() {
        return Ok(Vec::new());
    }
    let mut member_dirs: Vec<PathBuf> = read_entries(&dir)?
        .into_iter()
        .map(|entry| entry.path())
        .filter(|path| path.is_dir() && path.join(member_doc).is_file())
        .collect();
    member_dirs.sort();
    let mut units = Vec::with_capacity(member_dirs.len());
    for member_dir in &member_dirs {
        units.push(Unit::from_member_document(
            member_dir,
            &member_dir.join(member_doc),
        )?);
    }
    Ok(units)
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
///
/// After the per-member sections come the program's **declaration rows** — kind facts,
/// clauses, requirements, assembly facts under an implicit `[declaration]` table
/// (`specs/architecture/20-surface.md`, "The lock and drift"); the drift/gate side reads them
/// through [`crate::drift::read_declarations`].
fn write_rollup(
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

/// Gather the composed program's **declaration rows** for the lock
/// (`specs/architecture/20-surface.md`, "The lock and drift"): a kind fact per kind in play, the
/// clauses of each built-in kind's floor contract, the assembly's requirements, and its
/// assembly-scope facts. Sourced from the current extraction — the SDK becomes the producer
/// at `SDK-RECUT-CORPUS-FACE`.
///
/// `builtin_kinds` are the member-discovering built-in kinds (the ones the lock carries a
/// `[[<kind>]]` section for); `custom_kinds` the registered custom kinds. Clauses are the
/// **floor** contract's only: a custom kind's contract is a bound package, and author-layer
/// overrides fold over the floor — both the SDK producer's to resolve, not this bootstrap's.
fn collect_declarations(
    builtin_kinds: &[CustomKind],
    custom_kinds: &[CustomKind],
    layer: Option<&AuthorLayer>,
) -> miette::Result<Declarations> {
    // Kind facts cover every kind in play, name-sorted for a stable lock.
    let mut kinds: Vec<KindFactRow> = builtin_kinds
        .iter()
        .chain(custom_kinds)
        .map(kind_fact)
        .collect();
    kinds.sort_by(|a, b| a.name.cmp(&b.name));

    // Clauses: each built-in kind's floor contract, in the caller's (name-sorted) order.
    let mut clauses = Vec::new();
    for kind in builtin_kinds {
        if let Some(package) = builtin::floor_package(&kind.qualified_name())
            && let Some(contract) = builtin::contract(package)?
        {
            clauses.extend(
                contract
                    .clauses
                    .iter()
                    .map(|clause| clause_row(&kind.name, clause)),
            );
        }
    }

    let requirements = layer
        .map(|layer| layer.requirements().values().map(requirement_row).collect())
        .unwrap_or_default();

    Ok(Declarations {
        kinds,
        clauses,
        requirements,
        assembly: assembly_facts(layer),
        satisfies: layer.map(satisfies_rows).unwrap_or_default(),
    })
}

/// One kind's declaration row — identity plus its declared runtime facts, each optional
/// fact folded to its label (`specs/architecture/15-kinds.md`).
fn kind_fact(kind: &CustomKind) -> KindFactRow {
    KindFactRow {
        name: kind.name.clone(),
        // The provider authority alone (`claude-code`), the qualified label's prefix —
        // not the whole `qualified_name()` (`claude-code.skill`).
        provider: kind
            .qualified
            .as_deref()
            .and_then(|qualified| qualified.rsplit_once('.'))
            .map(|(provider, _)| provider.to_string()),
        governs_root: kind.governs.root.clone(),
        governs_glob: kind.governs.glob.clone(),
        format: kind.format.map(format_label),
        unit_shape: kind.unit_shape.map(unit_shape_label),
        activation: kind.activation.as_ref().map(activation_label),
    }
}

/// One clause's declaration row — the kind it governs, the predicate key, the field it
/// targets (when it names one), and its declared severity (`specs/architecture/10-contracts.md`).
fn clause_row(kind: &str, clause: &Clause) -> ClauseRow {
    ClauseRow {
        kind: kind.to_string(),
        predicate: clause.predicate.key().to_string(),
        field: clause.predicate.target().map(str::to_string),
        severity: severity_label(clause.severity).to_string(),
    }
}

/// One requirement's declaration row — its scalar facets plus the set-scope bounds
/// already parsed onto `compose::Requirement` (`specs/architecture/10-contracts.md`;
/// `specs/architecture/45-governance.md`).
fn requirement_row(requirement: &Requirement) -> RequirementRow {
    RequirementRow {
        name: requirement.name.clone(),
        kind: requirement.kind.clone(),
        package: requirement.package.clone(),
        required: requirement.required,
        count: requirement.count.map(|count| CountBoundRow {
            min: count.min,
            max: count.max,
        }),
        unique: requirement.unique.clone(),
        membership: requirement
            .membership
            .as_ref()
            .map(|membership| MembershipRow {
                field: membership.field.clone(),
                source: membership.source.clone(),
                source_kind: membership.source_kind.clone(),
                source_feature: membership.source_feature.clone(),
                source_package: membership.source_package.clone(),
            }),
        degree: requirement.degree.map(|degree| DegreeBoundRow {
            incoming: degree.incoming.map(|bound| EdgeBoundRow {
                min: bound.min,
                max: bound.max,
            }),
            outgoing: degree.outgoing.map(|bound| EdgeBoundRow {
                min: bound.min,
                max: bound.max,
            }),
        }),
        verified_by: requirement.verified_by.clone(),
    }
}

/// The member→requirement fill edges — every imported member's `satisfies` keys, folded
/// from the `AuthorLayer`'s pre-extracted [`ManifestMember`]s and its **in-place**
/// members alike (`specs/architecture/20-surface.md`, "The lock and drift"), so the
/// roster/coverage tiers' requirement↔satisfies join rides the lock instead of
/// re-importing the harness. Members arrive kind-then-id sorted
/// (`AuthorLayer::members`), so the row order is stable across a re-import.
fn satisfies_rows(layer: &AuthorLayer) -> Vec<SatisfiesRow> {
    let mut rows = Vec::new();
    for member in layer.members() {
        for requirement in &member.features.satisfies {
            rows.push(SatisfiesRow {
                member: member.features.id.clone(),
                requirement: requirement.clone(),
            });
        }
    }
    for member in layer.inplace_members() {
        for requirement in &member.satisfies {
            rows.push(SatisfiesRow {
                member: member.name.clone(),
                requirement: requirement.clone(),
            });
        }
    }
    rows
}

/// The assembly-scope facts, in a stable order: authority (always declared — absent ⇒ the
/// `shared` default, so it anchors the family for every harness), then reachability when the
/// assembly opts in, then one row per declared edge in declaration order
/// (`specs/architecture/40-composition.md`; `specs/architecture/45-governance.md`).
fn assembly_facts(layer: Option<&AuthorLayer>) -> Vec<AssemblyFactRow> {
    let mut facts = Vec::new();
    let authority = layer.map(AuthorLayer::authority).unwrap_or_default();
    facts.push(AssemblyFactRow {
        fact: "authority".to_string(),
        value: Some(authority_label(authority).to_string()),
        from: None,
        field: None,
        to: None,
    });
    if let Some(reachability) = layer.and_then(AuthorLayer::reachability) {
        facts.push(AssemblyFactRow {
            fact: "reachability".to_string(),
            value: Some(severity_label(reachability.severity).to_string()),
            from: None,
            field: None,
            to: None,
        });
    }
    if let Some(layer) = layer {
        for edge in layer.edges() {
            facts.push(AssemblyFactRow {
                fact: "edge".to_string(),
                value: None,
                from: Some(edge.from.clone()),
                field: Some(edge.field.clone()),
                to: Some(edge.to.clone()),
            });
        }
    }
    facts
}

/// The lock label for a kind's declared projection format.
fn format_label(format: Format) -> String {
    match format {
        Format::YamlFrontmatter => "yaml-frontmatter".to_string(),
    }
}

/// The lock label for a kind's declared unit shape.
fn unit_shape_label(shape: UnitShape) -> String {
    match shape {
        UnitShape::File => "file".to_string(),
        UnitShape::Directory => "directory".to_string(),
    }
}

/// The lock label for a kind's declared activation — the field-carrying variants render
/// `via(field)`, matching the spec's `description-trigger(description)` shorthand.
fn activation_label(activation: &Activation) -> String {
    match activation {
        Activation::Always => "always".to_string(),
        Activation::DescriptionTrigger { field } => format!("description-trigger({field})"),
        Activation::PathsMatch { field } => format!("paths-match({field})"),
        Activation::Event { field } => format!("event({field})"),
    }
}

/// The lock label for a clause or reachability severity.
fn severity_label(severity: Severity) -> &'static str {
    match severity {
        Severity::Required => "required",
        Severity::Advisory => "advisory",
    }
}

/// The lock label for the assembly's surface-authority posture.
fn authority_label(authority: Authority) -> &'static str {
    match authority {
        Authority::Shared => "shared",
        Authority::Surface => "surface",
    }
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
    fn no_declared_custom_kind_imports_builtins_only() {
        // The base fixture carries skills and rules and a `specs/` corpus on disk,
        // but NO `temper.toml` declaring the `spec` kind — so discovery is
        // data-driven to nothing: the built-ins import, the `specs/` are ignored.
        // This is the guarantee that the old hardwired scan is gone.
        let harness = tmpdir("nospec-src");
        write_fixture_harness(&harness);
        let specs = harness.join("specs");
        fs::create_dir_all(&specs).unwrap();
        fs::write(specs.join("20-surface.md"), "# The config surface\n").unwrap();
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
    /// with `glob` — the shape the compiled-in table carries. A built-in kind rides the
    /// frontmatter face, so it needs no declared `format`.
    fn embedded_carrier(name: &str, provider: &str, root: &str, glob: &str) -> CustomKind {
        CustomKind {
            qualified: Some(format!("{provider}.{name}")),
            ..CustomKind::new(
                name,
                Governs {
                    root: root.to_string(),
                    glob: glob.to_string(),
                },
                Extraction::new(Vec::new()),
            )
        }
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
}
