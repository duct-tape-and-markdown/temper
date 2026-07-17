//! The shipped `dial` kind: see the annoying finding, read its label, dial it.
//!
//! The whole point is the round trip — a finding's `rule` id is spelled back into
//! `.temper/dial.toml` verbatim and the next run re-reads that clause at the declared
//! severity. What the cases hold past the round trip is the envelope, in the two halves
//! it is held by: the *schema* cannot spell deletion (severity is the only verb, and its
//! vocabulary is the same closed two an authored clause declares under), and the *engine*
//! makes softening inert in block mode, so a block-mode pass on any machine implies the
//! shared gate's pass.
//!
//! The dial is temper's own kind, so no case here writes a kind row: a harness gets its
//! dial from adopting temper at all. `tests/local_locus.rs` owns the locus class the dial
//! rides; these cases own what the dial does with it.

use std::fs;
use std::path::Path;

use temper::drift::{self, AssemblyFactRow, Declarations};

mod common;

/// A skill whose `description` is absent — one `skill.required.description` finding at
/// the shipped `required` severity, and the softening cases' target.
const UNDESCRIBED_SKILL: &str = "---\n\
name: coordinate\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// A skill the shipped contract has nothing to say about — so a case's own target clause
/// is the run's only finding, and the run's verdict is that clause's alone.
const CLEAN_SKILL: &str = "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// A well-formed skill whose body runs past the shipped 500-line advisory — one
/// `skill.extent` finding at `advisory`, and the hardening cases' target.
fn overlong_skill() -> String {
    format!(
        "---\n\
         name: coordinate\n\
         description: Use when coordinating agents across axes; not for single-axis work.\n\
         ---\n\
         # Coordinate\n\
         \n{}",
        "Drive the team through the playbook.\n".repeat(501)
    )
}

/// Write `body` at the dial's own governed locus — the one place a dial is ever read.
fn write_dial(harness: &Path, body: &str) {
    common::write_sibling(harness, ".temper/dial.toml", body);
}

/// A dial naming one clause at one severity — the shape every case below varies.
fn dial_entry(label: &str, severity: &str) -> String {
    format!(
        "name = \"workstation\"\n\n[[clause]]\nlabel = \"{label}\"\nseverity = \"{severity}\"\n"
    )
}

/// A fresh harness carrying `skill` at `.claude/skills/coordinate`, and a lock declaring
/// the harness's enforcement `mode`.
fn harness_at_mode(label: &str, mode: &str, skill: &str) -> std::path::PathBuf {
    let harness = common::tmpdir(label);
    fs::create_dir_all(harness.join(".temper")).unwrap();
    common::write_skill(&harness, "coordinate", skill);
    common::write_lock(
        &harness,
        Declarations {
            assembly: vec![AssemblyFactRow {
                fact: "mode".to_string(),
                value: Some(mode.to_string()),
                from: None,
                field: None,
                to: None,
            }],
            ..Default::default()
        },
    );
    harness
}

/// The severity the github reporter printed for `rule` — `error`, `warning`, or `None`
/// when the rule fired at all. The reporter's line is the author's own read of a
/// finding's weight, so a case that scraped the severity anywhere else would be checking
/// something the author never sees.
fn reported_severity(findings: &[String], rule: &str) -> Option<String> {
    let line = common::findings_for(findings, rule).first().copied()?;
    Some(line.trim_start_matches("::").split(' ').next()?.to_string())
}

#[test]
fn a_dialed_clause_is_re_read_at_the_declared_severity_and_still_reports() {
    // The round trip the address exists for: the finding printed
    // `skill.required.description`, and that string — nothing derived from it, nothing
    // looked up — is what the entry names.
    let harness = harness_at_mode("dial-softens", "warn", UNDESCRIBED_SKILL);

    let (before, ok) = common::check_harness(&harness);
    assert_eq!(
        reported_severity(&before, "skill.required.description").as_deref(),
        Some("error"),
        "the shipped severity is the baseline this case dials off: {before:?}"
    );
    assert!(
        !ok,
        "an undescribed skill fails the shared gate: {before:?}"
    );

    write_dial(
        &harness,
        &dial_entry("skill.required.description", "advisory"),
    );
    let (after, ok) = common::check_harness(&harness);

    // Still reported — the softening is visible, and that is the whole of what a dial
    // may do downward. A machine that wanted the finding gone has no spelling for it.
    assert_eq!(
        reported_severity(&after, "skill.required.description").as_deref(),
        Some("warning"),
        "the dialed clause reports at the machine's declared severity: {after:?}"
    );
    assert!(
        ok,
        "an advisory finding no longer blocks this machine: {after:?}"
    );
}

