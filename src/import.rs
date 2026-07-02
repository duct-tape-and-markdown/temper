//! `temper import` — scan a Claude Code harness into the typed config surface.
//!
//! specs/20-surface.md, "Artifact kinds & contract selection"; custom kinds
//! specs/40-composition.md.
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

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use toml_edit::{ArrayOfTables, DocumentMut, Item, Table, value};

use crate::builtin_kind;
use crate::compose::AuthorLayer;
use crate::document::{self, Document};
use crate::frontmatter::{FrontmatterError, Member};
use crate::kind::{BUILTIN_KINDS, CustomKind, Governs, KindError};

/// Filename of the generated roll-up index — the contents' state-of-record —
/// written at the workspace root (`specs/20-surface.md`, "Topology").
const LOCK_FILENAME: &str = "lock.toml";

/// Errors raised while importing a harness. Distinct from a [`FrontmatterError`]
/// (which a malformed source member produces) by also covering the surface-write
/// side: creating the workspace tree and copying companions.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ImportError {
    /// A source member could not be read or projected through the generic
    /// frontmatter adapter (`specs/15-kinds.md`).
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
/// (`specs/20-surface.md`, "Drift / apply — three states").
pub(crate) struct RollupEntry {
    /// Artifact name (and its `<kind>/<name>/` surface directory).
    pub(crate) name: String,
    /// Path to the original source file, as given relative to the harness arg.
    pub(crate) source_path: String,
    /// SHA-256 of the original source bytes (the drift anchor).
    pub(crate) import_hash: String,
    /// The fingerprint of the source as it was when `temper` last projected the
    /// surface onto it — the **third state** the three-state merge needs, beside
    /// desired (the surface) and real (on-disk). It lets `apply` tell a surface
    /// edit from a world drift (`specs/20-surface.md`, "three states, never two").
    /// At import it equals `import_hash`: import writes a complete baseline, so the
    /// last thing applied to the source *is* the source as imported.
    pub(crate) last_applied: String,
}

/// Import every built-in artifact plus every declared custom-kind unit under
/// `harness_path` into the surface workspace `into`.
///
/// Idempotent over an unchanged harness. See the module header for the discovery
/// rules and the invariant.
pub fn run(harness_path: &Path, into: &Path) -> miette::Result<()> {
    // The built-in frontmatter kinds ride one generic adapter driven by each kind's
    // declared format + unit shape (`specs/15-kinds.md`) — no per-kind writer. Ordered
    // skill-then-rule so the roll-up's byte layout is stable.
    let skills = import_frontmatter_kind(harness_path, into, "skill")?;
    let rules = import_frontmatter_kind(harness_path, into, "rule")?;

    // A custom kind's definition — the `governs` locus discovery keys on — is the
    // authored `<harness>/.temper/kinds/<name>/KIND.md`, not an inline `temper.toml`
    // block (specs/40-composition.md). Absent a registered custom kind, only the
    // built-ins import.
    let layer = AuthorLayer::load(&harness_path.join("temper.toml"))?;
    let mut custom: BTreeMap<String, Vec<RollupEntry>> = BTreeMap::new();
    if let Some(layer) = &layer {
        let kinds_dir = harness_path.join(".temper").join("kinds");
        for name in layer.registered_kinds() {
            if BUILTIN_KINDS.contains(&name) {
                continue;
            }
            let kind = CustomKind::load(&kinds_dir, name)?;
            let unit_files = discover_kind_units(harness_path, &kind.governs)?;
            let mut units = Vec::with_capacity(unit_files.len());
            for file in &unit_files {
                units.push(import_custom_unit(&kind, file, into)?);
            }
            units.sort_by(|a, b| a.name.cmp(&b.name));
            custom.insert(name.to_string(), units);
        }
    }

    write_rollup(into, &skills, &rules, &custom)?;

    Ok(())
}

/// Import every source of one built-in frontmatter kind (`skill`, `rule`) into the
/// surface, driven by the kind's embedded declaration. Discover the source files off
/// the kind's `governs` locus, project each through the generic frontmatter adapter,
/// and return the roll-up rows name-sorted for a stable index.
///
/// The kind name is always an embedded built-in at every call site, so an absent
/// definition is a genuine invariant surfaced (not panicked) so import stays
/// panic-free (`.claude/rules/rust.md`).
fn import_frontmatter_kind(
    harness: &Path,
    into: &Path,
    name: &str,
) -> Result<Vec<RollupEntry>, ImportError> {
    let kind = builtin_kind::definition(name)?
        .expect("a built-in frontmatter kind resolves to an embedded KIND.md");
    let files = discover_builtin(harness, name)?;
    let mut rows = Vec::with_capacity(files.len());
    for file in &files {
        rows.push(import_frontmatter_member(&kind, file, into)?);
    }
    rows.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(rows)
}

