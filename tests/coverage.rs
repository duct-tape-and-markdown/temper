//! End-to-end acceptance over requirement coverage — the referential shadow of the
//! meaningful contract (`specs/architecture/10-contracts.md`, "Requirements and `satisfies` — the
//! meaningful contract").
//!
//! Drives the built `temper` binary so the whole path is pinned: a golden lock at the
//! project root carrying the declared requirements (`specs/architecture/20-surface.md`,
//! "The lock and drift — one vocabulary" — the gate sources requirements from the lock,
//! never a re-imported assembly), the authored `satisfies` opt-in reconstructed off
//! each surface artifact, and the coverage gate's exit code. Each case sets the process
//! working directory to a project root, exactly as a real invocation would.
//!
//! The cases mirror the entry's acceptance:
//! - a `required` requirement with a skill declaring a resolving `satisfies` stays
//!   silent (covered ⇒ zero);
//! - a `required` requirement with no satisfying artifact fires UNFILLED (⇒ non-zero);
//! - a skill whose `satisfies` names no declared requirement fires DANGLING (⇒ non-zero);
//! - a typo'd link yields the paired UNFILLED+DANGLING — exact-match precision, not one
//!   folding into the other (⇒ non-zero, both rules named);
//! - a duplicated `satisfies` entry emits exactly ONE dangling finding — the loop dedups
//!   per artifact (⇒ non-zero, one finding);
//! - a non-`required` requirement left unfilled does not block (⇒ zero).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::drift::{self, Declarations, EmitOptions, Payload, RequirementRow};

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-coverage-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A clean skill that trips no floor clause — the isolated subject for the coverage
/// gate, so the only findings a case sees are coverage ones.
const CLEAN_SKILL: &str = "---\n\
name: dev-standards\n\
description: Use when maintaining development standards across the harness.\n\
---\n\
# Dev standards\n\
\n\
Keep the bar high.\n";

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

/// A `RequirementRow` naming `name`, `required` otherwise bare — the shape these cases
/// need.
fn requirement(name: &str, required: bool) -> RequirementRow {
    RequirementRow {
        name: name.to_string(),
        kind: None,
        required,
        clauses: Vec::new(),
        verified_by: None,
    }
}

/// Compile a golden lock at `<root>/.temper/lock.toml` carrying just the declared
/// `requirements` — the SDK-emitted fixture standing in for `import::run`'s scratch
/// projection of the retired manifest's `[requirement.*]` table: the gate sources
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

/// Run `temper check --reporter github` from `root`, so findings render as one
/// `::error` workflow-command line per diagnostic — a stable substrate for counting
/// how many findings a case emits.
fn check_github(root: &Path) -> CheckRun {
    let out = Command::new(BIN)
        .current_dir(root)
        .arg("check")
        .arg("--reporter")
        .arg("github")
        .output()
        .unwrap();
    let mut output = String::from_utf8_lossy(&out.stdout).into_owned();
    output.push_str(&String::from_utf8_lossy(&out.stderr));
    CheckRun {
        ok: out.status.success(),
        output,
    }
}

#[test]
fn a_required_requirement_with_a_resolving_satisfies_stays_silent() {
    let root = tmpdir("covered");
    write_skill(&root, "dev-standards", CLEAN_SKILL);
    // The skill opts into the requirement, so its intent has a resolving home.
    author_satisfies(&root, "dev-standards", &["dev-standards"]);
    write_requirements(&root, vec![requirement("dev-standards", true)]);

    let run = check_in(&root);
    assert!(
        run.ok,
        "a covered required requirement must not block ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_required_requirement_with_no_satisfying_artifact_fires_unfilled() {
    let root = tmpdir("unfilled");
    write_skill(&root, "dev-standards", CLEAN_SKILL);
    // No `satisfies` authored: nothing opts into the requirement.
    write_requirements(&root, vec![requirement("dev-standards", true)]);

    let run = check_in(&root);
    assert!(
        !run.ok,
        "an unfilled required requirement must block ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("requirement.unfilled"),
        "the finding names the unfilled rule, got:\n{}",
        run.output
    );
}

#[test]
fn a_satisfies_naming_no_requirement_fires_dangling() {
    let root = tmpdir("dangling");
    write_skill(&root, "dev-standards", CLEAN_SKILL);
    // The skill opts into the required requirement (so no UNFILLED) *and* a second,
    // undeclared one — the link that dangles.
    author_satisfies(
        &root,
        "dev-standards",
        &["dev-standards", "ghost-requirement"],
    );
    write_requirements(&root, vec![requirement("dev-standards", true)]);

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a dangling `satisfies` must block ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("requirement.dangling"),
        "the finding names the dangling rule, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("ghost-requirement"),
        "the finding names the unresolvable link, got:\n{}",
        run.output
    );
}

#[test]
fn a_typo_in_a_satisfies_link_yields_paired_unfilled_and_dangling() {
    let root = tmpdir("typo");
    write_skill(&root, "dev-standards", CLEAN_SKILL);
    // The link misspells the requirement name. `satisfies` is exact-string matched,
    // never folded, so the real requirement goes UNFILLED (nothing resolves to it)
    // *and* the typo'd name DANGLES (it names no declared requirement). Both are true
    // positives — the pair is what pins exact-match precision, not one masking the
    // other.
    author_satisfies(&root, "dev-standards", &["dev-standatds"]);
    write_requirements(&root, vec![requirement("dev-standards", true)]);

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a typo'd link must block on both counts ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("requirement.unfilled"),
        "the real requirement is unfilled, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("requirement.dangling"),
        "the misspelled link dangles, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("dev-standatds"),
        "the dangling finding names the typo, got:\n{}",
        run.output
    );
}

