//! Proofs over the unified read verb, `explain`: target-species resolution (member / requirement / leaf
//! address), the member-vs-requirement collision error, the qualified-prefix escape
//! hatch, and coverage disclosure — plus the surviving library-level default-contract-binding
//! proofs over `why` (READ-FLOOR-BINDING-DEFAULT), one of the four traversals `explain`
//! re-homes.
//!
//! The four read *CLI verbs* (`why`/`requirements`/`impact`/`context`) retired at
//! CLI-COLLAPSE; `explain <target>` is their sole CLI spelling as of EXPLAIN-UNIFY. The
//! traversal *engine* survives untouched ([`temper::read`]) for `explain` to reuse, so
//! this file exercises the read library directly rather than spawning the binary.

use std::collections::BTreeMap;

use temper::compose::Requirement;
use temper::extract::{EmbeddedMember, Features};
use temper::read::{self, CustomMember};
use temper::tap::{self, TAP_RECORD_VERSION, TapEvent, TapRecord};

mod common;

/// A member's [`Features`] as the read family reads them: its id, the requirements it
/// opts into, and a `description` field (so `impact`'s reachability strand has a
/// non-panicking registration input). Mirrors `read.rs`'s own `impact_tests::feature`
/// helper — duplicated here since this file, being outside the crate, can only build
/// `Features` through its public fields.
fn feature(id: &str, satisfies: &[&str]) -> Features {
    let mut fields = BTreeMap::new();
    fields.insert(
        "description".to_string(),
        serde_json::Value::String("d".to_string()),
    );
    Features {
        id: id.to_string(),
        fields,
        body_lines: 1,
        rendered_lines: Some(1),
        rendered_chars: Some(0),
        headings: Vec::new(),
        sections: Vec::new(),
        source_dir: Some(id.to_string()),
        directives: Vec::new(),
        fenced_blocks: Vec::new(),
        nested_members: Vec::new(),
        satisfies: satisfies.iter().map(|s| (*s).to_string()).collect(),
        edge_placements: None,
    }
}

/// A `required`/advisory requirement with everything else defaulted.
fn req(name: &str, required: bool) -> Requirement {
    Requirement {
        name: name.to_string(),
        prose: None,
        kind: None,
        required,
        clauses: Vec::new(),
        verifier: None,
    }
}

/// Call `read::explain` over the given custom members (`why`/`requirements`'s own
/// member listing), and the given by-kind corpus + roster (`impact`/`context`'s
/// corpus, and the species-resolution existence check) — every scenario below only
/// needs those to drive target-species resolution. Kept in sync like `main.rs`'s own
/// `explain` wiring keeps its custom-member listing and by-kind corpus in sync (both
/// read off the same surface directory there); here the caller threads matching ids
/// into both by hand.
fn explain(
    custom: &[CustomMember],
    by_kind: &BTreeMap<&str, &[Features]>,
    roster: &BTreeMap<String, Requirement>,
    target: &str,
) -> String {
    explain_over_log(custom, by_kind, roster, &[], 0, target)
}

/// Call `read::explain` with a tap-log readout threaded in — the field strand's inputs.
/// The bare [`explain`] helper is this with an absent log (empty records, no older
/// records), the no-field-strand case every non-telemetry scenario reads against.
fn explain_over_log(
    custom: &[CustomMember],
    by_kind: &BTreeMap<&str, &[Features]>,
    roster: &BTreeMap<String, Requirement>,
    tap_records: &[TapRecord],
    tap_older_version: usize,
    target: &str,
) -> String {
    let registrations = BTreeMap::new();
    read::explain(
        custom,
        roster,
        &BTreeMap::new(),
        by_kind,
        &[],
        &[],
        &registrations,
        &[],
        &[],
        &[],
        tap_records,
        tap_older_version,
        target,
    )
}

