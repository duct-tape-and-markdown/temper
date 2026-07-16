//! Nested-member facts, read off the lock's declared rows.
//!
//! An embedded member's facts are declaration rows the lock carries
//! (`Declarations::nested_members`), matched to their host member by its own
//! `kind:name` address — never mined by re-parsing the host's rendered TOML fence
//! (0018, "the projection is not the database"). `builtin_kind::features` is the
//! **sole choke point** every custom/built-in member's `Features` builds through, so
//! these proofs drive it directly rather than the retired `CustomKind::fold_members`.

use std::collections::BTreeMap;

use temper::builtin_kind;
use temper::drift::{CollectionEntryRow, KindFactRow, NestedMemberRow, TemplateRow};
use temper::kind::{CustomKind, Extraction, Governs, Template};

mod common;

/// A custom `decision` kind. Its own composed extraction carries no primitive at
/// all — nested-member facts never come from a kind's own extraction, so an empty
/// one is enough to prove the point.
fn decision_kind() -> CustomKind {
    CustomKind::new(
        "decision",
        Governs {
            root: "docs/decisions".to_string(),
            glob: "*.md".to_string(),
        },
        Extraction::new(Vec::new()),
    )
}

/// The lock row a `blocks()` value composes for a host member: leaves plus one
/// sibling collection's entries, authored out of alphabetical order — the same
/// shape `sdk/src/declarations.ts`'s `nestedMemberRow` writes, `host` addressed as
/// `${kind}:${name}`.
fn surface_authority_row(host: &str) -> NestedMemberRow {
    NestedMemberRow {
        host: host.to_string(),
        kind: "decision".to_string(),
        key: "surface-authority".to_string(),
        leaves: BTreeMap::from([(
            "chosen".to_string(),
            "the composition surface is canonical".to_string(),
        )]),
        collections: vec![
            CollectionEntryRow {
                collection: "rejected".to_string(),
                key: "read-only-lens".to_string(),
                leaves: BTreeMap::from([(
                    "because".to_string(),
                    "you cannot compose a harness you only mirror".to_string(),
                )]),
            },
            CollectionEntryRow {
                collection: "rejected".to_string(),
                key: "baked-projection".to_string(),
                leaves: BTreeMap::from([(
                    "because".to_string(),
                    "a stamping projector breaks law 5".to_string(),
                )]),
            },
        ],
        placed_edges: None,
    }
}

/// A raw `Unit` for the `05-surface-authority` decision member — its body is
/// ordinary prose; nothing in it is read for embedded-member facts.
fn surface_authority_unit() -> temper::kind::Unit {
    common::raw_unit(
        "05-surface-authority",
        BTreeMap::new(),
        "# Decision: the surface is the source of truth\n\nLeading prose that is only prose.\n",
        "docs/decisions/05-surface-authority.md",
    )
}

#[test]
fn a_lock_row_addressed_to_this_member_resolves_with_its_own_leaves_and_children() {
    let rows = vec![surface_authority_row("decision:05-surface-authority")];
    let features = builtin_kind::features(&decision_kind(), &surface_authority_unit(), &rows);

    assert_eq!(features.nested_members.len(), 1);
    let member = &features.nested_members[0];
    assert_eq!(member.kind, "decision");
    assert_eq!(member.key, "surface-authority");

    // Leaves are top-level authored strings, keyed by field name — the member's own
    // prose.
    assert_eq!(
        member.leaves.get("chosen").map(String::as_str),
        Some("the composition surface is canonical")
    );

    // The nested-member collection's entries are addressed by identity (`rejected` →
    // `baked-projection` → `because`), never position — each entry is itself a full
    // nested member, one layer deeper, in the row's own authored order.
    assert_eq!(
        member
            .members
            .iter()
            .map(|entry| entry.key.as_str())
            .collect::<Vec<_>>(),
        vec!["read-only-lens", "baked-projection"],
        "authored order (not alphabetical) survives the lift"
    );
    let entry = member
        .members
        .iter()
        .find(|entry| entry.collection == "rejected" && entry.key == "baked-projection")
        .expect("the collection entry is lifted");
    assert_eq!(
        entry.member.leaves.get("because").map(String::as_str),
        Some("a stamping projector breaks law 5")
    );
}

#[test]
fn leaf_addresses_are_structural_member_kind_key_child_path() {
    let rows = vec![surface_authority_row("decision:05-surface-authority")];
    let features = builtin_kind::features(&decision_kind(), &surface_authority_unit(), &rows);

    // Every leaf carries a full structural address — the member, the nested member's
    // identity, and the child path — the leaf-grain surface the read family
    // consumes.
    let leaves = features.embedded_leaves();
    let paths: Vec<&str> = leaves
        .iter()
        .map(|(address, _)| address.child_path.as_str())
        .collect();
    assert!(paths.contains(&"chosen"));
    // The nested entry's path is keyed by structure, not a positional `rejected.0.because`.
    assert!(paths.contains(&"rejected.baked-projection.because"));
    assert!(!paths.iter().any(|path| path.contains(".0.")));

    let (address, leaf) = leaves
        .iter()
        .find(|(address, _)| address.child_path == "rejected.baked-projection.because")
        .expect("the keyed nested-member leaf is addressed");
    assert_eq!(address.member, "05-surface-authority");
    assert_eq!(address.kind, "decision");
    assert_eq!(address.key, "surface-authority");
    assert_eq!(*leaf, "a stamping projector breaks law 5");
}

