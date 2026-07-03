//! The generic `yaml-frontmatter` adapter — one declaration-driven projection for
//! every kind that names it (`specs/architecture/15-kinds.md`, "Decision: the adapter faces are
//! declared — a kind names its projection format").
//!
//! Replaces the retired per-kind IRs (`src/skill.rs`, `src/rule.rs`) with one
//! [`Member`]: `import`/`re-add` split the external artifact per the kind's declared
//! [`unit_shape`](crate::kind::UnitShape) and lift its declared `field` extractors
//! into `[clause.<field>]` header tables; `apply` renders the member document back
//! byte-deterministically; drift compares the declared fields with no per-kind
//! serializer. Built-in and custom kinds ride the same adapter — the built-in/custom
//! split is purely source.
//!
//! Round-trip discipline (`.claude/rules/rust.md`): the markdown body and every
//! companion are byte-faithful — never re-rendered. Only the structured header is
//! written, via `toml_edit`. Unknown frontmatter keys are preserved verbatim; the
//! declared fields lead in declaration order, then the unknown keys sorted, so the
//! projection is deterministic and `import` idempotent. `import_hash` is the SHA-256
//! of the original source bytes, the drift anchor.

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use gray_matter::Pod;
use gray_matter::engine::{Engine, YAML};
use serde_json::{Map as JsonMap, Value as JsonValue};
use toml_edit::{Array, DocumentMut, Value};
use walkdir::WalkDir;

use crate::document::{self, Document, PublishedRequirement, Satisfies};
use crate::kind::{CustomKind, UnitShape};

/// A member projected through the generic frontmatter adapter: the declared and
/// preserved frontmatter fields, a byte-faithful body, the companions that travel
/// with a directory-shaped unit, the authored surface layer, and the provenance lock
/// that anchors drift. The one type both faces (`import` from source, surface reload)
/// produce and drift/apply consume — replacing the per-kind `Skill`/`Rule` IRs.
#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    /// The member id — the surface directory name (`directory` shape) or the file
    /// stem (`file` shape). The emit face's locus follows from it
    /// (`specs/architecture/15-kinds.md`), never a field the member sets.
    pub id: String,
    /// The frontmatter fields in projection order: the kind's declared `field`s
    /// present, in declaration order, then the preserved unknown keys sorted. Both
    /// `[clause.<field>]` emission and the `apply` YAML projection iterate this, so
    /// they agree by construction.
    pub fields: Vec<(String, JsonValue)>,
    /// Markdown after the frontmatter, byte-faithful (trailing bytes intact). For a
    /// source carrying no frontmatter this is the whole file.
    pub body: String,
    /// Sibling files that ship with a directory-shaped unit (a skill's `PLAYBOOK.md`,
    /// `scripts/**`), relative to the source directory and sorted. Empty for a
    /// file-shaped unit and for a surface reload.
    pub companions: Vec<PathBuf>,
    /// The requirements this member opts into filling (`specs/architecture/20-surface.md`, the
    /// `[satisfies.<requirement>]` modules) — authored on the surface, never imported,
    /// so a source parse leaves this empty.
    pub satisfies: Vec<Satisfies>,
    /// The requirements this member publishes (`specs/architecture/10-contracts.md`) — the header's
    /// `[requirement.<name>]` modules, authored on the surface, never imported.
    pub published_requirements: Vec<PublishedRequirement>,
    /// Where the member came from and the hash of its original bytes.
    pub provenance: Provenance,
}

/// The import lock for a member: its origin path and a content hash of the original
/// source bytes. `import_hash` drives drift detection.
#[derive(Debug, Clone, PartialEq)]
pub struct Provenance {
    /// Absolute or workspace-relative path to the source file.
    pub source_path: PathBuf,
    /// Lowercase hex SHA-256 of the original source bytes.
    pub import_hash: String,
}

