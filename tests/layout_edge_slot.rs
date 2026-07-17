//! A layout field section the kind marks as an edge field — an edge slot.
//!
//! A field section whose slot is one of the kind's edge fields carries addresses, not a
//! verbatim span. Two species resolve on distinct paths, each keyed by the host's member
//! id: `satisfies` fills derive into the lock's own `satisfies` family at emit and reach
//! the roster/coverage/read tiers exactly as a file member's SDK-emitted fills do; a
//! declared relationship's entries resolve live off the host's features, the same
//! reference graph a file member's frontmatter list feeds. Either way a dangling entry is
//! the gate's existing finding — the `requirement.dangling` coverage check for a fill, the
//! `graph.route` resolution check for a relationship — never a silent drop.

use std::collections::BTreeMap;
use std::fs;

use temper::check::Severity;
use temper::compose::Requirement;
use temper::coverage;
use temper::drift::{
    self, AssemblyFactRow, Declarations, EmitOptions, KindFactRow, LayoutRegionRow, LayoutRow,
    Payload, PayloadMember,
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
        host: None,
        fields: Vec::new(),
        body: String::new(),
        source_path: None,
    }
}

/// An `edge` assembly fact — the lock row a `[[kind.<from>.relationships]]` table
/// projects. A custom kind carries its declared edges only here, never on its kind-fact
/// row, so this is the one place the gate and emit learn a layout slot is a relationship.
fn edge(from: &str, field: &str, to: &str) -> AssemblyFactRow {
    AssemblyFactRow {
        fact: "edge".to_string(),
        value: None,
        from: Some(from.to_string()),
        field: Some(field.to_string()),
        to: Some(vec![to.to_string()]),
    }
}

/// Run `temper explain <target>` from `root`, capturing stdout+stderr — the read verb
/// that narrates a member's resolved edges in and out.
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

/// The `guide` host's `satisfies` fill claims as the lock carries them, in derived order
/// — the exact rows emit wrote, read straight back off the committed lock.
fn guide_fills(into: &std::path::Path) -> Vec<String> {
    drift::read_declarations(into)
        .unwrap()
        .satisfies
        .into_iter()
        .filter(|row| row.member == "guide:guide")
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
        rendered_lines: Some(0),
        rendered_chars: Some(0),
        headings: Vec::new(),
        sections: Vec::new(),
        source_dir: None,
        directives: Vec::new(),
        fenced_blocks: Vec::new(),
        nested_members: Vec::new(),
        satisfies: fills.to_vec(),
        edge_placements: None,
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
    let harness = common::scaffold("layout-edge-slot-satisfies");
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
    // lock, keyed by the host's `kind:name` address, in document order.
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
    let narration = read::why(
        &[],
        &requirements,
        &BTreeMap::new(),
        &by_kind,
        &[],
        &[],
        "guide",
    );
    assert!(
        narration.contains("dev-standards") && narration.contains("layout-edge-slot"),
        "the fills are narrated: {narration}"
    );
}

#[test]
fn a_dangling_satisfies_edge_slot_entry_refuses_through_the_existing_coverage_refusal() {
    let harness = common::scaffold("layout-edge-slot-dangling");
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

#[test]
fn a_declared_relationship_edge_slots_entries_reach_the_gate_and_read_verbs() {
    let harness = common::scaffold("layout-edge-slot-relationship");
    // A `guide` layout kind declaring a `routes_to` edge into `skill`, and a document
    // whose relationship section names a real skill — the addresses resolve live off the
    // host's features, exactly as a file member's frontmatter reference list does.
    common::write_skill(&harness, "standards", &common::clean_skill("standards"));
    fs::write(
        harness.join("specs/guide.md"),
        "# Purpose\nA worked layout carrying a relationship edge slot.\n\
         \n# Routes to\n- standards\n",
    )
    .unwrap();
    common::write_lock(
        &harness,
        Declarations {
            kinds: vec![layout_kind(
                "guide",
                vec![field_region("purpose"), field_region("routes_to")],
            )],
            assembly: vec![edge("guide", "routes_to", "skill")],
            ..Default::default()
        },
    );

    // The gate ranges over the resolved edge: the entry names a real skill, so the graph
    // route resolves and the run is clean.
    let run = common::check_in(&harness, &[], None);
    assert!(
        run.ok,
        "a relationship edge resolving to a real skill is clean, got:\n{}",
        run.output
    );

    // A read verb narrates the same resolved edge the gate ranges over — the entry parsed
    // as an address, never a verbatim span (a span would resolve to no node and narrate
    // nothing).
    let narration = explain_in(&harness, "guide");
    assert!(
        narration.contains("points at `standards`") && narration.contains("routes_to"),
        "`explain` narrates the host's resolved out-edge: {narration}"
    );
}

#[test]
fn a_dangling_relationship_edge_entry_is_a_route_finding_never_a_silent_drop() {
    let harness = common::scaffold("layout-edge-slot-relationship-dangling");
    // The relationship section names a skill no member provides — a dangling address.
    common::write_skill(&harness, "standards", &common::clean_skill("standards"));
    fs::write(
        harness.join("specs/guide.md"),
        "# Purpose\nx\n\n# Routes to\n- absent\n",
    )
    .unwrap();
    common::write_lock(
        &harness,
        Declarations {
            kinds: vec![layout_kind(
                "guide",
                vec![field_region("purpose"), field_region("routes_to")],
            )],
            assembly: vec![edge("guide", "routes_to", "skill")],
            ..Default::default()
        },
    );

    // The dangling address is the gate's route-resolution finding — never silently
    // dropped: the run fails and the finding names the host, the target, and the field.
    let run = common::check_in(&harness, &[], None);
    assert!(
        !run.ok,
        "a dangling relationship entry fails the run, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("guide")
            && run.output.contains("absent")
            && run.output.contains("routes_to"),
        "the finding names the host, the dangling target, and the reference field: {}",
        run.output
    );
}
