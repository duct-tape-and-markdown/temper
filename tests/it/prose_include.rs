//! A composed-prose include — engine-side resolve, splice-into-emitted-bytes, fingerprint,
//! edge.
//!
//! A member's `text` body declares an include: at emit the engine pulls the target file's
//! bytes into the host's projection at the include slot, a content dependency the lock
//! fingerprints so a moved target is drift. A dangling include refuses before a byte is
//! written. When the target is a member's own file, the include resolves to a declared
//! edge that joins the one enumeration the gate and read verbs share, path-resolved once
//! at emit — the same surface a layout import rides.

use std::collections::BTreeMap;
use std::fs;

use temper::compose::Edge;
use temper::drift::{self, Declarations, EmitOptions, IncludeRow, Payload};
use temper::extract::Features;
use temper::graph::{self, ImportDeclaration};
use temper::read;

use crate::common;
use crate::common::fresh_clause;

/// The include slot byte the SDK plants per include (`U+0001`) — the engine splices the
/// target's bytes here.
const INCLUDE_SLOT: char = '\u{1}';

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
            kinds: vec![common::bare_rule_kind_facts()],
            includes: vec![IncludeRow {
                member: "rule:host".to_string(),
                source_path: harness.join("fragment.md").to_string_lossy().into_owned(),
            }],
            ..Default::default()
        },
        members: vec![common::rule_member("host", None, &host_body)],
    };
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    // The target's bytes landed verbatim inside the host's emitted artifact, at the slot,
    // under the managed-projection banner emit heads every frontmatterless markdown
    // projection with.
    let projected = fs::read_to_string(harness.join(".claude/rules/host.md")).unwrap();
    assert_eq!(
        projected,
        format!(
            "{}\n\nIntro.\nshared prose.\nOutro.\n",
            temper::placement::BANNER
        )
    );

    // The dependency reached the lock as a fingerprinted content dependency: the host, the
    // resolved target path, a non-empty hash, and — a plain file, not a member — no edge.
    let includes = drift::includes(&into).unwrap();
    assert_eq!(includes.len(), 1);
    assert_eq!(includes[0].member, "rule:host");
    assert!(includes[0].source_path.ends_with("fragment.md"));
    assert!(includes[0].target.is_empty());
    assert!(!includes[0].import_hash.is_empty());

    // The fingerprint tracks the target's bytes: fresh now, drift once the target moves.
    let clause = fresh_clause();
    assert!(drift::include_stale(&into, &clause).unwrap().is_empty());
    fs::write(harness.join("fragment.md"), "edited prose.\n").unwrap();
    let stale = drift::include_stale(&into, &clause).unwrap();
    assert_eq!(stale.len(), 1, "a moved include target is drift: {stale:?}");
    assert_eq!(
        stale[0].rule, clause.label,
        "and it reports under the `fresh` clause's own label, never a baked rule id"
    );
}

#[test]
fn a_crlf_include_target_is_fresh_against_its_own_baseline() {
    let harness = common::scaffold("prose-include-crlf");
    let into = harness.join(".temper");
    // A CRLF include target — the shape a checkout git-filtered to CRLF hands emit.
    fs::write(
        harness.join("fragment.md"),
        "shared prose.\r\nsecond line.\r\n",
    )
    .unwrap();

    let host_body = format!("Intro.\n{INCLUDE_SLOT}Outro.\n");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![common::bare_rule_kind_facts()],
            includes: vec![IncludeRow {
                member: "rule:host".to_string(),
                source_path: harness.join("fragment.md").to_string_lossy().into_owned(),
            }],
            ..Default::default()
        },
        members: vec![common::rule_member("host", None, &host_body)],
    };
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    // The baseline speaks the comparator's EOL-blind vocabulary, so the target reads fresh
    // immediately after emit — nothing on disk moved between the write and the read.
    assert!(
        drift::include_stale(&into, &fresh_clause())
            .unwrap()
            .is_empty(),
        "a CRLF include target is fresh against the baseline emit just wrote"
    );

    // Only the fingerprint went EOL-blind: the splice still pulls the target's own bytes,
    // so every meaning-carrying word lands verbatim. Line endings are layout, not content —
    // the projection is written LF like any other, whatever the target's convention.
    let projected = fs::read_to_string(harness.join(".claude/rules/host.md")).unwrap();
    assert_eq!(
        projected,
        format!(
            "{}\n\nIntro.\nshared prose.\nsecond line.\nOutro.\n",
            temper::placement::BANNER
        )
    );

    // Still a fingerprint, not a waiver: a non-EOL edit to the target is drift.
    fs::write(harness.join("fragment.md"), "edited prose.\r\n").unwrap();
    let stale = drift::include_stale(&into, &fresh_clause()).unwrap();
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
            kinds: vec![common::bare_rule_kind_facts()],
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
            kinds: vec![common::bare_rule_kind_facts()],
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
    let host_features = [common::features("host")];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("rule", &host_features[..])]);
    let no_edges: Vec<Edge> = Vec::new();
    let narration = read::why(
        &[],
        &BTreeMap::new(),
        &BTreeMap::new(),
        &by_kind,
        &no_edges,
        &edges,
        "host",
    );
    assert!(
        narration.contains("shared") && narration.contains("import"),
        "the include edge is narrated: {narration}"
    );
}

