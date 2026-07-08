//! End-to-end acceptance over the harness-contract roster — the set-scope
//! predicates (`count` / `unique` / `membership`), each quantified over a
//! requirement's **satisfier set**.
//!
//! Drives the built `temper` binary so the whole path is pinned: a golden lock at the
//! project root carrying the declared requirements, and running the roster over the harness's live
//! skills and their authored `satisfies` opt-in. The name-`match` selector is
//! eradicated — opt-in `satisfies` is the sole fill — so a satisfier set is the
//! artifacts of a requirement's `kind` whose `satisfies` names it.
//!
//! The cases mirror the entry's acceptance:
//! - the `count` cardinality bound quantifies over the satisfier set;
//! - the `unique` predicate quantifies over the satisfier set;
//! - the `membership` predicate draws its allowed set from a *second* satisfier set;
//! - a `match = {…}` key, and the retired `[role.*]` surface, are inert — a
//!   the retired manifest is never read at all, so a stray one carrying either changes
//!   nothing;
//! - the roster is itself checked (admissibility);
//! - a stray retired manifest declaring no roster leaves the floor outcome unchanged.

use std::fs;
use std::path::Path;
use std::process::Command;

mod common;

use temper::drift::{self, ClauseRow, Declarations, EmitOptions, Payload, RequirementRow};

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

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
/// (`kind` and `required`; `means` unused by these cases).
fn published(
    name: &str,
    kind: Option<&str>,
    required: bool,
) -> temper::document::PublishedRequirement {
    temper::document::PublishedRequirement {
        name: name.to_string(),
        means: None,
        kind: kind.map(str::to_string),
        required,
    }
}

/// Run `temper check --harness <root>` — the one-shot wedge, gating `root` directly
/// rather than through the two-step `./.temper` default. `root` already carries its own
/// `.temper/` surface (`write_requirements`/`author_satisfies` project it there), so
/// this exercises the one-shot gate's surface-present branch: its lock's declared
/// requirement/satisfies rows must gate exactly as the two-step path's do.
fn check_harness_in(root: &Path) -> common::CheckRun {
    let out = Command::new(BIN)
        .arg("check")
        .arg("--harness")
        .arg(root)
        .output()
        .unwrap();
    let mut output = String::from_utf8_lossy(&out.stdout).into_owned();
    output.push_str(&String::from_utf8_lossy(&out.stderr));
    common::CheckRun {
        ok: out.status.success(),
        output,
    }
}

/// A required-severity `ClauseRow` wrapping one set-scope predicate — the shape
/// every case below nests on a `RequirementRow`'s `clauses`. `kind` is `None`: a
/// nested requirement clause names no kind of its own.
fn required_clause_row(
    predicate: &str,
    field: Option<&str>,
    count: Option<temper::drift::CountBoundRow>,
    target: Option<&str>,
    degree: Option<temper::drift::DegreeBoundRow>,
) -> ClauseRow {
    ClauseRow {
        kind: None,
        predicate: predicate.to_string(),
        field: field.map(str::to_string),
        severity: "required".to_string(),
        guidance: None,
        cite: None,
        count,
        target: target.map(str::to_string),
        degree,
        bound: None,
        charset: None,
        keys: None,
        values: None,
    }
}

/// Compile a golden lock at `<root>/.temper/lock.toml` carrying just the declared
/// `clauses` — the SDK-emitted fixture standing in for an `expect` binding's
/// erasure (`sdk/src/declarations.ts`): the gate's per-kind contract sources its
/// clause/severity overrides from the lock's `ClauseRow` family, never a
/// re-imported manifest `[kind.*]` layer.
fn write_clauses(root: &Path, clauses: Vec<ClauseRow>) {
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            clauses,
            ..Declarations::default()
        },
        members: Vec::new(),
    };
    drift::emit(&payload, &root.join(".temper"), EmitOptions::default()).unwrap();
}

/// A skill carrying the Cursor `globs` key — the skill floor's `forbidden_keys`
/// clause forbids it at `required` severity, a
/// real, otherwise floor-clean violation to flip the verdict on.
fn skill_with_forbidden_key(name: &str) -> String {
    format!(
        "---\n\
         name: {name}\n\
         description: Use when {name} is the task at hand; not for anything else.\n\
         globs: \"**/*.rs\"\n\
         ---\n\
         # {name}\n\
         \n\
         Body.\n"
    )
}

