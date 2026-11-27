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

use std::collections::BTreeSet;
use std::fs;

use temper::drift::{
    self, ClauseRow, Declarations, EmitOptions, KindFactRow, LayoutRegionRow, LayoutRow, Payload,
};
use temper::layout::{Layout, LayoutError, LayoutRegion};

use crate::common;

/// No edge-field slots — the reader treats every field section as a verbatim span, the
/// shape these non-edge cases exercise.
fn no_edges() -> BTreeSet<String> {
    BTreeSet::new()
}

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
        .read(
            INTENT_DOC,
            std::path::Path::new("specs/intent.md"),
            &no_edges(),
        )
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
    // A lone leading heading is the document's title, so the collection binds one level
    // down: `## Invariants` is the collection heading whose children are its members.
    let doc = |heading: &str| {
        format!(
            "# The product intent\n\n## Invariants\n\n### {heading}\nthe member body.\n#### id\nloud-or-nothing\n"
        )
    };

    let first = layout
        .read(
            &doc("Loud or nothing"),
            std::path::Path::new("specs/intent.md"),
            &no_edges(),
        )
        .unwrap();
    let retitled = layout
        .read(
            &doc("Fail loud"),
            std::path::Path::new("specs/intent.md"),
            &no_edges(),
        )
        .unwrap();

    // Identity reads off the `id` sub-heading, so a heading retitle leaves it untouched
    // — where the default slugged identity would have moved with the heading.
    assert_eq!(first.members[0].key, "loud-or-nothing");
    assert_eq!(retitled.members[0].key, "loud-or-nothing");
    assert_eq!(retitled.members[0].heading, "Fail loud");
}

#[test]
fn a_collection_member_missing_its_declared_explicit_key_fires_loud() {
    let layout = Layout {
        regions: vec![LayoutRegion::Collection {
            member_kind: "invariant".to_string(),
            key: Some("id".to_string()),
        }],
    };
    let doc = "# The product intent\n\n## Invariants\n\n\
### Loud or nothing\nthe member body without id subheading\n";

    let err = layout
        .read(doc, std::path::Path::new("specs/intent.md"), &no_edges())
        .unwrap_err();

    assert!(matches!(err, LayoutError::MissingKey { .. }));
    assert!(err.to_string().contains("Loud or nothing"));
    assert!(err.to_string().contains("id"));
}

#[test]
fn an_unfilled_region_reads_empty_not_loud() {
    // A prose-only document under the intent layout: only the leading prose region has
    // anything to bind — the `intent` field section and the `invariant` collection have
    // no heading. A region states what may appear, never what must, so both read empty
    // rather than refusing.
    let doc = "The product intent, authored in prose.\n";
    let reading = intent_layout()
        .read(doc, std::path::Path::new("specs/intent.md"), &no_edges())
        .unwrap();

    assert_eq!(
        reading.prose,
        vec!["The product intent, authored in prose.".to_string()]
    );
    assert!(
        !reading.fields.contains_key("intent"),
        "unfilled field slot reads absent: {:?}",
        reading.fields
    );
    assert!(
        reading.members.is_empty(),
        "unfilled collection reads zero members: {:?}",
        reading.members
    );

    // A trailing empty region: the field is filled, the collection has no heading — still
    // green, the collection empty.
    let trailing = intent_layout()
        .read(
            "lead prose\n\n# Intent\nthe intent\n",
            std::path::Path::new("specs/intent.md"),
            &no_edges(),
        )
        .unwrap();
    assert_eq!(
        trailing.fields.get("intent").map(String::as_str),
        Some("the intent")
    );
    assert!(trailing.members.is_empty());
}

#[test]
fn a_collection_members_own_span_lands_under_the_reserved_prose_leaf() {
    // The shape a real spec member takes: a member heading, the member's own paragraph,
    // and a `**Test.**` line — no sub-headings at all, so the sub-heading leaves are the
    // empty set and the own span is every authored word the member carries.
    let layout = Layout {
        regions: vec![LayoutRegion::Collection {
            member_kind: "invariant".to_string(),
            key: None,
        }],
    };
    let doc = "# The product intent\n\n\
## Invariants\n\n\
### Loud or nothing\n\
A failure temper can detect is an error message at author-time.\n\
\n\
**Test.** `tests/gate_fail_loud.rs`\n";

    let reading = layout
        .read(doc, std::path::Path::new("specs/intent.md"), &no_edges())
        .unwrap();

    let member = &reading.members[0];
    assert_eq!(member.key, "loud-or-nothing");

    // Non-vacuity first (0048's conservation bar): the span the reader captured carries
    // the member's authored words — a green assertion over an empty capture would prove
    // nothing.
    let own_span = member
        .leaves
        .get(temper::layout::OWN_SPAN_LEAF)
        .expect("the member's own span lands under the reserved leaf");
    assert!(!own_span.is_empty(), "the captured own span is non-empty");
    assert_eq!(
        own_span,
        "A failure temper can detect is an error message at author-time.\n\n\
**Test.** `tests/gate_fail_loud.rs`"
    );

    // And it is the member's whole leaf set — a sub-headingless member used to
    // materialize an empty map, its every authored word reaching no row at all.
    assert_eq!(member.leaves.len(), 1, "leaves: {:?}", member.leaves);
}

#[test]
fn a_collection_members_own_span_cuts_at_its_first_sub_heading() {
    // `HeadingNode::body` runs to the member's end with its sub-headings included as
    // text, so the own span has to be cut at the first child heading — else it would
    // duplicate every sub-heading leaf's words back into `prose`.
    let layout = Layout {
        regions: vec![LayoutRegion::Collection {
            member_kind: "invariant".to_string(),
            key: None,
        }],
    };
    let doc = "# The product intent\n\n\
## Invariants\n\n\
### Loud or nothing\n\
the member's own words\n\
\n\
#### Rationale\n\
a gate that cries wolf gets disabled\n\
\n\
##### Deeper\n\
nested under the rationale\n";

    let reading = layout
        .read(doc, std::path::Path::new("specs/intent.md"), &no_edges())
        .unwrap();
    let leaves = &reading.members[0].leaves;

    assert_eq!(
        leaves
            .get(temper::layout::OWN_SPAN_LEAF)
            .map(String::as_str),
        Some("the member's own words")
    );
    // The sub-heading keeps its own leaf, deeper text included as text — untouched by
    // the own span's arrival.
    assert_eq!(
        leaves.get("rationale").map(String::as_str),
        Some("a gate that cries wolf gets disabled\n\n##### Deeper\nnested under the rationale")
    );
    assert_eq!(leaves.len(), 2, "leaves: {leaves:?}");

    // A member that authored no own span declares no `prose` leaf — an absent span is
    // nothing to conserve, not an empty string to store.
    let spanless = layout
        .read(
            "# The product intent\n\n## Invariants\n\n\
### Loud or nothing\n#### Rationale\nonly the sub-heading\n",
            std::path::Path::new("specs/intent.md"),
            &no_edges(),
        )
        .unwrap();
    assert!(
        !spanless.members[0]
            .leaves
            .contains_key(temper::layout::OWN_SPAN_LEAF),
        "leaves: {:?}",
        spanless.members[0].leaves
    );
}

