//! End-to-end acceptance over the harness reference graph — route resolution
//! against a lock-declared edge field.
//!
//! Drives the built `temper` binary so the whole path is pinned: a harness of a rule
//! (carrying a `routes_to` frontmatter field) and a skill written straight at their real
//! Claude Code locus, a golden lock declaring the `routes_to` edge,
//! building the graph over the live corpus, and the exit code.
//!
//! The cases mirror the entry's acceptance:
//! - a rule whose `routes_to` names a real skill resolves and the run is clean;
//! - a rule whose `routes_to` names an absent skill trips a route-resolution
//!   finding and fails the run;
//! - no declared edge at all, no graph runs (the floor-only outcome is unchanged).

use std::fs;
use std::path::Path;

mod common;

use temper::drift::{
    AssemblyFactRow, ClauseRow, Declarations, DegreeBoundRow, EdgeBoundRow, RequirementRow,
};
use temper::drift::{KindFactRow, LayoutRegionRow, LayoutRow, NestedMemberRow, TemplateRow};

/// A floor-clean rule carrying a `routes_to` reference field — the declared edge
/// the graph reads. `routes_to` is not a floor-forbidden rule key, so the rule
/// stays clean and the only finding a case can produce is the route one.
fn routing_rule(routes_to: &str) -> String {
    format!(
        "---\n\
         routes_to: {routes_to}\n\
         ---\n\
         # Style\n\
         \n\
         Prefer the standards skill.\n"
    )
}

/// An `edge` assembly fact declaring one target kind — the lock row a
/// `[[kind.<from>.relationships]]` table used to project.
fn edge(from: &str, field: &str, to: &str) -> AssemblyFactRow {
    edge_to_set(from, field, &[to])
}

/// An `edge` assembly fact over a declared target *set* — the general row `edge` is the
/// one-element case of.
fn edge_to_set(from: &str, field: &str, to: &[&str]) -> AssemblyFactRow {
    AssemblyFactRow {
        fact: "edge".to_string(),
        value: None,
        from: Some(from.to_string()),
        field: Some(field.to_string()),
        to: Some(to.iter().map(|kind| (*kind).to_string()).collect()),
    }
}

/// The `gate` requirement's declaration row, optionally bound to `kind` and carrying
/// a required `degree` clause — the lock row a `[requirement.gate]` table used to
/// project. `kind: None` is the kind-blind case.
fn degree_requirement(kind: Option<&str>, degree: DegreeBoundRow) -> RequirementRow {
    RequirementRow {
        clauses: vec![common::required_clause_row(
            "degree",
            None,
            None,
            None,
            Some(degree),
        )],
        ..common::requirement("gate", false, kind)
    }
}

/// A floor-clean skill carrying a `routes_to` reference field. A skill preserves
/// unknown frontmatter keys under `extra`, so `routes_to` rides along as a declared
/// edge — the skill→rule return arc a cycle needs — without tripping the floor.
fn routing_skill(name: &str, routes_to: &str) -> String {
    format!(
        "---\n\
         name: {name}\n\
         description: Use when {name} is the task at hand; not for anything else.\n\
         routes_to: {routes_to}\n\
         ---\n\
         # {name}\n\
         \n\
         Body.\n"
    )
}

/// The two `routes_to` edges — `rule` and `skill` each pointing at the other — so the
/// reference graph can carry a `rule → skill → rule` circle. The acyclic cases build on
/// both.
fn mutual_routes_edges() -> Vec<AssemblyFactRow> {
    vec![
        edge("rule", "routes_to", "skill"),
        edge("skill", "routes_to", "rule"),
    ]
}

/// The one `routes_to` edge on the `rule` kind (its owning kind the edge source),
/// targeting skills — the harness reference graph the cases build. A reference is a
/// kind capability.
fn routes_to_edge() -> Vec<AssemblyFactRow> {
    vec![edge("rule", "routes_to", "skill")]
}

#[test]
fn a_resolving_route_is_clean() {
    let root = common::tmpdir("resolves");
    // The rule routes to `standards`, which the skill provides — the route resolves,
    // so the whole run is clean.
    common::write_rule_skill_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &common::clean_skill("standards"),
    );
    common::write_lock(
        &root,
        Declarations {
            assembly: routes_to_edge(),
            ..Declarations::default()
        },
    );

    let run = common::check_in(&root, &[], None);
    assert!(
        run.ok,
        "a declared route that resolves to a real skill passes ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_dangling_route_fails_the_run_with_a_route_resolution_finding() {
    let root = common::tmpdir("dangling");
    // The rule routes to `absent`, but the only skill is `standards` — the route
    // resolves to no artifact, a dangling route that fails the run.
    common::write_rule_skill_harness(
        &root,
        "style",
        &routing_rule("absent"),
        "standards",
        &common::clean_skill("standards"),
    );
    common::write_lock(
        &root,
        Declarations {
            assembly: routes_to_edge(),
            ..Declarations::default()
        },
    );

    let run = common::check_in(&root, &[], None);
    assert!(
        !run.ok,
        "a declared route that resolves to no artifact must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("style")
            && run.output.contains("absent")
            && run.output.contains("routes_to"),
        "the finding names the routing artifact, the dangling target, and the reference field, got:\n{}",
        run.output
    );
}

#[test]
fn an_unadopted_harness_runs_no_graph() {
    let root = common::tmpdir("no-edge");
    // The same corpus with a dangling `routes_to`, but no declared edge at all: no
    // graph runs and the (floor-clean) corpus passes. The reference is a declared
    // *contract*, never inferred — with none declared, temper says nothing about the
    // route.
    common::write_rule_skill_harness(
        &root,
        "style",
        &routing_rule("absent"),
        "standards",
        &common::clean_skill("standards"),
    );

    let absent = common::check_in(&root, &[], None);
    assert!(
        absent.ok,
        "with no declared edge the graph does not run ⇒ zero, got:\n{}",
        absent.output
    );

    // A stray retired manifest carrying a `[kind]` layer — never read, so it declares
    // no lock edge either — runs no graph: the outcome is byte-for-byte the floor's.
    common::write_retired_manifest(&root, "[kind.skill]\npackage = \"skill.anthropic\"\n");
    let no_edge = common::check_in(&root, &[], None);
    assert!(no_edge.ok, "an empty graph changes nothing ⇒ still zero");
    assert_eq!(
        absent.output, no_edge.output,
        "a stray manifest must produce identical output to none"
    );
}

