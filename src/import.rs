//! Harness discovery and the `lock.toml` roll-up writer.
//!
//! The discovery walk (`discover_kind_units`/`discover_builtin`) is the sole member
//! extractor the gate and `emit`'s lock-writer ([`write_rollup`]) both ride.
//! The `init`/`lift` on-ramp verbs that once wrote
//! an in-place `[[member]]` table over members in place retired with the `[[member]]`
//! codec (`CODEC-RETIRE`) — `install` is the
//! on-ramp going forward; a trunk gap between the two is an
//! accepted clean-slate cost (John, 2026-07-06).
//!
//! Keystone invariant (`.claude/rules/rust.md`): idempotence. It holds because
//! every write is content-derived, name-sorted, and overwrites in place.

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use ignore::WalkBuilder;
use toml_edit::{ArrayOfTables, DocumentMut, Item, Table, value};

use crate::drift::Declarations;
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

/// Errors raised while discovering or rolling up a harness's members.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ImportError {
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
/// provenance, and its two freshness facts — disk-vs-lock drift's whole comparison.
/// Shared by every kind —
/// a `[[skill]]`, `[[rule]]`, and every custom `[[<kind>]]` row all carry the same
/// four columns.
///
/// `pub(crate)` so `emit` ([`crate::drift`]) can build the row for a freshly
/// projected member and hand it to this module's single round-trip write path
/// ([`write_rollup`]) rather than re-deriving the fingerprints.
pub(crate) struct RollupEntry {
    /// Artifact name (and its `<kind>/<name>/` surface directory).
    pub(crate) name: String,
    /// Path to the original source file, as given relative to the harness arg.
    pub(crate) source_path: String,
    /// SHA-256 of the authored source bytes — the **source freshness fact**, the
    /// anchor source-drift detection compares against.
    pub(crate) source_hash: String,
    /// SHA-256 of the last emitted projection — the **emit freshness fact**, the
    /// baseline `config.stale` and projection freshness compare a committed output
    /// against. At import it provisionally equals `source_hash`: no `emit` has run
    /// yet, so the last thing projected onto the source is the source as imported
    /// (`emit` advances it once it lands).
    pub(crate) emit_hash: String,
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
    harness: &Path,
    kind: &CustomKind,
    kinds: &BTreeMap<String, CustomKind>,
    over: LocalOverride,
) -> Result<Vec<PathBuf>, ImportError> {
    match &kind.governs {
        Some(governs) => discover_kind_files(harness, kind, governs, over),
        None => Ok(discover_nested_file(harness, kind, kinds, over)?
            .into_iter()
            .map(|unit| unit.file)
            .collect()),
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
///
/// # Errors
///
/// Returns an [`ImportError`] if a host's locus or unit cannot be enumerated.
pub fn discover_nested_file(
    harness: &Path,
    kind: &CustomKind,
    kinds: &BTreeMap<String, CustomKind>,
    over: LocalOverride,
) -> Result<Vec<NestedFileUnit>, ImportError> {
    // A path a declared kind's own `governs` locus claims has one home — that kind's
    // member — so it is carved out of every host template's discovery here, at the
    // single point one path is decided. Without the carve a declared exact-path kind and
    // a host template both materialize the path: a phantom twin the coverage, `explain`,
    // and `degree` consumers would each then have to un-see. Position stays decidable at
    // this one seam instead.
    let claimed = declared_governed_paths(harness, kinds, over)?;
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
        let discoverable = discoverable_paths(harness, local_governs(host, over));
        let root = harness.join(&governs.root);
        for entry in discover_kind_files(harness, host, governs, over)? {
            let Some(host_unit) = unit_dir(&root, &entry) else {
                continue;
            };
            for file in scan_locus(&host_unit, pattern, &discoverable)? {
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
    Ok(found)
}

/// Every path some declared kind's own `governs` locus claims, across `kinds` — the set a
/// host template's discovery carves out so a declared kind's member is the sole home of its
/// path. A nested file kind governs no locus of its own (a template child's path is its
/// host's fact), so it contributes nothing; only the kinds carrying a `governs` pair claim
/// paths, scanned through the same `discover_kind_files` walk discovery itself rides.
///
/// # Errors
///
/// Returns an [`ImportError`] if a declared kind's locus cannot be enumerated.
fn declared_governed_paths(
    harness: &Path,
    kinds: &BTreeMap<String, CustomKind>,
    over: LocalOverride,
) -> Result<BTreeSet<PathBuf>, ImportError> {
    let mut claimed = BTreeSet::new();
    for kind in kinds.values() {
        if let Some(governs) = kind.governs.as_ref() {
            claimed.extend(discover_kind_files(harness, kind, governs, over)?);
        }
    }
    Ok(claimed)
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
/// `governs` locus — the generalized scan [`discover_kind_units`] runs, plus `skill`'s
/// bare-root special case (a `<harness>/SKILL.md`, a harness that is itself a skill).
/// Decoupled from the kind's own [`CustomKind::governs`] so a caller can walk a
/// *different* declared locus for the same kind — the committed lock's own kind-fact
/// row on an adopted
/// harness, the kind's embedded default otherwise (the built-in lock) — while the
/// bare-root-skill convention still applies wherever `skill`'s locus is walked from.
/// [`discover_builtin`] is the thin caller that always walks the kind's own governs.
///
/// `over` decides whether the kind's own commitment class may override discovery's
/// presumptions for this walk; the `kind` is what carries that class, which is why the
/// generalized scan cannot decide it off `governs` alone.
///
/// # Errors
///
/// Returns an [`ImportError`] if a directory under `governs.root` cannot be
/// enumerated.
pub fn discover_kind_files(
    harness: &Path,
    kind: &CustomKind,
    governs: &Governs,
    over: LocalOverride,
) -> Result<Vec<PathBuf>, ImportError> {
    let mut files = discover_kind_units(harness, governs, local_governs(kind, over))?;
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
fn discover_kind_units(
    harness: &Path,
    governs: &Governs,
    local_governs: bool,
) -> Result<Vec<PathBuf>, ImportError> {
    // A member is authored content; an ignored file is by declaration not authored here,
    // so discovery sees only what the repo's ignore rules leave in — else a `**` glob
    // would import a vendored dep's memory file. A local-locus kind's own walk is the one
    // exception, and its `governs` says so: `local_governs` carries that scope in.
    // Resolved off the harness (repo) root so a root `.gitignore` governs every kind's
    // walk, whatever its `governs.root` depth.
    let discoverable = discoverable_paths(harness, local_governs);
    scan_locus(&harness.join(&governs.root), &governs.glob, &discoverable)
}

/// The scan itself: every file under `root` matching `glob`, deterministically ordered.
/// Split from [`discover_kind_units`] so a nested file child's walk under each host unit
/// rides the same matcher and the same already-computed `discoverable` set — one scanner
/// serves every kind's locus, host and child alike.
fn scan_locus(
    root: &Path,
    glob: &str,
    discoverable: &BTreeSet<PathBuf>,
) -> Result<Vec<PathBuf>, ImportError> {
    // A glob is a `/`-separated segment list: the final segment matches files, each
    // earlier one a subdirectory to descend into — a `**` segment descending any
    // number of levels. `split` always yields at least one segment.
    let segments: Vec<&str> = glob.split('/').collect();
    let mut files = Vec::new();
    collect_glob(root, &segments, discoverable, &mut files)?;
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
/// memory nesting) is discovered at every level, not just the fixed glob depth.
/// A missing or non-directory `dir`
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
            if path.is_dir() && discoverable.contains(&crate::graph::normalize_path(&path)) {
                collect_glob(&path, segments, discoverable, out)?;
            }
        }
        return Ok(());
    }
    for entry in read_entries(dir)? {
        let path = entry.path();
        // An ignored entry is not authored here — skip it whether it would be
        // collected as a file or descended as a subdirectory.
        if !discoverable.contains(&crate::graph::normalize_path(&path)) {
            continue;
        }
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if !crate::kind::compile_glob(segment).is_some_and(|matcher| matcher.is_match(name)) {
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

/// The set of paths under `harness` that a walk for a kind whose locus `local_governs`
/// may see. Two presumptions prune it, and the flag waives **both, together**: the
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
fn discoverable_paths(harness: &Path, local_governs: bool) -> BTreeSet<PathBuf> {
    let mut allowed = BTreeSet::new();
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
    // A walk error (an unreadable entry) drops that entry rather than aborting
    // discovery — the same tolerance the raw scan takes on `read_dir`.
    for entry in walk.flatten() {
        allowed.insert(crate::graph::normalize_path(entry.path()));
    }
    allowed
}

/// Whether `dir` carries its own `.temper/lock.toml` — the mark of an independently
/// governed harness whose members are its own corpus, not the enclosing walk's.
fn is_governed_root(dir: &Path) -> bool {
    dir.join(crate::WORKSPACE_DIR)
        .join(crate::LOCK_FILENAME)
        .is_file()
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
/// key-sorted, so the emitted order is deterministic. `drift::emit` is the sole caller:
/// a kind with no emitted
/// member simply has no entry, matching the toml round-trip reality — an empty
/// `ArrayOfTables` emits nothing, so a written-then-vanished section would break
/// idempotence against a re-parse that never sees it.
///
/// After the per-member sections come the program's **declaration rows** — kind facts,
/// clauses, requirements, assembly facts under an implicit `[declaration]` table;
/// the drift/gate side reads them
/// through [`crate::drift::read_declarations`]. The `nested_member` family carries the
/// program's own embedded-member facts *and* the rows emit derives from layout sources
/// in the same pass (`crate::drift::emit` merges them before this write), so a layout
/// document's members reach the lock as declaration rows without a projection of their own.
/// `layout_imports` and `includes` are the layout sources' and composed prose's
/// fingerprinted content dependencies, written into the same `[declaration]` table under
/// their own families.
pub(crate) fn write_rollup(
    into: &Path,
    builtins: &BTreeMap<String, Vec<RollupEntry>>,
    custom: &BTreeMap<String, Vec<RollupEntry>>,
    declarations: &Declarations,
    layout_imports: &[crate::drift::LayoutImportRow],
    includes: &[crate::drift::LayoutImportRow],
) -> Result<(), ImportError> {
    let mut doc = DocumentMut::new();
    for (kind, rows) in builtins {
        doc[kind.as_str()] = Item::ArrayOfTables(rollup_tables(rows));
    }
    for (kind, units) in custom {
        doc[kind.as_str()] = Item::ArrayOfTables(rollup_tables(units));
    }
    declarations.write_into(&mut doc);
    // The source-dependency fingerprints ride the same `[declaration]` table, each in its
    // own family, engine-derived alongside the program's own declaration rows.
    crate::drift::write_source_deps(&mut doc, "layout_import", layout_imports);
    crate::drift::write_source_deps(&mut doc, "include", includes);

    create_dir_all(into)?;
    write_bytes(&into.join(crate::LOCK_FILENAME), doc.to_string().as_bytes())
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

        let skill_kind = builtin_kind::definition("skill").unwrap().unwrap();
        let rule_kind = builtin_kind::definition("rule").unwrap().unwrap();

        // The skill locus (`.claude/skills` + `*/SKILL.md`) yields the `SKILL.md`
        // files themselves — the subdir glob descended one level.
        let skills =
            discover_builtin(&harness, &skill_kind, &no_hosts(), LocalOverride::Honored).unwrap();
        assert_eq!(
            skills,
            vec![
                harness.join(".claude/skills/coordinate").join("SKILL.md"),
                harness.join(".claude/skills/demo").join("SKILL.md"),
            ]
        );

        // The rule locus (`.claude/rules` + `*.md`) is flat — immediate `*.md` files.
        let rules =
            discover_builtin(&harness, &rule_kind, &no_hosts(), LocalOverride::Honored).unwrap();
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

        let found =
            discover_builtin(&harness, &memory, &no_hosts(), LocalOverride::Honored).unwrap();
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
        let found = discover_kind_units(&harness, &governs, false).unwrap();
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

        let adopted = discover_builtin(&harness, &local, &no_hosts(), LocalOverride::Withheld);
        assert_eq!(adopted.unwrap(), Vec::<PathBuf>::new());

        let read = discover_builtin(&harness, &local, &no_hosts(), LocalOverride::Honored);
        assert_eq!(
            read.unwrap(),
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
            &harness,
            &dial_kind(None),
            &no_hosts(),
            LocalOverride::Honored,
        );
        assert_eq!(committed.unwrap(), Vec::<PathBuf>::new());
    }

    #[test]
    fn discover_builtin_finds_a_bare_harness_that_is_itself_a_skill() {
        // A `<harness>` whose own SKILL.md makes it a skill dir, with no skills/ — the
        // real bare-skill-repo shape, not a tmpdir artifact.
        let harness = tmpdir("bare-src").join("demo");
        fs::create_dir_all(&harness).unwrap();
        fs::write(harness.join("SKILL.md"), DEMO).unwrap();

        let skill_kind = builtin_kind::definition("skill").unwrap().unwrap();
        let found =
            discover_builtin(&harness, &skill_kind, &no_hosts(), LocalOverride::Honored).unwrap();
        assert_eq!(found, vec![harness.join("SKILL.md")]);
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

        let found =
            discover_builtin(&harness, &memory, &no_hosts(), LocalOverride::Honored).unwrap();
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

        let skill_kind = builtin_kind::definition("skill").unwrap().unwrap();
        let found =
            discover_builtin(&harness, &skill_kind, &no_hosts(), LocalOverride::Honored).unwrap();
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
            .unwrap()
            .overlay_templates(&[TemplateRow {
                kind: "reference-doc".to_string(),
                path: Some("*.md".to_string()),
            }]);
        let kinds = BTreeMap::from([("skill".to_string(), host)]);
        let child = CustomKind::nested_file("reference-doc", Extraction::new(Vec::new()));

        // `coordinate`'s companion doc, and nothing from `demo` (which carries none) or the
        // hosts' own `SKILL.md` entry files.
        let found = discover_builtin(&harness, &child, &kinds, LocalOverride::Honored).unwrap();
        assert_eq!(
            found,
            vec![
                harness
                    .join(".claude/skills/coordinate")
                    .join("PLAYBOOK.md")
            ]
        );
    }
}
