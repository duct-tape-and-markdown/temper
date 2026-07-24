//! The generic JSON-manifest adapter — `frontmatter.rs`'s peer for structured config.
//!
//! Where the frontmatter adapter reads a markdown artifact's YAML header, this one reads
//! a structured manifest (`settings.json`, `.mcp.json`): a real JSON parser owns the
//! grammar, and a kind's declared collection address selects which key paths walk into
//! the generic surface extractor (`crate::extract`). Each entry at a declared address
//! reads as a fields-only registration member (a hook, an MCP server, an installed
//! plugin) — three entry shapes: an array of matcher groups, an entry object whose keys
//! fold in, and a bare scalar carried as one declared field. Every top-level
//! key no address consumed survives as an opaque field of the container. Reading an
//! unrepresented manifest still infers its registration members off the addresses handed
//! in — the file need not be modelled as a member for its members to surface.
//!
//! A `json-document` kind ([`DocumentMember`]) inverts that read: it declares no address,
//! so the member owns the whole document — every top-level key its own field, its identity
//! a declared key among them.
//!
//! The write faces mirror the reads: [`write_manifest`] regenerates a represented manifest
//! whole (declared collection segments in address order, then the opaque residue), and
//! [`write_document`] renders one member's fields back as the whole document — both
//! canonical 2-space-pretty, LF-terminated. The unrepresented-manifest write stays
//! `src/json_splice.rs`, splicing in place. Every face here is a pure function of its
//! inputs, so a re-read and a double-emit are each byte-identical, the idempotence keystone
//! the frontmatter face also holds.

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use serde_json::{Map as JsonMap, Value as JsonValue};

use crate::extract::{self, FeatureValue};
use crate::frontmatter::Provenance;
use crate::kind::{CollectionAddress, CustomKind, Unit, UnitShape};