/// **An include edge names a nested child by its whole address.** The projection-path
/// index a reference resolves through keys every member, and a nested file child's identity
/// is its `<host-address>/<kind>/<key>` address — so an include landing on one records that
/// address, the edge it lifts into names a member that exists, and two hosts' same-keyed
/// children stay two targets a reader can tell apart. A top-level target is unmoved.
mod an_include_naming_a_nested_child_spells_its_whole_address {
    use std::collections::BTreeMap;
    use std::fs;

    use temper::drift::{
        self, Declarations, EmitOptions, IncludeRow, KindFactRow, Payload, TemplateRow,
    };
    use temper::graph::{self, ImportDeclaration};

    use super::INCLUDE_SLOT;
    use crate::common;

    /// Two guides, each templating a `*.md` file child and each carrying a child keyed
    /// `checklist` — the collision the host segment of the address resolves — plus a
    /// top-level `rule` whose three include slots take both children and one plain
    /// top-level member's file.
    fn two_hosts_one_key(harness: &std::path::Path) -> Payload {
        let guide = KindFactRow {
            unit_shape: Some("directory".to_string()),
            templates: vec![TemplateRow {
                kind: "supporting-doc".to_string(),
                path: Some("*.md".to_string()),
            }],
            ..common::kind_facts("guide", ".claude/guides", "*/GUIDE.md")
        };
        let supporting_doc = KindFactRow {
            governs_root: None,
            governs_glob: None,
            unit_shape: Some("file".to_string()),
            ..common::kind_facts("supporting-doc", "", "")
        };
        let host_body = format!("Intro.\n{INCLUDE_SLOT}{INCLUDE_SLOT}{INCLUDE_SLOT}Outro.\n");
        let include = |relative: &str| IncludeRow {
            member: "rule:host".to_string(),
            source_path: harness.join(relative).to_string_lossy().into_owned(),
        };
        Payload {
            version: drift::SEAM_VERSION,
            declarations: Declarations {
                kinds: vec![common::bare_rule_kind_facts(), guide, supporting_doc],
                includes: vec![
                    include(".claude/guides/operate-the-gate/checklist.md"),
                    include(".claude/guides/adopt-the-harness/checklist.md"),
                    include(".claude/rules/shared.md"),
                ],
                ..Default::default()
            },
            members: vec![
                common::rule_member("host", None, &host_body),
                common::rule_member("shared", None, "shared prose.\n"),
                common::payload_member("guide", "operate-the-gate", None, "# Operate the gate\n"),
                common::payload_member("guide", "adopt-the-harness", None, "# Adopt the harness\n"),
                common::payload_member(
                    "supporting-doc",
                    "checklist",
                    Some("guide:operate-the-gate"),
                    "operate checklist.\n",
                ),
                common::payload_member(
                    "supporting-doc",
                    "checklist",
                    Some("guide:adopt-the-harness"),
                    "adopt checklist.\n",
                ),
            ],
        }
    }

    #[test]
    fn a_nested_childs_include_target_carries_its_host_and_a_top_levels_still_reads_kind_name() {
        let harness = common::scaffold("prose-include-nested-target");
        let into = harness.join(".temper");
        // Every include target is pre-placed on disk, as an idempotent re-emit would find
        // it: the resolve reads raw disk ahead of writing any projection.
        for (relative, body) in [
            (
                ".claude/guides/operate-the-gate/checklist.md",
                "operate checklist.\n",
            ),
            (
                ".claude/guides/adopt-the-harness/checklist.md",
                "adopt checklist.\n",
            ),
            (".claude/rules/shared.md", "shared prose.\n"),
        ] {
            let path = harness.join(relative);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, body).unwrap();
        }

        drift::emit(&two_hosts_one_key(&harness), &into, EmitOptions::default())
            .expect("two hosts each carrying a `checklist` is one address apiece, never a clash");

        let includes = drift::includes(&into).unwrap();
        let targets: Vec<&str> = includes.iter().map(|row| row.target.as_str()).collect();
        assert_eq!(
            targets,
            vec![
                "guide:operate-the-gate/supporting-doc/checklist",
                "guide:adopt-the-harness/supporting-doc/checklist",
                "rule:shared",
            ],
            "each child's target is its whole address, and the top-level target is \
             byte-identical to the one spelling it always read: {includes:#?}"
        );

        // Lifted into the resolved-edge enumeration the gate and the read verbs range over:
        // each nested target parses back to the child's own node, never a bare
        // `supporting-doc:checklist` naming a member no corpus holds.
        let edges = graph::resolved_import_edges(
            &includes
                .iter()
                .map(|row| ImportDeclaration {
                    member: row.member.clone(),
                    target: row.target.clone(),
                })
                .collect::<Vec<_>>(),
        );
        let endpoints: Vec<(&str, &str, &str)> = edges
            .iter()
            .map(|edge| (edge.field.as_str(), edge.to.0.as_str(), edge.to.1.as_str()))
            .collect();
        assert_eq!(
            endpoints,
            vec![
                (
                    "import",
                    "supporting-doc",
                    "guide:operate-the-gate/supporting-doc/checklist"
                ),
                (
                    "import",
                    "supporting-doc",
                    "guide:adopt-the-harness/supporting-doc/checklist"
                ),
                ("import", "rule", "shared"),
            ],
            "one edge per include, each nested endpoint the child's own node: {endpoints:#?}"
        );

        // The three `to` nodes really are distinct, so neither host's `checklist` shadows
        // the other's in the one graph both land in.
        let distinct: BTreeMap<_, _> = edges.iter().map(|edge| (&edge.to, ())).collect();
        assert_eq!(
            distinct.len(),
            3,
            "two same-keyed children collide on no address: {endpoints:#?}"
        );
    }
}