#[test]
fn a_sub_heading_slugging_to_the_reserved_leaf_refuses_loud() {
    // The reserved key is one leaf name, so a sub-heading slugging to it is a coincident
    // leaf address — refused, never resolved by precedence (0051).
    let layout = Layout {
        regions: vec![LayoutRegion::Collection {
            member_kind: "invariant".to_string(),
            key: None,
        }],
    };
    let doc = "# The product intent\n\n\
## Invariants\n\n\
### Loud or nothing\n\
the member's own words\n\
\n\
#### Prose\n\
a sub-heading claiming the reserved key\n";

    let err = layout
        .read(doc, std::path::Path::new("specs/intent.md"), &no_edges())
        .unwrap_err();

    assert!(matches!(err, LayoutError::ReservedLeaf { .. }));
    assert!(err.to_string().contains("Loud or nothing"));
    assert!(err.to_string().contains("prose"));
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
        .read(doc, std::path::Path::new("specs/intent.md"), &no_edges())
        .unwrap_err();
    assert!(matches!(err, LayoutError::Unadmitted { .. }));
    assert!(err.to_string().contains("Stray"));
}

/// The `intent` kind's fact row — a layout kind governing the single `specs/intent.md`
/// document, carrying the `intent_layout` regions in wire form.
fn intent_kind_facts() -> KindFactRow {
    KindFactRow {
        content: Some(common::intent_layout_row()),
        ..common::kind_facts("intent", "specs", "intent.md")
    }
}

fn intent_payload() -> Payload {
    Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![intent_kind_facts()],
            ..Default::default()
        },
        members: vec![common::payload_member("intent", "intent", None, "")],
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

    // Each member's own span rides that same row as its reserved `prose` leaf, so the one
    // leaf family leaf predicates, leaf addresses, and `explain` range over carries the
    // member's paragraph exactly as it carries a sub-heading's (0051).
    let spans: Vec<&str> = declarations
        .nested_members
        .iter()
        .filter(|row| row.host == "intent:intent" && row.kind == "invariant")
        .filter_map(|row| row.leaves.get(temper::layout::OWN_SPAN_LEAF))
        .map(String::as_str)
        .collect();
    assert_eq!(
        spans,
        vec![
            "A gate never fabricates absence.",
            "Facts are declared, never mined back.",
        ]
    );

    // Its verbatim prose region's span reaches the lock too, under the region that took
    // it — the leading position here, where `trailing_prose_payload` proves the other.
    let prose = drift::layout_prose(&into).unwrap();
    assert_eq!(prose.len(), 1, "one capturing region, one row: {prose:?}");
    assert_eq!(prose[0].member, "intent:intent");
    assert_eq!(prose[0].region_index, 0);
    assert_eq!(prose[0].prose, "The product intent, authored in prose.");
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
fn a_keyless_retitle_of_a_collection_heading_is_a_delete_plus_a_create() {
    let harness = common::tmpdir("layout-keyless-retitle");
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    let doc_path = harness.join("specs").join("intent.md");
    fs::create_dir_all(doc_path.parent().unwrap()).unwrap();

    // The `intent` collection declares no explicit key, so a member's identity is its
    // slugged heading alone.
    let invariant_keys = |into: &std::path::Path| -> BTreeSet<(String, Option<String>)> {
        drift::read_declarations(into)
            .unwrap()
            .nested_members
            .into_iter()
            .filter(|row| row.host == "intent:intent" && row.kind == "invariant")
            .map(|row| {
                let span = row.leaves.get(temper::layout::OWN_SPAN_LEAF).cloned();
                (row.key, span)
            })
            .collect()
    };

    fs::write(&doc_path, INTENT_DOC).unwrap();
    drift::emit(&intent_payload(), &into, EmitOptions::default()).unwrap();
    let before = invariant_keys(&into);

    // Retitle one member's heading, its body byte-identical.
    let retitled = INTENT_DOC.replace("## Loud or nothing", "## Fail loud");
    assert_ne!(retitled, INTENT_DOC);
    fs::write(&doc_path, &retitled).unwrap();
    drift::emit(&intent_payload(), &into, EmitOptions::default()).unwrap();
    let after = invariant_keys(&into);

    let body = Some("A gate never fabricates absence.".to_string());
    let deleted: Vec<_> = before.difference(&after).cloned().collect();
    let created: Vec<_> = after.difference(&before).cloned().collect();

    // The old identity leaves the lock and a new one enters carrying the same body — no
    // member survives the retitle under either key, and the untouched sibling stays put.
    assert_eq!(deleted, vec![("loud-or-nothing".to_string(), body.clone())]);
    assert_eq!(created, vec![("fail-loud".to_string(), body)]);
    assert!(
        after
            .iter()
            .any(|(key, _)| key == "the-projection-is-not-the-database")
    );
    assert!(after.iter().all(|(key, _)| key != "loud-or-nothing"));
}

#[test]
fn emit_refuses_a_non_fitting_layout_document() {
    let harness = common::tmpdir("layout-nonfit");
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    let doc_path = harness.join("specs").join("intent.md");
    fs::create_dir_all(doc_path.parent().unwrap()).unwrap();
    // A trailing top-level heading no region admits — structure the layout cannot place.
    // (An unfilled region reads empty; only unplaceable structure refuses.)
    fs::write(
        &doc_path,
        "lead\n\n# Intent\nthe intent\n\n# Invariants\n\n## Loud or nothing\nbody\n\n# Stray\nunadmitted\n",
    )
    .unwrap();

    let err = drift::emit(&intent_payload(), &into, EmitOptions::default()).unwrap_err();
    let rendered = format!("{err:?}");
    assert!(rendered.contains("intent.md"), "names file: {rendered}");
}

