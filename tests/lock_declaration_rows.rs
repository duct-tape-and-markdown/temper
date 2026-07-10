//! The lock's declaration-row family — the composed program's erased declarations.
//!
//! `emit` is the sole producer of a declaration-row family (kind facts, clauses,
//! requirements — including the set-scope `count`/`unique`/`membership`/`degree`
//! facets — assembly facts, and the member→requirement `satisfies` family) beside the
//! existing provenance + emit-fingerprint rows, and the drift/gate side reads it back
//! through [`temper::drift::read_declarations`]. These tests drive `emit` directly over
//! hand-built [`Payload`]s — a golden-lock fixture (`tests/emit.rs`'s pattern), no
//! scratch import — asserting the family is present and populated, that a double emit is
//! byte-stable — the round-trip emit pins — and that a bare payload (no requirements,
//! no satisfies) still round-trips.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

mod common;

use temper::builtin;
use temper::builtin_lock;
use temper::contract::Severity;
use temper::drift::{
    self, AssemblyFactRow, BoundRow, CharsetRow, ClauseRow, CollectionEntryRow, CountBoundRow,
    Declarations, DegreeBoundRow, EdgeBoundRow, EmitOptions, KindFactRow, LayoutRegionRow,
    LayoutRow, MentionRow, NestedMemberRow, Payload, PayloadMember, RequirementRow, SatisfiesRow,
};
use temper::kind::{Content, CustomKind, Layout, LayoutRegion};

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

/// A host kind declaring one embedded nesting template — the `decision` child kind,
/// the shape [`tests/nested_member.rs`]'s `decision_kind` declares live.
fn spec_kind_facts_with_template() -> KindFactRow {
    KindFactRow {
        format: Some("yaml-frontmatter".to_string()),
        unit_shape: Some("directory".to_string()),
        templates: vec!["decision".to_string()],
        ..common::kind_facts("spec", "specs", "*.md")
    }
}

/// A `spec` kind declaring a `layout` content in all three corpus primitives — an
/// importing prose region, a field section filling a named slot, and a member collection
/// of a named kind carrying an explicit key. The shape an SDK-declared layout kind's row
/// carries into the lock.
fn spec_kind_facts_with_layout() -> KindFactRow {
    KindFactRow {
        content: Some(LayoutRow {
            regions: vec![
                LayoutRegionRow {
                    region: "prose".to_string(),
                    import: Some("specs/intent.md".to_string()),
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
                    key: Some("core".to_string()),
                },
            ],
        }),
        ..common::kind_facts("spec", "specs", "*.md")
    }
}

/// The one skill + one rule this file's payloads project.
fn skill_and_rule_members() -> Vec<PayloadMember> {
    vec![
        common::skill_member(
            "coordinate",
            "Use when coordinating agents across axes; not for single-axis work.",
            "# Coordinate\n\nDrive the team through the playbook.\n",
        ),
        common::rule_member(
            "rust",
            Some(&["src/**/*.rs"]),
            "# Rust conventions\n\nPrefer a clone over a lifetime fight.\n",
        ),
    ]
}

/// A rich declaration set: a `block` enforcement mode, a `required` requirement, a
/// second requirement exercising every set-scope facet (`count`/`unique`/`membership`/
/// `degree`), and a member that opts into both via `satisfies` — so the requirement and
/// satisfies families carry more than the bare-payload minimum.
fn rich_declarations() -> Declarations {
    Declarations {
        kinds: vec![
            common::rule_kind_facts(Some("claude-code"), &["paths-match(paths)"]),
            common::skill_kind_facts(
                Some("claude-code"),
                &["user-invoked", "description-trigger(description)"],
            ),
        ],
        clauses: vec![
            ClauseRow {
                kind: Some("skill".to_string()),
                field: Some("description".to_string()),
                ..common::clause("required", "required")
            },
            ClauseRow {
                kind: Some("rule".to_string()),
                field: Some("paths".to_string()),
                ..common::clause("required", "advisory")
            },
        ],
        requirements: vec![
            RequirementRow {
                required: true,
                ..common::requirement("review-coverage", false, Some("skill"))
            },
            RequirementRow {
                clauses: vec![
                    common::required_clause_row(
                        "count",
                        None,
                        Some(CountBoundRow { min: 1, max: 2 }),
                        None,
                        None,
                    ),
                    ClauseRow {
                        field: Some("name".to_string()),
                        ..common::clause("unique", "advisory")
                    },
                    common::required_clause_row(
                        "membership",
                        Some("name"),
                        None,
                        Some("review-coverage"),
                        None,
                    ),
                    common::required_clause_row(
                        "degree",
                        None,
                        None,
                        None,
                        Some(DegreeBoundRow {
                            incoming: Some(EdgeBoundRow {
                                min: Some(1),
                                max: None,
                            }),
                            outgoing: Some(EdgeBoundRow {
                                min: None,
                                max: Some(3),
                            }),
                        }),
                    ),
                ],
                ..common::requirement("roster-coverage", false, Some("skill"))
            },
        ],
        assembly: vec![AssemblyFactRow {
            fact: "mode".to_string(),
            value: Some("block".to_string()),
            from: None,
            field: None,
            to: None,
        }],
        satisfies: vec![
            SatisfiesRow {
                member: "coordinate".to_string(),
                requirement: "review-coverage".to_string(),
            },
            SatisfiesRow {
                member: "coordinate".to_string(),
                requirement: "roster-coverage".to_string(),
            },
        ],
        mentions: vec![MentionRow {
            member: "skill:coordinate".to_string(),
            target: "rule:rust".to_string(),
        }],
        nested_members: Vec::new(),
    }
}

/// The whole seam payload: the one skill + one rule member, plus `declarations`.
fn golden_payload(declarations: Declarations) -> Payload {
    Payload {
        version: drift::SEAM_VERSION,
        declarations,
        members: skill_and_rule_members(),
    }
}

/// Compile `payload`'s projections and its whole lock into a fresh `<harness>/.temper`
/// pair (`tests/emit.rs`'s `workspace` pattern) — the golden-lock fixture standing in for
/// `import::run`, the retired scratch-copy producer.
fn emitted(label: &str, payload: &Payload) -> (PathBuf, PathBuf) {
    let harness = common::tmpdir(&format!("{label}-src"));
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    drift::emit(payload, &into, EmitOptions::default()).unwrap();
    (harness, into)
}