#[test]
fn a_dial_hardens_in_every_mode() {
    // Hardening is the unbounded direction: it needs no review, because a machine
    // holding itself stricter than the shared gate can never let something through.
    for mode in ["note", "warn", "block"] {
        let harness = harness_at_mode(&format!("dial-hardens-{mode}"), mode, &overlong_skill());

        let (before, ok) = common::check_harness(&harness);
        assert_eq!(
            reported_severity(&before, "skill.extent").as_deref(),
            Some("warning"),
            "the shipped `extent` is advisory: {before:?}"
        );
        assert!(
            ok,
            "an advisory finding alone does not fail a run: {before:?}"
        );

        write_dial(&harness, &dial_entry("skill.extent", "required"));
        let (after, ok) = common::check_harness(&harness);

        assert_eq!(
            reported_severity(&after, "skill.extent").as_deref(),
            Some("error"),
            "the dialed clause blocks this machine under `{mode}`: {after:?}"
        );
        assert!(!ok, "hardening binds under `{mode}`: {after:?}");
    }
}

#[test]
fn a_dialed_softening_is_inert_in_block_mode() {
    // The bound that makes a block-mode pass portable: this machine's dial says
    // `advisory`, and the gate reads `required` anyway, so the machine's verdict and the
    // shared gate's cannot disagree.
    let harness = harness_at_mode("dial-block-inert", "block", UNDESCRIBED_SKILL);
    write_dial(
        &harness,
        &dial_entry("skill.required.description", "advisory"),
    );

    let (findings, ok) = common::check_harness(&harness);

    assert_eq!(
        reported_severity(&findings, "skill.required.description").as_deref(),
        Some("error"),
        "the authored severity stands: a block-mode pass on any machine implies the \
         shared gate's pass: {findings:?}"
    );
    assert!(
        !ok,
        "the softening bought this machine nothing: {findings:?}"
    );
}

#[test]
fn a_dial_cannot_spell_deletion() {
    // The envelope's structural half. Every spelling a machine might reach for to make a
    // finding *go away* is a key or a value the dial's own contract refuses — and the
    // refusal is a finding of its own, so the attempt is louder than the clause it
    // targeted, never quieter.
    let harness = harness_at_mode("dial-no-deletion", "warn", UNDESCRIBED_SKILL);

    // A third severity: the vocabulary is the closed two, and `off` is not one of them.
    write_dial(&harness, &dial_entry("skill.required.description", "off"));
    let (findings, ok) = common::check_harness(&harness);
    assert_eq!(
        common::findings_for(&findings, "dial.enum.clause[*].severity").len(),
        1,
        "a severity outside the closed two is a finding: {findings:?}"
    );
    assert_eq!(
        reported_severity(&findings, "skill.required.description").as_deref(),
        Some("error"),
        "and the clause it named is untouched: {findings:?}"
    );
    assert!(!ok, "the malformed dial fails this machine: {findings:?}");

    // A verb beside the entry: `closed-keys` is what makes an unknown key a finding
    // rather than a key the read politely ignored.
    write_dial(
        &harness,
        "name = \"workstation\"\nskip = [\"skill.required.description\"]\n",
    );
    let (findings, ok) = common::check_harness(&harness);
    assert_eq!(
        common::findings_for(&findings, "dial.closed-keys").len(),
        1,
        "a key outside `name`/`clause` is a verb this schema does not have: {findings:?}"
    );
    assert_eq!(
        reported_severity(&findings, "skill.required.description").as_deref(),
        Some("error"),
        "and the clause it named is untouched: {findings:?}"
    );
    assert!(!ok, "the malformed dial fails this machine: {findings:?}");
}

