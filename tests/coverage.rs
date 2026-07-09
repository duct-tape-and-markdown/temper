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

use temper::drift::{Declarations, KindFactRow};

mod common;

/// A clean skill that trips no floor clause — the isolated subject for the coverage
/// gate, so the only findings a case sees are coverage ones.
const CLEAN_SKILL: &str = "---\n\
name: dev-standards\n\
description: Use when maintaining development standards across the harness.\n\
---\n\
# Dev standards\n\
\n\
Keep the bar high.\n";

#[test]
fn a_required_requirement_with_a_resolving_satisfies_stays_silent() {
    let root = common::tmpdir("covered");
    common::write_skill(&root, "dev-standards", CLEAN_SKILL);
    // The skill opts into the requirement, so its intent has a resolving home.
    common::author_satisfies(&root, "skills", "dev-standards", &["dev-standards"]);
    common::write_requirements(
        &root,
        vec![common::requirement("dev-standards", true, None)],
    );

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
    common::write_requirements(
        &root,
        vec![common::requirement("dev-standards", true, None)],
    );

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
    common::write_requirements(
        &root,
        vec![common::requirement("dev-standards", true, None)],
    );

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
    common::write_requirements(
        &root,
        vec![common::requirement("dev-standards", true, None)],
    );

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
    common::write_requirements(
        &root,
        vec![common::requirement("dev-standards", true, None)],
    );

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
    common::write_requirements(
        &root,
        vec![common::requirement("nice-to-have", false, None)],
    );

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
    common::write_requirements(&root, vec![common::requirement("domain-model", true, None)]);

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
    common::write_requirements(
        &root,
        vec![common::requirement("dev-standards", true, None)],
    );

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

#[test]
fn a_required_requirement_is_covered_by_a_rules_opt_in_same_as_a_skills() {
    // Coverage draws the satisfier set kind-blind — a member of *any* modeled kind
    // that opts in via `satisfies` counts toward the requirement, matching the
    // unified roster/graph satisfier set.
    // A `rule`'s opt-in covers `dev-standards` exactly as a skill's would.
    let root = common::tmpdir("kind-blind-cover");
    common::write_rule(&root, "dev-standards-rule");
    common::author_rule_satisfies(&root, "dev-standards-rule", &["dev-standards"]);
    common::write_requirements(
        &root,
        vec![common::requirement("dev-standards", true, None)],
    );

    let run = common::check_in(&root, &[], None);
    assert!(
        run.ok,
        "a rule's opt-in covers a required requirement ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_kind_name_colliding_with_a_built_in_fires_an_admissibility_diagnostic() {
    // A lock-declared kind sharing a built-in's bare name (`rule`) but an incompatible
    // shape (`unit_shape: directory`, where the built-in `rule` is `file`) is neither
    // an admissible relocation of the built-in's `governs` locus (the one legitimate
    // reason a row reuses a built-in's name) nor a distinct custom kind of its own —
    // the bare-name namespace has one home per name
    // (KIND-NAME-COLLISION-ADMISSIBILITY). Before this fix the row was silently
    // dropped (a bare `continue` on the matching name) with no diagnostic and its
    // members lost from every corpus.
    let root = common::tmpdir("kind-collision");
    let policies = root.join("policies");
    fs::create_dir_all(&policies).unwrap();
    fs::write(
        policies.join("data-retention.md"),
        "# Data retention\n\nKeep it.\n",
    )
    .unwrap();

    common::write_lock(
        &root,
        Declarations {
            kinds: vec![KindFactRow {
                name: "rule".to_string(),
                provider: None,
                governs_root: "policies".to_string(),
                governs_glob: "*.md".to_string(),
                format: None,
                unit_shape: Some("directory".to_string()),
                registration: Vec::new(),
                templates: Vec::new(),
            }],
            ..Declarations::default()
        },
    );

    let run = common::check_in(&root, &[], Some("github"));
    assert!(
        !run.ok,
        "a kind-name collision must block ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("kind.admissibility"),
        "the finding names the admissibility rule, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains('`') && run.output.contains("rule"),
        "the finding names the colliding kind, got:\n{}",
        run.output
    );
}

#[test]
fn a_kind_row_relocating_a_built_ins_governs_fires_no_collision_diagnostic() {
    // The legitimate mechanism (`effective_governs`, proven by
    // `lock_declaration_rows.rs`'s
    // `check_walks_the_locks_declared_governs_locus_not_the_kinds_embedded_default`):
    // a row named exactly like a built-in, declaring only a relocated `governs` and no
    // diverging `format`/`unit_shape`/`registration`, is a relocation — not a
    // collision — so it must never trip KIND-NAME-COLLISION-ADMISSIBILITY.
    let root = common::tmpdir("kind-relocation");
    let rules = root.join("custom-locus").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(rules.join("dev-standards.md"), "# Dev standards\n\nBody.\n").unwrap();

    common::write_lock(
        &root,
        Declarations {
            kinds: vec![KindFactRow {
                name: "rule".to_string(),
                provider: None,
                governs_root: "custom-locus/rules".to_string(),
                governs_glob: "*.md".to_string(),
                format: None,
                unit_shape: None,
                registration: Vec::new(),
                templates: Vec::new(),
            }],
            ..Declarations::default()
        },
    );

    let run = common::check_in(&root, &[], Some("github"));
    assert!(
        !run.output.contains("kind.admissibility"),
        "a governs relocation must never fire the collision rule, got:\n{}",
        run.output
    );
}