#[test]
fn a_member_target_walks_why_impact_and_context() {
    // `solo` names no requirement, so it resolves unambiguously as a member and its
    // explanation carries all three member-grain traversals: why's forward walk,
    // impact's blast radius, and context's neighborhood. `why`/`requirements` read a
    // member's rationale off `custom`, so it is threaded there too (existence alone
    // would resolve off `by_kind`).
    let custom = [CustomMember {
        kind: "spec".to_string(),
        id: "solo".to_string(),
        satisfies: Vec::new(),
    }];
    let members = [feature("solo", &[])];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("spec", &members[..])]);
    let roster: BTreeMap<String, Requirement> = BTreeMap::new();

    let out = explain(&custom, &by_kind, &roster, "solo");
    assert!(
        out.contains("everything that holds it in place"),
        "the member species includes why's forward walk: {out}"
    );
    assert!(
        out.contains("the blast radius if it is removed or renamed"),
        "the member species includes impact's blast radius: {out}"
    );
    assert!(
        out.contains("its declared neighborhood"),
        "the member species includes context's neighborhood: {out}"
    );
}

#[test]
fn a_requirement_target_walks_the_reverse_roster() {
    // `only-req` names no member, so it resolves unambiguously as a requirement and
    // narrates through `requirements`'s reverse walk (satisfier set, coverage, blast
    // radius) alone. `filler` satisfies it via `custom`, so its rationale narrates
    // (`by_kind`'s decidable `Features::satisfies` carries none).
    let custom = [CustomMember {
        kind: "spec".to_string(),
        id: "filler".to_string(),
        satisfies: vec![temper::document::Satisfies::new("only-req")],
    }];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::new();
    let roster = BTreeMap::from([("only-req".to_string(), req("only-req", true))]);

    let out = explain(&custom, &by_kind, &roster, "only-req");
    assert!(out.contains("Requirement `only-req`:"), "{out}");
    assert!(out.contains("Satisfied by:"), "{out}");
    assert!(out.contains("`filler`"), "{out}");
    // Member-grain traversals never fire for a requirement target.
    assert!(!out.contains("everything that holds it in place"), "{out}");
}

#[test]
fn a_requirement_targets_authored_prose_narrates_verbatim() {
    // The requirement's `prose` — carried, never interpreted (contract.md,
    // "requirement — a shipped kind, not a primitive") — must reach `explain`'s
    // narration exactly as authored.
    let custom = [CustomMember {
        kind: "spec".to_string(),
        id: "filler".to_string(),
        satisfies: vec![temper::document::Satisfies::new("only-req")],
    }];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::new();
    let roster = BTreeMap::from([(
        "only-req".to_string(),
        Requirement {
            prose: Some("the corpus carries a north-star intent spec".to_string()),
            ..req("only-req", true)
        },
    )]);

    let out = explain(&custom, &by_kind, &roster, "only-req");
    assert!(
        out.contains("the corpus carries a north-star intent spec"),
        "explain must narrate the requirement's authored prose verbatim: {out}"
    );
}

#[test]
fn a_member_known_only_to_by_kind_resolves_without_a_spurious_not_found() {
    // `live` is discovered live off disk (`by_kind`) but never threaded into the
    // rationale-carrying custom listing — a drift `main.rs`'s two-source wiring can
    // produce (`by_kind`'s features resolve live off the harness root, while the
    // custom listing is populated only for non-built-in kinds). `why` must
    // resolve existence off `by_kind`, the same corpus the dispatcher's own species
    // resolution already used to dispatch here — never report it not-found and have
    // `impact`/`context` (called right after, over the same target) narrate it anyway.
    let members = [feature("live", &[])];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("spec", &members[..])]);
    let roster: BTreeMap<String, Requirement> = BTreeMap::new();

    let out = explain(&[], &by_kind, &roster, "live");
    assert!(
        !out.contains("No member named `live`"),
        "a member `by_kind` already resolved is never reported not-found: {out}"
    );
    assert!(
        out.contains("everything that holds it in place"),
        "it still narrates why's forward walk: {out}"
    );
    assert!(
        !out.contains("`import`"),
        "no output suggests the retired `import` verb: {out}"
    );
}

