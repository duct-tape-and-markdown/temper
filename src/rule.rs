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
//! YAML frontmatter, hash the original bytes for provenance), projected to its **one
//! authored document** with [`Rule::to_document`], and reloaded from the written
//! surface with [`Rule::from_dir`].
//!
//! The member is a single document (`specs/20-surface.md`, "Decision: the member is
//! one document in the surface language"): a `+++`-fenced TOML header over the body,
//! in place of the retired `meta.toml` + body pair. The header carries a
//! `[clause.<field>]` module per structured field (the optional `paths` *and* the
//! verbatim-preserved unknown frontmatter keys), the authored `[satisfies.*]` /
//! `[edge.*]` modules, and the generated `[provenance]`.
//!
//! Round-trip discipline (`.claude/rules/rust.md`): the markdown body is
//! byte-faithful — never re-rendered. Only the structured header is written, via
//! `toml_edit`. Unknown frontmatter keys are preserved verbatim in [`Rule::extra`]
//! (never dropped), so a `forbidden_keys` clause can later resolve the Cursor keys
//! (`description`/`globs`/`alwaysApply`) Claude Code ignores. `import_hash` is the
//! SHA-256 of the original file bytes — the drift anchor, computed at import so the
//! provenance lock is complete before write-back exists.
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
use toml_edit::{Array, DocumentMut, Item, Value};