#[test]
fn emit_refuses_an_undecodable_layout_document_by_name() {
    // The lowering half of the same read. `compose` refuses in the layout vocabulary;
    // emit reaches the document through `read_layout_document`, so its decode failure
    // must name itself too rather than arriving as an `InvalidData` I/O string under
    // the generic source-read code.
    let harness = common::tmpdir("layout-not-utf8-emit");
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    let doc_path = harness.join("specs").join("intent.md");
    fs::create_dir_all(doc_path.parent().unwrap()).unwrap();
    fs::write(&doc_path, b"lead\n\n# Intent\nthe intent \xff\n".as_slice()).unwrap();

    let err = drift::emit(&intent_payload(), &into, EmitOptions::default()).unwrap_err();
    let rendered = format!("{err:?}");
    assert!(
        rendered.contains("temper::drift::read_not_utf8"),
        "the decode failure carries its own code: {rendered}"
    );
    assert!(rendered.contains("intent.md"), "names file: {rendered}");
}

/// A lock row that **relocates the built-in `rule`** to a `decisions/*.md` locus and
/// declares the `intent_layout` as its body — every fact besides `governs`/`content`
/// deferring to the built-in, so `row_relocates_builtin` admits it. The overlay must carry
/// the row's `content` (not only `governs`/`templates`) for check to read the document
/// under this layout rather than as frontmatter.
fn relocated_rule_with_layout() -> KindFactRow {
    KindFactRow {
        content: Some(common::intent_layout_row()),
        ..common::kind_facts("rule", "decisions", "*.md")
    }
}

/// A document that fits `intent_layout` — a prose preamble, an `# Intent` field section,
/// and an `# Invariants` collection — beneath a frontmatter block, which reads as on any
/// file member of the built-in it relocates.
const FITS_LAYOUT_WITH_FRONTMATTER: &str = "---\npaths: [\"decisions/**\"]\n---\n\n\
The product intent, authored in prose.\n\
\n\
# Intent\n\
temper makes a harness good.\n\
\n\
# Invariants\n\
\n\
## Loud or nothing\n\
A gate never fabricates absence.\n";

#[test]
fn check_reads_a_relocated_builtin_document_under_its_declared_layout() {
    let root = common::tmpdir("relocated-rule-fit");
    common::write_lock(
        &root,
        Declarations {
            kinds: vec![relocated_rule_with_layout()],
            ..Default::default()
        },
    );
    let decisions = root.join("decisions");
    fs::create_dir_all(&decisions).unwrap();
    fs::write(decisions.join("0001.md"), FITS_LAYOUT_WITH_FRONTMATTER).unwrap();

    let run = common::check_in(&root, &[], None);
    // Vacuity pin: the clean verdict is over the one relocated document, so it must have
    // been discovered under the moved locus before its silence means anything.
    assert!(
        run.output.contains("rule (1)"),
        "the relocated built-in must discover the document it judges:\n{}",
        run.output
    );
    assert!(
        run.ok,
        "a fitting document must read clean under the relocated built-in's layout:\n{}",
        run.output
    );
}

#[test]
fn check_refuses_a_non_fitting_relocated_builtin_layout_document() {
    let root = common::tmpdir("relocated-rule-nonfit");
    common::write_lock(
        &root,
        Declarations {
            kinds: vec![relocated_rule_with_layout()],
            ..Default::default()
        },
    );
    let decisions = root.join("decisions");
    fs::create_dir_all(&decisions).unwrap();
    // Valid frontmatter (no fence ⇒ empty frontmatter, whole body) but NOT the layout: a
    // trailing `# Stray` heading fits no declared region. Pre-overlay this was read as
    // frontmatter and passed silently; under the layout it must refuse loud.
    fs::write(
        decisions.join("0001.md"),
        "lead\n\n# Intent\nthe intent\n\n# Invariants\n\n## Loud or nothing\nbody\n\n# Stray\nunadmitted\n",
    )
    .unwrap();

    let run = common::check_in(&root, &[], None);
    assert!(
        !run.ok,
        "a non-fitting layout document must make check exit non-zero — never a silent \
         frontmatter fallback, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("Stray"),
        "the refusal must name the unadmitted heading, got:\n{}",
        run.output
    );
}

#[test]
fn a_layout_document_that_is_not_utf8_refuses_in_the_layout_vocabulary() {
    let root = common::tmpdir("relocated-rule-not-utf8");
    common::write_lock(
        &root,
        Declarations {
            kinds: vec![relocated_rule_with_layout()],
            ..Default::default()
        },
    );
    let decisions = root.join("decisions");
    fs::create_dir_all(&decisions).unwrap();
    // A lone `0xFF`: no UTF-8 sequence begins with it, so the bytes decode to nothing —
    // there is no heading tree to cut, let alone a region to place it in.
    fs::write(
        decisions.join("0001.md"),
        b"lead\n\n# Intent\nthe intent \xff\n".as_slice(),
    )
    .unwrap();

    let run = common::check_in(&root, &[], Some("github"));
    assert!(
        !run.ok,
        "an undecodable layout document must make check exit non-zero, got:\n{}",
        run.output
    );
    // The code, not the prose: the read goes through the shared read+decode primitive
    // and the layout adapter maps it into its own vocabulary, so the refusal is coded
    // the way every other adapter's load fault is — never an uncoded read string.
    let not_utf8_code = "temper::layout::not_utf8".replace(':', "%3A");
    assert!(
        run.output.contains(&not_utf8_code),
        "the refusal carries the layout not-UTF-8 code, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("0001.md"),
        "the refusal names the undecodable document, got:\n{}",
        run.output
    );
}

/// The titled shape of [`INTENT_DOC`]: one top-level heading naming the document, its
/// own paragraph beneath it, and the layout's sections as its children.
const TITLED_INTENT_DOC: &str = "# The representation model\n\
The product intent, authored in prose.\n\
\n\
## Intent\n\
temper makes a harness good.\n\
\n\
## Invariants\n\
\n\
### Loud or nothing\n\
A gate never fabricates absence.\n\
\n\
### The projection is not the database\n\
Facts are declared, never mined back.\n";

