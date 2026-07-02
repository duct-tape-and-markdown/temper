//! End-to-end acceptance for the **wired** `reachable` predicate
//! (`specs/45-governance.md`, "The world is a node — reachability is a predicate").
//!
//! The library fixture (`tests/graph.rs`'s `reachability` module) proves the predicate
//! over constructed `Features`; this drives the built binary so the whole gate path is
//! pinned: importing a harness whose kinds declare an activation (the built-in `skill`'s
//! description-trigger, the `rule`'s paths-match), reading the assembly's `[reachability]`
//! opt-in + severity off `temper.toml`, scanning the real repo file-set for the
//! paths-match liveness input, and the exit code.
//!
//! The cases mirror the entry's acceptance:
//! - a member whose declared activation edge is provably dead (a blank
//!   description-trigger, a zero-match paths glob) is a finding at the assembly's
//!   declared severity — `required` fails the run, `advisory` reports without failing;
//! - a live edge (a real description, an unscoped rule with no `paths`) stays silent;
//! - absent the `[reachability]` opt-in, a dead edge fires nothing at all;
//! - the finding names the world node, the kind, the member, and the dead-edge reason.

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
        "reachable-gate-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A floor-clean skill (name matching its directory, a present description) whose
/// description-trigger world-edge is **live**.
fn live_skill(name: &str) -> String {
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

/// A floor-clean skill whose `description` is whitespace-only: present and non-empty
/// (so the floor's `required`/`min_len` clauses pass) yet **blank** once trimmed — a
/// dead description-trigger world-edge, the harness has nothing to load. The only
/// finding a case can produce is the reachability one.
fn blank_description_skill(name: &str) -> String {
    format!(
        "---\n\
         name: {name}\n\
         description: \"   \"\n\
         ---\n\
         # {name}\n\
         \n\
         Body.\n"
    )
}

/// A floor-clean rule scoped to `glob` via `paths` — a paths-match world-edge, live
/// only if the glob matches a repo file. `paths` is the rule kind's one documented key,
/// so the rule stays clean and the only finding a case can produce is the reachability
/// one.
fn paths_rule(glob: &str) -> String {
    format!(
        "---\n\
         paths: \"{glob}\"\n\
         ---\n\
         # Scoped\n\
         \n\
         Body.\n"
    )
}

/// A floor-clean rule with no frontmatter — an unscoped rule the harness loads
/// unconditionally (a live `always`-shaped edge, post-PATHS-MATCH-ABSENCE: an absent
/// `paths` field is not a dead edge).
fn unscoped_rule() -> String {
    "# Global\n\nAlways-on guidance.\n".to_string()
}

/// Import a harness of the given skills and rules into `<root>/.temper` via the real
/// `import` verb, so the workspace `check` reads is built exactly as a user's would be —
/// each skill under `.claude/skills/<name>/SKILL.md`, each rule under
/// `.claude/rules/<name>.md`. Both surface roots are always created so `import` scans a
/// well-formed tree even when one kind is empty.
fn import_harness(root: &Path, skills: &[(&str, String)], rules: &[(&str, String)]) {
    let harness = tmpdir("harness");

    let skills_root = harness.join(".claude").join("skills");
    fs::create_dir_all(&skills_root).unwrap();
    for (name, md) in skills {
        let dir = skills_root.join(name);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("SKILL.md"), md).unwrap();
    }

    let rules_root = harness.join(".claude").join("rules");
    fs::create_dir_all(&rules_root).unwrap();
    for (name, md) in rules {
        fs::write(rules_root.join(format!("{name}.md")), md).unwrap();
    }

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
/// stdout+stderr.
struct CheckRun {
    ok: bool,
    output: String,
}

/// Run `temper check` from `root` (so a `temper.toml` there is discovered, and its
/// parent is the repo root the paths-match glob-set is scanned from) against the
/// default `./.temper` workspace.
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

/// The assembly's reachability opt-in at the given severity.
fn reachability_toml(severity: &str) -> String {
    format!("[reachability]\nseverity = \"{severity}\"\n")
}

#[test]
fn a_dead_description_trigger_fires_at_the_declared_required_severity() {
    let root = tmpdir("dead-desc-required");
    // The skill `standards` is floor-clean but its description is whitespace-only — a
    // dead description-trigger. The assembly opts reachability in at `required`, so the
    // dead world→member edge fails the run.
    import_harness(
        &root,
        &[("standards", blank_description_skill("standards"))],
        &[],
    );
    write_temper_toml(&root, &reachability_toml("required"));

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a dead activation edge at `required` severity must fail the run ⇒ non-zero, got:\n{}",
        run.output
    );
    // The finding names the world node, the kind, the member, and the dead-edge reason.
    assert!(
        run.output.contains("world")
            && run.output.contains("skill")
            && run.output.contains("standards")
            && run.output.contains("description"),
        "the finding names the world, the kind, the member, and the dead-edge reason, got:\n{}",
        run.output
    );
}