#[test]
fn a_requirement_satisfied_only_in_by_kind_reports_filled_agreeing_with_the_gate() {
    // `locked` satisfies `req` only through `by_kind` — the corpus `roster::check`
    // (the gate) counts satisfiers from — never threaded into `custom`. `explain` must
    // report the same fill status the gate reports: filled, never unfilled, even
    // though the custom listing carries no rationale-bearing record of it.
    let members = [feature("locked", &["req"])];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("spec", &members[..])]);
    let roster = BTreeMap::from([("req".to_string(), req("req", true))]);

    let out = explain(&[], &by_kind, &roster, "req");
    assert!(out.contains("Requirement `req`:"), "{out}");
    assert!(
        out.contains("required, filled by 1 member(s)"),
        "the gate counts `locked` as a satisfier, so `explain` must too: {out}"
    );
    assert!(
        !out.contains("required, and unfilled"),
        "a locked satisfier must never narrate the requirement as unfilled: {out}"
    );
    assert!(out.contains("`locked`"), "{out}");
}

#[test]
fn a_leaf_address_walks_impact_and_context_at_leaf_grain_and_discloses_coverage() {
    let mut leafy = feature("20-surface", &[]);
    leafy.nested_members = vec![EmbeddedMember {
        kind: "decision".to_string(),
        key: "surface-authority".to_string(),
        leaves: BTreeMap::from([("chosen".to_string(), "the surface is canonical".to_string())]),
        members: Vec::new(),
    }];
    let members = [leafy];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("spec", &members[..])]);
    let roster: BTreeMap<String, Requirement> = BTreeMap::new();

    // A leaf address never dispatches to `why`/`requirements`, so no custom member is
    // needed to back it.
    let out = explain(
        &[],
        &by_kind,
        &roster,
        "20-surface/decision/surface-authority/chosen",
    );
    assert!(
        out.contains("leaf grain:"),
        "impact's leaf-grain header: {out}"
    );
    assert!(
        out.contains("its declared neighborhood"),
        "context's leaf-grain header: {out}"
    );
    assert!(out.contains("Fallout: none"), "{out}");
    assert!(
        out.contains("Coverage:"),
        "a leaf-grain answer discloses coverage (`specs/intent.md`): {out}"
    );
}

#[test]
fn a_member_vs_requirement_collision_errors_with_both_qualified_spellings() {
    // `shared` is both a member id and a requirement name — `explain` never guesses
    // which the author meant. Species resolution checks `by_kind` alone (never
    // `custom`), so no custom member is needed to trigger the collision.
    let members = [feature("shared", &[])];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &members[..])]);
    let roster = BTreeMap::from([("shared".to_string(), req("shared", false))]);

    let out = explain(&[], &by_kind, &roster, "shared");
    assert!(
        out.contains("names more than one thing"),
        "the collision is reported, never silently resolved: {out}"
    );
    assert!(out.contains("`member:shared`"), "{out}");
    assert!(out.contains("`requirement:shared`"), "{out}");
}

#[test]
fn a_qualified_prefix_resolves_directly_even_when_ambiguous() {
    // The same collision as above, but spelled explicitly — the qualifier is always
    // accepted, bypassing the ambiguity check entirely. `shared` is backed as a custom
    // member too, so the `member:` spelling's `why` walk actually resolves.
    let custom = [CustomMember {
        kind: "spec".to_string(),
        id: "shared".to_string(),
        satisfies: Vec::new(),
    }];
    let members = [feature("shared", &[])];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("spec", &members[..])]);
    let roster = BTreeMap::from([("shared".to_string(), req("shared", false))]);

    let as_member = explain(&custom, &by_kind, &roster, "member:shared");
    assert!(
        as_member.contains("everything that holds it in place"),
        "an explicit `member:` spelling resolves as a member, no collision error: {as_member}"
    );

    let as_requirement = explain(&custom, &by_kind, &roster, "requirement:shared");
    assert!(
        as_requirement.contains("Requirement `shared`:"),
        "an explicit `requirement:` spelling resolves as a requirement: {as_requirement}"
    );
}

#[test]
fn an_unrecognized_target_is_a_clean_read_naming_no_namespace() {
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::new();
    let roster: BTreeMap<String, Requirement> = BTreeMap::new();

    let out = explain(&[], &by_kind, &roster, "ghost");
    assert!(
        out.contains("No member, requirement, or leaf address named `ghost`"),
        "{out}"
    );
}

/// Default-contract-binding narration over the read family's public `why` API
/// (READ-FLOOR-BINDING-DEFAULT): a default contract is named for its kind — never a
/// `<kind>.<source>` package, and never every non-rule kind defaulting to the skill
/// default contract. A memory member is threaded as a custom member the way a
/// built-in reaches the read family. Skills/rules keep their own default contracts —
/// these exercise the resolution branches directly.
mod default_contract_binding {
    use std::collections::BTreeMap;

