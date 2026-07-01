//! The `Skill` artifact — the typed IR for `~/.claude/skills/<name>/SKILL.md`.
//!
//! Models the skill instance of the IR in `specs/20-surface.md` ("The member
//! document — the surface language"). A skill is read from source with
//! [`Skill::from_source_dir`] (split YAML frontmatter, scan companions, hash the
//! original bytes for provenance), projected to its **one authored document** with
//! [`Skill::to_document`], and reloaded from that surface with [`Skill::from_dir`].
//!
//! The member is a single document (`specs/20-surface.md`, "Decision: the member is
//! one document in the surface language"): a `+++`-fenced TOML header over the body,
//! written in place of the retired `meta.toml` + body pair. The header is
//! **clause-structured** — one `[clause.<field>]` module per structured field
//! (typed fields *and* the verbatim-preserved unknown frontmatter keys), the
//! authored `[satisfies.<requirement>]` and `[edge.<target>]` modules, and the
//! generated `[provenance]`.
//!
//! Round-trip discipline (`.claude/rules/rust.md`): the markdown body and every
//! companion are byte-faithful — never re-rendered. Only the structured header is
//! written, via `toml_edit`. Unknown frontmatter keys are preserved in it, never
//! dropped. `import_hash` is the SHA-256 of the original `SKILL.md` bytes; it is the
//! drift anchor and is computed at import so the provenance lock is complete even
//! before write-back exists.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use gray_matter::Pod;
use gray_matter::engine::{Engine, YAML};
use serde_json::{Map as JsonMap, Value as JsonValue};
use toml_edit::{DocumentMut, Item, Value};

use crate::document::{self, Document, EdgeClause, Satisfies};
use walkdir::WalkDir;

/// A Claude Code skill: typed frontmatter, a byte-faithful body, the companion
/// files that travel with it, and the provenance lock that anchors drift.
#[derive(Debug, Clone, PartialEq)]
pub struct Skill {
    /// Skill name from frontmatter. A later lint asserts it equals the dir name.
    pub name: String,
    /// The trigger text from frontmatter — when the model should reach for this.
    pub description: String,
    /// Optional `version` frontmatter.
    pub version: Option<String>,
    /// Optional `license` frontmatter.
    pub license: Option<String>,
    /// Markdown after the frontmatter, byte-faithful (trailing bytes intact).
    pub body: String,
    /// The requirements this artifact opts into filling (`specs/20-surface.md`, the
    /// `[satisfies.<requirement>]` clause modules) — the coverage check's opt-in
    /// bindings, each carrying its optional authored `rationale`. **Authored** on
    /// the surface, not imported: the source `SKILL.md` carries no such field, so
    /// `from_source_dir` leaves this empty. It lives in the header's `[satisfies.*]`
    /// modules, kept out of the frontmatter `extra` the contract engine ranges over.
    pub satisfies: Vec<Satisfies>,
    /// The declared references/relationships to other members
    /// (`specs/45-governance.md`, "an edge is a declared field on the surface") —
    /// the header's `[edge.<target>]` modules. **Authored** on the surface, not
    /// imported (`from_source_dir` leaves this empty); the graph's source, never
    /// grepped from prose.
    pub edges: Vec<EdgeClause>,
    /// Sibling files that ship with the skill (e.g. `PLAYBOOK.md`, `scripts/**`),
    /// as paths relative to the skill directory, sorted for determinism.
    pub companions: Vec<PathBuf>,
    /// Unknown frontmatter keys, preserved verbatim so the surface never drops
    /// authoring intent. Sorted by key (`serde_json::Map` is a `BTreeMap`), which
    /// makes `to_document` deterministic and `import` idempotent.
    pub extra: JsonMap<String, JsonValue>,
    /// Where the skill came from and the hash of its original bytes.
    pub provenance: Provenance,
}

/// The import lock for a skill: its origin path and a content hash of the
/// original `SKILL.md` bytes. `import_hash` drives future drift detection.
#[derive(Debug, Clone, PartialEq)]
pub struct Provenance {
    /// Absolute or workspace-relative path to the source `SKILL.md`.
    pub source_path: PathBuf,
    /// Lowercase hex SHA-256 of the original `SKILL.md` bytes.
    pub import_hash: String,
}

