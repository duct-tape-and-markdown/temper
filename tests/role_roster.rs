//! End-to-end acceptance over the harness-contract roster — role match-selection
//! and single-filler conformance (`specs/10-contracts.md`, "Roles and matching").
//!
//! Drives the built `temper` binary so the whole path is pinned: `temper.toml`
//! discovery at the project root, parsing its `[role.<name>]` tables onto the
//! author layer, running selection over the imported skills, and the exit code.
//! Each case sets the working directory to a project root carrying a `temper.toml`
//! whose roster the imported skills do or do not satisfy.
//!
//! The cases mirror the entry's acceptance:
//! - a `required` role matching zero artifacts fails with a precise finding;
//! - exactly one match passes (the single-filler role is satisfied);
//! - two matches fail (a single-filler role needs exactly one);
//! - a non-`required` unfilled role is silent;
//! - a `temper.toml` declaring no roster leaves the floor outcome unchanged.

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
        "author-role-roster-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A floor-clean skill named `name` (matching its directory, a lowercase slug, a
/// present description), optionally declaring the `role:` marker it fills. Clean
/// against the floor, so the only finding a case can produce is a roster one.
fn clean_skill(name: &str, role_marker: Option<&str>) -> String {
    let role_line = role_marker.map_or(String::new(), |marker| format!("role: {marker}\n"));
    format!(
        "---\n\
         name: {name}\n\
         description: Use when {name} is the task at hand; not for anything else.\n\
         {role_line}\
         ---\n\
         # {name}\n\
         \n\
         Body.\n"
    )
}

/// Project a one-skill harness into `<root>/.temper` via the real `import` verb, so
/// the workspace `check` reads is built exactly as a user's would be. The surface
/// directory is the skill `name`, so the floor's `name-matches-dir` clause holds.
fn import_skill(root: &Path, name: &str, skill_md: &str) {
    let harness = tmpdir("harness");
    let dir = harness.join("skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("SKILL.md"), skill_md).unwrap();

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

/// A `temper.toml` declaring one `required`, name-glob, single-filler role over
/// the `skill` kind.
fn required_role_toml(glob: &str) -> String {
    format!(
        "[role.planner]\n\
         artifact = \"skill\"\n\
         contract = \"contracts/skill.anthropic.toml\"\n\
         match = {{ name = \"{glob}\" }}\n\
         required = true\n"
    )
}

#[test]
fn a_required_role_matching_zero_artifacts_fails_with_a_precise_finding() {
    let root = tmpdir("zero");
    // The only skill is floor-clean but does not match the role's `plan*` glob.
    import_skill(&root, "lint-rust", &clean_skill("lint-rust", None));
    write_temper_toml(&root, &required_role_toml("plan*"));

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a required role no artifact fills must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("planner") && run.output.contains("no `skill` artifact"),
        "the finding names the unfilled role and the kind it expected, got:\n{}",
        run.output
    );
}

#[test]
fn exactly_one_match_satisfies_the_single_filler_role() {
    let root = tmpdir("one");
    // One floor-clean skill matching `plan*`, and a non-matching clean skill —
    // exactly one filler, so the required single-filler role is satisfied and the
    // whole run is clean.
    import_skill(&root, "plan-tasks", &clean_skill("plan-tasks", None));
    import_skill(&root, "lint-rust", &clean_skill("lint-rust", None));
    write_temper_toml(&root, &required_role_toml("plan*"));

    let run = check_in(&root);
    assert!(
        run.ok,
        "exactly one filler satisfies the single-filler role ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn two_matches_fail_the_single_filler_role() {
    let root = tmpdir("many");
    // Two floor-clean skills both match `plan*` — a single-filler role needs
    // exactly one, so two fillers is a conformance error.
    import_skill(&root, "plan-tasks", &clean_skill("plan-tasks", None));
    import_skill(&root, "plan-sprints", &clean_skill("plan-sprints", None));
    write_temper_toml(&root, &required_role_toml("plan*"));

    let run = check_in(&root);
    assert!(
        !run.ok,
        "two fillers of a single-filler role must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("plan-tasks") && run.output.contains("plan-sprints"),
        "the finding names the colliding fillers, got:\n{}",
        run.output
    );
}

#[test]
fn a_non_required_unfilled_role_is_silent() {
    let root = tmpdir("non-required");
    // A clean skill that does not match the role's glob, and a role with no
    // `required` flag — an unfilled non-required role never fires, so the run is
    // clean.
    import_skill(&root, "lint-rust", &clean_skill("lint-rust", None));
    write_temper_toml(
        &root,
        "[role.planner]\n\
         artifact = \"skill\"\n\
         contract = \"contracts/skill.anthropic.toml\"\n\
         match = { name = \"plan*\" }\n",
    );

    let run = check_in(&root);
    assert!(
        run.ok,
        "a non-required unfilled role adds no finding ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_temper_toml_declaring_no_roster_leaves_the_floor_outcome_unchanged() {
    let root = tmpdir("no-roster");
    import_skill(&root, "lint-rust", &clean_skill("lint-rust", None));

    // Absent `temper.toml`: the floor runs, the clean skill passes.
    let absent = check_in(&root);
    assert!(absent.ok, "the clean skill passes the floor ⇒ zero");

    // A `temper.toml` carrying a `[kind]` layer but no `[role]` table declares an
    // empty roster — selection adds nothing, so the outcome is byte-for-byte the
    // floor's.
    write_temper_toml(
        &root,
        "[kind.skill]\n\
         adopt = \"skill.anthropic\"\n",
    );
    let no_roster = check_in(&root);
    assert!(no_roster.ok, "an empty roster changes nothing ⇒ still zero");
    assert_eq!(
        absent.output, no_roster.output,
        "a temper.toml declaring no roster must produce identical output to none"
    );
}