#[test]
fn a_lone_leading_heading_is_the_title_binding_the_regions_to_its_children() {
    // A lone leading heading is the document's title: its span is the document's own
    // prose, and the regions bind to its child headings — never to the title itself,
    // which would shift every later region by one heading.
    let reading = intent_layout()
        .read(
            TITLED_INTENT_DOC,
            std::path::Path::new("specs/intent.md"),
            &no_edges(),
        )
        .unwrap();

    // The title's own span — cut at its first child heading, 0051's cut one level up —
    // is what the verbatim prose region takes, in place of the (blank) preamble.
    assert_eq!(
        reading.prose,
        vec!["The product intent, authored in prose.".to_string()]
    );

    // The regions bind one level down: the field section and the collection are the
    // title's children, not the title itself.
    assert_eq!(
        reading.fields.get("intent").map(String::as_str),
        Some("temper makes a harness good.")
    );
    let ids: Vec<&str> = reading.members.iter().map(|m| m.key.as_str()).collect();
    assert_eq!(
        ids,
        vec!["loud-or-nothing", "the-projection-is-not-the-database"]
    );

    // The title's span follows the preamble's standing rule: a layout declaring no
    // verbatim prose region drops it rather than parking it in a field slot.
    let proseless = Layout {
        regions: vec![
            LayoutRegion::Field {
                slot: "intent".to_string(),
            },
            LayoutRegion::Collection {
                member_kind: "invariant".to_string(),
                key: None,
            },
        ],
    }
    .read(
        TITLED_INTENT_DOC,
        std::path::Path::new("specs/intent.md"),
        &no_edges(),
    )
    .unwrap();
    assert!(proseless.prose.is_empty(), "prose: {:?}", proseless.prose);
    assert_eq!(
        proseless.fields.get("intent").map(String::as_str),
        Some("temper makes a harness good.")
    );
}

#[test]
fn a_heading_that_is_not_a_lone_leading_one_binds_as_it_reads() {
    // The title rule turns on `leading`: prose before the heading means the author wrote
    // a preamble, not a title, so the heading stays the regions' ground. This is the
    // `examples/base-harness/docs/glossary.md` shape — lose it and every term member goes
    // with it.
    let glossary = Layout {
        regions: vec![
            LayoutRegion::Prose { import: None },
            LayoutRegion::Collection {
                member_kind: "term".to_string(),
                key: None,
            },
        ],
    };
    let reading = glossary
        .read(
            "The words this harness uses.\n\n## Terms\n\n### Member\nOne authored unit.\n",
            std::path::Path::new("docs/glossary.md"),
            &no_edges(),
        )
        .unwrap();
    assert_eq!(
        reading.prose,
        vec!["The words this harness uses.".to_string()]
    );
    let ids: Vec<&str> = reading.members.iter().map(|m| m.key.as_str()).collect();
    assert_eq!(ids, vec!["member"], "the collection keeps its members");

    // And two top-level headings are the rule's other named exclusion: neither is a
    // title, so they bind in document order at the top level.
    let flat = intent_layout()
        .read(
            INTENT_DOC,
            std::path::Path::new("specs/intent.md"),
            &no_edges(),
        )
        .unwrap();
    assert_eq!(
        flat.fields.get("intent").map(String::as_str),
        Some("temper makes a harness good.")
    );
    assert_eq!(flat.members.len(), 2);
}

#[test]
fn a_heading_with_children_swallowed_as_field_when_a_section_is_missing_refuses_loud() {
    // A document whose declared middle section is absent, so the field region in its
    // position consumes the next top-level heading — one carrying children, meant for the
    // collection — leaving the collection unbound and every later binding shifted by one.
    // Two top-level headings, so the title rule does not reach it.
    let layout = Layout {
        regions: vec![
            LayoutRegion::Field {
                slot: "summary".to_string(),
            },
            LayoutRegion::Field {
                slot: "intent".to_string(),
            },
            LayoutRegion::Collection {
                member_kind: "invariant".to_string(),
                key: None,
            },
        ],
    };
    let doc = "# Summary\nthe summary\n\n\
# Invariants\n\n\
## Loud or nothing\nA description\n\n\
## The projection is not the database\nAnother description\n";

    let err = layout
        .read(doc, std::path::Path::new("specs/intent.md"), &no_edges())
        .unwrap_err();

    assert!(matches!(err, LayoutError::SwallowedHeading { .. }));
    assert!(
        err.to_string().contains("Invariants"),
        "must name the swallowed heading: {}",
        err
    );
    assert!(
        err.to_string().contains("intent") && err.to_string().contains("invariant"),
        "must name the region that consumed it and the one it starved: {}",
        err
    );
}

#[test]
fn a_heading_with_children_swallowed_into_a_field_refuses_when_a_non_adjacent_later_region_starves()
{
    // `## Intent` is absent, so the intent field binds `Entities` (which has children),
    // the collection binds the childless `Contract`, the contract field binds
    // `Depends on`, and only the last region starves — two regions past the swallow.
    let layout = Layout {
        regions: vec![
            LayoutRegion::Field {
                slot: "intent".to_string(),
            },
            LayoutRegion::Collection {
                member_kind: "entity".to_string(),
                key: None,
            },
            LayoutRegion::Field {
                slot: "contract".to_string(),
            },
            LayoutRegion::Field {
                slot: "depends-on".to_string(),
            },
        ],
    };
    let doc = "# Spec\n\n\
## Entities\n\n\
### Widget\nA widget.\n\n\
## Contract\nthe contract\n\n\
## Depends on\n- other\n";

    let err = layout
        .read(
            doc,
            std::path::Path::new("specs/model/thing.md"),
            &no_edges(),
        )
        .unwrap_err();

    let LayoutError::SwallowedHeading {
        swallowed,
        consumed_slot,
        starved_slot,
        ..
    } = &err
    else {
        panic!("expected a swallowed-heading refusal, got {err}");
    };
    assert_eq!(swallowed, "Entities");
    assert_eq!(consumed_slot, "intent");
    assert_eq!(starved_slot, "depends-on");
}

#[test]
fn check_refuses_a_layout_declaring_two_verbatim_prose_regions() {
    // A layout kind declaring more than one verbatim (non-import) prose region is a
    // permanent no-op: Layout::read lands the preamble in the first prose region only,
    // so a second prose region without an import can never carry a byte. The admissibility
    // gate refuses this loud as a malformed kind definition.
    let root = common::tmpdir("layout-duplicate-prose");
    common::write_lock(
        &root,
        Declarations {
            kinds: vec![KindFactRow {
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
                            slot: Some("section".to_string()),
                            member_kind: None,
                            key: None,
                        },
                        LayoutRegionRow {
                            region: "prose".to_string(),
                            import: None,
                            slot: None,
                            member_kind: None,
                            key: None,
                        },
                    ],
                }),
                ..common::kind_facts("spec", "specs", "*.md")
            }],
            ..Default::default()
        },
    );

    let run = common::check_in(&root, &[], None);
    assert!(
        !run.ok,
        "a layout declaring two verbatim prose regions must make check exit non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("duplicate-prose-region"),
        "the refusal must name the rule, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("region 0") && run.output.contains("region 2"),
        "the refusal must name both prose regions (0 and 2), got:\n{}",
        run.output
    );
}