#[test]
fn lock_carries_all_four_declaration_families() {
    let payload = golden_payload(rich_declarations());
    let (_harness, into) = emitted("families", &payload);
    let declarations = drift::read_declarations(&into).unwrap();

    // Kind facts: one per member-discovering built-in kind, name-sorted, carrying the
    // declared runtime facts.
    let skill = declarations
        .kinds
        .iter()
        .find(|k| k.name == "skill")
        .expect("the skill kind fact is recorded");
    assert_eq!(skill.provider.as_deref(), Some("claude-code"));
    assert_eq!(skill.governs_root, ".claude/skills");
    assert_eq!(skill.governs_glob, "*/SKILL.md");
    assert_eq!(skill.format.as_deref(), Some("yaml-frontmatter"));
    assert_eq!(skill.unit_shape.as_deref(), Some("directory"));
    assert_eq!(
        skill.registration,
        vec![
            "user-invoked".to_string(),
            "description-trigger(description)".to_string()
        ]
    );
    assert!(
        declarations.kinds.iter().any(|k| k.name == "rule"),
        "the rule kind fact is recorded"
    );

    // Clauses: the built-in floor contract's clauses, keyed by kind.
    assert!(
        !declarations.clauses.is_empty(),
        "the floor clauses are recorded"
    );
    assert!(
        declarations
            .clauses
            .iter()
            .any(|c| c.kind.as_deref() == Some("skill")),
        "skill floor clauses are keyed by kind"
    );
    for clause in &declarations.clauses {
        assert!(
            matches!(clause.severity.as_str(), "required" | "advisory"),
            "a clause severity is one of the declared vocabulary, got {:?}",
            clause.severity
        );
    }

    // Requirements: the assembly's `[requirement.*]` obligations.
    let requirement = declarations
        .requirements
        .iter()
        .find(|r| r.name == "review-coverage")
        .expect("the declared requirement is recorded");
    assert_eq!(requirement.kind.as_deref(), Some("skill"));
    assert!(requirement.required);

    // The set-scope demands: count/unique/membership/degree all carried as clause
    // rows nested on the requirement.
    let roster = declarations
        .requirements
        .iter()
        .find(|r| r.name == "roster-coverage")
        .expect("the set-scope requirement is recorded");
    let count = roster
        .clauses
        .iter()
        .find(|c| c.predicate == "count")
        .and_then(|c| c.count)
        .expect("count bound is recorded");
    assert_eq!((count.min, count.max), (1, 2));
    let unique = roster
        .clauses
        .iter()
        .find(|c| c.predicate == "unique")
        .expect("unique clause is recorded");
    assert_eq!(unique.field.as_deref(), Some("name"));
    let membership = roster
        .clauses
        .iter()
        .find(|c| c.predicate == "membership")
        .expect("membership clause is recorded");
    assert_eq!(membership.field.as_deref(), Some("name"));
    assert_eq!(membership.target.as_deref(), Some("review-coverage"));
    let degree = roster
        .clauses
        .iter()
        .find(|c| c.predicate == "degree")
        .and_then(|c| c.degree.as_ref())
        .expect("degree bound is recorded");
    assert_eq!(degree.incoming.expect("incoming bound").min, Some(1));
    assert_eq!(degree.incoming.expect("incoming bound").max, None);
    assert_eq!(degree.outgoing.expect("outgoing bound").max, Some(3));
    assert_eq!(degree.outgoing.expect("outgoing bound").min, None);

    // Satisfies: the in-place member's declared fill keys, one row per key.
    let mut satisfied: Vec<&str> = declarations
        .satisfies
        .iter()
        .filter(|row| row.member == "coordinate")
        .map(|row| row.requirement.as_str())
        .collect();
    satisfied.sort_unstable();
    assert_eq!(satisfied, vec!["review-coverage", "roster-coverage"]);

    // Assembly facts: the root member's declared enforcement mode.
    let mode = declarations
        .assembly
        .iter()
        .find(|f| f.fact == "mode")
        .expect("the mode fact is recorded");
    assert_eq!(mode.value.as_deref(), Some("block"));

    // Mentions: the citing member's own address and the address its `n` names.
    assert_eq!(
        declarations.mentions,
        vec![MentionRow {
            member: "skill:coordinate".to_string(),
            target: "rule:rust".to_string(),
        }]
    );
}

#[test]
fn a_double_emit_is_byte_stable() {
    let payload = golden_payload(rich_declarations());
    let (_harness, into) = emitted("byte-stable", &payload);
    let lock = into.join("lock.toml");
    let first = fs::read(&lock).unwrap();

    // The declaration rows are a pure function of the same payload, so re-emitting
    // reproduces the whole lock byte-for-byte.
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    let second = fs::read(&lock).unwrap();
    assert_eq!(first, second, "a re-emit must not churn the lock");

    // The declaration table survived the round-trip: reading it back yields the same
    // populated families.
    let declarations = drift::read_declarations(&into).unwrap();
    assert!(!declarations.kinds.is_empty());
    assert!(!declarations.clauses.is_empty());
    assert!(!declarations.requirements.is_empty());
    assert!(!declarations.assembly.is_empty());
    assert!(!declarations.satisfies.is_empty());
    assert!(!declarations.mentions.is_empty());
}

/// A requirement's authored `prose` (contract.md, "requirement — a shipped kind, not
/// a primitive": carried, never interpreted) round-trips the lock row byte-for-byte —
/// the emit/read_declarations half of the pipe `emit`'s the sole writer of.
#[test]
fn a_requirements_prose_round_trips_the_lock_row_verbatim() {
    let mut declarations = rich_declarations();
    declarations.requirements[0].prose =
        Some("the corpus declares a governance model an architecture doc must satisfy".to_string());
    let payload = golden_payload(declarations);
    let (_harness, into) = emitted("requirement-prose", &payload);

    let read_back = drift::read_declarations(&into).unwrap();
    let requirement = read_back
        .requirements
        .iter()
        .find(|r| r.name == "review-coverage")
        .expect("the requirement is recorded");
    assert_eq!(
        requirement.prose.as_deref(),
        Some("the corpus declares a governance model an architecture doc must satisfy"),
        "the authored prose round-trips the lock row verbatim"
    );
}