use crate::document::{self, Document, EdgeClause, PublishedRequirement, Satisfies};

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
    /// The requirements this artifact opts into filling (`specs/20-surface.md`, the
    /// `[satisfies.<requirement>]` clause modules) — the coverage check's opt-in
    /// bindings, each carrying its optional authored `rationale`. **Authored** on
    /// the surface, not imported: the source `.md` carries no such field, so
    /// `from_source_file` leaves this empty. It lives in the header's `[satisfies.*]`
    /// modules, kept out of the frontmatter `extra` the contract engine ranges over.
    pub satisfies: Vec<Satisfies>,
    /// The declared references/relationships to other members
    /// (`specs/45-governance.md`, "an edge is a declared field on the surface") —
    /// the header's `[edge.<target>]` modules. **Authored** on the surface, not
    /// imported (`from_source_file` leaves this empty); the graph's source, never
    /// grepped from prose.
    pub edges: Vec<EdgeClause>,
    /// The requirements this artifact **publishes** (`specs/10-contracts.md`,
    /// "Decision: a requirement's publisher is any authored surface document") — the
    /// header's `[requirement.<name>]` modules, the demand side of the fill edge.
    /// **Authored** on the surface, not imported (`from_source_file` leaves this
    /// empty); the gate unions them with the assembly roster into the one requirement
    /// namespace.
    pub published_requirements: Vec<PublishedRequirement>,
    /// Unknown frontmatter keys, preserved verbatim so the surface never drops
    /// authoring intent — and so a `forbidden_keys` clause can resolve them by
    /// name. Sorted by key (`serde_json::Map` is a `BTreeMap`), which makes
    /// `to_document` deterministic and `import` idempotent.
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

    /// The surface `RULE.md` is not a well-formed `+++`-fenced document (missing or
    /// unterminated fence, or a malformed TOML header).
    #[error("{path}: {source}")]
    #[diagnostic(code(temper::rule::bad_document))]
    Document {
        /// The surface document that failed to parse.
        path: PathBuf,
        /// The underlying fenced-document parse error.
        #[source]
        source: crate::document::DocumentError,
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
        let import_hash = crate::hash::sha256_hex(&bytes);
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
            // `satisfies`/`edges` are authored on the surface, never present in the
            // source — so import leaves them empty and an unauthored rule's document
            // carries no `[satisfies.*]`/`[edge.*]` modules (import idempotence).
            satisfies: Vec::new(),
            edges: Vec::new(),
            published_requirements: Vec::new(),
            extra,
            provenance: Provenance {
                source_path: path.to_path_buf(),
                import_hash,
            },
        })
    }

    /// Reload a rule from its **one authored document** `<dir>/RULE.md`: parse the
    /// `+++`-fenced header, read the `[clause.<field>]` modules (the optional `paths`
    /// typed, unknown clauses preserved in `extra`), the `[satisfies.*]` / `[edge.*]`
    /// modules, and `[provenance]`; the body is everything below the header. The
    /// rule name is the surface directory name.
    ///
    /// The inverse of [`Rule::to_document`] over the same file: `import_hash` is read
    /// back from the provenance module, not recomputed (the surface body differs from
    /// the original, which may have carried frontmatter).
    pub fn from_dir(dir: &Path) -> Result<Self, RuleError> {
        let name = dir
            .file_name()
            .and_then(|name| name.to_str())
            .map(str::to_string)
            .ok_or_else(|| RuleError::MissingField {
                path: dir.to_path_buf(),
                field: "name",
            })?;

        let doc_path = dir.join("RULE.md");
        let raw = fs::read_to_string(&doc_path).map_err(|source| RuleError::Io {
            path: doc_path.clone(),
            source,
        })?;
        let doc = Document::parse(&raw).map_err(|source| RuleError::Document {
            path: doc_path.clone(),
            source,
        })?;
        let header = doc.header();

        // The `[clause.<field>]` modules: the optional typed `paths`, the rest
        // preserved verbatim in `extra` (the catch-all a `forbidden_keys` clause
        // ranges over — a Cursor key Claude Code ignores must survive to be caught).
        let mut paths = None;
        let mut extra = JsonMap::new();
        for (field, val) in document::clauses(header) {
            match field.as_str() {
                "paths" => paths = item_string_list(val),
                _ => {
                    if let Some(json) = toml_item_to_json(val) {
                        extra.insert(field, json);
                    }
                }
            }
        }

        let (source_path, import_hash) =
            document::provenance(header).ok_or(RuleError::MissingField {
                path: doc_path.clone(),
                field: "provenance",
            })?;

        let published_requirements =
            document::requirements(header).map_err(|source| RuleError::Document {
                path: doc_path.clone(),
                source,
            })?;

        Ok(Self {
            name,
            paths,
            body: doc.body().to_string(),
            satisfies: document::satisfies(header),
            edges: document::edges(header),
            published_requirements,
            extra,
            provenance: Provenance {
                source_path: PathBuf::from(source_path),
                import_hash,
            },
        })
    }

    /// Carry the authored surface layer (`satisfies` + `edges`) from an
    /// already-written surface artifact forward into this freshly-parsed source rule.
    /// Mirrors [`Skill::carry_representation`](crate::skill::Skill::carry_representation).
    ///
    /// The source `.md` never carries the authored clauses — they are surface-only
    /// state (`specs/20-surface.md`, "importing a member is recognizing it"). So a
    /// re-import or drifted-body `re-add`, which rebuilds the document from source,
    /// would otherwise clobber them. This is the authored layer's half of the
    /// three-state law: **merge rather than clobber** — the caller loads the existing
    /// surface, carries its authored clauses onto the re-parsed source, then writes.
    pub fn carry_representation(&mut self, existing: &Rule) {
        self.satisfies = existing.satisfies.clone();
        self.edges = existing.edges.clone();
        self.published_requirements = existing.published_requirements.clone();
    }

    /// Project the rule to its **one authored document**: a `+++`-fenced header of
    /// clause modules over the byte-faithful body (`specs/20-surface.md`, "The member
    /// document"). The header carries a `[clause.paths]` module when `paths` is
    /// present, a `[clause.<key>]` per unknown frontmatter key (sorted `extra`
    /// order), the authored `[satisfies.*]` / `[edge.*]` modules, then the generated
    /// `[provenance]` last.
    pub fn to_document(&self) -> Document {
        let mut header = DocumentMut::new();

        if let Some(paths) = &self.paths {
            let mut array = Array::new();
            for path in paths {
                array.push(path.clone());
            }
            document::add_clause(&mut header, "paths", Value::Array(array));
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

/// The string elements of a `[clause.<field>]` module's array `value`, or `None`
/// when it is absent or not an array — used to read the typed `paths` sequence back
/// off the member document.
fn item_string_list(value: &Item) -> Option<Vec<String>> {
    value.as_array().map(|array| {
        array
            .iter()
            .filter_map(|val| val.as_str().map(str::to_string))
            .collect()
    })
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

    /// Project a rule to its one member document (`<name>/RULE.md`) and reload.
    fn round_trip(name: &str, source: &str) {
        let src = tmpdir("rt-src");
        let path = write_source(&src, &format!("{name}.md"), source);
        let original = Rule::from_source_file(&path).unwrap();

        // The surface mirrors a skill: a `<name>/` dir carries ONE document. The
        // reloaded name comes from that directory name.
        let surface_root = tmpdir("rt-surface");
        let surface = surface_root.join(name);
        fs::create_dir_all(&surface).unwrap();
        fs::write(surface.join("RULE.md"), original.to_document().emit()).unwrap();

        let reloaded = Rule::from_dir(&surface).unwrap();

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
    fn authored_layer_round_trips_and_stays_out_of_frontmatter() {
        let src = tmpdir("rep-src");
        let path = write_source(&src, "rust.md", PATHS_RULE);
        let mut rule = Rule::from_source_file(&path).unwrap();

        // Author the surface-only layer — a `[satisfies.*]` (with rationale) module.
        rule.satisfies = vec![Satisfies {
            requirement: "req-rust-style".to_string(),
            rationale: Some("Encodes the Rust conventions the gate enforces.".to_string()),
        }];

        let surface_root = tmpdir("rep-surface");
        let surface = surface_root.join("rust");
        fs::create_dir_all(&surface).unwrap();
        let emitted = rule.to_document().emit();
        fs::write(surface.join("RULE.md"), &emitted).unwrap();

        let reloaded = Rule::from_dir(&surface).unwrap();

        // The authored clause survives the surface round-trip identically.
        assert_eq!(rule, reloaded);
        assert_eq!(reloaded.satisfies[0].requirement, "req-rust-style");
        assert_eq!(
            reloaded.satisfies[0].rationale.as_deref(),
            Some("Encodes the Rust conventions the gate enforces.")
        );

        // It rides a `[satisfies.*]` module, never leaking into the frontmatter
        // `extra` a `forbidden_keys` clause ranges over.
        assert!(emitted.contains("[satisfies.req-rust-style]"));
        assert!(!reloaded.extra.contains_key("satisfies"));
        assert!(!reloaded.extra.contains_key("rationale"));
    }

    #[test]
    fn unauthored_layer_leaves_the_document_without_authored_modules() {
        // A no-frontmatter rule with nothing authored carries no `[satisfies.*]` /
        // `[edge.*]` modules (import idempotence).
        let dir = tmpdir("rep-none");
        let path = write_source(&dir, "collaboration.md", NO_FRONTMATTER_RULE);
        let rule = Rule::from_source_file(&path).unwrap();

        let emitted = rule.to_document().emit();
        assert!(rule.satisfies.is_empty());
        assert!(rule.edges.is_empty());
        assert!(!emitted.contains("[satisfies."));
        assert!(!emitted.contains("[edge."));
    }

    #[test]
    fn document_is_deterministic_and_keeps_paths_and_unknown_keys_as_clauses() {
        let dir = tmpdir("doc");
        let path = write_source(&dir, "rust.md", PATHS_RULE);
        let rule = Rule::from_source_file(&path).unwrap();

        let once = rule.to_document().emit();
        let twice = rule.to_document().emit();
        assert_eq!(once, twice, "rendering must be deterministic");

        assert!(once.contains("[clause.paths]\nvalue = [\"src/**/*.rs\", \"tests/**/*.rs\"]"));
        // The preserved Cursor key rides its own clause module.
        assert!(once.contains("[clause.description]\nvalue = "));
        assert!(once.contains("[provenance]"));
        assert!(once.contains("import_hash = "));
    }
}
