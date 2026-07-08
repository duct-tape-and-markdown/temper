//! End-to-end acceptance over requirement coverage — the referential shadow of the
//! meaningful contract.
//!
//! Drives the built `temper` binary so the whole path is pinned: a golden lock at the
//! project root carrying the declared requirements, the authored `satisfies` opt-in reconstructed off
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
use std::path::Path;

mod common;

use temper::drift::RequirementRow;

/// A clean skill that trips no floor clause — the isolated subject for the coverage
/// gate, so the only findings a case sees are coverage ones.
const CLEAN_SKILL: &str = "---\n\
name: dev-standards\n\
description: Use when maintaining development standards across the harness.\n\
---\n\
# Dev standards\n\
\n\
Keep the bar high.\n";

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

#[test]
fn a_required_requirement_with_a_resolving_satisfies_stays_silent() {
    let root = common::tmpdir("covered");
    common::write_skill(&root, "dev-standards", CLEAN_SKILL);
    // The skill opts into the requirement, so its intent has a resolving home.
    common::author_satisfies(&root, "skills", "dev-standards", &["dev-standards"]);
    common::write_requirements(&root, vec![requirement("dev-standards", true)]);

    let run = common::check_in(&root, &[], None);
    assert!(
        run.ok,
        "a covered required requirement must not block ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_required_requirement_with_no_satisfying_artifact_fires_unfilled() {
    let root = common::tmpdir("unfilled");
    common::write_skill(&root, "dev-standards", CLEAN_SKILL);
    // No `satisfies` authored: nothing opts into the requirement.
    common::write_requirements(&root, vec![requirement("dev-standards", true)]);

    let run = common::check_in(&root, &[], None);
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
    let root = common::tmpdir("dangling");
    common::write_skill(&root, "dev-standards", CLEAN_SKILL);
    // The skill opts into the required requirement (so no UNFILLED) *and* a second,
    // undeclared one — the link that dangles.
    common::author_satisfies(
        &root,
        "skills",
        "dev-standards",
        &["dev-standards", "ghost-requirement"],
    );
    common::write_requirements(&root, vec![requirement("dev-standards", true)]);

    let run = common::check_in(&root, &[], None);
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
    let root = common::tmpdir("typo");
    common::write_skill(&root, "dev-standards", CLEAN_SKILL);
    // The link misspells the requirement name. `satisfies` is exact-string matched,
    // never folded, so the real requirement goes UNFILLED (nothing resolves to it)
    // *and* the typo'd name DANGLES (it names no declared requirement). Both are true
    // positives — the pair is what pins exact-match precision, not one masking the
    // other.
    common::author_satisfies(&root, "skills", "dev-standards", &["dev-standatds"]);
    common::write_requirements(&root, vec![requirement("dev-standards", true)]);

    let run = common::check_in(&root, &[], None);
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
    let root = common::tmpdir("dup");
    common::write_skill(&root, "dev-standards", CLEAN_SKILL);
    // The skill covers the declared requirement (so no UNFILLED) and repeats the same
    // undeclared link. The coverage check dedups each artifact's `satisfies` before
    // the dangling loop, so the single unresolvable name yields exactly ONE
    // diagnostic — a duplicated link is not a doubled fault.
    common::author_satisfies(
        &root,
        "skills",
        "dev-standards",
        &["dev-standards", "ghost-requirement", "ghost-requirement"],
    );
    common::write_requirements(&root, vec![requirement("dev-standards", true)]);

    // The github reporter renders one `::error` line per diagnostic, a stable count.
    let run = common::check_in(&root, &[], Some("github"));
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
    let root = common::tmpdir("advisory");
    common::write_skill(&root, "dev-standards", CLEAN_SKILL);
    // Nothing opts into it, but the requirement is advisory intent (no `required`),
    // so `temper` never fabricates a gate the author did not declare.
    common::write_requirements(&root, vec![requirement("nice-to-have", false)]);

    let run = common::check_in(&root, &[], None);
    assert!(
        run.ok,
        "a non-required unfilled requirement must not block ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_kind_blind_required_requirement_with_no_satisfier_still_fires_unfilled() {
    // Custom kinds retire with the KIND.md file format, and
    // there is no longer any manifest to register one from at all (the manifest
    // retires entirely) — so a kind-blind `required` requirement contributes no
    // fabricated coverage; it still fires UNFILLED absent a real satisfier.
    let root = common::tmpdir("custom-unfilled");
    common::write_requirements(&root, vec![requirement("domain-model", true)]);

    let run = common::check_in(&root, &[], None);
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
    let root = common::tmpdir("means-less");
    common::write_skill(&root, "dev-standards", CLEAN_SKILL);
    // The unified requirement makes `means` optional, but coverage keys off `required`, not
    // `means`: a `required` requirement with no `means` and nothing opting in still
    // fires UNFILLED and blocks the run.
    common::write_requirements(&root, vec![requirement("dev-standards", true)]);

    let run = common::check_in(&root, &[], None);
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
    // unified roster/graph satisfier set.
    // A `rule`'s opt-in covers `dev-standards` exactly as a skill's would.
    let root = common::tmpdir("kind-blind-cover");
    common::write_rule(&root, "dev-standards-rule");
    author_rule_satisfies(&root, "dev-standards-rule", &["dev-standards"]);
    common::write_requirements(&root, vec![requirement("dev-standards", true)]);

    let run = common::check_in(&root, &[], None);
    assert!(
        run.ok,
        "a rule's opt-in covers a required requirement ⇒ zero, got:\n{}",
        run.output
    );
}
