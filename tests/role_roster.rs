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
/// the `skill` kind. The contract is an admissible inline clause (a generous `name`
/// cap any clean filler stays within) so these selection cases isolate the
/// single-filler gate — the roster itself passes admissibility.
fn required_role_toml(glob: &str) -> String {
    format!(
        "[role.planner]\n\
         artifact = \"skill\"\n\
         match = {{ name = \"{glob}\" }}\n\
         required = true\n\
         [[role.planner.clause]]\n\
         severity = \"required\"\n\
         predicate = \"max_len\"\n\
         field = \"name\"\n\
         max = 64\n"
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
         match = { name = \"plan*\" }\n\
         [[role.planner.clause]]\n\
         severity = \"required\"\n\
         predicate = \"max_len\"\n\
         field = \"name\"\n\
         max = 64\n",
    );

    let run = check_in(&root);
    assert!(
        run.ok,
        "a non-required unfilled role adds no finding ⇒ zero, got:\n{}",
        run.output
    );
}

/// A `temper.toml` declaring one `required`, name-glob, single-filler role whose
/// **inline** contract caps the filler's `name` at `max` characters.
fn inline_maxlen_role_toml(glob: &str, max: usize) -> String {
    format!(
        "[role.planner]\n\
         artifact = \"skill\"\n\
         match = {{ name = \"{glob}\" }}\n\
         required = true\n\
         [[role.planner.clause]]\n\
         severity = \"required\"\n\
         predicate = \"max_len\"\n\
         field = \"name\"\n\
         max = {max}\n"
    )
}

#[test]
fn a_filler_violating_an_inline_role_contract_reports_a_finding() {
    let root = tmpdir("inline-bad");
    // One floor-clean filler matching `plan*`; the inline contract caps `name` at
    // 3 chars, which `plan-tasks` (10) breaks. Selection is clean (one filler), so
    // the only finding is the conformance one.
    import_skill(&root, "plan-tasks", &clean_skill("plan-tasks", None));
    write_temper_toml(&root, &inline_maxlen_role_toml("plan*", 3));

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a filler that breaks its role's inline contract must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("does not conform")
            && run.output.contains("plan-tasks")
            && run.output.contains("planner"),
        "the finding names the conformance violation, the filler, and the role, got:\n{}",
        run.output
    );
}

#[test]
fn a_filler_violating_an_adopted_template_contract_reports_a_finding() {
    let root = tmpdir("template-bad");
    import_skill(&root, "plan-tasks", &clean_skill("plan-tasks", None));

    // A template contract on disk, resolved relative to the temper.toml dir,
    // capping `name` at 3 chars — `plan-tasks` (10) breaks it.
    let contracts = root.join("contracts");
    fs::create_dir_all(&contracts).unwrap();
    fs::write(
        contracts.join("role-skill.toml"),
        "[[clause]]\n\
         severity = \"required\"\n\
         predicate = \"max_len\"\n\
         field = \"name\"\n\
         max = 3\n",
    )
    .unwrap();
    write_temper_toml(
        &root,
        "[role.planner]\n\
         artifact = \"skill\"\n\
         contract = \"contracts/role-skill.toml\"\n\
         match = { name = \"plan*\" }\n\
         required = true\n",
    );

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a filler that breaks its role's adopted template must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("does not conform")
            && run.output.contains("plan-tasks")
            && run.output.contains("planner"),
        "the finding names the conformance violation, the filler, and the role, got:\n{}",
        run.output
    );
}

#[test]
fn a_filler_conforming_to_its_role_contract_is_clean() {
    let root = tmpdir("inline-ok");
    // The same single filler, but the inline contract's cap (64) is one the filler
    // stays within — so conformance adds nothing and the run is clean.
    import_skill(&root, "plan-tasks", &clean_skill("plan-tasks", None));
    write_temper_toml(&root, &inline_maxlen_role_toml("plan*", 64));

    let run = check_in(&root);
    assert!(
        run.ok,
        "a filler within its role's contract passes ⇒ zero, got:\n{}",
        run.output
    );
}

