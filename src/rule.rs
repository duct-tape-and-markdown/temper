//! The `Rule` artifact — the typed IR for `.claude/rules/<name>.md`.
//!
//! Models the rule instance of the IR in `specs/20-surface.md` ("Artifact kinds
//! & contract selection" — the next kind after skill). A rule is a Claude Code
//! scoping document: an OPTIONAL `paths` frontmatter sequence (the real scoping
//! key) plus a byte-faithful markdown body. Unlike a skill — which errors without
//! frontmatter — a rule may carry none at all (`.claude/rules/collaboration.md`
//! has no header; `rust.md` has `paths:`). The rule name is the file stem, not an
//! internal field.
//!
//! A rule is read from its source file with [`Rule::from_source_file`] (split the
//! YAML frontmatter, hash the original bytes for provenance), projected to the
//! typed surface header with [`Rule::to_meta_document`], and reloaded from the
//! written surface with [`Rule::from_surface_dir`].
//!
//! Round-trip discipline (`.claude/rules/rust.md`): the markdown body is
//! byte-faithful — never re-rendered. Only the structured header (`meta.toml`) is
//! written, via `toml_edit`. Unknown frontmatter keys are preserved verbatim in
//! [`Rule::extra`] (never dropped), so a `forbidden_keys` clause can later resolve
//! the Cursor keys (`description`/`globs`/`alwaysApply`) Claude Code ignores.
//! `import_hash` is the SHA-256 of the original file bytes — the drift anchor,
//! computed at import so the provenance lock is complete before write-back exists.
//!
//! The split-frontmatter and SHA-256 helpers are duplicated from `src/skill.rs`
//! rather than shared: one artifact kind per module keeps the rule self-contained,
//! and clarity beats a premature de-duplication (`.claude/rules/rust.md`).

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use gray_matter::Pod;
use gray_matter::engine::{Engine, YAML};
use serde_json::{Map as JsonMap, Value as JsonValue};
use sha2::{Digest, Sha256};
use toml_edit::{Array, DocumentMut, Item, Table, Value, value};

/// The frontmatter keys projected to typed fields. Everything else is an
/// "unknown" key, preserved verbatim under [`Rule::extra`].
const KNOWN_KEYS: [&str; 1] = ["paths"];

/// A Claude Code rule: an optional `paths` scope, a byte-faithful body, the
/// unknown frontmatter keys preserved verbatim, and the provenance lock that
/// anchors drift.
#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    /// Rule name, derived from the source file stem (`rust.md` -> `rust`).
    pub name: String,
    /// The `paths` frontmatter sequence — Claude Code's scoping key. `None` when
    /// the rule declares no `paths` (or carries no frontmatter at all).
    pub paths: Option<Vec<String>>,
    /// Markdown after the frontmatter, byte-faithful (trailing bytes intact). For
    /// a rule with no frontmatter this is the whole file.
    pub body: String,
    /// The requirements this artifact opts into filling (`specs/20-surface.md`,
    /// "Each artifact directory is a representation, not a copy") — the coverage
    /// check's opt-in bindings. **Authored** on the surface, not imported: the
    /// source `.md` carries no such field, so `from_source_file` leaves this
    /// empty. It lives under `meta.toml`'s `[representation]` table, kept out of
    /// the frontmatter `extra` the contract engine ranges over.
    pub satisfies: Vec<String>,
    /// The authored *why* bound to this artifact (`00-intent.md` law 7 — the
    /// behavioral-intent layer). **Authored**, not imported (`from_source_file`
    /// leaves it `None`); carried under `[representation]` in `meta.toml`. It is
    /// the human rationale, never a decidable feature, so extraction never reads
    /// it.
    pub rationale: Option<String>,
    /// Unknown frontmatter keys, preserved verbatim so the surface never drops
    /// authoring intent — and so a `forbidden_keys` clause can resolve them by
    /// name. Sorted by key (`serde_json::Map` is a `BTreeMap`), which makes
    /// `to_meta_document` deterministic and `import` idempotent.
    pub extra: JsonMap<String, JsonValue>,
    /// Where the rule came from and the hash of its original bytes.
    pub provenance: Provenance,
}