/// The `spec` layout: two field sections, `summary` then `detail` — the two-region shape
/// the floor below is declared over, with the trailing slot the one a document may omit.
fn slot_floor_kind_facts() -> KindFactRow {
    KindFactRow {
        content: Some(LayoutRow {
            regions: ["summary", "detail"]
                .into_iter()
                .map(|slot| LayoutRegionRow {
                    region: "field".to_string(),
                    import: None,
                    slot: Some(slot.to_string()),
                    member_kind: None,
                    key: None,
                })
                .collect(),
        }),
        ..common::kind_facts("spec", "specs", "*.md")
    }
}

/// A `check` run over a lone `spec` document carrying `doc`, judged against the
/// two-field-region kind and one `required` clause over its trailing slot — the floor
/// itself, declared where the model puts it: on the selection, never on the region row.
fn slot_floor_run(label: &str, doc: &str) -> common::CheckRun {
    let root = common::tmpdir(label);
    common::write_lock(
        &root,
        Declarations {
            kinds: vec![slot_floor_kind_facts()],
            clauses: vec![ClauseRow {
                kind: Some("spec".to_string()),
                field: Some("detail".to_string()),
                ..common::clause("required", "required")
            }],
            ..Default::default()
        },
    );
    let specs = root.join("specs");
    fs::create_dir_all(&specs).unwrap();
    fs::write(specs.join("model.md"), doc).unwrap();
    common::check_in(&root, &[], None)
}

/// A layout's regions state what may appear, never what must: a document conforms with a
/// region empty, and "any floor — a required section, a minimum member count — is a
/// clause over the selection" (`specs/model/representation.md`, "kind"). So the floor over
/// a field slot is the ordinary `required` predicate naming it, and it must decide both
/// ways — firing on the absent section, silent on the present one. Neither half may pass
/// over an undiscovered document, so both assert the member reached the judged set first.
#[test]
fn a_required_clause_is_the_floor_over_a_layout_field_slot_in_both_directions() {
    // The trailing section is absent — the region reads empty, which conforms, and the
    // clause is the only thing that can object. It does, naming the slot.
    let absent = slot_floor_run(
        "layout-slot-floor-absent",
        "# Summary\nThe representation model, authored in prose.\n",
    );
    assert!(
        absent.output.contains("spec (1)"),
        "the case is vacuous unless the run judged the discovered layout member: {}",
        absent.output
    );
    assert!(
        absent.output.contains("spec.required.detail"),
        "an absent field section must fire the slot's `required` floor, naming the slot: {}",
        absent.output
    );
    assert!(
        !absent.ok,
        "a `required` floor is a blocking clause — the run fails: {}",
        absent.output
    );

    // The same document carrying that heading — nothing else changed — fills the slot, so
    // the floor is met and says nothing.
    let present = slot_floor_run(
        "layout-slot-floor-present",
        "# Summary\nThe representation model, authored in prose.\n\
         \n# Detail\nA kind declares the field schema and the content.\n",
    );
    assert!(
        present.output.contains("spec (1)"),
        "the case is vacuous unless the run judged the discovered layout member: {}",
        present.output
    );
    assert!(
        !present.output.contains("spec.required.detail"),
        "a present field section fills the slot — the floor holds silently: {}",
        present.output
    );
    assert!(
        present.ok,
        "the run carries no other blocking finding, so the absent half's failure above is \
         the floor and nothing else: {}",
        present.output
    );
}

/// The preamble of [`TRAILING_PROSE_DOC`] — the span the trailing prose region captures,
/// asserted non-empty before anything downstream is asked to carry it.
const TRAILING_PROSE_PREAMBLE: &str = "The representation model, authored in prose.";

/// A document for a layout whose prose region is declared last: a preamble, then the
/// member collection. Nothing distinguishes it on disk from the leading-prose shape —
/// the preamble binds to the one verbatim prose region wherever it is declared.
const TRAILING_PROSE_DOC: &str = "The representation model, authored in prose.\n\
\n\
# Invariants\n\
\n\
## Loud or nothing\n\
A gate never fabricates absence.\n";

/// The `intent` kind with its verbatim prose region declared **after** the collection —
/// a legal, non-first position for the one prose region a layout may declare.
fn trailing_prose_kind_facts() -> KindFactRow {
    KindFactRow {
        content: Some(LayoutRow {
            regions: vec![
                LayoutRegionRow {
                    region: "collection".to_string(),
                    import: None,
                    slot: None,
                    member_kind: Some("invariant".to_string()),
                    key: None,
                },
                LayoutRegionRow {
                    region: "prose".to_string(),
                    import: None,
                    slot: None,
                    member_kind: None,
                    key: None,
                },
            ],
        }),
        ..common::kind_facts("intent", "specs", "intent.md")
    }
}

/// The `intent` payload over [`trailing_prose_kind_facts`] — one layout member whose
/// document is already on disk.
fn trailing_prose_payload() -> Payload {
    Payload {
        declarations: Declarations {
            kinds: vec![trailing_prose_kind_facts()],
            ..Default::default()
        },
        ..intent_payload()
    }
}

#[test]
fn a_prose_region_declared_after_a_collection_captures_the_preamble_into_an_addressable_row() {
    let harness = common::scaffold("layout-trailing-prose");
    let into = harness.join(".temper");
    let doc_path = harness.join("specs").join("intent.md");
    fs::write(&doc_path, TRAILING_PROSE_DOC).unwrap();

    // Non-vacuity first: the reader really does bind the preamble to the trailing prose
    // region, so what the row must carry is a span with words in it.
    let reading = Layout {
        regions: vec![
            LayoutRegion::Collection {
                member_kind: "invariant".to_string(),
                key: None,
            },
            LayoutRegion::Prose { import: None },
        ],
    }
    .read(TRAILING_PROSE_DOC, &doc_path, &no_edges())
    .unwrap();
    assert_eq!(reading.prose, vec![TRAILING_PROSE_PREAMBLE.to_string()]);
    assert!(
        !reading.prose[0].is_empty(),
        "the case is vacuous unless the region captured a span"
    );

    drift::emit(&trailing_prose_payload(), &into, EmitOptions::default()).unwrap();

    // The captured span reaches the lock as its own row, keyed by the host's address and
    // naming the region that took it — the second region, not the first.
    let rows = drift::layout_prose(&into).unwrap();
    assert_eq!(rows.len(), 1, "one capturing region, one row: {rows:?}");
    assert_eq!(rows[0].member, "intent:intent");
    assert_eq!(rows[0].region_index, 1);
    assert_eq!(rows[0].prose, TRAILING_PROSE_PREAMBLE);

    // And the read verb narrates it: an author asking about the member is told which
    // region captured the preamble and what it captured.
    let narration = common::explain_in(&harness, "intent");
    assert!(
        narration.contains("Prose regions") && narration.contains("region 1"),
        "explain names the capturing region: {narration}"
    );
    assert!(
        narration.contains(TRAILING_PROSE_PREAMBLE),
        "explain carries the captured span: {narration}"
    );
}

