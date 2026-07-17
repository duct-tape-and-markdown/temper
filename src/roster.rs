//! The roster — a parsed harness contract's named requirements, and the **opt-in
//! selection** each one declares.
//!
//! [`selections`] resolves every requirement's opt-in selection — the members whose
//! `satisfies` edge targets it, kind-blind — and binds the requirement's own clauses to
//! it, so `crate::engine::judge` and `crate::graph::degree` judge it through the one
//! selection algebra: this is the existential instance of that algebra, never a second
//! machinery beside it. [`admissibility`] checks each requirement's own definition
//! before the roster is trusted to judge a harness.

use std::collections::BTreeMap;
use std::path::Path;

use crate::builtin;
use crate::check::Diagnostic;
use crate::compose::Requirement;
use crate::engine::{self, Selection, Selector};
use crate::extract::Features;

/// The diagnostic `rule` id every roster-admissibility finding reports under.
const REQUIREMENT_ADMISSIBILITY_RULE: &str = "requirement.admissibility";

/// Whether an artifact opts into the requirement named `requirement` — its
/// `satisfies` list carries that name. The decidable join at the heart of the opt-in
/// selection.
fn is_satisfier(requirement: &str, features: &Features) -> bool {
    features.satisfies.iter().any(|name| name == requirement)
}

/// Every candidate artifact any requirement may range over, before the opt-in
/// filter: every modeled kind's workspace [`Features`], each tagged with its own
/// kind label — kind-blind.
fn candidates<'a>(by_kind: &BTreeMap<&'a str, &'a [Features]>) -> Vec<(&'a str, &'a Features)> {
    by_kind
        .iter()
        .flat_map(|(kind, features)| features.iter().map(move |feature| (*kind, feature)))
        .collect()
}

/// Every requirement's **opt-in selection**: the members that opt in via `satisfies`,
/// each tagged with its own kind label, bound to the requirement's own clauses plus the
/// each-grain `kind` narrowing its `kind` facet sources
/// ([`builtin::kind_narrowing_clause`]).
///
/// The opt-in join is the *only* filter: a requirement's `kind` facet never narrows the
/// members (that would be a second selector, and selectors do not compose) — it sources
/// a clause judged over exactly this kind-blind set, so a wrong-kind opt-in is a finding
/// rather than a member excluded before it can be seen.
///
/// Requirements iterate in name order (the roster is a [`BTreeMap`]) and each kind's
/// candidates arrive name-sorted, so the selections — and every finding judged over
/// them — are stable across runs.
#[must_use]
pub fn selections<'a>(
    requirements: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&'a str, &'a [Features]>,
) -> Vec<Selection<'a>> {
    let candidates = candidates(by_kind);
    requirements
        .values()
        .map(|requirement| {
            let mut clauses: Vec<_> = requirement
                .kind
                .iter()
                .map(|kind| builtin::kind_narrowing_clause(&requirement.name, kind))
                .collect();
            clauses.extend(requirement.clauses.iter().cloned());
            Selection {
                selector: Selector::OptIn(requirement.name.clone()),
                clauses,
                members: candidates
                    .iter()
                    .filter(|(_, features)| is_satisfier(&requirement.name, features))
                    .copied()
                    .collect(),
            }
        })
        .collect()
}

