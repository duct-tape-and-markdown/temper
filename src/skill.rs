//! The `Skill` artifact — the typed IR for `~/.claude/skills/<name>/SKILL.md`.
//!
//! Models the skill instance of the IR in `specs/20-surface.md` ("The IR" —
//! one typed value per artifact kind). A skill is read from source with
//! [`Skill::from_source_dir`] (split YAML frontmatter, scan companions, hash the
//! original bytes for provenance), projected to the typed surface header with
//! [`Skill::to_meta_document`], and reloaded from that surface with
//! [`Skill::from_surface_dir`].
//!
//! Round-trip discipline (`.claude/rules/rust.md`): the markdown body and every
//! companion are byte-faithful — never re-rendered. Only the structured header
//! (`meta.toml`) is written, via `toml_edit`. Unknown frontmatter keys are
//! preserved in that header, never dropped. `import_hash` is the SHA-256 of the
//! original `SKILL.md` bytes; it is the drift anchor and is computed at import
//! so the provenance lock is complete even before write-back exists.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use gray_matter::Pod;
use gray_matter::engine::{Engine, YAML};
use serde_json::{Map as JsonMap, Value as JsonValue};
use sha2::{Digest, Sha256};
use toml_edit::{DocumentMut, Item, Table, Value, value};
use walkdir::WalkDir;