#[test]
fn an_acyclic_reference_graph_passes() {
    let root = common::tmpdir("acyclic");
    // `rule style → skill standards`, but the skill routes nowhere — even with both
    // edge kinds declared, the graph is a DAG, so `acyclic` is clean.
    common::write_rule_skill_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &common::clean_skill("standards"),
    );
    common::write_lock(
        &root,
        Declarations {
            assembly: mutual_routes_edges(),
            ..Declarations::default()
        },
    );

    let run = common::check_in(&root, &[], None);
    assert!(
        run.ok,
        "an acyclic reference graph passes ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_cyclic_reference_graph_fails_the_run() {
    let root = common::tmpdir("cyclic");
    // `rule style → skill standards → rule style`: the rule routes to the skill and
    // the skill routes back to the rule. Both routes resolve, so the only finding is
    // the cycle — which must fail the run.
    common::write_rule_skill_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &routing_skill("standards", "style"),
    );
    common::write_lock(
        &root,
        Declarations {
            assembly: mutual_routes_edges(),
            ..Declarations::default()
        },
    );

    let run = common::check_in(&root, &[], None);
    assert!(
        !run.ok,
        "a cycle in the reference graph must fail the run ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("cycle")
            && run.output.contains("style")
            && run.output.contains("standards"),
        "the finding names the cycle and the artifacts forming it, got:\n{}",
        run.output
    );
}

/// An inclusive `[min, max]` edge-count bound, either endpoint optional — the shape
/// each degree case declares.
fn edge_bound(min: Option<usize>, max: Option<usize>) -> EdgeBoundRow {
    EdgeBoundRow { min, max }
}

#[test]
fn a_self_registering_degree_bound_fires_when_the_node_is_pointed_at() {
    let root = common::tmpdir("degree-self-reg-fires");
    // The rule `style` routes to the skill `standards`, so `standards` has incoming
    // degree 1. A requirement declaring the skill self-registering (`incoming = { max = 0 }`,
    // "must not be pointed at") is violated — the run fails on the degree finding.
    common::write_rule_skill_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &common::clean_skill("standards"),
    );
    // The skill `standards` opts into `gate`, placing it in the degree bound's
    // satisfier set.
    common::write_lock(
        &root,
        Declarations {
            assembly: routes_to_edge(),
            requirements: vec![degree_requirement(
                Some("skill"),
                DegreeBoundRow {
                    incoming: Some(edge_bound(None, Some(0))),
                    outgoing: None,
                },
            )],
            ..Declarations::default()
        },
    );
    common::author_satisfies(&root, "skills", "standards", &["gate"]);

    let run = common::check_in(&root, &[], None);
    assert!(
        !run.ok,
        "a self-registering skill that is pointed at must fail the run ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("degree")
            && run.output.contains("incoming")
            && run.output.contains("standards"),
        "the finding names the degree bound, the direction, and the over-connected artifact, got:\n{}",
        run.output
    );
}

#[test]
fn a_self_registering_degree_bound_passes_when_the_node_is_not_pointed_at() {
    let root = common::tmpdir("degree-self-reg-passes");
    // Same edge and harness, but the bound ranges over the *rule* `style`: nothing
    // points at the rule (the only edge is rule → skill), so its incoming degree is
    // zero — inside `incoming = { max = 0 }`, and the run is clean.
    common::write_rule_skill_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &common::clean_skill("standards"),
    );
    // The rule `style` opts into `gate`, so the bound ranges over it.
    common::write_lock(
        &root,
        Declarations {
            assembly: routes_to_edge(),
            requirements: vec![degree_requirement(
                Some("rule"),
                DegreeBoundRow {
                    incoming: Some(edge_bound(None, Some(0))),
                    outgoing: None,
                },
            )],
            ..Declarations::default()
        },
    );
    common::author_satisfies(&root, "rules", "style", &["gate"]);

    let run = common::check_in(&root, &[], None);
    assert!(
        run.ok,
        "a self-registering rule that nothing points at passes ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_routed_degree_bound_passes_when_the_node_is_reachable() {
    let root = common::tmpdir("degree-routed-passes");
    // The rule routes to `standards`, so the skill has incoming degree 1 — inside the
    // open-above routed bound `incoming = { min = 1 }` ("must be reachable"). Clean.
    common::write_rule_skill_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &common::clean_skill("standards"),
    );
    // The skill `standards` opts into `gate`, so the routed bound ranges over it.
    common::write_lock(
        &root,
        Declarations {
            assembly: routes_to_edge(),
            requirements: vec![degree_requirement(
                Some("skill"),
                DegreeBoundRow {
                    incoming: Some(edge_bound(Some(1), None)),
                    outgoing: None,
                },
            )],
            ..Declarations::default()
        },
    );
    common::author_satisfies(&root, "skills", "standards", &["gate"]);

    let run = common::check_in(&root, &[], None);
    assert!(
        run.ok,
        "a routed skill that a rule reaches passes ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_routed_degree_bound_fires_when_the_node_is_unreachable() {
    let root = common::tmpdir("degree-routed-fires");
    // The bound ranges over the *rule* `style` and requires it reachable (`incoming =
    // { min = 1 }`), but nothing points at the rule (the only edge is rule → skill),
    // so its incoming degree is zero — outside the bound. The run fails on degree.
    common::write_rule_skill_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &common::clean_skill("standards"),
    );
    // The rule `style` opts into `gate`, so the routed bound ranges over it.
    common::write_lock(
        &root,
        Declarations {
            assembly: routes_to_edge(),
            requirements: vec![degree_requirement(
                Some("rule"),
                DegreeBoundRow {
                    incoming: Some(edge_bound(Some(1), None)),
                    outgoing: None,
                },
            )],
            ..Declarations::default()
        },
    );
    common::author_satisfies(&root, "rules", "style", &["gate"]);

    let run = common::check_in(&root, &[], None);
    assert!(
        !run.ok,
        "a routed rule nothing reaches must fail the run ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("degree")
            && run.output.contains("incoming")
            && run.output.contains("style"),
        "the finding names the degree bound, the direction, and the unreachable artifact, got:\n{}",
        run.output
    );
}

