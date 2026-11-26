//! Requirement coverage — the referential shadow of the meaningful contract.
//!
//! Implements the `check` gate (requirements):
//! a **requirement** declares a semantic
//! intent (`prose`) the harness must fill, and an artifact fills it by *opting in*
//! from its own representation with a resolving `satisfies` link. `temper` **never
//! interprets `prose`** — that is the author's attestation, optionally backed by a
//! wired `verifier`. What `check` gates is the **decidable shadow**: referential
//! coverage over the declared requirements and the authored `satisfies` edges.
//!
//! Both checks are clauses of the `requirement` kind's contract, judged over each
//! requirement's opt-in selection: a `count` bound is the fill (shipped at least one
//! satisfier), and `resolves` holds every `satisfies` link to a declared requirement. A
//! finding carries its clause's label and severity, so the dial reaches both; a contract
//! binding neither clause leaves its check silent.
//!
//! The target set is the declared **requirements**, not an artifact kind, which is why
//! this stays apart from [`crate::graph`]: its degree and route walks would need a
//! synthetic `requirement` node kind that imports nothing and conforms to nothing.

use std::collections::{BTreeMap, BTreeSet};

use crate::check::{Diagnostic, Severity};
use crate::compose::Requirement;
use crate::contract::{Clause, Predicate};
use crate::extract::Features;

