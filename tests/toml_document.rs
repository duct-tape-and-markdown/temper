//! The `toml-document` format — a file member whose whole artifact is one TOML table.
//!
//! The `json-document` face's peer over a second grammar, and held to the same two halves:
//! the declared format routes the read (a `.toml` artifact through the frontmatter adapter
//! carries no fields at all, so a `required` clause over a key the document plainly holds is
//! the falsifier for the format being decorative), and a malformed document refuses rather
//! than degrading to an empty read the gate would judge as honest absence.
//!
//! Where the faces part is the write side: this one has none, and that is the point rather
//! than a stage. A composed member declaring the format refuses at emit; the format's real
//! consumer — a **local** member, read in place and never projected — emits nothing and
//! refuses nothing, which is the case the refusal must not break.
//!
//! The kind is the suite's own `knob` at `.claude/local/*.toml`, never the shipped `dial`
//! that declares this format: these cases drive the face's `read` directly, so the kind is
//! only what a fixture needs to name, and the walk that reaches a real document is
//! `tests/local_locus.rs`'s.

use std::fs;
use std::path::{Path, PathBuf};

mod common;

use serde_json::json;
use temper::drift::{
    ClauseRow, Declarations, EmitOptions, KindFactRow, Payload, PayloadMember, SEAM_VERSION,
};
use temper::kind::{CustomKind, Extraction, Governs, UnitShape};
use temper::toml_document;

/// A knob document: scalars, an array, and a sub-table — the shapes a TOML top level puts
/// in a member's fields, including the nested-table-as-object the read must keep whole for
/// the shared fold to type.
const KNOB_TOML: &str = r#"name = "workstation"
mode = "advisory"
gates = ["fmt", "clippy"]

[thresholds]
line-bound = 100
"#;

/// The fixture kind: a `toml-document` file kind governing the knob locus, its identity read
/// from the document's top-level `name`.
fn knob_kind() -> CustomKind {
    let mut kind = CustomKind::new(
        "knob",
        Governs {
            root: ".claude/local".to_string(),
            glob: "*.toml".to_string(),
        },
        Extraction::new(Vec::new()),
    );
    kind.format = Some(temper::kind::Format::TomlDocument);
    kind.unit_shape = Some(UnitShape::NamedField {
        field: "name".to_string(),
    });
    kind
}

/// The fixture kind's lock row — the same declaration the SDK emits, carrying the format and
/// identity labels through their closed vocabularies.
fn knob_kind_row() -> KindFactRow {
    KindFactRow {
        format: Some("toml-document".to_string()),
        unit_shape: Some("named-field(name)".to_string()),
        ..common::kind_facts("knob", ".claude/local", "*.toml")
    }
}

/// Write `body` at the knob locus under `root`.
fn write_knob(root: &Path, body: &str) -> PathBuf {
    let path = root.join(".claude/local/knob.toml");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, body).unwrap();
    path
}

#[test]
fn a_toml_document_reads_its_top_level_keys_as_the_members_fields() {
    let dir = common::tmpdir("toml-document-fields");
    let path = write_knob(&dir, KNOB_TOML);

    let member = toml_document::read(&knob_kind(), &path).unwrap();

    // Every top-level key is the member's own field — a document member claims the whole
    // document, so unlike a manifest read nothing is opaque residue.
    let fields: Vec<&str> = member.fields.keys().map(String::as_str).collect();
    assert_eq!(fields, vec!["gates", "mode", "name", "thresholds"]);
    assert_eq!(member.fields.get("mode"), Some(&json!("advisory")));
    assert_eq!(member.fields.get("gates"), Some(&json!(["fmt", "clippy"])));
    // A sub-table lands whole, in the same currency a JSON document's nested object does —
    // the shared read-time fold types it, not this face.
    assert_eq!(
        member.fields.get("thresholds"),
        Some(&json!({ "line-bound": 100 }))
    );
}

#[test]
fn identity_is_read_from_the_declared_field_never_the_filename_stem() {
    let dir = common::tmpdir("toml-document-identity");
    let path = write_knob(&dir, KNOB_TOML);

    let member = toml_document::read(&knob_kind(), &path).unwrap();

    // The stem is `knob` for every knob ever written; the declared key is what distinguishes
    // one member from the next.
    assert_eq!(member.id, "workstation");
    // The unit's frontmatter is the document's fields, so a clause ranges over a TOML
    // document exactly as it ranges over a frontmatter member.
    assert_eq!(member.to_unit().id, "workstation");
    assert_eq!(member.to_unit().frontmatter, member.fields);
    assert!(member.to_unit().body.is_empty());
}

