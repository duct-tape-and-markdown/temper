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
        rendered_lines: None,
        rendered_chars: None,
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
        rendered_lines: None,
        rendered_chars: None,
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

/// A `skill` that both admits its own embedded `note` kind over its composed body **and**
/// hosts a `supporting-doc` file child under the same unit — the field defect's exact
/// shape (centercode). Admission is a declaration over the host kind, but it names only a
/// child kind, never a path, so it can speak only for the embedded (pathless) grain; the
/// `supporting-doc` file layer is the host kind's own declared fact and must survive the
/// composition, or emit finds no file template and refuses the child.
const ADMIT_OVER_FILE_TEMPLATE_HOST: &str = r#"
import { blocks, emit, embeddedMemberValue, harness, kind, text } from "@dtmd/temper";
import { skill, supportingDoc } from "@dtmd/temper/claude-code";

const note = kind<object>({
  name: "note",
  locus: { kind: "embedded" },
  unitShape: "file",
  registration: [],
});

const coordinating = skill({
  name: "coordinate",
  description: "Use when driving a complex task across a team of agents.",
  prose: blocks(
    embeddedMemberValue({ kind: note, key: "first", leaves: { body: "an embedded note" } }),
  ),
});

process.stdout.write(
  emit(
    harness({
      members: [
        coordinating,
        supportingDoc({ name: "checklist", host: coordinating, prose: text`# Checklist` }),
      ],
      admit: [{ host: skill, admits: [note] }],
    }),
  ).seam,
);
"#;

#[test]
fn admitting_an_embedded_kind_over_a_host_keeps_the_hosts_file_template_layer() {
    // The composed body admits `note` (the embedded grain), and the join must leave
    // `skill`'s declared `supporting-doc` file layer standing: the child still projects,
    // and the lock's `templates` column carries both layers rather than the admission
    // wiping the path-carrying one.
    let (harness, into) =
        common::wire_sdk_harness("admit-over-file-template", ADMIT_OVER_FILE_TEMPLATE_HOST);

    let report = temper::drift::emit_program(&into, temper::drift::EmitOptions::default()).expect(
        "a host that both admits an embedded kind and templates a file layer still emits its \
         file child — the admission overrides only the embedded grain",
    );

    let child = report
        .entries
        .iter()
        .find(|entry| entry.kind == "supporting-doc" && entry.name == "checklist")
        .expect("the skill's file child still projects — its file layer survived the admission");
    assert_eq!(
        child.source_path,
        harness.join(".claude/skills/coordinate/checklist.md")
    );
    assert!(child.source_path.is_file());

    // The lock join, at the fact grain: the file layer stands and the admitted embedded
    // kind is appended — never a replacement that leaves the file layer unspellable.
    let host_row = temper::drift::read_declarations(&into)
        .unwrap()
        .kinds
        .into_iter()
        .find(|row| row.name == "skill")
        .expect("the skill host takes a fact row");
    assert_eq!(
        host_row.templates,
        vec![
            TemplateRow {
                kind: "supporting-doc".to_string(),
                path: Some("*.md".to_string()),
            },
            TemplateRow {
                kind: "note".to_string(),
                path: None,
            },
        ]
    );
}

