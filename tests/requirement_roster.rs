//! End-to-end acceptance over the harness-contract roster — conformance and the
//! set-scope predicates (`count` / `unique` / `membership`), each quantified over a
//! requirement's **satisfier set** (`specs/architecture/10-contracts.md`, "Requirements — the
//! harness's named obligations"; `specs/architecture/45-governance.md`, "The set scope").
//!
//! Drives the built `temper` binary so the whole path is pinned: a golden lock at the
//! project root carrying the declared requirements (`specs/architecture/20-surface.md`,
//! "The lock and drift — one vocabulary" — the gate sources requirements from the lock,
//! never a re-imported `temper.toml`), and running the roster over the harness's live
//! skills and their authored `satisfies` opt-in. The name-`match` selector is
//! eradicated — opt-in `satisfies` is the sole fill — so a satisfier set is the
//! artifacts of a requirement's `kind` whose `satisfies` names it.
//!
//! The cases mirror the entry's acceptance:
//! - conformance validates the satisfiers against the requirement's bound package;
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

use temper::drift::{self, Declarations, EmitOptions, MembershipRow, Payload, RequirementRow};

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

/// Write a one-skill harness member directly at its real Claude Code locus
/// (`<root>/.claude/skills/<name>/SKILL.md`) — `check` reads built-in kind members
/// live off harness disk (`specs/architecture/20-surface.md`, "The lock and drift"), no
/// scratch import.
fn write_skill(root: &Path, name: &str, skill_md: &str) {
    let dir = root.join(".claude").join("skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("SKILL.md"), skill_md).unwrap();
}

/// Author a member's `satisfies` links on its surface overlay
/// (`<root>/.temper/skills/<name>/SKILL.md`) — the projected document a live off-disk
/// walk grafts a member's fill edges from (`specs/architecture/20-surface.md`, "The
/// lock and drift"); the real harness file itself carries no temper annotation.
fn author_satisfies(root: &Path, name: &str, requirements: &[&str]) {
    let skill_kind = temper::builtin_kind::definition("skill").unwrap().unwrap();
    let source = root
        .join(".claude")
        .join("skills")
        .join(name)
        .join("SKILL.md");
    let mut skill = temper::frontmatter::Member::from_source(&skill_kind, &source).unwrap();
    skill.satisfies = requirements
        .iter()
        .map(|r| temper::document::Satisfies::new(*r))
        .collect();

    let dir = root.join(".temper").join("skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("SKILL.md"), skill.to_document().emit()).unwrap();
}

/// Author the requirements a member **publishes** on its surface overlay — the demand
/// side of the fill edge, the mirror of [`author_satisfies`], grafted from the same
/// live off-disk walk.
fn author_published(
    root: &Path,
    name: &str,
    published: Vec<temper::document::PublishedRequirement>,
) {
    let skill_kind = temper::builtin_kind::definition("skill").unwrap().unwrap();
    let source = root
        .join(".claude")
        .join("skills")
        .join(name)
        .join("SKILL.md");
    let mut skill = temper::frontmatter::Member::from_source(&skill_kind, &source).unwrap();
    skill.published_requirements = published;

    let dir = root.join(".temper").join("skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("SKILL.md"), skill.to_document().emit()).unwrap();
}

/// A member-published requirement carrying only the facets a member header publishes
/// (`kind` and `required`; `means`/`package` unused by these cases).
fn published(
    name: &str,
    kind: Option<&str>,
    required: bool,
) -> temper::document::PublishedRequirement {
    temper::document::PublishedRequirement {
        name: name.to_string(),
        means: None,
        kind: kind.map(str::to_string),
        package: None,
        required,
    }
}

/// A `temper.toml` that declares no roster but is present, so the layered
/// member-published-requirement path runs (the union only happens under a layer).
const LAYERED_NO_ROSTER: &str = "[kind.skill]\npackage = \"skill.anthropic\"\n";

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

/// Write `<root>/temper.toml` verbatim, with no resync: requirements ride the lock
/// (`write_requirements`), so this is only for the assembly-scope facets `temper.toml`
/// still carries (a `[kind.*]` package registration, …), for a deliberately malformed
/// document a load-error case parses directly, and — written empty — to flip the
/// assembly from absent to present.
fn write_temper_toml(root: &Path, contents: &str) {
    fs::write(root.join("temper.toml"), contents).unwrap();
}

/// A bare `RequirementRow` naming `name` and typed to `kind`, otherwise empty — the
/// starting point each case's builder customizes.
fn requirement(name: &str, kind: &str) -> RequirementRow {
    RequirementRow {
        name: name.to_string(),
        kind: Some(kind.to_string()),
        package: None,
        required: false,
        count: None,
        unique: Vec::new(),
        membership: None,
        degree: None,
        verified_by: None,
    }
}

/// Compile a golden lock at `<root>/.temper/lock.toml` carrying just the declared
/// `requirements` — the SDK-emitted fixture standing in for `import::run`'s scratch
/// projection of a `temper.toml` `[requirement.*]` table: the gate sources
/// requirements from the lock, never a re-imported assembly
/// (`specs/architecture/20-surface.md`, "The lock and drift — one vocabulary").
fn write_requirements(root: &Path, requirements: Vec<RequirementRow>) {
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            requirements,
            ..Declarations::default()
        },
        members: Vec::new(),
    };
    drift::emit(&payload, &root.join(".temper"), EmitOptions::default()).unwrap();
}

// ---- conformance: satisfiers validated against the requirement's package ----
//
// There is no on-disk project-package file format any more
// (`specs/architecture/15-kinds.md`, "Decision: field typing lives in the SDK —
// there is no kind file format"), so these cases bind one of the compiled-in
// built-in packages — cross-kind, since a skill's own floor already binds
// `skill.anthropic` — to exercise the *distinct* requirement-conformance tier
// rather than the floor.

#[test]
fn a_satisfier_violating_its_bound_package_reports_a_finding() {
    let root = tmpdir("package-bad");
    // One floor-clean skill opts into `planner`; the bound `rule.anthropic` package
    // forbids a `description` frontmatter key, which every skill carries (its own
    // floor requires it) — a cross-kind package a skill satisfier is checked against
    // *in addition to* its own kind's floor.
    write_skill(&root, "plan-tasks", &clean_skill("plan-tasks"));
    author_satisfies(&root, "plan-tasks", &["planner"]);
    write_requirements(
        &root,
        vec![RequirementRow {
            package: Some("rule.anthropic".to_string()),
            required: true,
            ..requirement("planner", "skill")
        }],
    );
    write_temper_toml(&root, "");

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a satisfier that breaks its requirement's bound package must fail the run ⇒ non-zero"
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
fn a_satisfier_conforming_to_its_bound_package_is_clean() {
    let root = tmpdir("package-ok");
    // The same lone satisfier, but bound to `memory.anthropic` — a single advisory
    // `max_lines` cap the short floor-clean body stays within — so conformance adds
    // nothing and the run is clean.
    write_skill(&root, "plan-tasks", &clean_skill("plan-tasks"));
    author_satisfies(&root, "plan-tasks", &["planner"]);
    write_requirements(
        &root,
        vec![RequirementRow {
            package: Some("memory.anthropic".to_string()),
            required: true,
            ..requirement("planner", "skill")
        }],
    );
    write_temper_toml(&root, "");

    let run = check_in(&root);
    assert!(
        run.ok,
        "a satisfier within its requirement's package passes ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_requirement_binding_a_builtin_package_by_name_composes() {
    let root = tmpdir("package-builtin");
    // A requirement may bind a *built-in* package by name — `skill.anthropic` — so its
    // satisfiers are checked by that package's contract *in addition to* their own
    // kind's floor. A floor-clean skill within `skill.anthropic` passes, proving the
    // by-name built-in binding resolves and composes.
    write_skill(&root, "plan-tasks", &clean_skill("plan-tasks"));
    author_satisfies(&root, "plan-tasks", &["planner"]);
    write_requirements(
        &root,
        vec![RequirementRow {
            package: Some("skill.anthropic".to_string()),
            required: true,
            ..requirement("planner", "skill")
        }],
    );
    write_temper_toml(&root, "");

    let run = check_in(&root);
    assert!(
        run.ok,
        "a requirement binding the built-in `skill.anthropic` by name resolves and a clean satisfier passes ⇒ zero, got:\n{}",
        run.output
    );
}

// ---- set scope: the `count` cardinality bound over the satisfier set -------

/// The `agents` requirement's `count` row bounding its satisfier-set cardinality to
/// `[min, max]` — the set-scope `count` predicate. No `required` flag rides alongside
/// (`count` is its general form). The satisfiers are the skills opting into `agents`.
fn count_band_requirement(min: usize, max: usize) -> RequirementRow {
    RequirementRow {
        count: Some(temper::drift::CountBoundRow { min, max }),
        ..requirement("agents", "skill")
    }
}

#[test]
fn a_count_band_fires_when_the_satisfier_set_is_out_of_band() {
    let root = tmpdir("count-over");
    // Two skills opt into `agents`; the band caps the satisfier count at one, so two
    // is out of band.
    write_skill(&root, "agent-one", &clean_skill("agent-one"));
    write_skill(&root, "agent-two", &clean_skill("agent-two"));
    author_satisfies(&root, "agent-one", &["agents"]);
    author_satisfies(&root, "agent-two", &["agents"]);
    write_requirements(&root, vec![count_band_requirement(0, 1)]);
    write_temper_toml(&root, "");

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
    write_skill(&root, "agent-one", &clean_skill("agent-one"));
    write_skill(&root, "agent-two", &clean_skill("agent-two"));
    author_satisfies(&root, "agent-one", &["agents"]);
    author_satisfies(&root, "agent-two", &["agents"]);
    write_requirements(&root, vec![count_band_requirement(1, 2)]);
    write_temper_toml(&root, "");

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
    write_skill(&root, "agent-a", &model_skill("agent-a", "opus"));
    write_skill(&root, "agent-b", &model_skill("agent-b", "opus"));
    author_satisfies(&root, "agent-a", &["agents"]);
    author_satisfies(&root, "agent-b", &["agents"]);
    write_requirements(
        &root,
        vec![RequirementRow {
            unique: vec!["model".to_string()],
            ..requirement("agents", "skill")
        }],
    );
    write_temper_toml(&root, "");

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
    write_skill(&root, "lint-rust", &clean_skill("lint-rust"));
    author_satisfies(&root, "lint-rust", &["releaser"]);
    write_requirements(
        &root,
        vec![RequirementRow {
            required: true,
            ..requirement("releaser", "command")
        }],
    );
    write_temper_toml(&root, "");

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
fn a_requirement_binding_an_unresolvable_package_is_inadmissible() {
    let root = tmpdir("admit-bad-package");
    // The satisfier keeps coverage clean; the only fault is the bound `package` name
    // matching no built-in and no `.temper/packages/` project package — `names a real
    // package`, admissibility's finding.
    write_skill(&root, "plan-tasks", &clean_skill("plan-tasks"));
    author_satisfies(&root, "plan-tasks", &["planner"]);
    write_requirements(
        &root,
        vec![RequirementRow {
            package: Some("does-not-exist".to_string()),
            required: true,
            ..requirement("planner", "skill")
        }],
    );
    write_temper_toml(&root, "");

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a requirement whose bound package does not resolve must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("planner")
            && run.output.contains("does-not-exist")
            && run.output.contains("does not resolve"),
        "the finding names the requirement and that its package does not resolve, got:\n{}",
        run.output
    );
}

// A requirement binding an *inadmissible* package (e.g. a vacuous `enum` clause) is
// no longer reachable end-to-end: every bound-by-name package is one of the
// compiled-in built-ins, and each ships admissible (`tests/contract_template.rs`,
// `the_shipped_built_in_packages_are_admissible`) — there is no on-disk project
// package left to author a deliberately-broken one from
// (`specs/architecture/15-kinds.md`, "Decision: field typing lives in the SDK —
// there is no kind file format"). The admissibility check itself stays proven at
// the unit level (`src/roster.rs`, `a_bound_package_with_an_empty_enum_is_inadmissible`).

#[test]
fn a_requirement_with_a_dangling_verified_by_is_inadmissible() {
    let root = tmpdir("admit-dangling-verifier");
    // Coverage and conformance are clean (a satisfier, no package shape); the sole
    // fault is `verified_by` naming a path that does not exist under the root.
    write_skill(&root, "plan-tasks", &clean_skill("plan-tasks"));
    author_satisfies(&root, "plan-tasks", &["planner"]);
    write_requirements(
        &root,
        vec![RequirementRow {
            required: true,
            verified_by: Some("tests/does-not-exist.rs".to_string()),
            ..requirement("planner", "skill")
        }],
    );
    write_temper_toml(&root, "");

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
fn a_roster_whose_packages_and_verifiers_all_resolve_passes() {
    let root = tmpdir("admit-clean");
    write_skill(&root, "plan-tasks", &clean_skill("plan-tasks"));
    author_satisfies(&root, "plan-tasks", &["planner"]);

    // An admissible bound package (built-in, so it resolves and admits by
    // construction) and a `verified_by` path that exists under the root.
    fs::write(root.join("plan.rs"), "// a present verifier\n").unwrap();
    write_requirements(
        &root,
        vec![RequirementRow {
            package: Some("memory.anthropic".to_string()),
            required: true,
            verified_by: Some("plan.rs".to_string()),
            ..requirement("planner", "skill")
        }],
    );
    write_temper_toml(&root, "");

    let run = check_in(&root);
    assert!(
        run.ok,
        "a fully-resolving roster passes admissibility ⇒ zero, got:\n{}",
        run.output
    );
}

// ---- set scope: the `membership` roster predicate --------------------------

/// The `agents` + `approved-model` requirement rows: `agents` constrains each
/// satisfier's `model` to the `model` feature drawn from the `approved-model`
/// satisfier set (S₂) — the set-scope `membership` predicate, with a corpus-derived
/// allowed set. `source` names a *declared* requirement, so the approved skills'
/// `satisfies` link resolves. `agents` binds no package (no shape gate), leaving
/// membership the only gate these cases exercise.
fn membership_requirements() -> Vec<RequirementRow> {
    vec![
        RequirementRow {
            membership: Some(MembershipRow {
                field: "model".to_string(),
                source: "approved-model".to_string(),
                source_kind: "skill".to_string(),
                source_feature: "model".to_string(),
                source_package: None,
            }),
            ..requirement("agents", "skill")
        },
        requirement("approved-model", "skill"),
    ]
}

#[test]
fn a_membership_requirement_fires_when_a_satisfier_is_outside_the_derived_set() {
    let root = tmpdir("membership-bad");
    // The approved set draws `{ opus }` from the lone `approved-model` satisfier; the
    // `agent-gpt` satisfier declares `gpt`, which is not in it.
    write_skill(&root, "agent-gpt", &model_skill("agent-gpt", "gpt"));
    write_skill(
        &root,
        "approved-opus",
        &model_skill("approved-opus", "opus"),
    );
    author_satisfies(&root, "agent-gpt", &["agents"]);
    author_satisfies(&root, "approved-opus", &["approved-model"]);
    write_requirements(&root, membership_requirements());
    write_temper_toml(&root, "");

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
    write_skill(&root, "agent-opus", &model_skill("agent-opus", "opus"));
    write_skill(
        &root,
        "approved-opus",
        &model_skill("approved-opus", "opus"),
    );
    author_satisfies(&root, "agent-opus", &["agents"]);
    author_satisfies(&root, "approved-opus", &["approved-model"]);
    write_requirements(&root, membership_requirements());
    write_temper_toml(&root, "");

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

/// Write the two `approved-model` sources both typed-reference cases share and opt
/// each into `approved-model`: an `official` source carrying `opus` and a `draft`
/// source carrying `gpt`. Under a `conforms_to` = official constraint only the first
/// is a member-contributing source, so `gpt` comes *solely* from a non-conforming
/// source.
fn write_tiered_sources(root: &Path) {
    write_skill(
        root,
        "approved-opus",
        &tiered_skill("approved-opus", "opus", "official"),
    );
    write_skill(
        root,
        "approved-gpt",
        &tiered_skill("approved-gpt", "gpt", "draft"),
    );
    author_satisfies(root, "approved-opus", &["approved-model"]);
    author_satisfies(root, "approved-gpt", &["approved-model"]);
}

// A typed `conforms_to` reference bound to a deliberately-crafted package (here, an
// `enum` clause over a synthetic `tier` field) is no longer reachable end-to-end for
// the same reason as the inadmissible-package case above: every bound-by-name
// package is a compiled-in built-in, and none of the four carries an `enum` clause
// or a `tier` concept to narrow against. The mechanism itself stays proven at the
// unit level (`src/roster.rs`, `a_typed_reference_draws_its_set_only_from_conforming_sources`).

#[test]
fn dropping_the_conforms_to_puts_the_same_value_back_in_the_set() {
    let root = tmpdir("typed-ref-dropped");
    // The exact same corpus, but the membership carries no `conforms_to`: now the
    // non-conforming `approved-gpt` source contributes `gpt` to the derived set, so
    // `agent-gpt` is in-set and the run is silent — the constraint was the only gate.
    write_skill(&root, "agent-gpt", &model_skill("agent-gpt", "gpt"));
    author_satisfies(&root, "agent-gpt", &["agents"]);
    write_tiered_sources(&root);
    write_requirements(&root, membership_requirements());
    write_temper_toml(&root, "");

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
    write_skill(&root, "plan-tasks", &clean_skill("plan-tasks"));

    // The name-`match` selector is gone — fill is opt-in `satisfies` alone. A leftover
    // `match = {…}` is no longer a facet but an unknown key, rejected loudly at load
    // rather than silently dropped (`specs/architecture/10-contracts.md`, "Decision: unknown keys
    // are rejected, not ignored"). This is `temper.toml`'s own parse-time admissibility —
    // unrelated to the lock, so the malformed document is authored directly.
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
    write_skill(&root, "lint-rust", &clean_skill("lint-rust"));

    // Absent `temper.toml`: the floor runs, the clean skill passes.
    let absent = check_in(&root);
    assert!(absent.ok, "the clean skill passes the floor ⇒ zero");

    // A `temper.toml` carrying a `[kind]` layer but no lock-declared requirement
    // declares an empty roster — the roster adds nothing, so the outcome is
    // byte-for-byte the floor's.
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
    write_skill(&root, "plan-tasks", &clean_skill("plan-tasks"));

    // The `[role.*]` surface was hard-cut into `[requirement.*]` by the consolidation.
    // A `temper.toml` that still declares one must fail loudly at load — a silently
    // ignored roster is exactly the gap temper exists to catch — so it is rejected as
    // an unknown top-level key, not dropped. This is `temper.toml`'s own parse-time
    // admissibility — unrelated to the lock, so the malformed document is authored
    // directly.
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

// ---- member-published requirements join the one namespace --------------------

#[test]
fn a_member_published_requirement_filled_by_another_members_satisfies_is_clean() {
    let root = tmpdir("member-published-filled");
    // `arch-spec` publishes a required `[requirement.architecture]` in its own header;
    // `arch-impl` fills it by opting in via `satisfies`. One namespace, the demand
    // published on one surface and the fill claimed on another — coverage resolves the
    // join green, exactly as it does for an assembly-published requirement.
    write_skill(&root, "arch-spec", &clean_skill("arch-spec"));
    write_skill(&root, "arch-impl", &clean_skill("arch-impl"));
    author_published(
        &root,
        "arch-spec",
        vec![published("architecture", Some("skill"), true)],
    );
    author_satisfies(&root, "arch-impl", &["architecture"]);
    write_temper_toml(&root, LAYERED_NO_ROSTER);

    let run = check_in(&root);
    assert!(
        run.ok,
        "a member-published requirement filled by another member's `satisfies` passes ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn an_unfilled_required_member_published_requirement_fires() {
    let root = tmpdir("member-published-unfilled");
    // `arch-spec` publishes a required `[requirement.architecture]`, but no member
    // opts in — the published obligation has no resolving home, so the coverage gate
    // fires exactly as for an unfilled assembly requirement.
    write_skill(&root, "arch-spec", &clean_skill("arch-spec"));
    author_published(
        &root,
        "arch-spec",
        vec![published("architecture", Some("skill"), true)],
    );
    write_temper_toml(&root, LAYERED_NO_ROSTER);

    let run = check_in(&root);
    assert!(
        !run.ok,
        "an unfilled required member-published requirement must fail the run ⇒ non-zero"
    );
    assert!(
        run.output.contains("architecture") && run.output.contains("unfilled"),
        "the finding names the requirement and that it is unfilled, got:\n{}",
        run.output
    );
}

#[test]
fn a_requirement_published_by_two_members_is_an_admissibility_collision() {
    let root = tmpdir("member-published-collision");
    // Two members publish the same requirement name. A requirement lives in one
    // namespace, so the second publisher is a collision — an admissibility finding,
    // never a silent shadow that would let one member quietly redefine another's.
    write_skill(&root, "spec-a", &clean_skill("spec-a"));
    write_skill(&root, "spec-b", &clean_skill("spec-b"));
    author_published(
        &root,
        "spec-a",
        vec![published("shared", Some("skill"), false)],
    );
    author_published(
        &root,
        "spec-b",
        vec![published("shared", Some("skill"), false)],
    );
    write_temper_toml(&root, LAYERED_NO_ROSTER);

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a name published by two members must fail the run ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("shared") && run.output.contains("more than one surface"),
        "the finding names the collided requirement and the cross-publisher collision, got:\n{}",
        run.output
    );
}

#[test]
fn a_name_published_by_both_the_assembly_and_a_member_collides() {
    let root = tmpdir("member-published-assembly-collision");
    // The assembly *and* a member both publish `architecture`. Same namespace, so the
    // member's re-declaration collides with the assembly's — the assembly ⊕ member
    // half of the one-namespace rule.
    write_skill(&root, "arch-spec", &clean_skill("arch-spec"));
    author_published(
        &root,
        "arch-spec",
        vec![published("architecture", Some("skill"), false)],
    );
    write_requirements(&root, vec![requirement("architecture", "skill")]);
    write_temper_toml(&root, "");

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a name published by both the assembly and a member must fail the run ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("architecture") && run.output.contains("more than one surface"),
        "the finding names the collided requirement and the cross-publisher collision, got:\n{}",
        run.output
    );
}
