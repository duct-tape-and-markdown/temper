//! The generic `yaml-frontmatter` adapter — one declaration-driven projection for
//! every kind that names it.
//!
//! Replaces the retired per-kind IRs (`src/skill.rs`, `src/rule.rs`) with one
//! [`Member`]: import-from-source splits the external artifact per
//! the kind's declared [`unit_shape`](crate::kind::UnitShape) and lifts its declared
//! `field` extractors into projection order; `emit` renders the member
//! back byte-deterministically; drift compares the declared fields with no
//! per-kind serializer. Built-in and custom kinds ride the same adapter — the
//! built-in/custom split is purely source.
//!
//! Round-trip discipline (`.claude/rules/rust.md`): the markdown body and every
//! companion are byte-faithful — never re-rendered. Only the structured header is
//! written, via `toml_edit`. Unknown frontmatter keys are preserved verbatim; the
//! declared fields lead in declaration order, then the unknown keys sorted, so the
//! projection is deterministic and `import` idempotent. `source_hash` is the SHA-256
//! of the authored source bytes, the source-drift anchor.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use gray_matter::Pod;
use gray_matter::engine::{Engine, YAML};
use serde_json::{Map as JsonMap, Value as JsonValue};

use crate::document::Satisfies;
use crate::kind::{CustomKind, UnitShape};

/// A member projected through the generic frontmatter adapter: the declared and
/// preserved frontmatter fields, a byte-faithful body, the companions that travel
/// with a directory-shaped unit, the authored `satisfies` opt-ins, and the provenance
/// lock that anchors drift. The type `import` produces from source and drift/check
/// consume — replacing the per-kind `Skill`/`Rule` IRs.
#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    /// The member id — the surface directory name (`directory` shape) or the file
    /// stem (`file` shape). The emit face's locus follows from it,
    /// never a field the member sets.
    pub id: String,
    /// The frontmatter fields in projection order: the kind's declared `field`s
    /// present, in declaration order, then the preserved unknown keys sorted. Both
    /// `[clause.<field>]` emission and the `emit` YAML projection iterate this, so
    /// they agree by construction.
    pub fields: Vec<(String, JsonValue)>,
    /// Markdown after the frontmatter, byte-faithful (trailing bytes intact). For a
    /// source carrying no frontmatter this is the whole file.
    pub body: String,
    /// The requirements this member opts into filling — authored on the surface, never imported,
    /// so a source parse leaves this empty.
    pub satisfies: Vec<Satisfies>,
    /// Where the member came from and the hash of its original bytes.
    pub provenance: Provenance,
}

/// The import lock for a member: its origin path and a content hash of the original
/// source bytes. `source_hash` drives source-drift detection.
#[derive(Debug, Clone, PartialEq)]
pub struct Provenance {
    /// Absolute or workspace-relative path to the source file.
    pub source_path: PathBuf,
    /// Lowercase hex SHA-256 of the authored source bytes.
    pub source_hash: String,
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

    /// A `named-field`-shaped source carries no value for its declared identity
    /// field (an agent with no `name`) — the third unit-shape mode's own id source,
    /// absent where `NoId`'s fixed `file`/`directory` shape labels don't fit a
    /// dynamic field name.
    #[error("{path} has no `{field}` frontmatter field to name it")]
    #[diagnostic(code(temper::frontmatter::no_named_field_id))]
    NoNamedFieldId {
        /// The path missing the declared identity field.
        path: PathBuf,
        /// The frontmatter field the id was to be read from.
        field: String,
    },

    /// A source carries a present frontmatter block that is malformed YAML, or valid
    /// YAML that is not a mapping (a bare scalar or a sequence) — so no fields can be
    /// read from it. Surfaced loud rather than degraded to an empty field map, which
    /// would let the gate judge fabricated field absence (invariant 6: loud or nothing).
    #[error("{path}: {detail}")]
    #[diagnostic(code(temper::frontmatter::malformed))]
    Malformed {
        /// The file whose frontmatter block could not be read as a mapping.
        path: PathBuf,
        /// What was wrong with the block (unparseable, a bare scalar, a sequence).
        detail: String,
    },
}

impl From<crate::hash::ReadUtf8Error> for FrontmatterError {
    fn from(err: crate::hash::ReadUtf8Error) -> Self {
        match err {
            crate::hash::ReadUtf8Error::Io { path, source } => Self::Io { path, source },
            crate::hash::ReadUtf8Error::NotUtf8 { path, source } => Self::NotUtf8 { path, source },
        }
    }
}