/// Errors raised while reading or projecting a [`Skill`]. These are hard
/// failures (missing file, malformed surface) — distinct from a lint
/// `Diagnostic`, which is a finding the engine collects rather than an error.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum SkillError {
    /// A file under the skill directory could not be read.
    #[error("failed to read {path}")]
    #[diagnostic(code(temper::skill::io))]
    Io {
        /// The path that failed to read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// `SKILL.md` is not valid UTF-8, so its body cannot be modelled as text.
    #[error("{path} is not valid UTF-8")]
    #[diagnostic(code(temper::skill::not_utf8))]
    NotUtf8 {
        /// The offending file.
        path: PathBuf,
        /// The decode error.
        #[source]
        source: std::string::FromUtf8Error,
    },

    /// The source `SKILL.md` has no leading `---` delimited YAML frontmatter.
    #[error("{path} has no YAML frontmatter")]
    #[diagnostic(
        code(temper::skill::no_frontmatter),
        help("a skill must begin with a `---` delimited YAML block")
    )]
    NoFrontmatter {
        /// The file missing frontmatter.
        path: PathBuf,
    },

    /// A required frontmatter field is absent or not a scalar value.
    #[error("{path}: frontmatter is missing required field `{field}`")]
    #[diagnostic(code(temper::skill::missing_field))]
    MissingField {
        /// The file whose header is incomplete.
        path: PathBuf,
        /// The required key that was absent.
        field: &'static str,
    },

    /// The surface `SKILL.md` is not a well-formed `+++`-fenced document (missing or
    /// unterminated fence, or a malformed TOML header).
    #[error("{path}: {source}")]
    #[diagnostic(code(temper::skill::bad_document))]
    Document {
        /// The surface document that failed to parse.
        path: PathBuf,
        /// The underlying fenced-document parse error.
        #[source]
        source: crate::document::DocumentError,
    },
}

impl Skill {
    /// Parse a skill from its source directory: read `<dir>/SKILL.md`, split the
    /// YAML frontmatter, scan companions, and record provenance.
    ///
    /// The body is taken byte-faithfully from after the closing delimiter (unlike
    /// `gray_matter`'s `content`, which trims surrounding whitespace). The
    /// `import_hash` is the SHA-256 of the original file bytes.
    pub fn from_source_dir(dir: &Path) -> Result<Self, SkillError> {
        let source_path = dir.join("SKILL.md");
        let bytes = fs::read(&source_path).map_err(|source| SkillError::Io {
            path: source_path.clone(),
            source,
        })?;
        let import_hash = crate::hash::sha256_hex(&bytes);
        let raw = String::from_utf8(bytes).map_err(|source| SkillError::NotUtf8 {
            path: source_path.clone(),
            source,
        })?;

        let (matter, body) = split_frontmatter(&raw);
        let matter = matter.ok_or_else(|| SkillError::NoFrontmatter {
            path: source_path.clone(),
        })?;

        // Use gray_matter's YAML engine on the split block — a single source of
        // truth for the split keeps the frontmatter and body perfectly in sync.
        let mut fields = match YAML::parse(matter.trim()) {
            Pod::Hash(hash) => hash,
            // Frontmatter present but not a mapping (e.g. a bare scalar/list):
            // there is no `name`/`description` to read, so treat it as absent.
            _ => HashMap::new(),
        };

        let name = take_required_scalar(&mut fields, "name", &source_path)?;
        let description = take_required_scalar(&mut fields, "description", &source_path)?;
        let version = take_optional_scalar(&mut fields, "version");
        let license = take_optional_scalar(&mut fields, "license");
        let extra = pod_hash_to_json(fields);

        Ok(Self {
            name,
            description,
            version,
            license,
            body: body.to_string(),
            // `satisfies`/`edges` are authored on the surface, never present in the
            // source — so import leaves them empty and an unauthored skill's document
            // carries no `[satisfies.*]`/`[edge.*]` modules (import idempotence).
            satisfies: Vec::new(),
            edges: Vec::new(),
            companions: scan_companions(dir)?,
            extra,
            provenance: Provenance {
                source_path,
                import_hash,
            },
        })
    }

