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
    self, Declarations, EmitOptions, KindFactRow, LayoutRegionRow, LayoutRow, Payload,
    PayloadMember,
};
use temper::layout::{Layout, LayoutError, LayoutRegion};

mod common;

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
    let doc = |heading: &str| {
        format!("# Invariants\n\n## {heading}\nthe member body.\n### id\nloud-or-nothing\n")
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
    let doc = "# Invariants\n\n## Loud or nothing\nthe member body without id subheading\n";

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

/// The `intent_layout` regions in wire form — a leading prose region, an `intent` field
/// section, and an `invariant` member collection — shared by the layout row a `spec`-kind
/// declares and the one a relocated built-in declares.
fn intent_layout_row() -> LayoutRow {
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
    }
}

/// The `intent` kind's fact row — a layout kind governing the single `specs/intent.md`
/// document, carrying the `intent_layout` regions in wire form.
fn intent_kind_facts() -> KindFactRow {
    KindFactRow {
        content: Some(intent_layout_row()),
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
        members: vec![PayloadMember {
            kind: "intent".to_string(),
            name: "intent".to_string(),
            host: None,
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

/// A lock row that **relocates the built-in `rule`** to a `decisions/*.md` locus and
/// declares the `intent_layout` as its body — every fact besides `governs`/`content`
/// deferring to the built-in, so `row_relocates_builtin` admits it. The overlay must carry
/// the row's `content` (not only `governs`/`templates`) for check to read the document
/// under this layout rather than as frontmatter.
fn relocated_rule_with_layout() -> KindFactRow {
    KindFactRow {
        content: Some(intent_layout_row()),
        ..common::kind_facts("rule", "decisions", "*.md")
    }
}

/// A document that fits `intent_layout` — a prose preamble, an `# Intent` field section,
/// and an `# Invariants` collection — yet is **invalid as frontmatter**: the leading
/// `---` fence wraps unparseable YAML. Read under the layout the fence is ordinary
/// preamble prose; read as frontmatter (the pre-overlay path) it errors — so check
/// staying clean is proof it dispatched to the layout.
const FITS_LAYOUT_INVALID_FRONTMATTER: &str = "---\nnot: valid: yaml: [\n---\n\n\
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
    fs::write(decisions.join("0001.md"), FITS_LAYOUT_INVALID_FRONTMATTER).unwrap();

    let run = common::check_in(&root, &[], None);
    assert!(
        run.ok,
        "a fitting document must read clean under the relocated built-in's layout — \
         never through the frontmatter adapter, which its invalid `---` fence would trip:\n{}",
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
fn a_heading_with_children_swallowed_by_field_leaving_collection_unbound_refuses_loud() {
    // A document with a leading title heading that has subsections, followed by no
    // other top-level sections. The layout expects field(intent), collection(invariant).
    // The field region consumes the title heading (with children), leaving the collection
    // unbound — a heading-count mismatch that silently shifts all later regions.
    let layout = intent_layout();
    let doc = "# Title\n\n## Loud or nothing\nA description\n\n## The projection is not the database\nAnother description\n";

    let err = layout
        .read(doc, std::path::Path::new("specs/intent.md"), &no_edges())
        .unwrap_err();

    assert!(matches!(err, LayoutError::SwallowedHeading { .. }));
    assert!(
        err.to_string().contains("Title"),
        "must name the swallowed heading: {}",
        err
    );
}

#[test]
fn a_heading_with_children_swallowed_as_field_when_collection_section_missing_refuses_loud() {
    // A document where the declared field section is missing entirely, so the next
    // top-level heading (which has children and was meant to be the collection) gets
    // consumed by the field region, leaving the collection unbound.
    let layout = intent_layout();
    let doc = "# Invariants\n\n## Loud or nothing\nA description\n\n## The projection is not the database\nAnother description\n";

    let err = layout
        .read(doc, std::path::Path::new("specs/intent.md"), &no_edges())
        .unwrap_err();

    assert!(matches!(err, LayoutError::SwallowedHeading { .. }));
    assert!(
        err.to_string().contains("Invariants"),
        "must name the swallowed heading: {}",
        err
    );
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
    let narration = explain_in(&harness, "intent");
    assert!(
        narration.contains("Prose regions") && narration.contains("region 1"),
        "explain names the capturing region: {narration}"
    );
    assert!(
        narration.contains(TRAILING_PROSE_PREAMBLE),
        "explain carries the captured span: {narration}"
    );
}

/// Run `temper explain <target>` from `root`, capturing stdout+stderr.
fn explain_in(root: &std::path::Path, target: &str) -> String {
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_temper"))
        .current_dir(root)
        .arg("explain")
        .arg(target)
        .output()
        .unwrap();
    let mut narration = String::from_utf8_lossy(&out.stdout).into_owned();
    narration.push_str(&String::from_utf8_lossy(&out.stderr));
    narration
}
