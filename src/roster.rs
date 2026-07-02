//! Roster checks — the set-scope predicates, conformance, and admissibility passes
//! over a parsed harness contract's named requirements (`specs/10-contracts.md`;
//! `specs/45-governance.md`, "The set scope").
//!
//! Three decidable passes read the same parsed requirements: [`check`] gates the
//! author-declared `count`/`unique`/`membership` predicates over each requirement's
//! **satisfier set** — the `kind`-typed artifacts opting in via `satisfies`;
//! [`conformance`] validates those satisfiers against a bound package's contract; and
//! [`admissibility`] checks each requirement's own definition before the roster is
//! trusted to judge a harness.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::check::Diagnostic;
use crate::compose::{CountBound, Membership, PackageResolver, Requirement};
use crate::engine;
use crate::extract::{FeatureValue, Features};

/// The diagnostic `rule` id every set-scope `count` finding reports under
/// (`specs/45-governance.md`, "The set scope (the roster)").
const REQUIREMENT_COUNT_RULE: &str = "requirement.count";

/// The diagnostic `rule` id every conformance finding reports under — a requirement's
/// `conforms-to` clause (`specs/10-contracts.md`).
const REQUIREMENT_CONFORMS_TO_RULE: &str = "requirement.conforms-to";

/// The diagnostic `rule` id every roster-admissibility finding reports under
/// (`specs/10-contracts.md`, "Decision: the contract is itself checked — admissibility").
const REQUIREMENT_ADMISSIBILITY_RULE: &str = "requirement.admissibility";

/// The diagnostic `rule` id every set-scope `unique` finding reports under
/// (`specs/45-governance.md`, "The set scope (the roster)").
const REQUIREMENT_UNIQUE_RULE: &str = "requirement.unique";

/// The diagnostic `rule` id every set-scope `membership` finding reports under
/// (`specs/45-governance.md`, "The set scope (the roster)").
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
/// `satisfies` (`specs/45-governance.md`, "The set scope"). The set every set-scope
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

/// Run the set-scope predicates over the parsed roster, returning an error-severity
/// [`Diagnostic`] per satisfier set that violates a declared `count` / `unique` /
/// `membership` gate.
///
/// Every predicate quantifies over the requirement's **satisfier set** — the
/// `kind`-typed artifacts that opt in via `satisfies`; a kind-blind requirement draws
/// from every modeled kind of `by_kind`. Requirements iterate in name order (the roster
/// is a [`BTreeMap`]) and each kind's candidates arrive name-sorted, so the finding set
/// is stable across runs.
///
/// This pass gates only the predicates the author declared: the ≥1-satisfier presence
/// of a plain `required` requirement is [`crate::coverage`]'s gate, and `conforms-to`
/// is [`conformance`]'s. `resolver` resolves a `membership` `conforms_to` package name
/// to its contract.
#[must_use]
pub fn check(
    requirements: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&str, &[Features]>,
    resolver: &PackageResolver,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for requirement in requirements.values() {
        let satisfiers = satisfiers_for(requirement, by_kind);

        // `count` is an author-declared gate — it fires whenever declared, mutually
        // exclusive with `required` (which coverage gates as ≥1).
        // specs/45-governance.md, "The set scope"
        if let Some(bound) = &requirement.count
            && !(bound.min..=bound.max).contains(&satisfiers.len())
        {
            diagnostics.push(out_of_band(requirement, bound, &satisfiers));
        }

        // `unique` is orthogonal to `count`, so it fires regardless of it.
        // specs/45-governance.md, "The set scope"
        for field in &requirement.unique {
            diagnostics.extend(duplicates(requirement, field, &satisfiers));
        }

        // S₂'s kind may differ from the requirement's own, so the source set is built
        // off the full `by_kind` map, not `satisfiers`. Orthogonal to `count`/`unique`.
        // specs/45-governance.md, "The set scope"
        if let Some(membership) = &requirement.membership {
            diagnostics.extend(out_of_set(
                requirement,
                membership,
                &satisfiers,
                by_kind,
                resolver,
            ));
        }
    }
    diagnostics
}

/// Run the `conforms-to` half of a requirement over the parsed roster
/// (`specs/10-contracts.md`, the `package` typing facet): validate each
/// `package`-binding requirement's satisfiers against the resolved package's contract,
/// retagging every finding under [`REQUIREMENT_CONFORMS_TO_RULE`] and naming the
/// requirement the satisfier broke. Packages **compose** — this is *in addition to* the
/// satisfier's own kind check.
///
/// A requirement binding no `package` imposes no shape (skipped). A requirement whose
/// package does not resolve or fails to load is skipped rather than reported: that is
/// admissibility's finding, and double-reporting would be noise. `resolver` resolves a
/// bound package name (PACKAGE-BINDING's order); `by_kind` is [`check`]'s map.
#[must_use]
pub fn conformance(
    requirements: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&str, &[Features]>,
    resolver: &PackageResolver,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for requirement in requirements.values() {
        // A requirement binding no package imposes no shape — nothing to conform.
        let Some(package) = &requirement.package else {
            continue;
        };
        // A non-resolving/malformed package is admissibility's to report, not
        // ours — skip the conformance check rather than double-report it.
        let Ok(Some(contract)) = resolver.resolve(package) else {
            continue;
        };
        let satisfiers: Vec<Features> = satisfiers_for(requirement, by_kind)
            .into_iter()
            .cloned()
            .collect();
        for finding in engine::validate(&contract, &satisfiers) {
            diagnostics.push(conformance_finding(requirement, &finding));
        }
    }
    diagnostics
}

