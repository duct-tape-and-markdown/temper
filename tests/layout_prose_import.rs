//! A layout prose region declared as an import — engine-side resolve, fingerprint, edge.
//!
//! A prose region whose `import` names a file resolves to that file's contents against
//! raw disk (`specs/model/representation.md`, "kind"), a content dependency the lock
//! fingerprints so a moved target is drift (`pipeline.md`, "The lock"). A dangling target
//! refuses before a byte is written (`pipeline.md`, "Emit": "Refusing"). The resolved edge
//! joins the one enumeration the gate and read verbs share (`contract.md`, "edge"), so a
//! read cannot disagree with the gate about the import.

use std::collections::BTreeMap;
use std::fs;

use temper::compose::Edge;
use temper::drift::{
    self, Declarations, EmitOptions, KindFactRow, LayoutRegionRow, LayoutRow, Payload,
    PayloadMember,
};
use temper::extract::Features;
use temper::graph::{self, ImportDeclaration};
use temper::read;

mod common;

/// A layout kind governing a single lone `.md` document under `specs/`, carrying the
/// given ordered region rows. The one shape every case here builds a member of.
fn layout_kind(name: &str, regions: Vec<LayoutRegionRow>) -> KindFactRow {
    KindFactRow {
        content: Some(LayoutRow { regions }),
        ..common::kind_facts(name, "specs", &format!("{name}.md"))
    }
}

/// A `prose` region row importing `target`.
fn import_region(target: &str) -> LayoutRegionRow {
    LayoutRegionRow {
        region: "prose".to_string(),
        import: Some(target.to_string()),
        slot: None,
        member_kind: None,
        key: None,
    }
}

/// A `field` region row filling `slot`.
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

#[test]
fn an_import_region_resolves_to_its_target_and_is_fingerprinted_in_the_lock() {
    let harness = common::scaffold("layout-import-fingerprint");
    let into = harness.join(".temper");
    // A `guide` document with an import region (resolving to a sibling file) and one
    // field section for the document's single heading.
    fs::write(
        harness.join("specs/guide.md"),
        "# Intent\ntemper makes a harness good.\n",
    )
    .unwrap();
    fs::write(harness.join("specs/included.md"), "shared prose.\n").unwrap();

    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![layout_kind(
                "guide",
                vec![import_region("included.md"), field_region("intent")],
            )],
            ..Default::default()
        },
        members: vec![layout_member("guide")],
    };
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    // The import reached the lock as a fingerprinted content dependency: the host member,
    // the resolved target path, and a non-empty hash of its bytes. The target is a plain
    // file, not a member, so the edge names no member.
    let imports = drift::layout_imports(&into).unwrap();
    assert_eq!(imports.len(), 1);
    assert_eq!(imports[0].member, "guide:guide");
    assert!(imports[0].source_path.ends_with("specs/included.md"));
    assert!(imports[0].target.is_empty());
    assert!(!imports[0].import_hash.is_empty());

    // The fingerprint tracks the target's bytes: fresh now, drift once the target moves.
    assert!(drift::layout_import_stale(&into).unwrap().is_empty());
    fs::write(harness.join("specs/included.md"), "edited prose.\n").unwrap();
    let stale = drift::layout_import_stale(&into).unwrap();
    assert_eq!(stale.len(), 1, "a moved target is drift: {stale:?}");
}

