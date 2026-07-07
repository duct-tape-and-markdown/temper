//! End-to-end acceptance over the harness reference graph — route resolution
//! against a lock-declared edge field (`specs/architecture/45-governance.md`, "The
//! harness is a graph too — and references are declared edges").
//!
//! Drives the built `temper` binary so the whole path is pinned: a harness of a rule
//! (carrying a `routes_to` frontmatter field) and a skill written straight at their real
//! Claude Code locus, a golden lock declaring the `routes_to` edge
//! (`specs/architecture/20-surface.md`, "The lock and drift — one vocabulary" — the gate
//! sources edges from the lock, never a re-imported manifest relationships table),
//! building the graph over the live corpus, and the exit code.
//!
//! The cases mirror the entry's acceptance:
//! - a rule whose `routes_to` names a real skill resolves and the run is clean;
//! - a rule whose `routes_to` names an absent skill trips a route-resolution
//!   finding and fails the run;
//! - no declared edge at all, no graph runs (the floor-only outcome is unchanged).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::drift::{
    self, AssemblyFactRow, ClauseRow, Declarations, DegreeBoundRow, EdgeBoundRow, EmitOptions,
    Payload, RequirementRow,
};

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-graph-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A floor-clean skill named `name` (matching its directory, a lowercase slug, a
/// present description). Clean against the floor, so the only finding a case can
/// produce is a graph one.
fn clean_skill(name: &str) -> String {
    format!(
        "---\n\
         name: {name}\n\
         description: Use when {name} is the task at hand; not for anything else.\n\
         ---\n\
         # {name}\n\
         \n\
         Body.\n"
    )
}

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
/// members live off harness disk (`specs/architecture/20-surface.md`, "The lock and
/// drift").
fn write_harness(root: &Path, rule_name: &str, rule_md: &str, skill_name: &str, skill_md: &str) {
    let rules = root.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(rules.join(format!("{rule_name}.md")), rule_md).unwrap();

    let skill_dir = root.join(".claude").join("skills").join(skill_name);
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(skill_dir.join("SKILL.md"), skill_md).unwrap();
}

/// The outcome of a `check` run: whether it exited zero and its combined
/// stdout+stderr (diagnostics render to stdout, a load error to stderr).
struct CheckRun {
    ok: bool,
    output: String,
}

/// Run `temper check` from `root` against the default `./.temper` workspace,
/// capturing the result.
fn check_in(root: &Path) -> CheckRun {
    let out = Command::new(BIN)
        .current_dir(root)
        .arg("check")
        .output()
        .unwrap();
    let mut output = String::from_utf8_lossy(&out.stdout).into_owned();
    output.push_str(&String::from_utf8_lossy(&out.stderr));
    CheckRun {
        ok: out.status.success(),
        output,
    }
}

/// The retired manifest's filename, spelled by concatenation so the retired token
/// itself never appears as a literal in this source (`specs/architecture/20-surface.md`,
/// "the name … retires with the manifest era entirely").
fn retired_manifest_name() -> String {
    format!("temper{}toml", '.')
}

/// Write the retired manifest verbatim at the project root — the filename is inert
/// (never read by any verb), so every case using this proves exactly that: the file
/// changes nothing, whatever it carries.
fn write_retired_manifest(root: &Path, contents: &str) {
    fs::write(root.join(retired_manifest_name()), contents).unwrap();
}

/// Compile a golden lock at `<root>/.temper/lock.toml` carrying just `declarations` —
/// the SDK-emitted fixture standing in for `import::run`'s scratch projection of a
/// manifest's `[[kind.<name>.relationships]]`/`[requirement.*]` table: the gate
/// sources edges and requirements from the lock, never a re-imported assembly
/// (`specs/architecture/20-surface.md`, "The lock and drift — one vocabulary").
fn write_lock(root: &Path, declarations: Declarations) {
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations,
        members: Vec::new(),
    };
    drift::emit(&payload, &root.join(".temper"), EmitOptions::default()).unwrap();
}

/// An `edge` assembly fact — the lock row a `[[kind.<from>.relationships]]` table used
/// to project (`specs/architecture/45-governance.md`, "The harness is a graph too").
fn edge(from: &str, field: &str, to: &str) -> AssemblyFactRow {
    AssemblyFactRow {
        fact: "edge".to_string(),
        value: None,
        from: Some(from.to_string()),
        field: Some(field.to_string()),
        to: Some(to.to_string()),
    }
}