/// Validate the harness roster against **the definition** — admissibility
/// (`specs/10-contracts.md`, "Decision: the contract is itself checked —
/// admissibility"). Each requirement's own definition must pass a check *before* the
/// roster is used to judge anything; every finding is [`Diagnostic::error`] (an
/// inadmissible requirement cannot be trusted) and names the requirement it indicts.
///
/// Five decidable clauses over the requirement's *present* facets — an absent facet
/// imposes no check:
///
/// - **(a)** a `required` typed requirement's `kind` is one `temper` models, else it
///   can never be filled (a kind-blind requirement is filled by opt-in `satisfies`).
/// - **(b)** its `package` resolves through the [`PackageResolver`] and the resolved
///   [`Contract`](crate::contract::Contract) is itself admissible — `names a real
///   package`, the case [`conformance`] skips. Whether it is *filled* stays coverage's.
/// - **(c)** any `verified_by` path exists relative to `base_dir` (a dangling verifier
///   is the silent no-op `00-intent.md` law 1 forbids).
/// - **(d)** a declared `count` bound is well-ordered (`min <= max`), mirroring
///   `range`'s `min > max` rejection (`specs/45-governance.md`).
/// - **(e)** a `membership` `conforms_to` reference, held to (b)'s bar.
///
/// `by_kind` supplies only the modeled kinds (its keys), never satisfiers. `resolver`
/// resolves a bound package name; `base_dir` is the `temper.toml` directory a
/// `verified_by` path resolves against.
#[must_use]
pub fn admissibility(
    requirements: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&str, &[Features]>,
    resolver: &PackageResolver,
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

        // (b) The `package` resolves and the resolved contract is itself admissible; a
        // non-resolving name is this pass's finding, not `conformance`'s.
        if let Some(package) = &requirement.package {
            match resolver.resolve(package) {
                Ok(None) => diagnostics.push(Diagnostic::error(
                    REQUIREMENT_ADMISSIBILITY_RULE,
                    name,
                    format!(
                        "requirement `{name}` binds package `{package}`, which does not resolve to a built-in or project package"
                    ),
                )),
                Err(error) => diagnostics.push(Diagnostic::error(
                    REQUIREMENT_ADMISSIBILITY_RULE,
                    name,
                    format!("requirement `{name}` package `{package}` does not load: {error}"),
                )),
                Ok(Some(contract)) => {
                    for finding in engine::admissibility(&contract) {
                        diagnostics.push(Diagnostic::error(
                            REQUIREMENT_ADMISSIBILITY_RULE,
                            name,
                            format!(
                                "requirement `{name}` package is inadmissible: {}",
                                finding.message
                            ),
                        ));
                    }
                }
            }
        }

        // (c) A `verified_by` path must exist — a dangling verifier is a silent no-op.
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

        // (d) An inverted `count` bound (`min > max`) admits no cardinality —
        // inadmissible, mirroring `range`'s rejection (`specs/45-governance.md`).
        if let Some(bound) = &requirement.count
            && bound.min > bound.max
        {
            diagnostics.push(Diagnostic::error(
                REQUIREMENT_ADMISSIBILITY_RULE,
                name,
                format!(
                    "requirement `{name}` declares an inverted count bound (min {} greater than max {}), which no satisfier set can satisfy",
                    bound.min, bound.max
                ),
            ));
        }

        // (e) A `membership` `conforms_to` reference, held to (b)'s bar. A non-resolving
        // one would silently drop the whole membership check (`out_of_set` skips it), so
        // it must be reported here.
        if let Some(source_package) = requirement
            .membership
            .as_ref()
            .and_then(|m| m.source_package.as_ref())
        {
            match resolver.resolve(source_package) {
                Ok(None) => diagnostics.push(Diagnostic::error(
                    REQUIREMENT_ADMISSIBILITY_RULE,
                    name,
                    format!(
                        "requirement `{name}` `membership` `conforms_to` binds package `{source_package}`, which does not resolve to a built-in or project package"
                    ),
                )),
                Err(error) => diagnostics.push(Diagnostic::error(
                    REQUIREMENT_ADMISSIBILITY_RULE,
                    name,
                    format!(
                        "requirement `{name}` `membership` `conforms_to` package `{source_package}` does not load: {error}"
                    ),
                )),
                Ok(Some(contract)) => {
                    for finding in engine::admissibility(&contract) {
                        diagnostics.push(Diagnostic::error(
                            REQUIREMENT_ADMISSIBILITY_RULE,
                            name,
                            format!(
                                "requirement `{name}` `membership` `conforms_to` is inadmissible: {}",
                                finding.message
                            ),
                        ));
                    }
                }
            }
        }
    }
    diagnostics
}