    use temper::compose::Requirement;
    use temper::extract::Features;
    use temper::read::{self, CustomMember};

    /// Narrate one custom member (its `kind` and `id`) through `why` over an otherwise-empty
    /// corpus, returning the stdout narration. The roster/edge inputs are empty, so the
    /// governing-default-contract line is all this exercises.
    fn why_kind(kind: &str, id: &str) -> String {
        let custom = [CustomMember {
            kind: kind.to_string(),
            id: id.to_string(),
            satisfies: Vec::new(),
        }];
        let roster: BTreeMap<String, Requirement> = BTreeMap::new();
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::new();
        read::why(&custom, &roster, &BTreeMap::new(), &by_kind, &[], &[], id)
    }

    #[test]
    fn a_memory_member_names_its_own_default_contract_never_the_skill_default_contract() {
        // `memory` binds the `memory` default contract — never mis-narrated as the
        // `skill` default contract, the default-to-skill bug this entry closes.
        let memory = why_kind("memory", "project-memory");
        assert!(
            memory.contains("binds the `memory` default contract"),
            "a memory member is bound to its own default contract: {memory}"
        );
        assert!(
            !memory.contains("binds the `skill` default contract"),
            "a memory member is never narrated as skill-bound: {memory}"
        );
    }

    #[test]
    fn a_builtin_name_resolves_to_its_bound_default_contract() {
        // `skill`/`rule` each name their own default contract — the kind's bare label.
        let skill = why_kind("skill", "reviewer");
        assert!(
            skill.contains("binds the `skill` default contract"),
            "{skill}"
        );
        let rule = why_kind("rule", "collaboration");
        assert!(rule.contains("binds the `rule` default contract"), "{rule}");
    }

    #[test]
    fn a_kind_with_no_default_contract_falls_back_to_its_own_name() {
        // A kind with no author binding and no embedded default contract is named by
        // its own kind name, not silently mis-bound to the skill default contract.
        let out = why_kind("adr", "0001-adopt-temper");
        assert!(out.contains("binds the `adr` default contract"), "{out}");
        assert!(!out.contains("binds the `skill` default contract"), "{out}");
    }
}

/// Append `records` to a fresh workspace through the real tap writer, then read them
/// back — driving the field strand off the true on-disk record shape and the reader's own
/// older-version counting, never a hand-built readout.
fn tap_readout(records: &[TapRecord]) -> tap::LogReadout {
    let workspace = common::tmpdir("field-strand");
    for record in records {
        tap::append(&workspace, record).unwrap();
    }
    tap::read_log(&workspace).unwrap()
}

/// A tap record naming `identity` under `event`, written at `version` — a `version` below
/// `TAP_RECORD_VERSION` exercises the reader's older-version toleration.
fn tap_record(version: u32, event: TapEvent, identity: &str) -> TapRecord {
    TapRecord {
        version,
        session: "sess".to_string(),
        event,
        identity: identity.to_string(),
        reason: None,
    }
}

#[test]
fn a_member_target_narrates_its_tap_event_counts_and_denominators() {
    // Two skill members share the `skill_invoked` denominator; a `tool_use` for a tool no
    // kind declares (`Bash`) joins to no member, so it never enters a denominator — the
    // join is through the lock, never a raw string tally.
    let skills = [feature("deploy", &[]), feature("audit", &[])];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
    let roster: BTreeMap<String, Requirement> = BTreeMap::new();

    let readout = tap_readout(&[
        tap_record(TAP_RECORD_VERSION, TapEvent::SkillInvoked, "deploy"),
        tap_record(TAP_RECORD_VERSION, TapEvent::SkillInvoked, "deploy"),
        tap_record(TAP_RECORD_VERSION, TapEvent::SkillInvoked, "deploy"),
        tap_record(TAP_RECORD_VERSION, TapEvent::SkillInvoked, "audit"),
        tap_record(TAP_RECORD_VERSION, TapEvent::SkillInvoked, "audit"),
        tap_record(TAP_RECORD_VERSION, TapEvent::ToolUse, "Bash"),
    ]);

    let out = explain_over_log(
        &[],
        &by_kind,
        &roster,
        &readout.records,
        readout.older_version,
        "deploy",
    );
    assert!(
        out.contains("its local telemetry"),
        "the member target carries a field strand: {out}"
    );
    assert!(
        out.contains("`skill_invoked` — 3 of 5"),
        "the member's counts + denominators are joined through the lock: {out}"
    );
    assert!(
        !out.contains("tool_use"),
        "an event naming no declared member is never counted against the members: {out}"
    );
    // The field strand is joined beside — never in place of — the other member strands.
    assert!(
        out.contains("everything that holds it in place"),
        "why's forward walk still narrates: {out}"
    );
}

