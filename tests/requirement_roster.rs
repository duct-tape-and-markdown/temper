//! End-to-end acceptance over the harness-contract roster — conformance and the
//! set-scope predicates (`count` / `unique` / `membership`), each quantified over a
//! requirement's **satisfier set** (`specs/10-contracts.md`, "Requirements — the
//! harness's named obligations"; `specs/45-governance.md`, "The set scope").
//!
//! Drives the built `temper` binary so the whole path is pinned: `temper.toml`
//! discovery at the project root, parsing its `[requirement.<name>]` tables onto the
//! author layer, and running the roster over the imported skills and their authored
//! `[representation].satisfies` opt-in. The name-`match` selector is eradicated —
//! opt-in `satisfies` is the sole fill — so a satisfier set is the artifacts of a
//! requirement's `kind` whose `satisfies` names it.
//!
//! The cases mirror the entry's acceptance:
//! - conformance validates the satisfiers against the requirement's contract;
//! - the `count` cardinality bound quantifies over the satisfier set;
//! - the `unique` predicate quantifies over the satisfier set;
//! - the `membership` predicate (and its typed-reference `conforms_to`) draws its
//!   allowed set from a *second* satisfier set;
//! - a `match = {…}` key is rejected as an unknown key;
//! - the roster is itself checked (admissibility);
//! - a `temper.toml` declaring no roster leaves the floor outcome unchanged;
//! - the retired `[role.*]` surface is now rejected at load.

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
        "author-requirement-roster-{}-{}-{}",
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
/// produce is a roster one.
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

/// Author the `[representation].satisfies` opt-in on an imported skill's surface
/// `meta.toml` — the binding the roster reads to build a requirement's satisfier set.
/// `import` never writes it (it is surface-authored, not frontmatter), so a case
/// appends the table exactly as a human editing the surface would.
fn author_satisfies(root: &Path, name: &str, requirements: &[&str]) {
    let meta = root
        .join(".temper")
        .join("skills")
        .join(name)
        .join("meta.toml");
    let mut contents = fs::read_to_string(&meta).unwrap();
    let list = requirements
        .iter()
        .map(|r| format!("\"{r}\""))
        .collect::<Vec<_>>()
        .join(", ");
    contents.push_str(&format!("\n[representation]\nsatisfies = [{list}]\n"));
    fs::write(&meta, contents).unwrap();
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

// ---- conformance: satisfiers validated against the requirement contract ----

/// A `temper.toml` declaring one `required` requirement over the `skill` kind whose
/// **inline** contract caps its satisfiers' `name` at `max` characters. Fill is by
/// opt-in `satisfies` — the requirement carries no `match` selector.
fn inline_maxlen_requirement_toml(max: usize) -> String {
    format!(
        "[requirement.planner]\n\
         kind = \"skill\"\n\
         required = true\n\
         [[requirement.planner.clause]]\n\
         severity = \"required\"\n\
         predicate = \"max_len\"\n\
         field = \"name\"\n\
         max = {max}\n"
    )
}

#[test]
fn a_satisfier_violating_an_inline_contract_reports_a_finding() {
    let root = tmpdir("inline-bad");
    // One floor-clean skill opts into `planner`; the inline contract caps `name` at
    // 3 chars, which `plan-tasks` (10) breaks. The satisfier is the conformance subject.
    import_skill(&root, "plan-tasks", &clean_skill("plan-tasks"));
    author_satisfies(&root, "plan-tasks", &["planner"]);
    write_temper_toml(&root, &inline_maxlen_requirement_toml(3));

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a satisfier that breaks its requirement's inline contract must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("does not conform")
            && run.output.contains("plan-tasks")
            && run.output.contains("planner"),
        "the finding names the conformance violation, the satisfier, and the requirement, got:\n{}",
        run.output
    );
}