#[test]
fn a_file_childs_projection_composes_from_its_hosts_unit_and_the_templates_pattern() {
    // The engine is the sole compiler of every projection, so the composed path is proven
    // where it is actually written: `emit` reports where each member landed, and the
    // child's own kind declares no glob the path could have come from instead.
    let (harness, into) = common::wire_sdk_harness("nested-file-locus", NESTED_FILE_PROGRAM);

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
    // Governing no glob, it owns no surface subdirectory and no source of its own: the
    // locus it *is* discovered at composes under its host's unit, never here.
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

/// The `skill` built-in, overlaid with a template declaring `child`'s file layer at
/// `pattern` — the relocation path a lock row's `templates` column reaches a live kind
/// through, so the host half of the locus is a declared fact and not a test's invention.
fn skill_templating(child: &str, pattern: &str) -> CustomKind {
    builtin_kind::definition("skill")
        .unwrap()
        .overlay_templates(&[TemplateRow {
            kind: child.to_string(),
            path: Some(pattern.to_string()),
        }])
}

/// A nested file kind's declaration: no governs pair at all, a lone file per member.
fn nested_file_kind(name: &str) -> CustomKind {
    CustomKind::from_kind_fact_row(&KindFactRow {
        governs_root: None,
        governs_glob: None,
        unit_shape: Some("file".to_string()),
        ..common::kind_facts(name, "", "")
    })
    .unwrap()
}

/// A skill at `.claude/skills/<name>/SKILL.md` — the real Claude Code layout — with a
/// companion markdown doc and a companion script beside it.
fn write_skill_with_companions(harness: &std::path::Path, name: &str) -> std::path::PathBuf {
    let unit = harness.join(".claude").join("skills").join(name);
    std::fs::create_dir_all(unit.join("scripts")).unwrap();
    std::fs::write(
        unit.join("SKILL.md"),
        format!("---\nname: {name}\ndescription: A host skill.\n---\n# {name}\n"),
    )
    .unwrap();
    std::fs::write(unit.join("PLAYBOOK.md"), "# Playbook\n").unwrap();
    std::fs::write(unit.join("scripts").join("run.sh"), "#!/bin/sh\n").unwrap();
    unit
}

#[test]
fn a_matching_file_under_a_hosts_unit_is_discovered_as_that_hosts_file_child() {
    // 0027's read half: an adopted harness's file is classified through the *host*
    // template's pattern, the host's own declared fact — the child kind governs no glob
    // for the walk to have keyed on instead.
    let harness = common::tmpdir("nested-file-discovery");
    let unit = write_skill_with_companions(&harness, "coordinate");

    let child = nested_file_kind("reference-doc");
    let kinds = BTreeMap::from([(
        "skill".to_string(),
        skill_templating("reference-doc", "*.md"),
    )]);

    let found = temper::import::discover_nested_file(
        &temper::import::Discovery::new(&harness),
        &child,
        &kinds,
        temper::import::LocalOverride::Honored,
    );

    // The companion doc surfaces as the skill's child, carrying the host unit its path
    // composed under; `scripts/run.sh` matches no `*.md` and the host's own `SKILL.md` is
    // the host member itself, never its own child.
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].file, unit.join("PLAYBOOK.md"));
    assert_eq!(found[0].host_unit, unit);

    // The classification is the template's declared child kind: a second nested file kind
    // no template names surfaces nothing, off the identical tree.
    let unnamed = nested_file_kind("appendix");
    assert!(
        temper::import::discover_nested_file(
            &temper::import::Discovery::new(&harness),
            &unnamed,
            &kinds,
            temper::import::LocalOverride::Honored
        )
        .is_empty()
    );
}

#[test]
fn a_file_the_hosts_pattern_does_not_match_is_discovered_as_no_member() {
    // Unmodeled, never mis-classified: the pattern is the whole classification rule, so a
    // file sitting under the host's unit outside it belongs to no kind at all.
    let harness = common::tmpdir("nested-file-unmatched");
    let unit = write_skill_with_companions(&harness, "coordinate");
    std::fs::write(unit.join("NOTES.txt"), "loose\n").unwrap();

    let child = nested_file_kind("reference-doc");
    let kinds = BTreeMap::from([(
        "skill".to_string(),
        // A fixed-name template: only this one file under a host's unit is a child.
        skill_templating("reference-doc", "PLAYBOOK.md"),
    )]);

    let found = temper::import::discover_nested_file(
        &temper::import::Discovery::new(&harness),
        &child,
        &kinds,
        temper::import::LocalOverride::Honored,
    );
    assert_eq!(
        found.iter().map(|unit| &unit.file).collect::<Vec<_>>(),
        vec![&unit.join("PLAYBOOK.md")]
    );
}

