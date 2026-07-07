//! Roster checks — the set-scope predicates and admissibility pass over a parsed
//! harness contract's named requirements (`specs/architecture/10-contracts.md`;
//! `specs/architecture/45-governance.md`, the node-set scope).
//!
//! Two decidable passes read the same parsed requirements: [`check`] gates the
//! author-declared `count`/`unique`/`membership` predicates over each requirement's
//! **satisfier set** — the `kind`-typed artifacts opting in via `satisfies`; and
//! [`admissibility`] checks each requirement's own definition before the roster is
//! trusted to judge a harness.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::check::Diagnostic;
use crate::compose::Requirement;
use crate::contract::{Clause, Predicate};
use crate::engine;
use crate::extract::{FeatureValue, Features};

/// The diagnostic `rule` id every set-scope `count` finding reports under
/// (`specs/architecture/45-governance.md`, the node-set scope).
const REQUIREMENT_COUNT_RULE: &str = "requirement.count";

/// The diagnostic `rule` id every roster-admissibility finding reports under
/// (`specs/architecture/10-contracts.md`, admissibility).
const REQUIREMENT_ADMISSIBILITY_RULE: &str = "requirement.admissibility";

/// The diagnostic `rule` id every set-scope `unique` finding reports under
/// (`specs/architecture/45-governance.md`, the node-set scope).
const REQUIREMENT_UNIQUE_RULE: &str = "requirement.unique";

/// The diagnostic `rule` id every set-scope `membership` finding reports under
/// (`specs/architecture/45-governance.md`, the node-set scope).
const REQUIREMENT_MEMBERSHIP_RULE: &str = "requirement.membership";

/// Whether an artifact opts into the requirement named `requirement` — its
/// `satisfies` list carries that name. The decidable join at the heart of the
/// satisfier set. `pub(crate)` so the graph-scope `degree` check ([`crate::graph`])
/// selects a requirement's satisfier nodes by the *same* opt-in join this roster
/// scope uses, never a second selector that could disagree.
pub(crate) fn is_satisfier(requirement: &str, features: &Features) -> bool {
    features.satisfies.iter().any(|name| name == requirement)
}

/// The candidate artifacts a requirement ranges over before the opt-in filter: its
/// `kind` typing narrows them to that kind's workspace [`Features`], and a kind-blind
/// requirement (no `kind`) draws from every modeled kind. A `kind` the surface does
/// not model yields no candidates.
fn candidates_for<'a>(
    requirement: &Requirement,
    by_kind: &BTreeMap<&str, &'a [Features]>,
) -> Vec<&'a Features> {
    match &requirement.kind {
        Some(kind) => by_kind
            .get(kind.as_str())
            .copied()
            .unwrap_or(&[])
            .iter()
            .collect(),
        None => by_kind
            .values()
            .flat_map(|features| features.iter())
            .collect(),
    }
}

/// The requirement's **satisfier set** — its `kind`-typed candidates that opt in via
/// `satisfies` (`specs/architecture/45-governance.md`, the node-set scope). The set every set-scope
/// predicate quantifies over.
fn satisfiers_for<'a>(
    requirement: &Requirement,
    by_kind: &BTreeMap<&str, &'a [Features]>,
) -> Vec<&'a Features> {
    candidates_for(requirement, by_kind)
        .into_iter()
        .filter(|features| is_satisfier(&requirement.name, features))
        .collect()
}

/// The label for a requirement's `kind` in a diagnostic — the declared kind, or
/// `any` when the requirement is kind-blind (its satisfier may be of any kind).
fn kind_label(requirement: &Requirement) -> &str {
    requirement.kind.as_deref().unwrap_or("any")
}