/// The canonical, ordered frontmatter keys that are projected to typed fields.
/// Everything else is an "unknown" key, preserved verbatim under [`Skill::extra`].
const KNOWN_KEYS: [&str; 4] = ["name", "description", "version", "license"];

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
    /// The requirements this artifact opts into filling (`specs/20-surface.md`,
    /// "Each artifact directory is a representation, not a copy") — the coverage
    /// check's opt-in bindings. **Authored** on the surface, not imported: the
    /// source `SKILL.md` carries no such field, so `from_source_dir` leaves this
    /// empty. It lives under `meta.toml`'s `[representation]` table, kept out of
    /// the frontmatter `extra` the contract engine ranges over.
    pub satisfies: Vec<String>,
    /// The authored *why* bound to this artifact (`00-intent.md` law 7 — the
    /// behavioral-intent layer). **Authored**, not imported (`from_source_dir`
    /// leaves it `None`); carried under `[representation]` in `meta.toml`. It is
    /// the human rationale, never a decidable feature, so extraction never reads
    /// it.
    pub rationale: Option<String>,
    /// Sibling files that ship with the skill (e.g. `PLAYBOOK.md`, `scripts/**`),
    /// as paths relative to the skill directory, sorted for determinism.
    pub companions: Vec<PathBuf>,
    /// Unknown frontmatter keys, preserved verbatim so the surface never drops
    /// authoring intent. Sorted by key (`serde_json::Map` is a `BTreeMap`), which
    /// makes `to_meta_document` deterministic and `import` idempotent.
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

    /// `meta.toml` could not be parsed as TOML.
    #[error("failed to parse {path} as TOML")]
    #[diagnostic(code(temper::skill::bad_toml))]
    Toml {
        /// The surface header that failed to parse.
        path: PathBuf,
        /// The TOML parse error.
        #[source]
        source: toml_edit::TomlError,
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
        let import_hash = sha256_hex(&bytes);
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
            // `satisfies`/`rationale` are authored on the surface, never present
            // in the source — so import leaves them empty and an unauthored
            // skill's `meta.toml` stays byte-identical (import idempotence).
            satisfies: Vec::new(),
            rationale: None,
            companions: scan_companions(dir)?,
            extra,
            provenance: Provenance {
                source_path,
                import_hash,
            },
        })
    }

    /// Reload a skill from its written surface: `<dir>/meta.toml` (typed header +
    /// `[provenance]`) plus `<dir>/SKILL.md` (the body alone, no frontmatter).
    ///
    /// The inverse of [`Skill::to_meta_document`] over the same directory: the
    /// surface `SKILL.md` holds only the body, and `import_hash` is read back
    /// from the provenance lock, not recomputed (the surface body differs from
    /// the original, which carried frontmatter).
    pub fn from_surface_dir(dir: &Path) -> Result<Self, SkillError> {
        let meta_path = dir.join("meta.toml");
        let meta_src = fs::read_to_string(&meta_path).map_err(|source| SkillError::Io {
            path: meta_path.clone(),
            source,
        })?;
        let doc = meta_src
            .parse::<DocumentMut>()
            .map_err(|source| SkillError::Toml {
                path: meta_path.clone(),
                source,
            })?;
        let table = doc.as_table();

        let name = required_str(table, "name", &meta_path)?;
        let description = required_str(table, "description", &meta_path)?;
        let version = optional_str(table, "version");
        let license = optional_str(table, "license");

        let provenance =
            table
                .get("provenance")
                .and_then(Item::as_table)
                .ok_or(SkillError::MissingField {
                    path: meta_path.clone(),
                    field: "provenance",
                })?;
        let source_path = PathBuf::from(required_str(provenance, "source_path", &meta_path)?);
        let import_hash = required_str(provenance, "import_hash", &meta_path)?;

        // The authored representation layer, if present. Absent when the skill
        // was never authored with either field (its `meta.toml` has no table).
        let representation = table.get("representation").and_then(Item::as_table);
        let satisfies = representation
            .map(|table| string_list(table, "satisfies"))
            .unwrap_or_default();
        let rationale = representation.and_then(|table| optional_str(table, "rationale"));

        let mut extra = JsonMap::new();
        for (key, item) in table.iter() {
            // `representation` is authored, not frontmatter — like `provenance`,
            // it must never leak back into the contract-checked `extra`.
            if KNOWN_KEYS.contains(&key) || key == "provenance" || key == "representation" {
                continue;
            }
            if let Some(json) = toml_item_to_json(item) {
                extra.insert(key.to_string(), json);
            }
        }

        let body_path = dir.join("SKILL.md");
        let body = fs::read_to_string(&body_path).map_err(|source| SkillError::Io {
            path: body_path,
            source,
        })?;

        Ok(Self {
            name,
            description,
            version,
            license,
            body,
            satisfies,
            rationale,
            companions: scan_companions(dir)?,
            extra,
            provenance: Provenance {
                source_path,
                import_hash,
            },
        })
    }

    /// Project the typed header to a format-preserving `toml_edit` document:
    /// the known fields in canonical order, every unknown frontmatter key
    /// (written as TOML values, sorted), then the `[provenance]` table last.
    ///
    /// Unknown keys are emitted as inline values (objects become inline tables)
    /// so the only standard table is `[provenance]` — sidestepping TOML's
    /// "scalars before tables" ordering hazard while staying lossless.
    pub fn to_meta_document(&self) -> DocumentMut {
        let mut doc = DocumentMut::new();
        doc["name"] = value(self.name.clone());
        doc["description"] = value(self.description.clone());
        if let Some(version) = &self.version {
            doc["version"] = value(version.clone());
        }
        if let Some(license) = &self.license {
            doc["license"] = value(license.clone());
        }

        for (key, json) in &self.extra {
            if let Some(val) = json_to_toml_value(json) {
                doc[key.as_str()] = Item::Value(val);
            }
        }

        // The authored representation layer, emitted only when non-empty so an
        // unauthored skill's `meta.toml` is byte-identical to slice 1 (import
        // idempotence holds).
        if !self.satisfies.is_empty() || self.rationale.is_some() {
            let mut representation = Table::new();
            if !self.satisfies.is_empty() {
                let mut array = toml_edit::Array::new();
                for requirement in &self.satisfies {
                    array.push(requirement.clone());
                }
                representation["satisfies"] = Item::Value(Value::Array(array));
            }
            if let Some(rationale) = &self.rationale {
                representation["rationale"] = value(rationale.clone());
            }
            doc["representation"] = Item::Table(representation);
        }

        let mut provenance = Table::new();
        provenance["source_path"] =
            value(self.provenance.source_path.to_string_lossy().into_owned());
        provenance["import_hash"] = value(self.provenance.import_hash.clone());
        doc["provenance"] = Item::Table(provenance);

        doc
    }
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

/// A required string field of a TOML table.
fn required_str(table: &Table, field: &'static str, path: &Path) -> Result<String, SkillError> {
    table
        .get(field)
        .and_then(Item::as_str)
        .map(str::to_string)
        .ok_or(SkillError::MissingField {
            path: path.to_path_buf(),
            field,
        })
}

/// An optional string field of a TOML table.
fn optional_str(table: &Table, field: &str) -> Option<String> {
    table.get(field).and_then(Item::as_str).map(str::to_string)
}

