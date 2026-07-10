//! Requirement coverage — the referential shadow of the meaningful contract.
//!
//! Implements the `check` gate (requirements):
//! a **requirement** declares a semantic
//! intent (`prose`) the harness must fill, and an artifact fills it by *opting in*
//! from its own representation with a resolving `satisfies` link. `temper` **never
//! interprets `prose`** — that is the author's attestation, optionally backed by a
//! wired `verified_by`. What `check` gates is the **decidable shadow**: referential
//! coverage over the declared requirements and the authored `satisfies` edges.
//!
//! Two decidable, true-positive diagnostics — each ranges over skill *and* rule
//! features (every artifact kind that can opt in), the flattened stream the gate
//! assembles:
//!
//! - [`REQUIREMENT_UNFILLED_RULE`] — every `required` requirement is satisfied by **≥1
//!   artifact whose representation declares a resolving `satisfies` link naming it** —
//!   opt-in `satisfies` is the sole fill. A `required` requirement no artifact opts into
//!   is an `error`: the intent has no resolving home. A non-`required` requirement left
//!   unfilled is *not* a violation — `temper` never fabricates a gate the author did
//!   not declare.
//! - [`REQUIREMENT_DANGLING_RULE`] — every `satisfies` entry on any artifact names a
//!   **declared** requirement. A `satisfies` resolving to no requirement is an
//!   `error` on that artifact: a dangling link is a silent no-op, the very failure
//!   the decidable-only invariant forbids.
//!
//! This is the **referential** primitive — decidable coverage, a true positive
//! every time. `temper` NEVER judges
//! whether the artifact *actually* fulfils `prose`; the judged tier is delegated and
//! advisory, never this gate.
//!
//! # Kinship with the graph scope — and why coverage stays here
//!
//! The two checks are the graph-scope predicates ([`crate::graph`]) re-cast over a
//! *requirement* target set instead of an artifact one:
//!
//! - [`REQUIREMENT_DANGLING_RULE`] mirrors [`graph::check`](crate::graph::check)'s
//!   **route resolution** — a declared reference (`satisfies` here, a `routes_to`-style
//!   edge there) that resolves to no target is a dangling no-op. Both range over
//!   authored links and fire once per unresolved name.
//! - [`REQUIREMENT_UNFILLED_RULE`] mirrors [`graph::degree`](crate::graph::degree)'s
//!   **min-in-degree** bound — "≥1 artifact must point at this requirement" is exactly
//!   a `degree = { incoming = at_least(1) }` over the `satisfies` arcs.
//!
//! The difference that keeps this a separate module: the target set is the declared
//! **requirements**, *not* an artifact kind. Unifying into `graph.rs` would force a
//! synthetic `requirement` node kind into its `by_kind` corpus map — a fake artifact
//! kind that imports nothing and conforms to no contract, muddying the graph's honest
//! "artifact routes to artifact" model to reuse two small loops. The kinship is real
//! and worth naming; the merge is rejected — the duplication is cheaper than the lie.

use std::collections::{BTreeMap, BTreeSet};

use crate::check::Diagnostic;
use crate::compose::Requirement;
use crate::extract::Features;

/// A `required` requirement with no artifact opting in to satisfy it — the intent
/// has no resolving home.
const REQUIREMENT_UNFILLED_RULE: &str = "requirement.unfilled";

/// A `satisfies` link on an artifact that names no declared requirement — a
/// dangling reference.
const REQUIREMENT_DANGLING_RULE: &str = "requirement.dangling";

/// Gate referential coverage over the declared requirements and the authored
/// `satisfies` edges. Two
/// decidable checks over the flattened artifact stream:
///
/// 1. **Unfilled** — each `required` requirement must be named by ≥1 artifact's
///    resolving `satisfies` link, else the intent has no home ([`REQUIREMENT_UNFILLED_RULE`]).
/// 2. **Dangling** — each `satisfies` entry on any artifact must name a declared
///    requirement, else the link resolves to nothing ([`REQUIREMENT_DANGLING_RULE`]).
///
/// Both findings are `error` severity: a `required` requirement unfilled, or a
/// dangling `satisfies`, blocks the gate. `artifacts` is every opt-in-capable
/// artifact's features across every modeled kind, already flattened kind-blind by
/// the caller — the same unified satisfier set [`crate::roster`]/[`crate::graph`]
/// range over, so a requirement filled by
/// *any* kind is covered regardless of its own `kind` facet. `prose` is never
/// judged — coverage is the whole of the gate.
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

    // (1) Unfilled: every `required` requirement needs a resolving `satisfies` home —
    // opt-in `satisfies` is the sole fill. Iteration is over the name-sorted `BTreeMap`,
    // so the diagnostic set is stable.
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
    // kind), each link checked against the declared requirement names. Each artifact's
    // links are deduped first (a `BTreeSet` — the unfilled side's `satisfied` set does
    // the same collapse implicitly), so a link duplicated within one artifact's
    // `satisfies` emits ONE diagnostic, not one per repeat: the fault is the
    // unresolvable name, and it is named once. Dedup is per-artifact, so the *same*
    // dangling name on two different artifacts still fires once each — that is two
    // faults, one per opting-in artifact.
    for features in artifacts {
        let links: BTreeSet<&str> = features.satisfies.iter().map(String::as_str).collect();
        for target in links {
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
            sections: Vec::new(),
            source_dir: None,
            directives: Vec::new(),
            fenced_blocks: Vec::new(),
            nested_members: Vec::new(),
            satisfies: satisfies.iter().map(|s| s.to_string()).collect(),
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
            verified_by: None,
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
    fn a_non_required_unfilled_requirement_does_not_block() {
        let requirements = BTreeMap::from([(
            "nice-to-have".to_string(),
            requirement("nice-to-have", Some("an optional convenience"), false),
        )]);
        // Nothing opts into it, but it is advisory intent — no gate fires.
        let artifacts = vec![artifact("some-skill", &[])];

        assert!(check(&requirements, &artifacts).is_empty());
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