/// The import lock for a rule: its origin path and a content hash of the original
/// source bytes. `import_hash` drives future drift detection.
#[derive(Debug, Clone, PartialEq)]
pub struct Provenance {
    /// Absolute or workspace-relative path to the source `.md` file.
    pub source_path: PathBuf,
    /// Lowercase hex SHA-256 of the original source bytes.
    pub import_hash: String,
}

/// Errors raised while reading or projecting a [`Rule`]. These are hard failures
/// (missing file, malformed surface) — distinct from a lint `Diagnostic`, which
/// is a finding the engine collects rather than an error.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum RuleError {
    /// A file could not be read.
    #[error("failed to read {path}")]
    #[diagnostic(code(temper::rule::io))]
    Io {
        /// The path that failed to read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// The source `.md` file is not valid UTF-8, so its body cannot be modelled.
    #[error("{path} is not valid UTF-8")]
    #[diagnostic(code(temper::rule::not_utf8))]
    NotUtf8 {
        /// The offending file.
        path: PathBuf,
        /// The decode error.
        #[source]
        source: std::string::FromUtf8Error,
    },

    /// The source path has no file stem to derive the rule name from.
    #[error("{path} has no file stem to use as the rule name")]
    #[diagnostic(code(temper::rule::no_name))]
    NoName {
        /// The path missing a usable stem.
        path: PathBuf,
    },

    /// A required field is absent from the written surface (`meta.toml` or its
    /// surface directory name).
    #[error("{path}: surface is missing required field `{field}`")]
    #[diagnostic(code(temper::rule::missing_field))]
    MissingField {
        /// The surface whose header is incomplete.
        path: PathBuf,
        /// The required key that was absent.
        field: &'static str,
    },

    /// `meta.toml` could not be parsed as TOML.
    #[error("failed to parse {path} as TOML")]
    #[diagnostic(code(temper::rule::bad_toml))]
    Toml {
        /// The surface header that failed to parse.
        path: PathBuf,
        /// The TOML parse error.
        #[source]
        source: toml_edit::TomlError,
    },
}