#[test]
fn a_leaf_carrying_a_resolved_mentions_display_text_reads_as_a_plain_string() {
    // A `Text`-authored leaf resolves its mention before it ever reaches the lock
    // (`sdk/src/declarations.ts`'s `nestedMemberRow`) — the row is indistinguishable
    // from a bare-string leaf, which is the point: the engine never sees a mention,
    // only the resolved display the SDK already rendered into it.
    let row = NestedMemberRow {
        host: "decision:05-surface-authority".to_string(),
        kind: "decision".to_string(),
        key: "surface-authority".to_string(),
        leaves: BTreeMap::from([(
            "chosen".to_string(),
            "the composition surface is canonical, per the read-only lens rejection".to_string(),
        )]),
        collections: Vec::new(),
        placed_edges: None,
    };
    let features = builtin_kind::features(&decision_kind(), &surface_authority_unit(), &[row]);

    let leaves = features.embedded_leaves();
    let (address, leaf) = leaves
        .iter()
        .find(|(address, _)| address.child_path == "chosen")
        .expect("the leaf is addressed");
    assert_eq!(address.member, "05-surface-authority");
    assert_eq!(address.kind, "decision");
    assert_eq!(address.key, "surface-authority");
    assert_eq!(
        *leaf,
        "the composition surface is canonical, per the read-only lens rejection"
    );
}

#[test]
fn a_row_addressed_to_a_different_host_never_leaks_into_this_members_features() {
    let rows = vec![surface_authority_row("decision:some-other-member")];
    let features = builtin_kind::features(&decision_kind(), &surface_authority_unit(), &rows);
    assert!(features.nested_members.is_empty());
}

#[test]
fn a_member_with_no_matching_row_carries_no_nested_members_no_error() {
    // No row at all, for any host: `Features::nested_members` is simply empty, never
    // an error — adoption is opt-in per declared value.
    let features = builtin_kind::features(&decision_kind(), &surface_authority_unit(), &[]);
    assert!(features.nested_members.is_empty());
}

#[test]
fn a_body_fence_naming_a_declared_child_kind_is_never_re_read_for_facts() {
    // The body carries a `member.decision` fence a pre-0018 fold would have parsed —
    // but with no matching lock row, nothing surfaces. The read side never looks at
    // the body at all for this fact.
    let body = "# Decision\n\n```member.decision surface-authority\nchosen = \"x\"\n```\n";
    let unit = common::raw_unit(
        "05-surface-authority",
        BTreeMap::new(),
        body,
        "docs/decisions/05-surface-authority.md",
    );
    let features = builtin_kind::features(&decision_kind(), &unit, &[]);
    assert!(features.nested_members.is_empty());
}

/// The `decision` kind's declaration row a lock would carry, its `templates` column
/// recording the same child kind `decision_kind`'s live SDK declaration composes
/// (`LOCK-NESTING-TEMPLATES`) — a declared fact, independent of how nested members
/// are actually resolved.
fn decision_kind_fact_row() -> KindFactRow {
    KindFactRow {
        templates: vec![TemplateRow {
            kind: "decision".to_string(),
            path: None,
        }],
        ..common::kind_facts("decision", "docs/decisions", "*.md")
    }
}

/// A program whose `guide` host templates a `supporting-doc` file child at `*.md`, with
/// one child composed under one host — the whole composition surface a nested file locus
/// needs: the pattern is the host kind's declared fact, and the child kind declares no
/// locus of its own to compose from.
const NESTED_FILE_PROGRAM: &str = r#"
import { emit, harness, kind, text } from "@dtmd/temper";

const supportingDoc = kind<object>({
  name: "supporting-doc",
  locus: { kind: "nested-file" },
  unitShape: "file",
  registration: [],
});

const guide = kind<object>({
  name: "guide",
  locus: { kind: "at", root: ".claude/guides", glob: "GUIDE.md" },
  unitShape: "directory",
  registration: [],
  templates: [{ kind: supportingDoc, path: "*.md" }],
});

const operating = guide({ name: "operate-the-gate", prose: text`# Operate the gate` });

process.stdout.write(
  emit(
    harness({
      members: [operating, supportingDoc({ name: "checklist", host: operating, prose: text`# Checklist` })],
    }),
  ).seam,
);
"#;