#[test]
fn a_discovered_layout_document_the_lock_declares_no_member_for_surfaces_an_advisory() {
    // The lock declares the `intent` kind but no member of it — the shape an author
    // reaches by writing the document and never composing it into the program. Discovery
    // still finds `specs/intent.md` and reads it for its field slots; every other read of
    // it comes back empty, because a layout host's whole trace on the lock is the rows
    // `emit` lowered its document into and there are none.
    let root = common::tmpdir("layout-undeclared-member");
    common::write_lock(
        &root,
        Declarations {
            kinds: vec![intent_kind_facts()],
            ..Default::default()
        },
    );
    let specs = root.join("specs");
    fs::create_dir_all(&specs).unwrap();
    fs::write(specs.join("intent.md"), INTENT_DOC).unwrap();

    let run = common::check_in(&root, &[], None);

    // Non-vacuity: the run really did discover and judge the member. Without this the
    // advisory below could pass over a read that found nothing at all.
    assert!(
        run.output.contains("intent (1)"),
        "the case is vacuous unless the run judged the discovered member: {}",
        run.output
    );

    assert!(
        run.output.contains("root.locus-declared"),
        "an undeclared layout document must be named, never left as silent zero coverage: {}",
        run.output
    );
    assert!(
        run.output.contains("specs/intent.md"),
        "the finding names the document: {}",
        run.output
    );
    assert!(
        run.output.contains("re-emit") && run.output.contains("local"),
        "the finding names both remedies — declare and re-emit, or declare the kind local: {}",
        run.output
    );
    // Advisory: it states the gap, it never fails the run.
    assert!(
        run.ok,
        "the finding is advisory — the run stays green: {}",
        run.output
    );
}

#[test]
fn a_declared_layout_member_draws_no_undeclared_finding() {
    // The same document and kind, this time composed into the program: `emit` lowers it
    // into the lock's declaration rows, so the gate finds its member declared.
    let harness = common::scaffold("layout-declared-member");
    fs::write(harness.join("specs").join("intent.md"), INTENT_DOC).unwrap();
    drift::emit(
        &intent_payload(),
        &harness.join(".temper"),
        EmitOptions::default(),
    )
    .unwrap();

    let run = common::check_in(&harness, &[], None);
    assert!(
        run.output.contains("intent (1)"),
        "the case is vacuous unless the run judged the declared member: {}",
        run.output
    );
    assert!(
        !run.output.contains("root.locus-declared"),
        "a declared layout member draws no finding: {}",
        run.output
    );
}

/// A layout of **field regions alone** lowers into no declaration row at all — no
/// collection member, no captured prose, no import. Before the lock kept a
/// `layout_source` record, that emptiness was the member's whole trace and read exactly
/// like a document `emit` never saw, so a correctly declared member drew the undeclared
/// advisory — a false positive neither remedy the message offers could clear.
#[test]
fn a_declared_field_only_layout_member_draws_no_undeclared_finding() {
    const FIELD_ONLY_DOC: &str = "# Intent\ntemper makes a harness good.\n";

    let field_only_kind = KindFactRow {
        content: Some(LayoutRow {
            regions: vec![LayoutRegionRow {
                region: "field".to_string(),
                import: None,
                slot: Some("intent".to_string()),
                member_kind: None,
                key: None,
            }],
        }),
        ..common::kind_facts("intent", "specs", "intent.md")
    };
    let payload = Payload {
        declarations: Declarations {
            kinds: vec![field_only_kind],
            ..Default::default()
        },
        ..intent_payload()
    };

    let harness = common::scaffold("layout-field-only-member");
    fs::write(harness.join("specs").join("intent.md"), FIELD_ONLY_DOC).unwrap();
    drift::emit(&payload, &harness.join(".temper"), EmitOptions::default()).unwrap();

    // Non-vacuity twice over: the lowering really does yield nothing, and the run really
    // did discover and judge the member whose silence is asserted below.
    let into = harness.join(".temper");
    assert!(
        drift::read_declarations(&into)
            .unwrap()
            .nested_members
            .is_empty()
            && drift::layout_prose(&into).unwrap().is_empty(),
        "the case is vacuous unless the document lowered into no declaration row",
    );

    let run = common::check_in(&harness, &[], None);
    assert!(
        run.output.contains("intent (1)"),
        "the case is vacuous unless the run judged the declared member: {}",
        run.output
    );
    assert!(
        !run.output.contains("root.locus-declared"),
        "a declared member of a field-only layout draws no finding: {}",
        run.output
    );
}

/// The other rowless shape, and the one that survives any per-kind suppression: a
/// collection-bearing layout whose *document* holds the collection heading with no child
/// heading and no preamble. The kind declares a row-producing region; this member's body
/// simply had nothing to give it, and declaration must not turn on that.
#[test]
fn a_declared_layout_member_whose_document_lowers_to_nothing_draws_no_undeclared_finding() {
    const EMPTY_COLLECTION_DOC: &str = "# Invariants\n";

    let empty_collection_kind = KindFactRow {
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
                    region: "collection".to_string(),
                    import: None,
                    slot: None,
                    member_kind: Some("invariant".to_string()),
                    key: None,
                },
            ],
        }),
        ..common::kind_facts("intent", "specs", "intent.md")
    };
    let payload = Payload {
        declarations: Declarations {
            kinds: vec![empty_collection_kind],
            ..Default::default()
        },
        ..intent_payload()
    };

    let harness = common::scaffold("layout-empty-collection-member");
    fs::write(
        harness.join("specs").join("intent.md"),
        EMPTY_COLLECTION_DOC,
    )
    .unwrap();
    drift::emit(&payload, &harness.join(".temper"), EmitOptions::default()).unwrap();

    let into = harness.join(".temper");
    assert!(
        drift::read_declarations(&into)
            .unwrap()
            .nested_members
            .is_empty()
            && drift::layout_prose(&into).unwrap().is_empty(),
        "the case is vacuous unless the collection and the prose region both read empty",
    );

    let run = common::check_in(&harness, &[], None);
    assert!(
        run.output.contains("intent (1)"),
        "the case is vacuous unless the run judged the declared member: {}",
        run.output
    );
    assert!(
        !run.output.contains("root.locus-declared"),
        "a declared member whose document lowered into no row draws no finding: {}",
        run.output
    );
}

