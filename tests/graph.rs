//! End-to-end acceptance over the harness reference graph — route resolution
//! against a `temper.toml`-declared edge field (`specs/45-governance.md`, "The
//! harness is a graph too — and references are declared edges").
//!
//! Drives the built `temper` binary so the whole path is pinned: importing a
//! harness of a rule (carrying a `routes_to` frontmatter field) and a skill,
//! discovering `temper.toml` at the project root, parsing its `[[edge]]`
//! declaration onto the author layer, building the graph over the imported
//! corpus, and the exit code.
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

    let skill_dir = harness.join("skills").join(skill_name);
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

/// A `temper.toml` declaring one `routes_to` edge from rules to skills — the
/// harness reference graph the cases build.
const ROUTES_TO_EDGE: &str = "[[edge]]\n\
     field = \"routes_to\"\n\
     from = \"rule\"\n\
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

    // A `temper.toml` carrying a `[kind]` layer but no `[[edge]]` declares no graph
    // either — the outcome is byte-for-byte the floor's.
    write_temper_toml(
        &root,
        "[kind.skill]\n\
         adopt = \"skill.anthropic\"\n",
    );
    let no_edge = check_in(&root);
    assert!(no_edge.ok, "an empty graph changes nothing ⇒ still zero");
    assert_eq!(
        absent.output, no_edge.output,
        "a temper.toml declaring no edge must produce identical output to none"
    );
}
