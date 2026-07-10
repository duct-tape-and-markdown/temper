//! A layout field section the kind marks as an edge field — an edge slot.
//!
//! A field section whose slot is one of the kind's edge fields (`satisfies` among them)
//! carries addresses, not a verbatim span (`specs/model/representation.md`, "kind"). Emit
//! derives its entries into ordinary edge rows in the lock's own `satisfies` family — the
//! same family a file-content member's SDK-emitted fills land in — so a layout host's
//! fills reach the roster/coverage/read tiers exactly as any other member's do. A dangling
//! entry needs no new refusal: the derived row flows into the gate's existing
//! `requirement.dangling` check.

use std::collections::BTreeMap;
use std::fs;

use temper::check::Severity;
use temper::compose::Requirement;
use temper::coverage;
use temper::drift::{
    self, Declarations, EmitOptions, KindFactRow, LayoutRegionRow, LayoutRow, Payload,
    PayloadMember,
};
use temper::extract::Features;
use temper::read;

mod common;

/// A layout kind governing a single lone `.md` document under `specs/`, carrying the
/// given ordered region rows — the layout host every case here builds a member of. Its
/// kind facts ride the one home (`common::kind_facts`), overriding only `content`.
fn layout_kind(name: &str, regions: Vec<LayoutRegionRow>) -> KindFactRow {
    KindFactRow {
        content: Some(LayoutRow { regions }),
        ..common::kind_facts(name, "specs", &format!("{name}.md"))
    }
}

/// A `field` region row filling `slot` — an edge slot when `slot` is one of the kind's
/// edge fields, an ordinary field section otherwise.
fn field_region(slot: &str) -> LayoutRegionRow {
    LayoutRegionRow {
        region: "field".to_string(),
        import: None,
        slot: Some(slot.to_string()),
        member_kind: None,
        key: None,
    }
}

/// A layout member of `kind`, its document already on disk (a source, never projected).
fn layout_member(kind: &str) -> PayloadMember {
    PayloadMember {
        kind: kind.to_string(),
        name: kind.to_string(),
        fields: Vec::new(),
        body: String::new(),
        source_path: None,
    }
}

/// Lay out a `<harness>/.temper` workspace and the `specs/` tree a layout document sits
/// under, returning the harness root.
fn scaffold(slug: &str) -> std::path::PathBuf {
    let harness = common::tmpdir(slug);
    fs::create_dir_all(harness.join(".temper")).unwrap();
    fs::create_dir_all(harness.join("specs")).unwrap();
    harness
}

/// The `guide` host's `satisfies` fill claims as the lock carries them, in derived order
/// — the exact rows emit wrote, read straight back off the committed lock.
fn guide_fills(into: &std::path::Path) -> Vec<String> {
    drift::read_declarations(into)
        .unwrap()
        .satisfies
        .into_iter()
        .filter(|row| row.member == "guide")
        .map(|row| row.requirement)
        .collect()
}

/// A bare `Features` for the `guide` host carrying only its id and `satisfies` fills —
/// the shape the coverage gate and the `why` read verb both range over.
fn guide_features(fills: &[String]) -> Features {
    Features {
        id: "guide".to_string(),
        fields: BTreeMap::new(),
        body_lines: 0,
        headings: Vec::new(),
        sections: Vec::new(),
        source_dir: None,
        directives: Vec::new(),
        fenced_blocks: Vec::new(),
        nested_members: Vec::new(),
        satisfies: fills.to_vec(),
    }
}

/// A named requirement the coverage gate ranges over — `required`, no other facet.
fn requirement(name: &str) -> Requirement {
    Requirement {
        name: name.to_string(),
        prose: Some(format!("the intent behind {name}")),
        kind: None,
        required: true,
        clauses: Vec::new(),
        verified_by: None,
    }
}

#[test]
fn a_satisfies_edge_slot_derives_fill_rows_the_gate_and_read_verbs_range_over() {
    let harness = scaffold("layout-edge-slot-satisfies");
    let into = harness.join(".temper");
    // A `guide` document with an ordinary field section (a verbatim span) followed by a
    // `satisfies` edge slot whose entries are a bulleted list of requirement names.
    fs::write(
        harness.join("specs/guide.md"),
        "# Purpose\nA worked layout carrying an edge slot.\n\
         \n# Satisfies\n- dev-standards\n- layout-edge-slot\n",
    )
    .unwrap();

    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![layout_kind(
                "guide",
                vec![field_region("purpose"), field_region("satisfies")],
            )],
            ..Default::default()
        },
        members: vec![layout_member("guide")],
    };
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    // The edge slot's entries derived into ordinary `satisfies` fill-edge rows in the
    // lock, keyed by the host's member name, in document order.
    let fills = guide_fills(&into);
    assert_eq!(fills, vec!["dev-standards", "layout-edge-slot"]);

    // The gate ranges over them: coverage reads the host's fills off the same family,
    // so a `required` requirement the slot names reads filled and nothing dangles.
    let requirements = BTreeMap::from([
        ("dev-standards".to_string(), requirement("dev-standards")),
        (
            "layout-edge-slot".to_string(),
            requirement("layout-edge-slot"),
        ),
    ]);
    let features = [guide_features(&fills)];
    assert!(
        coverage::check(&requirements, &features).is_empty(),
        "the derived fills cover their requirements with no dangle"
    );

    // A read verb narrates them: `why` folds the host's fills into the requirements it
    // reports, off the same corpus the gate ranges over.
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("guide", &features[..])]);
    let narration = read::why(&[], &requirements, &by_kind, &[], &[], "guide");
    assert!(
        narration.contains("dev-standards") && narration.contains("layout-edge-slot"),
        "the fills are narrated: {narration}"
    );
}

#[test]
fn a_dangling_satisfies_edge_slot_entry_refuses_through_the_existing_coverage_refusal() {
    let harness = scaffold("layout-edge-slot-dangling");
    let into = harness.join(".temper");
    // The edge slot names a requirement no roster declares — a dangling fill.
    fs::write(
        harness.join("specs/guide.md"),
        "# Satisfies\n- no-such-requirement\n",
    )
    .unwrap();

    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![layout_kind("guide", vec![field_region("satisfies")])],
            ..Default::default()
        },
        members: vec![layout_member("guide")],
    };
    // Emit derives the fill row unconditionally — a dangling target is the gate's finding,
    // not an emit-time refusal (a `satisfies` fill names a requirement, resolved by the
    // roster the gate reads, never a byte on disk).
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(guide_fills(&into), vec!["no-such-requirement"]);

    // The derived row flows into the gate's existing dangling refusal — no new one: an
    // empty roster leaves the fill resolving to nothing, so coverage fires
    // `requirement.dangling` naming the host and the unresolvable target.
    let features = [guide_features(&["no-such-requirement".to_string()])];
    let diagnostics = coverage::check(&BTreeMap::new(), &features);
    let dangling: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.rule == "requirement.dangling")
        .collect();
    assert_eq!(dangling.len(), 1, "one dangling finding: {diagnostics:?}");
    assert_eq!(dangling[0].severity, Severity::Error);
    assert_eq!(dangling[0].artifact, "guide");
    assert!(dangling[0].message.contains("no-such-requirement"));
}