#[test]
fn a_satisfier_violating_an_adopted_template_contract_reports_a_finding() {
    let root = tmpdir("template-bad");
    import_skill(&root, "plan-tasks", &clean_skill("plan-tasks"));
    author_satisfies(&root, "plan-tasks", &["planner"]);

    // A template contract on disk, resolved relative to the temper.toml dir,
    // capping `name` at 3 chars — `plan-tasks` (10) breaks it.
    let contracts = root.join("contracts");
    fs::create_dir_all(&contracts).unwrap();
    fs::write(
        contracts.join("skill-shape.toml"),
        "[[clause]]\n\
         severity = \"required\"\n\
         predicate = \"max_len\"\n\
         field = \"name\"\n\
         max = 3\n",
    )
    .unwrap();
    write_temper_toml(
        &root,
        "[requirement.planner]\n\
         kind = \"skill\"\n\
         contract = \"contracts/skill-shape.toml\"\n\
         required = true\n",
    );

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a satisfier that breaks its requirement's adopted template must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("does not conform")
            && run.output.contains("plan-tasks")
            && run.output.contains("planner"),
        "the finding names the conformance violation, the satisfier, and the requirement, got:\n{}",
        run.output
    );
}

#[test]
fn a_satisfier_conforming_to_its_requirement_contract_is_clean() {
    let root = tmpdir("inline-ok");
    // The same lone satisfier, but the inline contract's cap (64) is one it stays
    // within — so conformance adds nothing and the run is clean.
    import_skill(&root, "plan-tasks", &clean_skill("plan-tasks"));
    author_satisfies(&root, "plan-tasks", &["planner"]);
    write_temper_toml(&root, &inline_maxlen_requirement_toml(64));

    let run = check_in(&root);
    assert!(
        run.ok,
        "a satisfier within its requirement's contract passes ⇒ zero, got:\n{}",
        run.output
    );
}

// ---- set scope: the `count` cardinality bound over the satisfier set -------

/// A `temper.toml` whose `agents` requirement bounds its satisfier-set cardinality to
/// `[min, max]` — the set-scope `count` predicate. No `required` flag rides alongside
/// (`count` is its general form). The satisfiers are the skills opting into `agents`.
fn count_band_toml(min: usize, max: usize) -> String {
    format!(
        "[requirement.agents]\n\
         kind = \"skill\"\n\
         count = {{ min = {min}, max = {max} }}\n"
    )
}

#[test]
fn a_count_band_fires_when_the_satisfier_set_is_out_of_band() {
    let root = tmpdir("count-over");
    // Two skills opt into `agents`; the band caps the satisfier count at one, so two
    // is out of band.
    import_skill(&root, "agent-one", &clean_skill("agent-one"));
    import_skill(&root, "agent-two", &clean_skill("agent-two"));
    author_satisfies(&root, "agent-one", &["agents"]);
    author_satisfies(&root, "agent-two", &["agents"]);
    write_temper_toml(&root, &count_band_toml(0, 1));

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a satisfier count outside the declared band must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("agents")
            && run.output.contains("agent-one")
            && run.output.contains("agent-two")
            && run.output.contains("[0, 1]"),
        "the finding names the requirement, the satisfiers, and the bound, got:\n{}",
        run.output
    );
}

#[test]
fn a_count_band_is_clean_within_bounds() {
    let root = tmpdir("count-ok");
    // Two skills opt into `agents`, inside a `[1, 2]` band — clean.
    import_skill(&root, "agent-one", &clean_skill("agent-one"));
    import_skill(&root, "agent-two", &clean_skill("agent-two"));
    author_satisfies(&root, "agent-one", &["agents"]);
    author_satisfies(&root, "agent-two", &["agents"]);
    write_temper_toml(&root, &count_band_toml(1, 2));

    let run = check_in(&root);
    assert!(
        run.ok,
        "a satisfier count inside the band passes ⇒ zero, got:\n{}",
        run.output
    );
}

// ---- set scope: the `unique` predicate over the satisfier set --------------

