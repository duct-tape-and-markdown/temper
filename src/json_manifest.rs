//! The generic JSON-manifest adapter, read face — `frontmatter.rs`'s peer for
//! structured config.
//!
//! Where the frontmatter adapter reads a markdown artifact's YAML header, this one reads
//! a structured manifest (`settings.json`, `.mcp.json`): a real JSON parser owns the
//! grammar, and a kind's declared collection address selects which key paths walk into
//! the generic surface extractor (`crate::extract`). Each entry at a declared address
//! reads as a fields-only registration member (a hook, an MCP server); every top-level
//! key no address consumed survives as an opaque field of the container. Reading an
//! unrepresented manifest still infers its registration members off the addresses handed
//! in — the file need not be modelled as a member for its members to surface.
//!
//! Read face only: the canonical whole-manifest write face is a later slice, and the
//! unrepresented-manifest write stays `src/json_splice.rs`. The read is a pure function
//! of the manifest bytes — sorted-key entries, kind-preserving fields — so a re-read is
//! byte-identical, the idempotence keystone the frontmatter face also holds.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as JsonValue;

use crate::extract::{self, FeatureValue};
use crate::frontmatter::Provenance;
use crate::kind::{CollectionAddress, CustomKind};

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
    /// The member's typed fields, key → kind-preserving [`FeatureValue`], in sorted key
    /// order. Declared and opaque keys alike surface here: a fields-only member keeps
    /// every entry key, the same permissive read the frontmatter face gives unknown keys.
    pub fields: BTreeMap<String, FeatureValue>,
}

/// Errors raised while reading a [`Manifest`]. Hard failures (missing file, non-UTF-8,
/// malformed JSON) — distinct from a lint `Diagnostic`, which the engine collects rather
/// than throws. Mirrors [`crate::frontmatter::FrontmatterError`]'s shape for the JSON face.
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

    /// The manifest kind's source files could not be discovered off the harness.
    #[error(transparent)]
    #[diagnostic(transparent)]
    Discovery(#[from] crate::import::ImportError),
}

impl Manifest {
    /// Read the JSON manifest at `source_file`, walking each of `addresses` into its
    /// declared collection to infer that collection's registration members, and keeping
    /// every top-level key no address consumed as an opaque field. A real JSON parser
    /// (`serde_json`) owns the grammar; the per-entry field projection rides the shared
    /// surface extractor ([`extract::manifest_members`]), so the manifest read shares the
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
        let bytes = fs::read(source_file).map_err(|source| JsonManifestError::Io {
            path: source_file.to_path_buf(),
            source,
        })?;
        let source_hash = crate::hash::sha256_hex(&bytes);
        let raw = String::from_utf8(bytes).map_err(|source| JsonManifestError::NotUtf8 {
            path: source_file.to_path_buf(),
            source,
        })?;

        let value: JsonValue =
            serde_json::from_str(&raw).map_err(|err| JsonManifestError::Malformed {
                path: source_file.to_path_buf(),
                detail: err.to_string(),
            })?;
        let JsonValue::Object(manifest) = value else {
            return Err(JsonManifestError::Malformed {
                path: source_file.to_path_buf(),
                detail: "manifest top level is not a JSON object".to_string(),
            });
        };

        // Each declared address walks into its collection; the top-level keys it consumes
        // are tracked so the residue that stays opaque excludes them.
        let mut members = Vec::new();
        let mut consumed = BTreeSet::new();
        for address in addresses {
            let collection = address.key_path.collection_key();
            consumed.insert(collection);
            for (key, fields) in extract::manifest_members(&manifest, collection) {
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
    /// instead of the frontmatter loader. Discovers the kind's manifest file(s) through
    /// the same `governs` walk every kind's discovery runs
    /// ([`crate::import::discover_kind_files`]), then reads each through [`Manifest::read`]
    /// against the kind's own declared address. A kind with no collection address is not a
    /// manifest kind — the caller routes it to the frontmatter loader — so this yields no
    /// reads for it.
    ///
    /// # Errors
    ///
    /// Returns a [`JsonManifestError`] if discovery fails or a discovered manifest cannot
    /// be read.
    pub fn read_kind(harness: &Path, kind: &CustomKind) -> Result<Vec<Self>, JsonManifestError> {
        let Some(address) = &kind.collection_address else {
            return Ok(Vec::new());
        };
        crate::import::discover_kind_files(harness, kind, &kind.governs)?
            .iter()
            .map(|file| Manifest::read(file, &[address]))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract::ValueType;
    use crate::kind::{CollectionKeyPath, Extraction, Governs};
    use crate::test_support::tmpdir;

    fn mcp_address() -> CollectionAddress {
        CollectionAddress {
            manifest: ".mcp.json".to_string(),
            key_path: CollectionKeyPath::McpServers,
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

        // The entry's fields read back kind-preserving through the shared extractor: a
        // string stays `string`, an integer keeps `integer`, a list stays a list.
        let gmail = &manifest.members[1];
        assert_eq!(
            gmail.fields.get("command"),
            Some(&FeatureValue::scalar(ValueType::String, "npx"))
        );
        assert_eq!(
            gmail.fields.get("timeout").map(FeatureValue::kind),
            Some(ValueType::Integer)
        );
        assert_eq!(
            gmail.fields.get("args").map(FeatureValue::kind),
            Some(ValueType::List)
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

        let reads = Manifest::read_kind(&harness, &kind).unwrap();
        assert_eq!(reads.len(), 1);
        assert_eq!(reads[0].members.len(), 2);

        // A kind with no collection address is not a manifest kind — no read.
        let mut file_kind = kind.clone();
        file_kind.collection_address = None;
        assert!(
            Manifest::read_kind(&harness, &file_kind)
                .unwrap()
                .is_empty()
        );
    }
}
