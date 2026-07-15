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
use temper::contract::{self, Clause, Contract, Predicate, Severity};
use temper::drift::{
    self, AssemblyFactRow, BoundRow, CharsetRow, ClauseRow, CollectionAddressRow,
    CollectionEntryRow, CountBoundRow, Declarations, DegreeBoundRow, EdgeBoundRow, EmitOptions,
    KindFactRow, LayoutRegionRow, LayoutRow, MentionRow, NestedMemberRow, Payload, PayloadMember,
    RangeBoundRow, RegistrationRow, RequirementRow, SatisfiesRow, SectionContainsRow,
};
use temper::engine;
use temper::extract::Features;
use temper::kind::{
    CollectionAddress, CollectionKeyPath, Content, CustomKind, Extraction, Layout, LayoutRegion,
    Primitive,
};

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

/// A `hook` kind declaring the two manifest-authoring facts 0021 phase 1 adds: the
/// fields-only body shape (no body slot) and the collection address it registers at
/// (`settings.json`'s `hooks.<Event>`). The shape an SDK-declared registration kind's row
/// carries into the lock.
fn hook_kind_facts() -> KindFactRow {
    KindFactRow {
        shape: Some("fields".to_string()),
        collection_address: Some(CollectionAddressRow {
            manifest: "settings.json".to_string(),
            key_path: "hooks.<Event>".to_string(),
        }),
        ..common::kind_facts("hook", ".claude", "settings.json")
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
        includes: Vec::new(),
        nested_members: Vec::new(),
        registrations: Vec::new(),
        settings: Vec::new(),
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

/// The `range` (f64 bounds) and `section_contains` (heading+marker) clause rows
/// round-trip their arguments through `to_table`/`from_table` byte-stably — the
/// column homes `PREDICATE-CONSTRUCTORS` adds beside the existing node-scope args
/// (`a_floor_clause_row_round_trips_its_node_scope_predicate_argument`).
#[test]
fn a_range_and_section_clause_row_round_trip_the_lock() {
    let mut declarations = rich_declarations();
    declarations.clauses.push(ClauseRow {
        kind: Some("skill".to_string()),
        field: Some("priority".to_string()),
        range: Some(RangeBoundRow { min: 1.0, max: 5.5 }),
        ..common::clause("range", "advisory")
    });
    declarations.clauses.push(ClauseRow {
        kind: Some("skill".to_string()),
        section: Some(SectionContainsRow {
            heading: "Decision".to_string(),
            marker: "Rejected".to_string(),
        }),
        ..common::clause("section_contains", "required")
    });

    let payload = golden_payload(declarations);
    let (_harness, into) = emitted("range-section-args", &payload);
    let lock = into.join("lock.toml");
    let first = fs::read(&lock).unwrap();

    // Double-emit byte stability: the f64 bound formats stably across the round trip.
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(
        first,
        fs::read(&lock).unwrap(),
        "a re-emit must not churn the lock"
    );

    let read_back = drift::read_declarations(&into).unwrap();
    let range_row = read_back
        .clauses
        .iter()
        .find(|c| c.predicate == "range")
        .expect("the range clause row round-trips");
    let range = range_row.range.expect("the range bound is recorded");
    assert_eq!((range.min, range.max), (1.0, 5.5));
    assert_eq!(range_row.field.as_deref(), Some("priority"));

    let section_row = read_back
        .clauses
        .iter()
        .find(|c| c.predicate == "section_contains")
        .expect("the section_contains clause row round-trips");
    let section = section_row
        .section
        .as_ref()
        .expect("the section args are recorded");
    assert_eq!(section.heading, "Decision");
    assert_eq!(section.marker, "Rejected");
}

/// The five SDK-authorable predicates lift from their lock rows into the typed
/// `Predicate` the engine evaluates (`PREDICATE-CONSTRUCTORS`): the row an SDK
/// constructor emits decodes through `predicate_from_row`, and the lifted
/// `section_contains` fires on a violating member — author → row → `Predicate` →
/// finding.
#[test]
fn the_five_sdk_authorable_predicate_rows_lift_and_evaluate() {
    let optional = ClauseRow {
        field: Some("model".to_string()),
        ..common::clause("optional", "required")
    };
    assert_eq!(
        contract::predicate_from_row(&optional),
        Some(Predicate::Optional {
            field: "model".to_string()
        })
    );

    let range = ClauseRow {
        field: Some("priority".to_string()),
        range: Some(RangeBoundRow { min: 1.0, max: 5.0 }),
        ..common::clause("range", "required")
    };
    assert_eq!(
        contract::predicate_from_row(&range),
        Some(Predicate::Range {
            field: "priority".to_string(),
            min: 1.0,
            max: 5.0,
        })
    );

    let enumerated = ClauseRow {
        field: Some("status".to_string()),
        values: Some(vec!["draft".to_string(), "final".to_string()]),
        ..common::clause("enum", "required")
    };
    assert_eq!(
        contract::predicate_from_row(&enumerated),
        Some(Predicate::Enum {
            field: "status".to_string(),
            values: vec!["draft".to_string(), "final".to_string()],
        })
    );

    let must_define = ClauseRow {
        field: Some("disable-model-invocation".to_string()),
        ..common::clause("must_define", "required")
    };
    assert_eq!(
        contract::predicate_from_row(&must_define),
        Some(Predicate::MustDefine {
            marker: "disable-model-invocation".to_string()
        })
    );

    let section = ClauseRow {
        section: Some(SectionContainsRow {
            heading: "Decision".to_string(),
            marker: "Rejected".to_string(),
        }),
        ..common::clause("section_contains", "required")
    };
    let predicate = contract::predicate_from_row(&section).expect("section_contains lifts");
    assert_eq!(
        predicate,
        Predicate::SectionContains {
            heading: "Decision".to_string(),
            marker: "Rejected".to_string(),
        }
    );

    // The lifted predicate is a true positive: a `## Decision` section with no
    // `Rejected` marker fires exactly one finding.
    let contract = Contract {
        name: "spec".to_string(),
        guidance: None,
        clauses: vec![Clause {
            severity: Severity::Required,
            predicate,
            guidance: None,
            source: None,
        }],
    };
    let unit = common::raw_unit(
        "10-decisions",
        BTreeMap::new(),
        "# Spec\n\n## Decision: pick the generic engine\n\nChosen it; no alternative named.\n",
        "specs/decisions.md",
    );
    let features: Features = Extraction::new(vec![Primitive::Sections]).extract(&unit);
    let diagnostics = engine::validate(&contract, std::slice::from_ref(&features));
    assert_eq!(diagnostics.len(), 1, "the bare Decision section fires once");
    assert_eq!(diagnostics[0].rule, "section_contains");
}

/// A lock clause row the closed vocabulary cannot admit fails the run loud, never a
/// silently dropped clause (`specs/model/contract.md`, "clause": an unknown predicate
/// is rejected at load; `specs/model/pipeline.md`: the lock is tool-written, never
/// hand-patched). A custom `spec` kind carrying a bogus-predicate clause row must
/// error rather than check clean.
#[test]
fn check_rejects_a_lock_clause_row_the_closed_vocabulary_cannot_admit() {
    let root = common::tmpdir("reject-out-of-vocabulary-clause");
    common::write_lock(
        &root,
        Declarations {
            kinds: vec![common::kind_facts("spec", "specs", "*.md")],
            clauses: vec![ClauseRow {
                kind: Some("spec".to_string()),
                ..common::clause("not_a_predicate", "advisory")
            }],
            ..Declarations::default()
        },
    );

    let (ok, output) = check_in(&root);
    assert!(
        !ok,
        "an out-of-vocabulary clause row must fail the run loud, got:\n{output}"
    );
    assert!(
        output.contains("not_a_predicate"),
        "the load error names the offending predicate rather than dropping it, got:\n{output}"
    );
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
    let spec = CustomKind::from_kind_fact_row(spec_row).unwrap();
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
        CustomKind::from_kind_fact_row(rule_row).unwrap().content,
        Content::File
    );
}

/// A registration kind's two manifest-authoring facts round-trip SDK emit → lock kind row
/// → engine `CustomKind`: the fields-only `shape` lifts to `Content::Fields` and the
/// `collection_address` to the typed `CollectionAddress`, byte-stably across a double
/// emit, while a file-locus body-bearing kind (`rule`) carries neither column — nothing
/// file-locus or body-bearing regresses.
#[test]
fn a_kinds_fields_only_shape_and_collection_address_round_trip_the_lock_and_reach_the_engine() {
    let payload = golden_payload(Declarations {
        kinds: vec![
            common::rule_kind_facts(Some("claude-code"), &["paths-match(paths)"]),
            common::skill_kind_facts(
                Some("claude-code"),
                &["user-invoked", "description-trigger(description)"],
            ),
            hook_kind_facts(),
        ],
        clauses: rich_declarations().clauses,
        ..Declarations::default()
    });
    let (_harness, into) = emitted("collection-address", &payload);
    let lock = into.join("lock.toml");
    let first = fs::read(&lock).unwrap();

    // Double-emit byte stability: the new columns are a pure function of the payload.
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();
    assert_eq!(
        first,
        fs::read(&lock).unwrap(),
        "a re-emit must not churn the lock"
    );

    let declarations = drift::read_declarations(&into).unwrap();

    // The registration kind's facts reach the engine's CustomKind through the row.
    let hook_row = declarations
        .kinds
        .iter()
        .find(|k| k.name == "hook")
        .expect("the registration kind row is recorded");
    assert_eq!(hook_row.shape.as_deref(), Some("fields"));
    let hook = CustomKind::from_kind_fact_row(hook_row).unwrap();
    assert_eq!(hook.content, Content::Fields);
    assert_eq!(
        hook.collection_address,
        Some(CollectionAddress {
            manifest: "settings.json".to_string(),
            key_path: CollectionKeyPath::HooksEvent,
        }),
    );

    // A file-locus, body-bearing kind carries neither new column — both absent from its
    // row, `Content::File` and no collection address in the reconstructed CustomKind.
    let rule_row = declarations
        .kinds
        .iter()
        .find(|k| k.name == "rule")
        .expect("the file-locus kind row is recorded");
    assert!(rule_row.shape.is_none(), "a body-bearing kind omits shape");
    assert!(
        rule_row.collection_address.is_none(),
        "a file-locus kind omits the collection address"
    );
    let rule = CustomKind::from_kind_fact_row(rule_row).unwrap();
    assert_eq!(rule.content, Content::File);
    assert_eq!(rule.collection_address, None);
}

/// An `mcp-server` registration kind fact: fields-only, keyed at `.mcp.json`'s
/// `mcpServers.*`, its file locus the `.mcp.json` at the harness root.
fn mcp_server_kind_facts() -> KindFactRow {
    KindFactRow {
        shape: Some("fields".to_string()),
        collection_address: Some(CollectionAddressRow {
            manifest: ".mcp.json".to_string(),
            key_path: "mcpServers.*".to_string(),
        }),
        ..common::kind_facts("mcp-server", ".", ".mcp.json")
    }
}

/// A represented manifest's registration members cross the seam into the lock's
/// declaration rows AND reach the graph the read family builds — a hook/mcp-server member
/// is governable end to end, not only SDK-erased (MANIFEST-WRITE-SEAM). `emit` writes the
/// `.mcp.json` whole through the write face; the lock records the member as a declaration
/// row (identity + address, its fields the projected artifact); and `explain` resolves the
/// bare server name to the member, narrating it as a node — the read side no longer sees
/// only the kind that addresses the collection.
#[test]
fn a_registration_member_surfaces_in_the_lock_and_reaches_the_read_graph() {
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![
                common::rule_kind_facts(Some("claude-code"), &["paths-match(paths)"]),
                common::skill_kind_facts(
                    Some("claude-code"),
                    &["user-invoked", "description-trigger(description)"],
                ),
                mcp_server_kind_facts(),
            ],
            clauses: rich_declarations().clauses,
            registrations: vec![RegistrationRow {
                kind: "mcp-server".to_string(),
                key: "gmail".to_string(),
                manifest: ".mcp.json".to_string(),
                key_path: "mcpServers.*".to_string(),
                fields: vec![
                    ("type".to_string(), serde_json::json!("stdio")),
                    ("command".to_string(), serde_json::json!("npx")),
                ],
            }],
            ..Declarations::default()
        },
        members: skill_and_rule_members(),
    };
    let (harness, into) = emitted("registration-governable", &payload);

    // The lock records the registration member as a declaration row; `read_declarations`
    // reads it back with its identity and address, the fields left to the projected
    // manifest artifact (0018).
    let declarations = drift::read_declarations(&into).unwrap();
    let server = declarations
        .registrations
        .iter()
        .find(|row| row.key == "gmail")
        .expect("the registration member is recorded as a declaration row");
    assert_eq!(server.kind, "mcp-server");
    assert_eq!(server.manifest, ".mcp.json");
    assert_eq!(server.key_path, "mcpServers.*");
    assert!(
        server.fields.is_empty(),
        "the lock row records identity + address; the fields are the projected artifact"
    );

    // `emit` wrote the represented manifest whole at the harness root.
    assert!(
        harness.join(".mcp.json").is_file(),
        "emit wrote the represented manifest through the write face"
    );

    // Governable end to end: the read family resolves the bare `gmail` to the member and
    // narrates it as a node — not only the `mcp-server` kind that addresses the collection.
    let out = explain_in(&harness, "gmail");
    assert!(
        out.contains("Member `gmail` (mcp-server)"),
        "the represented manifest's member is a governable read-graph node, got:\n{out}"
    );
}