#[test]
fn a_document_that_is_not_valid_toml_refuses_loud() {
    let dir = common::tmpdir("toml-document-malformed");
    let path = write_knob(&dir, "name = \"workstation\"\nmode =\n");

    let err = toml_document::read(&knob_kind(), &path).unwrap_err();

    // Loud or nothing: a parse failure is never degraded to a fieldless read, which would
    // hand the gate a fabricated absence to judge.
    assert!(err.to_string().contains("knob.toml"), "{err}");
}

#[test]
fn a_document_missing_its_declared_identity_key_refuses_loud() {
    let dir = common::tmpdir("toml-document-no-identity");
    let path = write_knob(&dir, "mode = \"advisory\"\n");

    let err = toml_document::read(&knob_kind(), &path).unwrap_err();

    assert!(
        err.to_string().contains("no `name` key to name it"),
        "{err}"
    );
}

#[test]
fn a_toml_document_kind_declaring_no_identity_field_refuses_loud() {
    let dir = common::tmpdir("toml-document-undeclared-identity");
    let path = write_knob(&dir, KNOB_TOML);

    // A document identity is read from a declared key; a kind naming none leaves the engine
    // nothing to read, so the read refuses rather than inventing a rule.
    let mut kind = knob_kind();
    kind.unit_shape = Some(UnitShape::File);
    let err = toml_document::read(&kind, &path).unwrap_err();
    assert!(
        err.to_string().contains("no `named-field` identity"),
        "{err}"
    );
}

#[test]
fn the_declared_format_decides_which_adapter_reads_the_artifact() {
    // The gate-path proof: a `required` clause over a key the document plainly carries.
    // Routed to the frontmatter adapter, a `.toml` artifact yields no fields at all and the
    // clause fires — so a clean run is exactly the evidence that the declared format, not
    // the file locus, chose the reader.
    let dir = common::tmpdir("toml-document-dispatch");
    write_knob(&dir, KNOB_TOML);
    common::write_lock(
        &dir,
        Declarations {
            kinds: vec![knob_kind_row()],
            clauses: vec![ClauseRow {
                label: None,
                kind: Some("knob".to_string()),
                field: Some("mode".to_string()),
                ..common::clause("required", "required")
            }],
            ..Declarations::default()
        },
    );

    let run = common::check_harness_in(&dir, Some("github"));
    assert!(run.ok, "a conforming knob gates clean: {}", run.output);
    assert!(
        !run.output.contains("mode"),
        "no finding over a field the document carries: {}",
        run.output
    );
}

#[test]
fn the_dispatch_still_judges_a_toml_document_that_omits_a_required_field() {
    // The other half of the falsifier: the same kind over a document genuinely missing the
    // key must fail — the format routes the read, it does not exempt the member from its
    // contract.
    let dir = common::tmpdir("toml-document-dispatch-fires");
    write_knob(&dir, "name = \"workstation\"\n");
    common::write_lock(
        &dir,
        Declarations {
            kinds: vec![knob_kind_row()],
            clauses: vec![ClauseRow {
                label: None,
                kind: Some("knob".to_string()),
                field: Some("mode".to_string()),
                ..common::clause("required", "required")
            }],
            ..Declarations::default()
        },
    );

    let run = common::check_harness_in(&dir, Some("github"));
    assert!(
        !run.ok,
        "a missing required field fails the gate: {}",
        run.output
    );
    assert!(run.output.contains("mode"), "{}", run.output);
}

#[test]
fn a_malformed_document_fails_the_run_rather_than_gating_against_no_fields() {
    // The vivid case for the loud read: degraded to an empty read, this run would report a
    // `mode` finding — a verdict derived from fields the engine invented the absence of. The
    // run fails on the parse instead, and the clause never gets to judge.
    let dir = common::tmpdir("toml-document-malformed-gate");
    write_knob(&dir, "name = \"workstation\"\nmode =\n");
    common::write_lock(
        &dir,
        Declarations {
            kinds: vec![knob_kind_row()],
            clauses: vec![ClauseRow {
                label: None,
                kind: Some("knob".to_string()),
                field: Some("mode".to_string()),
                ..common::clause("required", "required")
            }],
            ..Declarations::default()
        },
    );

    let run = common::check_harness_in(&dir, Some("github"));
    assert!(
        !run.ok,
        "a malformed document fails the run: {}",
        run.output
    );
    assert!(
        run.output.contains("temper::toml_document::malformed"),
        "the run names the parse failure: {}",
        run.output
    );
    assert!(
        run.findings().is_empty(),
        "the clause never judges a document that would not parse: {}",
        run.output
    );
}

