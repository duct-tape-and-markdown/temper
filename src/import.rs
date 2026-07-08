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
use crate::kind::{CustomKind, Governs};

/// Filename of the generated roll-up index — the contents' state-of-record —
/// written at the workspace root.
const LOCK_FILENAME: &str = "lock.toml";

/// Directory name of the surface workspace (authored modules + lock) that
/// `install` scaffolds under the harness root. It is committed, not
/// gitignored, so without an explicit skip a `**` glob (e.g. `memory`'s
/// `**/CLAUDE.md`) would walk into it and count its contents as harness
/// members — the surface is categorically not a harness member.
const TEMPER_DIR: &str = ".temper";

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

/// Discover a built-in `kind`'s source files, keying off its declared `governs`
/// locus — the same data-driven scan a custom kind would get, so `skill`/`rule`
/// are no longer hardwired paths (the emit face's locus is the read face's scan root).
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
/// `pub(crate)` so drift re-scans the harness, and install's modeline placement
/// targets the same set, through the identical discovery `import` used.
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
/// row on an adopted
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
///
/// `pub(crate)` so the drift engine re-runs the same `governs`-keyed scan against a
/// live harness — every kind's members classify through the identical discovery
/// `import` used.
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
    // `**` glob would import a vendored dep's memory file. Resolved off the harness (repo) root so a
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

/// The set of paths under `harness` that discovery may see — every file and directory
/// the repo's ignore rules leave in, with `.git/` and the surface workspace
/// (`.temper/`) always excluded — the surface holds temper's own authored
/// modules and lock, never a harness member, and being committed (not
/// gitignored) it would otherwise enter the discoverable set on its own.
/// Built with
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
        .filter_entry(|entry| {
            entry.file_name() != OsStr::new(".git") && entry.file_name() != OsStr::new(TEMPER_DIR)
        })
        .build();
    // A walk error (an unreadable entry) drops that entry rather than aborting
    // discovery — the same tolerance the raw scan takes on `read_dir`.
    for entry in walk.flatten() {
        allowed.insert(crate::graph::normalize_path(entry.path()));
    }
    allowed
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

    use crate::builtin_kind;
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