#[test]
fn a_declared_kinds_exact_path_carves_its_path_out_of_a_host_template() {
    // 0038's gauntlet cell: a declared locus meets a host template's glob at one path.
    // The declared exact-path kind is that path's sole home — the host template's
    // discovery carves it out, so no phantom `supporting-doc` twin materializes for the
    // coverage/`explain`/`degree` consumers to each have to un-see.
    let harness = common::tmpdir("template-discovery-carve");
    let unit = write_skill_with_companions(&harness, "jobs");
    // A second `*.md` companion under the same unit — the exact path both the declared
    // kind and the host's `supporting-doc` template would otherwise claim.
    std::fs::write(unit.join("conventions.md"), "# Conventions\n").unwrap();

    let child = nested_file_kind("supporting-doc");
    // The declared kind governs `conventions.md` at its exact path under the skill unit.
    let declared = CustomKind::new(
        "convention",
        Governs {
            root: ".claude/skills/jobs".to_string(),
            glob: "conventions.md".to_string(),
        },
        Extraction::new(Vec::new()),
    );
    let kinds = BTreeMap::from([
        (
            "skill".to_string(),
            skill_templating("supporting-doc", "*.md"),
        ),
        ("convention".to_string(), declared),
    ]);

    let found = temper::import::discover_nested_file(
        &temper::import::Discovery::new(&harness),
        &child,
        &kinds,
        temper::import::LocalOverride::Honored,
    );

    // `PLAYBOOK.md` is the skill's only `supporting-doc` child: the template glob would
    // have swept up `conventions.md` too, but the declared kind's locus carves it out, so
    // the declared member is that path's sole home and the twin never forms.
    assert_eq!(
        found.iter().map(|unit| &unit.file).collect::<Vec<_>>(),
        vec![&unit.join("PLAYBOOK.md")]
    );
    assert!(
        !found
            .iter()
            .any(|found| found.file == unit.join("conventions.md")),
        "the declared kind's path is no host template's child — no phantom twin"
    );
}

/// A starred-segment `conventions` kind: a lone file per matching directory at
/// `*/conventions.md`, keyed by the directory segment its glob stars rather than the shared
/// `conventions` stem. It coexists inside a skill's directory — the skill owns the
/// directory, this file only borrows the segment for identity.
fn conventions_kind() -> CustomKind {
    CustomKind {
        unit_shape: Some(temper::kind::UnitShape::StarredSegment),
        ..CustomKind::new(
            "conventions",
            Governs {
                root: ".claude/skills".to_string(),
                glob: "*/conventions.md".to_string(),
            },
            Extraction::new(Vec::new()),
        )
    }
}

#[test]
fn a_starred_segment_kind_keys_one_member_per_directory_by_its_segment() {
    // The lone-file starred-segment locus: two `conventions.md` files, one per skill
    // directory, each keyed by its own directory segment rather than collapsing onto the
    // shared `conventions` stem. The skill owns each directory; the `conventions` file
    // coexists inside it, borrowing the segment for identity alone.
    let harness = common::tmpdir("starred-segment-discovery");
    let alpha = write_skill_with_companions(&harness, "alpha");
    let beta = write_skill_with_companions(&harness, "beta");
    std::fs::write(alpha.join("conventions.md"), "# Alpha conventions\n").unwrap();
    std::fs::write(beta.join("conventions.md"), "# Beta conventions\n").unwrap();

    let kind = conventions_kind();
    let governs = kind.governs.clone().unwrap();
    let files = temper::import::discover_kind_files(
        &temper::import::Discovery::new(&harness),
        &kind,
        &governs,
        temper::import::LocalOverride::Honored,
    );

    // One member per matching directory — the skills' own `SKILL.md` and companions match
    // `*/conventions.md` nowhere and stay this kind's non-members.
    assert_eq!(
        files,
        vec![alpha.join("conventions.md"), beta.join("conventions.md")]
    );

    let base = harness.join(&governs.root);
    let ids: Vec<String> = files
        .iter()
        .map(|file| {
            temper::frontmatter::Member::from_source_rooted(&kind, file, &base)
                .unwrap()
                .id
        })
        .collect();

    // Identity is the starred directory segment, never the stem — two same-stemmed files
    // carry distinct ids rather than both keying `conventions`.
    assert_eq!(ids, vec!["alpha".to_string(), "beta".to_string()]);
}