#[test]
fn a_lock_declared_clause_severity_override_gates_but_a_temper_toml_only_one_is_inert() {
    let root = common::tmpdir("clause-override-from-lock");
    common::write_skill(
        &root,
        "legacy-rule",
        &skill_with_forbidden_key("legacy-rule"),
    );

    // The floor's `forbidden_keys` is `required`: the `globs` key fails the run.
    let floor = common::check_in(&root, &[], None);
    assert!(
        !floor.ok,
        "the floor's required forbidden_keys must fail the run over a `globs` key, got:\n{}",
        floor.output
    );

    // The identical override, written only in a retired-manifest `[kind.skill]`
    // layer, is inert — the manifest is never read at all any more (`TEMPER-TOML-ZERO`),
    // so a stray one carrying a clause override changes nothing.
    common::write_retired_manifest(
        &root,
        "[kind.skill]\n\
         [[kind.skill.clause]]\n\
         severity = \"advisory\"\n\
         predicate = \"forbidden_keys\"\n\
         keys = [\"globs\", \"alwaysApply\"]\n",
    );
    let toml_only = common::check_in(&root, &[], None);
    assert!(
        !toml_only.ok,
        "a manifest-only clause override must not change the verdict ⇒ still non-zero, got:\n{}",
        toml_only.output
    );

    // The same override declared in the lock's `[[declaration.clause]]` rows
    // flips the clause's severity to advisory, so the same violation no longer
    // blocks the run.
    write_clauses(
        &root,
        vec![ClauseRow {
            kind: Some("skill".to_string()),
            predicate: "forbidden_keys".to_string(),
            field: None,
            severity: "advisory".to_string(),
            guidance: None,
            cite: None,
            count: None,
            target: None,
            degree: None,
            bound: None,
            charset: None,
            keys: None,
            values: None,
        }],
    );
    let lock_override = common::check_in(&root, &[], None);
    assert!(
        lock_override.ok,
        "a lock-declared clause severity override must change the verdict ⇒ zero, got:\n{}",
        lock_override.output
    );
}

// ---- set scope: the `count` cardinality bound over the satisfier set -------

/// The `agents` requirement's `count` row bounding its satisfier-set cardinality to
/// `[min, max]` — the set-scope `count` predicate. No `required` flag rides alongside
/// (`count` is its general form). The satisfiers are the skills opting into `agents`.
fn count_band_requirement(min: usize, max: usize) -> RequirementRow {
    RequirementRow {
        clauses: vec![required_clause_row(
            "count",
            None,
            Some(temper::drift::CountBoundRow { min, max }),
            None,
            None,
        )],
        ..common::requirement("agents", false, Some("skill"))
    }
}

#[test]
fn a_count_band_fires_when_the_satisfier_set_is_out_of_band() {
    let root = common::tmpdir("count-over");
    // Two skills opt into `agents`; the band caps the satisfier count at one, so two
    // is out of band.
    common::write_skill(&root, "agent-one", &common::clean_skill("agent-one"));
    common::write_skill(&root, "agent-two", &common::clean_skill("agent-two"));
    common::author_satisfies(&root, "skills", "agent-one", &["agents"]);
    common::author_satisfies(&root, "skills", "agent-two", &["agents"]);
    common::write_requirements(&root, vec![count_band_requirement(0, 1)]);

    let run = common::check_in(&root, &[], None);
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

    // The one-shot `check --harness` gate over the identical already-emitted harness
    // must reach the same finding — a locked `count` clause is not two-step-only.
    let harness_run = check_harness_in(&root);
    assert!(
        !harness_run.ok,
        "check --harness must also fail the run on the same out-of-band count ⇒ non-zero, got:\n{}",
        harness_run.output
    );
    assert!(
        harness_run.output.contains("agents")
            && harness_run.output.contains("agent-one")
            && harness_run.output.contains("agent-two")
            && harness_run.output.contains("[0, 1]"),
        "the one-shot gate's finding names the requirement, the satisfiers, and the bound, got:\n{}",
        harness_run.output
    );
}

#[test]
fn a_count_band_is_clean_within_bounds() {
    let root = common::tmpdir("count-ok");
    // Two skills opt into `agents`, inside a `[1, 2]` band — clean.
    common::write_skill(&root, "agent-one", &common::clean_skill("agent-one"));
    common::write_skill(&root, "agent-two", &common::clean_skill("agent-two"));
    common::author_satisfies(&root, "skills", "agent-one", &["agents"]);
    common::author_satisfies(&root, "skills", "agent-two", &["agents"]);
    common::write_requirements(&root, vec![count_band_requirement(1, 2)]);

    let run = common::check_in(&root, &[], None);
    assert!(
        run.ok,
        "a satisfier count inside the band passes ⇒ zero, got:\n{}",
        run.output
    );
}