/// Gate referential coverage over the declared requirements and the authored
/// `satisfies` edges, under the `requirement` kind's `clauses`:
///
/// 1. **Fill** — the contract's `count` clause bounds how many artifacts opt in to each
///    requirement. A requirement declared `required: false` reads it at advisory: an
///    optional requirement left unfilled still reports, it never blocks.
/// 2. **Dangling** — the contract's `resolves` clause holds each `satisfies` entry on
///    any artifact to a declared requirement.
///
/// `artifacts` is every opt-in-capable artifact's features across every modeled kind,
/// flattened kind-blind by the caller, so a requirement filled by *any* kind is covered.
/// `prose` is never judged.
#[must_use]
pub fn check(
    requirements: &BTreeMap<String, Requirement>,
    artifacts: &[Features],
    clauses: &[Clause],
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    let fill = clauses.iter().find_map(|clause| match clause.predicate {
        Predicate::Count { min, max } => Some((clause, min, max)),
        _ => None,
    });
    if let Some((clause, min, max)) = fill {
        // Iteration is over the name-sorted `BTreeMap`, so the diagnostic set is stable.
        for (name, requirement) in requirements {
            let filled = artifacts
                .iter()
                .filter(|features| features.satisfies.iter().any(|target| target == name))
                .count();
            if (min..=max).contains(&filled) {
                continue;
            }
            let message = if filled == 0 {
                format!(
                    "requirement `{name}` is unfilled: no artifact declares a `satisfies` link naming it"
                )
            } else {
                format!(
                    "requirement `{name}` is filled by {filled} artifact(s), outside its fill bound of {min}..={max}"
                )
            };
            let mut finding = Diagnostic::from_clause(clause, name, message);
            if !requirement.required {
                finding.severity = Severity::Warn;
            }
            diagnostics.push(finding);
        }
    }

    // Each artifact's links are deduped first, so a link repeated within one artifact is
    // one fault named once; the same dangling name on two artifacts is two faults.
    if let Some(clause) = clauses
        .iter()
        .find(|clause| clause.predicate == Predicate::Resolves)
    {
        for features in artifacts {
            let links: BTreeSet<&str> = features.satisfies.iter().map(String::as_str).collect();
            for target in links {
                if !requirements.contains_key(target) {
                    diagnostics.push(Diagnostic::from_clause(
                        clause,
                        &features.id,
                        format!(
                            "artifact `{}` declares `satisfies = [\"{target}\"]`, but no requirement `{target}` is declared",
                            features.id
                        ),
                    ));
                }
            }
        }
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract::Features;
    use crate::test_support;

    /// The fill and dangling clauses' labels in the shipped `requirement` contract.
    const REQUIREMENT_UNFILLED_RULE: &str = "requirement.count";
    const REQUIREMENT_DANGLING_RULE: &str = "requirement.resolves";

    /// The coverage check under the shipped `requirement` contract.
    fn check(
        requirements: &BTreeMap<String, Requirement>,
        artifacts: &[Features],
    ) -> Vec<Diagnostic> {
        let shipped = crate::builtin::contract(crate::compose::REQUIREMENT_KIND)
            .expect("the requirement kind ships a default contract");
        super::check(requirements, artifacts, &shipped.clauses)
    }

    /// A minimal [`Features`] carrying just an id and its `satisfies` edges — the
    /// only fields the coverage check reads.
    fn artifact(id: &str, satisfies: &[&str]) -> Features {
        Features {
            satisfies: satisfies.iter().map(|s| s.to_string()).collect(),
            ..test_support::features(id)
        }
    }

    /// A named requirement with the given `required` flag; `prose` is carried but
    /// never read by the coverage check, and the fill/typing facets are absent so the
    /// requirement is the pure opt-in-coverage form the gate ranges over.
    fn requirement(name: &str, prose: Option<&str>, required: bool) -> Requirement {
        Requirement {
            name: name.to_string(),
            prose: prose.map(str::to_string),
            kind: None,
            required,
            clauses: Vec::new(),
            verifier: None,
        }
    }

    #[test]
    fn a_required_requirement_with_a_resolving_satisfies_stays_silent() {
        let requirements = BTreeMap::from([(
            "dev-standards".to_string(),
            requirement(
                "dev-standards",
                Some("the harness maintains dev standards"),
                true,
            ),
        )]);
        let artifacts = vec![artifact("dev-standards-skill", &["dev-standards"])];

        assert!(check(&requirements, &artifacts).is_empty());
    }

    #[test]
    fn a_required_requirement_with_no_satisfying_artifact_fires_unfilled() {
        let requirements = BTreeMap::from([(
            "dev-standards".to_string(),
            requirement(
                "dev-standards",
                Some("the harness maintains dev standards"),
                true,
            ),
        )]);
        // An artifact that opts into a *different* requirement does not cover it.
        let artifacts = vec![artifact("other-skill", &["something-else"])];

        let diagnostics = check(&requirements, &artifacts);
        let unfilled: Vec<&Diagnostic> = diagnostics
            .iter()
            .filter(|d| d.rule == REQUIREMENT_UNFILLED_RULE)
            .collect();
        assert_eq!(unfilled.len(), 1);
        assert_eq!(unfilled[0].severity, Severity::Error);
        assert_eq!(unfilled[0].artifact, "dev-standards");
    }

    #[test]
    fn a_satisfies_naming_no_requirement_fires_dangling_on_that_artifact() {
        let requirements = BTreeMap::from([(
            "dev-standards".to_string(),
            requirement(
                "dev-standards",
                Some("the harness maintains dev standards"),
                true,
            ),
        )]);
        let artifacts = vec![artifact(
            "dev-standards-skill",
            &["dev-standards", "typo-req"],
        )];

        let diagnostics = check(&requirements, &artifacts);
        // The required requirement is covered — no unfilled — but the second,
        // unresolvable link dangles on the artifact that declares it.
        assert!(
            !diagnostics
                .iter()
                .any(|d| d.rule == REQUIREMENT_UNFILLED_RULE)
        );
        let dangling: Vec<&Diagnostic> = diagnostics
            .iter()
            .filter(|d| d.rule == REQUIREMENT_DANGLING_RULE)
            .collect();
        assert_eq!(dangling.len(), 1);
        assert_eq!(dangling[0].severity, Severity::Error);
        assert_eq!(dangling[0].artifact, "dev-standards-skill");
        assert!(dangling[0].message.contains("typo-req"));
    }

    #[test]
    fn a_non_required_unfilled_requirement_reports_at_advisory() {
        let requirements = BTreeMap::from([(
            "nice-to-have".to_string(),
            requirement("nice-to-have", Some("an optional convenience"), false),
        )]);
        let artifacts = vec![artifact("some-skill", &[])];

        let diagnostics = check(&requirements, &artifacts);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule, REQUIREMENT_UNFILLED_RULE);
        assert_eq!(diagnostics[0].severity, Severity::Warn);
    }

    #[test]
    fn a_contract_binding_neither_clause_judges_nothing() {
        let requirements = BTreeMap::from([(
            "dev-standards".to_string(),
            requirement("dev-standards", None, true),
        )]);
        let artifacts = vec![artifact("stray-skill", &["ghost"])];

        assert!(super::check(&requirements, &artifacts, &[]).is_empty());
    }

    #[test]
    fn a_prose_less_requirement_still_gates_coverage_via_required() {
        // `prose` is optional on the unified requirement, but coverage keys off
        // `required`, not `prose`: a `required` requirement with no `prose` and no
        // satisfying artifact still fires UNFILLED.
        let requirements = BTreeMap::from([(
            "dev-standards".to_string(),
            requirement("dev-standards", None, true),
        )]);
        let artifacts = vec![artifact("some-skill", &[])];

        let diagnostics = check(&requirements, &artifacts);
        let unfilled: Vec<&Diagnostic> = diagnostics
            .iter()
            .filter(|d| d.rule == REQUIREMENT_UNFILLED_RULE)
            .collect();
        assert_eq!(unfilled.len(), 1);
        assert_eq!(unfilled[0].artifact, "dev-standards");
    }

    #[test]
    fn coverage_ranges_over_skill_and_rule_features_alike() {
        // Two required requirements: one filled by a skill-shaped artifact, the
        // other by a rule-shaped one. The check reads only `id`/`satisfies`, so a
        // requirement filled by *either* kind is covered.
        let requirements = BTreeMap::from([
            (
                "dev-standards".to_string(),
                requirement("dev-standards", Some("skill fills this"), true),
            ),
            (
                "rust-style".to_string(),
                requirement("rust-style", Some("rule fills this"), true),
            ),
        ]);
        let artifacts = vec![
            artifact("dev-standards-skill", &["dev-standards"]),
            artifact("rust-rule", &["rust-style"]),
        ];

        assert!(check(&requirements, &artifacts).is_empty());
    }

    #[test]
    fn a_satisfies_entry_duplicated_within_one_artifact_dangles_once() {
        // One artifact repeats the same unresolvable link. The fault is the single
        // unresolvable name, so it is named ONCE — the dangling loop dedups each
        // artifact's `satisfies` before checking, mirroring the implicit collapse the
        // unfilled `satisfied` set already performs.
        let requirements = BTreeMap::new();
        let artifacts = vec![artifact("dup-skill", &["ghost", "ghost", "ghost"])];

        let diagnostics = check(&requirements, &artifacts);
        let dangling: Vec<&Diagnostic> = diagnostics
            .iter()
            .filter(|d| d.rule == REQUIREMENT_DANGLING_RULE)
            .collect();
        assert_eq!(dangling.len(), 1);
        assert_eq!(dangling[0].artifact, "dup-skill");
    }

    #[test]
    fn the_same_dangling_name_on_two_artifacts_dangles_once_each() {
        // Dedup is per-artifact: the same unresolvable name on two distinct opting-in
        // artifacts is two faults — one per artifact — not one collapsed finding.
        let requirements = BTreeMap::new();
        let artifacts = vec![
            artifact("skill-a", &["ghost", "ghost"]),
            artifact("skill-b", &["ghost"]),
        ];

        let diagnostics = check(&requirements, &artifacts);
        let dangling: Vec<&Diagnostic> = diagnostics
            .iter()
            .filter(|d| d.rule == REQUIREMENT_DANGLING_RULE)
            .collect();
        assert_eq!(dangling.len(), 2);
        assert_eq!(dangling[0].artifact, "skill-a");
        assert_eq!(dangling[1].artifact, "skill-b");
    }

    #[test]
    fn a_typo_link_yields_paired_unfilled_and_dangling() {
        // A misspelled link is exact-string-matched, never folded: the real
        // requirement goes UNFILLED (nothing resolves to it) *and* the typo'd name
        // DANGLES (it names no requirement). Two true positives, not one masking the
        // other — this pins exact-match precision.
        let requirements = BTreeMap::from([(
            "dev-standards".to_string(),
            requirement(
                "dev-standards",
                Some("the harness maintains dev standards"),
                true,
            ),
        )]);
        let artifacts = vec![artifact("dev-standards-skill", &["dev-standatds"])];

        let diagnostics = check(&requirements, &artifacts);
        let unfilled: Vec<&Diagnostic> = diagnostics
            .iter()
            .filter(|d| d.rule == REQUIREMENT_UNFILLED_RULE)
            .collect();
        let dangling: Vec<&Diagnostic> = diagnostics
            .iter()
            .filter(|d| d.rule == REQUIREMENT_DANGLING_RULE)
            .collect();
        assert_eq!(unfilled.len(), 1);
        assert_eq!(unfilled[0].artifact, "dev-standards");
        assert_eq!(dangling.len(), 1);
        assert_eq!(dangling[0].artifact, "dev-standards-skill");
        assert!(dangling[0].message.contains("dev-standatds"));
    }

    #[test]
    fn no_requirements_is_silent_even_with_authored_satisfies() {
        // No declared requirements: an unfilled check has nothing to gate. But an
        // authored `satisfies` still dangles — it names a requirement that does not
        // exist.
        let requirements = BTreeMap::new();
        let artifacts = vec![artifact("stray-skill", &["ghost"])];

        let diagnostics = check(&requirements, &artifacts);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule, REQUIREMENT_DANGLING_RULE);
    }
}
