//! The `json-document` format — a file member whose whole artifact is one JSON object.
//!
//! The format inventory's second entry, proven over a custom kind fixture: no built-in
//! declares it yet, so the kind is declared on the lock exactly as a corpus would declare
//! one. Its top-level keys are the member's own fields (no collection address, so nothing
//! is opaque residue), its identity is read from the declared key, and the write face
//! renders the member back byte-identically — the read↔write round trip and the
//! double-emit determinism the pipeline's "Emit" requires of every face.
//!
//! The dispatch is what makes the declared format load-bearing: a `.json` artifact read
//! through the frontmatter adapter would carry no fields at all, so a `required` clause
//! over a field the document plainly holds is the falsifier for the format being decorative.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

mod common;

use temper::drift::{ClauseRow, Declarations, KindFactRow};
use temper::json_manifest::{DocumentMember, write_document};
use temper::kind::{CustomKind, Extraction, Governs, UnitShape};

/// A plugin manifest in the real Claude Code shape and at its real locus — the artifact
/// the format was harvested for (code.claude.com/docs/en/plugins-reference, retrieved
/// 2026-07-16). Canonical 2-space-pretty with sorted keys and a trailing LF: the byte shape
/// the write face emits, so a round trip is byte-checkable.
const PLUGIN_JSON: &str = r#"{
  "author": {
    "name": "Ada Lovelace"
  },
  "description": "Formats the tree on demand",
  "name": "formatter",
  "version": "1.2.0"
}
"#;

/// Write a plugin manifest at the real `.claude-plugin/plugin.json` locus — never a layout
/// invented for the test's convenience (`.claude/rules/rust.md`).
fn write_plugin_json(root: &Path, body: &str) {
    let dir = root.join(".claude-plugin");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("plugin.json"), body).unwrap();
}

/// The fixture kind: a `json-document` file kind governing the plugin manifest locus, its
/// identity read from the document's top-level `name`.
fn plugin_kind() -> CustomKind {
    let mut kind = CustomKind::new(
        "plugin-manifest",
        Governs {
            root: ".claude-plugin".to_string(),
            glob: "plugin.json".to_string(),
        },
        Extraction::new(Vec::new()),
    );
    kind.format = Some(temper::kind::Format::JsonDocument);
    kind.unit_shape = Some(UnitShape::NamedField {
        field: "name".to_string(),
    });
    kind
}

/// The fixture kind's lock row — the same declaration the SDK emits, carrying the format
/// and identity labels through their closed vocabularies.
fn plugin_kind_row() -> KindFactRow {
    KindFactRow {
        format: Some("json-document".to_string()),
        unit_shape: Some("named-field(name)".to_string()),
        ..common::kind_facts("plugin-manifest", ".claude-plugin", "plugin.json")
    }
}

#[test]
fn a_json_document_reads_its_top_level_keys_as_the_members_fields() {
    let dir = common::tmpdir("json-document-fields");
    write_plugin_json(&dir, PLUGIN_JSON);

    let member =
        DocumentMember::read(&plugin_kind(), &dir.join(".claude-plugin/plugin.json")).unwrap();

    // Every top-level key is the member's own field — a document member claims the whole
    // document, so unlike a manifest read nothing is opaque residue.
    let fields: Vec<&str> = member.fields.keys().map(String::as_str).collect();
    assert_eq!(fields, vec!["author", "description", "name", "version"]);
    assert_eq!(
        member.fields.get("description"),
        Some(&serde_json::Value::from("Formats the tree on demand"))
    );
    // Raw, unprojected: a nested object stays whole for the shared read-time fold to type.
    assert!(
        member
            .fields
            .get("author")
            .is_some_and(serde_json::Value::is_object)
    );
}

#[test]
fn identity_is_read_from_the_declared_field_never_the_filename_stem() {
    let dir = common::tmpdir("json-document-identity");
    write_plugin_json(&dir, PLUGIN_JSON);

    let member =
        DocumentMember::read(&plugin_kind(), &dir.join(".claude-plugin/plugin.json")).unwrap();

    // The stem is `plugin` for every plugin manifest ever written; the declared key is what
    // distinguishes one member from the next.
    assert_eq!(member.id, "formatter");
    assert_eq!(member.to_unit().id, "formatter");
    // The unit's frontmatter is the document's fields, so a clause ranges over a JSON
    // document exactly as it ranges over a frontmatter member.
    assert_eq!(member.to_unit().frontmatter, member.fields);
    assert!(member.to_unit().body.is_empty());
}

#[test]
fn a_document_missing_its_declared_identity_key_refuses_loud() {
    let dir = common::tmpdir("json-document-no-identity");
    write_plugin_json(&dir, "{\n  \"description\": \"Nameless\"\n}\n");

    let err =
        DocumentMember::read(&plugin_kind(), &dir.join(".claude-plugin/plugin.json")).unwrap_err();

    // Loud or nothing: a nameless document is never degraded to a member named by its stem.
    assert!(
        err.to_string().contains("no `name` key to name it"),
        "{err}"
    );
}