/// The other half of the pipe: a lock-declared requirement's `prose` reaches the
/// running engine's own composed [`compose::Requirement`] and out through `explain`'s
/// narration verbatim — the persistence gap this entry closes (`main.rs`'s
/// `requirement_from_row` used to default it to `None` regardless of the row).
#[test]
fn a_requirements_prose_reaches_explains_narration_through_the_engine() {
    let root = common::tmpdir("requirement-prose-engine-compose");
    common::write_skill(
        &root,
        "governance-doc",
        &common::clean_skill("governance-doc"),
    );
    common::write_lock(
        &root,
        Declarations {
            requirements: vec![RequirementRow {
                prose: Some(
                    "the corpus declares a governance model an architecture doc must satisfy"
                        .to_string(),
                ),
                required: true,
                ..common::requirement("governance", false, Some("skill"))
            }],
            ..Declarations::default()
        },
    );
    common::author_satisfies(&root, "skills", "governance-doc", &["governance"]);

    let out = explain_in(&root, "governance");
    assert!(
        out.contains("the corpus declares a governance model an architecture doc must satisfy"),
        "explain's engine-composed narration must carry the lock-declared prose verbatim, got:\n{out}"
    );
}

/// A `ClauseRow` carrying the node-set/edge-scope predicates' arguments
/// (`REQUIREMENT-CLAUSES-ALGEBRA`) round-trips through `to_table`/`from_table` byte-stably
/// — the same law-5 double-emit guarantee `a_double_emit_is_byte_stable` pins for the rest
/// of the declaration-row family.
#[test]
fn a_clause_row_carrying_set_and_edge_scope_args_round_trips_byte_stably() {
    let mut declarations = rich_declarations();
    declarations.clauses.push(ClauseRow {
        kind: Some("skill".to_string()),
        count: Some(CountBoundRow { min: 1, max: 3 }),
        ..common::clause("count", "required")
    });
    declarations.clauses.push(ClauseRow {
        kind: Some("skill".to_string()),
        field: Some("name".to_string()),
        ..common::clause("unique", "advisory")
    });
    declarations.clauses.push(ClauseRow {
        kind: Some("skill".to_string()),
        field: Some("model".to_string()),
        target: Some("approved-models".to_string()),
        ..common::clause("membership", "required")
    });
    declarations.clauses.push(ClauseRow {
        kind: Some("skill".to_string()),
        degree: Some(DegreeBoundRow {
            incoming: Some(EdgeBoundRow {
                min: Some(1),
                max: None,
            }),
            outgoing: Some(EdgeBoundRow {
                min: None,
                max: Some(3),
            }),
        }),
        ..common::clause("degree", "advisory")
    });

    let payload = golden_payload(declarations);
    let (_harness, into) = emitted("clause-row-args", &payload);
    let lock = into.join("lock.toml");
    let first = fs::read(&lock).unwrap();

    // Double-emit byte stability: re-emitting the same payload reproduces
    // the whole lock byte-for-byte.
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    let second = fs::read(&lock).unwrap();
    assert_eq!(first, second, "a re-emit must not churn the lock");

    let read_back = drift::read_declarations(&into).unwrap();
    let count_row = read_back
        .clauses
        .iter()
        .find(|c| c.predicate == "count")
        .expect("the count clause row round-trips");
    let count = count_row.count.expect("count bound is recorded");
    assert_eq!((count.min, count.max), (1, 3));

    let unique_row = read_back
        .clauses
        .iter()
        .find(|c| c.predicate == "unique")
        .expect("the unique clause row round-trips");
    assert_eq!(unique_row.field.as_deref(), Some("name"));

    let membership_row = read_back
        .clauses
        .iter()
        .find(|c| c.predicate == "membership")
        .expect("the membership clause row round-trips");
    assert_eq!(membership_row.field.as_deref(), Some("model"));
    assert_eq!(membership_row.target.as_deref(), Some("approved-models"));

    let degree_row = read_back
        .clauses
        .iter()
        .find(|c| c.predicate == "degree")
        .expect("the degree clause row round-trips");
    let degree = degree_row.degree.expect("degree bound is recorded");
    assert_eq!(degree.incoming.expect("incoming bound").min, Some(1));
    assert_eq!(degree.incoming.expect("incoming bound").max, None);
    assert_eq!(degree.outgoing.expect("outgoing bound").min, None);
    assert_eq!(degree.outgoing.expect("outgoing bound").max, Some(3));
}

/// A kind's own floor clause row round-trips its **node-scope predicate argument**
/// (`LOCK-CLAUSE-PREDICATE-ARGS`) — `min_len`/`max_len`/`max_lines`'s bound,
/// `allowed_chars`'s charset, `forbidden_keys`'s keys, `deny`'s values — not just
/// identity+severity, so a floor `Contract` is reconstructable from the rows alone.
#[test]
fn a_floor_clause_row_round_trips_its_node_scope_predicate_argument() {
    let mut declarations = rich_declarations();
    declarations.clauses.push(ClauseRow {
        kind: Some("skill".to_string()),
        field: Some("name".to_string()),
        bound: Some(BoundRow {
            min: None,
            max: Some(64),
        }),
        ..common::clause("max_len", "required")
    });
    declarations.clauses.push(ClauseRow {
        kind: Some("skill".to_string()),
        keys: Some(vec!["globs".to_string(), "alwaysApply".to_string()]),
        ..common::clause("forbidden_keys", "required")
    });
    declarations.clauses.push(ClauseRow {
        kind: Some("skill".to_string()),
        field: Some("name".to_string()),
        charset: Some(CharsetRow {
            ranges: vec!["a-z".to_string(), "0-9".to_string()],
            chars: Some("-".to_string()),
        }),
        ..common::clause("allowed_chars", "required")
    });

    let payload = golden_payload(declarations);
    let (_harness, into) = emitted("floor-clause-args", &payload);
    let read_back = drift::read_declarations(&into).unwrap();

    let max_len_row = read_back
        .clauses
        .iter()
        .find(|c| c.predicate == "max_len" && c.field.as_deref() == Some("name"))
        .expect("the max_len clause row round-trips");
    let bound = max_len_row.bound.expect("the bound is recorded");
    assert_eq!((bound.min, bound.max), (None, Some(64)));

    let forbidden_keys_row = read_back
        .clauses
        .iter()
        .find(|c| c.predicate == "forbidden_keys")
        .expect("the forbidden_keys clause row round-trips");
    assert_eq!(
        forbidden_keys_row.keys.as_deref(),
        Some(["globs".to_string(), "alwaysApply".to_string()].as_slice())
    );

    let allowed_chars_row = read_back
        .clauses
        .iter()
        .find(|c| c.predicate == "allowed_chars")
        .expect("the allowed_chars clause row round-trips");
    let charset = allowed_chars_row
        .charset
        .as_ref()
        .expect("the charset is recorded");
    assert_eq!(charset.ranges, vec!["a-z".to_string(), "0-9".to_string()]);
    assert_eq!(charset.chars.as_deref(), Some("-"));
}

