//! The `json-document` format — a file member whose whole artifact is one JSON object.
//!
//! The format's own faces, held apart from any roster: the read and write functions are
//! driven over a kind value built here, so what they prove is the format rather than the
//! kind that happens to declare it. `plugin-manifest` ships at this format and locus now
//! (`tests/plugin_manifest_kind.rs` drives that kind), so the fixture is the real
//! artifact, and the lock row the gate test writes restates the built-in's own facts.
//!
//! Its top-level keys are the member's own fields (no collection address, so nothing is
//! opaque residue), its identity is read from the declared key, and the write face renders
//! the member back byte-identically — the read↔write round trip and the double-emit
//! determinism the pipeline's "Emit" requires of every face.
//!
//! The dispatch is what makes the declared format load-bearing: a `.json` artifact read
//! through the frontmatter adapter would carry no fields at all, so a `required` clause
//! over a field the document plainly holds is the falsifier for the format being decorative.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

mod common;

use serde_json::json;
use temper::drift::{
    ClauseRow, Declarations, EmitOptions, EmitOutcome, KindFactRow, Payload, PayloadMember,
};
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

/// The peer-branch fixtures' bodies: a `yaml-frontmatter` rule and a formatless memory
/// file, the two shapes the JSON face must leave untouched.
const RULE_BODY: &str = "# Rust conventions\n\nErrors via miette/thiserror.\n";
const MEMORY_BODY: &str = "# Project\n\nMemory body.\n";

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

/// The fixture kind's member as the seam carries it — the same four fields the read face
/// surfaces off [`PLUGIN_JSON`], handed to `emit` in a deliberately unsorted order the
/// write face's canonical key order must impose.
fn plugin_member() -> PayloadMember {
    PayloadMember {
        kind: "plugin-manifest".to_string(),
        name: "formatter".to_string(),
        host: None,
        fields: vec![
            ("version".to_string(), json!("1.2.0")),
            ("name".to_string(), json!("formatter")),
            (
                "description".to_string(),
                json!("Formats the tree on demand"),
            ),
            ("author".to_string(), json!({ "name": "Ada Lovelace" })),
        ],
        body: String::new(),
        source_path: None,
    }
}

/// A `<harness>/.temper` pair — `emit` derives the projection root from the workspace
/// dir's parent, the seam's own topology.
fn workspace(label: &str) -> (PathBuf, PathBuf) {
    let harness = common::tmpdir(label);
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    (harness, into)
}

#[test]
fn emit_projects_a_json_document_member_through_the_canonical_write_face() {
    // The write half of the dispatch: undispatched, `emit` renders a `---`-fenced
    // frontmatter block over every file member — at a `.json` path, bytes no reader
    // (Claude Code's or temper's own) can load. Revert the dispatch and this case fails on
    // the very first assert.
    let (harness, into) = workspace("json-document-emit");
    let payload = Payload {
        version: temper::drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![plugin_kind_row()],
            ..Declarations::default()
        },
        members: vec![plugin_member()],
    };

    let report = temper::drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(report.entries.len(), 1);
    assert_eq!(report.entries[0].outcome, EmitOutcome::Emitted);

    let projected = harness.join(".claude-plugin").join("plugin.json");
    let emitted = fs::read_to_string(&projected).unwrap();
    assert_eq!(emitted, PLUGIN_JSON);
    assert!(
        !emitted.starts_with("---"),
        "a JSON document never carries a frontmatter block: {emitted}"
    );

    // The projection closes the loop: the read face loads the emitted bytes back as the
    // same member the payload carried, so `emit` writes what `check` reads.
    let reread = DocumentMember::read(&plugin_kind(), &projected).unwrap();
    assert_eq!(reread.id, "formatter");
    assert_eq!(
        reread.fields.keys().map(String::as_str).collect::<Vec<_>>(),
        vec!["author", "description", "name", "version"]
    );

    // A re-emit is byte-identical and reports the idempotent no-op — the determinism pair
    // routes through the one dispatch, so the JSON face is held to it too.
    let second = temper::drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(second.entries[0].outcome, EmitOutcome::Unchanged);
    assert_eq!(fs::read_to_string(&projected).unwrap(), PLUGIN_JSON);
}

#[test]
fn the_write_dispatch_leaves_a_frontmatter_member_and_a_formatless_one_exactly_as_they_were() {
    // The dispatch's other branches, pinned: `yaml-frontmatter` and a kind declaring no
    // format keep today's bytes — a frontmatter block over a body, and a body alone. The
    // format decides the face; it does not perturb the faces it did not select.
    let (harness, into) = workspace("json-document-emit-peers");
    let payload = Payload {
        version: temper::drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![
                common::rule_kind_facts(None, &[]),
                common::kind_facts("memory", ".", "CLAUDE.md"),
            ],
            ..Declarations::default()
        },
        members: vec![
            common::rule_member("rust", Some(&["src/**/*.rs"]), RULE_BODY),
            PayloadMember {
                kind: "memory".to_string(),
                name: "CLAUDE".to_string(),
                host: None,
                fields: Vec::new(),
                body: MEMORY_BODY.to_string(),
                source_path: None,
            },
        ],
    };

    temper::drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    assert_eq!(
        fs::read_to_string(harness.join(".claude/rules/rust.md")).unwrap(),
        format!("---\npaths: [\"src/**/*.rs\"]\n---\n{RULE_BODY}")
    );
    assert_eq!(
        fs::read_to_string(harness.join("CLAUDE.md")).unwrap(),
        MEMORY_BODY
    );
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
