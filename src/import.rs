//! `temper import` — scan a Claude Code harness into the typed config surface.
//!
//! Implements `import` per `specs/20-surface.md` ("Artifact kinds & contract
//! selection"): `import` scans every **built-in** harness kind (`skills/*/SKILL.md`,
//! `.claude/rules/*.md`) *plus* every **custom** kind the active `temper.toml`
//! declares (`specs/40-composition.md`). For each skill it writes the surface tree
//! `<into>/skills/<name>/` — a typed `meta.toml` header projected with
//! [`Skill::to_meta_document`] alongside the byte-faithful `SKILL.md` body and
//! every companion copied byte-for-byte. For each rule it writes the parallel
//! tree `<into>/rules/<name>/` — a `meta.toml` header projected with
//! [`Rule::to_meta_document`] (the optional `paths` + `[provenance]`) alongside
//! the byte-faithful `RULE.md` body.
//!
//! ## Custom kinds are discovered from `temper.toml`, never hardwired
//!
//! A custom kind (a project's own specs, ADRs, playbooks; `specs/15-kinds.md`)
//! carries no bespoke IR. Its units are discovered **data-driven** from the kind's
//! declared [`governs`](crate::compose::Governs) locus — a root directory and a
//! filename glob ([`AuthorLayer::custom_kinds`]) — and each is projected to
//! `<into>/<root>/<name>/`: a provenance-only `meta.toml` (`[provenance]` alone —
//! a custom unit's typed header is composed by its extractor, not re-serialized
//! here) alongside the byte-faithful whole file as `<KIND>.md`. This is exactly why
//! "temper reads its own `specs/` because its own `temper.toml` declares the `spec`
//! kind, not because anything is hardwired" (`specs/40-composition.md`): absent a
//! `temper.toml` custom kind, `import` writes the built-ins only — there is no
//! phantom `specs/` scan.
//!
//! A roll-up index `<into>/lock.toml` records one `[[skill]]`/`[[rule]]` entry
//! per built-in artifact, then one `[[<kind>]]` entry per custom-kind unit, each
//! with its provenance and the `last_applied` fingerprint the three-state
//! drift/apply merge stands on (at import: equal to `import_hash`).
//!
//! Note the root asymmetry the built-in kinds carry: skills live at
//! `<harness>/skills/`, rules at `<harness>/.claude/rules/`. A custom kind sits
//! wherever its `governs` root names (the `spec` kind's `specs/`, no `.claude/`
//! prefix — temper's own corpus, not a Claude Code artifact).
//!
//! The keystone invariant (`.claude/rules/rust.md`) is idempotence: re-importing
//! an unchanged harness yields an identical workspace. It holds because every
//! written artifact is content-derived — headers render deterministically, bodies
//! and companions are copied verbatim, and the roll-up is built from the artifacts
//! in a fixed (name-sorted) order — and each write overwrites in place rather than
//! appending.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use toml_edit::{ArrayOfTables, DocumentMut, Item, Table, value};

use crate::compose::{AuthorLayer, CustomKind, Governs};
use crate::rule::{Rule, RuleError};
use crate::skill::{Skill, SkillError};

/// Filename of the generated roll-up index — the contents' state-of-record —
/// written at the workspace root (`specs/20-surface.md`, "Topology").
const LOCK_FILENAME: &str = "lock.toml";