/// Validate the harness roster against **the definition** — admissibility.
/// Each requirement's own
/// definition must pass a check *before* the roster is used to judge anything;
/// every finding is [`Diagnostic::error`] (an inadmissible requirement cannot be
/// trusted) and names the requirement it indicts.
///
/// Three decidable clauses:
///
/// - **(a)** a typed requirement's `kind` names a kind `temper` models, else the
///   each-grain clause it sources ([`builtin::kind_narrowing_clause`]) can never hold
///   for any satisfier — an unfillable selector, regardless of `required` (fillability
///   itself is kind-blind now: any opt-in artifact, of any modeled kind, satisfies
///   coverage; naming an unmodeled kind only ever breaks the *narrowing* clause).
/// - **(b)** any `verified_by` path exists relative to `base_dir`.
/// - **(c)** every clause in [`clauses`](Requirement::clauses) is itself well-formed —
///   reusing [`crate::engine::inadmissibilities`], the same rules a kind's own floor
///   clauses are checked against (an inverted `count`/`degree` bound, an empty
///   `membership` target), so a requirement's demands and a kind's clauses never carry
///   two definitions of "vacuous".
///
/// `by_kind` supplies only the modeled kinds (its keys), never satisfiers. `base_dir`
/// is the harness root a `verified_by` path resolves against.
#[must_use]
pub fn admissibility(
    requirements: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&str, &[Features]>,
    base_dir: &Path,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for requirement in requirements.values() {
        let name = requirement.name.as_str();

        // (a) A `kind` narrowing to an unmodeled kind sources a clause that can never
        // hold — no satisfier is ever of a kind `temper` does not model.
        if let Some(kind) = &requirement.kind
            && !by_kind.contains_key(kind.as_str())
        {
            diagnostics.push(Diagnostic::error(
 REQUIREMENT_ADMISSIBILITY_RULE,
 name,
 format!(
                    "requirement `{name}` narrows its satisfiers to kind `{kind}`, which `temper` does not model — that each-grain clause can never be filled"
 ),
 ));
        }

        // (b) A `verified_by` path must exist — a dangling verifier is a silent no-op.
        if let Some(verifier) = &requirement.verified_by
            && !base_dir.join(verifier).exists()
        {
            diagnostics.push(Diagnostic::error(
 REQUIREMENT_ADMISSIBILITY_RULE,
 name,
 format!(
                    "requirement `{name}` names verifier `{verifier}`, which does not resolve to a path under the project — a dangling verifier is a silent no-op"
 ),
 ));
        }

        // (c) Every clause's own predicate must be well-formed.
        for clause in &requirement.clauses {
            // The document locus, decidably: an opt-in selection draws satisfiers, and
            // an embedded member carries no `satisfies` edge to be drawn by, so every
            // satisfier owns a document of its own.
            for message in engine::inadmissibilities(&clause.predicate, &engine::Locus::Document) {
                diagnostics.push(Diagnostic::error(
                    REQUIREMENT_ADMISSIBILITY_RULE,
                    name,
                    message,
                ));
            }
        }
    }
    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use crate::check::Severity;
    use crate::contract::{Clause, Predicate, Severity as ClauseSeverity};
    use crate::extract::{FeatureValue, ValueType};

    /// Judge a roster's opt-in selections — the composition `main` runs, minus the
    /// by-kind selections no requirement's `membership` can name.
    fn judge_roster(
        requirements: &BTreeMap<String, Requirement>,
        by_kind: &BTreeMap<&str, &[Features]>,
    ) -> Vec<Diagnostic> {
        engine::judge(&selections(requirements, by_kind))
    }

    /// A required-severity clause wrapping `predicate` — the shape every set-scope
    /// test case below attaches to a requirement's `clauses`.
    fn required_clause(predicate: Predicate) -> Clause {
        Clause {
            label: crate::contract::clause_label(
                Some(&crate::contract::requirement_owner("gate")),
                predicate.key(),
                None,
            ),
            severity: ClauseSeverity::Required,
            predicate,
            guidance: None,
            source: None,
        }
    }

    /// A `Features` carrying a name (its `id`) and the requirements it opts into via
    /// `satisfies` — the facts the satisfier set is built from.
    fn features(name: &str, satisfies: &[&str]) -> Features {
        Features {
            id: name.to_string(),
            fields: BTreeMap::new(),
            body_lines: 1,
            headings: Vec::new(),
            sections: Vec::new(),
            source_dir: Some(name.to_string()),
            directives: Vec::new(),
            fenced_blocks: Vec::new(),
            nested_members: Vec::new(),
            satisfies: satisfies.iter().map(|s| s.to_string()).collect(),
            edge_placements: None,
        }
    }

    /// A bare `Requirement` template with every facet defaulted except its name —
    /// the parser's own optional-facet defaults, so each test case fills in only the
    /// facets it needs via struct-update syntax.
    fn requirement(name: &str) -> Requirement {
        Requirement {
            name: name.to_string(),
            prose: None,
            kind: None,
            required: false,
            clauses: Vec::new(),
            verified_by: None,
        }
    }

    /// A required, typed single-satisfier requirement over the `skill` kind.
    fn required_requirement() -> Requirement {
        Requirement {
            kind: Some("skill".to_string()),
            required: true,
            ..requirement("planner")
        }
    }

    /// Pack a roster of one requirement and a skill candidate set into the shapes
    /// [`check`] takes.
    fn run(req: Requirement, skills: &[Features]) -> Vec<Diagnostic> {
        let mut requirements = BTreeMap::new();
        requirements.insert(req.name.clone(), req);
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", skills)]);
        judge_roster(&requirements, &by_kind)
    }

    #[test]
    fn is_satisfier_reads_the_opt_in_link() {
        // The satisfier join is exact-string membership of the `satisfies` list.
        assert!(is_satisfier(
            "planner",
            &features("plan-skill", &["planner"])
        ));
        assert!(!is_satisfier(
            "planner",
            &features("other", &["something-else"])
        ));
        assert!(!is_satisfier("planner", &features("bare", &[])));
    }

    #[test]
    fn a_plain_required_requirement_fires_no_set_scope_finding() {
        // A `required` requirement declares no set-scope predicate, so `check` adds
        // nothing regardless of satisfier count — the ≥1-satisfier presence is
        // coverage's referential gate, not this pass's. Zero, one, or many satisfiers
        // are all silent here.
        let req = required_requirement();
        assert!(run(req.clone(), &[features("lint-rust", &[])]).is_empty());
        assert!(run(req.clone(), &[features("plan-tasks", &["planner"])]).is_empty());
        assert!(
            run(
                req,
                &[
                    features("plan-tasks", &["planner"]),
                    features("plan-sprints", &["planner"]),
                ],
            )
            .is_empty()
        );
    }

    /// Pack a roster of one requirement over a multi-kind candidate corpus into the
    /// shape [`check`] takes — the each-grain `kind` clause tests' way to place
    /// satisfiers of more than one modeled kind.
    fn run_multi_kind(req: Requirement, by_kind: &BTreeMap<&str, &[Features]>) -> Vec<Diagnostic> {
        let mut requirements = BTreeMap::new();
        requirements.insert(req.name.clone(), req);
        judge_roster(&requirements, by_kind)
    }

    #[test]
    fn a_wrong_kind_opt_in_fires_a_kind_finding_never_a_silent_exclusion() {
        // `agents` narrows to `skill`, but a `rule` also opts in via `satisfies` — the
        // kind-blind satisfier set draws it in, and the each-grain `kind` clause
        // `requirement.kind` sources flags it as a finding instead of silently
        // dropping it from the set.
        let req = Requirement {
            kind: Some("skill".to_string()),
            ..requirement("agents")
        };
        let skills = [features("agent-skill", &["agents"])];
        let rules = [features("agent-rule", &["agents"])];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("skill", &skills[..]), ("rule", &rules[..])]);
        let diags = run_multi_kind(req, &by_kind);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, "requirement.agents.kind");
        assert_eq!(diags[0].artifact, "agents");
        assert!(diags[0].message.contains("agent-rule"));
        assert!(diags[0].message.contains("skill"));
    }

    #[test]
    fn a_kind_blind_requirement_is_filled_by_opt_ins_of_every_modeled_kind() {
        // No `kind` at all: any opt-in artifact, of any modeled kind, satisfies the
        // requirement, and no narrowing clause attaches — silent regardless of which
        // kind opted in.
        let req = requirement("agents");
        let skills = [features("agent-skill", &["agents"])];
        let rules = [features("agent-rule", &["agents"])];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("skill", &skills[..]), ("rule", &rules[..])]);
        assert!(run_multi_kind(req, &by_kind).is_empty());
    }

    /// A `count = { min, max }` band requirement over the `skill` kind — the set-scope
    /// predicate, mutually exclusive with `required`.
    fn count_band_requirement(min: usize, max: usize) -> Requirement {
        Requirement {
            kind: Some("skill".to_string()),
            clauses: vec![required_clause(Predicate::Count { min, max })],
            ..requirement("agents")
        }
    }

    #[test]
    fn a_count_band_is_clean_inside_and_fires_outside() {
        // A `[1, 2]` band over the satisfier set: one or two satisfiers are clean, zero
        // or three fire. Only artifacts opting into `agents` count.
        let req = count_band_requirement(1, 2);
        let agent = |n: u8| features(&format!("agent-{n}"), &["agents"]);

        // In band (one satisfier, and two satisfiers) ⇒ clean.
        assert!(run(req.clone(), &[agent(1)]).is_empty());
        assert!(run(req.clone(), &[agent(1), agent(2)]).is_empty());

        // Below the band (zero satisfiers — the non-opting skill is ignored) ⇒ fires.
        let below = run(req.clone(), &[features("lint-rust", &[])]);
        assert_eq!(below.len(), 1);
        assert_eq!(below[0].severity, Severity::Error);
        assert_eq!(below[0].rule, "requirement.gate.count");
        assert_eq!(below[0].artifact, "agents");
        assert!(below[0].message.contains("[1, 2]"));

        // Above the band (three satisfiers) ⇒ fires, naming the satisfiers.
        let above = run(req, &[agent(1), agent(2), agent(3)]);
        assert_eq!(above.len(), 1);
        assert!(above[0].message.contains("agent-1"));
        assert!(above[0].message.contains("agent-3"));
        assert!(above[0].message.contains("[1, 2]"));
    }

    #[test]
    fn a_count_requirement_fires_without_a_required_flag() {
        // `count` is an author-declared gate, so it fires independent of `required`
        // (with which it is mutually exclusive) — a `{ min = 2, max = 4 }` requirement
        // with one satisfier is out of band.
        let one = run(
            count_band_requirement(2, 4),
            &[features("agent-1", &["agents"])],
        );
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].rule, "requirement.gate.count");
    }

    /// A requirement declaring `unique = ["model"]` over the `skill` kind — the
    /// set-scope uniqueness predicate over the satisfiers' `model` field.
    fn unique_model_requirement() -> Requirement {
        Requirement {
            kind: Some("skill".to_string()),
            clauses: vec![required_clause(Predicate::Unique {
                field: "model".to_string(),
            })],
            ..requirement("agents")
        }
    }

    /// A `Features` opting into `agents` and carrying an optional `model:` scalar
    /// field — the field the `unique`/`membership` predicates read.
    fn skill_with_model(name: &str, model: Option<&str>) -> Features {
        skill_satisfying(name, &["agents"], model)
    }

    /// A `Features` opting into the named requirements and carrying an optional
    /// `model:` scalar.
    fn skill_satisfying(name: &str, satisfies: &[&str], model: Option<&str>) -> Features {
        let mut f = features(name, satisfies);
        if let Some(model) = model {
            f.fields.insert(
                "model".to_string(),
                FeatureValue::scalar(ValueType::String, model),
            );
        }
        f
    }

    #[test]
    fn a_unique_field_fires_on_a_shared_value_and_is_silent_when_distinct() {
        let req = unique_model_requirement();

        // Two satisfiers sharing a `model` value ⇒ one error naming the field, the
        // shared value, and the colliding satisfiers.
        let collide = run(
            req.clone(),
            &[
                skill_with_model("agent-1", Some("opus")),
                skill_with_model("agent-2", Some("opus")),
            ],
        );
        assert_eq!(collide.len(), 1);
        assert_eq!(collide[0].severity, Severity::Error);
        assert_eq!(collide[0].rule, "requirement.gate.unique");
        assert_eq!(collide[0].artifact, "agents");
        assert!(collide[0].message.contains("model"));
        assert!(collide[0].message.contains("opus"));
        assert!(collide[0].message.contains("agent-1"));
        assert!(collide[0].message.contains("agent-2"));

        // Every satisfier's `model` differs ⇒ silent.
        let distinct = run(
            req,
            &[
                skill_with_model("agent-1", Some("opus")),
                skill_with_model("agent-2", Some("sonnet")),
            ],
        );
        assert!(distinct.is_empty());
    }

    #[test]
    fn a_unique_field_missing_from_the_satisfiers_is_no_collision() {
        // Neither satisfier declares `model` — no extracted value to share, so a
        // missing field is no collision.
        let req = unique_model_requirement();
        let diags = run(
            req,
            &[
                skill_with_model("agent-1", None),
                skill_with_model("agent-2", None),
            ],
        );
        assert!(diags.is_empty());
    }

    #[test]
    fn unique_groups_only_the_satisfiers() {
        // The non-opting `lint-rust` shares `model` with nothing in the satisfier set,
        // and only `agents` satisfiers are grouped — so a lone satisfier is silent even
        // though an out-of-set artifact carries the same `model`.
        let req = unique_model_requirement();
        let diags = run(
            req,
            &[
                skill_with_model("agent-1", Some("opus")),
                skill_satisfying("lint-rust", &[], Some("opus")),
            ],
        );
        assert!(diags.is_empty());
    }

    /// A two-requirement roster: `agents` declares `membership` of its satisfiers'
    /// `model` field, drawn from the `model` feature of `source_kind` artifacts
    /// satisfying the named `approved-model` requirement (R₂) — the set-scope
    /// membership predicate, with a corpus-derived allowed set. `target` names a
    /// *declared* requirement now (`10-contracts.md`), so R₂ itself must be in the
    /// roster for its kind/satisfier set to resolve.
    fn membership_roster(source_kind: &str) -> BTreeMap<String, Requirement> {
        let agents = Requirement {
            kind: Some("skill".to_string()),
            clauses: vec![required_clause(Predicate::Membership {
                field: "model".to_string(),
                target: "approved-model".to_string(),
            })],
            ..requirement("agents")
        };
        let approved = Requirement {
            kind: Some(source_kind.to_string()),
            ..requirement("approved-model")
        };
        BTreeMap::from([
            (agents.name.clone(), agents),
            (approved.name.clone(), approved),
        ])
    }

    #[test]
    fn a_membership_fires_outside_the_derived_set_and_is_silent_inside() {
        // S₁ (satisfying `agents`) and S₂ (satisfying `approved-model`) are both skills
        // here. The allowed set is { opus, sonnet } (the `model` of the two approved
        // skills); `agent-2`'s `gpt` is outside it, `agent-1`'s `opus` is inside.
        let requirements = membership_roster("skill");
        let skills = [
            skill_with_model("agent-1", Some("opus")),
            skill_with_model("agent-2", Some("gpt")),
            skill_satisfying("approved-a", &["approved-model"], Some("opus")),
            skill_satisfying("approved-b", &["approved-model"], Some("sonnet")),
        ];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        let diags = judge_roster(&requirements, &by_kind);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, "requirement.gate.membership");
        assert_eq!(diags[0].artifact, "agents");
        assert!(diags[0].message.contains("agent-2"));
        assert!(diags[0].message.contains("gpt"));
        assert!(diags[0].message.contains("model"));

        // Every satisfier's `model` is drawn from the approved set ⇒ silent.
        let clean = [
            skill_with_model("agent-1", Some("opus")),
            skill_with_model("agent-2", Some("sonnet")),
            skill_satisfying("approved-a", &["approved-model"], Some("opus")),
            skill_satisfying("approved-b", &["approved-model"], Some("sonnet")),
        ];
        let by_kind_clean: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &clean[..])]);
        assert!(judge_roster(&requirements, &by_kind_clean).is_empty());
    }

    #[test]
    fn a_membership_draws_its_set_from_a_second_kind() {
        // `approved-model` is typed to `manifest`, a kind other than `agents`'s own
        // `skill` — the allowed set is resolved off the full roster/by-kind map, so
        // no signature change is needed.
        let requirements = membership_roster("manifest");
        let skills = [
            skill_with_model("agent-1", Some("opus")),
            skill_with_model("agent-2", Some("gpt")),
        ];
        let manifests = [
            skill_satisfying("approved-a", &["approved-model"], Some("opus")),
            skill_satisfying("approved-b", &["approved-model"], Some("sonnet")),
        ];
        let by_kind: BTreeMap<&str, &[Features]> =
            BTreeMap::from([("skill", &skills[..]), ("manifest", &manifests[..])]);
        let diags = judge_roster(&requirements, &by_kind);
        // Only `agent-2` (`gpt`) is outside the manifest-derived { opus, sonnet }.
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "requirement.gate.membership");
        assert!(diags[0].message.contains("agent-2"));
        assert!(diags[0].message.contains("approved-model"));
    }

    #[test]
    fn a_membership_satisfier_missing_the_field_is_skipped() {
        // `agent-2` declares no `model`, so it carries no value to check — a missing
        // field is no membership violation. `agent-1`'s `opus` is in the set, so the
        // run is clean.
        let requirements = membership_roster("skill");
        let skills = [
            skill_with_model("agent-1", Some("opus")),
            skill_with_model("agent-2", None),
            skill_satisfying("approved-a", &["approved-model"], Some("opus")),
        ];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        assert!(judge_roster(&requirements, &by_kind).is_empty());
    }

    #[test]
    fn a_membership_with_an_empty_source_set_flags_every_valued_satisfier() {
        // S₂ (satisfying `approved-model`) has no members, so the derived set is empty —
        // every satisfier that declares `model` is genuinely a non-member, a true
        // positive over the corpus-derived set.
        let requirements = membership_roster("skill");
        let skills = [
            skill_with_model("agent-1", Some("opus")),
            skill_with_model("agent-2", Some("sonnet")),
        ];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        let diags = judge_roster(&requirements, &by_kind);
        assert_eq!(diags.len(), 2);
        assert!(
            diags
                .iter()
                .all(|d| d.rule == "requirement.gate.membership")
        );
    }

    #[test]
    fn a_membership_naming_an_undeclared_target_flags_every_valued_satisfier() {
        // `approved-model` is not itself in the roster — an undeclared `target` has
        // no satisfier set at all, so the allowed set is empty exactly as when the
        // declared target has no satisfiers.
        let mut requirements = BTreeMap::new();
        let agents = Requirement {
            kind: Some("skill".to_string()),
            clauses: vec![required_clause(Predicate::Membership {
                field: "model".to_string(),
                target: "approved-model".to_string(),
            })],
            ..requirement("agents")
        };
        requirements.insert(agents.name.clone(), agents);
        let skills = [skill_with_model("agent-1", Some("opus"))];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        let diags = judge_roster(&requirements, &by_kind);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "requirement.gate.membership");
    }

    // ---- admissibility ----------------------------------------------------

    /// Run the admissibility pass over a one-requirement roster against a `skill`-only
    /// `by_kind` (the modeled kinds are its keys; admissibility reads no satisfiers).
    fn run_admissibility(req: Requirement, base_dir: &Path) -> Vec<Diagnostic> {
        let mut requirements = BTreeMap::new();
        requirements.insert(req.name.clone(), req);
        let skills: [Features; 0] = [];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        admissibility(&requirements, &by_kind, base_dir)
    }

    #[test]
    fn a_required_requirement_over_an_unmodeled_kind_is_inadmissible() {
        // `command` is not a kind `temper` models (only `skill` is in `by_kind`), so
        // the each-grain `kind` clause `command` sources can never be filled by any
        // real satisfier — inadmissible.
        let req = Requirement {
            kind: Some("command".to_string()),
            required: true,
            ..requirement("releaser")
        };
        let diags = run_admissibility(req, Path::new(""));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, "requirement.admissibility");
        assert_eq!(diags[0].artifact, "releaser");
        assert!(diags[0].message.contains("command"));
        assert!(diags[0].message.contains("never be filled"));
    }

    #[test]
    fn a_dangling_verified_by_is_inadmissible() {
        // The `verified_by` path does not exist under the base dir — a dangling
        // verifier is a silent no-op, so it fails admissibility.
        let req = Requirement {
            kind: Some("skill".to_string()),
            required: true,
            verified_by: Some("tests/nope.rs".to_string()),
            ..requirement("planner")
        };
        let diags = run_admissibility(req, Path::new("/no-such-temper-base-dir"));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "requirement.admissibility");
        assert!(diags[0].message.contains("verifier"));
        assert!(diags[0].message.contains("tests/nope.rs"));
    }

    #[test]
    fn a_fully_resolving_roster_is_admissible() {
        // A modeled kind and no verifier — nothing for admissibility to reject.
        assert!(run_admissibility(required_requirement(), Path::new("")).is_empty());
    }

    #[test]
    fn a_bare_requirement_is_admissible() {
        // A pure opt-in-coverage requirement (only `prose` + `required`, no `kind`) has
        // no facet for admissibility to reject — coverage gates its fill, not the
        // roster.
        let req = Requirement {
            prose: Some("the harness maintains dev standards".to_string()),
            required: true,
            ..requirement("dev-standards")
        };
        assert!(run_admissibility(req, Path::new("")).is_empty());
    }

    #[test]
    fn an_inverted_count_bound_is_inadmissible() {
        // `min > max` admits no cardinality at all — a vacuous clause the author
        // cannot have meant, so the definition fails admissibility (reusing
        // `crate::engine::inadmissibilities`' `range`-mirroring `min > max` rule).
        let req = Requirement {
            kind: Some("skill".to_string()),
            clauses: vec![required_clause(Predicate::Count { min: 3, max: 1 })],
            ..requirement("agents")
        };
        let diags = run_admissibility(req, Path::new(""));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, "requirement.admissibility");
        assert_eq!(diags[0].artifact, "agents");
        assert!(diags[0].message.contains("count"));
        assert!(diags[0].message.contains("min 3 greater than max 1"));
    }

    #[test]
    fn a_requirements_clauses_admit_every_predicate_this_scope_judges() {
        // The other half of the facet split: a per-artifact contract has no judge for
        // the node-set family and fences it, but here every one of them is judged —
        // `count`/`unique`/`membership`/`kind` over the satisfier set by `check`,
        // `degree` over the reference graph by `crate::graph` — so a requirement
        // declaring all five is admissible.
        let req = Requirement {
            kind: Some("skill".to_string()),
            clauses: vec![
                required_clause(Predicate::Count { min: 1, max: 3 }),
                required_clause(Predicate::Unique {
                    field: "name".to_string(),
                }),
                required_clause(Predicate::Membership {
                    field: "model".to_string(),
                    target: "approved-models".to_string(),
                }),
                required_clause(Predicate::Degree {
                    incoming: Some(crate::contract::EdgeBound {
                        min: Some(1),
                        max: None,
                    }),
                    outgoing: None,
                }),
                required_clause(Predicate::Kind {
                    kind: "skill".to_string(),
                }),
            ],
            ..requirement("agents")
        };
        assert!(
            run_admissibility(req, Path::new("")).is_empty(),
            "the facet that carries a judge must keep admitting all five"
        );
    }

    #[test]
    fn a_well_ordered_count_bound_is_admissible() {
        // `min <= max` (including a degenerate exactly-one `[1, 1]` band) is a
        // satisfiable bound — nothing for admissibility to reject.
        let req = Requirement {
            kind: Some("skill".to_string()),
            clauses: vec![required_clause(Predicate::Count { min: 1, max: 1 })],
            ..requirement("agents")
        };
        assert!(run_admissibility(req, Path::new("")).is_empty());
    }

    #[test]
    fn a_non_required_requirement_over_an_unmodeled_kind_is_inadmissible_too() {
        // Kind-blindness decouples fillability from `kind`: any opt-in artifact of
        // any modeled kind satisfies coverage regardless of `required`. But the
        // each-grain `kind` clause an unmodeled `kind` sources can never hold for any
        // real satisfier either way, so admissibility (a) no longer gates on
        // `required` — a non-required requirement over an unmodeled kind is
        // inadmissible exactly like a required one.
        let req = Requirement {
            kind: Some("command".to_string()),
            ..requirement("releaser")
        };
        let diags = run_admissibility(req, Path::new(""));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "requirement.admissibility");
        assert!(diags[0].message.contains("command"));
    }
}