// ---- admissibility: the roster is itself checked --------------------------

#[test]
fn a_role_naming_an_unknown_artifact_kind_is_inadmissible() {
    let root = tmpdir("admit-unknown-kind");
    // A floor-clean skill is present, but the role names `command` — a kind
    // `temper` does not model — so a required role over it can never be filled.
    import_skill(&root, "lint-rust", &clean_skill("lint-rust", None));
    write_temper_toml(
        &root,
        "[role.releaser]\n\
         artifact = \"command\"\n\
         match = { name = \"release*\" }\n\
         required = true\n\
         [[role.releaser.clause]]\n\
         severity = \"required\"\n\
         predicate = \"max_len\"\n\
         field = \"name\"\n\
         max = 64\n",
    );

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a required role over an unmodeled kind must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("releaser")
            && run.output.contains("command")
            && run.output.contains("never be filled"),
        "the finding names the role, the kind, and that it can never be filled, got:\n{}",
        run.output
    );
}

#[test]
fn a_role_whose_template_does_not_resolve_is_inadmissible() {
    let root = tmpdir("admit-bad-template");
    // The single matching filler keeps selection clean; the only fault is the
    // `contract` template path resolving to no file under the temper.toml dir.
    import_skill(&root, "plan-tasks", &clean_skill("plan-tasks", None));
    write_temper_toml(
        &root,
        "[role.planner]\n\
         artifact = \"skill\"\n\
         contract = \"contracts/does-not-exist.toml\"\n\
         match = { name = \"plan*\" }\n\
         required = true\n",
    );

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a role whose contract template does not resolve must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("planner") && run.output.contains("does not resolve"),
        "the finding names the role and that its contract does not resolve, got:\n{}",
        run.output
    );
}

#[test]
fn a_role_with_an_inline_empty_enum_contract_is_inadmissible() {
    let root = tmpdir("admit-empty-enum");
    // One matching filler (selection clean); the inline contract carries an `enum`
    // clause listing no values — vacuous, so `engine::admissibility` rejects it.
    import_skill(&root, "plan-tasks", &clean_skill("plan-tasks", None));
    write_temper_toml(
        &root,
        "[role.planner]\n\
         artifact = \"skill\"\n\
         match = { name = \"plan*\" }\n\
         required = true\n\
         [[role.planner.clause]]\n\
         severity = \"required\"\n\
         predicate = \"enum\"\n\
         field = \"status\"\n\
         values = []\n",
    );

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a role whose inline contract is inadmissible must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("planner")
            && run.output.contains("inadmissible")
            && run.output.contains("enum"),
        "the finding names the role and the vacuous `enum` clause, got:\n{}",
        run.output
    );
}

#[test]
fn a_role_with_a_dangling_verified_by_is_inadmissible() {
    let root = tmpdir("admit-dangling-verifier");
    // Selection and conformance are clean (one filler, a generous inline cap); the
    // sole fault is `verified_by` naming a path that does not exist under the root.
    import_skill(&root, "plan-tasks", &clean_skill("plan-tasks", None));
    write_temper_toml(
        &root,
        "[role.planner]\n\
         artifact = \"skill\"\n\
         match = { name = \"plan*\" }\n\
         required = true\n\
         verified_by = \"tests/does-not-exist.rs\"\n\
         [[role.planner.clause]]\n\
         severity = \"required\"\n\
         predicate = \"max_len\"\n\
         field = \"name\"\n\
         max = 64\n",
    );

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a role with a dangling `verified_by` must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("planner")
            && run.output.contains("verifier")
            && run.output.contains("tests/does-not-exist.rs"),
        "the finding names the role and the dangling verifier path, got:\n{}",
        run.output
    );
}