/// Errors raised while reading or projecting a [`Member`]. Hard failures (missing
/// file, malformed surface) — distinct from a lint `Diagnostic`, which the engine
/// collects rather than throws.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum FrontmatterError {
    /// A source or surface file could not be read.
    #[error("failed to read {path}")]
    #[diagnostic(code(temper::frontmatter::io))]
    Io {
        /// The path that failed to read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// A source file is not valid UTF-8, so its body cannot be modelled as text.
    #[error("{path} is not valid UTF-8")]
    #[diagnostic(code(temper::frontmatter::not_utf8))]
    NotUtf8 {
        /// The offending file.
        path: PathBuf,
        /// The decode error.
        #[source]
        source: std::string::FromUtf8Error,
    },

    /// The source path yields no id — no file stem (`file` shape) or no parent
    /// directory name (`directory` shape) to name the member by.
    #[error("{path} has no {shape}-shape id")]
    #[diagnostic(code(temper::frontmatter::no_id))]
    NoId {
        /// The path missing a usable id component.
        path: PathBuf,
        /// The unit shape whose id component was absent (`file` / `directory`).
        shape: &'static str,
    },

    /// The surface member document is not a well-formed `+++`-fenced document.
    #[error("{path}: {source}")]
    #[diagnostic(code(temper::frontmatter::bad_document))]
    Document {
        /// The surface document that failed to parse.
        path: PathBuf,
        /// The underlying fenced-document parse error.
        #[source]
        source: crate::document::DocumentError,
    },

    /// A surface member document is missing a required part — its `[provenance]`
    /// module. A surface missing what the emit face always writes is malformed.
    #[error("{path}: surface is missing required field `{field}`")]
    #[diagnostic(code(temper::frontmatter::missing_field))]
    MissingField {
        /// The surface whose part is absent.
        path: PathBuf,
        /// The required field that was absent.
        field: &'static str,
    },
}

impl Member {
    /// Import a member from its source file, driven by the kind's declared unit shape
    /// and `field` extractors: split the YAML frontmatter, derive the id (file stem or
    /// parent directory name), lift the declared fields then the preserved unknown keys
    /// into the projection order, scan companions (directory shape), and hash the
    /// original bytes for provenance. The authored surface layer (`satisfies`/
    /// published requirements) is never in the source, so it starts empty.
    ///
    /// The body is taken byte-faithfully from after the closing frontmatter delimiter;
    /// a source with no frontmatter parses to no fields and a whole-file body — the
    /// permissive read the built-in adapters shared (`specs/architecture/15-kinds.md`).
    ///
    /// # Errors
    ///
    /// Returns a [`FrontmatterError`] if the source cannot be read, is not UTF-8, or
    /// yields no id for its declared shape.
    pub fn from_source(kind: &CustomKind, source_file: &Path) -> Result<Self, FrontmatterError> {
        // No scan-root context: a file-shaped unit folds no placement (its immediate
        // parent is the base, so the relative placement is empty — the bare stem). The
        // import driver, which knows the `governs` root, calls
        // [`from_source_rooted`](Member::from_source_rooted) so a nested unit folds.
        let base = source_file.parent().unwrap_or(source_file);
        Self::from_source_rooted(kind, source_file, base)
    }

    /// Import a member as [`from_source`](Member::from_source), but resolve a
    /// file-shaped unit's id against the `governs`-root directory `base`: a unit nested
    /// below `base` folds its placement into the id (`specs/architecture/40-composition.md`,
    /// "Registering a custom kind" — file placement is an extraction primitive), so two
    /// nested same-named files (`sub/AGENTS.md` and a root `AGENTS.md`) carry distinct
    /// surface ids rather than collapsing to one bare stem. A directory-shaped unit is
    /// unaffected — its id is its own directory name, already distinct per member.
    ///
    /// # Errors
    ///
    /// As [`from_source`](Member::from_source).
    pub fn from_source_rooted(
        kind: &CustomKind,
        source_file: &Path,
        base: &Path,
    ) -> Result<Self, FrontmatterError> {
        let bytes = fs::read(source_file).map_err(|source| FrontmatterError::Io {
            path: source_file.to_path_buf(),
            source,
        })?;
        let import_hash = crate::hash::sha256_hex(&bytes);
        let raw = String::from_utf8(bytes).map_err(|source| FrontmatterError::NotUtf8 {
            path: source_file.to_path_buf(),
            source,
        })?;

        let (id, companions) = match kind.unit_shape {
            Some(UnitShape::Directory) => {
                let dir = source_file
                    .parent()
                    .filter(|dir| !dir.as_os_str().is_empty())
                    .ok_or_else(|| FrontmatterError::NoId {
                        path: source_file.to_path_buf(),
                        shape: "directory",
                    })?;
                let id = dir
                    .file_name()
                    .and_then(OsStr::to_str)
                    .ok_or_else(|| FrontmatterError::NoId {
                        path: source_file.to_path_buf(),
                        shape: "directory",
                    })?
                    .to_string();
                let name = source_file.file_name().unwrap_or(OsStr::new(""));
                (id, scan_companions(dir, name)?)
            }
            // `file` shape, or an undeclared shape defaulting to a lone file.
            Some(UnitShape::File) | None => (fold_file_id(base, source_file)?, Vec::new()),
        };

        let parsed = parse_frontmatter(&raw);
        let fields = order_fields(&kind.declared_fields(), parsed);

        Ok(Self {
            id,
            fields,
            body: split_frontmatter(&raw).1.to_string(),
            companions,
            satisfies: Vec::new(),
            published_requirements: Vec::new(),
            provenance: Provenance {
                source_path: source_file.to_path_buf(),
                import_hash,
            },
        })
    }