/// Discover a built-in kind's source files by name, keying off the `governs` its
/// embedded `KIND.md` declares — the same data-driven scan a custom kind gets, so
/// `skill`/`rule` are no longer hardwired paths (`specs/15-kinds.md`, "A built-in
/// kind is an adapter": the emit face's locus is the read face's scan root). The
/// `skill` locus (`.claude/skills` + `*/SKILL.md`) resolves through the generalized
/// subdir glob; `rule`'s (`.claude/rules` + `*.md`) is flat. Yields the member
/// source *files* — for a skill the `SKILL.md`, not its directory.
///
/// The bare-harness-is-a-skill case — a `<harness>/SKILL.md`, a project root that is
/// itself a skill — is Claude Code's own convention, outside the `.claude/skills`
/// locus the `governs` scan covers, so it is layered on for the `skill` kind only.
///
/// `pub(crate)` so drift re-scans the harness, and install's modeline placement
/// targets the same set, through the identical discovery `import` used
/// (`specs/20-surface.md`, the drift "added" axis).
pub(crate) fn discover_builtin(harness: &Path, name: &str) -> Result<Vec<PathBuf>, ImportError> {
    let mut files = discover_kind_units(harness, &builtin_governs(name)?)?;
    if name == "skill" {
        let bare = harness.join("SKILL.md");
        if bare.is_file() {
            files.push(bare);
            // Re-sort so the bare root skill lands in name order beside the children.
            files.sort();
        }
    }
    Ok(files)
}

/// The `governs` locus of a built-in kind, read off its embedded `KIND.md` through
/// the same [`builtin_kind::definition`] parse `check` uses. `name` is always an
/// embedded built-in (`skill`, `rule`) at every call site, so an absent definition
/// is a genuine invariant.
fn builtin_governs(name: &str) -> Result<Governs, ImportError> {
    let kind = builtin_kind::definition(name)?
        .expect("a built-in kind name resolves to an embedded KIND.md");
    Ok(kind.governs)
}