#[test]
fn a_duplicated_satisfies_entry_emits_exactly_one_dangling() {
    let root = tmpdir("dup");
    write_skill(&root, "dev-standards", CLEAN_SKILL);
    // The skill covers the declared requirement (so no UNFILLED) and repeats the same
    // undeclared link. The coverage check dedups each artifact's `satisfies` before
    // the dangling loop, so the single unresolvable name yields exactly ONE
    // diagnostic — a duplicated link is not a doubled fault.
    author_satisfies(
        &root,
        "dev-standards",
        &["dev-standards", "ghost-requirement", "ghost-requirement"],
    );
    write_requirements(&root, vec![requirement("dev-standards", true)]);

    // The github reporter renders one `::error` line per diagnostic, a stable count.
    let run = check_github(&root);
    assert!(
        !run.ok,
        "a dangling link must block ⇒ non-zero, got:\n{}",
        run.output
    );
    let dangling = run.output.matches("requirement.dangling").count();
    assert_eq!(
        dangling, 1,
        "a duplicated `satisfies` must emit exactly one dangling finding, got {dangling} in:\n{}",
        run.output
    );
    // And no spurious unfilled — the requirement is covered by the first link.
    assert_eq!(
        run.output.matches("requirement.unfilled").count(),
        0,
        "the covered requirement must not fire unfilled, got:\n{}",
        run.output
    );
}

#[test]
fn a_non_required_unfilled_requirement_does_not_block() {
    let root = tmpdir("advisory");
    write_skill(&root, "dev-standards", CLEAN_SKILL);
    // Nothing opts into it, but the requirement is advisory intent (no `required`),
    // so `temper` never fabricates a gate the author did not declare.
    write_requirements(&root, vec![requirement("nice-to-have", false)]);

    let run = check_in(&root);
    assert!(
        run.ok,
        "a non-required unfilled requirement must not block ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_kind_blind_required_requirement_with_no_satisfier_still_fires_unfilled() {
    // Custom kinds retire with the KIND.md file format (`specs/architecture/15-kinds.md`,
    // "Decision: field typing lives in the SDK — there is no kind file format"), and
    // there is no longer any manifest to register one from at all (the manifest
    // retires entirely) — so a kind-blind `required` requirement contributes no
    // fabricated coverage; it still fires UNFILLED absent a real satisfier.
    let root = tmpdir("custom-unfilled");
    write_requirements(&root, vec![requirement("domain-model", true)]);

    let run = check_in(&root);
    assert!(
        !run.ok,
        "an unfilled required requirement must block ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("requirement.unfilled"),
        "the finding names the unfilled rule, got:\n{}",
        run.output
    );
}

#[test]
fn a_means_less_required_requirement_still_gates() {
    let root = tmpdir("means-less");
    write_skill(&root, "dev-standards", CLEAN_SKILL);
    // The unified requirement makes `means` optional (`specs/architecture/10-contracts.md`, "all
    // facets optional except its name"), but coverage keys off `required`, not
    // `means`: a `required` requirement with no `means` and nothing opting in still
    // fires UNFILLED and blocks the run.
    write_requirements(&root, vec![requirement("dev-standards", true)]);

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a means-less required requirement left unfilled must block ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("requirement.unfilled"),
        "the finding names the unfilled rule, got:\n{}",
        run.output
    );
}

/// Write a floor-clean rule directly at its real Claude Code locus
/// (`<root>/.claude/rules/<name>.md`) — a second modeled kind, so a requirement typed
/// to `skill` can be filled by an opt-in of a *different* kind.
fn write_rule(root: &Path, name: &str) {
    let dir = root.join(".claude").join("rules");
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join(format!("{name}.md")),
        format!("# {name}\n\nBody.\n"),
    )
    .unwrap();
}

/// Author a rule's `satisfies` links on its surface overlay — the mirror of
/// [`author_satisfies`] for the `rule` kind.
fn author_rule_satisfies(root: &Path, name: &str, requirements: &[&str]) {
    let rule_kind = temper::builtin_kind::definition("rule").unwrap().unwrap();
    let source = root
        .join(".claude")
        .join("rules")
        .join(format!("{name}.md"));
    let mut rule = temper::frontmatter::Member::from_source(&rule_kind, &source).unwrap();
    rule.satisfies = requirements
        .iter()
        .map(|r| temper::document::Satisfies::new(*r))
        .collect();

    let dir = root.join(".temper").join("rules").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("RULE.md"), rule.to_document().emit()).unwrap();
}

#[test]
fn a_required_requirement_is_covered_by_a_rules_opt_in_same_as_a_skills() {
    // Coverage draws the satisfier set kind-blind — a member of *any* modeled kind
    // that opts in via `satisfies` counts toward the requirement, matching the
    // unified roster/graph satisfier set (`specs/model/contract.md`, "selection").
    // A `rule`'s opt-in covers `dev-standards` exactly as a skill's would.
    let root = tmpdir("kind-blind-cover");
    write_rule(&root, "dev-standards-rule");
    author_rule_satisfies(&root, "dev-standards-rule", &["dev-standards"]);
    write_requirements(&root, vec![requirement("dev-standards", true)]);

    let run = check_in(&root);
    assert!(
        run.ok,
        "a rule's opt-in covers a required requirement ⇒ zero, got:\n{}",
        run.output
    );
}
