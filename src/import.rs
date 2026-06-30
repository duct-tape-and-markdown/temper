//! `author import` — scan a Claude Code harness into the typed config surface.
//!
//! Implements `import` per `specs/20-surface.md` (the "CLI surface" verb
//! `author import` — scan → surface + provenance lock): discover
//! every skill under `<harness>` (a `skills/*/SKILL.md` layout, plus a bare
//! `<harness>` that is itself a skill directory), and for each one write the
//! surface tree `<into>/skills/<name>/` — a typed `meta.toml` header projected
//! with [`Skill::to_meta_document`] alongside the byte-faithful `SKILL.md` body
//! and every companion copied byte-for-byte. A roll-up index `<into>/author.toml`
//! records one `[[skill]]` entry per skill with its provenance and a `body_hash`.
//!
//! The keystone invariant (`.claude/rules/rust.md`) is idempotence: re-importing
//! an unchanged harness yields an identical workspace. It holds because every
//! written artifact is content-derived — `to_meta_document` renders the same
//! header deterministically, bodies and companions are copied verbatim, and the
//! roll-up is built from the skills in a fixed (name-sorted) order — and each
//! write overwrites in place rather than appending.

use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use toml_edit::{ArrayOfTables, DocumentMut, Item, Table, value};

use crate::skill::{Skill, SkillError};

