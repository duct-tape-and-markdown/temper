//! A layout kind's document, read under its declared layout.
//!
//! Two faces of the one governed source (`specs/model/pipeline.md`, "Emit"):
//!
//! - **the reader**, `Layout::read`, driven directly over a real document — field
//!   sections fill named slots with their verbatim spans; a member collection turns
//!   each child heading into a member carrying slugged-heading identity, an explicit
//!   key surviving a retitle; a verbatim prose region lands the preamble; a document
//!   that does not fit the declared layout refuses loud, naming the file and heading.
//! - **emit**, `drift::emit`, driven over a payload carrying a layout member — the
//!   document is a source: emit derives its members into the lock's `nested_member`
//!   family and writes nothing at its path, never regenerating or reaping it.

use std::fs;

use temper::drift::{
    self, Declarations, EmitOptions, KindFactRow, LayoutRegionRow, LayoutRow, Payload,
    PayloadMember,
};
use temper::kind::{Layout, LayoutError, LayoutRegion};

mod common;

const INTENT_DOC: &str = "The product intent, authored in prose.\n\
\n\
# Intent\n\
temper makes a harness good.\n\
\n\
# Invariants\n\
\n\
## Loud or nothing\n\
A gate never fabricates absence.\n\
\n\
## The projection is not the database\n\
Facts are declared, never mined back.\n";

/// The `intent` layout: a leading prose region, an `intent` field section, and a
/// member collection of `invariant`s.
fn intent_layout() -> Layout {
    Layout {
        regions: vec![
            LayoutRegion::Prose { import: None },
            LayoutRegion::Field {
                slot: "intent".to_string(),
            },
            LayoutRegion::Collection {
                member_kind: "invariant".to_string(),
                key: None,
            },
        ],
    }
}

#[test]
fn the_reader_fills_slots_collects_members_and_lands_prose_verbatim() {
    let reading = intent_layout()
        .read(INTENT_DOC, std::path::Path::new("specs/intent.md"))
        .unwrap();

    // A field section fills its named slot with the heading's verbatim span.
    assert_eq!(
        reading.fields.get("intent").map(String::as_str),
        Some("temper makes a harness good.")
    );

    // A prose region lands the document preamble verbatim.
    assert_eq!(
        reading.prose,
        vec!["The product intent, authored in prose.".to_string()]
    );

    // Each collection member carries its slugged-heading identity, in document order.
    let ids: Vec<&str> = reading.members.iter().map(|m| m.key.as_str()).collect();
    assert_eq!(
        ids,
        vec!["loud-or-nothing", "the-projection-is-not-the-database"]
    );
    assert!(reading.members.iter().all(|m| m.member_kind == "invariant"));
}

#[test]
fn an_explicit_key_survives_a_heading_retitle() {
    let layout = Layout {
        regions: vec![LayoutRegion::Collection {
            member_kind: "invariant".to_string(),
            key: Some("id".to_string()),
        }],
    };
    let doc = |heading: &str| {
        format!("# Invariants\n\n## {heading}\nthe member body.\n### id\nloud-or-nothing\n")
    };

    let first = layout
        .read(
            &doc("Loud or nothing"),
            std::path::Path::new("specs/intent.md"),
        )
        .unwrap();
    let retitled = layout
        .read(&doc("Fail loud"), std::path::Path::new("specs/intent.md"))
        .unwrap();

    // Identity reads off the `id` sub-heading, so a heading retitle leaves it untouched
    // — where the default slugged identity would have moved with the heading.
    assert_eq!(first.members[0].key, "loud-or-nothing");
    assert_eq!(retitled.members[0].key, "loud-or-nothing");
    assert_eq!(retitled.members[0].heading, "Fail loud");
}

#[test]
fn a_document_missing_a_declared_section_fails_loud_naming_file_and_heading() {
    // The layout declares a field section and a collection, but the document carries a
    // single top-level heading — the collection has none.
    let doc = "lead prose\n\n# Intent\nthe intent\n";
    let err = intent_layout()
        .read(doc, std::path::Path::new("specs/intent.md"))
        .unwrap_err();
    let rendered = err.to_string();
    assert!(
        rendered.contains("specs/intent.md"),
        "names file: {rendered}"
    );
    assert!(rendered.contains("invariant"), "names heading: {rendered}");
    assert!(matches!(err, LayoutError::MissingSection { .. }));
}