/// A floor-clean skill named `name` carrying a `model:` frontmatter field — the field
/// the `unique` and `membership` predicates read. `model` is not a floor-forbidden
/// key, so the skill stays clean and the only finding a case produces is a roster one.
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

#[test]
fn a_unique_field_fires_when_two_satisfiers_share_a_value() {
    let root = tmpdir("unique-bad");
    // Two `agents` satisfiers share `model = opus`; `unique = ["model"]` requires each
    // distinct across the satisfier set.
    import_skill(&root, "agent-a", &model_skill("agent-a", "opus"));
    import_skill(&root, "agent-b", &model_skill("agent-b", "opus"));
    author_satisfies(&root, "agent-a", &["agents"]);
    author_satisfies(&root, "agent-b", &["agents"]);
    write_temper_toml(
        &root,
        "[requirement.agents]\n\
         kind = \"skill\"\n\
         unique = [\"model\"]\n",
    );

    let run = check_in(&root);
    assert!(
        !run.ok,
        "two satisfiers sharing a `unique` field must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("agents")
            && run.output.contains("model")
            && run.output.contains("opus")
            && run.output.contains("agent-a")
            && run.output.contains("agent-b"),
        "the finding names the requirement, the field, the shared value, and the satisfiers, got:\n{}",
        run.output
    );
}

// ---- admissibility: the roster is itself checked --------------------------

#[test]
fn a_requirement_naming_an_unknown_kind_is_inadmissible() {
    let root = tmpdir("admit-unknown-kind");
    // A floor-clean skill opts into the requirement (so coverage is satisfied), but the
    // requirement is typed to `command` — a kind `temper` does not model — so a
    // required requirement over it can never be filled.
    import_skill(&root, "lint-rust", &clean_skill("lint-rust"));
    author_satisfies(&root, "lint-rust", &["releaser"]);
    write_temper_toml(
        &root,
        "[requirement.releaser]\n\
         kind = \"command\"\n\
         required = true\n\
         [[requirement.releaser.clause]]\n\
         severity = \"required\"\n\
         predicate = \"max_len\"\n\
         field = \"name\"\n\
         max = 64\n",
    );

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a required requirement over an unmodeled kind must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("releaser")
            && run.output.contains("command")
            && run.output.contains("never be filled"),
        "the finding names the requirement, the kind, and that it can never be filled, got:\n{}",
        run.output
    );
}

#[test]
fn a_requirement_whose_template_does_not_resolve_is_inadmissible() {
    let root = tmpdir("admit-bad-template");
    // The satisfier keeps coverage clean; the only fault is the `contract` template
    // path resolving to no file under the temper.toml dir.
    import_skill(&root, "plan-tasks", &clean_skill("plan-tasks"));
    author_satisfies(&root, "plan-tasks", &["planner"]);
    write_temper_toml(
        &root,
        "[requirement.planner]\n\
         kind = \"skill\"\n\
         contract = \"contracts/does-not-exist.toml\"\n\
         required = true\n",
    );

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a requirement whose contract template does not resolve must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("planner") && run.output.contains("does not resolve"),
        "the finding names the requirement and that its contract does not resolve, got:\n{}",
        run.output
    );
}

#[test]
fn a_requirement_with_an_inline_empty_enum_contract_is_inadmissible() {
    let root = tmpdir("admit-empty-enum");
    // A satisfier keeps coverage clean; the inline contract carries an `enum` clause
    // listing no values — vacuous, so `engine::admissibility` rejects it.
    import_skill(&root, "plan-tasks", &clean_skill("plan-tasks"));
    author_satisfies(&root, "plan-tasks", &["planner"]);
    write_temper_toml(
        &root,
        "[requirement.planner]\n\
         kind = \"skill\"\n\
         required = true\n\
         [[requirement.planner.clause]]\n\
         severity = \"required\"\n\
         predicate = \"enum\"\n\
         field = \"status\"\n\
         values = []\n",
    );

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a requirement whose inline contract is inadmissible must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("planner")
            && run.output.contains("inadmissible")
            && run.output.contains("enum"),
        "the finding names the requirement and the vacuous `enum` clause, got:\n{}",
        run.output
    );
}

