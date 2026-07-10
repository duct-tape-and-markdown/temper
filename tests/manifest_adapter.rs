//! The JSON manifest adapter's read face — a JSON-parser peer to the frontmatter loader.
//!
//! A kind's declared collection address selects which key paths of a structured manifest
//! walk into the generic surface extractor: each entry reads as a fields-only
//! registration member, every undeclared top-level key survives as an opaque field, and
//! an unrepresented manifest still infers its members off the address handed in — the
//! read a pure function of the bytes, so a re-read is byte-identical.

use std::fs;

use temper::extract::{FeatureValue, ValueType};
use temper::json_manifest::Manifest;
use temper::kind::{CollectionAddress, CollectionKeyPath, CustomKind, Extraction, Governs};

mod common;

/// The `mcpServers.*` collection address a test MCP-server kind declares — the manifest
/// segment its registration members surface at.
fn mcp_address() -> CollectionAddress {
    CollectionAddress {
        manifest: ".mcp.json".to_string(),
        key_path: CollectionKeyPath::McpServers,
    }
}

/// A `.mcp.json`-shaped manifest: one declared collection (`mcpServers`, two entries with
/// typed fields) plus two genuinely unschematized top-level keys — the residue that stays
/// opaque. A real Claude Code manifest shape, not one invented for the test's convenience.
const MANIFEST: &str = r#"{
  "autoMemoryEnabled": false,
  "mcpServers": {
    "gmail": { "command": "npx", "args": ["gmail-mcp"], "timeout": 30 },
    "drive": { "command": "npx" }
  },
  "permissions": { "allow": ["Bash(cargo build:*)"] }
}"#;

#[test]
fn a_declared_collection_address_infers_one_member_per_entry_with_the_declared_fields() {
    let dir = common::tmpdir("manifest-adapter-members");
    let file = dir.join(".mcp.json");
    fs::write(&file, MANIFEST).unwrap();

    let manifest = Manifest::read(&file, &[&mcp_address()]).unwrap();

    // One registration member per collection entry, in the collection's own sorted key
    // order, each surfacing in the `mcpServers` collection the address walked into.
    let members: Vec<(&str, &str)> = manifest
        .members
        .iter()
        .map(|member| (member.collection.as_str(), member.key.as_str()))
        .collect();
    assert_eq!(
        members,
        vec![("mcpServers", "drive"), ("mcpServers", "gmail")]
    );

    // The declared fields read back kind-preserving through the shared surface extractor:
    // a string stays `string`, an integer keeps `integer`, a list stays a list.
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
fn undeclared_keys_read_back_as_opaque_fields() {
    let dir = common::tmpdir("manifest-adapter-opaque");
    let file = dir.join(".mcp.json");
    fs::write(&file, MANIFEST).unwrap();

    let manifest = Manifest::read(&file, &[&mcp_address()]).unwrap();

    // Every top-level key the address did not consume is an opaque field, named as such;
    // the consumed `mcpServers` collection became members instead, so it is not opaque.
    let opaque: Vec<&str> = manifest.opaque_fields.keys().map(String::as_str).collect();
    assert_eq!(opaque, vec!["autoMemoryEnabled", "permissions"]);
    assert_eq!(
        manifest.opaque_fields.get("autoMemoryEnabled"),
        Some(&FeatureValue::scalar(ValueType::Boolean, "false"))
    );
}

#[test]
fn an_unrepresented_manifest_infers_its_members_off_the_address() {
    let dir = common::tmpdir("manifest-adapter-unrepresented");
    let file = dir.join(".mcp.json");
    fs::write(&file, MANIFEST).unwrap();

    // The file is not modelled as a member, yet handed the address its collection's
    // members are inferred all the same — the read is driven by the address, not by the
    // manifest being represented.
    let manifest = Manifest::read(&file, &[&mcp_address()]).unwrap();
    assert_eq!(manifest.members.len(), 2);
    assert!(!manifest.opaque_fields.contains_key("mcpServers"));
}

#[test]
fn re_reading_an_unchanged_manifest_is_idempotent() {
    let dir = common::tmpdir("manifest-adapter-idempotent");
    let file = dir.join(".mcp.json");
    fs::write(&file, MANIFEST).unwrap();

    let first = Manifest::read(&file, &[&mcp_address()]).unwrap();
    let second = Manifest::read(&file, &[&mcp_address()]).unwrap();
    assert_eq!(first, second);
}

#[test]
fn read_kind_routes_a_manifest_kind_through_its_governs_locus() {
    // A manifest kind whose `governs` discovers the `.mcp.json` at the harness root, its
    // declared collection address naming the `mcpServers` map — the loader dispatch a kind
    // with a collection address takes instead of the frontmatter loader.
    let harness = common::tmpdir("manifest-adapter-read-kind");
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
}