#[test]
fn a_kind_blind_degree_bound_ranges_over_the_opt_in_satisfier_instead_of_being_skipped() {
    let root = common::tmpdir("degree-kind-blind");
    // `gate` declares no `kind` at all. Its satisfier is the *rule* `style`, which
    // nothing points at — a kind-blind requirement's `degree` bound must still range
    // over the opt-in satisfier (whichever modeled kind it is), not be skipped for
    // lack of a declared `kind`.
    common::write_rule_skill_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &common::clean_skill("standards"),
    );
    common::write_lock(
        &root,
        Declarations {
            assembly: routes_to_edge(),
            requirements: vec![degree_requirement(
                None,
                DegreeBoundRow {
                    incoming: Some(edge_bound(Some(1), None)),
                    outgoing: None,
                },
            )],
            ..Declarations::default()
        },
    );
    common::author_satisfies(&root, "rules", "style", &["gate"]);

    let run = common::check_in(&root, &[], None);
    assert!(
        !run.ok,
        "a kind-blind requirement's degree bound must still fire ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("degree")
            && run.output.contains("incoming")
            && run.output.contains("style"),
        "the finding names the degree bound, the direction, and the unreachable satisfier, got:\n{}",
        run.output
    );
}

/// The `gate` requirement bound to `rule`, carrying an **advisory** `mention-reachable`
/// clause over `paths` → `paths`: the source rule's own scope field, and the field read
/// off the *mentioned* member for its gate. Advisory is the shipped severity — literal
/// containment can be wrong, so the check must not block (0028).
fn mention_reachable_requirement() -> RequirementRow {
    RequirementRow {
        clauses: vec![ClauseRow {
            label: None,
            field: Some("paths".to_string()),
            gate: Some("paths".to_string()),
            ..common::clause("mention-reachable", "advisory")
        }],
        ..common::requirement("gate", false, Some("rule"))
    }
}

/// Drive a `mention-reachable` case: a rule `style` scoped by `rule_paths` mentioning a
/// skill `standards` gated by `skill_paths`, with the clause bound to `style` via the
/// `gate` requirement's opt-in selection. `mention` carries the rule→skill mention row
/// when the case wants the graph to reach the target at all.
fn mention_reachable_run(
    slug: &str,
    rule_paths: Option<&str>,
    skill_paths: Option<&str>,
    mention_edge: bool,
) -> common::CheckRun {
    let root = common::tmpdir(slug);
    common::write_rule_skill_harness(
        &root,
        "style",
        &common::scoped_rule(rule_paths),
        "standards",
        &common::gated_skill("standards", skill_paths),
    );
    common::write_lock(
        &root,
        Declarations {
            mentions: if mention_edge {
                vec![common::mention("rule:style", "skill:standards")]
            } else {
                Vec::new()
            },
            requirements: vec![mention_reachable_requirement()],
            ..Declarations::default()
        },
    );
    common::author_satisfies(&root, "rules", "style", &["gate"]);

    common::check_in(&root, &[], None)
}

/// The first diagnosis: a **scoped** source whose scope globs are not literally
/// contained in the mentioned target's gate. The rule loads under `src/**`, the skill is
/// invocable only once a `docs/**` file is read — so the mention fires exactly where the
/// skill cannot be invoked, and the harness answers "no such skill".
#[test]
fn a_scoped_source_outside_the_mentioned_targets_gate_is_an_advisory_finding() {
    let run = mention_reachable_run("mr-uncontained", Some("src/**"), Some("docs/**"), true);
    assert!(
        run.output.contains("mention-reachable")
            && run.output.contains("src/**")
            && run.output.contains("standards"),
        "the finding names the predicate, the uncovered scope glob, and the target, got:\n{}",
        run.output
    );
    assert!(
        run.ok,
        "the clause is advisory — literal containment can be wrong, so it must not block \
         the run ⇒ zero, got:\n{}",
        run.output
    );
}

/// The second diagnosis: an **unscoped** source mentioning a gated target. The rule is
/// always loaded, the skill only after a `docs/**` read — so every session outside the
/// gate carries an obligation it cannot act on. Killed the session's "the mention is
/// just early" position: the probe showed the sad path is a hard error (0028).
#[test]
fn an_unscoped_source_mentioning_a_gated_target_is_an_advisory_finding() {
    let run = mention_reachable_run("mr-unscoped", None, Some("docs/**"), true);
    assert!(
        run.output.contains("mention-reachable")
            && run.output.contains("unscoped")
            && run.output.contains("standards"),
        "the finding names the predicate, the unscoped source, and the gated target, got:\n{}",
        run.output
    );
    assert!(
        run.ok,
        "the clause is advisory ⇒ zero, got:\n{}",
        run.output
    );
}

/// Literal containment holds: the source's every glob appears verbatim in the target's
/// gate, so the mention fires only where the skill is invocable — silent.
#[test]
fn a_source_scoped_inside_the_targets_gate_is_no_finding() {
    let run = mention_reachable_run("mr-contained", Some("docs/**"), Some("docs/**"), true);
    assert!(
        !run.output.contains("mention-reachable"),
        "a scope literally contained in the gate is no finding, got:\n{}",
        run.output
    );
    assert!(run.ok, "and the run is clean ⇒ zero, got:\n{}", run.output);
}

/// An **ungated** target imposes nothing: with no `paths` on the skill, it is invocable
/// from the start, so the source's scope is unconstrained. The trigger is the target's
/// gate field carrying a value — never the source's scope, and never the kind.
#[test]
fn an_ungated_target_is_no_finding_whatever_the_source_scope() {
    let run = mention_reachable_run("mr-ungated", Some("src/**"), None, true);
    assert!(
        !run.output.contains("mention-reachable"),
        "an ungated target constrains no mention of it, got:\n{}",
        run.output
    );
    assert!(run.ok, "and the run is clean ⇒ zero, got:\n{}", run.output);
}

/// The clause judges only the mentions a member **authored**: the same
/// would-be-uncontained scope and gate, with no mention edge between them, is silent. A
/// mention is obligation-free by default — this clause never invents one.
#[test]
fn a_target_the_mention_does_not_reach_is_no_finding() {
    let run = mention_reachable_run("mr-no-mention", Some("src/**"), Some("docs/**"), false);
    assert!(
        !run.output.contains("mention-reachable"),
        "no authored mention, no reachability claim to judge, got:\n{}",
        run.output
    );
    assert!(run.ok, "and the run is clean ⇒ zero, got:\n{}", run.output);
}