/// Run the set-scope predicates over the parsed roster, returning a [`Diagnostic`] —
/// at the violating clause's own declared severity — per satisfier set that violates
/// a `count` / `unique` / `membership` clause (`specs/architecture/10-contracts.md`,
/// set-scope demands are clauses).
///
/// Every predicate quantifies over the requirement's **satisfier set** — the
/// `kind`-typed artifacts that opt in via `satisfies`; a kind-blind requirement draws
/// from every modeled kind of `by_kind`. Requirements iterate in name order (the roster
/// is a [`BTreeMap`]), each requirement's clauses in declaration order, and each kind's
/// candidates arrive name-sorted, so the finding set is stable across runs.
///
/// This pass gates only the predicates the author declared: the ≥1-satisfier presence
/// of a plain `required` requirement is [`crate::coverage`]'s gate. `degree` ranges
/// over the edge graph, so [`crate::graph::degree`] reads it off the same
/// [`clauses`](Requirement::clauses) instead.
#[must_use]
pub fn check(
    requirements: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&str, &[Features]>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for requirement in requirements.values() {
        let satisfiers = satisfiers_for(requirement, by_kind);

        for clause in &requirement.clauses {
            match &clause.predicate {
                // `count` fires whenever declared — orthogonal to `required` (which
                // coverage gates as ≥1). specs/architecture/10-contracts.md
                Predicate::Count { min, max } => {
                    if !(*min..=*max).contains(&satisfiers.len()) {
                        diagnostics.push(out_of_band(requirement, clause, *min, *max, &satisfiers));
                    }
                }
                // `unique` is orthogonal to `count`, so it fires regardless of it.
                Predicate::Unique { field } => {
                    diagnostics.extend(duplicates(requirement, clause, field, &satisfiers));
                }
                // S₂'s kind may differ from the requirement's own, so the source set is
                // resolved off the full `requirements` roster, not `satisfiers`.
                Predicate::Membership { field, target } => {
                    diagnostics.extend(out_of_set(
                        requirement,
                        clause,
                        field,
                        target,
                        &satisfiers,
                        requirements,
                        by_kind,
                    ));
                }
                // `degree` is graph-scope — `crate::graph::degree` owns it.
                _ => {}
            }
        }
    }
    diagnostics
}