#[test]
fn a_requirement_with_a_dangling_verified_by_is_inadmissible() {
    let root = tmpdir("admit-dangling-verifier");
    // Coverage and conformance are clean (a satisfier, a generous inline cap); the
    // sole fault is `verified_by` naming a path that does not exist under the root.
    import_skill(&root, "plan-tasks", &clean_skill("plan-tasks"));
    author_satisfies(&root, "plan-tasks", &["planner"]);
    write_temper_toml(
        &root,
        "[requirement.planner]\n\
         kind = \"skill\"\n\
         required = true\n\
         verified_by = \"tests/does-not-exist.rs\"\n\
         [[requirement.planner.clause]]\n\
         severity = \"required\"\n\
         predicate = \"max_len\"\n\
         field = \"name\"\n\
         max = 64\n",
    );

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a requirement with a dangling `verified_by` must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("planner")
            && run.output.contains("verifier")
            && run.output.contains("tests/does-not-exist.rs"),
        "the finding names the requirement and the dangling verifier path, got:\n{}",
        run.output
    );
}

#[test]
fn a_roster_whose_contracts_and_verifiers_all_resolve_passes() {
    let root = tmpdir("admit-clean");
    import_skill(&root, "plan-tasks", &clean_skill("plan-tasks"));
    author_satisfies(&root, "plan-tasks", &["planner"]);

    // An admissible template contract on disk (a generous `name` cap the satisfier
    // stays within), and a `verified_by` path that exists under the root.
    let contracts = root.join("contracts");
    fs::create_dir_all(&contracts).unwrap();
    fs::write(
        contracts.join("skill-shape.toml"),
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
        "[requirement.planner]\n\
         kind = \"skill\"\n\
         contract = \"contracts/skill-shape.toml\"\n\
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

/// A `temper.toml` whose `agents` requirement constrains each satisfier's `model` to
/// the `model` feature drawn from the `approved-model` satisfier set (S₂) — the
/// set-scope `membership` predicate, with a corpus-derived allowed set. The `source`
/// names a *declared* requirement (below), so the approved skills' `satisfies` link
/// resolves. The inline `max_len` contract is generous so admissibility and
/// conformance pass, leaving membership the only gate these cases exercise.
fn membership_requirement_toml() -> &'static str {
    "[requirement.agents]\n\
     kind = \"skill\"\n\
     membership = { field = \"model\", kind = \"skill\", source = \"approved-model\", feature = \"model\" }\n\
     [[requirement.agents.clause]]\n\
     severity = \"required\"\n\
     predicate = \"max_len\"\n\
     field = \"name\"\n\
     max = 64\n\
     \n\
     [requirement.approved-model]\n\
     kind = \"skill\"\n\
     means = \"a skill on the approved-model roster\"\n"
}

#[test]
fn a_membership_requirement_fires_when_a_satisfier_is_outside_the_derived_set() {
    let root = tmpdir("membership-bad");
    // The approved set draws `{ opus }` from the lone `approved-model` satisfier; the
    // `agent-gpt` satisfier declares `gpt`, which is not in it.
    import_skill(&root, "agent-gpt", &model_skill("agent-gpt", "gpt"));
    import_skill(
        &root,
        "approved-opus",
        &model_skill("approved-opus", "opus"),
    );
    author_satisfies(&root, "agent-gpt", &["agents"]);
    author_satisfies(&root, "approved-opus", &["approved-model"]);
    write_temper_toml(&root, membership_requirement_toml());

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a satisfier whose field falls outside the S₂-derived set must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("agents")
            && run.output.contains("agent-gpt")
            && run.output.contains("gpt"),
        "the finding names the requirement, the offending satisfier, and the non-member value, got:\n{}",
        run.output
    );
}