impl Member {
    /// Import a member from its source file, driven by the kind's declared unit shape
    /// and `field` extractors: split the YAML frontmatter, derive the id (file stem,
    /// parent directory name, or a declared frontmatter field's value), lift the
    /// declared fields then the preserved unknown keys into the projection order, scan
    /// companions (directory shape), and hash the original bytes for provenance. The
    /// authored `satisfies` layer is never in the source, so it starts empty.
    ///
    /// The body is taken byte-faithfully from after the closing frontmatter delimiter;
    /// a source with no frontmatter parses to no fields and a whole-file body — the
    /// permissive read the built-in adapters shared.
    ///
    /// # Errors
    ///
    /// Returns a [`FrontmatterError`] if the source cannot be read, is not UTF-8, carries
    /// a malformed or non-mapping frontmatter block, or yields no id for its declared
    /// shape.
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
    /// below `base` folds its placement into the id, so two
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
        let (bytes, raw) = crate::hash::read_utf8(source_file)?;
        let source_hash = crate::hash::sha256_hex(&bytes);

        let parsed = parse_frontmatter(&raw).map_err(|detail| FrontmatterError::Malformed {
            path: source_file.to_path_buf(),
            detail,
        })?;

        let id = match &kind.unit_shape {
            Some(UnitShape::Directory) => {
                let dir = source_file
                    .parent()
                    .filter(|dir| !dir.as_os_str().is_empty())
                    .ok_or_else(|| FrontmatterError::NoId {
                        path: source_file.to_path_buf(),
                        shape: "directory",
                    })?;
                dir.file_name()
                    .and_then(OsStr::to_str)
                    .ok_or_else(|| FrontmatterError::NoId {
                        path: source_file.to_path_buf(),
                        shape: "directory",
                    })?
                    .to_string()
            }
            Some(UnitShape::NamedField { field }) => parsed
                .get(field)
                .and_then(JsonValue::as_str)
                .map(str::to_string)
                .ok_or_else(|| FrontmatterError::NoNamedFieldId {
                    path: source_file.to_path_buf(),
                    field: field.clone(),
                })?,
            Some(UnitShape::StarredSegment) => {
                // A lone file keyed by its starred directory segment, coexisting inside
                // another kind's directory: it borrows the segment for identity.
                crate::extract::source_dir_name(source_file).ok_or_else(|| {
                    FrontmatterError::NoId {
                        path: source_file.to_path_buf(),
                        shape: "starred-segment",
                    }
                })?
            }
            // `file` shape, or an undeclared shape defaulting to a lone file.
            Some(UnitShape::File) | None => fold_file_id(base, source_file)?,
        };

        let fields = order_fields(&kind.declared_fields(), parsed);

