//! The JSON manifest adapter's two faces — a JSON-parser peer to the frontmatter loader.
//!
//! Read: a kind's declared collection address selects which key paths of a structured
//! manifest walk into the generic surface extractor — each entry reads as a fields-only
//! registration member, every undeclared top-level key survives as an opaque field, and
//! an unrepresented manifest still infers its members off the address handed in.
//!
//! Write: a represented manifest regenerates whole — its declared collection segments in
//! address order, then the opaque residue, canonical LF. Both faces are pure functions of
//! their inputs, so a re-read and a double-emit are each byte-identical.

use std::collections::BTreeMap;
use std::fs;

use temper::extract::{FeatureValue, ValueType};
use temper::json_manifest::{CollectionSegment, Manifest, write_manifest};
use temper::kind::{CollectionAddress, CollectionKeyPath, CustomKind, Extraction, Governs};

mod common;

/// The `mcpServers.*` collection address a test MCP-server kind declares — the manifest
/// segment its registration members surface at.
fn mcp_address() -> CollectionAddress {
    CollectionAddress {
        manifest: ".mcp.json".to_string(),
        key_path: CollectionKeyPath::McpServers,
        entry_shape: temper::kind::EntryShape::Object,
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

    // A member's fields read back as raw JSON — unprojected, so the shared read-time fold
    // types them exactly as a frontmatter member's fields, never a second projector.
    let gmail = &manifest.members[1];
    assert_eq!(
        gmail.fields.get("command"),
        Some(&serde_json::Value::from("npx"))
    );
    assert_eq!(
        gmail.fields.get("timeout"),
        Some(&serde_json::Value::from(30))
    );
    assert!(
        gmail
            .fields
            .get("args")
            .is_some_and(serde_json::Value::is_array)
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
    common::write_mcp_json(&harness, MANIFEST);

    let mut kind = CustomKind::new(
        "mcp-server",
        Governs {
            root: ".".to_string(),
            glob: ".mcp.json".to_string(),
        },
        Extraction::new(Vec::new()),
    );
    kind.collection_address = Some(mcp_address());

    let disc = temper::import::Discovery::new(&harness);
    let files = temper::import::discover_kind_files(
        &disc,
        &kind,
        kind.governs.as_ref().unwrap(),
        temper::import::LocalOverride::Honored,
    );
    let reads = Manifest::read_kind(&files, &kind).unwrap();
    assert_eq!(reads.len(), 1);
    assert_eq!(reads[0].members.len(), 2);
}

#[test]
fn a_represented_manifest_regenerates_whole_declared_order_then_residue() {
    // A represented manifest's write face over a `hooks` collection (one lifecycle event,
    // its member an array) plus an opaque residue: the collection leads in declared
    // address order, the residue follows sorted, LF throughout.
    let mut entries = BTreeMap::new();
    entries.insert(
        "SessionStart".to_string(),
        serde_json::json!([{ "hooks": [{ "type": "command", "command": "temper explain" }] }]),
    );
    let segment = CollectionSegment {
        collection_key: "hooks".to_string(),
        entries,
    };
    let mut residue = BTreeMap::new();
    residue.insert(
        "autoMemoryEnabled".to_string(),
        serde_json::Value::Bool(true),
    );

    let segments = [segment];
    let expected = "{\n  \"hooks\": {\n    \"SessionStart\": [\n      {\n        \"hooks\": [\n          {\n            \"command\": \"temper explain\",\n            \"type\": \"command\"\n          }\n        ]\n      }\n    ]\n  },\n  \"autoMemoryEnabled\": true\n}\n";
    assert_eq!(write_manifest(&segments, &residue), expected);

    // The pipeline's "Emit" double-emit byte-check: a pure function of its inputs.
    assert_eq!(
        write_manifest(&segments, &residue),
        write_manifest(&segments, &residue)
    );
}