#[test]
fn an_unadmitted_top_level_heading_refuses_loud() {
    // A field section consumes the first heading; the second top-level heading fits no
    // declared region — structure no primitive admits.
    let layout = Layout {
        regions: vec![LayoutRegion::Field {
            slot: "intent".to_string(),
        }],
    };
    let doc = "# Intent\nthe intent\n\n# Stray\nunadmitted\n";
    let err = layout
        .read(doc, std::path::Path::new("specs/intent.md"))
        .unwrap_err();
    assert!(matches!(err, LayoutError::Unadmitted { .. }));
    assert!(err.to_string().contains("Stray"));
}

/// The `intent` kind's fact row — a layout kind governing the single `specs/intent.md`
/// document, carrying the `intent_layout` regions in wire form.
fn intent_kind_facts() -> KindFactRow {
    KindFactRow {
        name: "intent".to_string(),
        provider: None,
        governs_root: "specs".to_string(),
        governs_glob: "intent.md".to_string(),
        format: None,
        unit_shape: None,
        registration: Vec::new(),
        templates: Vec::new(),
        content: Some(LayoutRow {
            regions: vec![
                LayoutRegionRow {
                    region: "prose".to_string(),
                    import: None,
                    slot: None,
                    member_kind: None,
                    key: None,
                },
                LayoutRegionRow {
                    region: "field".to_string(),
                    import: None,
                    slot: Some("intent".to_string()),
                    member_kind: None,
                    key: None,
                },
                LayoutRegionRow {
                    region: "collection".to_string(),
                    import: None,
                    slot: None,
                    member_kind: Some("invariant".to_string()),
                    key: None,
                },
            ],
        }),
    }
}

fn intent_payload() -> Payload {
    Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![intent_kind_facts()],
            ..Default::default()
        },
        members: vec![PayloadMember {
            kind: "intent".to_string(),
            name: "intent".to_string(),
            fields: Vec::new(),
            body: String::new(),
            source_path: None,
        }],
    }
}

#[test]
fn emit_derives_layout_members_into_the_lock_and_leaves_the_document_untouched() {
    let harness = common::tmpdir("layout-emit");
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    let doc_path = harness.join("specs").join("intent.md");
    fs::create_dir_all(doc_path.parent().unwrap()).unwrap();
    fs::write(&doc_path, INTENT_DOC).unwrap();

    let report = drift::emit(&intent_payload(), &into, EmitOptions::default()).unwrap();

    // The document is a source: emit writes nothing at its path — byte-untouched — and
    // no entry projects or reaps it.
    assert_eq!(fs::read_to_string(&doc_path).unwrap(), INTENT_DOC);
    assert!(
        report.entries.iter().all(|e| e.name != "intent"),
        "the layout source is neither projected nor reaped: {:?}",
        report.entries
    );

    // Its collection members reach the lock as `nested_member` declaration rows, keyed
    // off the layout host's own address.
    let declarations = drift::read_declarations(&into).unwrap();
    let ids: Vec<&str> = declarations
        .nested_members
        .iter()
        .filter(|row| row.host == "intent:intent" && row.kind == "invariant")
        .map(|row| row.key.as_str())
        .collect();
    assert_eq!(
        ids,
        vec!["loud-or-nothing", "the-projection-is-not-the-database"]
    );
}

#[test]
fn re_emitting_a_layout_source_never_reaps_it() {
    let harness = common::tmpdir("layout-reemit");
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    let doc_path = harness.join("specs").join("intent.md");
    fs::create_dir_all(doc_path.parent().unwrap()).unwrap();
    fs::write(&doc_path, INTENT_DOC).unwrap();

    drift::emit(&intent_payload(), &into, EmitOptions::default()).unwrap();
    // A second pass over the same lock: the layout source is not a lock-known projection
    // to reap — the document survives, byte-identical.
    let report = drift::emit(&intent_payload(), &into, EmitOptions::default()).unwrap();
    assert!(report.entries.iter().all(|e| e.name != "intent"));
    assert_eq!(fs::read_to_string(&doc_path).unwrap(), INTENT_DOC);
}

#[test]
fn emit_refuses_a_non_fitting_layout_document() {
    let harness = common::tmpdir("layout-nonfit");
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    let doc_path = harness.join("specs").join("intent.md");
    fs::create_dir_all(doc_path.parent().unwrap()).unwrap();
    // Only the field section's heading — the declared collection has none.
    fs::write(&doc_path, "lead\n\n# Intent\nthe intent\n").unwrap();

    let err = drift::emit(&intent_payload(), &into, EmitOptions::default()).unwrap_err();
    let rendered = format!("{err:?}");
    assert!(rendered.contains("intent.md"), "names file: {rendered}");
}