/// Read one source member of a frontmatter `kind` and write its surface tree under
/// `<into>/<subdir>/<id>/`, returning the roll-up row for the index. The one write
/// path for every frontmatter kind (`specs/15-kinds.md`, "the adapter faces are
/// declared"): the member document via [`Member::to_document`], plus the copied
/// companions of a directory-shaped unit. The surface subdir is the kind's declared
/// `governs` leaf, the member document its declared name (`SKILL.md`, `RULE.md`).
///
/// `pub(crate)` so `re-add` reuses this exact round-trip write path when it pulls a
/// drifted or added on-disk source back into the surface, rather than re-implementing
/// the projection (`specs/20-surface.md`, "Drift / apply").
pub(crate) fn import_frontmatter_member(
    kind: &CustomKind,
    source_file: &Path,
    into: &Path,
) -> Result<RollupEntry, ImportError> {
    let mut member = Member::from_source(kind, source_file)?;
    let out_dir = into.join(kind.surface_subdir()).join(&member.id);

    // Merge, never clobber: the source carries no authored clauses (they are
    // surface-only state), so a re-import or drifted-body `re-add` rebuilds the
    // document from source and would wipe the authored `satisfies`/`edges`. Carry
    // any existing surface layer forward before writing (`specs/20-surface.md`,
    // "three states, never two").
    let member_doc = kind.member_document();
    if let Some(existing) = existing_surface_member(&out_dir, &member_doc) {
        member.carry_representation(&existing);
    }

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
            copy_companion(source_dir, &out_dir, companion)?;
        }
    }

    Ok(RollupEntry {
        name: member.id,
        source_path: member.provenance.source_path.to_string_lossy().into_owned(),
        // At import the last-applied fingerprint is the import hash: the source as
        // it stands on disk is exactly what the surface was just derived from.
        last_applied: member.provenance.import_hash.clone(),
        import_hash: member.provenance.import_hash,
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
/// immediate files) or carry a **subdirectory** segment (`*/SKILL.md` — a file inside
/// each matching immediate child); the one scanner resolves both, so it serves every
/// custom kind and the built-in `skill`/`rule` loci alike. Non-matching entries are
/// skipped, and a missing root yields an empty list (a declared kind whose corpus
/// does not exist on this harness). Data-driven discovery — the locus is the kind's
/// own `governs` declaration (`specs/40-composition.md`), never a hardwired path.
///
/// `pub(crate)` so the drift engine re-runs the same `governs`-keyed scan against a
/// live harness — every kind's members classify through the identical discovery
/// `import` used (`specs/20-surface.md`, the drift "added" axis).
pub(crate) fn discover_kind_units(
    harness: &Path,
    governs: &Governs,
) -> Result<Vec<PathBuf>, ImportError> {
    let root = harness.join(&governs.root);
    // A glob is a `/`-separated segment list: the final segment matches files, each
    // earlier one an immediate subdirectory to descend into. `split` always yields at
    // least one segment.
    let segments: Vec<&str> = governs.glob.split('/').collect();
    let mut files = Vec::new();
    collect_glob(&root, &segments, &mut files)?;
    // `read_dir` order is unspecified; sort for deterministic processing.
    files.sort();
    Ok(files)
}

/// Walk `dir` collecting every file whose path matches the remaining glob
/// `segments`. The head segment selects entries at this level; if it is the last,
/// matching **files** are collected, otherwise matching **subdirectories** are
/// descended. A missing or non-directory `dir` contributes nothing — a subdir glob
/// whose intermediate level is absent, or a locus that does not exist on this
/// harness, both resolve to no units rather than an error.
fn collect_glob(dir: &Path, segments: &[&str], out: &mut Vec<PathBuf>) -> Result<(), ImportError> {
    if !dir.is_dir() {
        return Ok(());
    }
    let (segment, rest) = segments
        .split_first()
        .expect("a governs glob has at least one path segment");
    let listing = fs::read_dir(dir).map_err(|source| ImportError::ReadDir {
        path: dir.to_path_buf(),
        source,
    })?;
    for entry in listing {
        let entry = entry.map_err(|source| ImportError::ReadDir {
            path: dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if !glob_matches(segment, name) {
            continue;
        }
        if rest.is_empty() {
            if path.is_file() {
                out.push(path);
            }
        } else if path.is_dir() {
            collect_glob(&path, rest, out)?;
        }
    }
    Ok(())
}

/// Read one discovered custom-kind unit and write its surface tree under
/// `<into>/<governs.root>/<name>/`, returning the roll-up row for the index.
///
/// A custom kind carries no bespoke IR, so the unit is projected generically as ONE
/// member document `<KIND>.md`: a `[provenance]`-only `+++` header over the *whole*
/// file byte-faithful. The whole file is the body so a unit's frontmatter, if any,
/// survives verbatim for its extractor to read at `check` time (`specs/15-kinds.md`).
///
/// `pub(crate)` for the same reason as [`import_frontmatter_member`]: `re-add` reuses
/// this exact generic write path to reconcile a drifted or added on-disk custom-kind
/// unit into the surface, folding the returned row straight into the lock.
pub(crate) fn import_custom_unit(
    kind: &CustomKind,
    source_file: &Path,
    into: &Path,
) -> Result<RollupEntry, ImportError> {
    let bytes = fs::read(source_file).map_err(|source| ImportError::ReadDir {
        path: source_file.to_path_buf(),
        source,
    })?;
    let import_hash = crate::hash::sha256_hex(&bytes);
    let body = String::from_utf8(bytes).map_err(|source| ImportError::NotUtf8 {
        path: source_file.to_path_buf(),
        source,
    })?;
    let name = source_file
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(str::to_string)
        .ok_or_else(|| ImportError::Write {
            path: source_file.to_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "source file has no stem to name the unit",
            ),
        })?;

    let out_dir = into.join(&kind.governs.root).join(&name);
    create_dir_all(&out_dir)?;

    // The member is ONE document: a provenance-only `+++` header over the whole
    // byte-faithful source file as the body, written format-preserving.
    let mut header = DocumentMut::new();
    document::add_provenance(&mut header, &source_file.to_string_lossy(), &import_hash);
    write_bytes(
        &out_dir.join(body_filename(&kind.name)),
        Document::new(header, body).emit().as_bytes(),
    )?;

    Ok(RollupEntry {
        name,
        source_path: source_file.to_string_lossy().into_owned(),
        // At import the last-applied fingerprint is the import hash (see the built-in
        // frontmatter member writer).
        last_applied: import_hash.clone(),
        import_hash,
    })
}

/// The byte-faithful body filename for a custom kind — the kind name upper-cased
/// with a `.md` suffix (`spec` → `SPEC.md`), mirroring the built-in `SKILL.md` and
/// `RULE.md` bodies so a custom kind's surface reads uniformly with them.
fn body_filename(kind: &str) -> String {
    format!("{}.md", kind.to_uppercase())
}

/// Whether `glob` matches `name`, treating `*` as "any run of characters (including
/// empty)" and every other character literally — the minimal in-crate wildcard a
/// `governs` glob needs (`*.md`), short of pulling in a glob crate for one
/// metacharacter, kept local so `import` stays self-contained
/// (`.claude/rules/rust.md`). A standard linear matcher with
/// single-star backtracking: on a mismatch it falls back to the most recent `*`,
/// extending what that star consumed by one character.
fn glob_matches(glob: &str, name: &str) -> bool {
    let pattern: Vec<char> = glob.chars().collect();
    let text: Vec<char> = name.chars().collect();
    let mut pi = 0;
    let mut ti = 0;
    // The position of the last `*` in `pattern`, and how much of `text` it had
    // consumed when we matched it — the backtrack point.
    let mut star: Option<usize> = None;
    let mut star_ti = 0;
    while ti < text.len() {
        if pi < pattern.len() && pattern[pi] == text[ti] {
            pi += 1;
            ti += 1;
        } else if pi < pattern.len() && pattern[pi] == '*' {
            star = Some(pi);
            star_ti = ti;
            pi += 1;
        } else if let Some(star_pi) = star {
            // Mismatch under an open `*`: let the star swallow one more character.
            pi = star_pi + 1;
            star_ti += 1;
            ti = star_ti;
        } else {
            return false;
        }
    }
    // Trailing `*`s match the empty remainder.
    while pi < pattern.len() && pattern[pi] == '*' {
        pi += 1;
    }
    pi == pattern.len()
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

/// Write the `<into>/lock.toml` roll-up: one `[[skill]]` table per imported
/// skill, then one `[[rule]]` table per imported rule, then one `[[<kind>]]` table
/// per imported custom-kind unit (custom kinds in name order), each with `name`,
/// `source_path`, `import_hash`, and the `last_applied` fingerprint.
///
/// An empty kind renders to no bytes (an empty `ArrayOfTables` emits nothing), so
/// a harness with only some kinds yields exactly the rows it has — a skill-only
/// harness with no declared custom kind emits `[[skill]]` rows and nothing else.
fn write_rollup(
    into: &Path,
    skills: &[RollupEntry],
    rules: &[RollupEntry],
    custom: &BTreeMap<String, Vec<RollupEntry>>,
) -> Result<(), ImportError> {
    let mut doc = DocumentMut::new();
    doc["skill"] = Item::ArrayOfTables(rollup_tables(skills));
    doc["rule"] = Item::ArrayOfTables(rollup_tables(rules));
    for (kind, units) in custom {
        doc[kind.as_str()] = Item::ArrayOfTables(rollup_tables(units));
    }

    create_dir_all(into)?;
    write_bytes(&into.join(LOCK_FILENAME), doc.to_string().as_bytes())
}

/// Build the `ArrayOfTables` for one kind's roll-up rows — the four shared columns
/// (`name`, `source_path`, `import_hash`, `last_applied`) in a fixed order, one
/// table per entry.
fn rollup_tables(rollup: &[RollupEntry]) -> ArrayOfTables {
    let mut tables = ArrayOfTables::new();
    for entry in rollup {
        let mut table = Table::new();
        table["name"] = value(entry.name.clone());
        table["source_path"] = value(entry.source_path.clone());
        table["import_hash"] = value(entry.import_hash.clone());
        table["last_applied"] = value(entry.last_applied.clone());
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
version: \"0.3.0\"\n\
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
    /// (`specs/40-composition.md`, "Decision: a custom kind is an authored `.temper/`
    /// artifact, registered in the assembly").
    const SPEC_TEMPER_TOML: &str = "[kind.spec]\npackage = \"spec\"\n";

    /// The authored `spec` KIND.md definition (`specs/20-surface.md`, "Decision: a kind
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
            reloaded.field("version").and_then(|v| v.as_str()),
            Some("0.3.0")
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
            let import_hash = table["import_hash"].as_str().unwrap();
            let last_applied = table["last_applied"].as_str().unwrap();
            assert_eq!(import_hash.len(), 64);
            assert!(table["source_path"].as_str().unwrap().ends_with("SKILL.md"));
            // The retired `body_hash` column is gone — no production reader.
            assert!(table.get("body_hash").is_none());
            // The baseline: at import the last-applied fingerprint is the import
            // hash — the surface was just derived from the source as it stands.
            assert_eq!(last_applied, import_hash);
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
            assert_eq!(table["import_hash"].as_str().unwrap().len(), 64);
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

        // The skill locus (`.claude/skills` + `*/SKILL.md`) yields the `SKILL.md`
        // files themselves — the subdir glob descended one level.
        let skills = discover_builtin(&harness, "skill").unwrap();
        assert_eq!(
            skills,
            vec![
                harness.join(".claude/skills/coordinate").join("SKILL.md"),
                harness.join(".claude/skills/demo").join("SKILL.md"),
            ]
        );

        // The rule locus (`.claude/rules` + `*.md`) is flat — immediate `*.md` files.
        let rules = discover_builtin(&harness, "rule").unwrap();
        assert_eq!(
            rules,
            vec![
                harness.join(".claude/rules/collaboration.md"),
                harness.join(".claude/rules/rust.md"),
            ]
        );
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
            assert_eq!(table["import_hash"].as_str().unwrap().len(), 64);
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
                .all(|t| t["import_hash"].as_str().unwrap().len() == 64)
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
}