    /// Reload a skill from its **one authored document** `<dir>/SKILL.md`: parse the
    /// `+++`-fenced header, read the `[clause.<field>]` modules into the typed fields
    /// (unknown clauses preserved in `extra`), the `[satisfies.*]` / `[edge.*]`
    /// modules, and `[provenance]`; the body is everything below the header.
    ///
    /// The inverse of [`Skill::to_document`] over the same file: `import_hash` is
    /// read back from the provenance module, not recomputed (the surface body differs
    /// from the original, which carried frontmatter).
    pub fn from_dir(dir: &Path) -> Result<Self, SkillError> {
        let doc_path = dir.join("SKILL.md");
        let raw = fs::read_to_string(&doc_path).map_err(|source| SkillError::Io {
            path: doc_path.clone(),
            source,
        })?;
        let doc = Document::parse(&raw).map_err(|source| SkillError::Document {
            path: doc_path.clone(),
            source,
        })?;
        let header = doc.header();

        // The `[clause.<field>]` modules: the known fields typed, the rest preserved
        // verbatim in `extra` — the same catch-all the contract engine ranges over.
        let mut name = None;
        let mut description = None;
        let mut version = None;
        let mut license = None;
        let mut extra = JsonMap::new();
        for (field, val) in document::clauses(header) {
            match field.as_str() {
                "name" => name = val.as_str().map(str::to_string),
                "description" => description = val.as_str().map(str::to_string),
                "version" => version = val.as_str().map(str::to_string),
                "license" => license = val.as_str().map(str::to_string),
                _ => {
                    if let Some(json) = toml_item_to_json(val) {
                        extra.insert(field, json);
                    }
                }
            }
        }
        let name = name.ok_or(SkillError::MissingField {
            path: doc_path.clone(),
            field: "name",
        })?;
        let description = description.ok_or(SkillError::MissingField {
            path: doc_path.clone(),
            field: "description",
        })?;

        let (source_path, import_hash) =
            document::provenance(header).ok_or(SkillError::MissingField {
                path: doc_path.clone(),
                field: "provenance",
            })?;

        Ok(Self {
            name,
            description,
            version,
            license,
            body: doc.body().to_string(),
            satisfies: document::satisfies(header),
            edges: document::edges(header),
            companions: scan_companions(dir)?,
            extra,
            provenance: Provenance {
                source_path: PathBuf::from(source_path),
                import_hash,
            },
        })
    }

    /// Carry the authored surface layer (`satisfies` + `edges`) from an
    /// already-written surface artifact forward into this freshly-parsed source
    /// skill.
    ///
    /// The source `SKILL.md` never carries the authored clauses — they are
    /// surface-only state (`specs/20-surface.md`, "importing a member is recognizing
    /// it"; the authored `[satisfies.*]`/`[edge.*]` accrue afterward). So a re-import
    /// or a drifted-body `re-add`, which rebuilds the document from source, would
    /// otherwise clobber them. This is the authored layer's half of the three-state
    /// law: **merge rather than clobber** — the caller loads the existing surface,
    /// carries its authored clauses onto the re-parsed source, then writes, so a body
    /// edit on disk never erases the authored `satisfies`/`edges`.
    pub fn carry_representation(&mut self, existing: &Skill) {
        self.satisfies = existing.satisfies.clone();
        self.edges = existing.edges.clone();
    }

    /// Project the skill to its **one authored document**: a `+++`-fenced header of
    /// clause modules over the byte-faithful body (`specs/20-surface.md`, "The member
    /// document"). The header carries a `[clause.<field>]` module per structured
    /// field (the typed fields in canonical order, then every unknown frontmatter key
    /// in sorted `extra` order), the authored `[satisfies.*]` / `[edge.*]` modules,
    /// then the generated `[provenance]` last.
    pub fn to_document(&self) -> Document {
        let mut header = DocumentMut::new();
        document::add_clause(&mut header, "name", Value::from(self.name.clone()));
        document::add_clause(
            &mut header,
            "description",
            Value::from(self.description.clone()),
        );
        if let Some(version) = &self.version {
            document::add_clause(&mut header, "version", Value::from(version.clone()));
        }
        if let Some(license) = &self.license {
            document::add_clause(&mut header, "license", Value::from(license.clone()));
        }
        for (key, json) in &self.extra {
            if let Some(val) = json_to_toml_value(json) {
                document::add_clause(&mut header, key, val);
            }
        }

        for satisfies in &self.satisfies {
            document::add_satisfies(&mut header, satisfies);
        }
        for edge in &self.edges {
            document::add_edge(&mut header, edge);
        }

        document::add_provenance(
            &mut header,
            &self.provenance.source_path.to_string_lossy(),
            &self.provenance.import_hash,
        );

        Document::new(header, self.body.clone())
    }
}

