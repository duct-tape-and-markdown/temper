//! The `mcp` built-in kind: the committed `.mcp.json` as a whole
//! (`specs/builtins.md`, "The shipped kinds"; decision 0050).
//!
//! A `json-document` at the committed commitment class, singleton identity from its
//! documented path, and — like `settings` — the **container** of a registration
//! collection address rather than a member at one. `mcpServers` is `mcp-server`'s
//! address, so the container types nothing at all: `.mcp.json` documents that one key,
//! every other key is opaque residue, and the typed half is empty by construction.
//!
//! The engine's container machinery is generic (`src/drift.rs` recognizes a container by
//! its projection path); these cases pin it to the shipped kind, driven over the real SDK
//! the way `tests/it/settings_kind.rs` drives it — `node` running the built
//! `@dtmd/temper/claude-code` against a fixture program. The kind's embedded home is
//! `src/builtin_lock.toml`, so every harness gets `.mcp.json` governed whether or not it
//! carries a program; what these cases drive over the seam is the *authoring* half — a
//! program declaring the container, and the projection and rollup row that follow. The
//! two halves must agree, or `kind.admissibility` refuses the fixture's own `mcp` for
//! colliding with the built-in rather than relocating it.
//!
//! One fact stated three ways: the file is a whole projection, so the residue survives
//! byte-faithfully beside the declared segment, the container's rollup row fingerprints
//! the whole file so a hand edit to *either* half is drift under the root `fresh` clause,
//! and the servers under `mcpServers` stay `mcp-server` members — counted as their own
//! kind, never folded into the container's fields.
//!
//! Every format fact here is the live MCP docs' (code.claude.com/docs/en/mcp, retrieved
//! 2026-09-29): `mcpServers` is the one documented top-level key.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use sha2::{Digest, Sha256};
use temper::drift::{self, EmitOptions, EmitOutcome};
use temper::json_manifest;

use crate::common;

/// An `mcp` container member beside an `mcpServer` registering at that same file's
/// `mcpServers` collection address, plus one top-level key the docs never document —
/// `$schema`, the editor-autocomplete key a real repository picks up — riding the
/// container's opaque residue. Every key of `.mcp.json` is authored in the program.
const MCP_MEMBER_PROGRAM: &str = r#"
import { emit, harness } from "@dtmd/temper";
import { mcp, mcpServer } from "@dtmd/temper/claude-code";

const docs = mcpServer({
  name: "docs",
  type: "http",
  url: "https://example.com/mcp",
});

const manifest = mcp({
  name: ".mcp",
  residue: { $schema: "https://json.schemastore.org/mcp.json" },
});

process.stdout.write(emit(harness({ members: [docs, manifest] })).seam);
"#;

/// The manifest the program above describes: the `mcpServers` collection segment carrying
/// the one connection, then the container's undocumented key as opaque residue — rendered
/// through the canonical write face, so this is the byte oracle rather than a
/// hand-spelled JSON string.
fn expected_manifest() -> String {
    let mut entries = BTreeMap::new();
    entries.insert(
        "docs".to_string(),
        serde_json::json!({ "type": "http", "url": "https://example.com/mcp" }),
    );
    let segment = json_manifest::CollectionSegment {
        collection_key: "mcpServers".to_string(),
        entries,
    };
    let residue = BTreeMap::from([(
        "$schema".to_string(),
        serde_json::json!("https://json.schemastore.org/mcp.json"),
    )]);
    json_manifest::write_manifest(&[segment], &residue)
}

/// The committed MCP manifest under `harness` — the repository root itself, not a
/// `.claude/` interior.
fn mcp_path(harness: &Path) -> std::path::PathBuf {
    harness.join(".mcp.json")
}

/// The lock's rollup rows for `kind` — the `[[<kind>]]` array of tables, as
/// `(name, source_path, emit_hash)` triples. Empty when the family is absent.
fn rollup_rows(into: &Path, kind: &str) -> Vec<(String, String, String)> {
    let doc = fs::read_to_string(into.join("lock.toml"))
        .unwrap()
        .parse::<toml_edit::DocumentMut>()
        .unwrap();
    let Some(rows) = doc.get(kind).and_then(|item| item.as_array_of_tables()) else {
        return Vec::new();
    };
    rows.iter()
        .map(|row| {
            let column = |key: &str| {
                row.get(key)
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string()
            };
            (column("name"), column("source_path"), column("emit_hash"))
        })
        .collect()
}

#[test]
fn the_mcp_kind_crosses_the_seam_as_a_committed_whole_file_json_document() {
    let (_harness, into) = common::wire_sdk_harness("mcp-facts", MCP_MEMBER_PROGRAM);

    drift::emit_program(&into, EmitOptions::default()).unwrap();

    let declarations = drift::read_declarations(&into).unwrap();
    let facts = declarations
        .kinds
        .iter()
        .find(|row| row.name == "mcp")
        .expect("the `mcp` kind the program imported crosses the seam as a kind-fact row");

    // The documented singleton locus: the harness root, and a literal glob, so the
    // member's name never splices into the path.
    assert_eq!(facts.governs_root.as_deref(), Some("."));
    assert_eq!(facts.governs_glob.as_deref(), Some(".mcp.json"));
    // Committed, not local: absent is the class, and it is what makes the file an emit target.
    assert_eq!(facts.commitment, None);
    // The whole-file JSON format routes it to the document reader, like the settings pair.
    assert_eq!(facts.format.as_deref(), Some("json-document"));
    assert_eq!(facts.unit_shape.as_deref(), Some("file"));
    // It is the container of the `mcpServers` address, never a member at it — and it
    // reaches the model on no channel of its own.
    assert_eq!(facts.collection_address, None);
    assert_eq!(facts.shape, None);
    assert!(facts.registration.is_empty());

    // The same file, two faces: the container governs the document; `mcp-server` keys
    // inside it. Neither is the other's collection.
    let server = declarations
        .kinds
        .iter()
        .find(|row| row.name == "mcp-server")
        .expect("the server kind rides the same import");
    let address = server
        .collection_address
        .as_ref()
        .expect("mcp-server keys at a collection address");
    assert_eq!(address.manifest, ".mcp.json");
    assert_eq!(address.key_path, "mcpServers.*");
}