#[test]
fn a_json_document_kind_declaring_no_identity_field_refuses_loud() {
    let dir = common::tmpdir("json-document-undeclared-identity");
    write_plugin_json(&dir, PLUGIN_JSON);

    // A `json-document` identity is read from a declared key; a kind naming none leaves the
    // engine nothing to read, so the read refuses rather than inventing a rule.
    let mut kind = plugin_kind();
    kind.unit_shape = Some(UnitShape::File);
    let err = DocumentMember::read(&kind, &dir.join(".claude-plugin/plugin.json")).unwrap_err();
    assert!(
        err.to_string().contains("no `named-field` identity"),
        "{err}"
    );
}

#[test]
fn a_member_re_emits_byte_identically_and_a_double_emit_is_stable() {
    let dir = common::tmpdir("json-document-round-trip");
    write_plugin_json(&dir, PLUGIN_JSON);

    let member =
        DocumentMember::read(&plugin_kind(), &dir.join(".claude-plugin/plugin.json")).unwrap();

    // Read↔write: the whole artifact re-renders from the member's fields alone, byte for
    // byte — the format carries no residue outside the member.
    let emitted = write_document(&member.fields);
    assert_eq!(emitted, PLUGIN_JSON);

    // A double-emit is byte-identical: the write face is a pure function of its input.
    assert_eq!(write_document(&member.fields), emitted);

    // And the round trip closes: re-reading the emitted bytes yields the same member.
    let reemitted = DocumentMember::parse(
        &plugin_kind(),
        &dir.join(".claude-plugin/plugin.json"),
        &emitted,
    )
    .unwrap();
    assert_eq!(reemitted, member);
}

#[test]
fn an_empty_document_emits_the_empty_object() {
    // The write face's floor, LF-terminated like every other artifact it renders.
    assert_eq!(write_document(&BTreeMap::new()), "{}\n");
}

#[test]
fn the_declared_format_decides_which_adapter_reads_the_artifact() {
    // The gate-path proof: a `required` clause over a key the document plainly carries.
    // Routed to the frontmatter adapter, a `.json` artifact yields no fields at all and the
    // clause fires — so a clean run is exactly the evidence that the declared format, not
    // the file locus, chose the reader.
    let dir = common::tmpdir("json-document-dispatch");
    write_plugin_json(&dir, PLUGIN_JSON);
    common::write_lock(
        &dir,
        Declarations {
            kinds: vec![plugin_kind_row()],
            clauses: vec![ClauseRow {
                kind: Some("plugin-manifest".to_string()),
                field: Some("description".to_string()),
                ..common::clause("required", "required")
            }],
            ..Declarations::default()
        },
    );

    let run = common::check_in(&dir, &["--harness", dir.to_str().unwrap()], Some("github"));
    assert!(
        run.ok,
        "a conforming plugin manifest gates clean: {}",
        run.output
    );
    assert!(
        !run.output.contains("description"),
        "no finding over a field the document carries: {}",
        run.output
    );
}

#[test]
fn the_dispatch_still_judges_a_json_document_that_omits_a_required_field() {
    // The other half of the falsifier: the same kind over a document genuinely missing the
    // key must fail — the format routes the read, it does not exempt the member from its
    // contract.
    let dir = common::tmpdir("json-document-dispatch-fires");
    write_plugin_json(
        &dir,
        "{\n  \"name\": \"formatter\",\n  \"version\": \"1.2.0\"\n}\n",
    );
    common::write_lock(
        &dir,
        Declarations {
            kinds: vec![plugin_kind_row()],
            clauses: vec![ClauseRow {
                kind: Some("plugin-manifest".to_string()),
                field: Some("description".to_string()),
                ..common::clause("required", "required")
            }],
            ..Declarations::default()
        },
    );

    let run = common::check_in(&dir, &["--harness", dir.to_str().unwrap()], Some("github"));
    assert!(
        !run.ok,
        "a missing required field fails the gate: {}",
        run.output
    );
    assert!(run.output.contains("description"), "{}", run.output);
}

#[test]
fn an_unknown_format_label_is_a_load_error_never_a_skip() {
    // The closed vocabulary the SDK's `Format` union holds at the keystroke, held again at
    // load: a label outside it is corruption, refused rather than degraded to no format
    // (which would silently route the member to the wrong adapter).
    let row = KindFactRow {
        format: Some("toml-frontmatter".to_string()),
        ..plugin_kind_row()
    };
    let err = CustomKind::from_kind_fact_row(&row).unwrap_err();
    assert!(err.to_string().contains("toml-frontmatter"), "{err}");
}