#[test]
fn a_deferred_mention_resolves_against_a_discovered_member_at_check() {
    let root = common::tmpdir("mention-resolves");
    // The rule `style` cites the skill `standards` through a mention riding the lock —
    // the shape a discovery-locus mention takes: emit deferred it, and check resolves it
    // against the discovered corpus. `standards` opts into a routed `incoming = { min = 1 }`
    // bound, so the mention counting toward its degree proves the edge resolved to the
    // real, discovered member — the run is clean.
    common::write_rule_skill_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &common::clean_skill("standards"),
    );
    common::write_lock(
        &root,
        Declarations {
            mentions: vec![common::mention("rule:style", "skill:standards")],
            requirements: vec![degree_requirement(
                Some("skill"),
                DegreeBoundRow {
                    incoming: Some(edge_bound(Some(1), None)),
                    outgoing: None,
                },
            )],
            ..Declarations::default()
        },
    );
    common::author_satisfies(&root, "skills", "standards", &["gate"]);

    let run = common::check_in(&root, &[], None);
    assert!(
        run.ok,
        "a mention resolving against a discovered member reaches it ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_mention_naming_no_discovered_member_leaves_the_target_unreached() {
    let root = common::tmpdir("mention-dangles");
    // The same routed `incoming = { min = 1 }` bound on the discovered `standards`, but the
    // rule's mention names `skill:ghost` — no such skill is discovered. The mention reaches
    // the real `standards` never, whose incoming degree stays zero, so the degree bound is
    // missed: the gate refuses, the same verdict a dangling declared edge earns (a dangling
    // mention also trips its own `graph.route` finding — this case pins the degree miss).
    common::write_rule_skill_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &common::clean_skill("standards"),
    );
    common::write_lock(
        &root,
        Declarations {
            mentions: vec![common::mention("rule:style", "skill:ghost")],
            requirements: vec![degree_requirement(
                Some("skill"),
                DegreeBoundRow {
                    incoming: Some(edge_bound(Some(1), None)),
                    outgoing: None,
                },
            )],
            ..Declarations::default()
        },
    );
    common::author_satisfies(&root, "skills", "standards", &["gate"]);

    let run = common::check_in(&root, &[], None);
    assert!(
        !run.ok,
        "a mention naming no discovered member reaches the real target never ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("degree")
            && run.output.contains("incoming")
            && run.output.contains("standards"),
        "the finding names the unreached discovered member and the bound it misses, got:\n{}",
        run.output
    );
}

#[test]
fn a_deferred_mention_to_an_absent_member_fires_a_route_finding() {
    let root = common::tmpdir("mention-route-dangles");
    // The rule `style` mentions `skill:ghost`, absent from the discovered corpus, and no
    // degree clause opts the mention into counting — so the deferred mention's own dangling
    // verdict is check's to own: a `graph.route` finding naming the citing member and the
    // dangling target, the same verb a dangling declared edge trips.
    common::write_rule_skill_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &common::clean_skill("standards"),
    );
    common::write_lock(
        &root,
        Declarations {
            mentions: vec![common::mention("rule:style", "skill:ghost")],
            ..Declarations::default()
        },
    );

    let run = common::check_in(&root, &[], None);
    assert!(
        !run.ok,
        "a mention whose target is absent from the corpus dangles ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("graph.route")
            && run.output.contains("rule:style")
            && run.output.contains("ghost"),
        "the route finding names its citing member and the dangling mention target, got:\n{}",
        run.output
    );
}

#[test]
fn a_mention_to_a_declared_requirement_stays_clean() {
    let root = common::tmpdir("mention-requirement-resolves");
    // A mention may name a bare requirement, resolved against the roster rather than the
    // by-kind corpus. `gate` is declared (advisory, so its own coverage never gates), so the
    // mention resolves and the run stays clean.
    common::write_rule_skill_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &common::clean_skill("standards"),
    );
    common::write_lock(
        &root,
        Declarations {
            mentions: vec![common::mention("rule:style", "gate")],
            requirements: vec![common::requirement("gate", false, None)],
            ..Declarations::default()
        },
    );

    let run = common::check_in(&root, &[], None);
    assert!(
        run.ok,
        "a mention naming a declared requirement resolves ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_mention_to_an_undeclared_requirement_fires_a_route_finding() {
    let root = common::tmpdir("mention-requirement-dangles");
    // The same bare-requirement mention, but no `gate` requirement is declared — the roster
    // resolves it nowhere, so check owns the dangling verdict exactly as it does an absent
    // member target.
    common::write_rule_skill_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &common::clean_skill("standards"),
    );
    common::write_lock(
        &root,
        Declarations {
            mentions: vec![common::mention("rule:style", "gate")],
            ..Declarations::default()
        },
    );

    let run = common::check_in(&root, &[], None);
    assert!(
        !run.ok,
        "a mention naming no declared requirement dangles ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("graph.route")
            && run.output.contains("rule:style")
            && run.output.contains("gate"),
        "the route finding names its citing member and the dangling requirement target, got:\n{}",
        run.output
    );
}

/// End-to-end proof that an edge field targeting an **embedded** kind resolves against
/// that kind's nested members. An embedded kind carries no kind-fact row — it reaches the
/// lock only through its host's `templates` column — so without the by-kind fold it reads
/// as unmodeled and every route over it dangles. Here a `service` layout host declares a
/// `serves` relationship edge into `domain` and nests one `domain` member off the lock's
/// `nested_member` family.
mod embedded_edge_targets {
    use super::*;
    use std::collections::BTreeMap;

    /// A `field` region row filling `slot` — an edge slot when `slot` is one of the
    /// kind's declared edge fields.
    fn field_region(slot: &str) -> LayoutRegionRow {
        LayoutRegionRow {
            region: "field".to_string(),
            import: None,
            slot: Some(slot.to_string()),
            member_kind: None,
            key: None,
        }
    }