#[test]
fn a_shipped_skills_bundled_reference_document_is_discovered_as_its_supporting_doc_child() {
    // The built-in adoption, off the shipped kinds alone: no test-built host, no
    // overlaid template. `skill` templates `supporting-doc` at its directory's markdown,
    // so a real skill's companion doc is that skill's child by the shipped facts —
    // nesting-is-model-containment on the built-ins, not on a fixture's invention.
    let harness = common::tmpdir("builtin-supporting-doc");
    let unit = write_skill_with_companions(&harness, "coordinate");

    let kinds = builtin_kind::definitions();
    let child = kinds
        .get("supporting-doc")
        .expect("supporting-doc ships as a built-in kind")
        .clone();

    let found = temper::import::discover_nested_file(
        &temper::import::Discovery::new(&harness),
        &child,
        &kinds,
        temper::import::LocalOverride::Honored,
    );

    // `PLAYBOOK.md` is the skill's child, carrying the host unit its path composed under.
    // The host's own `SKILL.md` is the host member, never its own child, and
    // `scripts/run.sh` is a supporting file of a type the prose-only kind cannot hold —
    // it matches the `*.md` pattern nowhere and stays unmodeled rather than mis-typed.
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].file, unit.join("PLAYBOOK.md"));
    assert_eq!(found[0].host_unit, unit);

    // The child kind carries neither half of its own locus: the pattern is `skill`'s
    // declared template and the unit is `skill`'s own governs scan.
    assert_eq!(child.governs, None);
    assert_eq!(
        kinds.get("skill").unwrap().templates,
        vec![Template {
            kind: "supporting-doc".to_string(),
            path: Some("*.md".to_string()),
        }]
    );
}

/// **Host-qualified resolution** — an embedded member's identity is its address
/// (`<host-address>/<kind>/<key>`), so an edge resolves to the member under the host it
/// names, and nothing about the host rides the member's typed fields.
///
/// Every member here is built by the real writers: the host's by `builtin_kind::features`
/// off its lock rows, the embedded members' by `compose::embedded_features_by_kind`, and
/// the edges by `drift::edges_from_declarations` off the same lock.
mod host_qualified_addresses {
    use std::collections::BTreeMap;

    use temper::drift::{AssemblyFactRow, Declarations, KindFactRow, NestedMemberRow, TemplateRow};
    use temper::extract::Features;
    use temper::kind::{CustomKind, Extraction, Governs};
    use temper::{admissibility, builtin_kind, compose, drift, graph, read};

    use crate::common;

    /// The `service` host kind: it templates an embedded `domain` child and carries a
    /// `serves` reference field.
    fn service_kind() -> CustomKind {
        CustomKind::new(
            "service",
            Governs {
                root: "specs".to_string(),
                glob: "*.md".to_string(),
            },
            Extraction::new(Vec::new()),
        )
    }

    /// One `domain` member keyed `key`, nested under `host`.
    fn domain_row(host: &str, key: &str) -> NestedMemberRow {
        NestedMemberRow {
            host: host.to_string(),
            kind: "domain".to_string(),
            key: key.to_string(),
            leaves: BTreeMap::from([("purpose".to_string(), "a domain".to_string())]),
            collections: Vec::new(),
            placed_edges: None,
            rendered_lines: None,
            rendered_chars: None,
        }
    }