impl Rule {
    /// Parse a rule from its source file `.claude/rules/<name>.md`: read the
    /// bytes, split any YAML frontmatter, and record provenance.
    ///
    /// Unlike a skill (whose source is a directory), a rule's source is a single
    /// flat file, so this takes the file path directly and derives the name from
    /// its stem. Frontmatter is optional — a rule with none parses to an empty
    /// header and a body that is the whole file. The body is taken byte-faithfully
    /// from after the closing delimiter; the `import_hash` is the SHA-256 of the
    /// original file bytes.
    pub fn from_source_file(path: &Path) -> Result<Self, RuleError> {
        let bytes = fs::read(path).map_err(|source| RuleError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        let import_hash = sha256_hex(&bytes);
        let raw = String::from_utf8(bytes).map_err(|source| RuleError::NotUtf8 {
            path: path.to_path_buf(),
            source,
        })?;

        let name = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(str::to_string)
            .ok_or_else(|| RuleError::NoName {
                path: path.to_path_buf(),
            })?;

        // A rule may carry no frontmatter at all — then `paths` is absent and the
        // body is the entire file.
        let (matter, body) = split_frontmatter(&raw);
        let (paths, extra) = match matter {
            Some(matter) => {
                let mut fields = match YAML::parse(matter.trim()) {
                    Pod::Hash(hash) => hash,
                    // Frontmatter present but not a mapping: no keys to read.
                    _ => HashMap::new(),
                };
                let paths = take_optional_list(&mut fields, "paths");
                (paths, pod_hash_to_json(fields))
            }
            None => (None, JsonMap::new()),
        };

        Ok(Self {
            name,
            paths,
            body: body.to_string(),
            // `satisfies`/`rationale` are authored on the surface, never present
            // in the source — so import leaves them empty and an unauthored
            // rule's `meta.toml` stays byte-identical (import idempotence).
            satisfies: Vec::new(),
            rationale: None,
            extra,
            provenance: Provenance {
                source_path: path.to_path_buf(),
                import_hash,
            },
        })
    }

    /// Reload a rule from its written surface `<dir>/`: `meta.toml` (the optional
    /// `paths`, any preserved unknown keys, and the `[provenance]` table) plus
    /// `<dir>/RULE.md` (the body alone). The name is the surface directory name.
    ///
    /// The inverse of [`Rule::to_meta_document`] over the same directory:
    /// `import_hash` is read back from the provenance lock, not recomputed (the
    /// surface body differs from the original, which may have carried frontmatter).
    pub fn from_surface_dir(dir: &Path) -> Result<Self, RuleError> {
        let name = dir
            .file_name()
            .and_then(|name| name.to_str())
            .map(str::to_string)
            .ok_or_else(|| RuleError::MissingField {
                path: dir.to_path_buf(),
                field: "name",
            })?;

        let meta_path = dir.join("meta.toml");
        let meta_src = fs::read_to_string(&meta_path).map_err(|source| RuleError::Io {
            path: meta_path.clone(),
            source,
        })?;
        let doc = meta_src
            .parse::<DocumentMut>()
            .map_err(|source| RuleError::Toml {
                path: meta_path.clone(),
                source,
            })?;
        let table = doc.as_table();

        let paths = optional_list(table, "paths");

        let provenance =
            table
                .get("provenance")
                .and_then(Item::as_table)
                .ok_or(RuleError::MissingField {
                    path: meta_path.clone(),
                    field: "provenance",
                })?;
        let source_path = PathBuf::from(required_str(provenance, "source_path", &meta_path)?);
        let import_hash = required_str(provenance, "import_hash", &meta_path)?;

        // The authored representation layer, if present. Absent when the rule was
        // never authored with either field (its `meta.toml` has no table).
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

        let body_path = dir.join("RULE.md");
        let body = fs::read_to_string(&body_path).map_err(|source| RuleError::Io {
            path: body_path,
            source,
        })?;

        Ok(Self {
            name,
            paths,
            body,
            satisfies,
            rationale,
            extra,
            provenance: Provenance {
                source_path,
                import_hash,
            },
        })
    }

    /// Project the typed header to a format-preserving `toml_edit` document: the
    /// `paths` sequence if present, every unknown frontmatter key (written as TOML
    /// values, sorted), then the `[provenance]` table last.
    ///
    /// Unknown keys are emitted as inline values (objects become inline tables) so
    /// the only standard table is `[provenance]` — sidestepping TOML's
    /// "scalars before tables" ordering hazard while staying lossless.
    pub fn to_meta_document(&self) -> DocumentMut {
        let mut doc = DocumentMut::new();

        if let Some(paths) = &self.paths {
            let mut array = Array::new();
            for path in paths {
                array.push(path.clone());
            }
            doc["paths"] = Item::Value(Value::Array(array));
        }

        for (key, json) in &self.extra {
            if let Some(val) = json_to_toml_value(json) {
                doc[key.as_str()] = Item::Value(val);
            }
        }

        // The authored representation layer, emitted only when non-empty so an
        // unauthored rule's `meta.toml` is byte-identical to slice 1 (import
        // idempotence holds).
        if !self.satisfies.is_empty() || self.rationale.is_some() {
            let mut representation = Table::new();
            if !self.satisfies.is_empty() {
                let mut array = Array::new();
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

/// Split a source `.md` file into its YAML frontmatter block and a byte-faithful
/// body. Mirrors `gray_matter`'s `---` delimiter detection but, unlike its
/// `content` field (which trims surrounding whitespace), returns the body exactly
/// as it appears after the closing delimiter line. Returns `(None, raw)` when
/// there is no leading frontmatter block to strip — the common case for a rule.
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

/// Remove and stringify an optional list frontmatter field (a YAML sequence) into
/// a `Vec<String>`. A non-sequence value is left in `fields` so it is preserved
/// verbatim in `extra` rather than silently dropped.
fn take_optional_list(fields: &mut HashMap<String, Pod>, field: &str) -> Option<Vec<String>> {
    let Some(Pod::Array(items)) = fields.get(field) else {
        return None;
    };
    let list = items.iter().filter_map(pod_scalar_to_string).collect();
    fields.remove(field);
    Some(list)
}

/// Coerce a scalar [`Pod`] to its string form. Non-scalars (arrays, hashes, null)
/// yield `None`.
fn pod_scalar_to_string(pod: &Pod) -> Option<String> {
    match pod {
        Pod::String(value) => Some(value.clone()),
        Pod::Integer(value) => Some(value.to_string()),
        Pod::Float(value) => Some(value.to_string()),
        Pod::Boolean(value) => Some(value.to_string()),
        Pod::Null | Pod::Array(_) | Pod::Hash(_) => None,
    }
}

/// Convert the leftover (unknown) frontmatter keys into a JSON map, dropping nulls
/// (TOML has no null) so the surface stays representable and the source/surface
/// round-trip is symmetric.
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
fn required_str(table: &Table, field: &'static str, path: &Path) -> Result<String, RuleError> {
    table
        .get(field)
        .and_then(Item::as_str)
        .map(str::to_string)
        .ok_or(RuleError::MissingField {
            path: path.to_path_buf(),
            field,
        })
}

/// An optional list field of a TOML table, read back as `Vec<String>`.
fn optional_list(table: &Table, field: &str) -> Option<Vec<String>> {
    table.get(field).and_then(Item::as_array).map(|array| {
        array
            .iter()
            .filter_map(|val| val.as_str().map(str::to_string))
            .collect()
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
    optional_list(table, field).unwrap_or_default()
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
            "author-rule-{}-{}-{}",
            std::process::id(),
            id,
            label
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// A rule with `paths:` frontmatter and one unknown key, with a body whose
    /// trailing bytes must survive intact.
    const PATHS_RULE: &str = "---\n\
paths:\n\
  - \"src/**/*.rs\"\n\
  - \"tests/**/*.rs\"\n\
description: A Cursor key Claude Code ignores — preserved, not dropped.\n\
---\n\
# Rust conventions\n\
\n\
Prefer a clone over a lifetime fight.   \n\
Last line, no newline.";

    /// A rule with no frontmatter at all — the `collaboration.md` shape.
    const NO_FRONTMATTER_RULE: &str = "# Collaboration\n\
\n\
Pushback is the point.\n";

    fn write_source(dir: &Path, name: &str, body: &str) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, body).unwrap();
        path
    }

    #[test]
    fn parses_paths_and_preserves_unknown_keys() {
        let dir = tmpdir("paths");
        let path = write_source(&dir, "rust.md", PATHS_RULE);

        let rule = Rule::from_source_file(&path).unwrap();

        // Name is the file stem, not an internal field.
        assert_eq!(rule.name, "rust");
        assert_eq!(
            rule.paths,
            Some(vec!["src/**/*.rs".to_string(), "tests/**/*.rs".to_string()])
        );
        // An unknown frontmatter key is preserved verbatim, never dropped.
        assert_eq!(
            rule.extra.get("description"),
            Some(&JsonValue::from(
                "A Cursor key Claude Code ignores — preserved, not dropped."
            ))
        );
        // `paths` is a typed field, so it must not leak into `extra`.
        assert!(!rule.extra.contains_key("paths"));
    }

    #[test]
    fn body_is_byte_faithful() {
        let dir = tmpdir("body");
        let path = write_source(&dir, "rust.md", PATHS_RULE);

        let rule = Rule::from_source_file(&path).unwrap();

        assert_eq!(
            rule.body,
            "# Rust conventions\n\nPrefer a clone over a lifetime fight.   \nLast line, no newline."
        );
    }

    #[test]
    fn no_frontmatter_parses_to_empty_header_and_full_body() {
        let dir = tmpdir("nofm");
        let path = write_source(&dir, "collaboration.md", NO_FRONTMATTER_RULE);

        let rule = Rule::from_source_file(&path).unwrap();

        assert_eq!(rule.name, "collaboration");
        assert!(rule.paths.is_none());
        assert!(rule.extra.is_empty());
        // With no frontmatter, the body is the whole file, byte-for-byte.
        assert_eq!(rule.body, NO_FRONTMATTER_RULE);
    }

    #[test]
    fn import_hash_is_stable_across_reparse() {
        let dir = tmpdir("hash");
        let path = write_source(&dir, "rust.md", PATHS_RULE);

        let first = Rule::from_source_file(&path).unwrap();
        let second = Rule::from_source_file(&path).unwrap();

        assert_eq!(first.provenance.import_hash, second.provenance.import_hash);
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
    fn non_sequence_paths_is_preserved_in_extra_not_dropped() {
        let dir = tmpdir("scalar-paths");
        let path = write_source(&dir, "weird.md", "---\npaths: \"src/**\"\n---\nbody\n");

        let rule = Rule::from_source_file(&path).unwrap();

        // A scalar `paths` isn't the typed sequence, so it stays verbatim in extra
        // rather than being silently lost.
        assert!(rule.paths.is_none());
        assert_eq!(rule.extra.get("paths"), Some(&JsonValue::from("src/**")));
    }

    /// Project a rule to its surface (`<name>/meta.toml` + `RULE.md`) and reload.
    fn round_trip(name: &str, source: &str) {
        let src = tmpdir("rt-src");
        let path = write_source(&src, &format!("{name}.md"), source);
        let original = Rule::from_source_file(&path).unwrap();

        // The surface mirrors a skill: a `<name>/` dir carries the typed header
        // and the body alone. The reloaded name comes from that directory name.
        let surface_root = tmpdir("rt-surface");
        let surface = surface_root.join(name);
        fs::create_dir_all(&surface).unwrap();
        fs::write(
            surface.join("meta.toml"),
            original.to_meta_document().to_string(),
        )
        .unwrap();
        fs::write(surface.join("RULE.md"), &original.body).unwrap();

        let reloaded = Rule::from_surface_dir(&surface).unwrap();

        assert_eq!(original, reloaded);
    }

    #[test]
    fn surface_round_trips_a_paths_rule() {
        round_trip("rust", PATHS_RULE);
    }

    #[test]
    fn surface_round_trips_a_no_frontmatter_rule() {
        round_trip("collaboration", NO_FRONTMATTER_RULE);
    }

    #[test]
    fn authored_representation_round_trips_and_stays_out_of_frontmatter() {
        let src = tmpdir("rep-src");
        let path = write_source(&src, "rust.md", PATHS_RULE);
        let mut rule = Rule::from_source_file(&path).unwrap();

        // Author the surface-only representation layer.
        rule.satisfies = vec!["req.rust-style".to_string()];
        rule.rationale = Some("Encodes the Rust conventions the gate enforces.".to_string());

        let surface_root = tmpdir("rep-surface");
        let surface = surface_root.join("rust");
        fs::create_dir_all(&surface).unwrap();
        let meta = rule.to_meta_document().to_string();
        fs::write(surface.join("meta.toml"), &meta).unwrap();
        fs::write(surface.join("RULE.md"), &rule.body).unwrap();

        let reloaded = Rule::from_surface_dir(&surface).unwrap();

        // Satisfies/rationale survive the surface round-trip identically.
        assert_eq!(rule, reloaded);
        assert_eq!(reloaded.satisfies, vec!["req.rust-style"]);
        assert_eq!(
            reloaded.rationale.as_deref(),
            Some("Encodes the Rust conventions the gate enforces.")
        );

        // They ride `[representation]`, never leaking into the frontmatter
        // `extra` a `forbidden_keys` clause ranges over.
        assert!(meta.contains("[representation]"));
        assert!(!reloaded.extra.contains_key("satisfies"));
        assert!(!reloaded.extra.contains_key("rationale"));
        assert!(!reloaded.extra.contains_key("representation"));
    }

    #[test]
    fn unauthored_representation_leaves_meta_byte_identical() {
        // A no-frontmatter rule with no representation authored stays byte-
        // identical to slice 1: no `[representation]` table (import idempotence).
        let dir = tmpdir("rep-none");
        let path = write_source(&dir, "collaboration.md", NO_FRONTMATTER_RULE);
        let rule = Rule::from_source_file(&path).unwrap();

        let meta = rule.to_meta_document().to_string();
        assert!(rule.satisfies.is_empty());
        assert!(rule.rationale.is_none());
        assert!(!meta.contains("[representation]"));
    }

    #[test]
    fn meta_document_is_deterministic_and_keeps_paths_and_unknown_keys() {
        let dir = tmpdir("meta");
        let path = write_source(&dir, "rust.md", PATHS_RULE);
        let rule = Rule::from_source_file(&path).unwrap();

        let once = rule.to_meta_document().to_string();
        let twice = rule.to_meta_document().to_string();
        assert_eq!(once, twice, "rendering must be deterministic");

        assert!(once.contains("paths = [\"src/**/*.rs\", \"tests/**/*.rs\"]"));
        assert!(once.contains("description = "));
        assert!(once.contains("[provenance]"));
        assert!(once.contains("import_hash = "));
    }
}