#[test]
fn a_dial_cannot_dial_its_own_contract() {
    // The envelope must be out of reach of the file it bounds: a machine that could
    // soften `dial.closed-keys` would have talked its way out of the shape above.
    let harness = harness_at_mode("dial-self", "warn", UNDESCRIBED_SKILL);
    write_dial(&harness, &dial_entry("dial.closed-keys", "advisory"));

    let (findings, ok) = common::check_harness(&harness);

    let refusal = common::findings_for(&findings, "dial.entry");
    assert_eq!(
        refusal.len(),
        1,
        "the entry is refused, never inert: {findings:?}"
    );
    assert!(
        refusal[0].contains("dial.closed-keys") && refusal[0].contains("envelope"),
        "the refusal names the entry and why it cannot stand: {findings:?}"
    );
    assert!(
        !ok,
        "an entry that dials nothing fails this machine: {findings:?}"
    );
}

#[test]
fn an_entry_naming_no_clause_at_all_is_a_finding() {
    // A stale entry that quietly named nothing would defeat the address's whole reason
    // for being legible. Erring here is strictly *stricter* than the shared gate: this
    // machine fails its own check having softened nothing.
    let harness = harness_at_mode("dial-stale", "warn", UNDESCRIBED_SKILL);
    write_dial(&harness, &dial_entry("skill.max_len.retired", "advisory"));

    let (findings, ok) = common::check_harness(&harness);

    let refusal = common::findings_for(&findings, "dial.entry");
    assert_eq!(refusal.len(), 1, "the stale entry is named: {findings:?}");
    assert!(
        refusal[0].contains("skill.max_len.retired"),
        "the finding names the label that reached nothing: {findings:?}"
    );
    assert!(!ok, "a dial that dials nothing fails loud: {findings:?}");
}

#[test]
fn a_requirements_own_clause_dials_by_the_address_it_reports_under() {
    // A requirement's clauses reach a judge past every kind contract — as does the
    // each-grain narrowing clause its `kind` facet synthesizes, the one here. A dial that
    // covered the contracts alone would leave an author reading an address off a finding
    // they could not then spell back, which is the one thing the round trip may never do.
    let harness = harness_at_mode("dial-requirement", "warn", CLEAN_SKILL);
    common::write_lock(
        &harness,
        Declarations {
            requirements: vec![common::requirement("review-coverage", true, Some("rule"))],
            satisfies: vec![drift::SatisfiesRow {
                member: "skill:coordinate".to_string(),
                requirement: "review-coverage".to_string(),
            }],
            ..drift::read_declarations(&harness.join(".temper")).unwrap()
        },
    );

    let (before, ok) = common::check_harness(&harness);
    let rule = "requirement.review-coverage.kind";
    assert_eq!(
        reported_severity(&before, rule).as_deref(),
        Some("error"),
        "a skill filling a rule-narrowed requirement is a finding: {before:?}"
    );
    assert!(!ok, "at the shipped severity it blocks: {before:?}");

    write_dial(&harness, &dial_entry(rule, "advisory"));
    let (after, ok) = common::check_harness(&harness);

    assert_eq!(
        reported_severity(&after, rule).as_deref(),
        Some("warning"),
        "the requirement's clause dials by the address it reported under: {after:?}"
    );
    assert!(ok, "and no longer blocks this machine: {after:?}");
}

#[test]
fn a_dial_document_is_read_in_place_and_no_row_of_it_reaches_the_lock() {
    // The committed half is the kind; the uncommitted half is the document. The lock
    // captures the first and not the second, which is what makes committed bytes
    // layer-invariant by construction rather than by a rule something must remember.
    let harness = harness_at_mode("dial-not-locked", "warn", UNDESCRIBED_SKILL);
    let before = fs::read_to_string(harness.join(".temper/lock.toml")).unwrap();
    write_dial(
        &harness,
        &dial_entry("skill.required.description", "advisory"),
    );

    let (findings, _) = common::check_harness(&harness);

    // Read in place, under the kind the committed lock declares: the entry applied, so
    // the document reached the gate without any row of it existing anywhere.
    assert_eq!(
        reported_severity(&findings, "skill.required.description").as_deref(),
        Some("warning"),
        "the dial was read at its own locus: {findings:?}"
    );
    assert_eq!(
        fs::read_to_string(harness.join(".temper/lock.toml")).unwrap(),
        before,
        "check writes no lock, and the dial gave it nothing to write"
    );
    assert!(
        !before.contains("workstation"),
        "no row of a local member's ever enters the lock: {before}"
    );
}