/// The `spec` layout kind of [`slot_floor_kind_facts`], declared in the yaml-frontmatter
/// format with one `required` clause over a field its frontmatter — not its body —
/// carries.
fn frontmatter_floor_run(label: &str, doc: &str) -> common::CheckRun {
    let root = common::tmpdir(label);
    common::write_lock(
        &root,
        Declarations {
            kinds: vec![KindFactRow {
                format: Some("yaml-frontmatter".to_string()),
                ..slot_floor_kind_facts()
            }],
            clauses: vec![ClauseRow {
                kind: Some("spec".to_string()),
                field: Some("status".to_string()),
                ..common::clause("required", "required")
            }],
            ..Default::default()
        },
    );
    let specs = root.join("specs");
    fs::create_dir_all(&specs).unwrap();
    fs::write(specs.join("model.md"), doc).unwrap();
    common::check_in(&root, &[], None)
}

/// A layout structures the body alone: the format's other slots read as on a
/// `file`-content kind, so a frontmatter field reaches the clauses beside the layout's
/// own slots — and the frontmatter block is never mistaken for body structure.
#[test]
fn a_layout_document_reads_its_frontmatter_fields() {
    let present = frontmatter_floor_run(
        "layout-frontmatter-present",
        "---\nstatus: draft\n---\n# Summary\nThe representation model.\n",
    );
    assert!(
        present.output.contains("spec (1)"),
        "the case is vacuous unless the run judged the layout member: {}",
        present.output
    );
    assert!(
        present.ok && !present.output.contains("spec.required.status"),
        "the frontmatter's `status` fills the field the clause asks for: {}",
        present.output
    );

    let absent = frontmatter_floor_run(
        "layout-frontmatter-absent",
        "---\nowner: docs\n---\n# Summary\nThe representation model.\n",
    );
    assert!(
        absent.output.contains("spec.required.status"),
        "a frontmatter lacking the field fires the floor: {}",
        absent.output
    );
}

/// The `spec` kind keyed by a frontmatter field rather than its file stem.
fn named_field_spec_kind() -> KindFactRow {
    KindFactRow {
        format: Some("yaml-frontmatter".to_string()),
        unit_shape: Some("named-field(title)".to_string()),
        content: Some(LayoutRow {
            regions: vec![
                common::field_region("summary"),
                LayoutRegionRow {
                    region: "collection".to_string(),
                    import: None,
                    slot: None,
                    member_kind: Some("invariant".to_string()),
                    key: None,
                },
            ],
        }),
        ..common::kind_facts("spec", "specs", "*.md")
    }
}

/// A layout document's identity is its kind's locus, as any file member's: a
/// `named-field` unit is named by its frontmatter value on both faces — the rows emit
/// lowers and the `layout_source` record key under it, and check reads the same id, so
/// the declared member draws no undeclared advisory.
#[test]
fn a_layout_documents_identity_follows_its_kinds_unit_shape() {
    let harness = common::scaffold("layout-named-field-identity");
    fs::write(
        harness.join("specs").join("model.md"),
        "---\ntitle: representation\n---\n# Summary\nThe model.\n\n# Invariants\n\n\
         ## Loud or nothing\nA gate never fabricates absence.\n",
    )
    .unwrap();
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![named_field_spec_kind()],
            ..Default::default()
        },
        members: vec![common::payload_member("spec", "model", None, "")],
    };
    let into = harness.join(".temper");
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    let hosts: Vec<String> = drift::read_declarations(&into)
        .unwrap()
        .nested_members
        .into_iter()
        .map(|row| row.host)
        .collect();
    assert_eq!(hosts, vec!["spec:representation".to_string()]);

    let run = common::check_in(&harness, &[], None);
    assert!(
        run.output.contains("spec (1)"),
        "the case is vacuous unless the run judged the member: {}",
        run.output
    );
    assert!(
        !run.output.contains("root.locus-declared"),
        "check reads the identity emit keyed the member under: {}",
        run.output
    );
}

/// A `check` run over the declared `specs/intent.md` layout member carrying
/// [`INTENT_DOC`]'s two `invariant` entries, neither with a `test` sub-heading, judged
/// against `clause` beside the kind's own rows — the entry-grain contract the cases
/// below declare.
fn entry_clause_run(label: &str, clause: ClauseRow) -> common::CheckRun {
    let root = common::tmpdir(label);
    let specs = root.join("specs");
    fs::create_dir_all(&specs).unwrap();
    fs::write(specs.join("intent.md"), INTENT_DOC).unwrap();
    let mut payload = intent_payload();
    payload.declarations.clauses.push(clause);
    drift::emit(&payload, &root.join(".temper"), EmitOptions::default()).unwrap();
    common::check_in(&root, &[], None)
}

#[test]
fn a_required_clause_on_a_collection_entrys_kind_fires_on_the_entry() {
    let run = entry_clause_run(
        "layout-entry-kind-clause",
        ClauseRow {
            kind: Some("invariant".to_string()),
            field: Some("test".to_string()),
            ..common::clause("required", "required")
        },
    );
    assert!(
        run.output.contains("invariant.required.test"),
        "a clause on the entry's kind must judge the entry: {}",
        run.output
    );
    assert!(!run.ok, "{}", run.output);
}

#[test]
fn a_host_path_clause_over_collection_entries_fires_on_a_violating_entry() {
    let run = entry_clause_run(
        "layout-entry-host-path-clause",
        ClauseRow {
            kind: Some("intent".to_string()),
            field: Some("invariant[*].test".to_string()),
            ..common::clause("required", "required")
        },
    );
    assert!(
        run.output.contains("intent.required.invariant[*].test"),
        "a host-path clause over the entries must judge each entry: {}",
        run.output
    );
    assert!(!run.ok, "{}", run.output);
}

