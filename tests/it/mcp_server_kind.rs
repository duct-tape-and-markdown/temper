//! The `mcp-server` built-in kind: a `.mcp.json` `mcpServers.*` registration member
//! (`specs/builtins.md`, "The coverage bar"; 0021, "the manifest authoring surface").
//!
//! The second manifest kind, and the first whose entries are objects — fields-only,
//! discovered off the root `.mcp.json` manifest at the `mcpServers.*` collection address
//! and read through the JSON manifest adapter, each server's own fields folded into its
//! member. Driven over fixtures mirroring the real Claude Code layout (a `.mcp.json` whose
//! top level is `mcpServers`, per `.claude/rules/rust.md`): the read that turns a server
//! entry into a member, and the shipped default contract's one decidable clause — the
//! transport `type` must be documented — firing on a broken server and passing clean ones,
//! end to end through the `check --harness` gate.
//!
//! This kind's reach is its segment, never the file: `mcpServers` is the address it keys
//! at, and the document around it belongs to the `mcp` container kind
//! (`tests/it/mcp_kind.rs`). Both ship embedded, so a `.mcp.json` is governed whole for
//! every harness — which makes the ungoverned-surface reading gone and an ownerless
//! manifest the container's *undeclared* member instead, the last case below.

use std::path::Path;

use temper::drift::{self, EmitOptions};

use crate::common;

use crate::common::check_harness;

use serde_json::Value as JsonValue;
use temper::builtin_kind;
use temper::builtin_lock;
use temper::kind::{CollectionAddress, CollectionKeyPath, Content, Registration};

/// A `.mcp.json` carrying two servers — one on the documented `http` transport, one on an
/// undocumented `grpc` transport — in the real Claude Code shape (the top level is
/// `mcpServers`, each entry a server config object).
const BROKEN_MCP: &str = r#"{
  "mcpServers": {
    "docs": { "type": "http", "url": "https://mcp.example.com/mcp" },
    "legacy": { "type": "grpc", "command": "npx" }
  }
}"#;

/// A `.mcp.json` whose two servers both name documented transports — a `type`-less stdio
/// server (absent reads as `stdio`, the documented default) and an `http` server.
const CLEAN_MCP: &str = r#"{
  "mcpServers": {
    "docs": { "type": "http", "url": "https://mcp.example.com/mcp" },
    "files": { "command": "npx", "args": ["-y", "@modelcontextprotocol/server-filesystem", "."] }
  }
}"#;

fn mcp_server_kind() -> temper::kind::CustomKind {
    builtin_kind::definition("mcp-server").expect("mcp-server is embedded")
}

#[test]
fn the_mcp_server_kind_is_a_fields_only_manifest_kind_at_the_mcp_servers_collection_address() {
    let mcp = mcp_server_kind();
    assert_eq!(mcp.content, Content::Fields);
    assert_eq!(
        mcp.collection_address,
        Some(CollectionAddress {
            manifest: ".mcp.json".to_string(),
            key_path: CollectionKeyPath::McpServers,
            entry_shape: temper::kind::EntryShape::Object,
        })
    );
    assert_eq!(mcp.registration, vec![Registration::Connection]);
}

#[test]
fn a_mcp_json_server_entry_reads_as_an_mcp_server_member_with_its_fields() {
    let harness = common::tmpdir("read-mcp-members");
    common::write_mcp_json(&harness, BROKEN_MCP);

    let reads = common::manifest_members(&harness, &mcp_server_kind());
    assert_eq!(reads.len(), 1, "the one .mcp.json manifest is read once");

    // One member per `mcpServers.*` entry, keyed by server name, in the collection's own
    // sorted key order — each surfacing in the `mcpServers` collection.
    let members: Vec<(&str, &str)> = reads[0]
        .members
        .iter()
        .map(|m| (m.collection.as_str(), m.key.as_str()))
        .collect();
    assert_eq!(
        members,
        vec![("mcpServers", "docs"), ("mcpServers", "legacy")]
    );

    // Unlike a hook (whose event value is an array), a server entry is an object, so its
    // own fields fold into the member — carried raw for the shared read-time projection.
    let docs = &reads[0].members[0];
    assert_eq!(docs.fields.get("type"), Some(&JsonValue::from("http")));
    assert_eq!(
        docs.fields.get("url"),
        Some(&JsonValue::from("https://mcp.example.com/mcp"))
    );
}

#[test]
fn the_mcp_server_default_contract_fires_on_an_undocumented_transport() {
    let harness = common::tmpdir("mcp-broken-transport");
    common::write_mcp_json(&harness, BROKEN_MCP);

    let (findings, ok) = check_harness(&harness);

    // The strictest-documented-profile clause fires exactly once — on the `grpc` server,
    // never on the documented `http` one.
    let fired = common::findings_for(&findings, "mcp-server.enum.type");
    assert_eq!(
        fired.len(),
        1,
        "exactly the undocumented transport fires the clause, got: {findings:#?}"
    );
    assert!(
        fired[0].contains("legacy"),
        "the finding names the offending server, got: {}",
        fired[0]
    );
    assert!(
        !ok,
        "an undocumented transport is a required-severity finding — the run fails, got: {findings:#?}"
    );
}

