//! The file locus's `local` commitment class: per-machine and uncommitted — the kind is
//! declared and reviewed, its members' documents are not.
//!
//! Three faces of the one class:
//!
//! - **check** reads a local member's document in place under whatever format its kind
//!   declares and gates it, deriving the rows the lock never carries — a layout's
//!   collection members and `satisfies` fills — at read time, under the kind the
//!   committed lock declares.
//! - **emit** writes nothing at a local member's path, rows none of it, and never reaps
//!   it — including across the transition from a committed kind to a local one, where a
//!   prior rollup row would otherwise read the live document as an orphan.
//! - **admissibility** fences the class to a *file* locus and nothing else: the class
//!   rules the read side, so every declared format serves it.
//!
//! The fixture kind is a `dial` governing `.claude/local/*.md`. No shipped kind declares
//! the class yet — the first one's face is an open fork — so the locus here is the
//! suite's own, chosen to sit in the real `.claude/` tree rather than at `.temper/`,
//! which discovery excludes as temper's own surface.

use std::fs;

use temper::drift::{
    self, Declarations, EmitOptions, KindFactRow, LayoutRegionRow, LayoutRow, Payload,
    PayloadMember,
};

mod common;

/// A `dial` document: a lead prose region, a `mode` field section, an `overrides` member
/// collection, and a `satisfies` edge section — every primitive a local member's read has
/// to carry through, in one document.
const DIAL_DOC: &str = "The machine's own dial, uncommitted.\n\
\n\
# Mode\n\
advisory\n\
\n\
# Satisfies\n\
- dial-is-governed\n\
\n\
# Overrides\n\
\n\
## Skip The Slow Gate\n\
the local override's body.\n\
\n\
## Widen The Line Bound\n\
the second override's body.\n";

/// The `dial` layout in wire form — the regions the kind's `content` column declares.
fn dial_layout_row() -> LayoutRow {
    LayoutRow {
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
                slot: Some("mode".to_string()),
                member_kind: None,
                key: None,
            },
            LayoutRegionRow {
                region: "field".to_string(),
                import: None,
                slot: Some("satisfies".to_string()),
                member_kind: None,
                key: None,
            },
            LayoutRegionRow {
                region: "collection".to_string(),
                import: None,
                slot: None,
                member_kind: Some("override".to_string()),
                key: None,
            },
        ],
    }
}

/// The `dial` kind's fact row: a **local**-locus layout kind governing
/// `.claude/local/*.md`, templating an embedded `override` layer.
fn dial_kind_facts() -> KindFactRow {
    KindFactRow {
        commitment: Some("local".to_string()),
        content: Some(dial_layout_row()),
        templates: vec![drift::TemplateRow {
            kind: "override".to_string(),
            path: None,
        }],
        ..common::kind_facts("dial", ".claude/local", "*.md")
    }
}

fn dial_payload(kind: KindFactRow) -> Payload {
    Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![kind],
            ..Default::default()
        },
        members: vec![PayloadMember {
            kind: "dial".to_string(),
            name: "dial".to_string(),
            host: None,
            fields: Vec::new(),
            body: String::new(),
            source_path: None,
        }],
    }
}

/// A harness carrying a local `dial` document on disk and `lock`'s declarations in
/// `.temper` — the shape every case here opens on.
fn scaffold(slug: &str, lock: Declarations) -> std::path::PathBuf {
    let harness = common::tmpdir(slug);
    fs::create_dir_all(harness.join(".temper")).unwrap();
    common::write_sibling(&harness, ".claude/local/dial.md", DIAL_DOC);
    common::write_lock(&harness, lock);
    harness
}

