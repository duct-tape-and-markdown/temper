//! End-to-end acceptance for the **wired** `reachable` predicate
//! (`specs/architecture/45-governance.md`, "The world is a node — reachability is a predicate").
//!
//! The library fixture (`tests/graph.rs`'s `reachability` module) proves the predicate
//! over constructed `Features`; this drives the built binary so the whole gate path is
//! pinned: a harness whose kinds declare an activation (the built-in `skill`'s
//! description-trigger, the `rule`'s paths-match) written straight at their real Claude
//! Code locus, reading the assembly's `reachability` opt-in + severity off a golden lock
//! (`specs/architecture/20-surface.md`, "The lock and drift — one vocabulary" — the gate
//! sources the opt-in from the lock, never a re-imported `temper.toml`), scanning the
//! real repo file-set for the paths-match liveness input, and the exit code.
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

use temper::drift::{self, AssemblyFactRow, Declarations, EmitOptions, Payload};

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

/// Write a harness of the given skills and rules straight at their real Claude Code
/// locus — no scratch import — each skill under `.claude/skills/<name>/SKILL.md`, each
/// rule under `.claude/rules/<name>.md`. `check` reads built-in kind members live off
/// harness disk (`specs/architecture/20-surface.md`, "The lock and drift").
fn write_harness(root: &Path, skills: &[(&str, String)], rules: &[(&str, String)]) {
    let skills_root = root.join(".claude").join("skills");
    fs::create_dir_all(&skills_root).unwrap();
    for (name, md) in skills {
        let dir = skills_root.join(name);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("SKILL.md"), md).unwrap();
    }

    let rules_root = root.join(".claude").join("rules");
    fs::create_dir_all(&rules_root).unwrap();
    for (name, md) in rules {
        fs::write(rules_root.join(format!("{name}.md")), md).unwrap();
    }
}

/// Compile a golden lock at `<root>/.temper/lock.toml` carrying just `declarations` —
/// the SDK-emitted fixture standing in for `import::run`'s scratch projection of a
/// `temper.toml` `[reachability]` table: the gate sources the opt-in from the lock,
/// never a re-imported assembly (`specs/architecture/20-surface.md`, "The lock and
/// drift — one vocabulary").
fn write_lock(root: &Path, declarations: Declarations) {
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations,
        members: Vec::new(),
    };
    drift::emit(&payload, &root.join(".temper"), EmitOptions::default()).unwrap();
}

/// The assembly's `reachability` opt-in at `severity`, as a golden lock.
fn write_reachability(root: &Path, severity: &str) {
    write_lock(
        root,
        Declarations {
            assembly: vec![AssemblyFactRow {
                fact: "reachability".to_string(),
                value: Some(severity.to_string()),
                from: None,
                field: None,
                to: None,
            }],
            ..Declarations::default()
        },
    );
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

/// Write `<root>/temper.toml` verbatim, with no resync: the reachability opt-in rides
/// the lock (`write_reachability`), so this is only for the assembly-scope facets
/// `temper.toml` still carries (a `[kind.*]` package registration, …) — and, written
/// empty, is enough to flip the assembly from absent to present.
fn write_temper_toml(root: &Path, contents: &str) {
    fs::write(root.join("temper.toml"), contents).unwrap();
}

#[test]
fn a_dead_description_trigger_fires_at_the_declared_required_severity() {
    let root = tmpdir("dead-desc-required");
    // The skill `standards` is floor-clean but its description is whitespace-only — a
    // dead description-trigger. The assembly opts reachability in at `required`, so the
    // dead world→member edge fails the run.
    write_harness(
        &root,
        &[("standards", blank_description_skill("standards"))],
        &[],
    );
    write_reachability(&root, "required");
    write_temper_toml(&root, "");

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
    write_harness(
        &root,
        &[("standards", blank_description_skill("standards"))],
        &[],
    );
    write_reachability(&root, "advisory");
    write_temper_toml(&root, "");

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
    write_harness(&root, &[], &[("scoped", paths_rule("nowhere/**/*.md"))]);
    write_reachability(&root, "required");
    write_temper_toml(&root, "");

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
    write_harness(
        &root,
        &[("standards", live_skill("standards"))],
        &[("global", unscoped_rule())],
    );
    write_reachability(&root, "required");
    write_temper_toml(&root, "");

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
    write_harness(
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

// The directive-import reachability rescue (a dead-own member reached by a live
// importer stays silent) is retired at the CLI level along with custom kinds
// (`specs/architecture/15-kinds.md`, "Decision: field typing lives in the SDK —
// there is no kind file format"): the only built-in kind that composes the
// `at-import` directive is `memory`, which the gate's graph corpus does not yet
// fold in (a separate, pre-existing scope gap — `src/main.rs`,
// `assemble_by_kind`), so no built-in kind can stand in for the custom importer
// this end-to-end case drove. The rescue mechanism itself stays proven at the
// unit level in `tests/graph.rs`'s `reachability` module.