#[test]
fn a_dangling_import_refuses_before_any_byte_is_written() {
    let harness = common::scaffold("layout-import-dangling");
    let into = harness.join(".temper");
    fs::write(harness.join("specs/guide.md"), "# Intent\nthe intent.\n").unwrap();

    // A `guide` layout importing a file that does not exist, beside a projected skill
    // whose artifact would be written were emit to reach its write pass.
    let skill_facts = KindFactRow {
        name: "skill".to_string(),
        provider: None,
        governs_root: ".claude/skills".to_string(),
        governs_glob: "*/SKILL.md".to_string(),
        format: Some("yaml-frontmatter".to_string()),
        unit_shape: Some("directory".to_string()),
        registration: Vec::new(),
        templates: Vec::new(),
        content: None,
    };
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![
                layout_kind(
                    "guide",
                    vec![import_region("missing.md"), field_region("intent")],
                ),
                skill_facts,
            ],
            ..Default::default()
        },
        members: vec![
            layout_member("guide"),
            PayloadMember {
                kind: "skill".to_string(),
                name: "demo".to_string(),
                fields: vec![(
                    "name".to_string(),
                    serde_json::Value::String("demo".to_string()),
                )],
                body: "# Demo\n".to_string(),
                source_path: None,
            },
        ],
    };

    let err = drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    let rendered = format!("{err:?}");
    assert!(rendered.contains("dangling"), "names the fault: {rendered}");
    assert!(
        rendered.contains("missing.md"),
        "names the import: {rendered}"
    );

    // Refused before a byte is written: the skill projection never landed on disk.
    assert!(
        !harness.join(".claude/skills/demo/SKILL.md").exists(),
        "no projection is written when a sibling import dangles"
    );
    // And nothing was fingerprinted — the refusal precedes the lock write.
    assert!(drift::layout_imports(&into).unwrap().is_empty());
}

#[test]
fn an_import_edge_joins_the_resolved_enumeration_and_narrates() {
    let harness = common::scaffold("layout-import-edge");
    let into = harness.join(".temper");
    // A `guide` importing the `intent` member's own document — a member target, so the
    // import resolves to a declared member edge, not a bare content dependency.
    fs::write(
        harness.join("specs/guide.md"),
        "# Intent\ntemper makes a harness good.\n",
    )
    .unwrap();
    fs::write(
        harness.join("specs/intent.md"),
        "# Intent\nthe product intent.\n",
    )
    .unwrap();

    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![
                layout_kind(
                    "guide",
                    vec![import_region("intent.md"), field_region("intent")],
                ),
                layout_kind("intent", vec![field_region("intent")]),
            ],
            ..Default::default()
        },
        members: vec![layout_member("guide"), layout_member("intent")],
    };
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    // The import resolved to the `intent` member: the lock names the member edge.
    let imports = drift::layout_imports(&into).unwrap();
    let guide_import = imports
        .iter()
        .find(|row| row.member == "guide:guide")
        .expect("the guide import is fingerprinted");
    assert_eq!(guide_import.target, "intent:intent");

    // Lifted into the resolved-edge enumeration, path-resolved once at emit — the same
    // set the gate's graph predicates and the read verbs range over.
    let import_edges = graph::resolved_import_edges(&[ImportDeclaration {
        member: guide_import.member.clone(),
        target: guide_import.target.clone(),
    }]);
    assert_eq!(import_edges.len(), 1);
    assert_eq!(
        import_edges[0].from,
        ("guide".to_string(), "guide".to_string())
    );
    assert_eq!(
        import_edges[0].to,
        ("intent".to_string(), "intent".to_string())
    );
    assert_eq!(import_edges[0].field, "import");

    // A read verb narrates it: `why` folds the import edge into the resolved set it walks,
    // so the guide's outgoing reference reads exactly as the gate resolves it.
    let guide_features = [feature("guide")];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("guide", &guide_features[..])]);
    let no_edges: Vec<Edge> = Vec::new();
    let narration = read::why(
        &[],
        &BTreeMap::new(),
        &by_kind,
        &no_edges,
        &import_edges,
        "guide",
    );
    assert!(
        narration.contains("intent") && narration.contains("import"),
        "the import edge is narrated: {narration}"
    );
}

/// A bare `Features` carrying only an id — the corpus entry `why` matches a member on.
fn feature(id: &str) -> Features {
    Features {
        id: id.to_string(),
        fields: BTreeMap::new(),
        body_lines: 0,
        headings: Vec::new(),
        sections: Vec::new(),
        source_dir: None,
        directives: Vec::new(),
        fenced_blocks: Vec::new(),
        nested_members: Vec::new(),
        satisfies: Vec::new(),
    }
}