#[test]
fn a_membership_requirement_is_clean_when_every_satisfier_is_a_member() {
    let root = tmpdir("membership-ok");
    // The `agent-opus` satisfier's `model` is drawn from the approved set `{ opus }`,
    // so membership is satisfied and the whole run is clean.
    import_skill(&root, "agent-opus", &model_skill("agent-opus", "opus"));
    import_skill(
        &root,
        "approved-opus",
        &model_skill("approved-opus", "opus"),
    );
    author_satisfies(&root, "agent-opus", &["agents"]);
    author_satisfies(&root, "approved-opus", &["approved-model"]);
    write_temper_toml(&root, membership_requirement_toml());

    let run = check_in(&root);
    assert!(
        run.ok,
        "every satisfier drawn from the derived set passes ⇒ zero, got:\n{}",
        run.output
    );
}

// ---- set scope: the `membership` typed-reference (`conforms_to`) -----------

/// A floor-clean skill carrying both a `model:` and a `tier:` field. `model` is the
/// membership feature drawn into the allowed set; `tier` is what a `conforms_to`
/// contract discriminates on. Neither key is floor-forbidden, so the skill stays
/// clean and the only finding a case produces is the membership one.
fn tiered_skill(name: &str, model: &str, tier: &str) -> String {
    format!(
        "---\n\
         name: {name}\n\
         description: Use when {name} is the task at hand; not for anything else.\n\
         model: {model}\n\
         tier: {tier}\n\
         ---\n\
         # {name}\n\
         \n\
         Body.\n"
    )
}

/// Import the two `approved-model` sources both typed-reference cases share and opt
/// each into `approved-model`: an `official` source carrying `opus` and a `draft`
/// source carrying `gpt`. Under a `conforms_to` = official constraint only the first
/// is a member-contributing source, so `gpt` comes *solely* from a non-conforming
/// source.
fn import_tiered_sources(root: &Path) {
    import_skill(
        root,
        "approved-opus",
        &tiered_skill("approved-opus", "opus", "official"),
    );
    import_skill(
        root,
        "approved-gpt",
        &tiered_skill("approved-gpt", "gpt", "draft"),
    );
    author_satisfies(root, "approved-opus", &["approved-model"]);
    author_satisfies(root, "approved-gpt", &["approved-model"]);
}

#[test]
fn a_typed_reference_flags_a_satisfier_whose_value_comes_only_from_a_nonconforming_source() {
    let root = tmpdir("typed-ref-bad");
    // The `agent-gpt` satisfier declares `gpt`. `gpt` is carried only by `approved-gpt`,
    // whose `tier` is `draft` — so under a `conforms_to = official` constraint that
    // source is dropped and `gpt` is not in the derived set.
    import_skill(&root, "agent-gpt", &model_skill("agent-gpt", "gpt"));
    author_satisfies(&root, "agent-gpt", &["agents"]);
    import_tiered_sources(&root);

    // The typed reference: a template contract requiring the source's `tier` be
    // `official`, resolved relative to the temper.toml dir.
    let contracts = root.join("contracts");
    fs::create_dir_all(&contracts).unwrap();
    fs::write(
        contracts.join("approved.toml"),
        "[[clause]]\n\
         severity = \"required\"\n\
         predicate = \"enum\"\n\
         field = \"tier\"\n\
         values = [\"official\"]\n",
    )
    .unwrap();
    write_temper_toml(
        &root,
        "[requirement.agents]\n\
         kind = \"skill\"\n\
         membership = { field = \"model\", kind = \"skill\", source = \"approved-model\", feature = \"model\", conforms_to = \"contracts/approved.toml\" }\n\
         [[requirement.agents.clause]]\n\
         severity = \"required\"\n\
         predicate = \"max_len\"\n\
         field = \"name\"\n\
         max = 64\n\
         \n\
         [requirement.approved-model]\n\
         kind = \"skill\"\n\
         means = \"a skill on the approved-model roster\"\n",
    );

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a satisfier whose value comes only from a non-conforming source must fail ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("agents")
            && run.output.contains("agent-gpt")
            && run.output.contains("gpt"),
        "the finding names the requirement, the offending satisfier, and the non-member value, got:\n{}",
        run.output
    );
}