/// Errors raised while importing a harness. Distinct from a [`SkillError`]
/// (which a malformed source skill produces) by also covering the surface-write
/// side: creating the workspace tree and copying companions.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum ImportError {
    /// A source skill could not be read or projected.
    #[error(transparent)]
    #[diagnostic(transparent)]
    Skill(#[from] SkillError),

    /// A source rule could not be read or projected.
    #[error(transparent)]
    #[diagnostic(transparent)]
    Rule(#[from] RuleError),

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
/// Writes `<into>/skills/<name>/{meta.toml, SKILL.md, ...companions}` per skill and
/// `<into>/rules/<name>/{meta.toml, RULE.md}` per rule — the built-in kinds — then,
/// for every custom kind the project-root `<harness_path>/temper.toml` declares,
/// discovers its [`governs`](crate::compose::Governs) locus and writes
/// `<into>/<root>/<name>/{meta.toml, <KIND>.md}` per unit. Finally the
/// `<into>/lock.toml` roll-up index carries one `[[skill]]`/`[[rule]]` row per
/// built-in artifact and one `[[<kind>]]` row per custom-kind unit.
///
/// Spec discovery is now just a custom kind like any other — absent a `temper.toml`
/// declaring one, `import` writes the built-ins only (no phantom `specs/` scan).
/// Idempotent over an unchanged harness. See the module header for the discovery
/// rules and the invariant.
pub fn run(harness_path: &Path, into: &Path) -> miette::Result<()> {
    let skill_dirs = discover_skill_dirs(harness_path)?;
    let mut skills = Vec::with_capacity(skill_dirs.len());
    for dir in &skill_dirs {
        skills.push(import_skill(dir, into)?);
    }

    let rule_files = discover_rule_files(harness_path)?;
    let mut rules = Vec::with_capacity(rule_files.len());
    for file in &rule_files {
        rules.push(import_rule(file, into)?);
    }

    // The custom kinds the project-root `temper.toml` declares. Absent ⇒ `None`,
    // so a harness with no `temper.toml` (or none declaring a custom kind) imports
    // the built-ins alone — the hardwired `specs/*.md` scan is gone.
    let layer = AuthorLayer::load(&harness_path.join("temper.toml"))?;
    let mut custom: BTreeMap<String, Vec<RollupEntry>> = BTreeMap::new();
    if let Some(layer) = &layer {
        for (name, kind) in layer.custom_kinds() {
            let unit_files = discover_kind_units(harness_path, &kind.governs)?;
            let mut units = Vec::with_capacity(unit_files.len());
            for file in &unit_files {
                units.push(import_custom_unit(kind, file, into)?);
            }
            units.sort_by(|a, b| a.name.cmp(&b.name));
            custom.insert(name.clone(), units);
        }
    }

    // Sort by name so the roll-up — and thus the whole workspace — is stable
    // regardless of filesystem listing order.
    skills.sort_by(|a, b| a.name.cmp(&b.name));
    rules.sort_by(|a, b| a.name.cmp(&b.name));
    write_rollup(into, &skills, &rules, &custom)?;

    Ok(())
}

/// Find the skill directories under `harness`: a bare `<harness>` that is itself
/// a skill dir (has `SKILL.md`), followed by each immediate `skills/<name>/`
/// child that has one. Non-skill files and dirs are skipped.
///
/// `pub(crate)` so the drift engine can re-run the same per-kind scan against a
/// live harness without duplicating the discovery rules (`specs/20-surface.md`,
/// the drift "added" axis).
pub(crate) fn discover_skill_dirs(harness: &Path) -> Result<Vec<PathBuf>, ImportError> {
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

/// Find the rule source files under `<harness>/.claude/rules/`: every immediate
/// `*.md` child. Non-markdown files and subdirectories are skipped. Note the root
/// asymmetry with skills — rules live under `.claude/rules/`, not at the harness
/// root — which is the spec literal (`specs/20-surface.md`).
///
/// `pub(crate)` for the same reason as [`discover_skill_dirs`]: drift re-scans
/// the harness through the identical discovery the import used.
pub(crate) fn discover_rule_files(harness: &Path) -> Result<Vec<PathBuf>, ImportError> {
    let rules_root = harness.join(".claude").join("rules");
    if !rules_root.is_dir() {
        return Ok(Vec::new());
    }

    let listing = fs::read_dir(&rules_root).map_err(|source| ImportError::ReadDir {
        path: rules_root.clone(),
        source,
    })?;
    let mut files = Vec::new();
    for entry in listing {
        let entry = entry.map_err(|source| ImportError::ReadDir {
            path: rules_root.clone(),
            source,
        })?;
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
            files.push(path);
        }
    }
    // `read_dir` order is unspecified; sort for deterministic processing.
    files.sort();
    Ok(files)
}

/// Read one source skill and write its surface tree under `<into>/skills/<name>/`,
/// returning the roll-up row for the index.
///
/// `pub(crate)` so `re-add` reuses this single round-trip write path — the typed
/// `meta.toml` via [`Skill::to_meta_document`] plus the byte-faithful body — when
/// it pulls a drifted or added on-disk skill back into the surface, rather than
/// re-implementing the projection (`specs/20-surface.md`, "Drift / apply").
pub(crate) fn import_skill(source_dir: &Path, into: &Path) -> Result<RollupEntry, ImportError> {
    let mut skill = Skill::from_source_dir(source_dir)?;
    let out_dir = into.join("skills").join(&skill.name);

    // Merge, never clobber: the source carries no `[representation]` (it is
    // surface-only authored state), so a re-import or drifted-body `re-add`
    // rebuilds `meta.toml` from source and would wipe the authored
    // `satisfies`/`rationale`. Carry any existing surface representation forward
    // before writing (`specs/20-surface.md`, "three states, never two").
    if let Some(existing) = existing_surface_skill(&out_dir) {
        skill.carry_representation(&existing);
    }

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
        // At import the last-applied fingerprint is the import hash: the source as
        // it stands on disk is exactly what the surface was just derived from.
        last_applied: skill.provenance.import_hash.clone(),
        import_hash: skill.provenance.import_hash,
    })
}

