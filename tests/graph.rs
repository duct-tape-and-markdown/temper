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

use temper::drift::{AssemblyFactRow, Declarations, DegreeBoundRow, EdgeBoundRow, RequirementRow};
use temper::drift::{KindFactRow, LayoutRegionRow, LayoutRow, NestedMemberRow};

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

/// Write a harness of one rule and one skill straight at their real Claude Code locus
/// — the rule under `.claude/rules/<rule>.md`, the skill under
/// `.claude/skills/<skill>/SKILL.md` — no scratch import. `check` reads built-in kind
/// members live off harness disk.
fn write_harness(root: &Path, rule_name: &str, rule_md: &str, skill_name: &str, skill_md: &str) {
    let rules = root.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(rules.join(format!("{rule_name}.md")), rule_md).unwrap();

    common::write_skill(root, skill_name, skill_md);
}

/// An `edge` assembly fact — the lock row a `[[kind.<from>.relationships]]` table used
/// to project.
fn edge(from: &str, field: &str, to: &str) -> AssemblyFactRow {
    AssemblyFactRow {
        fact: "edge".to_string(),
        value: None,
        from: Some(from.to_string()),
        field: Some(field.to_string()),
        to: Some(to.to_string()),
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
    write_harness(
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
    write_harness(
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
    write_harness(
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
    write_harness(
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
    write_harness(
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
    write_harness(
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
    write_harness(
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
    write_harness(
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
    write_harness(
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
    write_harness(
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

/// End-to-end proof that an edge field targeting an **embedded** kind resolves against
/// that kind's nested members. An embedded kind carries no kind-fact row — it reaches the
/// lock only through its host's `templates` column — so without the by-kind fold it reads
/// as unmodeled and every route over it dangles. Here a `service` layout host declares a
/// `serves` relationship edge into `domain` and nests one `domain` member off the lock's
/// `nested_member` family; the cases mirror the field report's repro.
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
            templates: templates.iter().map(|t| (*t).to_string()).collect(),
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