/// A payload with no requirements/satisfies/assembly facts at all still emits and
/// round-trips: those families are simply empty, never an error or a malformed row —
/// the bootstrap's tolerant-read discipline extends to the new facets exactly as it
/// does the existing ones.
#[test]
fn a_bare_harness_lock_still_round_trips() {
    let payload = golden_payload(Declarations {
        kinds: vec![
            common::rule_kind_facts(Some("claude-code"), &["paths-match(paths)"]),
            common::skill_kind_facts(
                Some("claude-code"),
                &["user-invoked", "description-trigger(description)"],
            ),
        ],
        clauses: rich_declarations().clauses,
        ..Declarations::default()
    });
    let (_harness, into) = emitted("bare", &payload);

    let declarations = drift::read_declarations(&into).unwrap();
    assert!(!declarations.kinds.is_empty());
    assert!(!declarations.clauses.is_empty());
    assert!(declarations.requirements.is_empty());
    assert!(declarations.satisfies.is_empty());
    assert!(declarations.mentions.is_empty());
}

/// A host kind's declared nesting templates — the embedded child kind names it
/// folds — round-trip through the lock's `kind`
/// row unchanged, and a template-less kind (`rule`, `skill` here) still round-trips
/// with no `templates` column at all (the empty-array-vanishes tolerance the rest of
/// the declaration-row family already carries).
#[test]
fn a_host_kinds_declared_templates_round_trip_through_the_lock() {
    let payload = golden_payload(Declarations {
        kinds: vec![
            common::rule_kind_facts(Some("claude-code"), &["paths-match(paths)"]),
            common::skill_kind_facts(
                Some("claude-code"),
                &["user-invoked", "description-trigger(description)"],
            ),
            spec_kind_facts_with_template(),
        ],
        clauses: rich_declarations().clauses,
        ..Declarations::default()
    });
    let (_harness, into) = emitted("nesting-templates", &payload);
    let declarations = drift::read_declarations(&into).unwrap();

    let spec = declarations
        .kinds
        .iter()
        .find(|k| k.name == "spec")
        .expect("the templated kind fact is recorded");
    assert_eq!(spec.templates, vec!["decision".to_string()]);

    let rule = declarations
        .kinds
        .iter()
        .find(|k| k.name == "rule")
        .expect("the template-less kind fact is recorded");
    assert!(
        rule.templates.is_empty(),
        "a kind declaring no templates round-trips with an empty templates column"
    );
}

/// A kind's `layout`-content fact round-trips SDK emit → lock kind row → engine
/// `CustomKind` (`KIND-CONTENT-FACT`): the declared regions survive the lock byte-stably
/// and reach `CustomKind::from_kind_fact_row`'s `content`, while a kind declaring no
/// content reads as `Content::File` everywhere — the column absent from its row.
#[test]
fn a_kinds_layout_content_round_trips_the_lock_and_reaches_the_engine_custom_kind() {
    let payload = golden_payload(Declarations {
        kinds: vec![
            common::rule_kind_facts(Some("claude-code"), &["paths-match(paths)"]),
            common::skill_kind_facts(
                Some("claude-code"),
                &["user-invoked", "description-trigger(description)"],
            ),
            spec_kind_facts_with_layout(),
        ],
        clauses: rich_declarations().clauses,
        ..Declarations::default()
    });
    let (_harness, into) = emitted("layout-content", &payload);
    let lock = into.join("lock.toml");
    let first = fs::read(&lock).unwrap();

    // Double-emit byte stability: the content column is a pure function of the payload.
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(
        first,
        fs::read(&lock).unwrap(),
        "a re-emit must not churn the lock"
    );

    let declarations = drift::read_declarations(&into).unwrap();

    // The layout kind's declared content reaches the engine's CustomKind through the row.
    let spec_row = declarations
        .kinds
        .iter()
        .find(|k| k.name == "spec")
        .expect("the layout kind row is recorded");
    let spec = CustomKind::from_kind_fact_row(spec_row);
    assert_eq!(
        spec.content,
        Content::Layout(Layout {
            regions: vec![
                LayoutRegion::Prose {
                    import: Some("specs/intent.md".to_string()),
                },
                LayoutRegion::Field {
                    slot: "intent".to_string(),
                },
                LayoutRegion::Collection {
                    member_kind: "invariant".to_string(),
                    key: Some("core".to_string()),
                },
            ],
        }),
    );

    // A kind with no content declaration is file everywhere — the column absent from the
    // row, `Content::File` in the reconstructed CustomKind.
    let rule_row = declarations
        .kinds
        .iter()
        .find(|k| k.name == "rule")
        .expect("the file-content kind row is recorded");
    assert!(
        rule_row.content.is_none(),
        "a file-content kind's row omits the content column"
    );
    assert_eq!(
        CustomKind::from_kind_fact_row(rule_row).content,
        Content::File
    );
}