/// The `gate` requirement's declaration row, bound to `kind` and carrying a required
/// `degree` clause — the lock row a `[requirement.gate]` table used to project.
fn degree_requirement(kind: &str, degree: DegreeBoundRow) -> RequirementRow {
    RequirementRow {
        name: "gate".to_string(),
        kind: Some(kind.to_string()),
        required: false,
        clauses: vec![ClauseRow {
            kind: None,
            predicate: "degree".to_string(),
            field: None,
            severity: "required".to_string(),
            count: None,
            target: None,
            degree: Some(degree),
        }],
        verified_by: None,
    }
}

/// Author a member's `satisfies` links on its surface overlay
/// (`<root>/.temper/<kind_dir>/<name>/<doc>`) — the projected document a live off-disk
/// walk grafts a member's fill edges from (`specs/architecture/20-surface.md`, "The lock
/// and drift"); the real harness file itself carries no temper annotation. `kind_dir` is
/// the surface subdirectory (`skills` or `rules`), whose document is `SKILL.md` /
/// `RULE.md`, and whose real source lives at the harness's own locus.
fn author_satisfies(root: &Path, kind_dir: &str, name: &str, requirements: &[&str]) {
    let satisfies: Vec<temper::document::Satisfies> = requirements
        .iter()
        .map(|r| temper::document::Satisfies::new(*r))
        .collect();
    match kind_dir {
        "skills" => {
            let kind = temper::builtin_kind::definition("skill").unwrap().unwrap();
            let source = root
                .join(".claude")
                .join("skills")
                .join(name)
                .join("SKILL.md");
            let mut skill = temper::frontmatter::Member::from_source(&kind, &source).unwrap();
            skill.satisfies = satisfies;
            let dir = root.join(".temper").join("skills").join(name);
            fs::create_dir_all(&dir).unwrap();
            fs::write(dir.join("SKILL.md"), skill.to_document().emit()).unwrap();
        }
        "rules" => {
            let kind = temper::builtin_kind::definition("rule").unwrap().unwrap();
            let source = root
                .join(".claude")
                .join("rules")
                .join(format!("{name}.md"));
            let mut rule = temper::frontmatter::Member::from_source(&kind, &source).unwrap();
            rule.satisfies = satisfies;
            let dir = root.join(".temper").join("rules").join(name);
            fs::create_dir_all(&dir).unwrap();
            fs::write(dir.join("RULE.md"), rule.to_document().emit()).unwrap();
        }
        other => panic!("unknown kind_dir {other}"),
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
/// kind capability (`specs/architecture/15-kinds.md`).
fn routes_to_edge() -> Vec<AssemblyFactRow> {
    vec![edge("rule", "routes_to", "skill")]
}

#[test]
fn a_resolving_route_is_clean() {
    let root = tmpdir("resolves");
    // The rule routes to `standards`, which the skill provides — the route resolves,
    // so the whole run is clean.
    write_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &clean_skill("standards"),
    );
    write_lock(
        &root,
        Declarations {
            assembly: routes_to_edge(),
            ..Declarations::default()
        },
    );

    let run = check_in(&root);
    assert!(
        run.ok,
        "a declared route that resolves to a real skill passes ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_dangling_route_fails_the_run_with_a_route_resolution_finding() {
    let root = tmpdir("dangling");
    // The rule routes to `absent`, but the only skill is `standards` — the route
    // resolves to no artifact, a dangling route that fails the run.
    write_harness(
        &root,
        "style",
        &routing_rule("absent"),
        "standards",
        &clean_skill("standards"),
    );
    write_lock(
        &root,
        Declarations {
            assembly: routes_to_edge(),
            ..Declarations::default()
        },
    );

    let run = check_in(&root);
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
    let root = tmpdir("no-edge");
    // The same corpus with a dangling `routes_to`, but no declared edge at all: no
    // graph runs and the (floor-clean) corpus passes. The reference is a declared
    // *contract*, never inferred — with none declared, temper says nothing about the
    // route.
    write_harness(
        &root,
        "style",
        &routing_rule("absent"),
        "standards",
        &clean_skill("standards"),
    );

    let absent = check_in(&root);
    assert!(
        absent.ok,
        "with no declared edge the graph does not run ⇒ zero, got:\n{}",
        absent.output
    );

    // A stray retired manifest carrying a `[kind]` layer — never read, so it declares
    // no lock edge either — runs no graph: the outcome is byte-for-byte the floor's.
    write_retired_manifest(&root, "[kind.skill]\npackage = \"skill.anthropic\"\n");
    let no_edge = check_in(&root);
    assert!(no_edge.ok, "an empty graph changes nothing ⇒ still zero");
    assert_eq!(
        absent.output, no_edge.output,
        "a stray manifest must produce identical output to none"
    );
}

#[test]
fn an_acyclic_reference_graph_passes() {
    let root = tmpdir("acyclic");
    // `rule style → skill standards`, but the skill routes nowhere — even with both
    // edge kinds declared, the graph is a DAG, so `acyclic` is clean.
    write_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &clean_skill("standards"),
    );
    write_lock(
        &root,
        Declarations {
            assembly: mutual_routes_edges(),
            ..Declarations::default()
        },
    );

    let run = check_in(&root);
    assert!(
        run.ok,
        "an acyclic reference graph passes ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_cyclic_reference_graph_fails_the_run() {
    let root = tmpdir("cyclic");
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
    write_lock(
        &root,
        Declarations {
            assembly: mutual_routes_edges(),
            ..Declarations::default()
        },
    );

    let run = check_in(&root);
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
    let root = tmpdir("degree-self-reg-fires");
    // The rule `style` routes to the skill `standards`, so `standards` has incoming
    // degree 1. A requirement declaring the skill self-registering (`incoming = { max = 0 }`,
    // "must not be pointed at") is violated — the run fails on the degree finding.
    write_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &clean_skill("standards"),
    );
    // The skill `standards` opts into `gate`, placing it in the degree bound's
    // satisfier set.
    author_satisfies(&root, "skills", "standards", &["gate"]);
    write_lock(
        &root,
        Declarations {
            assembly: routes_to_edge(),
            requirements: vec![degree_requirement(
                "skill",
                DegreeBoundRow {
                    incoming: Some(edge_bound(None, Some(0))),
                    outgoing: None,
                },
            )],
            ..Declarations::default()
        },
    );

    let run = check_in(&root);
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
    let root = tmpdir("degree-self-reg-passes");
    // Same edge and harness, but the bound ranges over the *rule* `style`: nothing
    // points at the rule (the only edge is rule → skill), so its incoming degree is
    // zero — inside `incoming = { max = 0 }`, and the run is clean.
    write_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &clean_skill("standards"),
    );
    // The rule `style` opts into `gate`, so the bound ranges over it.
    author_satisfies(&root, "rules", "style", &["gate"]);
    write_lock(
        &root,
        Declarations {
            assembly: routes_to_edge(),
            requirements: vec![degree_requirement(
                "rule",
                DegreeBoundRow {
                    incoming: Some(edge_bound(None, Some(0))),
                    outgoing: None,
                },
            )],
            ..Declarations::default()
        },
    );

    let run = check_in(&root);
    assert!(
        run.ok,
        "a self-registering rule that nothing points at passes ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_routed_degree_bound_passes_when_the_node_is_reachable() {
    let root = tmpdir("degree-routed-passes");
    // The rule routes to `standards`, so the skill has incoming degree 1 — inside the
    // open-above routed bound `incoming = { min = 1 }` ("must be reachable"). Clean.
    write_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &clean_skill("standards"),
    );
    // The skill `standards` opts into `gate`, so the routed bound ranges over it.
    author_satisfies(&root, "skills", "standards", &["gate"]);
    write_lock(
        &root,
        Declarations {
            assembly: routes_to_edge(),
            requirements: vec![degree_requirement(
                "skill",
                DegreeBoundRow {
                    incoming: Some(edge_bound(Some(1), None)),
                    outgoing: None,
                },
            )],
            ..Declarations::default()
        },
    );

    let run = check_in(&root);
    assert!(
        run.ok,
        "a routed skill that a rule reaches passes ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_routed_degree_bound_fires_when_the_node_is_unreachable() {
    let root = tmpdir("degree-routed-fires");
    // The bound ranges over the *rule* `style` and requires it reachable (`incoming =
    // { min = 1 }`), but nothing points at the rule (the only edge is rule → skill),
    // so its incoming degree is zero — outside the bound. The run fails on degree.
    write_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &clean_skill("standards"),
    );
    // The rule `style` opts into `gate`, so the routed bound ranges over it.
    author_satisfies(&root, "rules", "style", &["gate"]);
    write_lock(
        &root,
        Declarations {
            assembly: routes_to_edge(),
            requirements: vec![degree_requirement(
                "rule",
                DegreeBoundRow {
                    incoming: Some(edge_bound(Some(1), None)),
                    outgoing: None,
                },
            )],
            ..Declarations::default()
        },
    );

    let run = check_in(&root);
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

/// Library-level fixture proof of the `reachable` predicate (`specs/architecture/45-governance.md`,
/// "The world is a node — reachability is a predicate"): the pure machinery over
/// constructed `Features`, including a caller-declared severity threaded into the
/// finding. The dial that once wired this into the gate retired
/// (`specs/architecture/40-composition.md`, "the reachability dial retires into the clause
/// algebra"); the predicate itself stays a live capability for a future edge-scope
/// clause to call.
mod reachability {
    use std::collections::BTreeMap;

    use temper::check::Severity;
    use temper::extract::{FeatureValue, Features, Kind};
    use temper::graph::{ResolvedEdge, reachable};
    use temper::kind::Activation;

    /// A member carrying an id and, optionally, one frontmatter field — the only inputs
    /// the reachability predicate reads (the id for the finding, the named activation
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
            genres: Vec::new(),
            satisfies: Vec::new(),
            published_requirements: Vec::new(),
        }
    }

    /// A `description-trigger` activation over the named field.
    fn description_trigger(field: &str) -> Activation {
        Activation::DescriptionTrigger {
            field: field.to_string(),
        }
    }

    /// A `paths-match` activation over the named field.
    fn paths_match(field: &str) -> Activation {
        Activation::PathsMatch {
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
    fn a_live_activation_edge_is_reachable() {
        // A skill with a non-empty `description` (a live description-trigger) and a rule
        // whose `paths` glob matches a repo file (a live paths-match) each have a live
        // inbound edge from the world — nothing fires.
        let skills = [member(
            "standards",
            Some((
                "description",
                FeatureValue::scalar(Kind::String, "Use when styling the code."),
            )),
        )];
        let rules = [member(
            "style",
            Some(("paths", FeatureValue::scalar(Kind::String, "src/**/*.rs"))),
        )];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("skill", &skills[..]), ("rule", &rules[..])]);
        let activations = BTreeMap::from([
            ("skill", description_trigger("description")),
            ("rule", paths_match("paths")),
        ]);
        let files = vec!["src/graph.rs".to_string()];
        assert!(reachable(&activations, &by_kind, &files, &[], Severity::Error).is_empty());
    }

    #[test]
    fn a_blank_description_trigger_field_is_unreachable() {
        // The skill declares a description-trigger on `description`, but the field is
        // whitespace-only — the harness has nothing to load, a dead inbound edge.
        let skills = [member(
            "standards",
            Some(("description", FeatureValue::scalar(Kind::String, "   "))),
        )];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        let activations = BTreeMap::from([("skill", description_trigger("description"))]);

        let diags = reachable(&activations, &by_kind, &[], &[], Severity::Error);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, "graph.reachable");
        assert_eq!(diags[0].artifact, "standards");
        assert!(diags[0].message.contains("description"));
        assert!(diags[0].message.contains("world"));

        // The dial is the assembly's: the same dead edge at `advisory` is a warn, so a
        // required-vs-advisory reachability declaration is honored (REACHABILITY-WIRE).
        let advisory = reachable(&activations, &by_kind, &[], &[], Severity::Warn);
        assert_eq!(advisory.len(), 1);
        assert_eq!(advisory[0].severity, Severity::Warn);
    }

    #[test]
    fn a_zero_match_paths_glob_is_unreachable() {
        // The rule declares a paths-match on `paths`, but its glob matches no file in
        // the supplied repo file-set — the harness activates it never.
        let rules = [member(
            "style",
            Some(("paths", FeatureValue::scalar(Kind::String, "docs/**/*.md"))),
        )];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("rule", &rules[..])]);
        let activations = BTreeMap::from([("rule", paths_match("paths"))]);
        let files = vec!["src/graph.rs".to_string(), "README.md".to_string()];

        let diags = reachable(&activations, &by_kind, &files, &[], Severity::Error);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "graph.reachable");
        assert_eq!(diags[0].artifact, "style");
        assert!(diags[0].message.contains("paths"));
        assert!(diags[0].message.contains("world"));
    }

    #[test]
    fn an_absent_or_blank_paths_field_is_reachable() {
        // An unscoped rule declares a paths-match activation but carries no `paths` field
        // (or a whitespace-only one) — the harness falls back to unconditional loading
        // (specs/architecture/15-kinds.md paths-match bullet), so the inbound edge is live, not dead.
        let absent = member("global", None);
        let blank = member(
            "blank",
            Some(("paths", FeatureValue::scalar(Kind::String, "   "))),
        );
        let rules = [absent, blank];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("rule", &rules[..])]);
        let activations = BTreeMap::from([("rule", paths_match("paths"))]);
        // A non-empty repo file-set the absent/blank field is *not* tested against.
        let files = vec!["src/graph.rs".to_string()];

        assert!(reachable(&activations, &by_kind, &files, &[], Severity::Error).is_empty());
    }

    #[test]
    fn a_kind_that_declares_no_activation_is_not_subject() {
        // The corpus holds a skill with a blank `description`, but no kind declares an
        // activation (the map is empty) — the predicate ranges over declared edges only,
        // so nothing fires. `temper` never invents an edge the kind did not declare.
        let skills = [member(
            "standards",
            Some(("description", FeatureValue::scalar(Kind::String, ""))),
        )];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        let activations: BTreeMap<&str, Activation> = BTreeMap::new();

        assert!(reachable(&activations, &by_kind, &[], &[], Severity::Error).is_empty());
    }

    #[test]
    fn a_dead_own_member_imported_by_a_reachable_member_is_live() {
        // The rule `scoped` has a zero-match `paths` glob — its own world-edge is dead.
        // But the memory member `root` declares no activation (unconditionally live) and
        // imports it, so the closure carries `root`'s liveness across the directive edge:
        // `scoped` is reachable, and no finding fires. This is the false positive the
        // slice fixes.
        let memories = [member("root", None)];
        let rules = [member(
            "scoped",
            Some((
                "paths",
                FeatureValue::scalar(Kind::String, "nowhere/**/*.md"),
            )),
        )];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("memory", &memories[..]), ("rule", &rules[..])]);
        // Only `rule` declares an activation; `memory` is absent ⇒ always live.
        let activations = BTreeMap::from([("rule", paths_match("paths"))]);
        let edges = [import_edge(("memory", "root"), ("rule", "scoped"))];
        let files = vec!["src/graph.rs".to_string()];

        assert!(reachable(&activations, &by_kind, &files, &edges, Severity::Error).is_empty());
    }

    #[test]
    fn a_member_imported_only_by_a_dead_member_stays_dead() {
        // Both skills have a blank `description` — dead own-edges. `importer` imports
        // `target`, but a dead importer carries no liveness, so `target` is *not* rescued:
        // both fire. Liveness inherits only from a *reachable* importer.
        let skills = [
            member(
                "importer",
                Some(("description", FeatureValue::scalar(Kind::String, "   "))),
            ),
            member(
                "target",
                Some(("description", FeatureValue::scalar(Kind::String, "   "))),
            ),
        ];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        let activations = BTreeMap::from([("skill", description_trigger("description"))]);
        let edges = [import_edge(("skill", "importer"), ("skill", "target"))];

        let diags = reachable(&activations, &by_kind, &[], &edges, Severity::Error);
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
                        FeatureValue::scalar(Kind::String, "nowhere/**/*.md"),
                    )),
                )
            })
            .collect();
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("memory", &memories[..]), ("rule", &rules[..])]);
        let activations = BTreeMap::from([("rule", paths_match("paths"))]);
        let mut edges = vec![import_edge(("memory", "root"), ("rule", "r1"))];
        for n in 1..=5 {
            edges.push(import_edge(
                ("rule", &format!("r{n}")),
                ("rule", &format!("r{}", n + 1)),
            ));
        }
        let files = vec!["src/graph.rs".to_string()];

        let diags = reachable(&activations, &by_kind, &files, &edges, Severity::Error);
        assert_eq!(diags.len(), 1, "only the past-cap member stays dead");
        assert_eq!(diags[0].artifact, "r6");
    }
}