    /// Reload a member from its written surface member document `<dir>/<member_doc>`:
    /// parse the `+++`-fenced header, read the `[clause.<field>]` fields (in document
    /// order — the canonical order the emit face wrote), the authored `[satisfies.*]` /
    /// `[requirement.*]` modules, and `[provenance]`; the body is
    /// everything below the header. The id is the surface directory name.
    ///
    /// The inverse of [`Member::to_document`] over the same file: `import_hash` is read
    /// back from the provenance module, not recomputed.
    ///
    /// # Errors
    ///
    /// Returns a [`FrontmatterError`] if the surface has no directory name, the
    /// document is unreadable or malformed, or it carries no `[provenance]`.
    pub fn from_surface(dir: &Path, member_doc: &str) -> Result<Self, FrontmatterError> {
        let id = dir
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or_else(|| FrontmatterError::NoId {
                path: dir.to_path_buf(),
                shape: "directory",
            })?
            .to_string();

        let doc_path = dir.join(member_doc);
        let raw = fs::read_to_string(&doc_path).map_err(|source| FrontmatterError::Io {
            path: doc_path.clone(),
            source,
        })?;
        let doc = Document::parse(&raw).map_err(|source| FrontmatterError::Document {
            path: doc_path.clone(),
            source,
        })?;
        let header = doc.header();

        let fields = document::clauses(header)
            .into_iter()
            .filter_map(|(field, item)| document::item_to_json(item).map(|json| (field, json)))
            .collect();

        let (source_path, import_hash) =
            document::provenance(header).ok_or(FrontmatterError::MissingField {
                path: doc_path.clone(),
                field: "provenance",
            })?;

        let published_requirements =
            document::requirements(header).map_err(|source| FrontmatterError::Document {
                path: doc_path.clone(),
                source,
            })?;

        Ok(Self {
            id,
            fields,
            body: doc.body().to_string(),
            companions: Vec::new(),
            satisfies: document::satisfies(header),
            published_requirements,
            provenance: Provenance {
                source_path: PathBuf::from(source_path),
                import_hash,
            },
        })
    }

    /// The frontmatter value at `key`, if the member carries it — the generic accessor
    /// replacing the retired IRs' typed fields.
    #[must_use]
    pub fn field(&self, key: &str) -> Option<&JsonValue> {
        self.fields
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value)
    }

    /// Whether the member carries a frontmatter field/key by this name.
    #[must_use]
    pub fn has_field(&self, key: &str) -> bool {
        self.fields.iter().any(|(name, _)| name == key)
    }

    /// Carry the authored surface layer (`satisfies` / published
    /// requirements) from an existing surface member forward onto this freshly-parsed
    /// source member — **merge rather than clobber** (`specs/architecture/20-surface.md`, "three
    /// states, never two"): a re-import or drifted-body `re-add` rebuilds the document
    /// from source, which never carries the authored layer, so the caller loads the
    /// existing surface and carries it before writing.
    pub fn carry_representation(&mut self, existing: &Member) {
        self.satisfies = existing.satisfies.clone();
        self.published_requirements = existing.published_requirements.clone();
    }

    /// Project the member to its one authored document: a `+++`-fenced header of clause
    /// modules over the byte-faithful body (`specs/architecture/20-surface.md`, "The member
    /// document"). One `[clause.<field>]` module per field in projection order, then the
    /// authored `[satisfies.*]` / `[requirement.*]` modules, then the
    /// generated `[provenance]` last.
    #[must_use]
    pub fn to_document(&self) -> Document {
        let mut header = DocumentMut::new();
        for (key, json) in &self.fields {
            if let Some(val) = json_to_toml_value(json) {
                document::add_clause(&mut header, key, val);
            }
        }
        for satisfies in &self.satisfies {
            document::add_satisfies(&mut header, satisfies);
        }
        for requirement in &self.published_requirements {
            document::add_requirement(&mut header, requirement);
        }
        document::add_provenance(
            &mut header,
            &self.provenance.source_path.to_string_lossy(),
            &self.provenance.import_hash,
        );
        Document::new(header, self.body.clone())
    }
}