#[test]
fn the_mcp_server_default_contract_passes_documented_and_absent_transports() {
    let harness = common::tmpdir("mcp-clean-transport");
    common::write_mcp_json(&harness, CLEAN_MCP);

    let (findings, _ok) = check_harness(&harness);

    // A documented `http` and a `type`-less stdio server (absent reads as the documented
    // `stdio` default) both pass the clause.
    assert!(
        common::findings_for(&findings, "mcp-server.enum.type").is_empty(),
        "every documented transport passes the clause, got: {findings:#?}"
    );
}

/// A program declaring the connection and nothing else — no container member, so the
/// document `emit` writes is owned by no member of the kind that governs it.
const SERVER_ONLY_PROGRAM: &str = r#"
import { emit, harness } from "@dtmd/temper";
import { mcpServer } from "@dtmd/temper/claude-code";

const docs = mcpServer({
  name: "docs",
  type: "http",
  url: "https://mcp.example.com/mcp",
});

process.stdout.write(emit(harness({ members: [docs] })).seam);
"#;

#[test]
fn a_mcp_json_is_governed_whole_by_the_container_rather_than_by_this_segment_kind() {
    // This kind reaches its segment, so on its own it would leave the document around
    // `mcpServers` ungoverned — the shape a `settings.json` hook leaves its container in.
    // The `mcp` container kind ships embedded beside it, so the file is governed whole for
    // every harness, program or not, and the coverage note names both members apart.
    let harness = common::tmpdir("mcp-modeled-surface");
    common::write_mcp_json(&harness, CLEAN_MCP);

    let (findings, _ok) = check_harness(&harness);

    let unmodeled = common::findings_for(&findings, "coverage.unmodeled-surface");
    assert!(
        unmodeled.iter().all(|line| !line.contains(".mcp.json")),
        "the embedded `mcp` kind governs .mcp.json whole — no unmodeled-surface finding, got: {findings:#?}"
    );
    // Silence above is truthful only because the container really is read: the note says so
    // by name, one container member beside the two connections.
    assert!(
        findings.iter().any(|line| line.contains("mcp (1)")),
        "the container member is announced as checked, never silently skipped, got: {findings:#?}"
    );
    assert!(
        findings.iter().any(|line| line.contains("mcp-server (2)")),
        "and the connections stay members of this kind, got: {findings:#?}"
    );
}

#[test]
fn a_program_declaring_only_servers_leaves_the_containers_document_undeclared() {
    // The consequence of the split reach, stated where it bites: a program that declares
    // the connections alone still writes a whole `.mcp.json`, and that document sits at the
    // `mcp` kind's governed locus with no member of that kind naming it. The advisory it
    // draws is `root.locus-declared` — a document temper maintains nothing about — never
    // the ungoverned-surface reading, which the embedded container retired.
    let (harness, into) = common::wire_sdk_harness("mcp-server-only", SERVER_ONLY_PROGRAM);
    drift::emit_program(&into, EmitOptions::default()).unwrap();
    assert!(
        Path::new(&harness).join(".mcp.json").is_file(),
        "the server's own registration projects the manifest"
    );

    let (findings, _ok) = check_harness(&harness);

    assert!(
        common::findings_for(&findings, "coverage.unmodeled-surface").is_empty(),
        "a governing kind retires the unmodeled-surface advisory, got: {findings:#?}"
    );
    let undeclared = common::findings_for(&findings, "root.locus-declared");
    assert_eq!(undeclared.len(), 1, "{findings:#?}");
    assert!(
        undeclared[0].contains(".mcp.json") && undeclared[0].contains("`mcp`"),
        "the finding names the undeclared document and the container kind whose locus it \
         sits at, got: {}",
        undeclared[0]
    );
    // And the note discloses the same member as undeclared rather than counting it read.
    assert!(
        findings
            .iter()
            .any(|line| line.contains("mcp (1: 0 declared, 1 undeclared)")),
        "the coverage note states the undeclared container apart, got: {findings:#?}"
    );
}

#[test]
fn the_embedded_builtin_lock_carries_the_mcp_server_kind_and_its_type_clause() {
    // The embedded side of the built-in lock (`tests/builtin_lock_frozen.rs` pins its
    // byte-equality with the SDK module's own memberless emit): the mcp-server kind fact
    // and its one enum clause are present, so SDK-less checking ships the same contract.
    let declarations = builtin_lock::declarations();

    let mcp = declarations
        .kinds
        .iter()
        .find(|k| k.name == "mcp-server")
        .expect("the mcp-server kind fact is embedded");
    assert_eq!(mcp.shape.as_deref(), Some("fields"));
    let address = mcp
        .collection_address
        .as_ref()
        .expect("the mcp-server kind carries its collection address");
    assert_eq!(address.manifest, ".mcp.json");
    assert_eq!(address.key_path, "mcpServers.*");

    let clause = declarations
        .clauses
        .iter()
        .find(|c| c.kind.as_deref() == Some("mcp-server") && c.predicate == "enum")
        .expect("the mcp-server default contract's type clause is embedded");
    assert_eq!(clause.field.as_deref(), Some("type"));
    let values = clause.values.as_ref().expect("the enum carries its values");
    assert!(
        values.iter().any(|v| v == "stdio") && values.iter().any(|v| v == "http"),
        "the documented-transport allowlist carries the transports, got: {values:?}"
    );
}