/// A `<harness>/.temper` pair — `emit` derives the projection root from the workspace dir's
/// parent, the seam's own topology.
fn workspace(label: &str) -> (PathBuf, PathBuf) {
    let harness = common::tmpdir(label);
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    (harness, into)
}

/// The fixture kind's member as the seam carries it.
fn knob_member() -> PayloadMember {
    PayloadMember {
        kind: "knob".to_string(),
        name: "workstation".to_string(),
        host: None,
        fields: vec![
            ("name".to_string(), json!("workstation")),
            ("mode".to_string(), json!("advisory")),
        ],
        body: String::new(),
        source_path: None,
    }
}

#[test]
fn a_composed_toml_document_member_refuses_at_emit_rather_than_being_written() {
    // The read face is honestly read-only: there is no writer for this member to reach. The
    // refusal is what keeps that honest — without it the write dispatch falls through to the
    // frontmatter face and lands a `---`-fenced block at a `.toml` path, a member's fields on
    // disk in a format its author never declared and no reader would load back.
    let (harness, into) = workspace("toml-document-emit-refuses");
    let payload = Payload {
        version: SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![knob_kind_row()],
            ..Declarations::default()
        },
        members: vec![knob_member()],
    };

    let err = temper::drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();

    // The refusal addresses the member and names the format, so the author knows which
    // declaration to change and why the member cannot be projected.
    let message = err.to_string();
    assert!(message.contains("knob:workstation"), "{message}");
    assert!(message.contains("toml-document"), "{message}");

    // Loud or nothing: the refusal precedes output, so no artifact was written.
    assert!(
        !harness.join(".claude/local/workstation.toml").exists(),
        "a refused emit writes no bytes"
    );
    assert!(
        !harness.join(".claude/local").exists(),
        "a refused emit creates no directory"
    );
}

#[test]
fn a_local_toml_document_member_emits_nothing_and_refuses_nothing() {
    // The format's actual consumer, and the falsifier for the refusal above being too wide:
    // a local member is read-side only, so emit skips it ahead of any format question. A
    // refusal here would break the one case the face exists to serve.
    let (harness, into) = workspace("toml-document-emit-local");
    let payload = Payload {
        version: SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![KindFactRow {
                commitment: Some("local".to_string()),
                ..knob_kind_row()
            }],
            ..Declarations::default()
        },
        members: vec![knob_member()],
    };

    let report = temper::drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    assert!(
        report.entries.is_empty(),
        "a local member rows no projection: {:?}",
        report.entries
    );
    assert!(
        !harness.join(".claude/local/workstation.toml").exists(),
        "emit writes nothing at a local member's path"
    );
}

#[test]
fn an_unknown_format_label_is_a_load_error_never_a_skip() {
    // The closed vocabulary the SDK's `Format` union holds at the keystroke, held again at
    // load: joining `toml-document` widens the vocabulary by exactly one member, and a label
    // outside it stays corruption rather than degrading to no format.
    let row = KindFactRow {
        format: Some("toml-frontmatter".to_string()),
        ..knob_kind_row()
    };
    let err = CustomKind::from_kind_fact_row(&row).unwrap_err();
    assert!(err.to_string().contains("toml-frontmatter"), "{err}");
}

#[test]
fn the_refusal_precedes_include_resolution_rather_than_trailing_it() {
    // Why the refusal is sited ahead of the include resolution and not left to the write
    // dispatch's own: a member of a format nothing projects has no business resolving
    // dependencies first. Emit reaches for a dangling target here, so a refusal sited after
    // resolution would report a dangling include — a true statement about the wrong problem,
    // sending the author to fix a path when the declaration is what cannot stand.
    let (harness, into) = workspace("toml-document-emit-include");
    let payload = Payload {
        version: SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![knob_kind_row()],
            includes: vec![temper::drift::IncludeRow {
                member: "knob:workstation".to_string(),
                source_path: harness.join("absent.md").to_string_lossy().into_owned(),
            }],
            ..Declarations::default()
        },
        members: vec![PayloadMember {
            body: "Intro.\u{1}".to_string(),
            ..knob_member()
        }],
    };

    let err = temper::drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    let message = err.to_string();
    assert!(message.contains("read face only"), "{message}");
    assert!(
        !message.contains("dangling"),
        "the refusal names the declaration, not the include it never reached: {message}"
    );
}