/// Order a parsed frontmatter map into projection order: the kind's declared `field`s
/// present, in declaration order, then every remaining (unknown) key sorted. The
/// single ordering both faces read, so a projection is deterministic and idempotent.
fn order_fields(
    declared: &[&str],
    mut parsed: JsonMap<String, JsonValue>,
) -> Vec<(String, JsonValue)> {
    let mut fields = Vec::with_capacity(parsed.len());
    for key in declared {
        if let Some(value) = parsed.remove(*key) {
            fields.push(((*key).to_string(), value));
        }
    }
    // The leftover unknown keys, already key-sorted (`serde_json::Map` is a `BTreeMap`).
    for (key, value) in parsed {
        fields.push((key, value));
    }
    fields
}

/// Parse a source file's YAML frontmatter into a JSON map, dropping nulls (TOML has no
/// null) so the surface stays representable and the round-trip symmetric. A file with
/// no frontmatter block yields an empty map.
fn parse_frontmatter(raw: &str) -> JsonMap<String, JsonValue> {
    let Some(matter) = split_frontmatter(raw).0 else {
        return JsonMap::new();
    };
    let fields = match YAML::parse(matter.trim()) {
        Pod::Hash(hash) => hash,
        // Frontmatter present but not a mapping (a bare scalar/list): no keys to read.
        _ => return JsonMap::new(),
    };
    let mut out = JsonMap::new();
    for (key, pod) in fields {
        if matches!(pod, Pod::Null) {
            continue;
        }
        out.insert(key, pod.into());
    }
    out
}

/// Split a source file into its YAML frontmatter block and a byte-faithful body.
/// Mirrors `gray_matter`'s `---` delimiter detection but, unlike its `content` field
/// (which trims surrounding whitespace), returns the body exactly as it appears after
/// the closing delimiter line. Returns `(None, raw)` when there is no leading
/// frontmatter block to strip.
fn split_frontmatter(raw: &str) -> (Option<&str>, &str) {
    let Some((first, rest)) = raw.split_once('\n') else {
        return (None, raw);
    };
    if first.trim_end() != "---" {
        return (None, raw);
    }

    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        let content = line.strip_suffix('\n').unwrap_or(line);
        if content.trim_end() == "---" {
            let matter = &rest[..offset];
            let body = &rest[offset + line.len()..];
            return (Some(matter), body);
        }
        offset += line.len();
    }

    // Opening delimiter but no close — not a frontmatter block.
    (None, raw)
}

/// Walk a directory-shaped unit's source directory and collect its companion files —
/// every file except the member document `member` itself — as paths relative to `dir`,
/// sorted for determinism.
///
/// `pub(crate)` so the whole-file custom import path (`src/import.rs`) copies a
/// `Directory`-shaped frontmatterless unit's companions through the identical scan the
/// frontmatter face uses, rather than a second implementation that could drift.
pub(crate) fn scan_companions(
    dir: &Path,
    member: &OsStr,
) -> Result<Vec<PathBuf>, FrontmatterError> {
    let mut companions = Vec::new();
    for entry in WalkDir::new(dir).min_depth(1).sort_by_file_name() {
        let entry = entry.map_err(|err| FrontmatterError::Io {
            path: err
                .path()
                .map(Path::to_path_buf)
                .unwrap_or_else(|| dir.to_path_buf()),
            source: err
                .into_io_error()
                .unwrap_or_else(|| std::io::Error::other("directory walk failed")),
        })?;
        if !entry.file_type().is_file() {
            continue;
        }
        if entry.file_name() == member {
            continue;
        }
        let relative = entry
            .path()
            .strip_prefix(dir)
            .map(Path::to_path_buf)
            .unwrap_or_else(|_| entry.path().to_path_buf());
        companions.push(relative);
    }
    companions.sort();
    Ok(companions)
}