    /// The lock the corpus commits: `service` templating the embedded `domain` kind, the
    /// `service.serves → domain` edge, and one `common`-keyed domain under each of two
    /// hosts — the same key twice, which is the whole point of addressing by host.
    fn declarations() -> Declarations {
        Declarations {
            kinds: vec![KindFactRow {
                templates: vec![TemplateRow {
                    kind: "domain".to_string(),
                    path: None,
                }],
                ..common::kind_facts("service", "specs", "*.md")
            }],
            assembly: vec![AssemblyFactRow {
                fact: "edge".to_string(),
                value: None,
                from: Some("service".to_string()),
                field: Some("serves".to_string()),
                to: Some(vec!["domain".to_string()]),
            }],
            nested_members: vec![
                domain_row("service:alpha", "common"),
                domain_row("service:beta", "common"),
                domain_row("service:gamma", "billing"),
            ],
            ..Declarations::default()
        }
    }

    /// The `alpha` host member, its `serves` field naming `target`, built through the sole
    /// choke point every member's `Features` is built through.
    fn alpha(rows: &[NestedMemberRow], target: &str) -> Features {
        builtin_kind::features(
            &service_kind(),
            &common::raw_unit(
                "alpha",
                BTreeMap::from([(
                    "serves".to_string(),
                    serde_json::Value::String(target.to_string()),
                )]),
                "# Alpha\n",
                "specs/alpha.md",
            ),
            rows,
        )
    }

    /// The corpus `alpha`'s `serves` edge resolves against, and the resolution over it.
    fn resolve(target: &str) -> (Vec<graph::ResolvedEdge>, Vec<String>) {
        let declarations = declarations();
        let embedded = compose::embedded_features_by_kind(&declarations);
        // Non-vacuity: the same key really is carried by two members of the same kind, so
        // a resolution that ignored the host would have something to get wrong.
        let domains = embedded.get("domain").expect("`domain` is a declared kind");
        assert_eq!(
            domains
                .iter()
                .filter(|features| features.id.ends_with("/domain/common"))
                .count(),
            2,
            "two same-keyed members under different hosts"
        );

        let hosts = BTreeMap::from([(
            "service".to_string(),
            vec![alpha(&declarations.nested_members, target)],
        )]);
        let by_kind = compose::assemble_by_kind(&hosts, &[], &embedded);
        let edges = drift::edges_from_declarations(&declarations).expect("the edge row is whole");
        let result = graph::resolved_edges(&edges, &by_kind);
        let dangling = result
            .dangling_diagnostics
            .iter()
            .map(|diagnostic| diagnostic.message.clone())
            .collect();
        (result.resolved, dangling)
    }

    /// `explain <target>` over a by-kind corpus alone — every strand this module's reads
    /// exercise resolves off it, so the remaining inputs (custom members, roster,
    /// contracts, registrations, tap log) are the empty ones `main.rs` would thread.
    fn explain(by_kind: &BTreeMap<&str, &[Features]>, target: &str) -> String {
        read::explain(
            &[],
            &BTreeMap::new(),
            &BTreeMap::new(),
            &[],
            by_kind,
            &[],
            &[],
            &BTreeMap::new(),
            &[],
            &[],
            &[],
            &[],
            0,
            target,
            &BTreeMap::new(),
        )
    }

    #[test]
    fn a_host_qualified_target_resolves_to_the_member_under_that_host() {
        let (resolved, dangling) = resolve("service:beta/domain/common");
        assert!(dangling.is_empty(), "the address resolves: {dangling:?}");
        assert_eq!(
            resolved
                .iter()
                .map(|edge| edge.to.clone())
                .collect::<Vec<_>>(),
            vec![(
                "domain".to_string(),
                "service:beta/domain/common".to_string()
            )],
            "the arc lands on `beta`'s member, not the same-keyed one under `alpha`"
        );
    }