/// A host member's declared embedded-member value's row — the shape a `blocks()`
/// value like `tests/nested_member.rs`'s `decision_body` composes: leaves plus one
/// collection's entries, authored out of alphabetical order.
fn nested_member_row() -> NestedMemberRow {
    let leaves = BTreeMap::from([(
        "chosen".to_string(),
        "the composition surface is canonical".to_string(),
    )]);
    let collections = vec![
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
    ];
    NestedMemberRow {
        host: "memory:CLAUDE".to_string(),
        kind: "decision".to_string(),
        key: "surface-authority".to_string(),
        leaves,
        collections,
    }
}

/// A declared embedded member's facts round-trip through the lock as
/// `[[declaration.nested_member]]` rows (`NESTED-MEMBER-LOCK-ROW`), the same way
/// `a_host_kinds_declared_templates_round_trip_through_the_lock` above proves for the
/// templates row — additive: the fold-based read side (`tests/nested_member.rs`)
/// never reads this family, only `emit`/`read_declarations` round-trip it.
#[test]
fn a_declared_embedded_members_facts_round_trip_through_the_lock_as_nested_member_rows() {
    let payload = golden_payload(Declarations {
        kinds: vec![
            common::rule_kind_facts(Some("claude-code"), &["paths-match(paths)"]),
            common::skill_kind_facts(
                Some("claude-code"),
                &["user-invoked", "description-trigger(description)"],
            ),
        ],
        clauses: rich_declarations().clauses,
        nested_members: vec![nested_member_row()],
        ..Declarations::default()
    });
    let (_harness, into) = emitted("nested-member-row", &payload);
    let lock = into.join("lock.toml");
    let first = fs::read(&lock).unwrap();

    // Double-emit byte stability: re-emitting the same payload reproduces the whole
    // lock byte-for-byte.
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    let second = fs::read(&lock).unwrap();
    assert_eq!(first, second, "a re-emit must not churn the lock");

    let declarations = drift::read_declarations(&into).unwrap();
    let row = declarations
        .nested_members
        .iter()
        .find(|row| row.host == "memory:CLAUDE")
        .expect("the nested-member row is recorded");
    assert_eq!(row.kind, "decision");
    assert_eq!(row.key, "surface-authority");
    assert_eq!(
        row.leaves.get("chosen").map(String::as_str),
        Some("the composition surface is canonical")
    );
    // Authored out of alphabetical order (`read-only-lens` before `baked-projection`)
    // — the round trip through the lock preserves that authored order verbatim.
    assert_eq!(
        row.collections
            .iter()
            .map(|entry| entry.key.as_str())
            .collect::<Vec<_>>(),
        vec!["read-only-lens", "baked-projection"],
    );
    let entry = row
        .collections
        .iter()
        .find(|entry| entry.collection == "rejected" && entry.key == "baked-projection")
        .expect("the collection entry round-trips");
    assert_eq!(
        entry.leaves.get("because").map(String::as_str),
        Some("a stamping projector breaks law 5")
    );
}

/// A workspace with no `[declaration]` table (any pre-recut lock) reads back an empty
/// declaration set rather than erroring — absent evidence forges no finding.
#[test]
fn a_lock_without_declarations_reads_empty() {
    let dir = common::tmpdir("no-declarations");
    fs::write(
        dir.join("lock.toml"),
        "[[skill]]\nname = \"x\"\nsource_path = \"/h/SKILL.md\"\nsource_hash = \"abc\"\nemit_hash = \"abc\"\n",
 )
.unwrap();

    let declarations = drift::read_declarations(&dir).unwrap();
    assert_eq!(declarations, drift::Declarations::default());
}

/// A missing lock is the pre-import state, not an error.
#[test]
fn a_missing_lock_reads_empty() {
    let dir: &Path = &common::tmpdir("missing-lock");
    let declarations = drift::read_declarations(dir).unwrap();
    assert_eq!(declarations, drift::Declarations::default());
}

// ---- check resolves members via the lock's governs locus --------------------
//
// The lock: the gate
// walks each kind's `governs` locus off the committed lock's own kind-fact row, read
// straight off the harness disk — never a copied surface tree — and a harness with no
// lock at all is still gated by the embedded default program's own locus (the built-in
// lock), never a silent zero-member skip.

/// A forbidden-key skill under `<root>/.claude/skills/coordinate/SKILL.md`.
const GOVERNS_WALK_SKILL: &str = "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
globs: \"**/*.rs\"\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// Run `temper check --reporter github` from `root`, returning `(exit success, combined
/// output)` — the shape this file's assertions destructure.
fn check_in(root: &Path) -> (bool, String) {
    let run = common::check_in(root, &[], Some("github"));
    (run.ok, run.output)
}

#[test]
fn check_walks_the_locks_declared_governs_locus_not_the_kinds_embedded_default() {
    // Prove the walk is driven by the lock's own kind-fact row, not `skill`'s embedded
    // `.claude/skills` default: point the lock's `skill` governs at a nonstandard
    // locus, place the member only there, and confirm `check` finds and judges it.
    let root = common::tmpdir("custom-governs-locus");
    let skill = root.join("custom-locus").join("skills").join("coordinate");
    fs::create_dir_all(&skill).unwrap();
    fs::write(skill.join("SKILL.md"), GOVERNS_WALK_SKILL).unwrap();

    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(
        temper_dir.join("lock.toml"),
        "[[declaration.kind]]\n\
         name = \"skill\"\n\
         provider = \"claude-code\"\n\
         governs_root = \"custom-locus/skills\"\n\
         governs_glob = \"*/SKILL.md\"\n",
    )
    .unwrap();

    let (ok, output) = check_in(&root);
    assert!(
        !ok,
        "the member at the lock's declared locus must be found and fire, got:\n{output}"
    );
    assert!(
        output.contains("forbidden_keys"),
        "the finding names the clause the relocated member tripped, got:\n{output}"
    );
}

/// A `rule` member whose projected body carries one `member.directive` fence keyed
/// `rendered-key` — inert prose under 0018 (the projection is write-only), kept here
/// only to prove `explain` never re-reads it for facts: the lock's own
/// `nested_member` row below declares the same child kind under a *different* key
/// (`at-import`), so a fold-through-fence read and a lock-row read would disagree.
const DIRECTIVE_TEMPLATED_RULE: &str = "# Rule using a nested directive\n\
\n\
Some prose.\n\
\n\
```member.directive rendered-key\n\
target = \"some/path.md\"\n\
```\n";