/// Split a source `SKILL.md` into its YAML frontmatter block and a byte-faithful
/// body. Mirrors `gray_matter`'s `---` delimiter detection but, unlike its
/// `content` field (which trims surrounding whitespace), returns the body
/// exactly as it appears after the closing delimiter line. Returns `(None, raw)`
/// when there is no leading frontmatter block to strip.
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

/// Remove and stringify a required scalar frontmatter field, erroring if it is
/// absent or not a scalar.
fn take_required_scalar(
    fields: &mut HashMap<String, Pod>,
    field: &'static str,
    path: &Path,
) -> Result<String, SkillError> {
    fields
        .remove(field)
        .as_ref()
        .and_then(pod_scalar_to_string)
        .ok_or(SkillError::MissingField {
            path: path.to_path_buf(),
            field,
        })
}

/// Remove and stringify an optional scalar frontmatter field, if present.
fn take_optional_scalar(fields: &mut HashMap<String, Pod>, field: &str) -> Option<String> {
    fields.remove(field).as_ref().and_then(pod_scalar_to_string)
}

/// Coerce a scalar [`Pod`] to its string form. Non-scalars (arrays, hashes,
/// null) yield `None`.
fn pod_scalar_to_string(pod: &Pod) -> Option<String> {
    match pod {
        Pod::String(value) => Some(value.clone()),
        Pod::Integer(value) => Some(value.to_string()),
        Pod::Float(value) => Some(value.to_string()),
        Pod::Boolean(value) => Some(value.to_string()),
        Pod::Null | Pod::Array(_) | Pod::Hash(_) => None,
    }
}

/// Convert the leftover (unknown) frontmatter keys into a JSON map, dropping
/// nulls (TOML has no null) so the surface stays representable and the
/// source/surface round-trip is symmetric.
fn pod_hash_to_json(fields: HashMap<String, Pod>) -> JsonMap<String, JsonValue> {
    let mut out = JsonMap::new();
    for (key, pod) in fields {
        if matches!(pod, Pod::Null) {
            continue;
        }
        out.insert(key, pod.into());
    }
    out
}