#[test]
fn a_roster_whose_selectors_templates_and_verifiers_all_resolve_passes() {
    let root = tmpdir("admit-clean");
    import_skill(&root, "plan-tasks", &clean_skill("plan-tasks", None));

    // An admissible template contract on disk (a generous `name` cap the filler
    // stays within), and a `verified_by` path that exists under the root.
    let contracts = root.join("contracts");
    fs::create_dir_all(&contracts).unwrap();
    fs::write(
        contracts.join("role-skill.toml"),
        "[[clause]]\n\
         severity = \"required\"\n\
         predicate = \"max_len\"\n\
         field = \"name\"\n\
         max = 64\n",
    )
    .unwrap();
    fs::write(root.join("plan.rs"), "// a present verifier\n").unwrap();
    write_temper_toml(
        &root,
        "[role.planner]\n\
         artifact = \"skill\"\n\
         contract = \"contracts/role-skill.toml\"\n\
         match = { name = \"plan*\" }\n\
         required = true\n\
         verified_by = \"plan.rs\"\n",
    );

    let run = check_in(&root);
    assert!(
        run.ok,
        "a fully-resolving roster passes admissibility ⇒ zero, got:\n{}",
        run.output
    );
}

// ---- set scope: the `membership` roster predicate --------------------------

/// A floor-clean skill named `name` carrying a `model:` frontmatter field — the
/// field the `membership` predicate constrains on a filler and the source feature
/// it draws the allowed set from. `model` is not a floor-forbidden key, so the
/// skill stays clean and the only finding a case can produce is the membership one.
fn model_skill(name: &str, model: &str) -> String {
    format!(
        "---\n\
         name: {name}\n\
         description: Use when {name} is the task at hand; not for anything else.\n\
         model: {model}\n\
         ---\n\
         # {name}\n\
         \n\
         Body.\n"
    )
}

/// A `temper.toml` whose `agents` role constrains each `agent-*` filler's `model`
/// to the `model` feature drawn from the `approved-*` skills (S₂) — the set-scope
/// `membership` predicate, with a corpus-derived allowed set. The inline `max_len`
/// contract is generous so the roster passes admissibility and conformance, leaving
/// membership the only gate these cases exercise.
fn membership_role_toml() -> &'static str {
    "[role.agents]\n\
     artifact = \"skill\"\n\
     match = { name = \"agent-*\" }\n\
     membership = { field = \"model\", kind = \"skill\", match = { name = \"approved-*\" }, feature = \"model\" }\n\
     [[role.agents.clause]]\n\
     severity = \"required\"\n\
     predicate = \"max_len\"\n\
     field = \"name\"\n\
     max = 64\n"
}

#[test]
fn a_membership_role_fires_when_a_filler_is_outside_the_derived_set() {
    let root = tmpdir("membership-bad");
    // The approved set draws `{ opus }` from the lone `approved-*` skill; the
    // `agent-gpt` filler declares `gpt`, which is not in it.
    import_skill(&root, "agent-gpt", &model_skill("agent-gpt", "gpt"));
    import_skill(
        &root,
        "approved-opus",
        &model_skill("approved-opus", "opus"),
    );
    write_temper_toml(&root, membership_role_toml());

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a filler whose field falls outside the S₂-derived set must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("agents")
            && run.output.contains("agent-gpt")
            && run.output.contains("gpt"),
        "the finding names the role, the offending filler, and the non-member value, got:\n{}",
        run.output
    );
}

#[test]
fn a_membership_role_is_clean_when_every_filler_is_a_member() {
    let root = tmpdir("membership-ok");
    // The `agent-opus` filler's `model` is drawn from the approved set `{ opus }`,
    // so membership is satisfied and the whole run is clean.
    import_skill(&root, "agent-opus", &model_skill("agent-opus", "opus"));
    import_skill(
        &root,
        "approved-opus",
        &model_skill("approved-opus", "opus"),
    );
    write_temper_toml(&root, membership_role_toml());

    let run = check_in(&root);
    assert!(
        run.ok,
        "every filler drawn from the derived set passes ⇒ zero, got:\n{}",
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