thread_local! {
    static MANIFEST_READ_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// This thread's cumulative count of manifest file read operations. Pinned by count-tests
/// to enforce the cost doctrine: manifest files are read exactly once per run, never once
/// per governing kind (GATE-MANIFEST-SHARED-READ-HOIST, pinned by
/// `check_cost.rs::gate_manifest_cache_read_is_hoisted_across_governing_kinds`).
#[must_use]
pub fn manifest_read_count() -> usize {
    MANIFEST_READ_COUNT.with(std::cell::Cell::get)
}

/// A JSON manifest read through the adapter's read face: the registration members
/// inferred at its declared collection addresses, the opaque field residue no address
/// consumed, and the provenance lock the idempotence keystone rests on. The read-side
/// peer of [`crate::frontmatter::Member`] for structured config.
#[derive(Debug, Clone, PartialEq)]
pub struct Manifest {
    /// The registration members inferred at the manifest's declared collection
    /// addresses — one per collection entry, in address order then each collection's own
    /// sorted key order.
    pub members: Vec<RegistrationMember>,
    /// The container member's **opaque field residue** — every top-level manifest key no
    /// collection address consumed, projected kind-preserving and kept as an opaque
    /// field, named as such. Sorted by key (`serde_json::Map` is a `BTreeMap`).
    pub opaque_fields: BTreeMap<String, FeatureValue>,
    /// Where the manifest came from and the hash of its original bytes — the source-drift
    /// anchor, exactly as a frontmatter member's [`Provenance`] carries.
    pub provenance: Provenance,
}

/// One fields-only registration member read off a manifest's collection: the collection
/// it surfaces in (the top-level key its address walked into), its key among that
/// collection's entries, and its own typed fields — the same [`FeatureValue`] currency a
/// frontmatter member's fields carry.
#[derive(Debug, Clone, PartialEq)]
pub struct RegistrationMember {
    /// The manifest collection this member surfaces in (`hooks`, `mcpServers`) — the
    /// top-level key its declared address walked into.
    pub collection: String,
    /// The member's key among its collection's entries — the entry name (an MCP server's
    /// name, a hook's lifecycle event).
    pub key: String,
    /// The member's own fields, key → **raw** [`JsonValue`], in sorted key order — kept
    /// unprojected so the one shared read-time fold ([`crate::builtin_kind::features`])
    /// types them exactly as a frontmatter member's fields, never a second projector.
    /// Declared and opaque keys alike surface here: a fields-only member keeps every entry
    /// key, the same permissive read the frontmatter face gives unknown keys. A hook's
    /// event value is an array, so its member carries no fields; an MCP server's entry is
    /// an object, so its fields fold in; an installed plugin's entry is a bare scalar, so
    /// its member carries that one value as its declared `enabled` field.
    pub fields: BTreeMap<String, JsonValue>,
}

impl RegistrationMember {
    /// This registration member as a raw [`Unit`] for the shared extraction: its own object
    /// fields become the unit's frontmatter, and the collection key surfaces under the
    /// address's key field where it names one (`hooks.<Event>` → `event`), never
    /// overwriting a same-named object field. The one member→unit mapping the gate's
    /// manifest read and the write guard both run, so neither can disagree about the fields
    /// a clause ranges over. `body`/`satisfies` are empty — a fields-only member carries
    /// neither, and the caller folds any `satisfies` off the lock.
    #[must_use]
    pub fn to_unit(&self, address: &CollectionAddress, source_path: &Path) -> Unit {
        let mut frontmatter = self.fields.clone();
        if let Some(field) = address.key_path.key_field() {
            frontmatter
                .entry(field.to_string())
                .or_insert_with(|| JsonValue::String(self.key.clone()));
        }
        // A composite key's edge half surfaces as its own field, so a declared edge over it
        // (an installed plugin's `<plugin>@<marketplace>` → `known-marketplace`) resolves off
        // the member's features like any frontmatter reference — never overwriting a
        // same-named object field.
        if let Some((field, value)) = address.key_path.identity_edge(&self.key) {
            frontmatter
                .entry(field.to_string())
                .or_insert_with(|| JsonValue::String(value));
        }
        Unit {
            id: self.key.clone(),
            frontmatter,
            body: String::new(),
            source_path: source_path.to_path_buf(),
            satisfies: Vec::new(),
            satisfies_clauses: Vec::new(),
        }
    }
}

/// One member whose **whole artifact is one JSON object** — the `json-document` format's
/// read face. Where a [`Manifest`] read walks declared collection addresses into a host
/// document and keeps the rest as opaque residue, a document member claims the document:
/// no address, so every top-level key is the member's own field, and identity is a
/// declared key among them rather than the filename stem. The JSON peer of
/// [`crate::frontmatter::Member`] for a kind whose fields have no markdown body to ride.
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentMember {
    /// The member id, read off the document's declared identity key.
    pub id: String,
    /// The member's fields — every top-level key of the document, raw and in sorted key
    /// order, kept unprojected so the one shared read-time fold
    /// ([`crate::builtin_kind::features`]) types them exactly as a frontmatter member's.
    pub fields: BTreeMap<String, JsonValue>,
    /// Where the document came from and the hash of its original bytes — the same
    /// source-drift anchor a [`Manifest`] read carries.
    pub provenance: Provenance,
}

impl DocumentMember {
    /// Read the JSON document at `source_file` as one member of `kind`, its identity taken
    /// from the top-level key the kind's [`UnitShape::NamedField`] declares, or from the
    /// file stem for a singleton [`UnitShape::File`] document (a `settings.local.json`).
    ///
    /// # Errors
    ///
    /// Returns a [`JsonManifestError`] if the file cannot be read, is not UTF-8, is not a
    /// top-level JSON object, or carries no string value at the declared identity key; and
    /// [`JsonManifestError::NoDeclaredIdentity`] if `kind` declares an identity mode a JSON
    /// document cannot serve (`directory`/`starred-segment`).
    pub fn read(kind: &CustomKind, source_file: &Path) -> Result<Self, JsonManifestError> {
        let raw = read_to_string(source_file)?;
        Self::parse(kind, source_file, &raw)
    }

    /// Read a document straight from its `raw` bytes rather than off disk — the split
    /// [`Manifest::parse`] takes, for the same reason: an in-flight write's pending content
    /// is read through the one soundness boundary the disk read rides. `source_file` labels
    /// the provenance and any diagnostic; nothing is read from it.
    ///
    /// # Errors
    ///
    /// As [`read`](Self::read), less the I/O and UTF-8 failures `raw` has already passed.
    pub fn parse(
        kind: &CustomKind,
        source_file: &Path,
        raw: &str,
    ) -> Result<Self, JsonManifestError> {
        let (source_hash, document) = parse_top_level_object(source_file, raw, "document")?;

        // A named-field document reads its id from a declared top-level key (a manifest's
        // `name`); a `file`-shaped one is a singleton at a fixed path, so its identity is
        // the file stem (`settings.local`) — the same source a frontmatter `file` member
        // takes. Every other shape names no identity a whole-JSON document can carry.
        let id = match &kind.unit_shape {
            Some(UnitShape::NamedField { field }) => document
                .get(field)
                .and_then(JsonValue::as_str)
                .map(str::to_string)
                .ok_or_else(|| JsonManifestError::NoIdentityValue {
                    path: source_file.to_path_buf(),
                    field: field.clone(),
                })?,
            Some(UnitShape::File) | None => source_file
                .file_stem()
                .and_then(OsStr::to_str)
                .map(str::to_string)
                .ok_or_else(|| JsonManifestError::NoStemIdentity {
                    path: source_file.to_path_buf(),
                })?,
            Some(UnitShape::Directory) | Some(UnitShape::StarredSegment) => {
                return Err(JsonManifestError::NoDeclaredIdentity {
                    path: source_file.to_path_buf(),
                    kind: kind.name.clone(),
                });
            }
        };

        Ok(Self {
            id,
            fields: document.into_iter().collect(),
            provenance: Provenance {
                source_path: source_file.to_path_buf(),
                source_hash,
            },
        })
    }

    /// This document member as a raw [`Unit`] for the shared extraction: its top-level keys
    /// become the unit's frontmatter, so a clause ranges over a JSON document's fields
    /// exactly as it ranges over a frontmatter member's. `body` is empty — the document is
    /// all fields, no prose — and `satisfies` is left to the caller's lock fold, as for
    /// every other member.
    #[must_use]
    pub fn to_unit(&self) -> Unit {
        Unit {
            id: self.id.clone(),
            frontmatter: self.fields.clone(),
            body: String::new(),
            source_path: self.provenance.source_path.clone(),
            satisfies: Vec::new(),
            satisfies_clauses: Vec::new(),
        }
    }
}

/// Errors raised while reading a [`Manifest`] or a [`DocumentMember`]. Hard failures
/// (missing file, non-UTF-8, malformed JSON) — distinct from a lint `Diagnostic`, which the
/// engine collects rather than throws. Mirrors [`crate::frontmatter::FrontmatterError`]'s
/// shape for the JSON face.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum JsonManifestError {
    /// A manifest file could not be read.
    #[error("failed to read {path}")]
    #[diagnostic(code(temper::json_manifest::io))]
    Io {
        /// The path that failed to read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// A manifest file is not valid UTF-8, so its text cannot be parsed.
    #[error("{path} is not valid UTF-8")]
    #[diagnostic(code(temper::json_manifest::not_utf8))]
    NotUtf8 {
        /// The offending file.
        path: PathBuf,
        /// The decode error.
        #[source]
        source: std::string::FromUtf8Error,
    },

    /// A manifest's text is not valid JSON, or is valid JSON that is not a top-level
    /// object (a manifest is always a `{...}` of keys) — so no collection address can be
    /// walked and no field read. Surfaced loud rather than degraded to an empty read,
    /// which would let the gate judge fabricated absence (invariant 6: loud or nothing).
    #[error("{path}: {detail}")]
    #[diagnostic(code(temper::json_manifest::malformed))]
    Malformed {
        /// The file whose JSON could not be read as an object.
        path: PathBuf,
        /// What was wrong (a parse error, or a non-object top level).
        detail: String,
    },

    /// A `json-document` kind declares a `directory`/`starred-segment` unit shape, which a
    /// whole-JSON document cannot serve — its identity is a declared top-level key
    /// (`named-field`) or the file stem (`file`), never a directory or a glob segment.
    /// Refused at load rather than guessed.
    #[error("kind `{kind}` declares `json-document` with an identity shape it cannot serve")]
    #[diagnostic(code(temper::json_manifest::no_declared_identity))]
    NoDeclaredIdentity {
        /// The document whose kind names no identity field.
        path: PathBuf,
        /// The kind missing the declaration.
        kind: String,
    },

    /// A `file`-shaped JSON document's path yields no stem to name it — the [`DocumentMember`]
    /// peer of [`crate::frontmatter::FrontmatterError::NoId`] for a singleton document.
    #[error("{path} has no file stem to name the document")]
    #[diagnostic(code(temper::json_manifest::no_stem_identity))]
    NoStemIdentity {
        /// The document whose path yields no stem.
        path: PathBuf,
    },

    /// A JSON document carries no string value at its kind's declared identity key — the
    /// [`DocumentMember`] peer of [`crate::frontmatter::FrontmatterError::NoNamedFieldId`].
    #[error("{path} has no `{field}` key to name it")]
    #[diagnostic(code(temper::json_manifest::no_identity_value))]
    NoIdentityValue {
        /// The document missing the declared identity key.
        path: PathBuf,
        /// The top-level key the id was to be read from.
        field: String,
    },
}

impl From<crate::hash::ReadUtf8Error> for JsonManifestError {
    fn from(err: crate::hash::ReadUtf8Error) -> Self {
        match err {
            crate::hash::ReadUtf8Error::Io { path, source } => Self::Io { path, source },
            crate::hash::ReadUtf8Error::NotUtf8 { path, source } => Self::NotUtf8 { path, source },
        }
    }
}

/// Read a file and decode it as UTF-8, mapping I/O and encoding errors to [`JsonManifestError`].
/// Shared by [`Manifest::read`] and [`DocumentMember::read`].
fn read_to_string(source_file: &Path) -> Result<String, JsonManifestError> {
    let (_bytes, string) = crate::hash::read_utf8(source_file)?;
    Ok(string)
}

/// Parse JSON bytes and require a top-level object, returning the source hash and the parsed
/// object map. The noun parameter is used in error messages to distinguish the context
/// (e.g., "document" vs "manifest").
fn parse_top_level_object(
    source_file: &Path,
    raw: &str,
    noun: &str,
) -> Result<(String, serde_json::Map<String, JsonValue>), JsonManifestError> {
    let source_hash = crate::hash::sha256_hex(raw.as_bytes());
    let value: JsonValue =
        serde_json::from_str(raw).map_err(|err| JsonManifestError::Malformed {
            path: source_file.to_path_buf(),
            detail: err.to_string(),
        })?;
    let JsonValue::Object(object) = value else {
        return Err(JsonManifestError::Malformed {
            path: source_file.to_path_buf(),
            detail: format!("{noun} top level is not a JSON object"),
        });
    };
    Ok((source_hash, object))
}

impl Manifest {
    /// Read the JSON manifest at `source_file`, walking each of `addresses` into its
    /// declared collection to infer that collection's registration members, and keeping
    /// every top-level key no address consumed as an opaque field. A real JSON parser
    /// (`serde_json`) owns the grammar; the per-entry field projection rides the shared
    /// surface extractor ([`manifest_members`]), so the manifest read shares the
    /// one soundness boundary the frontmatter path rides.
    ///
    /// `addresses` are the collection addresses that target *this* manifest — from the
    /// kinds that declare it. An empty set reads the whole manifest as opaque fields (no
    /// collection is claimed), which is exactly the unrepresented case: nothing to infer,
    /// every key opaque.
    ///
    /// # Errors
    ///
    /// Returns a [`JsonManifestError`] if the file cannot be read, is not UTF-8, or is not
    /// a top-level JSON object.
    pub fn read(
        source_file: &Path,
        addresses: &[&CollectionAddress],
    ) -> Result<Self, JsonManifestError> {
        MANIFEST_READ_COUNT.with(|c| c.set(c.get() + 1));
        let raw = read_to_string(source_file)?;
        Self::parse(source_file, &raw, addresses)
    }

    /// Read a manifest straight from its `raw` bytes rather than off disk — the read
    /// [`read`](Self::read) runs once it has decoded the file, split out so an in-flight
    /// write's pending content is inferred through the one soundness boundary the disk
    /// read rides (the `PreToolUse` guard checks a manifest before it lands). `source_file`
    /// labels the provenance and any diagnostic; nothing is read from it.
    ///
    /// # Errors
    ///
    /// Returns a [`JsonManifestError::Malformed`] if `raw` is not a top-level JSON object.
    pub fn parse(
        source_file: &Path,
        raw: &str,
        addresses: &[&CollectionAddress],
    ) -> Result<Self, JsonManifestError> {
        let (source_hash, manifest) = parse_top_level_object(source_file, raw, "manifest")?;

        // Each declared address walks into its collection; the top-level keys it consumes
        // are tracked so the residue that stays opaque excludes them.
        let mut members = Vec::new();
        let mut consumed = BTreeSet::new();
        for address in addresses {
            let collection = address.key_path.collection_key();
            consumed.insert(collection);
            for (key, fields) in manifest_members(&manifest, collection, &address.entry_shape) {
                members.push(RegistrationMember {
                    collection: collection.to_string(),
                    key,
                    fields,
                });
            }
        }

        let opaque_fields = manifest
            .iter()
            .filter(|(key, _)| !consumed.contains(key.as_str()))
            .map(|(key, value)| (key.clone(), extract::json_to_feature(value)))
            .collect();

        Ok(Self {
            members,
            opaque_fields,
            provenance: Provenance {
                source_path: source_file.to_path_buf(),
                source_hash,
            },
        })
    }

    /// Read a **manifest kind**'s members off harness disk — the loader dispatch a kind
    /// with a declared [`collection_address`](CustomKind::collection_address) takes
    /// instead of the frontmatter loader. Takes an already-discovered list of manifest
    /// files (discovered by the caller), then reads each through [`Manifest::read`] against
    /// the kind's own declared address. A kind with no collection address is not a
    /// manifest kind — the caller routes it to the frontmatter loader — so this yields no
    /// reads for it.
    ///
    /// # Errors
    ///
    /// Returns a [`JsonManifestError`] if a discovered manifest cannot be read.
    pub fn read_kind(files: &[PathBuf], kind: &CustomKind) -> Result<Vec<Self>, JsonManifestError> {
        let Some(address) = &kind.collection_address else {
            return Ok(Vec::new());
        };
        files
            .iter()
            .map(|file| Manifest::read(file, &[address]))
            .collect()
    }
}

/// One collection segment of a manifest's write face: a declared collection address's
/// top-level key and the entries that surface there, each an entry key paired with the
/// whole JSON value it holds — an MCP server's object, a hook event's array. Entries ride
/// a `BTreeMap`, so a re-emit lands them in the collection's own sorted key order, the
/// order the read face surfaces them in.
#[derive(Debug, Clone, PartialEq)]
pub struct CollectionSegment {
    /// The collection's top-level manifest key (`hooks`, `mcpServers`).
    pub collection_key: String,
    /// The collection's entries: entry key → the whole JSON value it holds.
    pub entries: BTreeMap<String, JsonValue>,
}

/// Regenerate a represented manifest whole: its `segments` in the order given (declared
/// collection-address order), then the opaque `residue` in sorted key order, serialized
/// as canonical 2-space-pretty JSON with a trailing LF. The byte shape
/// [`serde_json::to_string_pretty`] produces — the one encoder `bundle`'s manifests also
/// ride — but with the top-level key order the manifest declares (collections then
/// residue), which serde_json's own sorted-map serialization cannot express. A pure
/// function of its inputs, so a double-emit is byte-identical.
///
/// This is the represented-manifest path; an unrepresented manifest stays on the
/// `json_splice` text splicer, which edits a human's document in place rather than
/// regenerating it. Each value is rendered through the one shared pretty printer
/// ([`crate::json_splice::pretty_at`]), never a second encoder.
#[must_use]
pub fn write_manifest(
    segments: &[CollectionSegment],
    residue: &BTreeMap<String, JsonValue>,
) -> String {
    let mut members: Vec<(&str, JsonValue)> = Vec::with_capacity(segments.len() + residue.len());
    for segment in segments {
        let object: serde_json::Map<String, JsonValue> =
            segment.entries.clone().into_iter().collect();
        members.push((segment.collection_key.as_str(), JsonValue::Object(object)));
    }
    for (key, value) in residue {
        members.push((key.as_str(), value.clone()));
    }
    render_object(&members)
}

/// Render one member's `fields` back as the whole JSON document — the `json-document`
/// format's write face, [`write_manifest`]'s peer for a member that owns its file rather
/// than surfacing in a collection of one. The same canonical 2-space-pretty encoder with a
/// trailing LF, keys in the sorted order the read face surfaces them in. A pure function of
/// its input, so a double-emit is byte-identical and a canonical document round-trips
/// read→write unchanged.
#[must_use]
pub fn write_document(fields: &BTreeMap<String, JsonValue>) -> String {
    let members: Vec<(&str, JsonValue)> = fields
        .iter()
        .map(|(key, value)| (key.as_str(), value.clone()))
        .collect();
    render_object(&members)
}

/// Serialize an ordered list of top-level `(key, value)` members as canonical
/// 2-space-pretty JSON with a trailing LF — the outer object framing built here so the
/// keys land in the given order (serde_json's own map serialization would sort them),
/// each value rendered by the shared pretty printer at one indent level. An empty
/// manifest is `{}\n`.
fn render_object(members: &[(&str, JsonValue)]) -> String {
    if members.is_empty() {
        return "{}\n".to_string();
    }
    let mut out = String::from("{\n");
    for (index, (key, value)) in members.iter().enumerate() {
        // A plain string key serializes infallibly; fall back to an empty-string literal
        // rather than panic on the unreachable error path.
        let key_text = serde_json::to_string(key).unwrap_or_else(|_| "\"\"".to_string());
        out.push_str("  ");
        out.push_str(&key_text);
        out.push_str(": ");
        out.push_str(&crate::json_splice::pretty_at(value, "  "));
        if index + 1 < members.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("}\n");
    out
}

/// Read a manifest collection's members: each entry unwraps into a member's raw fields
/// under the manifest format. Dispatches on the collection's declared entry-shape:
///
/// - **Object shape** — entries decode as objects — their key/value pairs carry directly
///   (via [`entry_fields`]); a non-object entry passes no fields.
/// - **Scalar shape** — each entry's value is a bare scalar — lifts it as one declared
///   field (via [`enablement_member_fields`]).
/// - **GroupArray shape** — an array of matcher groups — decomposes each group into flat
///   lifted fields plus handler fields, one member per handler (via [`hook_member_fields`]).
///
/// The read and write faces ride this grammar — that invariant is the answer to the
/// lock-row lifter's contract-edge challenge (decision 0040): the grammar is the one
/// channel, the divergence the write face mirrors.
fn manifest_members(
    manifest: &JsonMap<String, JsonValue>,
    collection_key: &str,
    entry_shape: &crate::kind::EntryShape,
) -> Vec<(String, BTreeMap<String, JsonValue>)> {
    let Some(JsonValue::Object(collection)) = manifest.get(collection_key) else {
        return Vec::new();
    };
    match entry_shape {
        crate::kind::EntryShape::GroupArray {
            member_key,
            lifted_fields,
        } => collection
            .iter()
            .flat_map(|(event, value)| {
                hook_member_fields(value, member_key, lifted_fields)
                    .into_iter()
                    .map(move |fields| (event.clone(), fields))
            })
            .collect(),
        crate::kind::EntryShape::Scalar { field } => collection
            .iter()
            .map(|(plugin, value)| (plugin.clone(), enablement_member_fields(value, field)))
            .collect(),
        crate::kind::EntryShape::Object => collection
            .iter()
            .map(|(key, value)| (key.clone(), entry_fields(value)))
            .collect(),
    }
}

/// One registration entry's raw fields: an object's members carried verbatim as JSON; a
/// non-object value yields no fields, since it holds no key/value pairs a fields-only
/// member reads. Projection to [`crate::extract::FeatureValue`] is deferred to the shared read-time fold,
/// so a manifest member and a frontmatter member type their fields through the one path.
fn entry_fields(value: &JsonValue) -> BTreeMap<String, JsonValue> {
    match value {
        JsonValue::Object(fields) => fields
            .iter()
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect(),
        _ => BTreeMap::new(),
    }
}

/// Decompose one scalar-shape entry value into the fields its member carries. The
/// **scalar** entry shape: where a group-array's value is an array of matcher groups
/// and an object shape's entry is an object whose keys fold in, a scalar entry's value
/// is a bare scalar — Claude Code "writes `true` for an enablement" at install or enable
/// time (`code.claude.com/docs/en/plugins-reference`, "Default enablement", retrieved
/// 2026-07-16), and schemastore's `claude-code-settings.json` types `enabledPlugins` an
/// object whose values `anyOf` a boolean, a string array, or nothing (retrieved
/// 2026-07-16). So the member has no object to fold: it carries the entry's whole value
/// as its one declared `field` (the shape parameter).
///
/// The value is carried **whatever its JSON type** — the same permissive read the
/// frontmatter face gives an unknown key. Only the documented boolean has documented
/// semantics; schemastore's string-array alternative is admitted by the schema and
/// explained by no prose, so surfacing it as a field infers nothing, where rejecting it
/// would forge a finding on a manifest the format admits. The inverse of
/// [`enablement_entry_value`], so an entry read off `settings.json` re-renders to the
/// identical bytes on write.
fn enablement_member_fields(entry_value: &JsonValue, field: &str) -> BTreeMap<String, JsonValue> {
    BTreeMap::from([(field.to_string(), entry_value.clone())])
}

/// Render one scalar-shape member's fields back to its entry value — the bare scalar
/// the wire carries, read straight off the declared `field`. The inverse of
/// [`enablement_member_fields`], and the reason emit writes the scalar Claude Code loads
/// rather than the `{field: …}` object a naive entry-object write would land.
///
/// A member declaring no `field` renders `true`: the documented value Claude Code
/// itself writes when it enables a plugin (same source), so an entry authored at all reads
/// as the enablement it spells. Any other field a member carries has no home on the wire —
/// the entry's value is one scalar — and is dropped rather than invented into a shape the
/// format does not document.
pub(crate) fn enablement_entry_value(fields: &[(String, JsonValue)], field: &str) -> JsonValue {
    fields
        .iter()
        .find(|(key, _)| key == field)
        .map_or(JsonValue::Bool(true), |(_, value)| value.clone())
}

/// Decompose one group-array entry value — Claude Code's array of matcher groups
/// (`[{matcher?, hooks:[{type, command}]}]`, code.claude.com/docs/en/hooks) — into the
/// flat fields a fields-only member carries, one per handler: each group's lifted fields
/// (when present) alongside each handler object's own keys (`type`, `command`, …). The
/// `member_key` is the key of the handler array within each group (e.g., `"hooks"`), and
/// `lifted_fields` are the names of fields to lift from the group level to each handler's
/// flat representation (e.g., `["matcher"]`). The inverse of [`hook_matcher_group`], so
/// an entry read back off `settings.json` re-nests to the identical bytes on write. A value
/// that is not an array, a group that is not an object, or an entry with no handler array
/// yields no member — a shape Claude Code would itself ignore infers nothing.
fn hook_member_fields(
    event_value: &JsonValue,
    member_key: &str,
    lifted_fields: &[String],
) -> Vec<BTreeMap<String, JsonValue>> {
    let JsonValue::Array(groups) = event_value else {
        return Vec::new();
    };
    let mut members = Vec::new();
    for group in groups {
        let JsonValue::Object(group) = group else {
            continue;
        };
        let Some(JsonValue::Array(handlers)) = group.get(member_key) else {
            continue;
        };
        for handler in handlers {
            let JsonValue::Object(handler) = handler else {
                continue;
            };
            let mut fields = BTreeMap::new();
            for lifted_field in lifted_fields {
                if let Some(value) = group.get(lifted_field) {
                    fields.insert(lifted_field.clone(), value.clone());
                }
            }
            for (key, value) in handler {
                fields.insert(key.clone(), value.clone());
            }
            members.push(fields);
        }
    }
    members
}

/// Nest one group-array member's flat fields back into its entry value — a matcher group
/// `{lifted?, member_key:[{...handler}]}`: each lifted field lifts to the group level
/// (when present), every other field becomes the single handler's own. The `member_key` is
/// the key of the handler array within each group (e.g., `"hooks"`), and `lifted_fields`
/// are the names of fields to lift to the group level (e.g., `["matcher"]`). The inverse
/// of [`hook_member_fields`], and the reason emit writes the array-of-groups shape Claude
/// Code loads rather than the flat object form it silently ignores. `pub(crate)` so the
/// write face (`crate::drift`) nests through the one shape this module's read face
/// decomposes.
pub(crate) fn hook_matcher_group(
    fields: &[(String, JsonValue)],
    member_key: &str,
    lifted_fields: &[String],
) -> JsonValue {
    let mut lifted = BTreeMap::new();
    let mut handler = JsonMap::new();
    for (key, value) in fields {
        if lifted_fields.contains(key) {
            lifted.insert(key.clone(), value.clone());
        } else {
            handler.insert(key.clone(), value.clone());
        }
    }
    let mut group = JsonMap::new();
    for (key, value) in lifted {
        group.insert(key, value);
    }
    group.insert(
        member_key.to_string(),
        JsonValue::Array(vec![JsonValue::Object(handler)]),
    );
    JsonValue::Object(group)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::extract::ValueType;
    use crate::kind::{CollectionKeyPath, Extraction, Governs};
    use crate::test_support::tmpdir;

    fn mcp_address() -> CollectionAddress {
        CollectionAddress {
            manifest: ".mcp.json".to_string(),
            key_path: CollectionKeyPath::McpServers,
            entry_shape: crate::kind::EntryShape::Object,
        }
    }

    /// A manifest with one declared collection (`mcpServers`, two entries) plus two
    /// genuinely unschematized top-level keys — the residue that stays opaque.
    const MANIFEST: &str = r#"{
  "autoMemoryEnabled": false,
  "mcpServers": {
    "gmail": { "command": "npx", "args": ["gmail-mcp"], "timeout": 30 },
    "drive": { "command": "npx" }
  },
  "permissions": { "allow": ["Bash(cargo build:*)"] }
}"#;

    #[test]
    fn a_declared_collection_infers_one_member_per_entry_with_its_fields() {
        let dir = tmpdir("manifest-members");
        let file = dir.join(".mcp.json");
        fs::write(&file, MANIFEST).unwrap();

        let manifest = Manifest::read(&file, &[&mcp_address()]).unwrap();

        // One registration member per collection entry, in the collection's sorted key
        // order (`drive` before `gmail`), each surfacing in the `mcpServers` collection.
        let members: Vec<(&str, &str)> = manifest
            .members
            .iter()
            .map(|m| (m.collection.as_str(), m.key.as_str()))
            .collect();
        assert_eq!(
            members,
            vec![("mcpServers", "drive"), ("mcpServers", "gmail")]
        );

        // The entry's fields read back as raw JSON, unprojected — the shared fold types
        // them at read time, exactly as it does a frontmatter member's fields.
        let gmail = &manifest.members[1];
        assert_eq!(gmail.fields.get("command"), Some(&JsonValue::from("npx")));
        assert_eq!(gmail.fields.get("timeout"), Some(&JsonValue::from(30)));
        assert!(
            gmail
                .fields
                .get("args")
                .is_some_and(serde_json::Value::is_array)
        );
    }

    #[test]
    fn undeclared_top_level_keys_survive_as_opaque_fields() {
        let dir = tmpdir("manifest-opaque");
        let file = dir.join(".mcp.json");
        fs::write(&file, MANIFEST).unwrap();

        let manifest = Manifest::read(&file, &[&mcp_address()]).unwrap();

        // Every top-level key the address did not consume is an opaque field; the
        // consumed `mcpServers` collection is not (it became members instead).
        let opaque: Vec<&str> = manifest.opaque_fields.keys().map(String::as_str).collect();
        assert_eq!(opaque, vec!["autoMemoryEnabled", "permissions"]);
        assert_eq!(
            manifest.opaque_fields.get("autoMemoryEnabled"),
            Some(&FeatureValue::scalar(ValueType::Boolean, "false"))
        );
    }

    #[test]
    fn an_unrepresented_manifest_reads_wholly_opaque_then_infers_members_once_addressed() {
        let dir = tmpdir("manifest-unrepresented");
        let file = dir.join(".mcp.json");
        fs::write(&file, MANIFEST).unwrap();

        // With no address, nothing is claimed: every top-level key — `mcpServers`
        // included — reads as an opaque field, and no member is inferred.
        let bare = Manifest::read(&file, &[]).unwrap();
        assert!(bare.members.is_empty());
        assert!(bare.opaque_fields.contains_key("mcpServers"));

        // Handed the address, the same bytes infer the collection's members — the read is
        // driven by the address, not by the file being modelled as a member.
        let addressed = Manifest::read(&file, &[&mcp_address()]).unwrap();
        assert_eq!(addressed.members.len(), 2);
        assert!(!addressed.opaque_fields.contains_key("mcpServers"));
    }

    #[test]
    fn re_reading_an_unchanged_manifest_is_idempotent() {
        let dir = tmpdir("manifest-idempotent");
        let file = dir.join(".mcp.json");
        fs::write(&file, MANIFEST).unwrap();

        let first = Manifest::read(&file, &[&mcp_address()]).unwrap();
        let second = Manifest::read(&file, &[&mcp_address()]).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn a_non_object_top_level_is_a_loud_error_not_an_empty_read() {
        let dir = tmpdir("manifest-non-object");
        let file = dir.join(".mcp.json");
        fs::write(&file, "[1, 2, 3]\n").unwrap();

        let err = Manifest::read(&file, &[&mcp_address()]).unwrap_err();
        assert!(matches!(err, JsonManifestError::Malformed { .. }));
        assert!(err.to_string().contains("not a JSON object"));
    }

    #[test]
    fn malformed_json_is_a_loud_error() {
        let dir = tmpdir("manifest-malformed");
        let file = dir.join(".mcp.json");
        fs::write(&file, "{ \"mcpServers\": { \n").unwrap();

        let err = Manifest::read(&file, &[&mcp_address()]).unwrap_err();
        assert!(matches!(err, JsonManifestError::Malformed { .. }));
    }

    #[test]
    fn read_kind_dispatches_a_manifest_kind_off_its_governs_locus() {
        // A manifest kind whose `governs` finds the `.mcp.json` at the harness root, its
        // declared collection address naming the `mcpServers` map — the loader dispatch a
        // kind with a collection address takes instead of the frontmatter loader.
        let harness = tmpdir("manifest-read-kind");
        fs::write(harness.join(".mcp.json"), MANIFEST).unwrap();

        let mut kind = CustomKind::new(
            "mcp-server",
            Governs {
                root: ".".to_string(),
                glob: ".mcp.json".to_string(),
            },
            Extraction::new(Vec::new()),
        );
        kind.collection_address = Some(mcp_address());

        let disc = crate::import::Discovery::new(&harness);
        let files = crate::import::discover_kind_files(
            &disc,
            &kind,
            kind.governs.as_ref().unwrap(),
            crate::import::LocalOverride::Honored,
        );
        let reads = Manifest::read_kind(&files, &kind).unwrap();
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0].members.len(), 2);

        // A kind with no collection address is not a manifest kind — no read.
        let mut file_kind = kind.clone();
        file_kind.collection_address = None;
        let file_reads = Manifest::read_kind(&[], &file_kind).unwrap();
        assert!(file_reads.is_empty());
    }

    /// A `.mcp.json`-shaped represented manifest for the write face: one `mcpServers`
    /// collection (two entries, `serde_json`'s sorted order) and two opaque residue keys.
    fn represented() -> (Vec<CollectionSegment>, BTreeMap<String, JsonValue>) {
        let mut entries = BTreeMap::new();
        entries.insert(
            "gmail".to_string(),
            serde_json::json!({ "command": "npx", "timeout": 30 }),
        );
        entries.insert("drive".to_string(), serde_json::json!({ "command": "npx" }));
        let segment = CollectionSegment {
            collection_key: "mcpServers".to_string(),
            entries,
        };
        let mut residue = BTreeMap::new();
        residue.insert("autoMemoryEnabled".to_string(), JsonValue::Bool(false));
        residue.insert(
            "permissions".to_string(),
            serde_json::json!({ "allow": ["Bash(cargo build:*)"] }),
        );
        (vec![segment], residue)
    }

    #[test]
    fn write_manifest_regenerates_whole_declared_order_then_residue() {
        let (segments, residue) = represented();
        // The collection segment leads (its declared address order), the opaque residue
        // follows in sorted key order — the write face imposes that top-level order, which
        // serde_json's own sorted-map serialization could not (it would interleave
        // `autoMemoryEnabled` before `mcpServers`).
        let expected = "{\n  \"mcpServers\": {\n    \"drive\": {\n      \"command\": \"npx\"\n    },\n    \"gmail\": {\n      \"command\": \"npx\",\n      \"timeout\": 30\n    }\n  },\n  \"autoMemoryEnabled\": false,\n  \"permissions\": {\n    \"allow\": [\n      \"Bash(cargo build:*)\"\n    ]\n  }\n}\n";
        assert_eq!(write_manifest(&segments, &residue), expected);
    }

    #[test]
    fn write_manifest_is_byte_identical_across_a_double_emit() {
        // A pure function of its inputs — the double-emit determinism the pipeline's
        // "Emit" byte-check rests on.
        let (segments, residue) = represented();
        assert_eq!(
            write_manifest(&segments, &residue),
            write_manifest(&segments, &residue)
        );
    }

    #[test]
    fn write_manifest_matches_serde_pretty_when_the_top_level_is_already_sorted() {
        // With one collection and no residue the top-level order is trivially sorted, so
        // the write face lands byte-for-byte on `serde_json::to_string_pretty` + LF — the
        // canonical encoder `bundle`'s manifests ride, the target entry 3/5 consolidates
        // onto (no second encoder that could drift).
        let mut entries = BTreeMap::new();
        entries.insert("drive".to_string(), serde_json::json!({ "command": "npx" }));
        let segment = CollectionSegment {
            collection_key: "mcpServers".to_string(),
            entries: entries.clone(),
        };
        let equivalent = serde_json::json!({ "mcpServers": { "drive": { "command": "npx" } } });
        let expected = format!("{}\n", serde_json::to_string_pretty(&equivalent).unwrap());
        assert_eq!(write_manifest(&[segment], &BTreeMap::new()), expected);
    }

    #[test]
    fn an_empty_manifest_writes_a_bare_object() {
        assert_eq!(write_manifest(&[], &BTreeMap::new()), "{}\n");
    }

    #[test]
    fn an_unrepresented_manifest_stays_on_json_splice_in_place() {
        // The write face regenerates a represented manifest whole; an unrepresented one
        // stays on `json_splice`, which edits a human's document in place. Splicing a new
        // member leaves every other byte — key order, spacing — untouched, the guarantee
        // that keeps `json_splice` the unrepresented path this write face does not replace.
        let text = "{\n  \"mcpServers\": {\n    \"gmail\": { \"command\": \"npx\" }\n  }\n}";
        let root = crate::json_splice::object_shape(text, 0);
        let servers = root
            .members
            .iter()
            .find(|member| member.key == "mcpServers")
            .expect("the mcpServers member is present");
        let servers_shape = crate::json_splice::object_shape(text, servers.value_span.0);
        let edit = crate::json_splice::insert_member(
            &servers_shape,
            "drive",
            &serde_json::json!({ "command": "npx" }),
            2,
        );
        let spliced = crate::json_splice::apply_edits(text, vec![edit]);
        // The human's `gmail` entry survives byte-for-byte; only `drive` is grafted in.
        assert!(spliced.contains("\"gmail\": { \"command\": \"npx\" }"));
        assert!(spliced.contains("\"drive\""));
    }

    #[test]
    fn manifest_members_walk_each_entry_raw_and_skip_a_non_object() {
        // The collection object's entries each become a member's raw JSON fields, in the
        // map's own sorted key order — unprojected, so the one shared fold types them at
        // read time. A non-object entry contributes no fields (it holds no key/value pairs
        // a fields-only member reads).
        let manifest = serde_json::json!({
            "mcpServers": {
                "gmail": { "command": "npx", "timeout": 30 },
                "opaque": "not-an-object"
            },
            "permissions": { "allow": ["Bash"] }
        });
        let manifest = manifest.as_object().unwrap();

        let members = manifest_members(manifest, "mcpServers", &crate::kind::EntryShape::Object);
        let keys: Vec<&str> = members.iter().map(|(key, _)| key.as_str()).collect();
        assert_eq!(keys, vec!["gmail", "opaque"]);

        let gmail = &members[0].1;
        assert_eq!(gmail.get("command"), Some(&JsonValue::from("npx")));
        assert_eq!(gmail.get("timeout"), Some(&JsonValue::from(30)));
        // The string-valued entry has no object fields to read.
        assert!(members[1].1.is_empty());

        // An absent collection key yields no members — absent, never errored.
        assert!(manifest_members(manifest, "hooks", &crate::kind::EntryShape::Object).is_empty());
    }

    #[test]
    fn enablement_members_carry_the_scalar_value_and_re_render_identically() {
        // An `enabledPlugins` entry is a bare scalar, not an array or an entry object: the
        // member carries the value as its one declared field, and rendering it back
        // reproduces the scalar the manifest held.
        let manifest = serde_json::json!({
            "enabledPlugins": {
                "formatter@my-marketplace": true,
                "legacy@my-marketplace": false
            }
        });
        let manifest = manifest.as_object().unwrap();

        let members = manifest_members(
            manifest,
            "enabledPlugins",
            &crate::kind::EntryShape::Scalar {
                field: "enabled".to_string(),
            },
        );
        let keys: Vec<&str> = members.iter().map(|(key, _)| key.as_str()).collect();
        assert_eq!(
            keys,
            vec!["formatter@my-marketplace", "legacy@my-marketplace"]
        );

        for (index, expected) in [true, false].into_iter().enumerate() {
            let fields = &members[index].1;
            assert_eq!(fields.len(), 1, "one field, off the scalar value");
            assert_eq!(fields.get("enabled"), Some(&JsonValue::Bool(expected)));

            // Re-rendering the member's fields reproduces the entry value byte-for-byte —
            // the inverse the write face rides.
            let round_tripped: Vec<(String, JsonValue)> =
                fields.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            assert_eq!(
                enablement_entry_value(&round_tripped, "enabled"),
                JsonValue::Bool(expected)
            );
        }

        // An absent collection key yields no members — absent, never errored.
        assert!(manifest_members(manifest, "hooks", &crate::kind::EntryShape::Object).is_empty());
    }

    #[test]
    fn an_enablement_member_with_no_enabled_field_renders_the_documented_true() {
        // Claude Code writes `true` when it enables a plugin, so a member that declares no
        // `enabled` field renders as the enablement it spells rather than an invented shape.
        assert_eq!(
            enablement_entry_value(&[], "enabled"),
            JsonValue::Bool(true)
        );
    }

    #[test]
    fn hook_members_decompose_the_matcher_group_array_and_re_nest_identically() {
        // A `hooks.<Event>` value is the array of matcher groups Claude Code loads, not a
        // lone entry object: each handler decomposes into flat {matcher?, type, command}
        // fields, and re-nesting one such member reproduces the group byte-for-byte.
        let manifest = serde_json::json!({
            "hooks": {
                "PreToolUse": [
                    { "matcher": "Bash", "hooks": [ { "type": "command", "command": "echo guard" } ] }
                ],
                "SessionStart": [
                    { "hooks": [ { "type": "command", "command": "echo hi" } ] }
                ]
            }
        });
        let manifest = manifest.as_object().unwrap();

        let members = manifest_members(
            manifest,
            "hooks",
            &crate::kind::EntryShape::GroupArray {
                member_key: "hooks".to_string(),
                lifted_fields: vec!["matcher".to_string()],
            },
        );
        let keys: Vec<&str> = members.iter().map(|(key, _)| key.as_str()).collect();
        assert_eq!(keys, vec!["PreToolUse", "SessionStart"]);

        // The tool-scoped event lifts its group `matcher` alongside the handler fields; the
        // event with no matcher carries only the handler's own.
        let pre = &members[0].1;
        assert_eq!(pre.get("matcher"), Some(&JsonValue::from("Bash")));
        assert_eq!(pre.get("command"), Some(&JsonValue::from("echo guard")));
        assert_eq!(pre.get("type"), Some(&JsonValue::from("command")));
        assert!(!members[1].1.contains_key("matcher"));

        // Re-nesting a decomposed member is the inverse of the read — byte-for-byte the
        // group it came from.
        let fields: Vec<(String, JsonValue)> = pre.clone().into_iter().collect();
        let regrouped = hook_matcher_group(&fields, "hooks", &["matcher".to_string()]);
        let source_group = manifest["hooks"]["PreToolUse"].as_array().unwrap()[0].clone();
        assert_eq!(regrouped, source_group);
    }
}