        Ok(Self {
            id,
            fields,
            body: split_frontmatter(&raw).1.to_string(),
            satisfies: Vec::new(),
            provenance: Provenance {
                source_path: source_file.to_path_buf(),
                source_hash,
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
/// null) so the surface stays representable and the round-trip symmetric. A file with no
/// frontmatter block, or one whose block is empty, yields an empty map.
///
/// A present block that is malformed YAML, or valid YAML that is not a mapping (a bare
/// scalar or a sequence), is an `Err` carrying the malformation detail — never a silent
/// empty map, which would let the gate judge fabricated field absence. `gray_matter`
/// collapses a YAML parse failure to [`Pod::Null`], so a null result over non-empty input
/// reads as unparseable.
///
/// # Errors
///
/// Returns the malformation detail (without the file path — the caller attaches it) when
/// a present, non-empty block is not a YAML mapping.
fn parse_frontmatter(raw: &str) -> Result<JsonMap<String, JsonValue>, String> {
    let Some(matter) = split_frontmatter(raw).0 else {
        return Ok(JsonMap::new());
    };
    let trimmed = matter.trim();
    // An empty block is a legitimate no-fields source, not a malformation.
    if trimmed.is_empty() {
        return Ok(JsonMap::new());
    }
    let fields = match YAML::parse(trimmed) {
        Pod::Hash(hash) => hash,
        Pod::Null => {
            return Err("frontmatter block is not valid YAML, or is an explicit null".to_string());
        }
        Pod::Array(_) => {
            return Err("frontmatter block is a YAML sequence, not a mapping".to_string());
        }
        _ => return Err("frontmatter block is a bare YAML scalar, not a mapping".to_string()),
    };
    let mut out = JsonMap::new();
    for (key, pod) in fields {
        if matches!(pod, Pod::Null) {
            continue;
        }
        out.insert(key, pod.into());
    }
    Ok(out)
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
    match closing_delimiter(rest) {
        Some((matter, body)) => (Some(matter), body),
        // Opening delimiter but no close — not a frontmatter block.
        None => (None, raw),
    }
}

/// Scan `rest` — the text after an opening `---\n` line — for the closing `---`
/// delimiter line, returning the frontmatter matter and the byte-faithful body split
/// around it. `None` when no line is a bare `---` (an opening delimiter with no
/// close, so not a frontmatter block after all).
///
/// `pub(crate)` so `src/install.rs`'s modeline/note projectors and
/// `placement_lines` share the same closing-delimiter scan the loaders use,
/// rather than a second implementation that could drift.
pub(crate) fn closing_delimiter(rest: &str) -> Option<(&str, &str)> {
    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        let content = line.strip_suffix('\n').unwrap_or(line);
        if content.trim_end() == "---" {
            let matter = &rest[..offset];
            let body = &rest[offset + line.len()..];
            return Some((matter, body));
        }
        offset += line.len();
    }
    None
}

/// Strip a `---\n` opening frontmatter delimiter and scan for the closing `---`,
/// returning everything after the opening delimiter and the frontmatter matter.
/// Wraps [`closing_delimiter`] to consolidate the split-frontmatter pattern used
/// by `src/install.rs`'s projectors and `src/placement.rs`'s placement line scanner.
pub(crate) fn frontmatter_matter(source: &str) -> Option<(&str, &str)> {
    let rest = source.strip_prefix("---\n")?;
    closing_delimiter(rest).map(|(matter, _)| (rest, matter))
}

/// Derive a **file-shaped** unit's surface id, folding the directory placement below
/// the `governs`-root directory `base` into it. A unit
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
pub fn fold_file_id(base: &Path, source_file: &Path) -> Result<String, FrontmatterError> {
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

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::test_support::{rule_kind, skill_kind, tmpdir};

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
    fn a_non_mapping_frontmatter_block_is_a_loud_error_not_empty_fields() {
        let dir = tmpdir("non-mapping-frontmatter");
        fs::write(
            dir.join("collaboration.md"),
            "---\njust a bare scalar\n---\n# Collab\n\nBody.\n",
        )
        .unwrap();

        let err = Member::from_source(&rule_kind(), &dir.join("collaboration.md")).unwrap_err();
        assert!(matches!(err, FrontmatterError::Malformed { .. }));
        // Names the malformation, not a fabricated missing field.
        assert!(err.to_string().contains("not a mapping"));
    }

    #[test]
    fn a_malformed_yaml_frontmatter_block_is_a_loud_error() {
        let dir = tmpdir("malformed-yaml-frontmatter");
        // An unterminated flow sequence is not valid YAML; `gray_matter` collapses the
        // parse failure to a null Pod, which must surface loud rather than as no fields.
        fs::write(
            dir.join("rust.md"),
            "---\npaths: [\"unterminated\n---\n# Rust\n\nBody.\n",
        )
        .unwrap();

        let err = Member::from_source(&rule_kind(), &dir.join("rust.md")).unwrap_err();
        assert!(matches!(err, FrontmatterError::Malformed { .. }));
    }

    #[test]
    fn an_empty_frontmatter_block_is_no_fields_not_an_error() {
        let dir = tmpdir("empty-frontmatter");
        fs::write(
            dir.join("collaboration.md"),
            "---\n---\n# Collab\n\nBody.\n",
        )
        .unwrap();

        let member = Member::from_source(&rule_kind(), &dir.join("collaboration.md")).unwrap();
        assert!(member.fields.is_empty());
        assert_eq!(member.body, "# Collab\n\nBody.\n");
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

    #[test]
    fn frontmatter_matter_returns_rest_and_matter_for_well_formed_source() {
        let source = "---\nname: test\n---\n# Body\n";
        let (rest, matter) = frontmatter_matter(source).unwrap();
        assert_eq!(rest, "name: test\n---\n# Body\n");
        assert_eq!(matter, "name: test\n");
    }

    #[test]
    fn frontmatter_matter_returns_none_for_missing_opening_delimiter() {
        let source = "# No frontmatter\n";
        assert_eq!(frontmatter_matter(source), None);
    }

    #[test]
    fn frontmatter_matter_returns_none_for_unterminated_frontmatter() {
        let source = "---\nname: test\n";
        assert_eq!(frontmatter_matter(source), None);
    }
}