#[test]
fn a_dead_edge_at_advisory_severity_is_reported_but_does_not_fail() {
    let root = tmpdir("dead-desc-advisory");
    // The same dead description-trigger, but the assembly declares `advisory`: the dial
    // is the assembly's, so the finding is reported yet the run stays green — the
    // required-vs-advisory reachability declaration is honored.
    import_harness(
        &root,
        &[("standards", blank_description_skill("standards"))],
        &[],
    );
    write_temper_toml(&root, &reachability_toml("advisory"));

    let run = check_in(&root);
    assert!(
        run.ok,
        "a dead activation edge at `advisory` severity is reported but does not fail ⇒ zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("world") && run.output.contains("standards"),
        "the advisory finding is still reported, naming the world and the member, got:\n{}",
        run.output
    );
}

#[test]
fn a_zero_match_paths_glob_rule_fires() {
    let root = tmpdir("dead-paths");
    // The rule `scoped` declares a `paths` glob matching no file under the repo root
    // (only `temper.toml` and the imported `.temper/` live there) — the harness
    // activates it never, a dead paths-match edge that fails the `required` run.
    import_harness(&root, &[], &[("scoped", paths_rule("nowhere/**/*.md"))]);
    write_temper_toml(&root, &reachability_toml("required"));

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a zero-match paths glob is a dead edge that must fail the run ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("world")
            && run.output.contains("rule")
            && run.output.contains("scoped")
            && run.output.contains("paths"),
        "the finding names the world, the kind, the member, and the dead paths edge, got:\n{}",
        run.output
    );
}

#[test]
fn a_live_edge_stays_silent() {
    let root = tmpdir("live");
    // A skill with a real description (a live description-trigger) and an unscoped rule
    // with no `paths` (a live `always`-shaped edge) — both inbound world-edges are live,
    // so reachability fires nothing even with the opt-in armed at `required`.
    import_harness(
        &root,
        &[("standards", live_skill("standards"))],
        &[("global", unscoped_rule())],
    );
    write_temper_toml(&root, &reachability_toml("required"));

    let run = check_in(&root);
    assert!(
        run.ok,
        "a harness whose activation edges are all live passes ⇒ zero, got:\n{}",
        run.output
    );
    assert!(
        !run.output.contains("graph.reachable"),
        "no reachability finding fires on a live harness, got:\n{}",
        run.output
    );
}

#[test]
fn absent_the_opt_in_a_dead_edge_is_silent() {
    let root = tmpdir("no-opt-in");
    // The same dead description-trigger skill, but the `temper.toml` declares a benign
    // kind layer and *no* `[reachability]`: the predicate is opt-in like `degree`, so
    // without the assembly's declaration nothing fires — temper fabricates no gate the
    // author did not declare.
    import_harness(
        &root,
        &[("standards", blank_description_skill("standards"))],
        &[],
    );
    write_temper_toml(&root, "[kind.skill]\npackage = \"skill.anthropic\"\n");

    let run = check_in(&root);
    assert!(
        run.ok,
        "absent the reachability opt-in a dead edge is silent ⇒ zero, got:\n{}",
        run.output
    );
    assert!(
        !run.output.contains("graph.reachable"),
        "the reachability predicate does not run without the assembly opt-in, got:\n{}",
        run.output
    );
}