    /// The `service` layout host: a lone document under `specs/` carrying a `purpose`
    /// field section and a `serves` relationship edge slot, hosting the embedded kinds
    /// named in `templates`.
    fn service_kind(templates: &[&str]) -> KindFactRow {
        KindFactRow {
            content: Some(LayoutRow {
                regions: vec![field_region("purpose"), field_region("serves")],
            }),
            templates: templates
                .iter()
                .map(|t| TemplateRow {
                    kind: (*t).to_string(),
                    path: None,
                })
                .collect(),
            ..common::kind_facts("service", "specs", "service.md")
        }
    }

    /// One embedded `domain` member nested under the `service` host — the lock row emit
    /// would derive, hand-authored so the case owns its whole corpus.
    fn domain_row(key: &str) -> NestedMemberRow {
        NestedMemberRow {
            host: "service:service".to_string(),
            kind: "domain".to_string(),
            key: key.to_string(),
            leaves: BTreeMap::new(),
            collections: Vec::new(),
            placed_edges: None,
        }
    }

    /// Write the `service` document whose `serves` slot names `target`.
    fn write_service(root: &Path, target: &str) {
        fs::write(
            root.join("specs/service.md"),
            format!("# Purpose\nA service.\n\n# Serves\n- {target}\n"),
        )
        .unwrap();
    }

    #[test]
    fn an_edge_targeting_an_embedded_kind_resolves_by_member_identity() {
        let root = common::scaffold("embedded-edge-resolves");
        // `service` serves the embedded domain `billing`, declared via the host's
        // `templates` and nested off the lock — the edge resolves by identity, clean.
        write_service(&root, "billing");
        common::write_lock(
            &root,
            Declarations {
                kinds: vec![service_kind(&["domain"])],
                assembly: vec![edge("service", "serves", "domain")],
                nested_members: vec![domain_row("billing")],
                ..Declarations::default()
            },
        );

        let run = common::check_in(&root, &[], None);
        assert!(
            run.ok,
            "an edge to an embedded kind resolving to a declared nested member is clean, got:\n{}",
            run.output
        );
    }

    #[test]
    fn a_dangling_embedded_edge_entry_is_a_route_finding() {
        let root = common::scaffold("embedded-edge-dangling");
        // The domain `billing` is nested, but the host serves `absent` — a route over a
        // modeled embedded kind that resolves to no member, the graph's route finding.
        write_service(&root, "absent");
        common::write_lock(
            &root,
            Declarations {
                kinds: vec![service_kind(&["domain"])],
                assembly: vec![edge("service", "serves", "domain")],
                nested_members: vec![domain_row("billing")],
                ..Declarations::default()
            },
        );

        let run = common::check_in(&root, &[], None);
        assert!(
            !run.ok,
            "a dangling embedded-edge entry must fail the run ⇒ non-zero, got:\n{}",
            run.output
        );
        assert!(
            run.output.contains("service")
                && run.output.contains("absent")
                && run.output.contains("serves"),
            "the finding names the host, the dangling target, and the reference field, got:\n{}",
            run.output
        );
    }

    #[test]
    fn an_orphaned_nested_member_row_fails_at_admissibility() {
        let root = common::scaffold("embedded-nested-member-orphan");
        // A `domain` nested member is committed to the lock, but no host declares
        // `domain` as a nested template — no `templates` entry, no member collection. The
        // by-kind corpus would unmodel the orphan while the host-address read still
        // carries it; admissibility rejects the malformed lock instead, naming the kind.
        write_service(&root, "billing");
        common::write_lock(
            &root,
            Declarations {
                kinds: vec![service_kind(&[])],
                nested_members: vec![domain_row("billing")],
                ..Declarations::default()
            },
        );

        let run = common::check_in(&root, &[], None);
        assert!(
            !run.ok,
            "an orphaned nested-member row must fail the run ⇒ non-zero, got:\n{}",
            run.output
        );
        assert!(
            run.output.contains("domain")
                && run.output.contains("billing")
                && run.output.contains("nested"),
            "the finding names the orphaned kind, the member, and its nested-template class, got:\n{}",
            run.output
        );
    }

    #[test]
    fn a_file_child_template_never_admits_an_embedded_row_of_its_kind() {
        let root = common::scaffold("embedded-nested-member-file-child");
        // The host templates `domain` as a *file* child — its members own units at the
        // declared pattern, never a fence in the host's body. So a `domain` nested-member
        // row is as orphaned as if no template named the kind at all: a file-child
        // template declares no embedded layer to hang it on.
        write_service(&root, "billing");
        let host = KindFactRow {
            templates: vec![TemplateRow {
                kind: "domain".to_string(),
                path: Some("domains/*.md".to_string()),
            }],
            ..service_kind(&[])
        };
        common::write_lock(
            &root,
            Declarations {
                kinds: vec![host],
                nested_members: vec![domain_row("billing")],
                ..Declarations::default()
            },
        );

        let run = common::check_in(&root, &[], None);
        assert!(
            !run.ok,
            "a nested-member row of a file-templated kind must fail the run ⇒ non-zero, got:\n{}",
            run.output
        );
        assert!(
            run.output.contains("domain") && run.output.contains("nested"),
            "the finding names the kind and its nested-template class, got:\n{}",
            run.output
        );
    }

    #[test]
    fn an_edge_to_a_kind_no_host_declares_stays_an_admissibility_finding() {
        let root = common::scaffold("embedded-edge-unmodeled");
        // No host declares `domain` — no `templates` entry, no nested member — so it is
        // genuinely unmodeled: the edge is an admissibility fault, not a route dangle.
        write_service(&root, "billing");
        common::write_lock(
            &root,
            Declarations {
                kinds: vec![service_kind(&[])],
                assembly: vec![edge("service", "serves", "domain")],
                ..Declarations::default()
            },
        );

        let run = common::check_in(&root, &[], None);
        assert!(
            !run.ok,
            "an edge to an undeclared kind must fail the run ⇒ non-zero, got:\n{}",
            run.output
        );
        assert!(
            run.output.contains("domain") && run.output.contains("does not model"),
            "the finding names the unmodeled kind as an admissibility fault, got:\n{}",
            run.output
        );
    }
}