    #[test]
    fn a_host_qualified_target_naming_a_host_without_that_member_dangles() {
        // `common` exists — twice — but never under `gamma`. The key alone is not the
        // member: an address that names a host carrying no such member resolves to
        // nothing rather than being cross-attributed to a same-keyed member elsewhere.
        let (resolved, dangling) = resolve("service:gamma/domain/common");
        assert!(resolved.is_empty());
        assert_eq!(dangling.len(), 1);
        assert!(
            dangling[0].contains("service:gamma/domain/common"),
            "the finding names the address its author wrote, got: {}",
            dangling[0]
        );
    }

    #[test]
    fn a_bare_key_one_host_carries_still_names_that_embedded_member() {
        // The short form the corpus already writes: within a one-kind target set a bare
        // key names a member of that kind — and does so exactly when one host carries it.
        // `billing` is `gamma`'s alone, so the short form resolves and the arc carries the
        // identity its author spelled.
        let (resolved, dangling) = resolve("billing");
        assert!(dangling.is_empty(), "the bare key resolves: {dangling:?}");
        assert_eq!(
            resolved
                .iter()
                .map(|edge| edge.to.clone())
                .collect::<Vec<_>>(),
            vec![("domain".to_string(), "billing".to_string())],
            "the arc carries the identity its author spelled"
        );
    }

    #[test]
    fn a_bare_key_two_hosts_carry_refuses_naming_both() {
        // The inversion of the prior policy (a bare key took whichever member of the kind
        // carried it first, "ambiguous but deterministic"). `common` is `alpha`'s *and*
        // `beta`'s: resolution is total, so an address naming two members names none, and
        // the refusal names every carrier plus the spelling that tells them apart. The
        // declaration stays legal — only this reference refuses.
        let (resolved, dangling) = resolve("common");
        assert!(
            resolved.is_empty(),
            "an ambiguous bare key resolves to nothing, got {} arcs",
            resolved.len()
        );
        assert_eq!(dangling.len(), 1);
        for carrier in ["`service:alpha`", "`service:beta`"] {
            assert!(
                dangling[0].contains(carrier),
                "the refusal names every carrier host ({carrier}), got: {}",
                dangling[0]
            );
        }
        assert!(
            dangling[0].contains("<host-address>/<kind>/<key>"),
            "the refusal points at the spelling that resolves, got: {}",
            dangling[0]
        );
        // Both host-qualified spellings still resolve — the long form is never ambiguous.
        for host in ["alpha", "beta"] {
            let (resolved, dangling) = resolve(&format!("service:{host}/domain/common"));
            assert!(
                dangling.is_empty(),
                "`{host}`'s address resolves: {dangling:?}"
            );
            assert_eq!(resolved.len(), 1);
        }
    }

    #[test]
    fn one_host_declaring_a_key_twice_is_a_malformed_lock() {
        // Coincidence is a *within-host* judgment: the same `(kind, key)` twice under one
        // host spells one address for two members, refused at admissibility. The base
        // corpus's cross-host pair is the non-vacuity twin — two rows sharing a
        // `(kind, key)` that are *not* coincident, and are admitted.
        assert!(
            admissibility::nested_member_coincidence(&declarations()).is_empty(),
            "two hosts sharing a `(kind, key)` are distinct addresses, not a coincidence"
        );

        let mut duplicated = declarations();
        duplicated
            .nested_members
            .push(domain_row("service:alpha", "common"));
        let findings = admissibility::nested_member_coincidence(&duplicated);
        assert_eq!(
            findings.len(),
            1,
            "one refusal for the one repeated address"
        );
        assert!(
            findings[0].message.contains("service:alpha/domain/common"),
            "the refusal names the coincident address, got: {}",
            findings[0].message
        );
    }

