//! Requirement coverage — the referential shadow of the meaningful contract.
//!
//! Implements the `check` gate for `specs/10-contracts.md` ("Requirements and
//! `satisfies` — the meaningful contract"): a **requirement** declares a semantic
//! intent (`means`) the harness must fill, and an artifact fills it by *opting in*
//! from its own representation with a resolving `satisfies` link. `temper` **never
//! interprets `means`** — that is the author's attestation, optionally backed by a
//! wired `verified_by`. What `check` gates is the **decidable shadow**: referential
//! coverage over the declared requirements and the authored `satisfies` edges.
//!
//! Two decidable, true-positive diagnostics — each ranges over skill *and* rule
//! features (every artifact kind that can opt in), the flattened stream the gate
//! assembles:
//!
//! - [`REQUIREMENT_UNFILLED_RULE`] — every `required` requirement is satisfied by
//!   **≥1 artifact whose representation declares a resolving `satisfies` link naming
//!   it**. A `required` requirement no artifact opts into is an `error`: the intent
//!   has no resolving home. A non-`required` requirement left unfilled is *not* a
//!   violation — `temper` never fabricates a gate the author did not declare
//!   (`00-intent.md` law 4).
//! - [`REQUIREMENT_DANGLING_RULE`] — every `satisfies` entry on any artifact names a
//!   **declared** requirement. A `satisfies` resolving to no requirement is an
//!   `error` on that artifact: a dangling link is a silent no-op, the very failure
//!   `00-intent.md` law 1 forbids.
//!
//! This is the **referential** primitive (`specs/10-contracts.md`, the primitive
//! algebra) — decidable coverage, a true positive every time. `temper` NEVER judges
//! whether the artifact *actually* fulfils `means`; the judged tier is delegated and
//! advisory (`00-intent.md` tier 2), never this gate.

use std::collections::{BTreeMap, BTreeSet};

use crate::check::Diagnostic;
use crate::compose::Requirement;
use crate::extract::Features;

/// A `required` requirement with no artifact opting in to satisfy it — the intent
/// has no resolving home (`specs/10-contracts.md`, "Requirements and `satisfies`").
const REQUIREMENT_UNFILLED_RULE: &str = "requirement.unfilled";

/// A `satisfies` link on an artifact that names no declared requirement — a
/// dangling reference (`specs/10-contracts.md`, the referential primitive).
const REQUIREMENT_DANGLING_RULE: &str = "requirement.dangling";

/// Gate referential coverage over the declared requirements and the authored
/// `satisfies` edges (`specs/10-contracts.md`, "Requirements and `satisfies` — the
/// meaningful contract"). Two decidable checks over the flattened artifact stream:
///
/// 1. **Unfilled** — each `required` requirement must be named by ≥1 artifact's
///    resolving `satisfies` link, else the intent has no home ([`REQUIREMENT_UNFILLED_RULE`]).
/// 2. **Dangling** — each `satisfies` entry on any artifact must name a declared
///    requirement, else the link resolves to nothing ([`REQUIREMENT_DANGLING_RULE`]).
///
/// Both findings are `error` severity: a `required` requirement unfilled, or a
/// dangling `satisfies`, blocks the gate. `artifacts` is every opt-in-capable
/// artifact's features (skill ⊕ rule), so a requirement filled by *either* kind is
/// covered. `means` is never judged — coverage is the whole of the gate.
#[must_use]
pub fn check(
    requirements: &BTreeMap<String, Requirement>,
    artifacts: &[Features],
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // The set of requirement names any artifact opts into — one pass over the
    // stream, so both checks read a decided fact rather than re-scanning.
    let satisfied: BTreeSet<&str> = artifacts
        .iter()
        .flat_map(|features| features.satisfies.iter().map(String::as_str))
        .collect();

    // (1) Unfilled: every `required` requirement needs a resolving `satisfies` home.
    // Iteration is over the name-sorted `BTreeMap`, so the diagnostic set is stable.
    for (name, requirement) in requirements {
        if requirement.required && !satisfied.contains(name.as_str()) {
            diagnostics.push(Diagnostic::error(
                REQUIREMENT_UNFILLED_RULE,
                name,
                format!(
                    "required requirement `{name}` is unfilled: no artifact declares a `satisfies` link naming it"
                ),
            ));
        }
    }

    // (2) Dangling: every authored `satisfies` link must resolve to a declared
    // requirement. Ranges over the artifacts in load order (already name-sorted per
    // kind), each link checked against the declared requirement names.
    for features in artifacts {
        for target in &features.satisfies {
            if !requirements.contains_key(target) {
                diagnostics.push(Diagnostic::error(
                    REQUIREMENT_DANGLING_RULE,
                    &features.id,
                    format!(
                        "artifact `{}` declares `satisfies = [\"{target}\"]`, but no requirement `{target}` is declared",
                        features.id
                    ),
                ));
            }
        }
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::check::Severity;
    use crate::extract::Features;

    /// A minimal [`Features`] carrying just an id and its `satisfies` edges — the
    /// only fields the coverage check reads.
    fn artifact(id: &str, satisfies: &[&str]) -> Features {
        Features {
            id: id.to_string(),
            fields: BTreeMap::new(),
            body_lines: 0,
            headings: Vec::new(),
            source_dir: None,
            companions: Vec::new(),
            satisfies: satisfies.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// A named requirement with the given `required` flag; `means` is carried but
    /// never read by the coverage check.
    fn requirement(means: &str, required: bool) -> Requirement {
        Requirement {
            means: means.to_string(),
            required,
        }
    }

    #[test]
    fn a_required_requirement_with_a_resolving_satisfies_stays_silent() {
        let requirements = BTreeMap::from([(
            "dev-standards".to_string(),
            requirement("the harness maintains dev standards", true),
        )]);
        let artifacts = vec![artifact("dev-standards-skill", &["dev-standards"])];

        assert!(check(&requirements, &artifacts).is_empty());
    }

    #[test]
    fn a_required_requirement_with_no_satisfying_artifact_fires_unfilled() {
        let requirements = BTreeMap::from([(
            "dev-standards".to_string(),
            requirement("the harness maintains dev standards", true),
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
            requirement("the harness maintains dev standards", true),
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
    fn a_non_required_unfilled_requirement_does_not_block() {
        let requirements = BTreeMap::from([(
            "nice-to-have".to_string(),
            requirement("an optional convenience", false),
        )]);
        // Nothing opts into it, but it is advisory intent — no gate fires.
        let artifacts = vec![artifact("some-skill", &[])];

        assert!(check(&requirements, &artifacts).is_empty());
    }

    #[test]
    fn coverage_ranges_over_skill_and_rule_features_alike() {
        // Two required requirements: one filled by a skill-shaped artifact, the
        // other by a rule-shaped one. The check reads only `id`/`satisfies`, so a
        // requirement filled by *either* kind is covered.
        let requirements = BTreeMap::from([
            (
                "dev-standards".to_string(),
                requirement("skill fills this", true),
            ),
            (
                "rust-style".to_string(),
                requirement("rule fills this", true),
            ),
        ]);
        let artifacts = vec![
            artifact("dev-standards-skill", &["dev-standards"]),
            artifact("rust-rule", &["rust-style"]),
        ];

        assert!(check(&requirements, &artifacts).is_empty());
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