/// Read one source rule and write its surface tree under `<into>/rules/<name>/`,
/// returning the roll-up row for the index.
///
/// Mirrors [`import_skill`] for the rule kind: a format-preserving `meta.toml`
/// header (the optional `paths` + `[provenance]`) and the byte-faithful body as
/// `RULE.md`. A rule carries no companions, so there is nothing else to copy.
///
/// `pub(crate)` for the same reason as [`import_skill`]: `re-add` reuses this exact
/// write path to reconcile a drifted or added on-disk rule into the surface.
pub(crate) fn import_rule(source_file: &Path, into: &Path) -> Result<RollupEntry, ImportError> {
    let mut rule = Rule::from_source_file(source_file)?;
    let out_dir = into.join("rules").join(&rule.name);

    // Merge, never clobber — see `import_skill`. Carry the surface's authored
    // representation forward so a body-only drift re-add never wipes it.
    if let Some(existing) = existing_surface_rule(&out_dir) {
        rule.carry_representation(&existing);
    }

    create_dir_all(&out_dir)?;

    // Typed header via the format-preserving writer — never a lossy re-serialize.
    write_bytes(
        &out_dir.join("meta.toml"),
        rule.to_meta_document().to_string().as_bytes(),
    )?;
    // The surface `RULE.md` is the body alone (no frontmatter), byte-faithful.
    write_bytes(&out_dir.join("RULE.md"), rule.body.as_bytes())?;

    Ok(RollupEntry {
        name: rule.name,
        source_path: rule.provenance.source_path.to_string_lossy().into_owned(),
        // At import the last-applied fingerprint is the import hash (see `import_skill`).
        last_applied: rule.provenance.import_hash.clone(),
        import_hash: rule.provenance.import_hash,
    })
}

/// Load an already-written surface skill from `out_dir` if one is there — the
/// carrier of the authored `[representation]` a re-import / `re-add` must preserve.
/// `None` on a first import (the directory does not exist yet) or if the surface
/// is unreadable, so a missing or malformed prior surface degrades to "nothing to
/// carry" rather than failing the write.
fn existing_surface_skill(out_dir: &Path) -> Option<Skill> {
    if !out_dir.join("meta.toml").is_file() {
        return None;
    }
    Skill::from_surface_dir(out_dir).ok()
}

/// Rule equivalent of [`existing_surface_skill`]: the prior surface rule whose
/// authored representation is carried forward before the header is rewritten.
fn existing_surface_rule(out_dir: &Path) -> Option<Rule> {
    if !out_dir.join("meta.toml").is_file() {
        return None;
    }
    Rule::from_surface_dir(out_dir).ok()
}