#[test]
fn a_lock_declared_nested_member_row_folds_a_builtin_hosts_embedded_member() {
    // NESTED-MEMBER-LOCK-ROW / RETIRE-FOLD-MEMBERS: a lock row naming a built-in
    // (`rule`) and declaring `templates` legitimately extends that built-in's host
    // with a child kind (`row_relocates_builtin`'s own doc comment already names a
    // declared, non-empty `templates` a legitimate extension, never a collision) —
    // but the member's embedded facts come from its own `[[declaration.nested_member]]`
    // row, addressed by `kind:name`, never by re-parsing the rule's rendered fence
    // (0018, "the projection is not the database"). The row below names a *different*
    // key than the rendered fence does, so `explain` narrating the row's key alone
    // proves the fence is never re-read.
    let root = common::tmpdir("nested-member-row-overlay");
    let rules = root.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(rules.join("uses-directive.md"), DIRECTIVE_TEMPLATED_RULE).unwrap();

    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(
        temper_dir.join("lock.toml"),
        "[[declaration.kind]]\n\
         name = \"rule\"\n\
         provider = \"claude-code\"\n\
         governs_root = \".claude/rules\"\n\
         governs_glob = \"*.md\"\n\
         templates = [\"directive\"]\n\
         \n\
         [[declaration.nested_member]]\n\
         host = \"rule:uses-directive\"\n\
         kind = \"directive\"\n\
         key = \"at-import\"\n\
         leaves = { target = \"declared/not-rendered.md\" }\n",
    )
    .unwrap();

    let out = explain_in(&root, "uses-directive");
    assert!(
        out.contains("Nested members (the embedded members it carries):"),
        "the lock's declared nested-member row must surface as a visible nested \
         member instead of leaving it dark, got:\n{out}"
    );
    assert!(
        out.contains("`directive` member `at-import`"),
        "the folded nested member must name the row's own declared key, got:\n{out}"
    );
    assert!(
        !out.contains("rendered-key"),
        "the rendered fence's key must never surface — nothing re-reads it, got:\n{out}"
    );
}

#[test]
fn a_harness_with_no_lock_is_gated_by_the_built_in_lock() {
    // No `.temper/lock.toml` at all (never imported): `declarations.kinds` is empty, so
    // `check` falls back to the embedded default program's own `governs` locus (the
    // built-in lock) to walk the harness — a forbidden-key skill must still fire,
    // never a silent zero-member skip.
    let root = common::tmpdir("no-lock-builtin-fallback");
    let skill = root.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(&skill).unwrap();
    fs::write(skill.join("SKILL.md"), GOVERNS_WALK_SKILL).unwrap();

    let (ok, output) = check_in(&root);
    assert!(
        !ok,
        "a forbidden-key skill must still fire with no committed lock at all, got:\n{output}"
    );
    assert!(
        output.contains("forbidden_keys"),
        "the finding names the clause the harness member tripped even with no lock, got:\n{output}"
    );
}

// ---- SATISFIER-KIND-CLAUSE: a requirement row's `kind` sources a clause -------
//
// Selection: a `RequirementRow`'s `kind` column is a
// declaration row in the lock, and it now *sources* the shipped each-grain "every
// satisfier is kind K" clause rather than narrowing which opt-in artifacts are
// candidates — a wrong-kind opt-in is a `requirement.kind` finding, never a silent
// exclusion.

#[test]
fn a_requirement_rows_kind_sources_the_each_grain_kind_clause() {
    // `gate`'s declaration row in the lock narrows to `skill`. A skill opts in
    // cleanly; a rule also opts in — the kind-blind satisfier set draws it in, and
    // the each-grain clause the row's `kind` column sources flags it as a
    // `requirement.kind` finding rather than silently excluding it.
    let root = common::tmpdir("kind-clause-sources-from-row");
    let skill_dir = root.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        "---\n\
 name: coordinate\n\
 description: Use when coordinating agents across axes; not for single-axis work.\n\
 ---\n\
 # Coordinate\n\
 \n\
         Body.\n",
    )
    .unwrap();
    let rules_dir = root.join(".claude").join("rules");
    fs::create_dir_all(&rules_dir).unwrap();
    fs::write(rules_dir.join("style.md"), "# Style\n\nBody.\n").unwrap();

    common::write_lock(
        &root,
        Declarations {
            requirements: vec![common::requirement("gate", false, Some("skill"))],
            ..Declarations::default()
        },
    );
    common::author_satisfies(&root, "skills", "coordinate", &["gate"]);
    common::author_satisfies(&root, "rules", "style", &["gate"]);

    let (ok, output) = check_in(&root);
    assert!(
        !ok,
        "a wrong-kind opt-in the row's `kind` narrows against must fail the run ⇒ non-zero, got:\n{output}"
    );
    assert!(
        output.contains("requirement.kind") && output.contains("style"),
        "the finding names the sourced kind clause and the wrong-kind satisfier, got:\n{output}"
    );
}

// ---- BUILTIN-LOCK-DERIVED: the embedded built-in lock ------------------------
//
// Decision: the built-in lock is derived
// from the SDK module, never transcribed: `src/builtin_lock.toml` is the real
// `[declaration.*]` family a memberless emit of `@dtmd/temper/claude-code`'s built-in
// kinds + four floors produces, and `temper::builtin` projects each kind's floor
// `Contract` straight off this lock's clause rows — no hand-written mirror any
// more. These tests pin that projection against the lock's own rows, proving
// `builtin::contract` round-trips every row's predicate/field/severity losslessly.
// `builtin_kind`'s kind facts stay a separate hand-written mirror, untouched here.