/// End-to-end proof of the **source** side of the same grain: an embedded member's own
/// edge field resolves, and counts for degree exactly as a frontmatter field's does.
///
/// The two halves this pins were separately broken. An embedded kind's declared edge
/// fields reached no assembly `edge` fact, so the edge did not exist; and an embedded
/// member's edge leaf is the target's full `kind:name` address, not the bare identity a
/// frontmatter field carries, so the edge resolved to nothing even once declared. Both
/// failed silently — every `degree({incoming})` over the target simply read 0.
mod embedded_edge_sources {
    use super::*;
    use std::collections::BTreeMap;

    /// The `article` layout host: a lone document under `specs/`, hosting the embedded
    /// `citation` members its `templates` column admits.
    fn article_kind() -> KindFactRow {
        KindFactRow {
            templates: vec![TemplateRow {
                kind: "citation".to_string(),
                path: None,
            }],
            ..common::kind_facts("article", "specs", "article.md")
        }
    }

    /// One embedded `citation` nested under the `article` host, its `source` edge leaf
    /// spelled exactly as emit derives it — the target's full `kind:name` address.
    fn citation_row(source: &str) -> NestedMemberRow {
        NestedMemberRow {
            host: "article:article".to_string(),
            kind: "citation".to_string(),
            key: "the-standard".to_string(),
            leaves: BTreeMap::from([("source".to_string(), source.to_string())]),
            collections: Vec::new(),
            placed_edges: None,
        }
    }

    /// The harness both cases open on: the `article` host document, and the skill
    /// `data-access` the citation points at, opted into the `gate` requirement so a
    /// degree bound ranges over it.
    fn write_cited_harness(root: &Path) {
        fs::write(root.join("specs/article.md"), "# Article\n\nBody.\n").unwrap();
        common::write_skill(root, "data-access", &common::clean_skill("data-access"));
    }

    /// The lock both cases commit: the `citation.source → skill` edge, one citation
    /// naming `source`, and a `gate` requirement demanding every satisfier be pointed at
    /// (`incoming = { min = 1 }`).
    fn cited_lock(root: &Path, source: &str) {
        common::write_lock(
            root,
            Declarations {
                kinds: vec![article_kind()],
                assembly: vec![edge("citation", "source", "skill")],
                nested_members: vec![citation_row(source)],
                requirements: vec![degree_requirement(
                    Some("skill"),
                    DegreeBoundRow {
                        incoming: Some(edge_bound(Some(1), None)),
                        outgoing: None,
                    },
                )],
                ..Declarations::default()
            },
        );
        common::author_satisfies(root, "skills", "data-access", &["gate"]);
    }

    #[test]
    fn an_address_spelled_embedded_edge_leaf_resolves_and_counts_for_degree() {
        let root = common::scaffold("embedded-edge-source-resolves");
        write_cited_harness(&root);
        // The citation cites `skill:data-access` — the address spelling emit writes. The
        // edge resolves by identity within `skill`, so `data-access` has incoming degree
        // 1 and clears its `min = 1` bound: the reach clause a consumer's citations are
        // judged by, with no leaf-mention shim.
        cited_lock(&root, "skill:data-access");

        let run = common::check_in(&root, &[], None);
        assert!(
            run.ok,
            "an embedded member's address-spelled edge leaf resolves and counts toward \
             its target's incoming degree, got:\n{}",
            run.output
        );
    }

    #[test]
    fn an_embedded_edge_leaf_addressing_another_kind_stays_dangling() {
        let root = common::scaffold("embedded-edge-source-cross-kind");
        write_cited_harness(&root);
        // `rule:data-access` names the edge's *target kind* wrong. Normalizing the
        // spelling is not cross-attribution: the address is not `edge.to`'s, so it
        // resolves to nothing and dangles under the name its author actually wrote —
        // never onto the same-named skill next door.
        cited_lock(&root, "rule:data-access");

        let run = common::check_in(&root, &[], None);
        assert!(
            !run.ok,
            "an edge leaf addressing a kind that is not the edge's target must fail the \
             run ⇒ non-zero, got:\n{}",
            run.output
        );
        assert!(
            run.output.contains("rule:data-access"),
            "the route finding names the authored spelling, not a normalized identity, got:\n{}",
            run.output
        );
    }
}

/// Library-level fixture proof of the `reachable` predicate: the pure machinery over
/// constructed `Features`, including a caller-declared severity threaded into the
/// finding. The dial that once wired this into the gate retired;
/// the predicate itself stays a live capability for a future edge-scope
/// clause to call.
mod reachability {
    use std::collections::BTreeMap;

    use temper::check::Severity;
    use temper::extract::{FeatureValue, Features, ValueType};
    use temper::graph::{ResolvedEdge, reachable};
    use temper::kind::Registration;

    /// A member carrying an id and, optionally, one frontmatter field — the only inputs
    /// the reachability predicate reads (the id for the finding, the named registration
    /// field for the edge). Everything else is inert.
    fn member(id: &str, field: Option<(&str, FeatureValue)>) -> Features {
        let mut fields = BTreeMap::new();
        if let Some((name, value)) = field {
            fields.insert(name.to_string(), value);
        }
        Features {
            id: id.to_string(),
            fields,
            body_lines: 1,
            headings: Vec::new(),
            sections: Vec::new(),
            source_dir: Some(id.to_string()),
            directives: Vec::new(),
            fenced_blocks: Vec::new(),
            nested_members: Vec::new(),
            satisfies: Vec::new(),
            edge_placements: None,
        }
    }

    /// A `description-trigger` registration over the named field.
    fn description_trigger(field: &str) -> Registration {
        Registration::DescriptionTrigger {
            field: field.to_string(),
        }
    }

    /// A `paths-match` registration over the named field.
    fn paths_match(field: &str) -> Registration {
        Registration::PathsMatch {
            field: field.to_string(),
        }
    }

    /// An observed directive edge `from` one member `to` another — the member→member
    /// occurrence [`classify_directives`](temper::graph) yields and reachability closes
    /// over. Each endpoint is a `(kind, id)` node.
    fn import_edge(from: (&str, &str), to: (&str, &str)) -> ResolvedEdge {
        ResolvedEdge {
            from: (from.0.to_string(), from.1.to_string()),
            field: "at-import".to_string(),
            to: (to.0.to_string(), to.1.to_string()),
        }
    }