#[test]
fn an_older_version_record_surfaces_as_a_count_never_a_silent_skip() {
    let skills = [feature("deploy", &[])];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
    let roster: BTreeMap<String, Requirement> = BTreeMap::new();

    // One current record and one an older tap wrote, both naming `deploy`. The older one
    // still materializes, so it is counted in the totals AND surfaced as an older line.
    let readout = tap_readout(&[
        tap_record(TAP_RECORD_VERSION, TapEvent::SkillInvoked, "deploy"),
        tap_record(TAP_RECORD_VERSION - 1, TapEvent::SkillInvoked, "deploy"),
    ]);
    assert_eq!(
        readout.older_version, 1,
        "the reader counts the older record: {readout:?}"
    );

    let out = explain_over_log(
        &[],
        &by_kind,
        &roster,
        &readout.records,
        readout.older_version,
        "deploy",
    );
    assert!(
        out.contains("Older records: 1 line an older tap wrote"),
        "the older record surfaces as a counted line: {out}"
    );
    assert!(
        out.contains("never") && out.contains("skipped"),
        "the older record is counted, never silently skipped: {out}"
    );
    assert!(
        out.contains("`skill_invoked` — 2 of 2"),
        "the older record is counted in the totals, not dropped: {out}"
    );
}

#[test]
fn an_absent_log_narrates_no_field_strand_and_explain_still_reads() {
    let skills = [feature("deploy", &[])];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
    let roster: BTreeMap<String, Requirement> = BTreeMap::new();

    // The bare `explain` helper threads an absent log (an empty readout).
    let out = explain(&[], &by_kind, &roster, "deploy");
    assert!(
        !out.contains("its local telemetry"),
        "an absent log narrates no field strand at all: {out}"
    );
    // The member still narrates whole — `explain` returns normally, so `main` exits zero.
    assert!(
        out.contains("everything that holds it in place"),
        "the other strands still narrate over an absent log: {out}"
    );
}

/// `why`'s edge narration route-resolves mentions against the same corpus the gate's
/// `route_mentions` does (READ-EDGE-UNIFY): a mention to a discovered member narrates as a
/// resolved edge, a mention to an absent target as a dangling mention — never folded into
/// the resolved set as though it pointed at a real member.
mod mention_narration {
    use std::collections::BTreeMap;

    use temper::compose::Requirement;
    use temper::extract::Features;
    use temper::graph::{self, MentionDeclaration};
    use temper::read;

    use super::feature;

    /// The lifted mention edge the read family folds in — built through the real lift so
    /// the edge carries the `mention` field route resolution keys on.
    fn mention_edges(member: &str, target: &str) -> Vec<graph::ResolvedEdge> {
        graph::resolved_mention_edges(&[MentionDeclaration {
            member: member.to_string(),
            target: target.to_string(),
        }])
    }

