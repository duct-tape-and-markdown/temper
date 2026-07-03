//! End-to-end acceptance over the harness reference graph — route resolution
//! against a `temper.toml`-declared edge field (`specs/architecture/45-governance.md`, "The
//! harness is a graph too — and references are declared edges").
//!
//! Drives the built `temper` binary so the whole path is pinned: importing a
//! harness of a rule (carrying a `routes_to` frontmatter field) and a skill,
//! discovering `temper.toml` at the project root, parsing its
//! `[[kind.<name>.relationships]]` declaration onto the author layer, building the
//! graph over the imported corpus, and the exit code.
//!
//! The cases mirror the entry's acceptance:
//! - a rule whose `routes_to` names a real skill resolves and the run is clean;
//! - a rule whose `routes_to` names an absent skill trips a route-resolution
//!   finding and fails the run;
//! - absent `temper.toml`, no graph runs (the floor-only outcome is unchanged).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

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

/// Import a harness of one rule and one skill into `<root>/.temper` via the real
/// `import` verb, so the workspace `check` reads is built exactly as a user's
/// would be — the rule under `.claude/rules/<rule>.md`, the skill under
/// `skills/<skill>/SKILL.md`.
fn import_harness(root: &Path, rule_name: &str, rule_md: &str, skill_name: &str, skill_md: &str) {
    let harness = tmpdir("harness");

    let rules = harness.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(rules.join(format!("{rule_name}.md")), rule_md).unwrap();

    let skill_dir = harness.join(".claude").join("skills").join(skill_name);
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(skill_dir.join("SKILL.md"), skill_md).unwrap();

    let status = Command::new(BIN)
        .arg("import")
        .arg(&harness)
        .arg("--into")
        .arg(root.join(".temper"))
        .status()
        .unwrap();
    assert!(status.success(), "import should succeed: {status}");
}

/// The outcome of a `check` run: whether it exited zero and its combined
/// stdout+stderr (diagnostics render to stdout, a load error to stderr).
struct CheckRun {
    ok: bool,
    output: String,
}

/// Run `temper check` from `root` (so a `temper.toml` there is discovered) against
/// the default `./.temper` workspace, capturing the result.
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

/// Write `<root>/temper.toml`.
fn write_temper_toml(root: &Path, contents: &str) {
    fs::write(root.join("temper.toml"), contents).unwrap();
}