#[test]
fn dropping_the_conforms_to_puts_the_same_value_back_in_the_set() {
    let root = tmpdir("typed-ref-dropped");
    // The exact same corpus, but the membership carries no `conforms_to`: now the
    // non-conforming `approved-gpt` source contributes `gpt` to the derived set, so
    // `agent-gpt` is in-set and the run is silent — the constraint was the only gate.
    import_skill(&root, "agent-gpt", &model_skill("agent-gpt", "gpt"));
    author_satisfies(&root, "agent-gpt", &["agents"]);
    import_tiered_sources(&root);
    write_temper_toml(&root, membership_requirement_toml());

    let run = check_in(&root);
    assert!(
        run.ok,
        "without the `conforms_to` constraint the same value is in-set ⇒ zero, got:\n{}",
        run.output
    );
}

// ---- the name-`match` selector is eradicated -------------------------------

#[test]
fn a_match_key_in_a_requirement_is_rejected_as_an_unknown_key() {
    let root = tmpdir("match-unknown-key");
    import_skill(&root, "plan-tasks", &clean_skill("plan-tasks"));

    // The name-`match` selector is gone — fill is opt-in `satisfies` alone. A leftover
    // `match = {…}` is no longer a facet but an unknown key, rejected loudly at load
    // rather than silently dropped (`specs/10-contracts.md`, "Decision: unknown keys
    // are rejected, not ignored").
    write_temper_toml(
        &root,
        "[requirement.planner]\n\
         kind = \"skill\"\n\
         match = { name = \"plan*\" }\n\
         required = true\n",
    );

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a `match` key must fail the run at load ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("unknown key") && run.output.contains("match"),
        "the load error names the unknown `match` key, got:\n{}",
        run.output
    );
}

#[test]
fn a_temper_toml_declaring_no_roster_leaves_the_floor_outcome_unchanged() {
    let root = tmpdir("no-roster");
    import_skill(&root, "lint-rust", &clean_skill("lint-rust"));

    // Absent `temper.toml`: the floor runs, the clean skill passes.
    let absent = check_in(&root);
    assert!(absent.ok, "the clean skill passes the floor ⇒ zero");

    // A `temper.toml` carrying a `[kind]` layer but no `[requirement]` table declares
    // an empty roster — the roster adds nothing, so the outcome is byte-for-byte the
    // floor's.
    write_temper_toml(
        &root,
        "[kind.skill]\n\
         package = \"skill.anthropic\"\n",
    );
    let no_roster = check_in(&root);
    assert!(no_roster.ok, "an empty roster changes nothing ⇒ still zero");
    assert_eq!(
        absent.output, no_roster.output,
        "a temper.toml declaring no roster must produce identical output to none"
    );
}

#[test]
fn a_retired_role_table_is_rejected() {
    let root = tmpdir("retired-role");
    import_skill(&root, "plan-tasks", &clean_skill("plan-tasks"));

    // The `[role.*]` surface was hard-cut into `[requirement.*]` by the consolidation.
    // A `temper.toml` that still declares one must fail loudly at load — a silently
    // ignored roster is exactly the gap temper exists to catch — so it is rejected as
    // an unknown top-level key, not dropped.
    write_temper_toml(
        &root,
        "[role.planner]\n\
         artifact = \"skill\"\n\
         required = true\n",
    );

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a retired `[role.*]` table must fail the run ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("unknown top-level key") && run.output.contains("role"),
        "the load error names the retired `role` root, got:\n{}",
        run.output
    );
}