    #[test]
    fn a_mention_to_a_discovered_member_narrates_as_a_resolved_edge() {
        // `style` mentions `skill:standards`, present in the corpus — the mention resolves
        // and narrates as an edge the citing member points at, exactly as the gate resolves
        // it.
        let rules = [feature("style", &[])];
        let skills = [feature("standards", &[])];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("rule", &rules[..]), ("skill", &skills[..])]);
        let roster: BTreeMap<String, Requirement> = BTreeMap::new();
        let mentions = mention_edges("rule:style", "skill:standards");

        let out = read::why(
            &[],
            &roster,
            &BTreeMap::new(),
            &by_kind,
            &[],
            &mentions,
            "style",
        );
        assert!(
            out.contains("it points at `standards` (skill) via its `mention` field"),
            "a mention to a discovered member narrates as a resolved edge: {out}"
        );
        assert!(
            !out.contains("dangling mention"),
            "a resolved mention is never narrated as dangling: {out}"
        );
    }

    #[test]
    fn a_deferred_mention_to_an_absent_target_narrates_as_dangling() {
        // `style` mentions `skill:ghost`, absent from the corpus — the deferred mention
        // dangles. `why` narrates it as the gate's route verdict, never folded into the
        // resolved edge set as though it reached a real member.
        let rules = [feature("style", &[])];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("rule", &rules[..])]);
        let roster: BTreeMap<String, Requirement> = BTreeMap::new();
        let mentions = mention_edges("rule:style", "skill:ghost");

        let out = read::why(
            &[],
            &roster,
            &BTreeMap::new(),
            &by_kind,
            &[],
            &mentions,
            "style",
        );
        assert!(
            out.contains("dangling mention") && out.contains("ghost"),
            "a mention to an absent target narrates as a dangling mention naming its target: {out}"
        );
        assert!(
            !out.contains("it points at `ghost`"),
            "a dangling mention is never narrated as a resolved edge: {out}"
        );
    }
}

#[test]
fn explain_narrates_kind_guidance_in_governing_contract() {
    // A kind's authored guidance in its contract rides the governing contract
    // narration, visible at the moment of authoring via `explain`.
    use temper::contract::{Clause, Contract, Predicate, Severity};
    let custom = [CustomMember {
        kind: "spec".to_string(),
        id: "myspec".to_string(),
        satisfies: Vec::new(),
    }];
    let members = [feature("myspec", &[])];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("spec", &members[..])]);
    let roster: BTreeMap<String, Requirement> = BTreeMap::new();
    let mut contracts: BTreeMap<String, Contract> = BTreeMap::new();
    contracts.insert(
        "spec".to_string(),
        Contract {
            name: "spec".to_string(),
            guidance: Some("A spec documents the harness's shape.".to_string()),
            clauses: vec![Clause {
                label: "spec.required".to_string(),
                severity: Severity::Required,
                guidance: None,
                source: None,
                predicate: Predicate::Required {
                    field: "name".to_string(),
                },
            }],
        },
    );

    let registrations = BTreeMap::new();
    let out = temper::read::explain(
        &custom,
        &roster,
        &contracts,
        &by_kind,
        &[],
        &[],
        &registrations,
        &[],
        &[],
        &[],
        &[],
        0,
        "myspec",
    );
    assert!(
        out.contains("A spec documents the harness's shape."),
        "guidance appears in the governing contract narration: {out}"
    );
    assert!(
        out.contains("spec.required"),
        "clauses still appear alongside guidance: {out}"
    );
}

#[test]
fn explain_omits_governing_contract_guidance_when_absent() {
    // A kind with no guidance emits no guidance line in the governing contract
    // narration, but still narrates the clauses normally.
    use temper::contract::{Clause, Contract, Predicate, Severity};
    let custom = [CustomMember {
        kind: "spec".to_string(),
        id: "myspec".to_string(),
        satisfies: Vec::new(),
    }];
    let members = [feature("myspec", &[])];
    let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("spec", &members[..])]);
    let roster: BTreeMap<String, Requirement> = BTreeMap::new();
    let mut contracts: BTreeMap<String, Contract> = BTreeMap::new();
    contracts.insert(
        "spec".to_string(),
        Contract {
            name: "spec".to_string(),
            guidance: None,
            clauses: vec![Clause {
                label: "spec.required".to_string(),
                severity: Severity::Required,
                guidance: None,
                source: None,
                predicate: Predicate::Required {
                    field: "name".to_string(),
                },
            }],
        },
    );

    let registrations = BTreeMap::new();
    let out = temper::read::explain(
        &custom,
        &roster,
        &contracts,
        &by_kind,
        &[],
        &[],
        &registrations,
        &[],
        &[],
        &[],
        &[],
        0,
        "myspec",
    );
    // Guidance narration marker (‣) should not appear.
    assert!(
        !out.contains("‣"),
        "no guidance marker appears when guidance is absent: {out}"
    );
    assert!(
        out.contains("spec.required"),
        "clauses still narrate normally: {out}"
    );
}