#[test]
fn a_file_childs_projection_composes_from_its_hosts_unit_and_the_templates_pattern() {
    // The engine is the sole compiler of every projection, so the composed path is proven
    // where it is actually written: `emit` reports where each member landed, and the
    // child's own kind declares no glob the path could have come from instead.
    let harness = common::tmpdir("nested-file-locus");
    let into = harness.join(".temper");
    std::fs::create_dir_all(&into).unwrap();
    std::fs::write(into.join("harness.ts"), NESTED_FILE_PROGRAM).unwrap();
    common::vendor_sdk(&into.join("node_modules").join("@dtmd"));

    let report = temper::drift::emit_program(&into, temper::drift::EmitOptions::default()).expect(
        "the nested file locus is proven through a real SDK program, never a hand-built row",
    );

    let child = report
        .entries
        .iter()
        .find(|entry| entry.kind == "supporting-doc" && entry.name == "checklist")
        .expect("a nested file child owns a file, so emit projects it");

    // The host's unit (`.claude/guides/operate-the-gate`) joined with the host template's
    // `*.md` pattern, the child's name spliced through it — never `.claude/guides/*.md`,
    // a locus the child kind does not carry.
    assert_eq!(
        child.source_path,
        harness.join(".claude/guides/operate-the-gate/checklist.md")
    );
    assert!(child.source_path.is_file());

    // The child kind governs no glob: two kinds still never share one, and the host's
    // template is the path fact's one home.
    let kinds = &temper::drift::read_declarations(&into).unwrap().kinds;
    let child_row = kinds
        .iter()
        .find(|row| row.name == "supporting-doc")
        .expect("a nested file kind takes a fact row — the engine places its file off one");
    assert_eq!(child_row.governs_root, None);
    assert_eq!(child_row.governs_glob, None);
    let host_row = kinds.iter().find(|row| row.name == "guide").unwrap();
    assert_eq!(
        host_row.templates,
        vec![TemplateRow {
            kind: "supporting-doc".to_string(),
            path: Some("*.md".to_string()),
        }]
    );
}

#[test]
fn a_declared_file_child_template_round_trips_off_the_lock_with_its_path_pattern() {
    // TEMPLATE-FILE-CHILD-FACT: a kind's nesting template is a declared kind-side fact —
    // the child kind, plus the path pattern relative to the parent's unit when the
    // children are files (`specs/model/representation.md`, "kind"). A file child is
    // never admitted over a host: admission is over an embedded body, so the lock's
    // `templates` column is the only surface this fact reaches the engine on.
    let row = KindFactRow {
        templates: vec![TemplateRow {
            kind: "supporting-doc".to_string(),
            path: Some("*.md".to_string()),
        }],
        ..common::kind_facts("skill", ".claude/skills", "SKILL.md")
    };

    let reconstructed = CustomKind::from_kind_fact_row(&row).unwrap();
    assert_eq!(
        reconstructed.templates,
        vec![Template {
            kind: "supporting-doc".to_string(),
            path: Some("*.md".to_string()),
        }]
    );

    // The same fact overlaid onto a live kind reads back identically — the one lift
    // serves both the reconstruction and the relocation path.
    let overlaid = CustomKind::new(
        "skill",
        Governs {
            root: ".claude/skills".to_string(),
            glob: "SKILL.md".to_string(),
        },
        Extraction::new(Vec::new()),
    )
    .overlay_templates(&row.templates);
    assert_eq!(overlaid.templates, reconstructed.templates);
}

#[test]
fn a_child_kinds_row_reconstructs_governing_no_glob_rather_than_a_fabricated_one() {
    // The locus under the declared fact: the child's path composes from its host's unit
    // and the host template's pattern, so its row carries no governs pair — and the lift
    // reads that absence as the spelling it is, never mining a root+glob the kind never
    // declared.
    let row = KindFactRow {
        governs_root: None,
        governs_glob: None,
        unit_shape: Some("file".to_string()),
        ..common::kind_facts("supporting-doc", "", "")
    };

    let reconstructed = CustomKind::from_kind_fact_row(&row).unwrap();
    assert_eq!(reconstructed.governs, None);
    // Governing no glob, it is discovered at none and owns no surface subdirectory to be
    // discovered under.
    assert_eq!(reconstructed.surface_subdir(), None);
    assert!(!reconstructed.owns_source(std::path::Path::new(".claude/skills/checklist.md")));
}

#[test]
fn a_lock_reconstructed_kind_resolves_the_same_embedded_members_as_its_live_declaration() {
    // Both a live SDK-composed `CustomKind` and one reconstructed off its lock row
    // share the same bare name, so both address a lock row identically —
    // `builtin_kind::features` resolves nested members off that address alone, never
    // off the kind's own extraction or declared `templates`.
    let rows = vec![surface_authority_row("decision:05-surface-authority")];

    let live = builtin_kind::features(&decision_kind(), &surface_authority_unit(), &rows);
    let reconstructed = builtin_kind::features(
        &CustomKind::from_kind_fact_row(&decision_kind_fact_row()).unwrap(),
        &surface_authority_unit(),
        &rows,
    );

    assert_eq!(reconstructed.nested_members.len(), 1);
    assert_eq!(reconstructed.nested_members, live.nested_members);
}
