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

use crate::common;

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
  locus: { kind: "at", root: ".claude/guides", glob: "*/GUIDE.md" },
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

/// The same composition with a **literal leading segment** in the host template's path
/// pattern (`notes/*.md`): the child still composes under the host's own unit, with
/// `notes/` carried verbatim and the name taking the pattern's final segment.
const NESTED_FILE_SUBDIRECTORY_PROGRAM: &str = r#"
import { emit, harness, kind, text } from "@dtmd/temper";

const supportingDoc = kind<object>({
  name: "supporting-doc",
  locus: { kind: "nested-file" },
  unitShape: "file",
  registration: [],
});

const guide = kind<object>({
  name: "guide",
  locus: { kind: "at", root: ".claude/guides", glob: "*/GUIDE.md" },
  unitShape: "directory",
  registration: [],
  templates: [{ kind: supportingDoc, path: "notes/*.md" }],
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
fn a_template_pattern_carrying_a_literal_directory_composes_under_the_hosts_unit() {
    // A host template's pattern places through the same one splice rule a flat glob does,
    // so a literal leading segment is fixed placement rather than depth the splice
    // refuses: the child lands inside the host's unit, one directory deeper.
    let (harness, into) =
        common::wire_sdk_harness("nested-file-subdirectory", NESTED_FILE_SUBDIRECTORY_PROGRAM);

    let report = temper::drift::emit_program(&into, temper::drift::EmitOptions::default()).expect(
        "a template pattern with a literal directory segment composes rather than refusing",
    );

    let child = report
        .entries
        .iter()
        .find(|entry| entry.kind == "supporting-doc" && entry.name == "checklist")
        .expect("the file child projects under the host's unit");
    assert_eq!(
        child.source_path,
        harness.join(".claude/guides/operate-the-gate/notes/checklist.md")
    );
    assert!(child.source_path.is_file());
}

/// **Literal template segments are locus, never identity.** A host template's pattern
/// spells the same literal directories for every member it places, so a child reads back
/// under the key its author declared — the fold strips them, exactly as the splice
/// re-inserts them, and the two stay inverses. What a wildcard spans is the other half:
/// there depth genuinely distinguishes same-named files, so it keeps folding.
mod literal_template_segments_are_locus {
    use std::collections::BTreeMap;
    use std::fs;

    use temper::drift::{self, EmitOptions};

    use super::{NESTED_FILE_SUBDIRECTORY_PROGRAM, nested_file_kind, skill_templating};
    use crate::common;

    #[test]
    fn a_child_under_a_literal_directory_pattern_reads_back_under_its_authored_key() {
        // The emit half above lands `notes/checklist.md`; this is the read half of the same
        // fixture. The author declared `checklist`, and `notes/` is placement the pattern
        // spells for every child it places — so the key the check side folds is
        // `checklist`, and the address is the host's with that key under it. Folding the
        // literal segment in would key `notes-checklist`: a name no author wrote, and the
        // one the splice would then place back at `notes/notes-checklist.md`.
        let (harness, into) = common::wire_sdk_harness(
            "nested-file-subdirectory-read",
            NESTED_FILE_SUBDIRECTORY_PROGRAM,
        );
        drift::emit_program(&into, EmitOptions::default())
            .expect("the literal-directory template emits its child");

        // Non-vacuity first: the child is a member the gate judged, so the narration below
        // is read against a live corpus rather than agreeing with an absence.
        let (findings, ok) = common::check_harness(&harness);
        let checked = common::findings_for(&findings, "coverage.checked");
        assert!(
            checked
                .iter()
                .any(|line| line.contains("supporting-doc (1)")),
            "the emitted child is a checked member: {findings:#?}"
        );
        assert!(ok, "the emitted corpus checks clean: {findings:#?}");

        let out = common::explain_in(&harness, "guide:operate-the-gate/supporting-doc/checklist");
        assert!(
            out.contains("Member `guide:operate-the-gate/supporting-doc/checklist`"),
            "the child resolves at the key its author declared: {out}"
        );

        // And the placement-folded spelling names nothing: the fold strips what the pattern
        // spells, so there is no second identity for the same file.
        let folded = common::explain_in(
            &harness,
            "guide:operate-the-gate/supporting-doc/notes-checklist",
        );
        assert!(
            !folded.contains("Member `guide:operate-the-gate/supporting-doc/notes-checklist`"),
            "no key is invented from the literal segment the pattern spells: {folded}"
        );
    }

    #[test]
    fn a_wildcard_spanned_pattern_still_folds_depth_into_distinct_keys() {
        // One pattern, both halves: `notes/` is literal, so it strips; the `**` spans
        // directories no pattern spells, so what it crossed folds — two same-named files at
        // different depths carry distinct keys rather than collapsing onto one.
        let harness = common::tmpdir("nested-file-wildcard-depth");
        let unit = harness.join(".claude").join("skills").join("coordinate");
        fs::create_dir_all(unit.join("notes").join("deep")).unwrap();
        fs::write(
            unit.join("SKILL.md"),
            "---\nname: coordinate\ndescription: A host skill.\n---\n# coordinate\n",
        )
        .unwrap();
        fs::write(unit.join("notes").join("home.md"), "# Home\n").unwrap();
        fs::write(unit.join("notes").join("deep").join("home.md"), "# Deep\n").unwrap();

        let child = nested_file_kind("supporting-doc");
        let kinds = BTreeMap::from([(
            "skill".to_string(),
            skill_templating("supporting-doc", "notes/**/*.md"),
        )]);
        let found = temper::import::discover_nested_file(
            &temper::import::Discovery::new(&harness),
            &child,
            &kinds,
            temper::import::LocalOverride::Honored,
        );

        // Each child's key, folded the way the composition folds it: the host's unit
        // descended through the segments its pattern spells literally, then the file's
        // remaining placement folded in.
        let keys: Vec<String> = found
            .iter()
            .map(|found| {
                let base = drift::fold_base(&found.host_unit, &found.pattern);
                temper::frontmatter::Member::from_source_rooted(&child, &found.file, &base)
                    .unwrap()
                    .id
            })
            .collect();
        assert_eq!(
            keys,
            vec!["deep-home".to_string(), "home".to_string()],
            "the literal `notes/` strips and the `**`-spanned depth folds: {found:#?}"
        );
    }
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
    use temper::{admissibility, builtin_kind, compose, drift, graph, member_address, read};

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

    /// A host member carrying no reference field — `alpha`'s counterpart for the cases
    /// that need a second host in the corpus rather than a second edge.
    fn host(name: &str, rows: &[NestedMemberRow]) -> Features {
        builtin_kind::features(
            &service_kind(),
            &common::raw_unit(
                name,
                BTreeMap::new(),
                "# Host\n",
                &format!("specs/{name}.md"),
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
    fn a_key_that_is_not_one_address_segment_is_a_malformed_lock() {
        // A member's identity *is* its address, so a key carrying the grammar's own `/`
        // spells one segment too many: `authority/rejected` under `service:alpha` spells
        // `service:alpha/domain/authority/rejected`, which is equally the `rejected` leaf
        // of the sibling keyed `authority`. The segment count decides the grain, so that
        // address is even and a leaf's by construction — without this refusal an edge or
        // an `explain` naming the member answers the sibling's leaf instead, silently and
        // with no dangling finding to read.
        //
        // The coincidence judge is blind to it: `authority` and `authority/rejected` are
        // two distinct `(host, kind, key)` triples, and it counts triples.
        assert!(
            admissibility::nested_member_key_segment(&declarations()).is_empty(),
            "the corpus's own keys are each one segment — the judge is silent on it"
        );

        let mut coincident = declarations();
        coincident
            .nested_members
            .push(domain_row("service:alpha", "authority"));
        coincident
            .nested_members
            .push(domain_row("service:alpha", "authority/rejected"));
        assert!(
            admissibility::nested_member_coincidence(&coincident).is_empty(),
            "two distinct triples are no coincidence by the triple count — this judge's \
             whole reason for existing"
        );

        let findings = admissibility::nested_member_key_segment(&coincident);
        assert_eq!(findings.len(), 1, "one refusal for the one malformed key");
        assert_eq!(findings[0].rule, "nested-member.admissibility");
        assert!(
            findings[0]
                .message
                .contains("service:alpha/domain/authority/rejected"),
            "the refusal names the address both members spell, got: {}",
            findings[0].message
        );

        // The other hole in the grammar: an empty key names nothing at any grain.
        let mut empty = declarations();
        empty.nested_members.push(domain_row("service:alpha", ""));
        let findings = admissibility::nested_member_key_segment(&empty);
        assert_eq!(findings.len(), 1, "an empty key is refused too");
        assert!(
            findings[0].message.contains("service:alpha/domain/"),
            "the refusal names the address the empty key spells, got: {}",
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
        // member under that host — at member grain, whole, with no leaf-address refusal
        // riding along. A nested member's identity *is* its three-segment address, so a
        // reader that re-read it as a malformed four-segment leaf would append the
        // refusal below to an otherwise correct narration.
        for carrier in ["service:alpha/domain/common", "service:beta/domain/common"] {
            let qualified = explain(&by_kind, carrier);
            assert!(
                !qualified.contains("names more than one thing")
                    && !qualified.contains("No member"),
                "`{carrier}` — the spelling the refusal offered — resolves, got: {qualified}"
            );
            assert!(
                !qualified.contains("is not a well-formed leaf address"),
                "`{carrier}` is a member address, never a malformed leaf one, got: {qualified}"
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
    fn explain_at_a_nested_members_address_narrates_every_member_strand() {
        // The positive twin of the refusal arm above: `explain` at a nested member's own
        // `<host-address>/<kind>/<key>` address narrates the three member-grain strands
        // — why, impact, context — and nothing else. The species is settled once, at
        // resolution; the strands below never re-decide it off the address's slashes.
        let declarations = declarations();
        let embedded = compose::embedded_features_by_kind(&declarations);
        let hosts = BTreeMap::from([(
            "service".to_string(),
            vec![alpha(&declarations.nested_members, "billing")],
        )]);
        let by_kind = compose::assemble_by_kind(&hosts, &[], &embedded);

        let address = "service:alpha/domain/common";
        let narration = explain(&by_kind, address);
        for strand in [
            "everything that holds it in place",
            "the blast radius if it is removed or renamed",
            "its declared neighborhood",
        ] {
            assert!(
                narration.contains(&format!("Member `{address}` (domain) — {strand}")),
                "the `{strand}` strand narrates at member grain, got: {narration}"
            );
        }
        assert!(
            !narration.contains("is not a well-formed leaf address"),
            "no strand re-reads the member address as a malformed leaf, got: {narration}"
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
    fn the_same_leaf_tailed_address_does_name_a_leaf() {
        // The twin of the refusal above, and the whole reason the two grains share one
        // parser: `service:beta/domain/common/purpose` names no MEMBER and does name a
        // LEAF. The head is the host address the nested-member address carries, read the
        // same way at both grains.
        let declarations = declarations();
        let embedded = compose::embedded_features_by_kind(&declarations);
        let hosts = BTreeMap::from([(
            "service".to_string(),
            vec![
                alpha(&declarations.nested_members, "domain:common"),
                host("beta", &declarations.nested_members),
            ],
        )]);
        let by_kind = compose::assemble_by_kind(&hosts, &[], &embedded);

        let parsed = member_address::parse_leaf_address("service:beta/domain/common/purpose")
            .expect("four segments is leaf grain");
        assert_eq!(parsed.member, "service:beta");
        let (outer_kind, value) =
            read::resolve_leaf(&by_kind, &parsed).expect("`beta` carries the `purpose` leaf");
        assert_eq!(outer_kind, "service");
        assert_eq!(value, "a domain");

        // Non-vacuity: the host segment is load-bearing, not decoration — the same
        // `(kind, key, leaf)` under a host the corpus has no member for resolves nowhere,
        // even though `alpha` carries a `common` domain with a `purpose` of its own.
        let ghost = member_address::parse_leaf_address("service:ghost/domain/common/purpose")
            .expect("four segments is leaf grain");
        assert!(read::resolve_leaf(&by_kind, &ghost).is_none());
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

/// **Host-keyed file children** — a nested *file* child's identity is the whole
/// `<host-address>/<kind>/<key>` address the grammar gives a nested member, exactly as an
/// embedded member's is. So a child's name is host-scoped rather than corpus-wide: two
/// hosts may each carry a `home`, and neither claims the other's.
mod host_keyed_file_children {
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::Path;

    use temper::extract::Features;
    use temper::{admissibility, compose, drift, import};

    use crate::common;

    /// A skill at its real Claude Code locus, carrying one `home.md` companion — the
    /// shipped `skill` kind templates `supporting-doc` at its unit's `*.md`, so the
    /// companion is that skill's file child by the shipped facts alone.
    fn write_host_with_home(harness: &Path, name: &str) {
        common::write_skill(
            harness,
            name,
            &format!("---\nname: {name}\ndescription: A host skill.\n---\n# {name}\n"),
        );
        fs::write(
            harness
                .join(".claude")
                .join("skills")
                .join(name)
                .join("home.md"),
            "# Home\n",
        )
        .unwrap();
    }

    /// The whole live corpus, composed the way `check` composes it: the committed lock's
    /// rows assembled, then every built-in kind's members resolved off harness disk.
    fn compose_corpus(harness: &Path) -> BTreeMap<String, compose::KindUnitsAndFeatures> {
        common::write_lock(harness, drift::Declarations::default());
        let committed = drift::read_declarations(&harness.join(temper::WORKSPACE_DIR)).unwrap();
        let disc = import::Discovery::new(harness);
        let cache: compose::ManifestCache = BTreeMap::new();
        let family = compose::assemble_lock_family(&disc, &committed, &[], &cache).unwrap();
        compose::builtin_units_and_features_by_kind(
            &family.overlaid_builtin_kinds,
            &disc,
            &family.declarations,
            &cache,
        )
        .unwrap()
    }

    #[test]
    fn two_hosts_each_carrying_a_home_compose_two_addresses_and_no_coincidence() {
        let harness = common::tmpdir("file-child-host-keyed");
        write_host_with_home(&harness, "alpha");
        write_host_with_home(&harness, "beta");

        let corpus = compose_corpus(&harness);
        let children = &corpus
            .get("supporting-doc")
            .expect("`supporting-doc` ships as a built-in kind")
            .features;

        // Each child is named by its whole address: the host segment is what tells the two
        // apart, and it is the only thing that differs between them.
        assert_eq!(
            children
                .iter()
                .map(|features| features.id.as_str())
                .collect::<Vec<_>>(),
            vec![
                "skill:alpha/supporting-doc/home",
                "skill:beta/supporting-doc/home",
            ],
        );

        // So the member-grain coincidence check sees two addresses rather than one name
        // twice — the `member.admissibility` refusal a corpus-wide `kind:name` keying
        // raised for a corpus that is perfectly well-formed.
        let by_kind: BTreeMap<&str, &[Features]> = corpus
            .iter()
            .map(|(kind, uaf)| (kind.as_str(), uaf.features.as_slice()))
            .collect();
        assert!(
            admissibility::member_address_coincidence(&by_kind).is_empty(),
            "two hosts carrying a same-named child is two members, never a coincidence"
        );

        // And the gate agrees end to end: both children are checked, and nothing refuses.
        let (findings, ok) = common::check_harness(&harness);
        assert!(
            common::findings_for(&findings, "member.admissibility").is_empty(),
            "no coincidence is reported: {findings:#?}"
        );
        let checked = common::findings_for(&findings, "coverage.checked");
        assert!(
            checked
                .iter()
                .any(|line| line.contains("supporting-doc (2)")),
            "both file children are checked members: {findings:#?}"
        );
        assert!(ok, "the corpus checks clean: {findings:#?}");
    }
}

/// **Host-keyed file children, through the authoring face.** The SDK keys a nested file
/// child by the same host-qualified address the engine resolves it at, so two hosts each
/// carrying a `home` child are two members rather than one identity twice. Driven through
/// the real seam — `node` runs the authored program, the engine compiles every projection
/// — because the keying is a claim about what the two faces agree on.
mod host_keyed_file_children_through_the_sdk {
    use temper::drift::{self, EmitOptions};

    use crate::common;

    /// The inbox reproduction as an authored program: a directory-unit `unit` host
    /// templating a `note` file child at `*.json`, with one child named `home` under each
    /// of two hosts. Corpus-wide `<kind>:<name>` keying refused this at
    /// `duplicate identity key 'note:home'` — a corpus that is perfectly well-formed.
    ///
    /// `alpha`'s body cites `beta`'s child, so the address the authoring face spells for a
    /// file child rides the lock: emit resolves the mention against its own declared set
    /// before writing a byte, and the row it writes is asserted below.
    const TWO_HOSTS_ONE_CHILD_NAME: &str = r#"
import { emit, harness, kind, mentionOf, text } from "@dtmd/temper";

const note = kind<{ title: string }>({
  name: "note",
  locus: { kind: "nested-file" },
  unitShape: "file",
  format: "json-document",
  registration: [],
});

const unit = kind<object>({
  name: "unit",
  locus: { kind: "at", root: "docs", glob: "*/UNIT.md" },
  unitShape: "directory",
  registration: [],
  templates: [{ kind: note, path: "*.json" }],
});

const beta = unit({ name: "beta", prose: text`# Beta` });
const betaHome = note({ name: "home", host: beta, title: "Beta's home" });
const alpha = unit({ name: "alpha", prose: text`# Alpha\n\nBeta keeps ${mentionOf(betaHome)}.` });

process.stdout.write(
  emit(
    harness({
      members: [
        alpha,
        beta,
        note({ name: "home", host: alpha, title: "Alpha's home" }),
        betaHome,
      ],
    }),
  ).seam,
);
"#;

    #[test]
    fn two_hosts_each_carrying_a_home_child_emit_both_files_and_check_counts_both() {
        let (harness, into) =
            common::wire_sdk_harness("sdk-file-child-host-keyed", TWO_HOSTS_ONE_CHILD_NAME);

        let report = drift::emit_program(&into, EmitOptions::default()).expect(
            "two hosts each carrying a `home` child is two addresses, so the program emits",
        );

        // Both children project, each under its own host's unit — the host segment of the
        // address is the whole of what tells them apart, and it is what the path composes
        // from too.
        for host in ["alpha", "beta"] {
            let projection = harness.join("docs").join(host).join("home.json");
            assert!(
                report
                    .entries
                    .iter()
                    .any(|entry| entry.kind == "note" && entry.source_path == projection),
                "`{host}`'s child is its own emit entry: {:#?}",
                report.entries
            );
            assert!(projection.is_file(), "emit wrote {}", projection.display());
        }

        // The mention rode the child's own address across the seam, and the host's body
        // renders the bare display text the one corpus-wide rule gives it.
        let cite = std::fs::read_to_string(harness.join("docs").join("alpha").join("UNIT.md"))
            .expect("alpha's own projection");
        assert!(cite.contains("Beta keeps home."), "rendered body: {cite}");
        let mentions = drift::read_declarations(&into)
            .expect("the emitted lock exists and is valid")
            .mentions;
        assert_eq!(
            mentions
                .iter()
                .map(|row| (row.member.as_str(), row.target.as_str()))
                .collect::<Vec<_>>(),
            vec![("unit:alpha", "unit:beta/note/home")],
        );

        // And the gate reads both back off disk: two members of the child kind, nothing
        // refused.
        let (findings, ok) = common::check_harness(&harness);
        let checked = common::findings_for(&findings, "coverage.checked");
        assert!(
            checked.iter().any(|line| line.contains("note (2)")),
            "both file children are checked members: {findings:#?}"
        );
        assert!(ok, "the corpus checks clean: {findings:#?}");
    }
}

/// **Nesting reaches any depth.** A `page` is a nested file child *and* a host: it owns a
/// directory unit under its `area`'s unit, and templates a `leaf` layer of its own under
/// that. One rule composes every layer — a host's unit is the directory its own
/// projection sits in, whatever derived that projection — so a corpus two layers deep
/// emits, discovers and resolves exactly as a corpus one layer deep does
/// (`specs/model/representation.md`, "nesting": a kind may template inner layers to
/// arbitrary depth).
mod nested_layers_compose_to_any_depth {
    use std::fs;

    use temper::drift::{self, EmitOptions};

    use crate::common;

    /// Three declared layers: an `area` at an `at` locus, a `page` nested under it with a
    /// **directory** unit of its own, and a `leaf` nested under the page at a pattern
    /// carrying a literal directory. The middle layer is the whole point — its template
    /// pattern names its entry file the way a directory kind's glob does (`*/PAGE.md`),
    /// and the layer beneath it composes against the unit that pattern seats, not against
    /// any root.
    const TWO_LAYER_PROGRAM: &str = r#"
import { emit, harness, kind, text } from "@dtmd/temper";

const leaf = kind<object>({
  name: "leaf",
  locus: { kind: "nested-file" },
  unitShape: "file",
  registration: [],
});

const page = kind<object>({
  name: "page",
  locus: { kind: "nested-file" },
  unitShape: "directory",
  registration: [],
  templates: [{ kind: leaf, path: "notes/*.md" }],
});

const area = kind<object>({
  name: "area",
  locus: { kind: "at", root: ".claude/areas", glob: "*/AREA.md" },
  unitShape: "directory",
  registration: [],
  templates: [{ kind: page, path: "*/PAGE.md" }],
});

const ops = area({ name: "ops", prose: text`# Ops` });
const gate = page({ name: "gate", host: ops, prose: text`# The gate` });

process.stdout.write(
  emit(
    harness({
      members: [ops, gate, leaf({ name: "home", host: gate, prose: text`# Home` })],
    }),
  ).seam,
);
"#;

    #[test]
    fn a_child_two_layers_down_emits_at_its_composed_path_and_reads_back_at_both_hosts() {
        let (harness, into) = common::wire_sdk_harness("two-layer-nesting", TWO_LAYER_PROGRAM);

        let report = drift::emit_program(&into, EmitOptions::default())
            .expect("a two-layer corpus composes every layer's locus and emits");

        let projection = |kind: &str, name: &str| {
            report
                .entries
                .iter()
                .find(|entry| entry.kind == kind && entry.name == name)
                .unwrap_or_else(|| panic!("emit projects `{kind}:{name}`"))
                .source_path
                .clone()
        };

        // Layer by layer: the area's own `at` locus, the page's unit under it (its
        // template's `*/PAGE.md` names the entry file the way a directory kind's glob
        // does), and the leaf under *that* unit, through its own pattern's literal
        // `notes/` segment.
        assert_eq!(
            projection("area", "ops"),
            harness.join(".claude/areas/ops/AREA.md")
        );
        assert_eq!(
            projection("page", "gate"),
            harness.join(".claude/areas/ops/gate/PAGE.md")
        );
        assert_eq!(
            projection("leaf", "home"),
            harness.join(".claude/areas/ops/gate/notes/home.md")
        );
        for (kind, name) in [("area", "ops"), ("page", "gate"), ("leaf", "home")] {
            assert!(
                projection(kind, name).is_file(),
                "`{kind}:{name}` is on disk"
            );
        }

        // The read side finds all three — the per-host scan descends into a host that is
        // itself a nested file child, so the deepest layer is discovered rather than
        // invisible.
        let (findings, ok) = common::check_harness(&harness);
        let checked = common::findings_for(&findings, "coverage.checked");
        assert!(
            checked.iter().any(|line| line.contains("page (1)"))
                && checked.iter().any(|line| line.contains("leaf (1)")),
            "both nested layers are checked members: {findings:#?}"
        );
        assert!(ok, "the two-layer corpus checks clean: {findings:#?}");

        // And the address composes through **both** hosts: five segments, the grandparent
        // intact. The nearer-host-only spelling names nothing.
        let deep = common::explain_in(&harness, "area:ops/page/gate/leaf/home");
        assert!(
            deep.contains("Member `area:ops/page/gate/leaf/home`"),
            "the deep child resolves at its two-host-composed address: {deep}"
        );
        let flattened = common::explain_in(&harness, "page:gate/leaf/home");
        assert!(
            !flattened.contains("Member `page:gate/leaf/home`"),
            "a host flattened to `kind:name` loses the grandparent and names nothing: \
             {flattened}"
        );
    }

    /// A kind whose file layer reaches back to itself composes **no** locus: the unit its
    /// members would sit in is their own descendant's. Arbitrary depth is three declared
    /// layers deep as above, never a kind containing itself — so the declaration is
    /// refused where it is declared rather than recursed into (invariant 6).
    #[test]
    fn a_kind_templating_its_own_file_layer_is_refused_rather_than_recursed_into() {
        use temper::drift::{Declarations, KindFactRow, Payload, TemplateRow};

        let (_harness, into) = common::workspace("self-templating-kind");
        let payload = Payload {
            version: drift::SEAM_VERSION,
            declarations: Declarations {
                kinds: vec![KindFactRow {
                    governs_root: None,
                    governs_glob: None,
                    unit_shape: Some("directory".to_string()),
                    templates: vec![TemplateRow {
                        kind: "page".to_string(),
                        path: Some("*/PAGE.md".to_string()),
                    }],
                    ..common::kind_facts("page", "", "")
                }],
                ..Declarations::default()
            },
            members: Vec::new(),
        };

        let err = drift::emit(&payload, &into, EmitOptions::default())
            .expect_err("a self-templating nested file kind composes no path and is refused");
        let rendered = format!("{err:?}");
        assert!(
            rendered.contains("page -> page") && rendered.contains("cycle"),
            "the refusal names the cycle it found: {rendered}"
        );

        // The refusal lands before a byte is written: emit is the sole writer of the lock,
        // so a cyclic declaration never reaches a committed corpus for the read side to
        // walk.
        assert!(
            !into.join("lock.toml").exists(),
            "a refused emit writes no lock"
        );
    }

    /// The same cycle, met from the **read** side: the per-host descent is handed a kind
    /// graph with no base case and still terminates, finding nothing. Emit refuses such a
    /// declaration outright, so this is the walk's own totality rather than a second
    /// verdict on the corpus.
    #[test]
    fn the_per_host_descent_terminates_on_a_kind_graph_with_no_base_case() {
        use std::collections::BTreeMap;

        use temper::drift::{KindFactRow, TemplateRow};
        use temper::kind::CustomKind;

        let harness = common::tmpdir("self-templating-descent");
        fs::create_dir_all(harness.join(".claude")).unwrap();

        let page = CustomKind::from_kind_fact_row(&KindFactRow {
            governs_root: None,
            governs_glob: None,
            unit_shape: Some("directory".to_string()),
            templates: vec![TemplateRow {
                kind: "page".to_string(),
                path: Some("*/PAGE.md".to_string()),
            }],
            ..common::kind_facts("page", "", "")
        })
        .unwrap();
        let kinds = BTreeMap::from([("page".to_string(), page.clone())]);

        let found = temper::import::discover_nested_file(
            &temper::import::Discovery::new(&harness),
            &page,
            &kinds,
            temper::import::LocalOverride::Honored,
        );
        assert!(
            found.is_empty(),
            "a cycle roots no unit anywhere, so the descent finds nothing: {found:#?}"
        );
    }
}

/// **A drift finding names its member by the one address grammar.** Two hosts each carry
/// a `checklist`, which is exactly what a nested member's address is scoped to tell apart
/// (`specs/model/representation.md`, "member") — so the `root.fresh` judge must name each
/// child's whole `<host-address>/<kind>/<key>` address, never its bare key, and a
/// top-level member's row must still read `<kind>:<name>`.
mod a_fresh_finding_names_the_members_whole_address {
    use std::fs;

    use temper::drift::{self, EmitOptions};

    use crate::common;

    /// Two guides, each templating a `*.md` child, and each carrying a child keyed
    /// `checklist` — the collision the host segment of the address resolves.
    const TWO_HOSTS_ONE_KEY: &str = r#"
import { emit, harness, kind, text } from "@dtmd/temper";

const supportingDoc = kind<object>({
  name: "supporting-doc",
  locus: { kind: "nested-file" },
  unitShape: "file",
  registration: [],
});

const guide = kind<object>({
  name: "guide",
  locus: { kind: "at", root: ".claude/guides", glob: "*/GUIDE.md" },
  unitShape: "directory",
  registration: [],
  templates: [{ kind: supportingDoc, path: "*.md" }],
});

const operating = guide({ name: "operate-the-gate", prose: text`# Operate the gate` });
const adopting = guide({ name: "adopt-the-harness", prose: text`# Adopt the harness` });

process.stdout.write(
  emit(
    harness({
      members: [
        operating,
        adopting,
        supportingDoc({ name: "checklist", host: operating, prose: text`# Operate checklist` }),
        supportingDoc({ name: "checklist", host: adopting, prose: text`# Adopt checklist` }),
      ],
    }),
  ).seam,
);
"#;

    #[test]
    fn a_hand_edited_childs_finding_names_its_host_and_a_top_levels_still_reads_kind_name() {
        let (harness, into) = common::wire_sdk_harness("fresh-finding-address", TWO_HOSTS_ONE_KEY);
        drift::emit_program(&into, EmitOptions::default())
            .expect("two hosts each carrying a `checklist` is one address apiece, never a clash");

        // The lock is the only place the host fact can reach the judge, so the column it
        // writes is pinned here: a child's row carries its host's own whole address, and a
        // top-level row carries no `host` key at all — the byte-level reason no committed
        // lock of top-level members moves on this column's account.
        let lock = fs::read_to_string(into.join("lock.toml")).unwrap();
        assert!(
            lock.contains("host = \"guide:operate-the-gate\"")
                && lock.contains("host = \"guide:adopt-the-harness\""),
            "each child's roll-up row carries its host's own address: {lock}"
        );
        assert_eq!(
            lock.matches("host = ").count(),
            2,
            "and only the two children carry one — a top-level row keeps its four \
             columns: {lock}"
        );

        // Hand-edit all four projections, so every row is drifted and the judge must name
        // four distinct members rather than agree with an absence.
        for relative in [
            ".claude/guides/operate-the-gate/GUIDE.md",
            ".claude/guides/adopt-the-harness/GUIDE.md",
            ".claude/guides/operate-the-gate/checklist.md",
            ".claude/guides/adopt-the-harness/checklist.md",
        ] {
            let path = harness.join(relative);
            let edited = format!("{}\n\nhand-edited.\n", fs::read_to_string(&path).unwrap());
            fs::write(&path, edited).unwrap();
        }

        let findings = drift::config_stale_from_doc(
            &drift::read_lock_document(&into).unwrap(),
            &into,
            &common::fresh_clause(),
        );
        let messages = common::messages(&findings);
        assert_eq!(
            messages.len(),
            4,
            "one finding per drifted row — two hosts and their two children: {messages:#?}"
        );

        // Each child is named by its whole address, so the two same-keyed children are two
        // findings a reader can tell apart; each host still reads `<kind>:<name>`.
        for member in [
            "guide:operate-the-gate",
            "guide:adopt-the-harness",
            "guide:operate-the-gate/supporting-doc/checklist",
            "guide:adopt-the-harness/supporting-doc/checklist",
        ] {
            assert_eq!(
                messages
                    .iter()
                    .filter(|line| line.contains(&format!("(member `{member}`)")))
                    .count(),
                1,
                "`{member}` is named by exactly one finding: {messages:#?}"
            );
        }

        // And the bare key is spelled nowhere: it names no member, and it named both
        // children identically before the host fact reached the judge.
        assert!(
            !messages
                .iter()
                .any(|line| line.contains("(member `checklist`)")),
            "a bare key spells an address no member wears: {messages:#?}"
        );
    }
}

/// **A placement refusal names its member by the one address grammar too.** The two
/// refusals the write face raises before a byte is written — `NestedFileLocus`, when a
/// host supplies no half of the composition, and `UngovernedProjection`, when the derived
/// path is one the deriving pattern cannot find — are findings about a *nested* member, so
/// each names the whole `<host-address>/<kind>/<key>` its subject wears
/// (`specs/model/representation.md`, "member"). A bare key names no member, and names two
/// same-keyed children under two hosts identically.
///
/// One emit per fixture: a refusal aborts the pass, so two addresses can never appear in
/// one report — the pair is told apart by comparing the two reports.
mod a_placement_refusal_names_the_members_whole_address {
    use temper::drift::{self, Declarations, EmitOptions, KindFactRow, Payload, PayloadMember};

    use crate::common;

    /// A `guide` host at an `at` locus owning a directory unit, and templating **no** file
    /// layer — the half of the composition a nested child cannot supply for itself.
    fn kinds_templating_no_layer() -> Vec<KindFactRow> {
        vec![
            KindFactRow {
                unit_shape: Some("directory".to_string()),
                ..common::kind_facts("guide", ".claude/guides", "*/GUIDE.md")
            },
            KindFactRow {
                governs_root: None,
                governs_glob: None,
                unit_shape: Some("file".to_string()),
                ..common::kind_facts("supporting-doc", "", "")
            },
        ]
    }

    /// One `supporting-doc` member keyed `checklist`, under `host` (or under none).
    fn checklist(host: Option<&str>) -> PayloadMember {
        common::payload_member("supporting-doc", "checklist", host, "# Checklist\n")
    }

    /// Emit the one-member corpus and render the refusal it raises.
    fn refusal(label: &str, kinds: Vec<KindFactRow>, members: Vec<PayloadMember>) -> String {
        let (_harness, into) = common::workspace(label);
        let payload = Payload {
            version: drift::SEAM_VERSION,
            declarations: Declarations {
                kinds,
                ..Declarations::default()
            },
            members,
        };
        let err = drift::emit(&payload, &into, EmitOptions::default())
            .expect_err("a child with no composable placement is refused, never guessed");
        format!("{err:?}")
    }

    #[test]
    fn a_nested_file_childs_locus_refusal_names_its_host_and_a_hostless_ones_reads_kind_name() {
        let under_operating = refusal(
            "locus-refusal-operating",
            kinds_templating_no_layer(),
            vec![checklist(Some("guide:operate-the-gate"))],
        );
        assert!(
            under_operating.contains("`guide:operate-the-gate/supporting-doc/checklist`")
                && under_operating.contains("has no path to compose"),
            "the refusal names the child's whole address: {under_operating}"
        );

        // The second host's same-keyed child: a separate emit, because the refusal aborts
        // the pass — so the pair is told apart across the two reports rather than within
        // one.
        let under_adopting = refusal(
            "locus-refusal-adopting",
            kinds_templating_no_layer(),
            vec![checklist(Some("guide:adopt-the-harness"))],
        );
        assert!(
            under_adopting.contains("`guide:adopt-the-harness/supporting-doc/checklist`"),
            "and the other host's child is named under its own host: {under_adopting}"
        );
        assert_ne!(
            under_operating, under_adopting,
            "two same-keyed children under two hosts are two refusals a reader can tell \
             apart"
        );

        // The bare key is spelled by neither: it names no member, and it named both of
        // these identically before the host column reached the refusal.
        for rendered in [&under_operating, &under_adopting] {
            assert!(
                !rendered.contains("`checklist`"),
                "a bare key spells an address no member wears: {rendered}"
            );
        }

        // And the arm that has no host at all still reads `<kind>:<name>` — the top-level
        // floor of the same grammar, not a nested address with an empty host.
        let hostless = refusal(
            "locus-refusal-hostless",
            kinds_templating_no_layer(),
            vec![checklist(None)],
        );
        assert!(
            hostless.contains("`supporting-doc:checklist`")
                && hostless.contains("it names no host member"),
            "a child naming no host is refused at its `<kind>:<name>` address: {hostless}"
        );
    }

    #[test]
    fn an_ungoverned_childs_projection_refusal_names_its_host_too() {
        // An admissible declaration that reaches the round-trip refusal: the child owns a
        // **directory** unit, so it places `<name>/<entry>` — one segment deep — while its
        // host's pattern carries a literal directory prefix of its own, which that
        // placement never lands under.
        let kinds = vec![
            KindFactRow {
                unit_shape: Some("directory".to_string()),
                templates: vec![temper::drift::TemplateRow {
                    kind: "page".to_string(),
                    path: Some("notes/PAGE.md".to_string()),
                }],
                ..common::kind_facts("area", ".claude/areas", "*/AREA.md")
            },
            KindFactRow {
                governs_root: None,
                governs_glob: None,
                unit_shape: Some("directory".to_string()),
                ..common::kind_facts("page", "", "")
            },
        ];
        let members = vec![
            common::payload_member("area", "ops", None, "# Ops\n"),
            common::payload_member("page", "gate", Some("area:ops"), "# The gate\n"),
        ];

        let rendered = refusal("ungoverned-child-address", kinds, members);
        assert!(
            rendered.contains("`area:ops/page/gate`") && rendered.contains("`gate/PAGE.md`"),
            "the ungoverned projection names the child's whole address: {rendered}"
        );
        assert!(
            !rendered.contains("`page:gate`"),
            "never the host-less `kind:name` a nested child does not wear: {rendered}"
        );
    }
}