/// Walk a skill directory and collect its companion files — every file except the
/// `SKILL.md` document itself — as paths relative to `dir`, sorted.
fn scan_companions(dir: &Path) -> Result<Vec<PathBuf>, SkillError> {
    let mut companions = Vec::new();
    for entry in WalkDir::new(dir).min_depth(1).sort_by_file_name() {
        let entry = entry.map_err(|err| SkillError::Io {
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
        let name = entry.file_name();
        if name == "SKILL.md" {
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

/// Convert a JSON value to a `toml_edit` value, rendering objects as inline
/// tables. Returns `None` for JSON null (unrepresentable in TOML).
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
            let mut array = toml_edit::Array::new();
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

/// Convert a `toml_edit` item back to a JSON value (the inverse of
/// [`json_to_toml_value`], also tolerating real subtables a human may have
/// hand-written into `meta.toml`).
fn toml_item_to_json(item: &Item) -> Option<JsonValue> {
    match item {
        Item::Value(val) => toml_value_to_json(val),
        Item::Table(table) => {
            let mut map = JsonMap::new();
            for (key, child) in table.iter() {
                if let Some(json) = toml_item_to_json(child) {
                    map.insert(key.to_string(), json);
                }
            }
            Some(JsonValue::Object(map))
        }
        Item::ArrayOfTables(tables) => Some(JsonValue::Array(
            tables
                .iter()
                .map(|t| toml_item_to_json(&Item::Table(t.clone())))
                .collect::<Option<Vec<_>>>()?,
        )),
        Item::None => None,
    }
}

/// Convert a `toml_edit` value to a JSON value.
fn toml_value_to_json(val: &Value) -> Option<JsonValue> {
    Some(match val {
        Value::String(s) => JsonValue::from(s.value().clone()),
        Value::Integer(i) => JsonValue::from(*i.value()),
        Value::Float(f) => JsonValue::from(*f.value()),
        Value::Boolean(b) => JsonValue::from(*b.value()),
        Value::Datetime(d) => JsonValue::from(d.value().to_string()),
        Value::Array(array) => {
            JsonValue::Array(array.iter().filter_map(toml_value_to_json).collect())
        }
        Value::InlineTable(inline) => {
            let mut map = JsonMap::new();
            for (key, child) in inline.iter() {
                if let Some(json) = toml_value_to_json(child) {
                    map.insert(key.to_string(), json);
                }
            }
            JsonValue::Object(map)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    /// A fresh, empty temp directory unique to this test run.
    fn tmpdir(label: &str) -> PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "author-skill-{}-{}-{}",
            std::process::id(),
            id,
            label
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    const FIXTURE: &str = "---\n\
name: demo\n\
description: Use when demonstrating the skill IR round-trip.\n\
version: \"1.2.0\"\n\
license: MIT\n\
allowed-tools: [\"Bash\", \"Read\"]\n\
priority: 7\n\
---\n\
# Demo\n\
\n\
Body text, with a trailing space.   \n\
Last line, no newline.";

    fn write_source(dir: &Path, skill_md: &str) {
        fs::write(dir.join("SKILL.md"), skill_md).unwrap();
    }

    #[test]
    fn parses_required_and_optional_and_unknown_keys() {
        let dir = tmpdir("parse");
        write_source(&dir, FIXTURE);

        let skill = Skill::from_source_dir(&dir).unwrap();

        assert_eq!(skill.name, "demo");
        assert_eq!(
            skill.description,
            "Use when demonstrating the skill IR round-trip."
        );
        assert_eq!(skill.version.as_deref(), Some("1.2.0"));
        assert_eq!(skill.license.as_deref(), Some("MIT"));

        // Unknown keys are preserved, never dropped — and not the typed ones.
        assert!(skill.extra.contains_key("allowed-tools"));
        assert_eq!(skill.extra["priority"], JsonValue::from(7));
        for typed in ["name", "description", "version", "license"] {
            assert!(!skill.extra.contains_key(typed));
        }
    }

    #[test]
    fn body_is_byte_faithful() {
        let dir = tmpdir("body");
        write_source(&dir, FIXTURE);

        let skill = Skill::from_source_dir(&dir).unwrap();

        // Everything after the closing `---\n`, trailing bytes intact.
        assert_eq!(
            skill.body,
            "# Demo\n\nBody text, with a trailing space.   \nLast line, no newline."
        );
    }

    #[test]
    fn import_hash_is_stable_across_reparse() {
        let dir = tmpdir("hash");
        write_source(&dir, FIXTURE);

        let first = Skill::from_source_dir(&dir).unwrap();
        let second = Skill::from_source_dir(&dir).unwrap();

        assert_eq!(first.provenance.import_hash, second.provenance.import_hash);
        // SHA-256 is 32 bytes -> 64 hex chars.
        assert_eq!(first.provenance.import_hash.len(), 64);
        assert!(
            first
                .provenance
                .import_hash
                .bytes()
                .all(|b| b.is_ascii_hexdigit())
        );
    }

    #[test]
    fn missing_required_field_is_an_error() {
        let dir = tmpdir("missing");
        write_source(&dir, "---\nname: demo\n---\nbody\n");

        let err = Skill::from_source_dir(&dir).unwrap_err();
        assert!(matches!(
            err,
            SkillError::MissingField {
                field: "description",
                ..
            }
        ));
    }

    #[test]
    fn no_frontmatter_is_an_error() {
        let dir = tmpdir("nofm");
        write_source(&dir, "# Just a body\nwith no frontmatter\n");

        assert!(matches!(
            Skill::from_source_dir(&dir).unwrap_err(),
            SkillError::NoFrontmatter { .. }
        ));
    }

    /// Write a skill to its surface as one member document `<dir>/SKILL.md`, exactly
    /// as `import` does, and return that directory.
    fn write_document(skill: &Skill, label: &str) -> PathBuf {
        let dir = tmpdir(label);
        fs::write(dir.join("SKILL.md"), skill.to_document().emit()).unwrap();
        dir
    }

    #[test]
    fn surface_round_trips_a_document() {
        let src = tmpdir("rt-src");
        write_source(&src, FIXTURE);
        let original = Skill::from_source_dir(&src).unwrap();

        // Project to the surface as ONE document, then reload it.
        let surface = write_document(&original, "rt-surface");
        let reloaded = Skill::from_dir(&surface).unwrap();

        assert_eq!(original, reloaded);
    }

    #[test]
    fn document_is_deterministic_and_keeps_unknown_keys_as_clauses() {
        let dir = tmpdir("doc");
        write_source(&dir, FIXTURE);
        let skill = Skill::from_source_dir(&dir).unwrap();

        let once = skill.to_document().emit();
        let twice = skill.to_document().emit();
        assert_eq!(once, twice, "rendering must be deterministic");

        // Each field is its own `[clause.<field>]` module — typed and unknown alike.
        assert!(once.contains("[clause.name]\nvalue = \"demo\""));
        assert!(once.contains("[clause.allowed-tools]\nvalue = [\"Bash\", \"Read\"]"));
        assert!(once.contains("[clause.priority]\nvalue = 7"));
        assert!(once.contains("[provenance]"));
        assert!(once.contains("import_hash = "));
        // The body rides below the closing fence, byte-faithful.
        assert!(once.contains("+++\n# Demo\n"));
    }

    #[test]
    fn authored_layer_round_trips_and_stays_out_of_frontmatter() {
        let src = tmpdir("rep-src");
        write_source(&src, FIXTURE);
        let mut skill = Skill::from_source_dir(&src).unwrap();

        // Author the surface-only layer — `[satisfies.*]` (with rationale) and an edge.
        skill.satisfies = vec![
            Satisfies {
                requirement: "req-one".to_string(),
                rationale: Some("Fills the demo requirement so coverage resolves.".to_string()),
            },
            Satisfies::new("req-two"),
        ];
        skill.edges = vec![EdgeClause {
            target: "lint-runner".to_string(),
            relation: Some("depends-on".to_string()),
        }];

        let surface = write_document(&skill, "rep-surface");
        let emitted = fs::read_to_string(surface.join("SKILL.md")).unwrap();
        let reloaded = Skill::from_dir(&surface).unwrap();

        // The authored clauses survive the surface round-trip identically.
        assert_eq!(skill, reloaded);
        assert_eq!(
            reloaded.satisfies,
            vec![
                Satisfies {
                    requirement: "req-one".to_string(),
                    rationale: Some("Fills the demo requirement so coverage resolves.".to_string()),
                },
                Satisfies::new("req-two"),
            ]
        );
        assert_eq!(reloaded.edges[0].target, "lint-runner");

        // They ride `[satisfies.*]` / `[edge.*]` modules, never leaking into the
        // frontmatter `extra` the contract engine ranges over.
        assert!(emitted.contains("[satisfies.req-one]"));
        assert!(emitted.contains("[edge.lint-runner]"));
        assert!(!reloaded.extra.contains_key("satisfies"));
        assert!(!reloaded.extra.contains_key("rationale"));
    }

    #[test]
    fn unauthored_layer_leaves_the_document_without_authored_modules() {
        let dir = tmpdir("rep-none");
        write_source(&dir, FIXTURE);
        let skill = Skill::from_source_dir(&dir).unwrap();

        // With nothing authored, the document carries no `[satisfies.*]`/`[edge.*]`.
        let emitted = skill.to_document().emit();
        assert!(skill.satisfies.is_empty());
        assert!(skill.edges.is_empty());
        assert!(!emitted.contains("[satisfies."));
        assert!(!emitted.contains("[edge."));
    }

    #[test]
    fn scans_companions_relative_and_sorted() {
        let dir = tmpdir("companions");
        write_source(&dir, FIXTURE);
        fs::write(dir.join("PLAYBOOK.md"), "playbook\n").unwrap();
        fs::create_dir_all(dir.join("scripts")).unwrap();
        fs::write(dir.join("scripts").join("run.sh"), "echo hi\n").unwrap();

        let skill = Skill::from_source_dir(&dir).unwrap();

        // SKILL.md (the document) is excluded; the rest are relative + sorted.
        assert_eq!(
            skill.companions,
            vec![
                PathBuf::from("PLAYBOOK.md"),
                PathBuf::from("scripts").join("run.sh"),
            ]
        );
    }
}