    #[test]
    fn a_live_registration_edge_is_reachable() {
        // A skill with a non-empty `description` (a live description-trigger) and a rule
        // whose `paths` glob matches a repo file (a live paths-match) each have a live
        // inbound edge from the world — nothing fires.
        let skills = [member(
            "standards",
            Some((
                "description",
                FeatureValue::scalar(ValueType::String, "Use when styling the code."),
            )),
        )];
        let rules = [member(
            "style",
            Some((
                "paths",
                FeatureValue::scalar(ValueType::String, "src/**/*.rs"),
            )),
        )];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("skill", &skills[..]), ("rule", &rules[..])]);
        let registrations = BTreeMap::from([
            ("skill", vec![description_trigger("description")]),
            ("rule", vec![paths_match("paths")]),
        ]);
        let files = vec!["src/graph.rs".to_string()];
        assert!(reachable(&registrations, &by_kind, &files, &[], Severity::Error).is_empty());
    }

    #[test]
    fn a_blank_description_trigger_field_is_unreachable() {
        // The skill declares a description-trigger on `description`, but the field is
        // whitespace-only — the harness has nothing to load, a dead inbound edge.
        let skills = [member(
            "standards",
            Some(("description", FeatureValue::scalar(ValueType::String, " "))),
        )];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        let registrations = BTreeMap::from([("skill", vec![description_trigger("description")])]);

        let diags = reachable(&registrations, &by_kind, &[], &[], Severity::Error);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, "graph.reachable");
        assert_eq!(diags[0].artifact, "standards");
        assert!(diags[0].message.contains("description"));
        assert!(diags[0].message.contains("world"));

        // The dial is the assembly's: the same dead edge at `advisory` is a warn, so a
        // required-vs-advisory reachability declaration is honored (REACHABILITY-WIRE).
        let advisory = reachable(&registrations, &by_kind, &[], &[], Severity::Warn);
        assert_eq!(advisory.len(), 1);
        assert_eq!(advisory[0].severity, Severity::Warn);
    }

    #[test]
    fn a_zero_match_paths_glob_is_unreachable() {
        // The rule declares a paths-match on `paths`, but its glob matches no file in
        // the supplied repo file-set — the harness activates it never.
        let rules = [member(
            "style",
            Some((
                "paths",
                FeatureValue::scalar(ValueType::String, "docs/**/*.md"),
            )),
        )];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("rule", &rules[..])]);
        let registrations = BTreeMap::from([("rule", vec![paths_match("paths")])]);
        let files = vec!["src/graph.rs".to_string(), "README.md".to_string()];

        let diags = reachable(&registrations, &by_kind, &files, &[], Severity::Error);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "graph.reachable");
        assert_eq!(diags[0].artifact, "style");
        assert!(diags[0].message.contains("paths"));
        assert!(diags[0].message.contains("world"));
    }

    #[test]
    fn an_absent_or_blank_paths_field_is_reachable() {
        // An unscoped rule declares a paths-match registration but carries no `paths` field
        // (or a whitespace-only one) — the harness falls back to unconditional loading,
        // so the inbound edge is live, not dead.
        let absent = member("global", None);
        let blank = member(
            "blank",
            Some(("paths", FeatureValue::scalar(ValueType::String, "   "))),
        );
        let rules = [absent, blank];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("rule", &rules[..])]);
        let registrations = BTreeMap::from([("rule", vec![paths_match("paths")])]);
        // A non-empty repo file-set the absent/blank field is *not* tested against.
        let files = vec!["src/graph.rs".to_string()];

        assert!(reachable(&registrations, &by_kind, &files, &[], Severity::Error).is_empty());
    }

    #[test]
    fn a_kind_that_declares_no_registration_is_not_subject() {
        // The corpus holds a skill with a blank `description`, but no kind declares an
        // registration (the map is empty) — the predicate ranges over declared edges only,
        // so nothing fires. `temper` never invents an edge the kind did not declare.
        let skills = [member(
            "standards",
            Some(("description", FeatureValue::scalar(ValueType::String, ""))),
        )];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        let registrations: BTreeMap<&str, Vec<Registration>> = BTreeMap::new();

        assert!(reachable(&registrations, &by_kind, &[], &[], Severity::Error).is_empty());
    }

    #[test]
    fn a_dead_own_member_imported_by_a_reachable_member_is_live() {
        // The rule `scoped` has a zero-match `paths` glob — its own world-edge is dead.
        // But the memory member `root` declares no registration (unconditionally live) and
        // imports it, so the closure carries `root`'s liveness across the directive edge:
        // `scoped` is reachable, and no finding fires. This is the false positive the
        // slice fixes.
        let memories = [member("root", None)];
        let rules = [member(
            "scoped",
            Some((
                "paths",
                FeatureValue::scalar(ValueType::String, "nowhere/**/*.md"),
            )),
        )];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("memory", &memories[..]), ("rule", &rules[..])]);
        // Only `rule` declares a registration; `memory` is absent ⇒ always live.
        let registrations = BTreeMap::from([("rule", vec![paths_match("paths")])]);
        let edges = [import_edge(("memory", "root"), ("rule", "scoped"))];
        let files = vec!["src/graph.rs".to_string()];

        assert!(reachable(&registrations, &by_kind, &files, &edges, Severity::Error).is_empty());
    }

    #[test]
    fn a_member_imported_only_by_a_dead_member_stays_dead() {
        // Both skills have a blank `description` — dead own-edges. `importer` imports
        // `target`, but a dead importer carries no liveness, so `target` is *not* rescued:
        // both fire. Liveness inherits only from a *reachable* importer.
        let skills = [
            member(
                "importer",
                Some(("description", FeatureValue::scalar(ValueType::String, " "))),
            ),
            member(
                "target",
                Some(("description", FeatureValue::scalar(ValueType::String, " "))),
            ),
        ];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        let registrations = BTreeMap::from([("skill", vec![description_trigger("description")])]);
        let edges = [import_edge(("skill", "importer"), ("skill", "target"))];

        let diags = reachable(&registrations, &by_kind, &[], &edges, Severity::Error);
        assert_eq!(diags.len(), 2);
        assert!(diags.iter().any(|d| d.artifact == "target"));
        assert!(diags.iter().any(|d| d.artifact == "importer"));
    }

    #[test]
    fn the_import_closure_is_hop_capped() {
        // A chain `root → r1 → … → r6`: `root` (a memory member) is unconditionally live,
        // and every `rN` rule has a zero-match `paths` glob (dead own-edge). Liveness
        // propagates one hop per round, capped at the format's five-hop import recursion:
        // `r1..r5` are rescued, but `r6` — six hops from the live seed — stays dead and is
        // the sole finding.
        let memories = [member("root", None)];
        let rules: Vec<Features> = (1..=6)
            .map(|n| {
                member(
                    &format!("r{n}"),
                    Some((
                        "paths",
                        FeatureValue::scalar(ValueType::String, "nowhere/**/*.md"),
                    )),
                )
            })
            .collect();
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("memory", &memories[..]), ("rule", &rules[..])]);
        let registrations = BTreeMap::from([("rule", vec![paths_match("paths")])]);
        let mut edges = vec![import_edge(("memory", "root"), ("rule", "r1"))];
        for n in 1..=5 {
            edges.push(import_edge(
                ("rule", &format!("r{n}")),
                ("rule", &format!("r{}", n + 1)),
            ));
        }
        let files = vec!["src/graph.rs".to_string()];

        let diags = reachable(&registrations, &by_kind, &files, &edges, Severity::Error);
        assert_eq!(diags.len(), 1, "only the past-cap member stays dead");
        assert_eq!(diags[0].artifact, "r6");
    }

    #[test]
    fn a_member_live_on_any_channel_of_its_set_is_reachable() {
        // The skill's declared registration is a two-channel set: a dead
        // description-trigger (blank field) and `user-invoked`, which carries no field
        // and so never dies (REGISTRATION-CHANNELS). Live on one channel is live overall
        // — user invocation and description trigger are channels, not rivals — so
        // nothing fires even though the description-trigger channel alone would.
        let skills = [member(
            "deploy",
            Some(("description", FeatureValue::scalar(ValueType::String, " "))),
        )];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        let registrations = BTreeMap::from([(
            "skill",
            vec![
                Registration::UserInvoked,
                description_trigger("description"),
            ],
        )]);

        assert!(reachable(&registrations, &by_kind, &[], &[], Severity::Error).is_empty());
    }
}