/// Recast one [`engine::validate`] finding as a requirement-conformance finding: the
/// satisfier artifact and the clause's declared severity carry over unchanged, the
/// `rule` becomes [`REQUIREMENT_CONFORMS_TO_RULE`], and the message names the
/// requirement whose contract the satisfier broke so a reader knows which one indicted it.
fn conformance_finding(requirement: &Requirement, finding: &Diagnostic) -> Diagnostic {
    Diagnostic::new(
        finding.severity,
        REQUIREMENT_CONFORMS_TO_RULE,
        finding.artifact.as_str(),
        format!(
            "satisfier `{}` does not conform to requirement `{}`: {}",
            finding.artifact, requirement.name, finding.message
        ),
    )
    // Carry the broken clause's guidance through the recast: the teaching moment
    // survives the requirement re-framing (`specs/10-contracts.md`, "Packages").
    .with_guidance(finding.guidance.clone())
}

/// The finding for a requirement whose satisfier-set cardinality falls outside its
/// declared `count` bound — naming the requirement, the count, the kind, the
/// satisfiers, and the `[min, max]` bound it missed (`specs/45-governance.md`, "The set
/// scope").
fn out_of_band(
    requirement: &Requirement,
    bound: &CountBound,
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
    Diagnostic::error(
        REQUIREMENT_COUNT_RULE,
        &requirement.name,
        format!(
            "requirement `{}` is satisfied by {} `{}` artifact(s){listed}, outside its declared count bound [{}, {}]",
            requirement.name,
            satisfiers.len(),
            kind_label(requirement),
            bound.min,
            bound.max
        ),
    )
}

/// The set-scope `unique` findings for one declared `field` over a requirement's
/// satisfier set (`specs/45-governance.md`, "The set scope"): group the satisfiers
/// by the field's extracted scalar value and emit one error per value two or more
/// satisfiers share. A satisfier missing the field carries no value to collide on, so
/// it is silently skipped — a missing field is no collision. Values are grouped in a
/// [`BTreeMap`] so the finding set is stable across runs.
fn duplicates(requirement: &Requirement, field: &str, satisfiers: &[&Features]) -> Vec<Diagnostic> {
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
        .map(|(value, satisfiers)| duplicate(requirement, field, value, &satisfiers))
        .collect()
}

/// The finding for a `unique` field two or more satisfiers share — naming the
/// requirement, the field, the shared value, and the colliding satisfiers.
fn duplicate(
    requirement: &Requirement,
    field: &str,
    value: &str,
    satisfiers: &[&str],
) -> Diagnostic {
    Diagnostic::error(
        REQUIREMENT_UNIQUE_RULE,
        &requirement.name,
        format!(
            "requirement `{}` requires `{field}` unique across its satisfier set, but {} satisfiers share `{field}` = `{value}` ({})",
            requirement.name,
            satisfiers.len(),
            satisfiers.join(", ")
        ),
    )
}

/// The set-scope `membership` findings for one requirement over its satisfier set
/// (`specs/45-governance.md`, "The set scope"): build the allowed set from the
/// source feature `G` extracted over the S₂ satisfier set — the `source_kind`
/// artifacts opting into the `source` requirement (R₂) — then emit one error per S₁
/// satisfier whose declared field-`F` scalar is absent from that set. A satisfier
/// missing `F` carries no value to check, so it is silently skipped — a missing field
/// is no violation, the way a missing `unique` field is no collision. The allowed set
/// is corpus-*derived*, so an S₂ with no satisfiers (or whose satisfiers all lack `G`)
/// yields the empty set, under which every valued satisfier is genuinely a non-member.
///
/// `by_kind` is the full workspace map — S₂'s `source_kind` may differ from the
/// requirement's own `kind`, so the source candidates come from the map, not the S₁
/// `satisfiers`. Findings follow `satisfiers` order, which is name-sorted, so the
/// set is stable across runs.
///
/// When the membership carries a `conforms_to` **typed-reference** constraint
/// (`specs/45-governance.md`, "The set scope"), S₂ is first narrowed to the source
/// satisfiers that *also* conform to that **package** — resolved through `resolver` and
/// validated by [`engine::validate`], the same machinery [`conformance`] runs — so
/// the allowed set is drawn only from the right *kind* of thing. A source that trips
/// any finding is dropped before the set is built; a non-resolving `conforms_to` is
/// admissibility's to report (like [`conformance`]'s skip), so the membership check
/// is skipped rather than run against an unconstrained source.
fn out_of_set(
    requirement: &Requirement,
    membership: &Membership,
    satisfiers: &[&Features],
    by_kind: &BTreeMap<&str, &[Features]>,
    resolver: &PackageResolver,
) -> Vec<Diagnostic> {
    let source = by_kind
        .get(membership.source_kind.as_str())
        .copied()
        .unwrap_or(&[]);
    // S₂ is the satisfier set of the named source requirement over `source_kind` — an
    // opt-in satisfier set, not a name glob (`specs/45-governance.md`, "each set an
    // opt-in satisfier set").
    let mut matched: Vec<&Features> = source
        .iter()
        .filter(|features| is_satisfier(&membership.source, features))
        .collect();

    // A typed reference narrows S₂ to sources that validate clean against contract C
    // (reusing `conformance`'s resolve + validate). A non-resolving `conforms_to` is
    // admissibility's finding — skip rather than draw a set off an unconstrained source.
    if let Some(source_package) = &membership.source_package {
        let Ok(Some(contract)) = resolver.resolve(source_package) else {
            return Vec::new();
        };
        let owned: Vec<Features> = matched.iter().map(|features| (*features).clone()).collect();
        let nonconforming: BTreeSet<String> = engine::validate(&contract, &owned)
            .into_iter()
            .map(|finding| finding.artifact)
            .collect();
        matched.retain(|features| !nonconforming.contains(features.id.as_str()));
    }

    let allowed: BTreeSet<&str> = matched
        .iter()
        .filter_map(|features| {
            features
                .field(&membership.source_feature)
                .and_then(FeatureValue::as_scalar)
        })
        .collect();

    satisfiers
        .iter()
        .filter_map(|features| {
            let value = features
                .field(&membership.field)
                .and_then(FeatureValue::as_scalar)?;
            if allowed.contains(value) {
                None
            } else {
                Some(not_member(
                    requirement,
                    membership,
                    features.id.as_str(),
                    value,
                ))
            }
        })
        .collect()
}