/// Derive a **file-shaped** unit's surface id, folding the directory placement below
/// the `governs`-root directory `base` into it (`specs/architecture/40-composition.md`,
/// "Registering a custom kind" — file placement is an extraction primitive). A unit
/// directly under `base` keeps its bare filename stem — the common flat case, unchanged
/// — while a nested one prefixes its placement path, the placement components and the
/// stem joined with `-` into one surface-directory component. This is what stops a
/// nested nearest-wins hierarchy (agents.md / `CLAUDE.md` memory nesting) from
/// collapsing two same-named files at different depths onto one clobbered surface entry.
///
/// Both adapter faces share it — the frontmatter face
/// ([`Member::from_source_rooted`]) and the whole-file face
/// (`import::wholefile_id`) — so a nested unit is named identically whichever path
/// imports it. A source not under `base` (a caller with no root context) folds no
/// placement, degrading to the bare stem rather than erroring.
///
/// # Errors
///
/// Returns [`FrontmatterError::NoId`] if the source has no filename stem, or a
/// placement component that is not valid UTF-8 — the same failure a bare stem raises.
pub(crate) fn fold_file_id(base: &Path, source_file: &Path) -> Result<String, FrontmatterError> {
    let no_id = || FrontmatterError::NoId {
        path: source_file.to_path_buf(),
        shape: "file",
    };
    let stem = source_file
        .file_stem()
        .and_then(OsStr::to_str)
        .ok_or_else(no_id)?;

    // Placement = the directories between the scan root and the file. `.` components
    // (a `base` that named the current dir) carry no placement and are skipped, so the
    // flat case reduces to the bare stem.
    let placement = source_file
        .strip_prefix(base)
        .ok()
        .and_then(Path::parent)
        .filter(|dirs| !dirs.as_os_str().is_empty());
    let Some(dirs) = placement else {
        return Ok(stem.to_string());
    };

    let mut parts = Vec::new();
    for component in dirs.iter() {
        let part = component.to_str().ok_or_else(no_id)?;
        if part == "." {
            continue;
        }
        parts.push(part);
    }
    parts.push(stem);
    Ok(parts.join("-"))
}