/// The declared `(predicate, field, severity)` triples a built-in floor's clauses
/// carry, in declaration order — the shape a `ClauseRow` reduces a `Clause` to
/// (`temper::drift::ClauseRow`; `Predicate::key`/`Predicate::target`).
fn floor_triples(kind: &str) -> Vec<(&'static str, Option<String>, &'static str)> {
    let contract = builtin::contract(kind)
        .unwrap_or_else(|| panic!("built-in kind `{kind}` ships an embedded floor"));
    contract
        .clauses
        .into_iter()
        .map(|clause| {
            let severity = match clause.severity {
                Severity::Required => "required",
                Severity::Advisory => "advisory",
            };
            (
                clause.predicate.key(),
                clause.predicate.target().map(str::to_string),
                severity,
            )
        })
        .collect()
}

/// The embedded built-in lock's own `(predicate, field, severity)` triples for one
/// kind, in the row order the lock carries them.
fn lock_triples(kind: &str) -> Vec<(&'static str, Option<String>, &'static str)> {
    builtin_lock::declarations()
        .clauses
        .iter()
        .filter(|row| row.kind.as_deref() == Some(kind))
        .map(|row| {
            (
                row.predicate.as_str(),
                row.field.clone(),
                row.severity.as_str(),
            )
        })
        .collect()
}

#[test]
fn the_embedded_lock_kind_facts_match_todays_hand_written_kinds() {
    let declarations = builtin_lock::declarations();

    let skill = declarations
        .kinds
        .iter()
        .find(|k| k.name == "skill")
        .expect("the skill kind fact is embedded");
    assert_eq!(skill.governs_root, ".claude/skills");
    assert_eq!(skill.governs_glob, "*/SKILL.md");
    assert_eq!(skill.format.as_deref(), Some("yaml-frontmatter"));
    assert_eq!(skill.unit_shape.as_deref(), Some("directory"));
    assert_eq!(
        skill.registration,
        vec![
            "user-invoked".to_string(),
            "description-trigger(description)".to_string()
        ]
    );

    let rule = declarations
        .kinds
        .iter()
        .find(|k| k.name == "rule")
        .expect("the rule kind fact is embedded");
    assert_eq!(rule.governs_root, ".claude/rules");
    assert_eq!(rule.governs_glob, "*.md");
    assert_eq!(rule.format.as_deref(), Some("yaml-frontmatter"));
    assert_eq!(rule.unit_shape.as_deref(), Some("file"));
    assert_eq!(rule.registration, vec!["paths-match(paths)".to_string()]);

    let memory = declarations
        .kinds
        .iter()
        .find(|k| k.name == "memory")
        .expect("the memory kind fact is embedded");
    assert_eq!(memory.governs_root, ".");
    assert_eq!(memory.governs_glob, "**/CLAUDE.md");
    assert_eq!(memory.format, None);
    assert_eq!(memory.unit_shape.as_deref(), Some("file"));
    assert_eq!(memory.registration, vec!["always".to_string()]);

    let command = declarations
        .kinds
        .iter()
        .find(|k| k.name == "command")
        .expect("the command kind fact is embedded");
    assert_eq!(command.governs_root, ".claude/commands");
    assert_eq!(command.governs_glob, "*.md");
    assert_eq!(command.format.as_deref(), Some("yaml-frontmatter"));
    assert_eq!(command.unit_shape.as_deref(), Some("file"));
    assert_eq!(
        command.registration,
        vec![
            "user-invoked".to_string(),
            "description-trigger(description)".to_string()
        ]
    );

    let agent = declarations
        .kinds
        .iter()
        .find(|k| k.name == "agent")
        .expect("the agent kind fact is embedded");
    assert_eq!(agent.governs_root, ".claude/agents");
    assert_eq!(agent.governs_glob, "**/*.md");
    assert_eq!(agent.format.as_deref(), Some("yaml-frontmatter"));
    // Named-field identity — the third mode, wire-spelled `named-field(<field>)`.
    assert_eq!(agent.unit_shape.as_deref(), Some("named-field(name)"));
    assert_eq!(
        agent.registration,
        vec!["description-trigger(description)".to_string()]
    );

    // The SDK module sets no `provider` on any of its five exported kinds yet, so
    // the derived rows carry none either — a real gap `BUILTIN-LOCK-ROW-DRIVEN`
    // reconciles (`(builtin-workspace-qualified-key)`), not this link.
    assert!(declarations.kinds.iter().all(|row| row.provider.is_none()));
    assert_eq!(declarations.kinds.len(), 5);
    assert!(declarations.requirements.is_empty());
    assert!(declarations.satisfies.is_empty());
    assert!(declarations.mentions.is_empty());
}

#[test]
fn the_embedded_lock_clauses_match_todays_hand_written_floors_per_kind() {
    assert_eq!(
        lock_triples("skill"),
        floor_triples("skill"),
        "skill's floor clauses round-trip through the derived lock unchanged"
    );
    assert_eq!(
        lock_triples("rule"),
        floor_triples("rule"),
        "rule's floor clauses round-trip through the derived lock unchanged"
    );
    // The memberless emit binds both memory default contracts to the SDK's one exported
    // `memory` kind; `memoryAgentsMdDefaultContract` is guidance-only (zero clauses), so only
    // `memoryAnthropicDefaultContract`'s clause survives under the `memory` kind's rows.
    assert_eq!(
        lock_triples("memory"),
        floor_triples("memory"),
        "memory's floor clauses round-trip through the derived lock unchanged"
    );
    assert_eq!(
        lock_triples("command"),
        floor_triples("command"),
        "command's floor clauses round-trip through the derived lock unchanged"
    );
    assert_eq!(
        lock_triples("agent"),
        floor_triples("agent"),
        "agent's floor clauses round-trip through the derived lock unchanged"
    );
}