// ---- each grain: the `kind` narrowing clause `requirement.kind` sources ----

#[test]
fn a_wrong_kind_opt_in_fires_a_kind_finding_never_a_silent_exclusion() {
    let root = common::tmpdir("kind-wrong");
    // `agents` narrows to `skill`, but a `rule` also opts in via `satisfies`. The
    // satisfier set is kind-blind, so the rule is drawn in — and the each-grain
    // `kind` clause `requirement.kind` sources flags it as a finding rather than
    // silently excluding it from the set.
    common::write_skill(&root, "agent-skill", &common::clean_skill("agent-skill"));
    common::write_rule(&root, "agent-rule");
    common::author_satisfies(&root, "skills", "agent-skill", &["agents"]);
    common::author_rule_satisfies(&root, "agent-rule", &["agents"]);
    common::write_requirements(
        &root,
        vec![common::requirement("agents", false, Some("skill"))],
    );

    let run = common::check_in(&root, &[], None);
    assert!(
        !run.ok,
        "a wrong-kind opt-in must fail the run ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("agents")
            && run.output.contains("agent-rule")
            && run.output.contains("skill"),
        "the finding names the requirement, the wrong-kind satisfier, and the declared kind, got:\n{}",
        run.output
    );
}

#[test]
fn a_kind_blind_requirement_is_filled_by_opt_ins_of_every_modeled_kind() {
    let root = common::tmpdir("kind-blind");
    // No `kind` at all: a skill and a rule both opt in, and neither is a finding —
    // a kind-blind requirement attaches no narrowing clause.
    common::write_skill(&root, "agent-skill", &common::clean_skill("agent-skill"));
    common::write_rule(&root, "agent-rule");
    common::author_satisfies(&root, "skills", "agent-skill", &["agents"]);
    common::author_rule_satisfies(&root, "agent-rule", &["agents"]);
    common::write_requirements(
        &root,
        vec![RequirementRow {
            kind: None,
            ..common::requirement("agents", false, Some("skill"))
        }],
    );

    let run = common::check_in(&root, &[], None);
    assert!(
        run.ok,
        "opt-ins of every modeled kind fill a kind-blind requirement ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_kind_narrowing_an_unmodeled_kind_is_inadmissible() {
    let root = common::tmpdir("kind-unmodeled");
    // A floor-clean skill opts in (coverage is satisfied), but the requirement
    // narrows to `command` — a kind `temper` does not model — so the each-grain
    // clause it sources can never hold for any satisfier: an admissibility finding,
    // never a silent "can never be filled" exclusion.
    common::write_skill(&root, "agent-skill", &common::clean_skill("agent-skill"));
    common::author_satisfies(&root, "skills", "agent-skill", &["agents"]);
    common::write_requirements(
        &root,
        vec![common::requirement("agents", false, Some("command"))],
    );

    let run = common::check_in(&root, &[], None);
    assert!(
        !run.ok,
        "a kind clause naming an unmodeled kind must fail the run ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("agents") && run.output.contains("command"),
        "the finding names the requirement and the unmodeled kind, got:\n{}",
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
    let root = common::tmpdir("unique-bad");
    // Two `agents` satisfiers share `model = opus`; `unique = ["model"]` requires each
    // distinct across the satisfier set.
    common::write_skill(&root, "agent-a", &model_skill("agent-a", "opus"));
    common::write_skill(&root, "agent-b", &model_skill("agent-b", "opus"));
    common::author_satisfies(&root, "skills", "agent-a", &["agents"]);
    common::author_satisfies(&root, "skills", "agent-b", &["agents"]);
    common::write_requirements(
        &root,
        vec![RequirementRow {
            clauses: vec![required_clause_row(
                "unique",
                Some("model"),
                None,
                None,
                None,
            )],
            ..common::requirement("agents", false, Some("skill"))
        }],
    );

    let run = common::check_in(&root, &[], None);
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
    let root = common::tmpdir("admit-unknown-kind");
    // A floor-clean skill opts into the requirement (so coverage is satisfied), but the
    // requirement is typed to `command` — a kind `temper` does not model — so a
    // required requirement over it can never be filled.
    common::write_skill(&root, "lint-rust", &common::clean_skill("lint-rust"));
    common::author_satisfies(&root, "skills", "lint-rust", &["releaser"]);
    common::write_requirements(
        &root,
        vec![RequirementRow {
            required: true,
            ..common::requirement("releaser", false, Some("command"))
        }],
    );

    let run = common::check_in(&root, &[], None);
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
fn a_requirement_with_a_dangling_verified_by_is_inadmissible() {
    let root = common::tmpdir("admit-dangling-verifier");
    // Coverage is clean (a satisfier opts in); the sole fault is `verified_by`
    // naming a path that does not exist under the root.
    common::write_skill(&root, "plan-tasks", &common::clean_skill("plan-tasks"));
    common::author_satisfies(&root, "skills", "plan-tasks", &["planner"]);
    common::write_requirements(
        &root,
        vec![RequirementRow {
            required: true,
            verified_by: Some("tests/does-not-exist.rs".to_string()),
            ..common::requirement("planner", false, Some("skill"))
        }],
    );

    let run = common::check_in(&root, &[], None);
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
fn a_roster_whose_verifiers_all_resolve_passes() {
    let root = common::tmpdir("admit-clean");
    common::write_skill(&root, "plan-tasks", &common::clean_skill("plan-tasks"));
    common::author_satisfies(&root, "skills", "plan-tasks", &["planner"]);

    // A `verified_by` path that exists under the root — nothing else for
    // admissibility to reject.
    fs::write(root.join("plan.rs"), "// a present verifier\n").unwrap();
    common::write_requirements(
        &root,
        vec![RequirementRow {
            required: true,
            verified_by: Some("plan.rs".to_string()),
            ..common::requirement("planner", false, Some("skill"))
        }],
    );

    let run = common::check_in(&root, &[], None);
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
/// allowed set. `target` names a *declared* requirement, so the approved skills'
/// `satisfies` link resolves, leaving membership the only gate these cases exercise.
fn membership_requirements() -> Vec<RequirementRow> {
    vec![
        RequirementRow {
            clauses: vec![required_clause_row(
                "membership",
                Some("model"),
                None,
                Some("approved-model"),
                None,
            )],
            ..common::requirement("agents", false, Some("skill"))
        },
        common::requirement("approved-model", false, Some("skill")),
    ]
}

#[test]
fn a_membership_requirement_fires_when_a_satisfier_is_outside_the_derived_set() {
    let root = common::tmpdir("membership-bad");
    // The approved set draws `{ opus }` from the lone `approved-model` satisfier; the
    // `agent-gpt` satisfier declares `gpt`, which is not in it.
    common::write_skill(&root, "agent-gpt", &model_skill("agent-gpt", "gpt"));
    common::write_skill(
        &root,
        "approved-opus",
        &model_skill("approved-opus", "opus"),
    );
    common::author_satisfies(&root, "skills", "agent-gpt", &["agents"]);
    common::author_satisfies(&root, "skills", "approved-opus", &["approved-model"]);
    common::write_requirements(&root, membership_requirements());

    let run = common::check_in(&root, &[], None);
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
    let root = common::tmpdir("membership-ok");
    // The `agent-opus` satisfier's `model` is drawn from the approved set `{ opus }`,
    // so membership is satisfied and the whole run is clean.
    common::write_skill(&root, "agent-opus", &model_skill("agent-opus", "opus"));
    common::write_skill(
        &root,
        "approved-opus",
        &model_skill("approved-opus", "opus"),
    );
    common::author_satisfies(&root, "skills", "agent-opus", &["agents"]);
    common::author_satisfies(&root, "skills", "approved-opus", &["approved-model"]);
    common::write_requirements(&root, membership_requirements());

    let run = common::check_in(&root, &[], None);
    assert!(
        run.ok,
        "every satisfier drawn from the derived set passes ⇒ zero, got:\n{}",
        run.output
    );
}

// ---- the name-`match` selector is eradicated, and so is the manifest itself ----

#[test]
fn a_retired_match_key_in_a_stray_temper_toml_is_inert() {
    let root = common::tmpdir("match-unknown-key");
    common::write_skill(&root, "plan-tasks", &common::clean_skill("plan-tasks"));

    // The name-`match` selector was gone even before this — fill is opt-in `satisfies`
    // alone. Now the whole file is: the manifest is never read at all, so this
    // once-rejected syntax changes nothing rather than failing to load.
    common::write_retired_manifest(
        &root,
        "[requirement.planner]\n\
         kind = \"skill\"\n\
         match = { name = \"plan*\" }\n\
         required = true\n",
    );

    let run = common::check_in(&root, &[], None);
    assert!(
        run.ok,
        "a stray manifest, whatever it carries, is never read ⇒ the clean skill still passes, got:\n{}",
        run.output
    );
}

#[test]
fn a_temper_toml_declaring_no_roster_leaves_the_floor_outcome_unchanged() {
    let root = common::tmpdir("no-roster");
    common::write_skill(&root, "lint-rust", &common::clean_skill("lint-rust"));

    // Absent the retired manifest: the floor runs, the clean skill passes.
    let absent = common::check_in(&root, &[], None);
    assert!(absent.ok, "the clean skill passes the floor ⇒ zero");

    // A retired manifest present on disk at all — never read, so it declares nothing —
    // leaves the outcome byte-for-byte the floor's.
    common::write_retired_manifest(
        &root,
        "[kind.skill]\n\
         package = \"skill.anthropic\"\n",
    );
    let no_roster = common::check_in(&root, &[], None);
    assert!(
        no_roster.ok,
        "an unread manifest changes nothing ⇒ still zero"
    );
    assert_eq!(
        absent.output, no_roster.output,
        "a stray manifest must produce identical output to none"
    );
}

#[test]
fn a_retired_role_table_in_a_stray_temper_toml_is_inert() {
    let root = common::tmpdir("retired-role");
    common::write_skill(&root, "plan-tasks", &common::clean_skill("plan-tasks"));

    // The `[role.*]` surface was hard-cut into `[requirement.*]` by the consolidation,
    // and used to be rejected loudly at load. Now the manifest is never read at all,
    // so this once-rejected root changes nothing rather than failing to load.
    common::write_retired_manifest(
        &root,
        "[role.planner]\n\
         artifact = \"skill\"\n\
         required = true\n",
    );

    let run = common::check_in(&root, &[], None);
    assert!(
        run.ok,
        "a stray manifest, whatever it carries, is never read ⇒ the clean skill still passes, got:\n{}",
        run.output
    );
}

// ---- member-published requirements join the one namespace --------------------

#[test]
fn a_member_published_requirement_filled_by_another_members_satisfies_is_clean() {
    let root = common::tmpdir("member-published-filled");
    // `arch-spec` publishes a required `[requirement.architecture]` in its own header;
    // `arch-impl` fills it by opting in via `satisfies`. One namespace, the demand
    // published on one surface and the fill claimed on another — coverage resolves the
    // join green, exactly as it does for an assembly-published requirement.
    common::write_skill(&root, "arch-spec", &common::clean_skill("arch-spec"));
    common::write_skill(&root, "arch-impl", &common::clean_skill("arch-impl"));
    author_published(
        &root,
        "arch-spec",
        vec![published("architecture", Some("skill"), true)],
    );
    common::author_satisfies(&root, "skills", "arch-impl", &["architecture"]);

    let run = common::check_in(&root, &[], None);
    assert!(
        run.ok,
        "a member-published requirement filled by another member's `satisfies` passes ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn an_unfilled_required_member_published_requirement_fires() {
    let root = common::tmpdir("member-published-unfilled");
    // `arch-spec` publishes a required `[requirement.architecture]`, but no member
    // opts in — the published obligation has no resolving home, so the coverage gate
    // fires exactly as for an unfilled assembly requirement.
    common::write_skill(&root, "arch-spec", &common::clean_skill("arch-spec"));
    author_published(
        &root,
        "arch-spec",
        vec![published("architecture", Some("skill"), true)],
    );

    let run = common::check_in(&root, &[], None);
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
    let root = common::tmpdir("member-published-collision");
    // Two members publish the same requirement name. A requirement lives in one
    // namespace, so the second publisher is a collision — an admissibility finding,
    // never a silent shadow that would let one member quietly redefine another's.
    common::write_skill(&root, "spec-a", &common::clean_skill("spec-a"));
    common::write_skill(&root, "spec-b", &common::clean_skill("spec-b"));
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

    let run = common::check_in(&root, &[], None);
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
    let root = common::tmpdir("member-published-assembly-collision");
    // The assembly *and* a member both publish `architecture`. Same namespace, so the
    // member's re-declaration collides with the assembly's — the assembly ⊕ member
    // half of the one-namespace rule.
    common::write_skill(&root, "arch-spec", &common::clean_skill("arch-spec"));
    author_published(
        &root,
        "arch-spec",
        vec![published("architecture", Some("skill"), false)],
    );
    common::write_requirements(
        &root,
        vec![common::requirement("architecture", false, Some("skill"))],
    );

    let run = common::check_in(&root, &[], None);
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