#[test]
fn emit_writes_nothing_at_a_local_members_path_and_rows_none_of_it() {
    let harness = common::tmpdir("local-emit");
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    let doc_path = harness.join(".claude").join("local").join("dial.md");
    common::write_sibling(&harness, ".claude/local/dial.md", DIAL_DOC);

    let report = drift::emit(
        &dial_payload(dial_kind_facts()),
        &into,
        EmitOptions::default(),
    )
    .unwrap();

    // The document is the author's own: byte-untouched, and no entry projects or reaps it.
    assert_eq!(fs::read_to_string(&doc_path).unwrap(), DIAL_DOC);
    assert!(
        report.entries.iter().all(|entry| entry.name != "dial"),
        "a local member is neither projected nor reaped: {:?}",
        report.entries
    );

    // The kind is declared and reviewed; its member's rows are not there to review. A
    // committed layout host would have contributed both families — that is exactly the
    // difference the class makes.
    let declarations = drift::read_declarations(&into).unwrap();
    assert_eq!(
        declarations
            .kinds
            .iter()
            .map(|k| k.name.as_str())
            .collect::<Vec<_>>(),
        vec!["dial"],
        "the kind's own row is committed"
    );
    assert_eq!(
        declarations.kinds[0].commitment.as_deref(),
        Some("local"),
        "the row carries the declared commitment class"
    );
    assert!(
        declarations.nested_members.is_empty(),
        "a local member's collection rows never enter the lock: {:?}",
        declarations.nested_members
    );
    assert!(
        declarations.satisfies.is_empty(),
        "a local member's fill rows never enter the lock: {:?}",
        declarations.satisfies
    );

    // No provenance/emit-hash row either: the lock captures the committed harness alone.
    let lock = fs::read_to_string(into.join("lock.toml")).unwrap();
    assert!(
        !lock.contains("emit_hash"),
        "a local member contributes no emit-hash rollup row:\n{lock}"
    );
}

#[test]
fn emit_never_reaps_a_document_whose_kind_turns_local() {
    let harness = common::tmpdir("local-reap");
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    let doc_path = harness.join(".claude").join("local").join("dial.md");

    // First the kind is committed and file-content: emit owns the bytes and writes them,
    // leaving a rollup row on the lock naming the path.
    let committed = KindFactRow {
        ..common::kind_facts("dial", ".claude/local", "*.md")
    };
    let mut payload = dial_payload(committed);
    payload.members[0].body = "the projected body.\n".to_string();
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert!(doc_path.exists(), "the committed projection is written");

    // Now the kind is declared local and the author owns the document. The prior rollup
    // row still names the path and this pass projects no member onto it — the reap must
    // not read it as an orphan and delete an author's uncommitted file.
    fs::write(&doc_path, DIAL_DOC).unwrap();
    let report = drift::emit(
        &dial_payload(dial_kind_facts()),
        &into,
        EmitOptions::default(),
    )
    .unwrap();

    assert_eq!(
        fs::read_to_string(&doc_path).unwrap(),
        DIAL_DOC,
        "the local document survives the transition byte-identical: {:?}",
        report.entries
    );
}

#[test]
fn check_derives_a_local_members_rows_at_read_time_under_the_declared_kind() {
    // The lock declares the kind and nothing about its member — no nested-member rows, no
    // satisfies rows. The requirement the document claims to fill is the assembly's, and
    // the `override` kind's members carry a `body` leaf the clause below ranges over.
    let harness = scaffold(
        "local-check",
        Declarations {
            kinds: vec![dial_kind_facts()],
            requirements: vec![common::requirement("dial-is-governed", true, None)],
            ..Default::default()
        },
    );

    let (findings, ok) = common::check_harness(&harness);

    // The fills the document claims reach the roster: a requirement nothing satisfies
    // would be an uncovered one, so a green run over a lock carrying no satisfies row is
    // the derivation having happened at read time.
    assert!(
        ok,
        "the local document reads and gates under the declared layout: {findings:?}"
    );
    assert!(
        common::findings_for(&findings, "requirement.unfilled").is_empty(),
        "the document's own `satisfies` fill reaches the roster: {findings:?}"
    );
}

#[test]
fn check_reads_a_local_members_document_under_the_declared_layout() {
    // The same kind over a document carrying one more top-level heading than the layout
    // has regions to bind: the read is real, so a non-fitting local document refuses
    // rather than passing unread.
    let harness = scaffold(
        "local-nonfit",
        Declarations {
            kinds: vec![dial_kind_facts()],
            ..Default::default()
        },
    );
    common::write_sibling(
        &harness,
        ".claude/local/dial.md",
        "# Mode\nadvisory\n\n# Satisfies\n\n# Overrides\n\n# Stray\nunadmitted structure.\n",
    );

    let run = common::check_harness_in(&harness, None);

    assert!(
        !run.ok,
        "a non-fitting local document refuses: {}",
        run.output
    );
    assert!(
        run.output.contains("Stray"),
        "the refusal names the unadmitted heading: {}",
        run.output
    );
}