#[test]
fn a_layout_kind_with_a_json_document_format_refuses_at_load() {
    // A layout structures a markdown body's heading tree; a whole-document format has no
    // such body, so the lift refuses rather than reading the document as frontmatter.
    for label in ["json-document", "toml-document"] {
        let row = KindFactRow {
            format: Some(label.to_string()),
            ..intent_kind_facts()
        };
        let err = temper::kind::CustomKind::from_kind_fact_row(&row).unwrap_err();
        let message = err.to_string();
        assert!(
            message.contains("`intent`") && message.contains(label),
            "the refusal must name the kind and its format, got: {message}"
        );

        let root = common::tmpdir("layout-non-markdown-format");
        common::write_lock(
            &root,
            Declarations {
                kinds: vec![row],
                ..Default::default()
            },
        );
        let run = common::check_in(&root, &[], None);
        assert!(
            !run.ok && run.output.contains("`intent`"),
            "check must refuse the {label} layout kind by name, got:\n{}",
            run.output
        );
    }

    for format in [None, Some("yaml-frontmatter".to_string())] {
        let row = KindFactRow {
            format,
            ..intent_kind_facts()
        };
        assert!(
            temper::kind::CustomKind::from_kind_fact_row(&row).is_ok(),
            "a markdown layout kind loads unchanged"
        );
    }
}

/// [`INTENT_DOC`] with each `invariant` entry carrying a `#### Test` field section — the
/// document both entry-grain clauses below are satisfied by.
const TESTED_INTENT_DOC: &str = "The product intent, authored in prose.\n\
\n\
# Intent\n\
temper makes a harness good.\n\
\n\
# Invariants\n\
\n\
## Loud or nothing\n\
A gate never fabricates absence.\n\
\n\
#### Test\n\
a gate test.\n\
\n\
## The projection is not the database\n\
Facts are declared, never mined back.\n\
\n\
#### Test\n\
a lock test.\n";

/// The second entry's `#### Test` section, the span a hand edit deletes.
const SECOND_ENTRY_TEST: &str = "\n#### Test\na lock test.\n";

/// The `specs/intent.md` layout member carrying [`TESTED_INTENT_DOC`], emitted once with
/// both entry-grain `required` clauses over the entries' `test` field.
fn tested_intent_harness(label: &str) -> std::path::PathBuf {
    let root = common::tmpdir(label);
    let specs = root.join("specs");
    fs::create_dir_all(&specs).unwrap();
    fs::write(specs.join("intent.md"), TESTED_INTENT_DOC).unwrap();
    let mut payload = intent_payload();
    payload.declarations.clauses.extend([
        ClauseRow {
            kind: Some("invariant".to_string()),
            field: Some("test".to_string()),
            ..common::clause("required", "required")
        },
        ClauseRow {
            kind: Some("intent".to_string()),
            field: Some("invariant[*].test".to_string()),
            ..common::clause("required", "required")
        },
    ]);
    drift::emit(&payload, &root.join(".temper"), EmitOptions::default()).unwrap();
    root
}

#[test]
fn deleting_a_layout_entrys_field_section_without_a_re_emit_fails_check() {
    let root = tested_intent_harness("layout-entry-delete-field-no-reemit");
    let before = common::check_in(&root, &[], None);
    assert!(
        before.ok,
        "every entry carries its test, so the case is vacuous unless check first passes: {}",
        before.output
    );

    let doc = root.join("specs").join("intent.md");
    fs::write(&doc, TESTED_INTENT_DOC.replace(SECOND_ENTRY_TEST, "")).unwrap();

    let run = common::check_in(&root, &[], None);
    assert!(!run.ok, "{}", run.output);
    for id in [
        "invariant.required.test",
        "intent.required.invariant[*].test",
    ] {
        assert!(
            run.output.contains(id),
            "the entry is read off the document, not the lock, so `{id}` fires: {}",
            run.output
        );
    }
}

#[test]
fn the_guard_edges_leave_a_layout_entry_edit_to_check() {
    let root = tested_intent_harness("layout-entry-edit-guard-edges");
    let doc = root.join("specs").join("intent.md");
    fs::write(&doc, TESTED_INTENT_DOC.replace(SECOND_ENTRY_TEST, "")).unwrap();

    // The pre edge over the very edit, and the shell edge after it: neither judges the
    // contract over a layout source's entries — that is check's.
    let pre = common::guard_edit_payload("specs/intent.md", SECOND_ENTRY_TEST, "");
    let post = "{\"session_id\":\"s\",\"hook_event_name\":\"PostToolUse\",\
                \"tool_name\":\"Bash\",\"tool_input\":{\"command\":\"true\"},\
                \"tool_response\":{\"stdout\":\"\",\"stderr\":\"\",\"interrupted\":false}}";
    for payload in [pre.as_str(), post] {
        let (code, output) = common::run_guard(&root, payload);
        assert_eq!(code, Some(0), "the guard lets the edit through: {output}");
        assert!(
            !output.contains("required.test") && !output.contains("invariant[*]"),
            "no guard edge judges the entry's contract: {output}"
        );
    }
}

#[test]
fn check_drops_a_satisfies_edge_removed_from_a_committed_layout_host_without_a_re_emit() {
    let root = common::tmpdir("layout-satisfies-delete-no-reemit");
    let specs = root.join("specs");
    fs::create_dir_all(&specs).unwrap();
    let doc = specs.join("guide.md");
    let authored = "# Purpose\nA guide that fills two requirements.\n\
                    \n# Satisfies\n- dev-standards\n- layout-edge-slot\n";
    fs::write(&doc, authored).unwrap();
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![common::layout_kind_facts(
                "guide",
                vec![
                    common::field_region("purpose"),
                    common::field_region("satisfies"),
                ],
            )],
            requirements: ["dev-standards", "layout-edge-slot"]
                .map(|name| common::requirement(name, true, Some("guide")))
                .to_vec(),
            ..Default::default()
        },
        members: vec![common::layout_member("guide")],
    };
    drift::emit(&payload, &root.join(".temper"), EmitOptions::default()).unwrap();
    let before = common::check_in(&root, &[], None);
    assert!(
        before.ok,
        "both fills are in the document, so the case is vacuous unless check first passes: {}",
        before.output
    );

    fs::write(&doc, authored.replace("- layout-edge-slot\n", "")).unwrap();

    let run = common::check_in(&root, &[], None);
    assert!(!run.ok, "{}", run.output);
    assert!(
        run.output.contains("layout-edge-slot"),
        "the fill is read off the document, not the lock, so the requirement reads unfilled: {}",
        run.output
    );
    assert!(
        !run.output.contains("dev-standards"),
        "the fill the document still carries stays credited: {}",
        run.output
    );
}