/// Validate the harness roster against **the definition** — admissibility
/// (`specs/architecture/10-contracts.md`, admissibility). Each requirement's own
/// definition must pass a check *before* the roster is used to judge anything;
/// every finding is [`Diagnostic::error`] (an inadmissible requirement cannot be
/// trusted) and names the requirement it indicts.
///
/// Three decidable clauses:
///
/// - **(a)** a `required` typed requirement's `kind` is one `temper` models, else it
///   can never be filled (a kind-blind requirement is filled by opt-in `satisfies`).
/// - **(b)** any `verified_by` path exists relative to `base_dir` (a dangling verifier
///   is the silent no-op `00-intent.md` law 1 forbids).
/// - **(c)** every clause in [`clauses`](Requirement::clauses) is itself well-formed —
///   reusing [`crate::engine::inadmissibilities`], the same vacuous-clause rules a
///   kind's own floor clauses are checked against (an inverted `count`/`degree` bound,
///   an empty `membership` target), so a requirement's demands and a kind's clauses
///   never carry two definitions of "vacuous".
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

        // (a) A required requirement typed to an unmodeled kind can never be filled.
        if requirement.required
            && let Some(kind) = &requirement.kind
            && !by_kind.contains_key(kind.as_str())
        {
            diagnostics.push(Diagnostic::error(
                REQUIREMENT_ADMISSIBILITY_RULE,
                name,
                format!(
                    "required requirement `{name}` names kind `{kind}`, which `temper` does not model — it can never be filled"
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
            for message in crate::engine::inadmissibilities(&clause.predicate) {
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

/// The finding for a requirement whose satisfier-set cardinality falls outside a
/// declared `count` clause's bound — naming the requirement, the count, the kind, the
/// satisfiers, and the `[min, max]` bound it missed, at the clause's own severity
/// (`specs/architecture/10-contracts.md`).
fn out_of_band(
    requirement: &Requirement,
    clause: &Clause,
    min: usize,
    max: usize,
    satisfiers: &[&Features],
) -> Diagnostic {
    let listed = if satisfiers.is_empty() {
        String::new()
    } else {
        format!(
            " ({})",
            satisfiers
                .iter()
                .map(|features| features.id.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    Diagnostic::new(
        engine::severity_of(clause.severity),
        REQUIREMENT_COUNT_RULE,
        &requirement.name,
        format!(
            "requirement `{}` is satisfied by {} `{}` artifact(s){listed}, outside its declared count bound [{min}, {max}]",
            requirement.name,
            satisfiers.len(),
            kind_label(requirement),
        ),
    )
    .with_guidance(clause.guidance.clone())
}

/// The set-scope `unique` findings for one declared `field` over a requirement's
/// satisfier set (`specs/architecture/10-contracts.md`): group the satisfiers by the
/// field's extracted scalar value and emit one finding per value two or more
/// satisfiers share, at the clause's own declared severity. A satisfier missing the
/// field carries no value to collide on, so it is silently skipped — a missing field
/// is no collision. Values are grouped in a [`BTreeMap`] so the finding set is stable
/// across runs.
fn duplicates(
    requirement: &Requirement,
    clause: &Clause,
    field: &str,
    satisfiers: &[&Features],
) -> Vec<Diagnostic> {
    let mut by_value: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for features in satisfiers {
        if let Some(value) = features.field(field).and_then(FeatureValue::as_scalar) {
            by_value
                .entry(value)
                .or_default()
                .push(features.id.as_str());
        }
    }
    by_value
        .into_iter()
        .filter(|(_, satisfiers)| satisfiers.len() > 1)
        .map(|(value, satisfiers)| duplicate(requirement, clause, field, value, &satisfiers))
        .collect()
}

/// The finding for a `unique` field two or more satisfiers share — naming the
/// requirement, the field, the shared value, and the colliding satisfiers.
fn duplicate(
    requirement: &Requirement,
    clause: &Clause,
    field: &str,
    value: &str,
    satisfiers: &[&str],
) -> Diagnostic {
    Diagnostic::new(
        engine::severity_of(clause.severity),
        REQUIREMENT_UNIQUE_RULE,
        &requirement.name,
        format!(
            "requirement `{}` requires `{field}` unique across its satisfier set, but {} satisfiers share `{field}` = `{value}` ({})",
            requirement.name,
            satisfiers.len(),
            satisfiers.join(", ")
        ),
    )
    .with_guidance(clause.guidance.clone())
}

/// The set-scope `membership` findings for one requirement over its satisfier set
/// (`specs/architecture/10-contracts.md`): build the allowed set from `field` extracted
/// over the `target` requirement's own satisfier set (S₂) — shaping S₂ is `target`'s
/// own job, never re-derived here — then emit one finding per S₁ satisfier whose
/// declared `field` scalar is absent from it, at the clause's own severity. A
/// satisfier missing `field` carries no value to check, so it is silently skipped —
/// a missing field is no violation, the way a missing `unique` field is no collision.
/// The allowed set is corpus-*derived*, so a `target` with no satisfiers (or an
/// undeclared `target`) yields the empty set, under which every valued satisfier is
/// genuinely a non-member.
///
/// `requirements`/`by_kind` are the full roster/workspace maps — `target` may name a
/// requirement typed to a different kind than this one's own, so its satisfier set is
/// resolved through the roster, not `satisfiers`. Findings follow `satisfiers` order,
/// which is name-sorted, so the set is stable across runs.
fn out_of_set(
    requirement: &Requirement,
    clause: &Clause,
    field: &str,
    target: &str,
    satisfiers: &[&Features],
    requirements: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&str, &[Features]>,
) -> Vec<Diagnostic> {
    // S₂ is the named target requirement's own satisfier set — an opt-in satisfier
    // set, not a name glob (`specs/architecture/10-contracts.md`, the satisfier set).
    // An undeclared `target` has no satisfier set at all.
    let source_satisfiers = requirements
        .get(target)
        .map(|target_requirement| satisfiers_for(target_requirement, by_kind))
        .unwrap_or_default();

    let allowed: BTreeSet<&str> = source_satisfiers
        .iter()
        .filter_map(|features| features.field(field).and_then(FeatureValue::as_scalar))
        .collect();

    satisfiers
        .iter()
        .filter_map(|features| {
            let value = features.field(field).and_then(FeatureValue::as_scalar)?;
            if allowed.contains(value) {
                None
            } else {
                Some(not_member(
                    requirement,
                    clause,
                    field,
                    target,
                    features.id.as_str(),
                    value,
                ))
            }
        })
        .collect()
}

/// The finding for an S₁ satisfier whose declared field falls outside the S₂-derived
/// set — naming the requirement, the constrained field, the target requirement the
/// allowed set is drawn from, the offending satisfier, and the value that is not a
/// member.
fn not_member(
    requirement: &Requirement,
    clause: &Clause,
    field: &str,
    target: &str,
    satisfier: &str,
    value: &str,
) -> Diagnostic {
    Diagnostic::new(
        engine::severity_of(clause.severity),
        REQUIREMENT_MEMBERSHIP_RULE,
        &requirement.name,
        format!(
            "requirement `{}` requires `{field}` of each satisfier drawn from the `{field}` feature of artifacts satisfying `{target}`, but satisfier `{satisfier}` declares `{field}` = `{value}`, which is not in that set",
            requirement.name,
        ),
    )
    .with_guidance(clause.guidance.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use crate::check::Severity;
    use crate::contract::Severity as ClauseSeverity;
    use crate::extract::Kind;

    /// A required-severity clause wrapping `predicate` — the shape every set-scope
    /// test case below attaches to a requirement's `clauses`.
    fn required_clause(predicate: Predicate) -> Clause {
        Clause {
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
            genres: Vec::new(),
            satisfies: satisfies.iter().map(|s| s.to_string()).collect(),
            published_requirements: Vec::new(),
        }
    }

    /// A bare `Requirement` template with every facet defaulted except its name —
    /// the parser's own optional-facet defaults, so each test case fills in only the
    /// facets it needs via struct-update syntax.
    fn requirement(name: &str) -> Requirement {
        Requirement {
            name: name.to_string(),
            means: None,
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
        check(&requirements, &by_kind)
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
        assert_eq!(below[0].rule, REQUIREMENT_COUNT_RULE);
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
        assert_eq!(one[0].rule, REQUIREMENT_COUNT_RULE);
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
                FeatureValue::scalar(Kind::String, model),
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
        assert_eq!(collide[0].rule, REQUIREMENT_UNIQUE_RULE);
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
        let diags = check(&requirements, &by_kind);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, REQUIREMENT_MEMBERSHIP_RULE);
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
        assert!(check(&requirements, &by_kind_clean).is_empty());
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
        let diags = check(&requirements, &by_kind);
        // Only `agent-2` (`gpt`) is outside the manifest-derived { opus, sonnet }.
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, REQUIREMENT_MEMBERSHIP_RULE);
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
        assert!(check(&requirements, &by_kind).is_empty());
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
        let diags = check(&requirements, &by_kind);
        assert_eq!(diags.len(), 2);
        assert!(diags.iter().all(|d| d.rule == REQUIREMENT_MEMBERSHIP_RULE));
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
        let diags = check(&requirements, &by_kind);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, REQUIREMENT_MEMBERSHIP_RULE);
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
        // `command` is not a kind `temper` models (only `skill` is in `by_kind`),
        // so a required requirement over it can never be filled — inadmissible.
        let req = Requirement {
            kind: Some("command".to_string()),
            required: true,
            ..requirement("releaser")
        };
        let diags = run_admissibility(req, Path::new(""));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, REQUIREMENT_ADMISSIBILITY_RULE);
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
        assert_eq!(diags[0].rule, REQUIREMENT_ADMISSIBILITY_RULE);
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
        // A pure opt-in-coverage requirement (only `means` + `required`, no `kind`) has
        // no facet for admissibility to reject — coverage gates its fill, not the
        // roster.
        let req = Requirement {
            means: Some("the harness maintains dev standards".to_string()),
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
        assert_eq!(diags[0].rule, REQUIREMENT_ADMISSIBILITY_RULE);
        assert_eq!(diags[0].artifact, "agents");
        assert!(diags[0].message.contains("count"));
        assert!(diags[0].message.contains("min 3 greater than max 1"));
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
    fn a_non_required_requirement_over_an_unmodeled_kind_is_admissible() {
        // Satisfiability gates on `required`: a non-required requirement over an
        // unmodeled kind is merely never filled, which the author may have meant — not
        // an inadmissibility.
        let req = Requirement {
            kind: Some("command".to_string()),
            ..requirement("releaser")
        };
        assert!(run_admissibility(req, Path::new("")).is_empty());
    }
}