/// The addressing rule a declared target *set* carries: which member of the set an
/// authored address names is read off the written text alone, never inferred from the
/// member population. The corpus is the same rule-routing-to-skill harness throughout —
/// only the edge's declared `to` moves — so each case isolates the addressing.
mod target_set {
    use super::*;

    /// The rule/skill corpus every case here routes over: a `style` rule whose
    /// `routes_to` carries `target`, and a `standards` skill.
    fn routing_harness(label: &str, target: &str) -> std::path::PathBuf {
        let root = common::tmpdir(label);
        common::write_rule_skill_harness(
            &root,
            "style",
            &routing_rule(target),
            "standards",
            &common::clean_skill("standards"),
        );
        root
    }

    #[test]
    fn a_multi_kind_set_resolves_a_kind_qualified_address_to_that_kind() {
        let root = routing_harness("set-qualified", "skill:standards");
        common::write_lock(
            &root,
            Declarations {
                assembly: vec![edge_to_set("rule", "routes_to", &["skill", "rule"])],
                ..Declarations::default()
            },
        );

        let run = common::check_in(&root, &[], None);
        assert!(
            run.ok,
            "a `kind:name` address naming a declared kind resolves within it ⇒ zero, got:\n{}",
            run.output
        );
    }

    #[test]
    fn a_multi_kind_set_dangles_a_bare_name_that_would_be_unique_across_the_set() {
        // `standards` names exactly one member across `skill` and `rule` — inferring it
        // would resolve. It must not: resolution reads the written text, so the answer
        // can never flip as members come and go.
        let root = routing_harness("set-bare", "standards");
        common::write_lock(
            &root,
            Declarations {
                assembly: vec![edge_to_set("rule", "routes_to", &["skill", "rule"])],
                ..Declarations::default()
            },
        );

        let run = common::check_in(&root, &[], None);
        assert!(
            !run.ok,
            "a bare name against a multi-kind set resolves to nothing ⇒ non-zero, got:\n{}",
            run.output
        );
        assert!(
            run.output.contains("standards") && run.output.contains("routes_to"),
            "the finding names the authored spelling and the reference field, got:\n{}",
            run.output
        );
    }

    #[test]
    fn a_multi_kind_set_dangles_an_address_naming_an_undeclared_kind() {
        let root = routing_harness("set-foreign", "agent:standards");
        common::write_lock(
            &root,
            Declarations {
                assembly: vec![edge_to_set("rule", "routes_to", &["skill", "rule"])],
                ..Declarations::default()
            },
        );

        let run = common::check_in(&root, &[], None);
        assert!(
            !run.ok,
            "an address naming a kind outside the declared set resolves to nothing ⇒ non-zero, got:\n{}",
            run.output
        );
    }

    #[test]
    fn a_one_element_set_resolves_a_bare_name_within_its_one_kind() {
        let root = routing_harness("singleton-bare", "standards");
        common::write_lock(
            &root,
            Declarations {
                assembly: vec![edge_to_set("rule", "routes_to", &["skill"])],
                ..Declarations::default()
            },
        );

        let run = common::check_in(&root, &[], None);
        assert!(
            run.ok,
            "a one-element set resolves a bare name within its one kind ⇒ zero, got:\n{}",
            run.output
        );
    }

    #[test]
    fn an_admissibility_finding_names_the_unmodeled_element_not_the_set() {
        let root = routing_harness("set-unmodeled", "skill:standards");
        common::write_lock(
            &root,
            Declarations {
                assembly: vec![edge_to_set("rule", "routes_to", &["skill", "sorcery"])],
                ..Declarations::default()
            },
        );

        let run = common::check_in(&root, &[], None);
        assert!(
            !run.ok,
            "a set declaring a kind `temper` does not model is inadmissible ⇒ non-zero"
        );
        assert!(
            run.output.contains("sorcery"),
            "the finding names the unmodeled element, got:\n{}",
            run.output
        );
        assert!(
            !run.output.contains("`skill`, `sorcery`") && !run.output.contains("skill or sorcery"),
            "the finding names the element, never the whole set — the modeled sibling is not at fault, got:\n{}",
            run.output
        );
    }
}