/// A built-in clause row carries its module floor's guidance and cite through the
/// derived lock (`LOCK-CLAUSE-CHANNELS`): the seam (`sdk/src/declarations.ts`
/// `clauseRow`) and `drift::ClauseRow` used to drop both channels, stranding the
/// gate's teaching prose on the wrong side of the erasure. Skill's `max_lines`
/// advisory is the worked example: its progressive-disclosure guidance and
/// agentskills.io cite (`sdk/src/builtins.ts` `skillDefaultContract`) must reach the embedded
/// lock's row, and `builtin::contract`'s projection, unchanged.
#[test]
fn the_embedded_lock_clause_row_carries_the_floors_guidance_and_cite() {
    let contract = builtin::contract("skill").expect("skill's built-in floor is embedded");
    let floor_clause = contract
        .clauses
        .iter()
        .find(|clause| clause.predicate.key() == "max_lines")
        .expect("skill's floor carries a max_lines clause");
    let expected_guidance = floor_clause
        .guidance
        .as_deref()
        .expect("the projected floor's max_lines clause carries guidance");
    let expected_cite = floor_clause
        .source
        .as_deref()
        .expect("the projected floor's max_lines clause carries a cite");
    assert!(
        expected_guidance.contains("Progressive disclosure"),
        "skill's max_lines advisory carries its progressive-disclosure guidance, got {expected_guidance:?}"
    );
    assert!(
        expected_cite.contains("agentskills.io"),
        "skill's max_lines advisory cites the agentskills spec, got {expected_cite:?}"
    );

    let row = builtin_lock::declarations()
        .clauses
        .iter()
        .find(|row| row.kind.as_deref() == Some("skill") && row.predicate == "max_lines")
        .expect("the derived lock carries skill's max_lines clause row");

    assert_eq!(row.guidance.as_deref(), Some(expected_guidance));
    assert_eq!(row.cite.as_deref(), Some(expected_cite));
}

// ---- MENTION-EDGE-LANDS: an authored mention binds the graph -----------------
//
// contract.md, "edge": a mention is one of four edge loci, and every edge resolves
// into the one enumeration the gate and every read verb share. These prove the whole
// pipeline past the lock round-trip already proven above: the mention row binds into
// the reference graph, a `degree` clause can count it, and `explain` narrates its
// resolved target rather than "points at no member" — with no declared reference
// field between the two members at all.

/// A floor-clean skill named `name` whose prose cites `target` in words alone (no
/// declared reference field) — the mention is the only edge this fixture carries.
fn mentioning_skill(name: &str, target: &str) -> String {
    format!(
        "---\n\
         name: {name}\n\
         description: Use when {name} is the task at hand; not for anything else.\n\
         ---\n\
         # {name}\n\
         \n\
         See the {target} rule.\n"
    )
}

/// A floor-clean rule with a plain body and no frontmatter at all — the mention's
/// target, declaring no reference field of its own.
fn clean_rule(name: &str) -> String {
    format!("# {name}\n\nBody.\n")
}

/// Run `temper explain <target>` from `root`, returning its stdout narration.
fn explain_in(root: &Path, target: &str) -> String {
    let out = Command::new(BIN)
        .current_dir(root)
        .arg("explain")
        .arg(target)
        .output()
        .unwrap();
    String::from_utf8_lossy(&out.stdout).into_owned()
}

/// The `gate` requirement's declaration row, typed to `rule`, carrying a required
/// `degree` clause bounding incoming edges to at least one.
fn incoming_degree_requirement() -> RequirementRow {
    RequirementRow {
        clauses: vec![common::required_clause_row(
            "degree",
            None,
            None,
            None,
            Some(DegreeBoundRow {
                incoming: Some(EdgeBoundRow {
                    min: Some(1),
                    max: None,
                }),
                outgoing: None,
            }),
        )],
        ..common::requirement("gate", false, Some("rule"))
    }
}

#[test]
fn a_mention_binds_the_graph_so_degree_counts_it_and_explain_narrates_it() {
    let root = common::tmpdir("mention-edge-lands");
    // A skill `coordinate` and a rule `rust`, on disk, declaring no reference field
    // between them at all — the only edge is the skill's authored mention of the rule.
    let skill_dir = root.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        mentioning_skill("coordinate", "rust"),
    )
    .unwrap();
    let rules_dir = root.join(".claude").join("rules");
    fs::create_dir_all(&rules_dir).unwrap();
    fs::write(rules_dir.join("rust.md"), clean_rule("rust")).unwrap();

    // The rule `rust` opts into `gate`, whose required `degree` clause bounds its
    // incoming edges to at least one — satisfiable only by the mention, since no
    // reference field is declared anywhere in this harness.
    common::write_lock(
        &root,
        Declarations {
            requirements: vec![incoming_degree_requirement()],
            mentions: vec![MentionRow {
                member: "skill:coordinate".to_string(),
                target: "rule:rust".to_string(),
            }],
            ..Declarations::default()
        },
    );
    // `why`'s member listing reads the lock's `satisfies` rows, not raw harness
    // disk — author one for `coordinate` too (no `satisfies` claims of its own) so
    // `explain` resolves it as a member at all.
    common::author_satisfies(&root, "skills", "coordinate", &[]);
    common::author_satisfies(&root, "rules", "rust", &["gate"]);

    let (ok, output) = check_in(&root);
    assert!(
        ok,
        "the mention alone satisfies the rule's incoming degree bound ⇒ clean, got:\n{output}"
    );

    let out = explain_in(&root, "coordinate");
    assert!(
        out.contains("it points at `rust` (rule) via its `mention` field"),
        "explain narrates the mention's resolved target rather than \"points at no member\": {out}"
    );
    assert!(
        !out.contains("it points at no member"),
        "a member whose only outgoing edge is a mention must not read as pointing at nothing: {out}"
    );
}

#[test]
fn a_mention_with_no_clause_ranging_over_it_is_obligation_free() {
    // No `degree` clause at all: the mention rides the lock and binds the graph, but
    // no shipped clause counts it — obligation-free by default (contract.md, "edge").
    let root = common::tmpdir("mention-obligation-free");
    let skill_dir = root.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        mentioning_skill("coordinate", "rust"),
    )
    .unwrap();
    let rules_dir = root.join(".claude").join("rules");
    fs::create_dir_all(&rules_dir).unwrap();
    fs::write(rules_dir.join("rust.md"), clean_rule("rust")).unwrap();

    common::write_lock(
        &root,
        Declarations {
            mentions: vec![MentionRow {
                member: "skill:coordinate".to_string(),
                target: "rule:rust".to_string(),
            }],
            ..Declarations::default()
        },
    );

    let (ok, output) = check_in(&root);
    assert!(
        ok,
        "a mention with no clause ranging over it never gates ⇒ clean, got:\n{output}"
    );
}