/// An out-of-vocabulary collection-address `key_path` label survives the TOML-typed read
/// but is a corrupt lock the kind lift rejects loud — never a silently narrowed address.
#[test]
fn from_kind_fact_row_rejects_an_out_of_vocabulary_collection_key_path() {
    let row = KindFactRow {
        shape: Some("fields".to_string()),
        collection_address: Some(CollectionAddressRow {
            manifest: "settings.json".to_string(),
            key_path: "hooks.everything".to_string(),
        }),
        ..common::kind_facts("hook", ".claude", "settings.json")
    };
    let err = CustomKind::from_kind_fact_row(&row).unwrap_err();
    assert!(
        matches!(&err, drift::LockRowError::Vocabulary { column, value, .. }
            if column == "collection_address" && value == "hooks.everything"),
        "expected an out-of-vocabulary collection key-path reject, got: {err:?}"
    );
}

/// An out-of-vocabulary `shape` marker is likewise a load error — the fields-only marker
/// admits only `fields`, so any other value rejects loud rather than reading a phantom
/// content mode.
#[test]
fn from_kind_fact_row_rejects_an_out_of_vocabulary_shape() {
    let row = KindFactRow {
        shape: Some("bodyless".to_string()),
        ..common::kind_facts("hook", ".claude", "settings.json")
    };
    let err = CustomKind::from_kind_fact_row(&row).unwrap_err();
    assert!(
        matches!(&err, drift::LockRowError::Vocabulary { column, value, .. }
            if column == "shape" && value == "bodyless"),
        "expected an out-of-vocabulary shape reject, got: {err:?}"
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

/// Read a raw `lock.toml` off a fresh workspace — the shape the present-but-malformed
/// cases drive `read_declarations` over directly, no emit round-trip.
fn read_raw(label: &str, lock: &str) -> miette::Result<Declarations> {
    let dir = common::tmpdir(label);
    fs::write(dir.join("lock.toml"), lock).unwrap();
    drift::read_declarations(&dir)
}

/// A present row missing a required column is a load error naming its family — one case
/// per declaration family. The tool-written lock admits no degrade-to-absent read of a
/// malformed row: a dropped `satisfies` row would forge an unfilled-requirement finding,
/// a dropped `kind` row would unmodel members.
#[test]
fn read_declarations_rejects_a_present_row_missing_a_required_column() {
    let cases = [
        (
            "kind",
            "[[declaration.kind]]\ngoverns_root = \"specs\"\ngoverns_glob = \"*.md\"\n",
        ),
        (
            "clause",
            "[[declaration.clause]]\nseverity = \"required\"\n",
        ),
        (
            "requirement",
            "[[declaration.requirement]]\nrequired = true\n",
        ),
        ("assembly", "[[declaration.assembly]]\nvalue = \"block\"\n"),
        (
            "satisfies",
            "[[declaration.satisfies]]\nmember = \"skill:x\"\n",
        ),
        ("mention", "[[declaration.mention]]\nmember = \"skill:x\"\n"),
        (
            "nested_member",
            "[[declaration.nested_member]]\nkind = \"decision\"\nkey = \"k\"\n",
        ),
    ];
    for (family, body) in cases {
        let err = read_raw(&format!("missing-col-{family}"), body).unwrap_err();
        assert!(
            err.to_string().contains(family),
            "the `{family}` family's missing-column load error must name the family, got:\n{err}"
        );
    }
}

/// A present column of the wrong TOML type is a load error naming its family and column —
/// the tool never emits an integer `name`, so a lock carrying one is corruption.
#[test]
fn read_declarations_rejects_a_wrong_typed_column() {
    let err = read_raw(
        "wrong-typed-kind-name",
        "[[declaration.kind]]\nname = 42\ngoverns_root = \"specs\"\ngoverns_glob = \"*.md\"\n",
    )
    .unwrap_err();
    let message = err.to_string();
    assert!(
        message.contains("kind") && message.contains("name"),
        "a wrong-typed column names its family and column, got:\n{message}"
    );
}

/// A malformed element inside a present row's array column fails the whole row — a
/// tolerant row, never a tolerant element.
#[test]
fn read_declarations_rejects_a_malformed_array_element() {
    let err = read_raw(
        "malformed-array-element",
        "[[declaration.clause]]\npredicate = \"unique\"\nseverity = \"required\"\nkeys = [\"a\", 3]\n",
    )
    .unwrap_err();
    assert!(
        err.to_string().contains("clause"),
        "a non-string array element fails the whole `clause` row, got:\n{err}"
    );
}

/// A well-formed row omitting every optional column reads clean — absent optional
/// columns stay legitimate absence; only a *present* malformed row errors.
#[test]
fn read_declarations_reads_a_row_with_absent_optional_columns() {
    let declarations = read_raw(
        "absent-optional-columns",
        "[[declaration.kind]]\nname = \"spec\"\ngoverns_root = \"specs\"\ngoverns_glob = \"*.md\"\n",
    )
    .unwrap();
    assert_eq!(declarations.kinds.len(), 1);
    assert_eq!(declarations.kinds[0].format, None);
    assert!(declarations.kinds[0].registration.is_empty());
}

/// An out-of-vocabulary `format` label survives the TOML-typed read but is a corrupt lock
/// the kind lift rejects loud at `check` — never a silently narrowed kind. The label read
/// tier's mirror of [`check_rejects_a_lock_clause_row_the_closed_vocabulary_cannot_admit`].
#[test]
fn check_rejects_a_lock_kind_row_with_an_out_of_vocabulary_format_label() {
    let root = common::tmpdir("reject-out-of-vocabulary-format");
    common::write_lock(
        &root,
        Declarations {
            kinds: vec![KindFactRow {
                format: Some("xml".to_string()),
                ..common::kind_facts("spec", "specs", "*.md")
            }],
            ..Declarations::default()
        },
    );

    let (ok, output) = check_in(&root);
    assert!(
        !ok,
        "an out-of-vocabulary format label must fail check loud, got:\n{output}"
    );
    assert!(
        output.contains("xml"),
        "the load error names the offending label rather than dropping it, got:\n{output}"
    );
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

/// A bare `satisfies` label (the shape an older engine wrote, before the row carried
/// the filler's `kind:name` address) qualifies against the live corpus where exactly one
/// kind bears the name — the robust read decision 0024 owes a committed lock — but a name
/// two kinds share is the malformed lock the compiled-label identity forbids, refused loud
/// rather than cross-attributed to both members.
#[test]
fn a_bare_satisfies_label_qualifies_where_unambiguous_and_a_cross_kind_collision_is_refused() {
    // Unambiguous: only a skill bears `solo`, so a bare row binds it and the run is clean.
    let unambiguous = common::tmpdir("bare-satisfies-unambiguous");
    common::write_skill(&unambiguous, "solo", &common::clean_skill("solo"));
    common::write_lock(
        &unambiguous,
        Declarations {
            requirements: vec![RequirementRow {
                required: true,
                ..common::requirement("doc", false, Some("skill"))
            }],
            satisfies: vec![SatisfiesRow {
                member: "solo".to_string(),
                requirement: "doc".to_string(),
            }],
            ..Declarations::default()
        },
    );
    let (ok, output) = check_in(&unambiguous);
    assert!(
        ok,
        "a bare satisfies label of an unambiguous member must qualify and fill ⇒ zero, got:\n{output}"
    );

    // Collision: a skill and a rule both named `csharp`, so a bare label neither member
    // can uniquely own is a malformed lock refused loud.
    let collision = common::tmpdir("bare-satisfies-collision");
    common::write_skill(&collision, "csharp", &common::clean_skill("csharp"));
    common::write_rule(&collision, "csharp");
    common::write_lock(
        &collision,
        Declarations {
            requirements: vec![RequirementRow {
                required: true,
                ..common::requirement("doc", false, None)
            }],
            satisfies: vec![SatisfiesRow {
                member: "csharp".to_string(),
                requirement: "doc".to_string(),
            }],
            ..Declarations::default()
        },
    );
    let (ok, output) = check_in(&collision);
    assert!(
        !ok,
        "a bare satisfies label two kinds share must be refused loud ⇒ non-zero, got:\n{output}"
    );
    assert!(
        output.contains("csharp") && output.contains("ambiguous"),
        "the refusal names the ambiguous label, got:\n{output}"
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

    // The `hook` kind is the first fields-only manifest kind: no `format` (it reads a
    // JSON manifest, not frontmatter), a `fields` shape, and the `hooks.<Event>`
    // collection address inside `settings.json`.
    let hook = declarations
        .kinds
        .iter()
        .find(|k| k.name == "hook")
        .expect("the hook kind fact is embedded");
    assert_eq!(hook.governs_root, ".claude");
    assert_eq!(hook.governs_glob, "settings.json");
    assert_eq!(hook.format, None);
    assert_eq!(hook.unit_shape.as_deref(), Some("file"));
    assert_eq!(hook.registration, vec!["event(event)".to_string()]);
    assert_eq!(hook.shape.as_deref(), Some("fields"));
    let address = hook
        .collection_address
        .as_ref()
        .expect("the hook kind carries its collection address");
    assert_eq!(address.manifest, "settings.json");
    assert_eq!(address.key_path, "hooks.<Event>");

    // The `mcp-server` kind is the second manifest kind: no `format`, a `fields` shape, the
    // `connection` channel, and the `mcpServers.*` collection address inside `.mcp.json`.
    let mcp = declarations
        .kinds
        .iter()
        .find(|k| k.name == "mcp-server")
        .expect("the mcp-server kind fact is embedded");
    assert_eq!(mcp.governs_root, ".");
    assert_eq!(mcp.governs_glob, ".mcp.json");
    assert_eq!(mcp.format, None);
    assert_eq!(mcp.registration, vec!["connection".to_string()]);
    assert_eq!(mcp.shape.as_deref(), Some("fields"));
    let mcp_address = mcp
        .collection_address
        .as_ref()
        .expect("the mcp-server kind carries its collection address");
    assert_eq!(mcp_address.manifest, ".mcp.json");
    assert_eq!(mcp_address.key_path, "mcpServers.*");

    // The SDK module sets no `provider` on any of its seven exported kinds yet, so
    // the derived rows carry none either — a real gap `BUILTIN-LOCK-ROW-DRIVEN`
    // reconciles (`(builtin-workspace-qualified-key)`), not this link.
    assert!(declarations.kinds.iter().all(|row| row.provider.is_none()));
    assert_eq!(declarations.kinds.len(), 7);
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
    // The hook floor is a single `enum` clause over the lifecycle event — the strictest
    // documented profile of a fields-only registration member.
    assert_eq!(
        lock_triples("hook"),
        floor_triples("hook"),
        "hook's floor clauses round-trip through the derived lock unchanged"
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