/// Convert a JSON value to a `toml_edit` value, rendering objects as inline tables.
/// Returns `None` for JSON null (unrepresentable in TOML).
fn json_to_toml_value(json: &JsonValue) -> Option<Value> {
    Some(match json {
        JsonValue::Null => return None,
        JsonValue::Bool(b) => Value::from(*b),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::from(i)
            } else {
                Value::from(n.as_f64()?)
            }
        }
        JsonValue::String(s) => Value::from(s.clone()),
        JsonValue::Array(items) => {
            let mut array = Array::new();
            for item in items {
                if let Some(val) = json_to_toml_value(item) {
                    array.push(val);
                }
            }
            Value::Array(array)
        }
        JsonValue::Object(map) => {
            let mut inline = toml_edit::InlineTable::new();
            for (key, val) in map {
                if let Some(val) = json_to_toml_value(val) {
                    inline.insert(key, val);
                }
            }
            Value::InlineTable(inline)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtin_kind;
    use std::sync::atomic::{AtomicU32, Ordering};

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    /// A fresh, empty temp directory unique to this test run.
    fn tmpdir(label: &str) -> PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "frontmatter-{}-{}-{}",
            std::process::id(),
            id,
            label
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn skill_kind() -> CustomKind {
        builtin_kind::definition("skill").unwrap().unwrap()
    }

    fn rule_kind() -> CustomKind {
        builtin_kind::definition("rule").unwrap().unwrap()
    }

    const SKILL: &str = "---\n\
name: demo\n\
description: Use when demonstrating the generic frontmatter adapter.\n\
license: \"MIT\"\n\
allowed-tools: [\"Bash\", \"Read\"]\n\
priority: 7\n\
---\n\
# Demo\n\
\n\
Body text, trailing space.   \n\
Last line, no newline.";

    #[test]
    fn directory_shape_ids_from_the_dir_and_orders_declared_then_unknown() {
        let dir = tmpdir("dir-shape").join("demo");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("SKILL.md"), SKILL).unwrap();

        let member = Member::from_source(&skill_kind(), &dir.join("SKILL.md")).unwrap();

        // id is the directory name, not the frontmatter field.
        assert_eq!(member.id, "demo");
        // The declared fields lead in declaration order; the unknown keys follow sorted.
        let order: Vec<&str> = member.fields.iter().map(|(k, _)| k.as_str()).collect();
        assert_eq!(
            order,
            vec![
                "name",
                "description",
                "license",
                "allowed-tools",
                "priority"
            ]
        );
        assert_eq!(
            member.field("license").and_then(JsonValue::as_str),
            Some("MIT")
        );
        // The body rides below the frontmatter byte-faithfully.
        assert_eq!(
            member.body,
            "# Demo\n\nBody text, trailing space.   \nLast line, no newline."
        );
    }

    #[test]
    fn file_shape_ids_from_the_stem_and_a_no_frontmatter_source_is_bodied_whole() {
        let dir = tmpdir("file-shape");
        fs::write(dir.join("collaboration.md"), "# Collab\n\nPushback.\n").unwrap();

        let member = Member::from_source(&rule_kind(), &dir.join("collaboration.md")).unwrap();
        assert_eq!(member.id, "collaboration");
        assert!(member.fields.is_empty());
        assert_eq!(member.body, "# Collab\n\nPushback.\n");
    }

    #[test]
    fn source_round_trips_through_the_surface_document() {
        let dir = tmpdir("rt").join("demo");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("SKILL.md"), SKILL).unwrap();
        let original = Member::from_source(&skill_kind(), &dir.join("SKILL.md")).unwrap();

        let surface = tmpdir("rt-surface").join("demo");
        fs::create_dir_all(&surface).unwrap();
        fs::write(surface.join("SKILL.md"), original.to_document().emit()).unwrap();
        let reloaded = Member::from_surface(&surface, "SKILL.md").unwrap();

        // The fields, body, and provenance survive the round trip; the source member's
        // companions are not part of the surface reload.
        assert_eq!(original.fields, reloaded.fields);
        assert_eq!(original.body, reloaded.body);
        assert_eq!(original.provenance, reloaded.provenance);
    }

    #[test]
    fn to_document_is_deterministic_and_emits_clause_modules() {
        let dir = tmpdir("det").join("demo");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("SKILL.md"), SKILL).unwrap();
        let member = Member::from_source(&skill_kind(), &dir.join("SKILL.md")).unwrap();

        let once = member.to_document().emit();
        let twice = member.to_document().emit();
        assert_eq!(once, twice, "rendering must be deterministic");
        assert!(once.contains("[clause.name]\nvalue = \"demo\""));
        assert!(once.contains("[clause.allowed-tools]\nvalue = [\"Bash\", \"Read\"]"));
        assert!(once.contains("[provenance]"));
    }

    #[test]
    fn companions_scan_relative_and_sorted_excluding_the_member() {
        let dir = tmpdir("companions").join("coordinate");
        fs::create_dir_all(dir.join("scripts")).unwrap();
        fs::write(dir.join("SKILL.md"), SKILL).unwrap();
        fs::write(dir.join("PLAYBOOK.md"), "playbook\n").unwrap();
        fs::write(dir.join("scripts").join("run.sh"), "echo hi\n").unwrap();

        let member = Member::from_source(&skill_kind(), &dir.join("SKILL.md")).unwrap();
        assert_eq!(
            member.companions,
            vec![
                PathBuf::from("PLAYBOOK.md"),
                PathBuf::from("scripts").join("run.sh"),
            ]
        );
    }

    #[test]
    fn a_paths_rule_lifts_the_declared_field_and_preserves_unknown_keys() {
        let dir = tmpdir("paths-rule");
        fs::write(
            dir.join("rust.md"),
            "---\npaths:\n  - \"src/**/*.rs\"\ndescription: A Cursor key, preserved.\n---\n# Rust\n\nBody.\n",
        )
        .unwrap();

        let member = Member::from_source(&rule_kind(), &dir.join("rust.md")).unwrap();
        assert_eq!(member.id, "rust");
        assert_eq!(
            member.field("paths"),
            Some(&serde_json::json!(["src/**/*.rs"]))
        );
        assert!(member.has_field("description"));
    }
}