#[test]
fn check_derives_a_local_members_collection_members_off_its_document() {
    // An `override` clause the derived members must exist to fail: a bound of at most one
    // member over a document declaring two. Nothing in the lock says the members exist,
    // so a firing clause is the read-time derivation reaching the embedded corpus.
    // The document's own `satisfies` fill is declared as a requirement here so the gate's
    // verdict rests on the count clause alone — a dangling fill would fail the run for a
    // reason that has nothing to do with the derivation under test.
    let harness = scaffold(
        "local-collection",
        Declarations {
            kinds: vec![dial_kind_facts()],
            requirements: vec![common::requirement("dial-is-governed", true, Some("dial"))],
            clauses: vec![drift::ClauseRow {
                label: None,
                kind: Some("override".to_string()),
                count: Some(drift::CountBoundRow { min: 0, max: 1 }),
                ..common::clause("count", "required")
            }],
            ..Default::default()
        },
    );

    let (findings, ok) = common::check_harness(&harness);

    let count = common::findings_for(&findings, "override.count");
    assert_eq!(
        count.len(),
        1,
        "the document's two derived `override` members breach a max-of-one bound: {findings:?}"
    );
    assert!(
        count[0].contains("skip-the-slow-gate") && count[0].contains("widen-the-line-bound"),
        "the finding names the members read off the document: {count:?}"
    );
    assert!(
        common::findings_for(&findings, "requirement.dangling").is_empty(),
        "the local member's derived fill reaches the roster, so its requirement resolves: \
         {findings:?}"
    );
    assert!(!ok, "the breached bound fails the gate: {findings:?}");
}

#[test]
fn a_local_kind_under_a_non_layout_format_is_admissible_and_its_members_rows_derive() {
    // The class rules the read side, so it fences no format out: a `file`-content local
    // kind reads through the frontmatter face exactly as a layout kind reads through the
    // layout one. The `enum` clause below is what proves the read reached the corpus —
    // the lock declares the kind and nothing about its member's fields, so a clause
    // firing on `mode` is the document's own value having derived at read time. Silence
    // here would be a member gated against nothing while looking governed.
    let harness = scaffold(
        "local-frontmatter",
        Declarations {
            kinds: vec![KindFactRow {
                commitment: Some("local".to_string()),
                format: Some("yaml-frontmatter".to_string()),
                ..common::kind_facts("dial", ".claude/local", "*.md")
            }],
            clauses: vec![drift::ClauseRow {
                kind: Some("dial".to_string()),
                field: Some("mode".to_string()),
                values: Some(vec!["block".to_string()]),
                ..common::clause("enum", "required")
            }],
            ..Default::default()
        },
    );
    common::write_sibling(
        &harness,
        ".claude/local/dial.md",
        "---\nmode: advisory\n---\n\nThe machine's own dial, uncommitted.\n",
    );

    let (findings, ok) = common::check_harness(&harness);

    assert!(
        common::findings_for(&findings, "kind.local-locus").is_empty(),
        "a local locus is admissible under any declared format: {findings:?}"
    );
    assert!(
        !ok,
        "the document's `mode` breaches the clause: {findings:?}"
    );
    assert!(
        findings.iter().any(|f| f.contains("advisory")),
        "the finding carries the value read off the document: {findings:?}"
    );
}

#[test]
fn a_local_class_on_a_kind_governing_no_glob_is_inadmissible() {
    // A commitment class is a *file* locus's fact; a nested-file kind governs no glob of
    // its own, so it has no locus to class.
    let harness = scaffold(
        "local-nested-file",
        Declarations {
            kinds: vec![KindFactRow {
                governs_root: None,
                governs_glob: None,
                commitment: Some("local".to_string()),
                content: Some(dial_layout_row()),
                ..common::kind_facts("dial", ".claude/local", "*.md")
            }],
            ..Default::default()
        },
    );

    let (findings, ok) = common::check_harness(&harness);

    assert!(!ok, "an unlocused local class fails the gate: {findings:?}");
    assert_eq!(
        common::findings_for(&findings, "kind.local-locus").len(),
        1,
        "one finding names the fence: {findings:?}"
    );
}

#[test]
fn a_commitment_label_outside_the_closed_vocabulary_refuses_at_load() {
    let harness = scaffold(
        "local-bad-label",
        Declarations {
            kinds: vec![KindFactRow {
                commitment: Some("per-machine".to_string()),
                content: Some(dial_layout_row()),
                ..common::kind_facts("dial", ".claude/local", "*.md")
            }],
            ..Default::default()
        },
    );

    let run = common::check_harness_in(&harness, None);

    assert!(
        !run.ok,
        "an out-of-vocabulary label refuses: {}",
        run.output
    );
    assert!(
        run.output.contains("per-machine"),
        "the refusal names the label it could not lift: {}",
        run.output
    );
}
