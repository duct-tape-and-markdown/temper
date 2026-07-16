//! A composed-prose include — engine-side resolve, splice-into-emitted-bytes, fingerprint,
//! edge.
//!
//! A member's `text` body declares an include: at emit the engine pulls the target file's
//! bytes into the host's projection at the include slot (`pipeline.md`, "The SDK"), a
//! content dependency the lock fingerprints so a moved target is drift (`pipeline.md`, "The
//! lock"). A dangling include refuses before a byte is written (`pipeline.md`, "Emit":
//! "Refusing"). When the target is a member's own file, the include resolves to a declared
//! edge that joins the one enumeration the gate and read verbs share (`contract.md`,
//! "edge"), path-resolved once at emit — the same surface a layout import rides.

use std::collections::BTreeMap;
use std::fs;

use temper::compose::Edge;
use temper::drift::{self, Declarations, EmitOptions, IncludeRow, KindFactRow, Payload};
use temper::extract::Features;
use temper::graph::{self, ImportDeclaration};
use temper::read;

mod common;

/// The include slot byte the SDK plants per include (`U+0001`) — the engine splices the
/// target's bytes here.
const INCLUDE_SLOT: char = '\u{1}';

/// A `rule` kind governing `.claude/rules/*.md` — a plain markdown, field-less projection,
/// so the emitted artifact is the resolved body verbatim.
fn rule_kind() -> KindFactRow {
    common::kind_facts("rule", ".claude/rules", "*.md")
}

#[test]
fn an_include_lands_byte_identical_and_is_fingerprinted() {
    let harness = common::scaffold("prose-include-fingerprint");
    let into = harness.join(".temper");
    // The include target — a plain repository fragment, not a member.
    fs::write(harness.join("fragment.md"), "shared prose.\n").unwrap();

    // A host rule whose body carries one include slot between two literal chunks.
    let host_body = format!("Intro.\n{INCLUDE_SLOT}Outro.\n");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![rule_kind()],
            includes: vec![IncludeRow {
                member: "rule:host".to_string(),
                source_path: harness.join("fragment.md").to_string_lossy().into_owned(),
            }],
            ..Default::default()
        },
        members: vec![common::rule_member("host", None, &host_body)],
    };
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    // The target's bytes landed verbatim inside the host's emitted artifact, at the slot.
    let projected = fs::read_to_string(harness.join(".claude/rules/host.md")).unwrap();
    assert_eq!(projected, "Intro.\nshared prose.\nOutro.\n");

    // The dependency reached the lock as a fingerprinted content dependency: the host, the
    // resolved target path, a non-empty hash, and — a plain file, not a member — no edge.
    let includes = drift::includes(&into).unwrap();
    assert_eq!(includes.len(), 1);
    assert_eq!(includes[0].member, "rule:host");
    assert!(includes[0].source_path.ends_with("fragment.md"));
    assert!(includes[0].target.is_empty());
    assert!(!includes[0].import_hash.is_empty());

    // The fingerprint tracks the target's bytes: fresh now, drift once the target moves.
    assert!(drift::include_stale(&into).unwrap().is_empty());
    fs::write(harness.join("fragment.md"), "edited prose.\n").unwrap();
    let stale = drift::include_stale(&into).unwrap();
    assert_eq!(stale.len(), 1, "a moved include target is drift: {stale:?}");
}

#[test]
fn a_dangling_include_refuses_before_any_byte_is_written() {
    let harness = common::scaffold("prose-include-dangling");
    let into = harness.join(".temper");

    // A host rule including a file that does not exist, beside a sibling rule whose
    // projection would land were emit to reach its write pass.
    let host_body = format!("Intro.\n{INCLUDE_SLOT}Outro.\n");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![rule_kind()],
            includes: vec![IncludeRow {
                member: "rule:host".to_string(),
                source_path: harness.join("missing.md").to_string_lossy().into_owned(),
            }],
            ..Default::default()
        },
        members: vec![
            common::rule_member("host", None, &host_body),
            common::rule_member("sibling", None, "# Sibling\n"),
        ],
    };

    let err = drift::emit(&payload, &into, EmitOptions::default()).unwrap_err();
    let rendered = format!("{err:?}");
    assert!(rendered.contains("dangling"), "names the fault: {rendered}");
    assert!(
        rendered.contains("missing.md"),
        "names the include: {rendered}"
    );

    // Refused before a byte is written: neither projection landed, and nothing was
    // fingerprinted (the refusal precedes the lock write).
    assert!(!harness.join(".claude/rules/host.md").exists());
    assert!(
        !harness.join(".claude/rules/sibling.md").exists(),
        "no projection is written when a sibling include dangles"
    );
    assert!(drift::includes(&into).unwrap().is_empty());
}

#[test]
fn an_include_edge_joins_the_resolved_enumeration_and_narrates() {
    let harness = common::scaffold("prose-include-edge");
    let into = harness.join(".temper");
    // The include target IS another member's own file — pre-placed on disk (an idempotent
    // re-emit sees its projection already there), so the include resolves to a member edge.
    fs::create_dir_all(harness.join(".claude/rules")).unwrap();
    fs::write(harness.join(".claude/rules/shared.md"), "shared prose.\n").unwrap();

    let host_body = format!("Intro.\n{INCLUDE_SLOT}Outro.\n");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![rule_kind()],
            includes: vec![IncludeRow {
                member: "rule:host".to_string(),
                source_path: harness
                    .join(".claude/rules/shared.md")
                    .to_string_lossy()
                    .into_owned(),
            }],
            ..Default::default()
        },
        members: vec![
            common::rule_member("host", None, &host_body),
            common::rule_member("shared", None, "shared prose.\n"),
        ],
    };
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    // The include resolved to the `shared` member: the lock names the member edge.
    let includes = drift::includes(&into).unwrap();
    let host_include = includes
        .iter()
        .find(|row| row.member == "rule:host")
        .expect("the host include is fingerprinted");
    assert_eq!(host_include.target, "rule:shared");

    // Lifted into the resolved-edge enumeration under the `import` locus, path-resolved
    // once at emit — the same set the gate's graph predicates and the read verbs range over.
    let edges = graph::resolved_import_edges(&[ImportDeclaration {
        member: host_include.member.clone(),
        target: host_include.target.clone(),
    }]);
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0].from, ("rule".to_string(), "host".to_string()));
    assert_eq!(edges[0].to, ("rule".to_string(), "shared".to_string()));
    assert_eq!(edges[0].field, "import");

    // A read verb narrates it: `why` folds the include edge into the resolved set it walks.
    let host_features = [feature("host")];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("rule", &host_features[..])]);
    let no_edges: Vec<Edge> = Vec::new();
    let narration = read::why(&[], &BTreeMap::new(), &by_kind, &no_edges, &edges, "host");
    assert!(
        narration.contains("shared") && narration.contains("import"),
        "the include edge is narrated: {narration}"
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
        edge_placements: BTreeMap::new(),
    }
}