/// The string elements of an array field of a TOML table, or an empty vec when
/// the field is absent or not an array. Used to read `[representation].satisfies`
/// back off the surface.
fn string_list(table: &Table, field: &str) -> Vec<String> {
    table
        .get(field)
        .and_then(Item::as_array)
        .map(|array| {
            array
                .iter()
                .filter_map(|val| val.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

/// Walk a skill directory and collect its companion files — every file except
/// `SKILL.md` and `meta.toml` — as paths relative to `dir`, sorted.
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
        if name == "SKILL.md" || name == "meta.toml" {
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

        // Unknown keys are preserved, never dropped — and not the known ones.
        assert!(skill.extra.contains_key("allowed-tools"));
        assert_eq!(skill.extra["priority"], JsonValue::from(7));
        for known in KNOWN_KEYS {
            assert!(!skill.extra.contains_key(known));
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

    #[test]
    fn surface_round_trips_a_meta_document() {
        let src = tmpdir("rt-src");
        write_source(&src, FIXTURE);
        let original = Skill::from_source_dir(&src).unwrap();

        // Project to the surface: meta.toml + the body alone.
        let surface = tmpdir("rt-surface");
        fs::write(
            surface.join("meta.toml"),
            original.to_meta_document().to_string(),
        )
        .unwrap();
        fs::write(surface.join("SKILL.md"), &original.body).unwrap();

        let reloaded = Skill::from_surface_dir(&surface).unwrap();

        assert_eq!(original, reloaded);
    }

    #[test]
    fn meta_document_is_deterministic_and_keeps_unknown_keys() {
        let dir = tmpdir("meta");
        write_source(&dir, FIXTURE);
        let skill = Skill::from_source_dir(&dir).unwrap();

        let once = skill.to_meta_document().to_string();
        let twice = skill.to_meta_document().to_string();
        assert_eq!(once, twice, "rendering must be deterministic");

        assert!(once.contains("allowed-tools = [\"Bash\", \"Read\"]"));
        assert!(once.contains("priority = 7"));
        assert!(once.contains("[provenance]"));
        assert!(once.contains("import_hash = "));
    }

    #[test]
    fn authored_representation_round_trips_and_stays_out_of_frontmatter() {
        let src = tmpdir("rep-src");
        write_source(&src, FIXTURE);
        let mut skill = Skill::from_source_dir(&src).unwrap();

        // Author the representation layer — the surface-only fields.
        skill.satisfies = vec!["req.one".to_string(), "req.two".to_string()];
        skill.rationale = Some("Fills the demo requirement so coverage resolves.".to_string());

        let surface = tmpdir("rep-surface");
        let meta = skill.to_meta_document().to_string();
        fs::write(surface.join("meta.toml"), &meta).unwrap();
        fs::write(surface.join("SKILL.md"), &skill.body).unwrap();

        let reloaded = Skill::from_surface_dir(&surface).unwrap();

        // The authored fields survive the surface round-trip identically.
        assert_eq!(skill, reloaded);
        assert_eq!(reloaded.satisfies, vec!["req.one", "req.two"]);
        assert_eq!(
            reloaded.rationale.as_deref(),
            Some("Fills the demo requirement so coverage resolves.")
        );

        // They live under `[representation]`, never leaking into the frontmatter
        // `extra` the contract engine ranges over.
        assert!(meta.contains("[representation]"));
        assert!(!reloaded.extra.contains_key("satisfies"));
        assert!(!reloaded.extra.contains_key("rationale"));
        assert!(!reloaded.extra.contains_key("representation"));
    }

    #[test]
    fn unauthored_representation_leaves_meta_byte_identical() {
        let dir = tmpdir("rep-none");
        write_source(&dir, FIXTURE);
        let skill = Skill::from_source_dir(&dir).unwrap();

        // With neither field authored, `meta.toml` must be byte-identical to
        // slice 1: no `[representation]` table at all (import idempotence).
        let meta = skill.to_meta_document().to_string();
        assert!(skill.satisfies.is_empty());
        assert!(skill.rationale.is_none());
        assert!(!meta.contains("[representation]"));
    }

    #[test]
    fn scans_companions_relative_and_sorted() {
        let dir = tmpdir("companions");
        write_source(&dir, FIXTURE);
        fs::write(dir.join("meta.toml"), "ignored = true\n").unwrap();
        fs::write(dir.join("PLAYBOOK.md"), "playbook\n").unwrap();
        fs::create_dir_all(dir.join("scripts")).unwrap();
        fs::write(dir.join("scripts").join("run.sh"), "echo hi\n").unwrap();

        let skill = Skill::from_source_dir(&dir).unwrap();

        // SKILL.md and meta.toml are excluded; the rest are relative + sorted.
        assert_eq!(
            skill.companions,
            vec![
                PathBuf::from("PLAYBOOK.md"),
                PathBuf::from("scripts").join("run.sh"),
            ]
        );
    }
}