/// Errors raised while importing a harness. Distinct from a [`SkillError`]
/// (which a malformed source skill produces) by also covering the surface-write
/// side: creating the workspace tree and copying companions.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ImportError {
    /// A source skill could not be read or projected.
    #[error(transparent)]
    #[diagnostic(transparent)]
    Skill(#[from] SkillError),

    /// The harness `skills/` directory could not be enumerated.
    #[error("failed to read harness directory {path}")]
    #[diagnostic(code(author::import::read_dir))]
    ReadDir {
        /// The directory whose listing failed.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// A surface file or directory could not be written.
    #[error("failed to write {path}")]
    #[diagnostic(code(author::import::write))]
    Write {
        /// The destination path that failed.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },
}

/// One row of the `author.toml` roll-up index: a skill's identity, its source
/// provenance, and the hash of its byte-faithful body.
struct RollupEntry {
    /// Skill name (and the `skills/<name>/` surface directory).
    name: String,
    /// Path to the original `SKILL.md`, as given relative to the harness arg.
    source_path: String,
    /// SHA-256 of the original `SKILL.md` bytes (the drift anchor).
    import_hash: String,
    /// SHA-256 of the byte-faithful body (frontmatter stripped).
    body_hash: String,
}

/// Import every skill under `harness_path` into the surface workspace `into`.
///
/// Writes `<into>/skills/<name>/{meta.toml, SKILL.md, ...companions}` per skill
/// and the `<into>/author.toml` roll-up index. Idempotent over an unchanged
/// harness. See the module header for the discovery rules and the invariant.
pub fn run(harness_path: &Path, into: &Path) -> miette::Result<()> {
    let dirs = discover_skill_dirs(harness_path)?;

    let mut rollup = Vec::with_capacity(dirs.len());
    for dir in &dirs {
        rollup.push(import_skill(dir, into)?);
    }

    // Sort by name so the roll-up — and thus the whole workspace — is stable
    // regardless of filesystem listing order.
    rollup.sort_by(|a, b| a.name.cmp(&b.name));
    write_rollup(into, &rollup)?;

    Ok(())
}

/// Find the skill directories under `harness`: a bare `<harness>` that is itself
/// a skill dir (has `SKILL.md`), followed by each immediate `skills/<name>/`
/// child that has one. Non-skill files and dirs are skipped.
fn discover_skill_dirs(harness: &Path) -> Result<Vec<PathBuf>, ImportError> {
    let mut dirs = Vec::new();

    if harness.join("SKILL.md").is_file() {
        dirs.push(harness.to_path_buf());
    }

    let skills_root = harness.join("skills");
    if skills_root.is_dir() {
        let listing = fs::read_dir(&skills_root).map_err(|source| ImportError::ReadDir {
            path: skills_root.clone(),
            source,
        })?;
        let mut children = Vec::new();
        for entry in listing {
            let entry = entry.map_err(|source| ImportError::ReadDir {
                path: skills_root.clone(),
                source,
            })?;
            let path = entry.path();
            if path.is_dir() && path.join("SKILL.md").is_file() {
                children.push(path);
            }
        }
        // `read_dir` order is unspecified; sort for deterministic processing.
        children.sort();
        dirs.extend(children);
    }

    Ok(dirs)
}

/// Read one source skill and write its surface tree under `<into>/skills/<name>/`,
/// returning the roll-up row for the index.
fn import_skill(source_dir: &Path, into: &Path) -> Result<RollupEntry, ImportError> {
    let skill = Skill::from_source_dir(source_dir)?;
    let out_dir = into.join("skills").join(&skill.name);
    create_dir_all(&out_dir)?;

    // Typed header via the format-preserving writer — never a lossy re-serialize.
    write_bytes(
        &out_dir.join("meta.toml"),
        skill.to_meta_document().to_string().as_bytes(),
    )?;
    // The surface `SKILL.md` is the body alone (no frontmatter), byte-faithful.
    write_bytes(&out_dir.join("SKILL.md"), skill.body.as_bytes())?;

    for companion in &skill.companions {
        copy_companion(source_dir, &out_dir, companion)?;
    }

    Ok(RollupEntry {
        name: skill.name,
        source_path: skill.provenance.source_path.to_string_lossy().into_owned(),
        import_hash: skill.provenance.import_hash,
        body_hash: sha256_hex(skill.body.as_bytes()),
    })
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

/// Write the `<into>/author.toml` roll-up: one `[[skill]]` table per imported
/// skill with `name`, `source_path`, `import_hash`, and `body_hash`.
fn write_rollup(into: &Path, rollup: &[RollupEntry]) -> Result<(), ImportError> {
    let mut tables = ArrayOfTables::new();
    for entry in rollup {
        let mut table = Table::new();
        table["name"] = value(entry.name.clone());
        table["source_path"] = value(entry.source_path.clone());
        table["import_hash"] = value(entry.import_hash.clone());
        table["body_hash"] = value(entry.body_hash.clone());
        tables.push(table);
    }

    let mut doc = DocumentMut::new();
    doc["skill"] = Item::ArrayOfTables(tables);

    create_dir_all(into)?;
    write_bytes(&into.join("author.toml"), doc.to_string().as_bytes())
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

/// Lowercase hex SHA-256 of `bytes`.
fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
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

    /// Build a harness with two skills under `skills/`; `coordinate` carries a
    /// companion markdown file and a nested script.
    fn write_fixture_harness(root: &Path) {
        let coordinate = root.join("skills").join("coordinate");
        fs::create_dir_all(coordinate.join("scripts")).unwrap();
        fs::write(coordinate.join("SKILL.md"), COORDINATE).unwrap();
        fs::write(coordinate.join("PLAYBOOK.md"), PLAYBOOK).unwrap();
        fs::write(coordinate.join("scripts").join("run.sh"), SCRIPT).unwrap();

        let demo = root.join("skills").join("demo");
        fs::create_dir_all(&demo).unwrap();
        fs::write(demo.join("SKILL.md"), DEMO).unwrap();
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

        // Per-skill surface dirs with header + body.
        let coord = into.join("skills").join("coordinate");
        assert!(coord.join("meta.toml").is_file());
        assert!(coord.join("SKILL.md").is_file());
        assert!(into.join("skills").join("demo").join("meta.toml").is_file());
        assert!(into.join("author.toml").is_file());

        // The surface SKILL.md is the body alone (no frontmatter), byte-faithful.
        let body = fs::read_to_string(coord.join("SKILL.md")).unwrap();
        assert_eq!(
            body,
            "# Coordinate\n\nSee PLAYBOOK.md for the full reference.   \nNo trailing newline here."
        );

        // The typed header round-trips back to the source skill.
        let reloaded = Skill::from_surface_dir(&coord).unwrap();
        assert_eq!(reloaded.name, "coordinate");
        assert_eq!(reloaded.version.as_deref(), Some("0.3.0"));
        assert!(reloaded.extra.contains_key("allowed-tools"));
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
    fn rollup_lists_one_entry_per_skill_with_both_hashes() {
        let harness = tmpdir("roll-src");
        write_fixture_harness(&harness);
        let into = tmpdir("roll-into");

        run(&harness, &into).unwrap();

        let doc = fs::read_to_string(into.join("author.toml"))
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();
        let skills = doc["skill"].as_array_of_tables().unwrap();

        // One entry per skill, name-sorted, each carrying both hashes.
        let names: Vec<&str> = skills.iter().map(|t| t["name"].as_str().unwrap()).collect();
        assert_eq!(names, vec!["coordinate", "demo"]);

        for table in skills.iter() {
            let import_hash = table["import_hash"].as_str().unwrap();
            let body_hash = table["body_hash"].as_str().unwrap();
            assert_eq!(import_hash.len(), 64);
            assert_eq!(body_hash.len(), 64);
            assert!(table["source_path"].as_str().unwrap().ends_with("SKILL.md"));
            // import_hash (whole source file) differs from body_hash (body only).
            assert_ne!(import_hash, body_hash);
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
    fn imports_a_bare_harness_that_is_itself_a_skill() {
        // A `<harness>` whose own SKILL.md makes it a skill dir, with no skills/.
        let harness = tmpdir("bare-src");
        fs::write(harness.join("SKILL.md"), DEMO).unwrap();
        let into = tmpdir("bare-into");

        run(&harness, &into).unwrap();

        assert!(into.join("skills").join("demo").join("meta.toml").is_file());
        let doc = fs::read_to_string(into.join("author.toml"))
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
        fs::write(harness.join("skills").join("README.md"), "not a skill\n").unwrap();
        fs::create_dir_all(harness.join("skills").join("empty")).unwrap();

        let into = tmpdir("skip-into");
        run(&harness, &into).unwrap();

        let doc = fs::read_to_string(into.join("author.toml"))
            .unwrap()
            .parse::<DocumentMut>()
            .unwrap();
        assert_eq!(doc["skill"].as_array_of_tables().unwrap().len(), 2);
        assert!(!into.join("skills").join("empty").exists());
    }
}