/// The finding for an S₁ satisfier whose declared field falls outside the S₂-derived
/// set — naming the requirement, the constrained field, the source feature and kind
/// the allowed set is drawn from, the offending satisfier, and the value that is not a
/// member.
fn not_member(
    requirement: &Requirement,
    membership: &Membership,
    satisfier: &str,
    value: &str,
) -> Diagnostic {
    Diagnostic::error(
        REQUIREMENT_MEMBERSHIP_RULE,
        &requirement.name,
        format!(
            "requirement `{}` requires `{}` of each satisfier drawn from the `{}` feature of `{}` artifacts satisfying `{}`, but satisfier `{}` declares `{}` = `{}`, which is not in that set",
            requirement.name,
            membership.field,
            membership.source_feature,
            membership.source_kind,
            membership.source,
            satisfier,
            membership.field,
            value
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use crate::check::Severity;
    use crate::compose::{AuthorLayer, PackageResolver, Requirement};
    use crate::contract::Contract;
    use crate::extract::Kind;

    /// A resolver whose built-in set is the given named packages (no on-disk dir) — the
    /// seam the roster resolves a requirement's `package` and a `membership`
    /// `conforms_to` through (PACKAGE-BINDING's order), driven in-memory so a unit test
    /// names its own packages without touching disk.
    fn pkg_resolver(entries: &[(&str, Contract)]) -> PackageResolver {
        PackageResolver::new(
            entries
                .iter()
                .map(|(name, contract)| ((*name).to_string(), contract.clone()))
                .collect(),
            PathBuf::new(),
        )
    }

    /// An empty resolver — every bound name resolves to nothing. The set-scope passes
    /// (`count` / `unique` / plain `membership`) resolve no package, so they run against
    /// this; a package-typed requirement checked against it is *skipped* by conformance
    /// (a non-resolving package is admissibility's finding).
    fn empty_resolver() -> PackageResolver {
        PackageResolver::new(BTreeMap::new(), PathBuf::new())
    }

    /// A package contract capping a satisfier's `name` at `max` characters — the shape a
    /// `package`-typed requirement binds in these tests, resolved by name.
    fn maxlen_package(max: usize) -> Contract {
        Contract::parse(
            &format!(
                "[[clause]]\n\
                 severity = \"required\"\n\
                 predicate = \"max_len\"\n\
                 field = \"name\"\n\
                 max = {max}\n"
            ),
            Path::new("shape.toml"),
        )
        .unwrap()
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
            satisfies: satisfies.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Parse a single requirement out of a `temper.toml` fragment — the parse
    /// foundation is the only constructor for a [`Requirement`], so the unit tests
    /// drive it.
    fn requirement(toml: &str, name: &str) -> Requirement {
        AuthorLayer::parse(toml, Path::new("temper.toml"))
            .unwrap()
            .requirements()
            .get(name)
            .expect("the fragment declares the requirement")
            .clone()
    }

    /// A required, typed single-satisfier requirement over the `skill` kind, binding the
    /// built-in `skill.anthropic` package by name.
    fn required_requirement() -> Requirement {
        requirement(
            "[requirement.planner]\n\
             kind = \"skill\"\n\
             package = \"skill.anthropic\"\n\
             required = true\n",
            "planner",
        )
    }

    /// Pack a roster of one requirement and a skill candidate set into the shapes
    /// [`check`] takes. The set-scope passes resolve no package, so an empty resolver
    /// suffices.
    fn run(req: Requirement, skills: &[Features]) -> Vec<Diagnostic> {
        let mut requirements = BTreeMap::new();
        requirements.insert(req.name.clone(), req);
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", skills)]);
        check(&requirements, &by_kind, &empty_resolver())
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
        requirement(
            &format!(
                "[requirement.agents]\n\
                 kind = \"skill\"\n\
                 package = \"skill.anthropic\"\n\
                 count = {{ min = {min}, max = {max} }}\n"
            ),
            "agents",
        )
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
        requirement(
            "[requirement.agents]\n\
             kind = \"skill\"\n\
             package = \"skill.anthropic\"\n\
             unique = [\"model\"]\n",
            "agents",
        )
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

    /// A requirement declaring `membership` of its satisfiers' `model` field in the
    /// `model` feature drawn from `source_kind` artifacts satisfying `approved-model` —
    /// the set-scope membership predicate, with a corpus-derived allowed set.
    fn membership_requirement(source_kind: &str) -> Requirement {
        requirement(
            &format!(
                "[requirement.agents]\n\
                 kind = \"skill\"\n\
                 package = \"skill.anthropic\"\n\
                 membership = {{ field = \"model\", kind = \"{source_kind}\", source = \"approved-model\", feature = \"model\" }}\n"
            ),
            "agents",
        )
    }

    /// Pack a roster of one requirement and a multi-kind candidate map into the shapes
    /// [`check`] takes — the membership predicate's S₂ may name a different kind, so
    /// the source artifacts live under their own key.
    fn run_multi(req: Requirement, by_kind: BTreeMap<&str, &[Features]>) -> Vec<Diagnostic> {
        let mut requirements = BTreeMap::new();
        requirements.insert(req.name.clone(), req);
        check(&requirements, &by_kind, &empty_resolver())
    }

    #[test]
    fn a_membership_fires_outside_the_derived_set_and_is_silent_inside() {
        // S₁ (satisfying `agents`) and S₂ (satisfying `approved-model`) are both skills
        // here. The allowed set is { opus, sonnet } (the `model` of the two approved
        // skills); `agent-2`'s `gpt` is outside it, `agent-1`'s `opus` is inside.
        let req = membership_requirement("skill");
        let skills = [
            skill_with_model("agent-1", Some("opus")),
            skill_with_model("agent-2", Some("gpt")),
            skill_satisfying("approved-a", &["approved-model"], Some("opus")),
            skill_satisfying("approved-b", &["approved-model"], Some("sonnet")),
        ];
        let diags = run(req.clone(), &skills);
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
        assert!(run(req, &clean).is_empty());
    }

    #[test]
    fn a_membership_draws_its_set_from_a_second_kind() {
        // S₂ names `manifest`, a kind other than the requirement's own `skill` — the
        // allowed set is built off the full by-kind map, so no signature change is
        // needed.
        let req = membership_requirement("manifest");
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
        let diags = run_multi(req, by_kind);
        // Only `agent-2` (`gpt`) is outside the manifest-derived { opus, sonnet }.
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, REQUIREMENT_MEMBERSHIP_RULE);
        assert!(diags[0].message.contains("agent-2"));
        assert!(diags[0].message.contains("manifest"));
    }

    #[test]
    fn a_membership_satisfier_missing_the_field_is_skipped() {
        // `agent-2` declares no `model`, so it carries no value to check — a missing
        // field is no membership violation. `agent-1`'s `opus` is in the set, so the
        // run is clean.
        let req = membership_requirement("skill");
        let skills = [
            skill_with_model("agent-1", Some("opus")),
            skill_with_model("agent-2", None),
            skill_satisfying("approved-a", &["approved-model"], Some("opus")),
        ];
        assert!(run(req, &skills).is_empty());
    }

    /// A requirement whose `membership` carries a `conforms_to` typed-reference
    /// constraint binding the `tier-required` package by name: the source set is
    /// narrowed to `approved-model` satisfiers that also declare a `tier` field before
    /// the allowed `model` set is drawn.
    fn typed_reference_requirement() -> Requirement {
        requirement(
            "[requirement.agents]\n\
             kind = \"skill\"\n\
             package = \"skill.anthropic\"\n\
             membership = { field = \"model\", kind = \"skill\", source = \"approved-model\", feature = \"model\", conforms_to = \"tier-required\" }\n",
            "agents",
        )
    }

    /// The package a `typed_reference_requirement`'s `conforms_to` binds: a source
    /// conforms iff it declares a `tier` field.
    fn tier_required_package() -> Contract {
        Contract::parse(
            "[[clause]]\n\
             severity = \"required\"\n\
             predicate = \"required\"\n\
             field = \"tier\"\n",
            Path::new("tier-required.toml"),
        )
        .unwrap()
    }

    /// A `skill_satisfying` also carrying a `tier:` scalar — the field the
    /// typed-reference `conforms_to` contract requires of a conforming source.
    fn skill_with_model_and_tier(
        name: &str,
        satisfies: &[&str],
        model: Option<&str>,
        tier: Option<&str>,
    ) -> Features {
        let mut f = skill_satisfying(name, satisfies, model);
        if let Some(tier) = tier {
            f.fields
                .insert("tier".to_string(), FeatureValue::scalar(Kind::String, tier));
        }
        f
    }

    #[test]
    fn a_typed_reference_draws_its_set_only_from_conforming_sources() {
        // `approved-a` conforms (it declares `tier`) and contributes `opus`;
        // `approved-b` does not (no `tier`) and is dropped, so its `gpt` never enters
        // the allowed set. The `agent-gpt` satisfier's `gpt` is therefore a non-member.
        let req = typed_reference_requirement();
        let skills = [
            skill_with_model_and_tier("agent-gpt", &["agents"], Some("gpt"), None),
            skill_with_model_and_tier(
                "approved-a",
                &["approved-model"],
                Some("opus"),
                Some("official"),
            ),
            skill_with_model_and_tier("approved-b", &["approved-model"], Some("gpt"), None),
        ];
        let mut requirements = BTreeMap::new();
        requirements.insert(req.name.clone(), req);
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        // The `conforms_to` binds the `tier-required` package by name, resolved through
        // the in-memory resolver.
        let resolver = pkg_resolver(&[("tier-required", tier_required_package())]);
        let diags = check(&requirements, &by_kind, &resolver);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, REQUIREMENT_MEMBERSHIP_RULE);
        assert!(diags[0].message.contains("agent-gpt"));
        assert!(diags[0].message.contains("gpt"));

        // Give `approved-b` a `tier` so it conforms too: now `gpt` is in the derived
        // set and the same satisfier is silent — the constraint was the only thing
        // excluding it.
        let conforming = [
            skill_with_model_and_tier("agent-gpt", &["agents"], Some("gpt"), None),
            skill_with_model_and_tier(
                "approved-a",
                &["approved-model"],
                Some("opus"),
                Some("official"),
            ),
            skill_with_model_and_tier(
                "approved-b",
                &["approved-model"],
                Some("gpt"),
                Some("official"),
            ),
        ];
        let mut requirements = BTreeMap::new();
        requirements.insert(
            typed_reference_requirement().name.clone(),
            typed_reference_requirement(),
        );
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &conforming[..])]);
        let resolver = pkg_resolver(&[("tier-required", tier_required_package())]);
        assert!(check(&requirements, &by_kind, &resolver).is_empty());
    }

    #[test]
    fn a_membership_with_an_empty_source_set_flags_every_valued_satisfier() {
        // S₂ (satisfying `approved-model`) has no members, so the derived set is empty —
        // every satisfier that declares `model` is genuinely a non-member, a true
        // positive over the corpus-derived set.
        let req = membership_requirement("skill");
        let skills = [
            skill_with_model("agent-1", Some("opus")),
            skill_with_model("agent-2", Some("sonnet")),
        ];
        let diags = run(req, &skills);
        assert_eq!(diags.len(), 2);
        assert!(diags.iter().all(|d| d.rule == REQUIREMENT_MEMBERSHIP_RULE));
    }

    /// A requirement binding the `shape` package by name — the package a
    /// [`maxlen_package`] resolver supplies with the `name`-cap contract these
    /// conformance/admissibility cases exercise.
    fn shape_requirement() -> Requirement {
        requirement(
            "[requirement.planner]\n\
             kind = \"skill\"\n\
             package = \"shape\"\n\
             required = true\n",
            "planner",
        )
    }

    /// `features` opting into `planner` with a `name` scalar field equal to its id —
    /// the field the `shape` package's `max_len` clause measures (the engine validates
    /// extracted *fields*, not the bare diagnostic id).
    fn named_skill(name: &str) -> Features {
        let mut f = features(name, &["planner"]);
        f.fields
            .insert("name".to_string(), FeatureValue::scalar(Kind::String, name));
        f
    }

    /// Pack a roster of one requirement and skill candidates and run the conformance
    /// pass, resolving the requirement's bound `package` through `resolver`.
    fn run_conformance(
        req: Requirement,
        skills: &[Features],
        resolver: &PackageResolver,
    ) -> Vec<Diagnostic> {
        let mut requirements = BTreeMap::new();
        requirements.insert(req.name.clone(), req);
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", skills)]);
        conformance(&requirements, &by_kind, resolver)
    }

    #[test]
    fn a_bound_package_validates_its_satisfiers_only() {
        // The `shape` package caps `name` at 3 chars; the satisfier `plan-tasks` (10)
        // breaks it, while the non-opting `lint-rust` is never validated against the
        // requirement's package.
        let mut lint = named_skill("lint-rust");
        lint.satisfies.clear();
        let resolver = pkg_resolver(&[("shape", maxlen_package(3))]);
        let diags = run_conformance(
            shape_requirement(),
            &[named_skill("plan-tasks"), lint],
            &resolver,
        );
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, REQUIREMENT_CONFORMS_TO_RULE);
        assert_eq!(diags[0].artifact, "plan-tasks");
        // The message names the requirement whose package the satisfier broke.
        assert!(diags[0].message.contains("planner"));
        assert!(diags[0].message.contains("does not conform"));
    }

    #[test]
    fn a_bound_package_is_silent_when_the_satisfier_conforms() {
        // The same shape, but a generous cap the satisfier stays within ⇒ clean.
        let resolver = pkg_resolver(&[("shape", maxlen_package(64))]);
        assert!(
            run_conformance(shape_requirement(), &[named_skill("plan-tasks")], &resolver)
                .is_empty()
        );
    }

    #[test]
    fn conformance_validates_every_satisfier() {
        // Two satisfiers, both breaking the package's cap — conformance reports each.
        let resolver = pkg_resolver(&[("shape", maxlen_package(3))]);
        let diags = run_conformance(
            shape_requirement(),
            &[named_skill("plan-tasks"), named_skill("plan-sprints")],
            &resolver,
        );
        assert_eq!(diags.len(), 2);
        assert!(diags.iter().all(|d| d.rule == REQUIREMENT_CONFORMS_TO_RULE));
    }

    #[test]
    fn a_requirement_whose_package_does_not_resolve_is_skipped_not_reported() {
        // The bound package resolves to nothing (an empty resolver): conformance skips
        // it (a non-resolving package is admissibility's finding), so nothing fires even
        // though a satisfier is present.
        let req = shape_requirement(); // package = "shape", absent from the resolver
        let skills = [features("plan-tasks", &["planner"])];
        let mut requirements = BTreeMap::new();
        requirements.insert(req.name.clone(), req);
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        let diags = conformance(&requirements, &by_kind, &empty_resolver());
        assert!(diags.is_empty());
    }

    // ---- admissibility ----------------------------------------------------

    /// Run the admissibility pass over a one-requirement roster against a `skill`-only
    /// `by_kind` (the modeled kinds are its keys; admissibility reads no satisfiers),
    /// resolving bound package names through `resolver`.
    fn run_admissibility(
        req: Requirement,
        resolver: &PackageResolver,
        base_dir: &Path,
    ) -> Vec<Diagnostic> {
        let mut requirements = BTreeMap::new();
        requirements.insert(req.name.clone(), req);
        let skills: [Features; 0] = [];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        admissibility(&requirements, &by_kind, resolver, base_dir)
    }

    #[test]
    fn a_required_requirement_over_an_unmodeled_kind_is_inadmissible() {
        // `command` is not a kind `temper` models (only `skill` is in `by_kind`),
        // so a required requirement over it can never be filled — inadmissible. The
        // bound package resolves, so the only finding is the satisfiability one.
        let req = requirement(
            "[requirement.releaser]\n\
             kind = \"command\"\n\
             package = \"shape\"\n\
             required = true\n",
            "releaser",
        );
        let resolver = pkg_resolver(&[("shape", maxlen_package(64))]);
        let diags = run_admissibility(req, &resolver, Path::new(""));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, REQUIREMENT_ADMISSIBILITY_RULE);
        assert_eq!(diags[0].artifact, "releaser");
        assert!(diags[0].message.contains("command"));
        assert!(diags[0].message.contains("never be filled"));
    }

    #[test]
    fn a_non_resolving_package_is_inadmissible() {
        // The bound package name matches no built-in and no project package (an empty
        // resolver) — the inadmissibility this pass owns, `names a real package` (the
        // case `conformance` skips).
        let req = requirement(
            "[requirement.planner]\n\
             kind = \"skill\"\n\
             package = \"no-such-package\"\n\
             required = true\n",
            "planner",
        );
        let diags = run_admissibility(req, &empty_resolver(), Path::new(""));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, REQUIREMENT_ADMISSIBILITY_RULE);
        assert_eq!(diags[0].artifact, "planner");
        assert!(diags[0].message.contains("no-such-package"));
        assert!(diags[0].message.contains("does not resolve"));
    }

    #[test]
    fn a_bound_package_with_an_empty_enum_is_inadmissible() {
        // `engine::admissibility` runs on the resolved package's contract, so a vacuous
        // `enum` clause in a bound package is caught here exactly as in a floor contract.
        let empty_enum = Contract::parse(
            "[[clause]]\n\
             severity = \"required\"\n\
             predicate = \"enum\"\n\
             field = \"status\"\n\
             values = []\n",
            Path::new("empty-enum.toml"),
        )
        .unwrap();
        let req = requirement(
            "[requirement.planner]\n\
             kind = \"skill\"\n\
             package = \"empty-enum\"\n\
             required = true\n",
            "planner",
        );
        let resolver = pkg_resolver(&[("empty-enum", empty_enum)]);
        let diags = run_admissibility(req, &resolver, Path::new(""));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, REQUIREMENT_ADMISSIBILITY_RULE);
        assert_eq!(diags[0].artifact, "planner");
        assert!(diags[0].message.contains("inadmissible"));
        assert!(diags[0].message.contains("enum"));
    }

    #[test]
    fn a_dangling_verified_by_is_inadmissible() {
        // The `verified_by` path does not exist under the base dir — a dangling
        // verifier is a silent no-op, so it fails admissibility.
        let req = requirement(
            "[requirement.planner]\n\
             kind = \"skill\"\n\
             package = \"shape\"\n\
             required = true\n\
             verified_by = \"tests/nope.rs\"\n",
            "planner",
        );
        let resolver = pkg_resolver(&[("shape", maxlen_package(64))]);
        let diags = run_admissibility(req, &resolver, Path::new("/no-such-temper-base-dir"));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, REQUIREMENT_ADMISSIBILITY_RULE);
        assert!(diags[0].message.contains("verifier"));
        assert!(diags[0].message.contains("tests/nope.rs"));
    }

    #[test]
    fn a_fully_resolving_roster_is_admissible() {
        // A modeled kind, an admissible bound package, and no verifier — nothing for
        // admissibility to reject.
        let resolver = pkg_resolver(&[("shape", maxlen_package(64))]);
        assert!(run_admissibility(shape_requirement(), &resolver, Path::new("")).is_empty());
    }

    #[test]
    fn a_bare_requirement_is_admissible() {
        // A pure opt-in-coverage requirement (only `means` + `required`, no `kind` or
        // `package`) has no facet for admissibility to reject — coverage gates its
        // fill, not the roster.
        let req = requirement(
            "[requirement.dev-standards]\n\
             means = \"the harness maintains dev standards\"\n\
             required = true\n",
            "dev-standards",
        );
        assert!(run_admissibility(req, &empty_resolver(), Path::new("")).is_empty());
    }

    #[test]
    fn an_inverted_count_bound_is_inadmissible() {
        // `min > max` admits no cardinality at all — a vacuous bound the author
        // cannot have meant, so the definition fails admissibility (mirroring
        // `range`'s `min > max` rejection).
        let req = requirement(
            "[requirement.agents]\n\
             kind = \"skill\"\n\
             package = \"shape\"\n\
             count = { min = 3, max = 1 }\n",
            "agents",
        );
        let resolver = pkg_resolver(&[("shape", maxlen_package(64))]);
        let diags = run_admissibility(req, &resolver, Path::new(""));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, REQUIREMENT_ADMISSIBILITY_RULE);
        assert_eq!(diags[0].artifact, "agents");
        assert!(diags[0].message.contains("inverted count bound"));
    }

    #[test]
    fn a_well_ordered_count_bound_is_admissible() {
        // `min <= max` (including a degenerate exactly-one `[1, 1]` band) is a
        // satisfiable bound — nothing for admissibility to reject.
        let req = requirement(
            "[requirement.agents]\n\
             kind = \"skill\"\n\
             package = \"shape\"\n\
             count = { min = 1, max = 1 }\n",
            "agents",
        );
        let resolver = pkg_resolver(&[("shape", maxlen_package(64))]);
        assert!(run_admissibility(req, &resolver, Path::new("")).is_empty());
    }

    #[test]
    fn a_membership_conforms_to_that_does_not_resolve_is_inadmissible() {
        // The `membership` typed reference binds a package name that resolves to nothing
        // (an empty resolver) — a `conforms_to` that never resolves would silently drop
        // the whole membership check, so admissibility reports it (clause (e)),
        // mirroring the requirement's own package-resolve clause.
        let req = requirement(
            "[requirement.agents]\n\
             kind = \"skill\"\n\
             membership = { field = \"model\", kind = \"skill\", source = \"approved-model\", feature = \"model\", conforms_to = \"no-such-package\" }\n",
            "agents",
        );
        let diags = run_admissibility(req, &empty_resolver(), Path::new(""));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, REQUIREMENT_ADMISSIBILITY_RULE);
        assert_eq!(diags[0].artifact, "agents");
        assert!(diags[0].message.contains("conforms_to"));
        assert!(diags[0].message.contains("does not resolve"));
    }

    #[test]
    fn a_membership_conforms_to_with_an_empty_enum_is_inadmissible() {
        // A bound `conforms_to` package carrying a vacuous `enum` is held to the same
        // admissibility bar as the requirement's own package — caught by (e).
        let empty_enum = Contract::parse(
            "[[clause]]\n\
             severity = \"required\"\n\
             predicate = \"enum\"\n\
             field = \"tier\"\n\
             values = []\n",
            Path::new("empty-enum.toml"),
        )
        .unwrap();
        let req = requirement(
            "[requirement.agents]\n\
             kind = \"skill\"\n\
             membership = { field = \"model\", kind = \"skill\", source = \"approved-model\", feature = \"model\", conforms_to = \"empty-enum\" }\n",
            "agents",
        );
        let resolver = pkg_resolver(&[("empty-enum", empty_enum)]);
        let diags = run_admissibility(req, &resolver, Path::new(""));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, REQUIREMENT_ADMISSIBILITY_RULE);
        assert!(diags[0].message.contains("conforms_to"));
        assert!(diags[0].message.contains("inadmissible"));
    }

    #[test]
    fn a_non_required_requirement_over_an_unmodeled_kind_is_admissible() {
        // Satisfiability gates on `required`: a non-required requirement over an
        // unmodeled kind is merely never filled, which the author may have meant — not
        // an inadmissibility.
        let req = requirement(
            "[requirement.releaser]\n\
             kind = \"command\"\n\
             package = \"shape\"\n",
            "releaser",
        );
        let resolver = pkg_resolver(&[("shape", maxlen_package(64))]);
        assert!(run_admissibility(req, &resolver, Path::new("")).is_empty());
    }
}