/// Author the `[satisfies.<requirement>]` opt-in modules on an imported artifact's
/// surface member document — the binding the roster and graph read to place a node in
/// a requirement's satisfier set. `kind_dir` is the surface subdirectory (`skills` or
/// `rules`), whose document is `SKILL.md` / `RULE.md`. `import` never writes them
/// (they are surface-authored, not frontmatter), so a case adds them exactly as a
/// human editing the member document would, via the same projection the tool uses.
fn author_satisfies(root: &Path, kind_dir: &str, name: &str, requirements: &[&str]) {
    let dir = root.join(".temper").join(kind_dir).join(name);
    let satisfies: Vec<temper::document::Satisfies> = requirements
        .iter()
        .map(|r| temper::document::Satisfies::new(*r))
        .collect();
    match kind_dir {
        "skills" => {
            let mut skill = temper::frontmatter::Member::from_surface(&dir, "SKILL.md").unwrap();
            skill.satisfies = satisfies;
            fs::write(dir.join("SKILL.md"), skill.to_document().emit()).unwrap();
        }
        "rules" => {
            let mut rule = temper::frontmatter::Member::from_surface(&dir, "RULE.md").unwrap();
            rule.satisfies = satisfies;
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

/// A `temper.toml` declaring `routes_to` on *both* the `rule` and `skill` kinds, so
/// the reference graph can carry a `rule → skill → rule` circle — the acyclic cases
/// build on these two edges.
const MUTUAL_ROUTES_EDGES: &str = "[[kind.rule.relationships]]\n\
     field = \"routes_to\"\n\
     to = \"skill\"\n\
     [[kind.skill.relationships]]\n\
     field = \"routes_to\"\n\
     to = \"rule\"\n";

/// A `temper.toml` declaring one `routes_to` relationship on the `rule` kind
/// (its owning kind the edge source), targeting skills — the harness reference
/// graph the cases build. A reference is a kind capability, declared under the
/// owning kind's `[[kind.<name>.relationships]]` array (`specs/architecture/15-kinds.md`).
const ROUTES_TO_EDGE: &str = "[[kind.rule.relationships]]\n\
     field = \"routes_to\"\n\
     to = \"skill\"\n";

#[test]
fn a_resolving_route_is_clean() {
    let root = tmpdir("resolves");
    // The rule routes to `standards`, which the imported skill provides — the
    // route resolves, so the whole run is clean.
    import_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &clean_skill("standards"),
    );
    write_temper_toml(&root, ROUTES_TO_EDGE);

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
    // The rule routes to `absent`, but the only imported skill is `standards` —
    // the route resolves to no artifact, a dangling route that fails the run.
    import_harness(
        &root,
        "style",
        &routing_rule("absent"),
        "standards",
        &clean_skill("standards"),
    );
    write_temper_toml(&root, ROUTES_TO_EDGE);

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
fn absent_temper_toml_runs_no_graph() {
    let root = tmpdir("no-edge");
    // The same corpus with a dangling `routes_to`, but no `temper.toml`: no edge is
    // declared, so no graph runs and the (floor-clean) corpus passes. The reference
    // is a declared *contract*, never inferred — with none declared, temper says
    // nothing about the route.
    import_harness(
        &root,
        "style",
        &routing_rule("absent"),
        "standards",
        &clean_skill("standards"),
    );

    let absent = check_in(&root);
    assert!(
        absent.ok,
        "with no `temper.toml` the graph does not run ⇒ zero, got:\n{}",
        absent.output
    );

    // A `temper.toml` carrying a `[kind]` layer but no `relationships` declares no
    // graph either — the outcome is byte-for-byte the floor's.
    write_temper_toml(
        &root,
        "[kind.skill]\n\
         package = \"skill.anthropic\"\n",
    );
    let no_edge = check_in(&root);
    assert!(no_edge.ok, "an empty graph changes nothing ⇒ still zero");
    assert_eq!(
        absent.output, no_edge.output,
        "a temper.toml declaring no edge must produce identical output to none"
    );
}

#[test]
fn an_acyclic_reference_graph_passes() {
    let root = tmpdir("acyclic");
    // `rule style → skill standards`, but the skill routes nowhere — even with both
    // edge kinds declared, the graph is a DAG, so `acyclic` is clean.
    import_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &clean_skill("standards"),
    );
    write_temper_toml(&root, MUTUAL_ROUTES_EDGES);

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
    import_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &routing_skill("standards", "style"),
    );
    write_temper_toml(&root, MUTUAL_ROUTES_EDGES);

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

/// A `temper.toml` declaring the `rule → skill` `routes_to` edge plus the `gate`
/// requirement carrying a `degree` bound (`clause`). The requirement binds no package —
/// the degree check reads the edge graph, not a contract — so the only finding a case
/// can produce is the degree one. `art` picks which kind the bound's satisfier nodes
/// come from; the node opts in via `satisfies = ["gate"]` (the case authors that with
/// [`author_satisfies`]).
fn degree_temper_toml(art: &str, clause: &str) -> String {
    format!(
        "[[kind.rule.relationships]]\n\
         field = \"routes_to\"\n\
         to = \"skill\"\n\
         [requirement.gate]\n\
         kind = \"{art}\"\n\
         degree = {{ {clause} }}\n"
    )
}

#[test]
fn a_self_registering_degree_bound_fires_when_the_node_is_pointed_at() {
    let root = tmpdir("degree-self-reg-fires");
    // The rule `style` routes to the skill `standards`, so `standards` has incoming
    // degree 1. A requirement declaring the skill self-registering (`incoming = { max = 0 }`,
    // "must not be pointed at") is violated — the run fails on the degree finding.
    import_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &clean_skill("standards"),
    );
    // The skill `standards` opts into `gate`, placing it in the degree bound's
    // satisfier set.
    author_satisfies(&root, "skills", "standards", &["gate"]);
    write_temper_toml(
        &root,
        &degree_temper_toml("skill", "incoming = { max = 0 }"),
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
    import_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &clean_skill("standards"),
    );
    // The rule `style` opts into `gate`, so the bound ranges over it.
    author_satisfies(&root, "rules", "style", &["gate"]);
    write_temper_toml(&root, &degree_temper_toml("rule", "incoming = { max = 0 }"));

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
    import_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &clean_skill("standards"),
    );
    // The skill `standards` opts into `gate`, so the routed bound ranges over it.
    author_satisfies(&root, "skills", "standards", &["gate"]);
    write_temper_toml(
        &root,
        &degree_temper_toml("skill", "incoming = { min = 1 }"),
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
    import_harness(
        &root,
        "style",
        &routing_rule("standards"),
        "standards",
        &clean_skill("standards"),
    );
    // The rule `style` opts into `gate`, so the routed bound ranges over it.
    author_satisfies(&root, "rules", "style", &["gate"]);
    write_temper_toml(&root, &degree_temper_toml("rule", "incoming = { min = 1 }"));

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
/// constructed `Features`, including the assembly-declared severity threaded into the
/// finding. The gate-side wiring — main.rs scanning the real repo file-set and reading
/// the assembly's `[reachability]` opt-in — is pinned end-to-end in
/// `tests/reachable_gate.rs`.
mod reachability {
    use std::collections::BTreeMap;

    use temper::check::Severity;
    use temper::extract::{FeatureValue, Features, Kind};
    use temper::graph::reachable;
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
        assert!(reachable(&activations, &by_kind, &files, Severity::Error).is_empty());
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

        let diags = reachable(&activations, &by_kind, &[], Severity::Error);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, "graph.reachable");
        assert_eq!(diags[0].artifact, "standards");
        assert!(diags[0].message.contains("description"));
        assert!(diags[0].message.contains("world"));

        // The dial is the assembly's: the same dead edge at `advisory` is a warn, so a
        // required-vs-advisory reachability declaration is honored (REACHABILITY-WIRE).
        let advisory = reachable(&activations, &by_kind, &[], Severity::Warn);
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

        let diags = reachable(&activations, &by_kind, &files, Severity::Error);
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

        assert!(reachable(&activations, &by_kind, &files, Severity::Error).is_empty());
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

        assert!(reachable(&activations, &by_kind, &[], Severity::Error).is_empty());
    }
}