/// Discover a custom kind's units under `<harness>/<governs.root>/`: every
/// immediate file whose name matches `governs.glob`. Non-matching files and
/// subdirectories are skipped, and a missing root yields an empty list (a declared
/// kind whose corpus does not exist on this harness). Data-driven discovery — the
/// locus is the kind's own `governs` declaration (`specs/40-composition.md`), never
/// a hardwired path.
fn discover_kind_units(harness: &Path, governs: &Governs) -> Result<Vec<PathBuf>, ImportError> {
    let root = harness.join(&governs.root);
    if !root.is_dir() {
        return Ok(Vec::new());
    }

    let listing = fs::read_dir(&root).map_err(|source| ImportError::ReadDir {
        path: root.clone(),
        source,
    })?;
    let mut files = Vec::new();
    for entry in listing {
        let entry = entry.map_err(|source| ImportError::ReadDir {
            path: root.clone(),
            source,
        })?;
        let path = entry.path();
        if path.is_file()
            && let Some(name) = path.file_name().and_then(|name| name.to_str())
            && glob_matches(&governs.glob, name)
        {
            files.push(path);
        }
    }
    // `read_dir` order is unspecified; sort for deterministic processing.
    files.sort();
    Ok(files)
}

/// Read one discovered custom-kind unit and write its surface tree under
/// `<into>/<governs.root>/<name>/`, returning the roll-up row for the index.
///
/// A custom kind carries no bespoke IR, so the unit is projected generically: a
/// provenance-only `meta.toml` (`[provenance]` — `source_path` + `import_hash`)
/// alongside the byte-faithful *whole* file as `<KIND>.md` (the kind name
/// upper-cased, `SPEC.md` for the `spec` kind, mirroring the built-in `SKILL.md` /
/// `RULE.md` bodies). The whole file is the body — a custom unit's frontmatter, if
/// any, is preserved verbatim in the body rather than dropped, and its extractor
/// (`crate::kind`) reads it at `check` time (`specs/15-kinds.md`).
fn import_custom_unit(
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

    // Typed header via the format-preserving writer — never a lossy re-serialize.
    write_bytes(
        &out_dir.join("meta.toml"),
        provenance_document(source_file, &import_hash)
            .to_string()
            .as_bytes(),
    )?;
    // The surface body is the whole source file, byte-faithful.
    write_bytes(&out_dir.join(body_filename(&kind.name)), body.as_bytes())?;

    Ok(RollupEntry {
        name,
        source_path: source_file.to_string_lossy().into_owned(),
        // At import the last-applied fingerprint is the import hash (see `import_skill`).
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

/// Build a provenance-only surface header for a custom-kind unit: a single
/// `[provenance]` table carrying `source_path` and `import_hash`. A custom kind
/// composes no typed frontmatter header at import (its extractor owns the read
/// side), so the surface header is provenance alone — byte-identical to the
/// built-in prose kinds' provenance table.
fn provenance_document(source_path: &Path, import_hash: &str) -> DocumentMut {
    let mut doc = DocumentMut::new();
    let mut provenance = Table::new();
    provenance["source_path"] = value(source_path.to_string_lossy().into_owned());
    provenance["import_hash"] = value(import_hash.to_string());
    doc["provenance"] = Item::Table(provenance);
    doc
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

    /// Build a harness with two skills under `skills/` and two rules under
    /// `.claude/rules/`; `coordinate` carries a companion markdown file and a
    /// nested script. The two kinds coexist so one import covers both.
    fn write_fixture_harness(root: &Path) {
        let coordinate = root.join("skills").join("coordinate");
        fs::create_dir_all(coordinate.join("scripts")).unwrap();
        fs::write(coordinate.join("SKILL.md"), COORDINATE).unwrap();
        fs::write(coordinate.join("PLAYBOOK.md"), PLAYBOOK).unwrap();
        fs::write(coordinate.join("scripts").join("run.sh"), SCRIPT).unwrap();

        let demo = root.join("skills").join("demo");
        fs::create_dir_all(&demo).unwrap();
        fs::write(demo.join("SKILL.md"), DEMO).unwrap();

        let rules = root.join(".claude").join("rules");
        fs::create_dir_all(&rules).unwrap();
        fs::write(rules.join("rust.md"), RUST_RULE).unwrap();
        fs::write(rules.join("collaboration.md"), COLLAB_RULE).unwrap();
    }

    /// A `temper.toml` declaring `spec` as a custom kind whose `governs` locus is
    /// `specs/*.md` — the data-driven discovery `import` runs in place of the old
    /// hardwired scan (`specs/40-composition.md`). The extractor is the `spec` kind's
    /// read side; `import` only needs the `governs` locus to discover units.
    const SPEC_TEMPER_TOML: &str = "[kind.spec]\n\
governs = { root = \"specs\", glob = \"*.md\" }\n\
\n\
[[kind.spec.extraction]]\n\
primitive = \"line_count\"\n\
\n\
[[kind.spec.extraction]]\n\
primitive = \"headings\"\n";

    /// Add a `specs/` corpus to an existing harness root, plus a `temper.toml`
    /// declaring the `spec` custom kind so discovery finds it: two spec files plus a
    /// non-markdown loose file and a subdirectory, both of which discovery skips.
    fn write_specs(root: &Path) {
        fs::write(root.join("temper.toml"), SPEC_TEMPER_TOML).unwrap();
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

        // Per-skill surface dirs with header + body.
        let coord = into.join("skills").join("coordinate");
        assert!(coord.join("meta.toml").is_file());
        assert!(coord.join("SKILL.md").is_file());
        assert!(into.join("skills").join("demo").join("meta.toml").is_file());
        assert!(into.join("lock.toml").is_file());

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

        // The rule surface mirrors a skill: a `rules/<name>/` dir with a typed
        // header and the body alone under `RULE.md`.
        let rust = into.join("rules").join("rust");
        assert!(rust.join("meta.toml").is_file());
        let body = fs::read_to_string(rust.join("RULE.md")).unwrap();
        assert_eq!(
            body,
            "# Rust conventions\n\nPrefer a clone over a lifetime fight.   \nLast line, no newline."
        );

        // The typed header round-trips back to the source rule (paths + the
        // preserved Cursor key).
        let reloaded = Rule::from_surface_dir(&rust).unwrap();
        assert_eq!(reloaded.name, "rust");
        assert_eq!(
            reloaded.paths.as_deref(),
            Some(&["src/**/*.rs".to_string()][..])
        );
        assert!(reloaded.extra.contains_key("description"));

        // A no-frontmatter rule writes its whole body byte-faithful.
        let collab = into.join("rules").join("collaboration");
        assert_eq!(
            fs::read_to_string(collab.join("RULE.md")).unwrap(),
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
    fn imports_a_bare_harness_that_is_itself_a_skill() {
        // A `<harness>` whose own SKILL.md makes it a skill dir, with no skills/.
        let harness = tmpdir("bare-src");
        fs::write(harness.join("SKILL.md"), DEMO).unwrap();
        let into = tmpdir("bare-into");

        run(&harness, &into).unwrap();

        assert!(into.join("skills").join("demo").join("meta.toml").is_file());
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
        fs::write(harness.join("skills").join("README.md"), "not a skill\n").unwrap();
        fs::create_dir_all(harness.join("skills").join("empty")).unwrap();

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

        // The spec surface mirrors a rule: a `specs/<name>/` dir with a
        // provenance-only header and the whole file alone under `SPEC.md`.
        let surface = into.join("specs").join("20-surface");
        assert!(surface.join("meta.toml").is_file());
        // The body is the *entire* source — a spec has no frontmatter, so the
        // leading `---` is prose and the missing final newline is preserved.
        assert_eq!(
            fs::read_to_string(surface.join("SPEC.md")).unwrap(),
            SURFACE_SPEC
        );

        // The generic custom-unit surface round-trips back through the generic
        // unit loader (`crate::kind::Unit`) — a custom kind carries no bespoke IR,
        // so `import`'s output is read by the same reader `check` uses.
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
        fs::write(
            harness.join("temper.toml"),
            "[kind.adr]\ngoverns = { root = \"docs/adr\", glob = \"*.md\" }\n",
        )
        .unwrap();

        let into = tmpdir("adr-into");
        run(&harness, &into).unwrap();

        // The unit lands at `<into>/<root>/<name>/` with a provenance header and the
        // whole file byte-faithful under `<KIND>.md`.
        let surface = into.join("docs").join("adr").join("0001-surface");
        assert!(surface.join("meta.toml").is_file());
        assert_eq!(
            fs::read_to_string(surface.join("ADR.md")).unwrap(),
            adr_body
        );

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