    #[test]
    fn the_citation_scoping_index_maps_an_ambiguous_key_to_no_host() {
        // The source side of the same judgment: `mention_reachable` scopes a body-carried
        // citation through this `(kind, key) → host` index. A key two hosts carry maps to
        // neither, rather than to whichever row was indexed last; a key one host carries
        // still maps to it.
        let declarations = declarations();
        let embedded = compose::embedded_features_by_kind(&declarations);
        let by_kind: BTreeMap<&str, &[Features]> = embedded
            .iter()
            .map(|(kind, members)| (kind.as_str(), members.as_slice()))
            .collect();
        let hosts = graph::embedded_hosts_by_key(&by_kind);

        assert_eq!(
            hosts.get(&("domain".to_string(), "billing".to_string())),
            Some(&("service".to_string(), "gamma".to_string())),
            "the unambiguous key scopes to its one host — the index is not empty"
        );
        assert_eq!(
            hosts.get(&("domain".to_string(), "common".to_string())),
            None,
            "the key `alpha` and `beta` both carry scopes to neither"
        );
    }

    #[test]
    fn explain_refuses_a_bare_key_two_hosts_carry() {
        // `explain` reads the same spelling the same way: a bare key one host carries
        // names that member, a key two hosts carry names nothing and comes back with every
        // full address to retry with — `explain` never guesses.
        let declarations = declarations();
        let embedded = compose::embedded_features_by_kind(&declarations);
        let hosts = BTreeMap::from([(
            "service".to_string(),
            vec![alpha(&declarations.nested_members, "billing")],
        )]);
        let by_kind = compose::assemble_by_kind(&hosts, &[], &embedded);

        let ambiguous = explain(&by_kind, "common");
        assert!(
            ambiguous.contains("names more than one thing"),
            "the ambiguous key is refused, got: {ambiguous}"
        );
        for carrier in ["service:alpha/domain/common", "service:beta/domain/common"] {
            assert!(
                ambiguous.contains(carrier),
                "the refusal lists `{carrier}` as a spelling to retry with, got: {ambiguous}"
            );
        }

        // The spellings it hands back are real: each full address resolves to the one
        // member under that host.
        for carrier in ["service:alpha/domain/common", "service:beta/domain/common"] {
            let qualified = explain(&by_kind, carrier);
            assert!(
                !qualified.contains("names more than one thing")
                    && !qualified.contains("No member"),
                "`{carrier}` — the spelling the refusal offered — resolves, got: {qualified}"
            );
        }

        // Non-vacuity: the same read over the key only `gamma` carries resolves to that
        // member rather than refusing.
        let resolved = explain(&by_kind, "billing");
        assert!(
            !resolved.contains("names more than one thing"),
            "a key one host carries is no ambiguity, got: {resolved}"
        );
        assert!(
            resolved.contains("service:gamma/domain/billing"),
            "the bare key resolves to the member under its one host, got: {resolved}"
        );
    }

    #[test]
    fn an_address_with_a_leaf_tail_names_no_member() {
        // `/<leaf>` beneath a nested address addresses a *leaf* — one authored string, a
        // grain of its own. A four-segment address is therefore no member address: it
        // dangles under the name its author wrote rather than truncating to the member
        // that contains the leaf, which would answer a leaf reference with a member.
        let (resolved, dangling) = resolve("service:beta/domain/common/purpose");
        assert!(resolved.is_empty());
        assert_eq!(dangling.len(), 1);
        assert!(
            dangling[0].contains("service:beta/domain/common/purpose"),
            "the finding names the whole authored address, got: {}",
            dangling[0]
        );
    }

    #[test]
    fn an_embedded_members_fields_are_its_leaves_and_nothing_else() {
        // The host is the address's business, never a field: a member reaching clause
        // judgment carries only what its author wrote.
        let embedded = compose::embedded_features_by_kind(&declarations());
        for features in embedded.get("domain").expect("`domain` is declared") {
            assert_eq!(
                features.fields.keys().collect::<Vec<_>>(),
                vec!["purpose"],
                "`{}` carries only its authored leaves",
                features.id
            );
        }
    }
}