#[test]
fn emit_renders_the_mcp_file_whole_and_rolls_up_its_byte_fingerprint() {
    let (harness, into) = common::wire_sdk_harness("mcp-emit", MCP_MEMBER_PROGRAM);

    let report = drift::emit_program(&into, EmitOptions::default()).unwrap();
    let entry = report
        .entries
        .iter()
        .find(|entry| entry.kind == "mcp")
        .expect("the container member is one emit entry, labelled by its own kind");
    assert_eq!(entry.outcome, EmitOutcome::Emitted);

    // The bytes are the canonical write face's: the declared collection segment first,
    // then the undocumented key as opaque residue — one file, two authorship routes, and
    // the residue survives verbatim rather than being normalized away.
    let path = mcp_path(&harness);
    let expected = expected_manifest();
    let written = fs::read_to_string(&path).unwrap();
    assert_eq!(written, expected);
    assert!(
        written.contains("\"$schema\": \"https://json.schemastore.org/mcp.json\""),
        "the undocumented key rides the residue intact: {written}"
    );

    // The container's rollup row carries the file's own byte fingerprint — segment and
    // residue alike. Without it nothing in the lock describes these bytes.
    let rows = rollup_rows(&into, "mcp");
    assert_eq!(
        rows.len(),
        1,
        "one container member, one rollup row: {rows:?}"
    );
    let (name, source_path, emit_hash) = &rows[0];
    assert_eq!(name, ".mcp");
    assert_eq!(source_path, ".mcp.json");
    let mut hasher = Sha256::new();
    hasher.update(expected.as_bytes());
    assert_eq!(emit_hash, &format!("{:x}", hasher.finalize()));

    // A second, independent compile reproduces every byte and reports the idempotent no-op.
    let second = drift::emit_program(&into, EmitOptions::default()).unwrap();
    assert_eq!(
        second
            .entries
            .iter()
            .find(|entry| entry.kind == "mcp")
            .map(|entry| entry.outcome),
        Some(EmitOutcome::Unchanged)
    );
    assert_eq!(fs::read_to_string(&path).unwrap(), expected);
}

#[test]
fn a_hand_edit_to_either_the_segment_or_the_residue_reports_config_stale() {
    // Two edits, one per authorship route, each against a freshly compiled harness: the
    // fingerprint is over the whole file, so neither half can be touched invisibly.
    for (label, edit) in [
        (
            "residue",
            (
                "https://json.schemastore.org/mcp.json",
                "https://example.invalid/mcp.json",
            ),
        ),
        (
            "segment",
            ("https://example.com/mcp", "https://evil.test/mcp"),
        ),
    ] {
        let (harness, into) =
            common::wire_sdk_harness(&format!("mcp-stale-{label}"), MCP_MEMBER_PROGRAM);
        drift::emit_program(&into, EmitOptions::default()).unwrap();

        let path = mcp_path(&harness);
        let before = fs::read_to_string(&path).unwrap();
        let (needle, replacement) = edit;
        let after = before.replace(needle, replacement);
        assert_ne!(
            before, after,
            "the {label} edit must actually change the file"
        );
        fs::write(&path, &after).unwrap();

        let (findings, _ok) = common::check_harness(&harness);
        let stale = common::findings_for(&findings, "root.fresh");
        assert_eq!(
            stale.len(),
            1,
            "a hand edit to the {label} half reports exactly one stale projection: {findings:?}"
        );
        assert!(
            stale[0].contains(".mcp.json"),
            "the finding names the file that moved: {}",
            stale[0]
        );
    }
}

#[test]
fn the_servers_under_mcp_servers_stay_mcp_server_members_rather_than_container_fields() {
    let (harness, into) = common::wire_sdk_harness("mcp-members", MCP_MEMBER_PROGRAM);
    drift::emit_program(&into, EmitOptions::default()).unwrap();

    // The container's residue is the file's undocumented keys and nothing else:
    // `mcpServers` is an address, so it belongs to neither half of the container's key
    // space — a container that swallowed it would show it here.
    let declarations = drift::read_declarations(&into).unwrap();
    let registrations: Vec<_> = declarations
        .registrations
        .iter()
        .filter(|row| row.kind == "mcp-server")
        .collect();
    assert_eq!(registrations.len(), 1, "{registrations:?}");
    assert_eq!(registrations[0].key, "docs");
    assert_eq!(registrations[0].key_path, "mcpServers.*");

    // And `check` counts them apart: one container member of kind `mcp`, one connection of
    // kind `mcp-server`, both announced as checked rather than one silently absorbing the
    // other.
    let (findings, ok) = common::check_harness(&harness);
    assert!(ok, "a governed .mcp.json gates clean: {findings:?}");
    assert!(
        findings.iter().any(|f| f.contains("mcp (1)")),
        "the container member is announced as checked: {findings:?}"
    );
    assert!(
        findings.iter().any(|f| f.contains("mcp-server (1)")),
        "the connection is still a member of its own kind: {findings:?}"
    );
    // Every key of the file is governed — the segment by its own kind, the residue by the
    // container — so no unmodeled-surface gap is left to name.
    assert!(
        common::findings_for(&findings, "coverage.unmodeled-surface").is_empty(),
        "{findings:?}"
    );
}
