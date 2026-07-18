//! The `toml-document` read face — a member whose whole artifact is one TOML table.
//!
//! `json_manifest.rs`'s [`DocumentMember`] read over a second grammar: no collection
//! address, so every top-level key is the member's own field and identity is a declared key
//! among them. The member shape, its identity rule's currency, and `to_unit` are that
//! module's; only the grammar is here, and `toml_edit` owns it.
//!
//! **Read-only, and that is the face — not a stage.** No `write_document` twin exists and
//! none is coming: a member of this format is read and gated in place, never projected
//! (decision 0034). `emit` refuses a member declaring it rather than reaching for a writer
//! that is not there.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as JsonValue;
use toml_edit::DocumentMut;

use crate::document::item_to_json;
use crate::frontmatter::Provenance;
use crate::json_manifest::DocumentMember;
use crate::kind::{CustomKind, UnitShape};

/// Read the TOML document at `source_file` as one member of `kind`, its identity taken from
/// the top-level key the kind's [`UnitShape::NamedField`] declares.
///
/// # Errors
///
/// Returns a [`TomlDocumentError`] if the file cannot be read, is not UTF-8, is not valid
/// TOML, or carries no string value at the declared identity key; and
/// [`TomlDocumentError::NoDeclaredIdentity`] if `kind` declares no identity field for its
/// document to be named by.
pub fn read(kind: &CustomKind, source_file: &Path) -> Result<DocumentMember, TomlDocumentError> {
    let bytes = fs::read(source_file).map_err(|source| TomlDocumentError::Io {
        path: source_file.to_path_buf(),
        source,
    })?;
    let raw = String::from_utf8(bytes).map_err(|source| TomlDocumentError::NotUtf8 {
        path: source_file.to_path_buf(),
        source,
    })?;
    parse(kind, source_file, &raw)
}

/// Read a document straight from its `raw` text rather than off disk — the split
/// [`crate::json_manifest::DocumentMember::parse`] takes, for the same reason: pending
/// content is read through the one soundness boundary the disk read rides. `source_file`
/// labels the provenance and any diagnostic; nothing is read from it.
///
/// # Errors
///
/// As [`read`], less the I/O and UTF-8 failures `raw` has already passed.
pub fn parse(
    kind: &CustomKind,
    source_file: &Path,
    raw: &str,
) -> Result<DocumentMember, TomlDocumentError> {
    let field = match &kind.unit_shape {
        Some(UnitShape::NamedField { field }) => field,
        Some(UnitShape::File)
        | Some(UnitShape::Directory)
        | Some(UnitShape::StarredSegment)
        | None => {
            return Err(TomlDocumentError::NoDeclaredIdentity {
                path: source_file.to_path_buf(),
                kind: kind.name.clone(),
            });
        }
    };

    let source_hash = crate::hash::sha256_hex(raw.as_bytes());
    let document: DocumentMut =
        raw.parse()
            .map_err(|err: toml_edit::TomlError| TomlDocumentError::Malformed {
                path: source_file.to_path_buf(),
                detail: err.to_string(),
            })?;

    // TOML's top level is a table by construction, so this face has no non-object
    // counterpart to the JSON one's refusal — a document that would not be a table is
    // already a parse error above.
    let fields: std::collections::BTreeMap<String, JsonValue> = document
        .as_table()
        .iter()
        .filter_map(|(key, item)| item_to_json(item).map(|json| (key.to_string(), json)))
        .collect();

    let id = fields
        .get(field)
        .and_then(JsonValue::as_str)
        .map(str::to_string)
        .ok_or_else(|| TomlDocumentError::NoIdentityValue {
            path: source_file.to_path_buf(),
            field: field.clone(),
        })?;

    Ok(DocumentMember {
        id,
        fields,
        provenance: Provenance {
            source_path: source_file.to_path_buf(),
            source_hash,
        },
    })
}

/// Errors raised while reading a `toml-document` member. Hard failures (missing file,
/// non-UTF-8, malformed TOML) — distinct from a lint `Diagnostic`, which the engine collects
/// rather than throws. The peer of [`crate::json_manifest::JsonManifestError`]'s document
/// arm, in this face's own vocabulary.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum TomlDocumentError {
    /// A document file could not be read.
    #[error("failed to read {path}")]
    #[diagnostic(code(temper::toml_document::io))]
    Io {
        /// The offending file.
        path: PathBuf,
        /// The underlying I/O failure.
        #[source]
        source: std::io::Error,
    },

    /// A document file is not valid UTF-8, so its text cannot be parsed.
    #[error("{path} is not valid UTF-8")]
    #[diagnostic(code(temper::toml_document::not_utf8))]
    NotUtf8 {
        /// The offending file.
        path: PathBuf,
        /// The decode error.
        #[source]
        source: std::string::FromUtf8Error,
    },

    /// A document's text is not valid TOML, so no field can be read. Surfaced loud rather
    /// than degraded to an empty read, which would let the gate judge fabricated absence
    /// (invariant 6: loud or nothing).
    #[error("{path}: {detail}")]
    #[diagnostic(code(temper::toml_document::malformed))]
    Malformed {
        /// The file whose TOML could not be parsed.
        path: PathBuf,
        /// What was wrong, as the parser reported it.
        detail: String,
    },

    /// A `toml-document` kind declares no `named-field` unit shape, so its documents have no
    /// declared key to be named by — a document's identity is read from a declared field,
    /// never derived from the path. Refused at load rather than guessed.
    #[error("kind `{kind}` declares `toml-document` with no `named-field` identity")]
    #[diagnostic(code(temper::toml_document::no_declared_identity))]
    NoDeclaredIdentity {
        /// The document whose kind names no identity field.
        path: PathBuf,
        /// The kind missing the declaration.
        kind: String,
    },

    /// A TOML document carries no string value at its kind's declared identity key.
    #[error("{path} has no `{field}` key to name it")]
    #[diagnostic(code(temper::toml_document::no_identity_value))]
    NoIdentityValue {
        /// The document missing the declared identity key.
        path: PathBuf,
        /// The top-level key the id was to be read from.
        field: String,
    },
}
