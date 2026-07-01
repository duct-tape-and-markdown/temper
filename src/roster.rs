//! Roster checks — selection, conformance, and admissibility over the parsed
//! harness contract.
//!
//! Implements the requirement tier of `specs/10-contracts.md` ("Requirements — the
//! harness's named obligations"; "Decision: role and requirement are one concept"): a
//! harness contract declares named **requirements**, and a requirement's contract-side
//! `match` selector picks whichever concrete artifact fills it — a decidable selector,
//! never a guess. The requirements parsed onto
//! [`AuthorLayer`](crate::compose::AuthorLayer) reach this module as typed
//! [`Requirement`]s.
//!
//! ## Three passes over one roster
//!
//! Three decidable passes read the same parsed requirements, each owning one facet:
//!
//! - [`check`] — **selection**: each requirement's [`MatchSelector`] is evaluated
//!   against the workspace artifacts of its `kind`, and a **`required` single-filler
//!   requirement filled by zero or by many artifacts is a conformance error**,
//!   reported precisely. A requirement that declares no `match` selector is filled by
//!   opt-in `satisfies` alone — [`crate::coverage`] gates that referential coverage —
//!   so this contract-side pass skips it.
//! - [`conformance`] — **`conforms-to`**: each selected filler is validated against
//!   the requirement's resolved contract.
//! - [`admissibility`] — **the contract is itself checked** (`specs/10-contracts.md`,
//!   "Decision: the contract is itself checked — admissibility"): before the roster
//!   is trusted to judge a harness, each requirement's own definition is held to the
//!   definition — its `match` selector resolves, a `required` typed requirement's kind
//!   is satisfiable, its contract resolves and is itself admissible, and any
//!   `verified_by` resolves to a real path.
//!
//! A non-`required` requirement never fires a *selection* gate (`temper` never
//! fabricates a gate the author did not declare, `00-intent.md` law 4), but every
//! requirement's definition is held to admissibility regardless.
//!
//! ## The two decidable selectors
//!
//! Matching ranges over the closed selector set the parse foundation already
//! captured:
//!
//! - [`MatchSelector::Name`] — a minimal in-crate `*` glob over the artifact's
//!   name ([`Features::id`]). `*` matches any run of characters (including empty);
//!   every other character is literal. No glob crate joins the sanctioned set for
//!   this one wildcard.
//! - [`MatchSelector::Role`] — the artifact declares the marker it carries in a
//!   `role:` frontmatter field, read off [`Features`] like any other field — the
//!   marker-opt-in selector, the contract-side alternative to a name glob.
//!
//! Candidates come from the loaded kinds (`skill`, `rule`): a requirement's `kind`
//! typing narrows them to that kind, and a kind-blind requirement (no `kind`) draws
//! from every modeled kind. A `required` requirement over a kind the surface does not
//! model finds zero candidates and fails — honest: an unfilled required requirement is
//! a true violation, not a reason to stay silent.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use crate::check::Diagnostic;
use crate::compose::{CountBound, MatchSelector, Membership, Requirement};
use crate::engine;
use crate::extract::{FeatureValue, Features};

/// The diagnostic `rule` id every match-selection finding reports under — the
/// requirement's contract-side selection "clause", the harness-contract analogue of
/// the artifact-clause keys [`crate::engine`] emits.
const REQUIREMENT_MATCH_RULE: &str = "requirement.match";

/// The diagnostic `rule` id every conformance finding reports under — the
/// `conforms-to` clause of a requirement (`specs/10-contracts.md`: a filler is
/// `present`, `conforms-to` contract C, and is selected by `match`).
const REQUIREMENT_CONFORMS_TO_RULE: &str = "requirement.conforms-to";

/// The diagnostic `rule` id every roster-admissibility finding reports under — the
/// admissibility clause of a requirement (`specs/10-contracts.md`, "Decision: the
/// contract is itself checked — admissibility"): the requirement's own definition is
/// well-formed against the definition, before the roster is trusted to judge a harness.
const REQUIREMENT_ADMISSIBILITY_RULE: &str = "requirement.admissibility";

/// The diagnostic `rule` id every set-scope uniqueness finding reports under — the
/// `unique` predicate of the roster scope (`specs/45-governance.md`, "The set scope
/// (the roster)"): a declared field must not repeat across a requirement's matched set.
const REQUIREMENT_UNIQUE_RULE: &str = "requirement.unique";

/// The diagnostic `rule` id every set-scope membership finding reports under — the
/// `membership` predicate of the roster scope (`specs/45-governance.md`, "The set
/// scope (the roster)"): a declared field of every artifact matching a requirement's
/// selector (S₁) must lie in the feature-set drawn from a second matched set (S₂).
const REQUIREMENT_MEMBERSHIP_RULE: &str = "requirement.membership";

/// The candidate artifacts a requirement's `match` selector ranges over: its `kind`
/// typing narrows them to that kind's workspace [`Features`], and a kind-blind
/// requirement (no `kind`) draws from every modeled kind. A `kind` the surface does
/// not model yields no candidates, so a `required` requirement over it fails honestly.
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

/// The label for a requirement's `kind` in a diagnostic — the declared kind, or
/// `any` when the requirement is kind-blind (its filler may be of any kind).
fn kind_label(requirement: &Requirement) -> &str {
    requirement.kind.as_deref().unwrap_or("any")
}

/// Run requirement match-selection over the parsed roster, returning an
/// error-severity [`Diagnostic`] per `required` single-filler requirement that its
/// `match` selector fills by zero or by many artifacts.
///
/// `by_kind` maps an artifact kind (`skill`, `rule`, …) to the workspace
/// [`Features`] of that kind; candidates come from the requirement's `kind` (or every
/// modeled kind when kind-blind), so a `required` requirement over an unmodeled kind
/// finds zero and fails honestly. Requirements iterate in name order (the roster is a
/// [`BTreeMap`]) and each kind's candidates arrive name-sorted, so the finding set is
/// stable across runs.
///
/// Contract-side selection only: a requirement that declares **no `match` selector**
/// is filled by opt-in `satisfies` alone, which [`crate::coverage`] gates — so this
/// pass skips it, leaving no double-gate. `conforms-to` the requirement's contract and
/// `verified_by` admissibility are separate passes; a non-`required` requirement never
/// fires the single-filler gate here.
///
/// `base_dir` is the `temper.toml` directory a `membership` `conforms_to` template
/// path resolves against — the typed-reference constraint (`specs/45-governance.md`,
/// "The set scope") needs it to load the contract that narrows the source set.
#[must_use]
pub fn check(
    requirements: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&str, &[Features]>,
    base_dir: &Path,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for requirement in requirements.values() {
        // No contract-side `match` selector ⇒ fill is by opt-in `satisfies` alone,
        // which `crate::coverage` gates. The roster scope has nothing to select over,
        // so it skips the requirement rather than double-gate coverage's domain.
        let Some(selector) = &requirement.selector else {
            continue;
        };
        let candidates = candidates_for(requirement, by_kind);
        let fillers = fillers(selector, &candidates);
        match &requirement.count {
            // A declared `count` bound quantifies over the matched set: the
            // cardinality must land in `[min, max]`, generalizing the single-filler
            // zero/one/many arms below (`specs/45-governance.md`, "The set scope").
            // `count` is itself an author-declared gate, so it fires regardless of
            // `required` (which it is mutually exclusive with).
            Some(bound) => {
                if !(bound.min..=bound.max).contains(&fillers.len()) {
                    diagnostics.push(out_of_band(requirement, bound, &fillers));
                }
            }
            // No `count`: the single-filler form. `temper` never fabricates a gate
            // the author did not declare, so a non-`required` requirement's filler
            // count is not a violation; a `required` one needs exactly one filler.
            None => {
                if requirement.required {
                    match fillers.as_slice() {
                        [_] => {}
                        [] => diagnostics.push(unfilled(requirement)),
                        many => diagnostics.push(overfilled(requirement, many)),
                    }
                }
            }
        }

        // The set-scope `unique` predicate (`specs/45-governance.md`, "The set
        // scope"): each declared field must not repeat across the requirement's
        // matched set. Author-declared and orthogonal to the cardinality bound above,
        // so it fires regardless of `count`/`required`.
        for field in &requirement.unique {
            diagnostics.extend(duplicates(requirement, selector, field, &candidates));
        }

        // The set-scope `membership` predicate (`specs/45-governance.md`, "The set
        // scope"): each S₁ filler's declared field must lie in the corpus-derived
        // set drawn from the second selector's matched artifacts (S₂). The S₂ kind
        // may differ from the requirement's own, so the allowed set is built off the
        // full `by_kind` map, not the `candidates`. Author-declared and orthogonal to
        // `count`/`unique`/`required`, so it fires regardless of them.
        if let Some(membership) = &requirement.membership {
            diagnostics.extend(out_of_set(
                requirement,
                selector,
                membership,
                &candidates,
                by_kind,
                base_dir,
            ));
        }
    }
    diagnostics
}

/// Run the `conforms-to` half of a requirement over the parsed roster
/// (`specs/10-contracts.md`, the `contract` typing facet): for each requirement that
/// declares *both* a `match` selector and a `contract`, validate the artifact(s) its
/// selector picks against the resolved contract, retagging every conformance finding
/// under [`REQUIREMENT_CONFORMS_TO_RULE`] and naming the requirement the filler broke.
///
/// A requirement with no `contract` imposes no shape (skipped), and one with no
/// `match` selector picks no contract-side fillers (skipped). `base_dir` is the
/// `temper.toml` directory a template-path contract resolves against; `by_kind` is the
/// same workspace-features map [`check`] reads. A requirement whose contract does not
/// resolve — a missing or malformed template — is skipped here rather than reported: a
/// non-resolving template is admissibility's finding, and double-reporting would be
/// noise.
///
/// Conformance and selection decide *independently*: this pass validates whichever
/// artifacts match (zero, one, or many), so a filler that violates the contract is
/// reported even when the same requirement also trips its single-filler gate in
/// [`check`].
#[must_use]
pub fn conformance(
    requirements: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&str, &[Features]>,
    base_dir: &Path,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for requirement in requirements.values() {
        // A requirement with no contract imposes no shape, and one with no selector
        // picks no contract-side fillers — either way there is nothing to conform.
        let (Some(contract_ref), Some(selector)) = (&requirement.contract, &requirement.selector)
        else {
            continue;
        };
        // A non-resolving/malformed template is admissibility's to report, not
        // ours — skip the conformance check rather than double-report it.
        let Ok(contract) = contract_ref.resolve(base_dir, &requirement.name) else {
            continue;
        };
        let candidates = candidates_for(requirement, by_kind);
        let fillers: Vec<Features> = candidates
            .iter()
            .filter(|features| matches(selector, features))
            .map(|features| (*features).clone())
            .collect();
        for finding in engine::validate(&contract, &fillers) {
            diagnostics.push(conformance_finding(requirement, &finding));
        }
    }
    diagnostics
}

/// Validate the harness roster against **the definition** — admissibility
/// (`specs/10-contracts.md`, "Decision: the contract is itself checked —
/// admissibility"). Each requirement earns trust the way a harness does, by passing a
/// check, *before* the roster is used to judge anything; every finding is
/// [`Diagnostic::error`] (an inadmissible requirement cannot be trusted, so it must
/// fail the run) and names the requirement it indicts.
///
/// Six decidable clauses over the requirement's *present* facets — every facet is
/// optional, so an absent one imposes no admissibility check:
///
/// - **(a) the `match` selector resolves** — when present, a [`MatchSelector::Role`]
///   marker is non-empty (an empty marker no artifact can declare admits nothing). A
///   [`MatchSelector::Name`] glob is always well-formed under the in-crate matcher
///   ([`glob_matches`] accepts any pattern), so it never fails here.
/// - **(b) a `required` typed requirement is satisfiable** — when the requirement
///   declares a `kind`, that kind is one `temper` models (a key of `by_kind`); a
///   required requirement typed to an unmodeled kind can *never* be filled, so its
///   definition is inadmissible. A kind-blind requirement (no `kind`) is filled by
///   opt-in `satisfies`, so this clause gates on both `required` *and* a declared kind.
/// - **(c) its contract resolves and is itself admissible** — when present,
///   [`RequirementContract::resolve`](crate::compose::RequirementContract::resolve)
///   succeeds (a non-resolving template path is the inadmissibility this pass owns,
///   the case [`conformance`] skips) and the resolved [`Contract`](crate::contract::Contract)
///   passes [`engine::admissibility`] (so an empty `enum` in an inline contract is
///   caught here, exactly as in a floor contract).
/// - **(d) any `verified_by` resolves** — the named path exists relative to
///   `base_dir` (the referential clause; a dangling verifier is a silent no-op,
///   the very failure `00-intent.md` law 1 forbids).
/// - **(e) a declared `count` bound is satisfiable** — `min <= max`; an inverted
///   bound admits no cardinality at all, so the definition is inadmissible, mirroring
///   `range`'s `min > max` rejection (`specs/45-governance.md`).
/// - **(f) a `membership` `conforms_to` typed reference resolves and is itself
///   admissible** — held to the same bar as the requirement's own contract in (c).
///
/// `by_kind` is the same workspace-features map [`check`] and [`conformance`] read
/// — admissibility uses only its *keys* (the modeled kinds), never the fillers.
/// `base_dir` is the `temper.toml` directory a template path or `verified_by` path
/// resolves against.
#[must_use]
pub fn admissibility(
    requirements: &BTreeMap<String, Requirement>,
    by_kind: &BTreeMap<&str, &[Features]>,
    base_dir: &Path,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for requirement in requirements.values() {
        let name = requirement.name.as_str();

        // (a) The `match` selector resolves. Only a `role` marker can be vacuous:
        // an empty marker is one no artifact can opt into.
        if let Some(MatchSelector::Role { marker }) = &requirement.selector
            && marker.is_empty()
        {
            diagnostics.push(Diagnostic::error(
                REQUIREMENT_ADMISSIBILITY_RULE,
                name,
                format!(
                    "requirement `{name}` selects by an empty `role` marker, which no artifact can declare"
                ),
            ));
        }

        // (b) A `required` typed requirement is satisfiable: its declared `kind` is
        // one `temper` models, else no artifact of that kind can ever fill it. A
        // kind-blind requirement is filled by opt-in `satisfies`, so this gates on a
        // declared kind.
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

        // (c) The requirement's contract resolves, and the resolved contract is
        // itself admissible. A non-resolving template is this pass's finding (the case
        // `conformance` skips to avoid double-reporting).
        if let Some(contract_ref) = &requirement.contract {
            match contract_ref.resolve(base_dir, name) {
                Err(error) => diagnostics.push(Diagnostic::error(
                    REQUIREMENT_ADMISSIBILITY_RULE,
                    name,
                    format!("requirement `{name}` contract does not resolve: {error}"),
                )),
                Ok(contract) => {
                    for finding in engine::admissibility(&contract) {
                        diagnostics.push(Diagnostic::error(
                            REQUIREMENT_ADMISSIBILITY_RULE,
                            name,
                            format!(
                                "requirement `{name}` contract is inadmissible: {}",
                                finding.message
                            ),
                        ));
                    }
                }
            }
        }

        // (d) Any `verified_by` resolves to a real path under the project — a
        // dangling verifier is a silent no-op.
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

        // (e) A declared `count` bound is satisfiable: `min <= max`. An inverted
        // bound admits no cardinality at all — a vacuous clause the author cannot
        // have meant — so the definition is inadmissible, mirroring `range`'s
        // `min > max` rejection (`specs/45-governance.md`, "reject min>max").
        if let Some(bound) = &requirement.count
            && bound.min > bound.max
        {
            diagnostics.push(Diagnostic::error(
                REQUIREMENT_ADMISSIBILITY_RULE,
                name,
                format!(
                    "requirement `{name}` declares an inverted count bound (min {} greater than max {}), which no matched set can satisfy",
                    bound.min, bound.max
                ),
            ));
        }

        // (f) A `membership` `conforms_to` typed reference resolves and is itself
        // admissible — held to the same bar as the requirement's own contract in (c).
        // A `conforms_to` that never loads would otherwise silently drop the whole
        // membership check (`out_of_set` skips a non-resolving one), so it must be
        // reported here.
        if let Some(source_contract) = requirement
            .membership
            .as_ref()
            .and_then(|m| m.source_contract.as_ref())
        {
            match source_contract.resolve(base_dir, name) {
                Err(error) => diagnostics.push(Diagnostic::error(
                    REQUIREMENT_ADMISSIBILITY_RULE,
                    name,
                    format!(
                        "requirement `{name}` `membership` `conforms_to` does not resolve: {error}"
                    ),
                )),
                Ok(contract) => {
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
/// filler artifact and the clause's declared severity carry over unchanged, the
/// `rule` becomes [`REQUIREMENT_CONFORMS_TO_RULE`], and the message names the
/// requirement whose contract the filler broke so a reader knows which one indicted it.
fn conformance_finding(requirement: &Requirement, finding: &Diagnostic) -> Diagnostic {
    Diagnostic::new(
        finding.severity,
        REQUIREMENT_CONFORMS_TO_RULE,
        finding.artifact.as_str(),
        format!(
            "filler `{}` does not conform to requirement `{}`: {}",
            finding.artifact, requirement.name, finding.message
        ),
    )
}

/// The names of the artifacts that fill `selector`, in candidate order. A candidate
/// fills the selector when its name matches the [`MatchSelector::Name`] glob or it
/// declares the [`MatchSelector::Role`] marker in its `role` field.
fn fillers<'a>(selector: &MatchSelector, candidates: &[&'a Features]) -> Vec<&'a str> {
    candidates
        .iter()
        .filter(|features| matches(selector, features))
        .map(|features| features.id.as_str())
        .collect()
}

/// Whether one artifact's [`Features`] fills the selector — the decidable match at
/// the heart of selection. `pub(crate)` so the graph-scope `degree` check
/// ([`crate::graph`]) selects a requirement's matched nodes by the *same* decidable
/// selector this roster scope uses, never a second matcher that could disagree.
pub(crate) fn matches(selector: &MatchSelector, features: &Features) -> bool {
    match selector {
        MatchSelector::Name { glob } => glob_matches(glob, &features.id),
        // The marker-opt-in selector: the artifact declares the marker in its `role:`
        // frontmatter field, read off `Features` like any other scalar field.
        MatchSelector::Role { marker } => {
            features.field("role").and_then(FeatureValue::as_scalar) == Some(marker.as_str())
        }
    }
}

/// Whether `glob` matches `name`, treating `*` as "any run of characters
/// (including empty)" and every other character literally — the minimal in-crate
/// wildcard the `name` selector needs, short of pulling in a glob crate for one
/// metacharacter. A standard linear matcher with single-star backtracking: on a
/// mismatch it falls back to the most recent `*`, extending what that star
/// consumed by one character.
fn glob_matches(glob: &str, name: &str) -> bool {
    let pattern: Vec<char> = glob.chars().collect();
    let text: Vec<char> = name.chars().collect();
    let mut pi = 0;
    let mut ti = 0;
    // The position of the last `*` in `pattern`, and how much of `text` it had
    // consumed when we matched it — the backtrack point.
    let mut star: Option<usize> = None;
    let mut star_ti = 0;
    while ti < text.len() {
        if pi < pattern.len() && (pattern[pi] == text[ti]) {
            pi += 1;
            ti += 1;
        } else if pi < pattern.len() && pattern[pi] == '*' {
            star = Some(pi);
            star_ti = ti;
            pi += 1;
        } else if let Some(star_pi) = star {
            // Mismatch under an open `*`: let the star swallow one more character.
            pi = star_pi + 1;
            star_ti += 1;
            ti = star_ti;
        } else {
            return false;
        }
    }
    // Trailing `*`s match the empty remainder.
    while pi < pattern.len() && pattern[pi] == '*' {
        pi += 1;
    }
    pi == pattern.len()
}

/// The finding for a `required` single-filler requirement no artifact fills — naming
/// the requirement, the kind it expected, and that a single-filler requirement needs
/// exactly one.
fn unfilled(requirement: &Requirement) -> Diagnostic {
    Diagnostic::error(
        REQUIREMENT_MATCH_RULE,
        &requirement.name,
        format!(
            "required requirement `{}` is filled by no `{}` artifact (a single-filler requirement needs exactly one)",
            requirement.name,
            kind_label(requirement)
        ),
    )
}

/// The finding for a `required` single-filler requirement that many artifacts fill —
/// naming the requirement, the count, the kind, and the colliding fillers.
fn overfilled(requirement: &Requirement, fillers: &[&str]) -> Diagnostic {
    Diagnostic::error(
        REQUIREMENT_MATCH_RULE,
        &requirement.name,
        format!(
            "required requirement `{}` is filled by {} `{}` artifacts ({}); a single-filler requirement needs exactly one",
            requirement.name,
            fillers.len(),
            kind_label(requirement),
            fillers.join(", ")
        ),
    )
}

/// The finding for a requirement whose matched-set cardinality falls outside its
/// declared `count` bound — naming the requirement, the count, the kind, the colliding
/// fillers, and the `[min, max]` bound it missed (`specs/45-governance.md`, "The set
/// scope").
fn out_of_band(requirement: &Requirement, bound: &CountBound, fillers: &[&str]) -> Diagnostic {
    let listed = if fillers.is_empty() {
        String::new()
    } else {
        format!(" ({})", fillers.join(", "))
    };
    Diagnostic::error(
        REQUIREMENT_MATCH_RULE,
        &requirement.name,
        format!(
            "requirement `{}` is filled by {} `{}` artifact(s){listed}, outside its declared count bound [{}, {}]",
            requirement.name,
            fillers.len(),
            kind_label(requirement),
            bound.min,
            bound.max
        ),
    )
}

/// The set-scope `unique` findings for one declared `field` over a requirement's
/// matched set (`specs/45-governance.md`, "The set scope"): group the matched fillers
/// by the field's extracted scalar value and emit one error per value two or more
/// fillers share. A filler missing the field carries no value to collide on, so it
/// is silently skipped — a missing field is no collision. Values are grouped in a
/// [`BTreeMap`] so the finding set is stable across runs.
fn duplicates(
    requirement: &Requirement,
    selector: &MatchSelector,
    field: &str,
    candidates: &[&Features],
) -> Vec<Diagnostic> {
    let mut by_value: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for features in candidates
        .iter()
        .filter(|features| matches(selector, features))
    {
        if let Some(value) = features.field(field).and_then(FeatureValue::as_scalar) {
            by_value
                .entry(value)
                .or_default()
                .push(features.id.as_str());
        }
    }
    by_value
        .into_iter()
        .filter(|(_, fillers)| fillers.len() > 1)
        .map(|(value, fillers)| duplicate(requirement, field, value, &fillers))
        .collect()
}

/// The finding for a `unique` field two or more matched fillers share — naming the
/// requirement, the field, the shared value, and the colliding fillers.
fn duplicate(requirement: &Requirement, field: &str, value: &str, fillers: &[&str]) -> Diagnostic {
    Diagnostic::error(
        REQUIREMENT_UNIQUE_RULE,
        &requirement.name,
        format!(
            "requirement `{}` requires `{field}` unique across its matched set, but {} fillers share `{field}` = `{value}` ({})",
            requirement.name,
            fillers.len(),
            fillers.join(", ")
        ),
    )
}

/// The set-scope `membership` findings for one requirement over its matched set
/// (`specs/45-governance.md`, "The set scope"): build the allowed set from the
/// source feature `G` extracted over the `source_kind` artifacts the second
/// selector (S₂) matches, then emit one error per S₁ filler whose declared field-`F`
/// scalar is absent from that set. A filler missing `F` carries no value to check,
/// so it is silently skipped — a missing field is no violation, the way a missing
/// `unique` field is no collision. The allowed set is corpus-*derived*, so an S₂
/// matching no artifacts (or whose matches all lack `G`) yields the empty set, under
/// which every valued filler is genuinely a non-member.
///
/// `by_kind` is the full workspace map — S₂'s `source_kind` may differ from the
/// requirement's own `kind`, so the source candidates come from the map, not the S₁
/// `candidates`. Findings follow `candidates` order, which is name-sorted, so the
/// set is stable across runs.
///
/// When the membership carries a `conforms_to` **typed-reference** constraint
/// (`specs/45-governance.md`, "The set scope"), S₂ is first narrowed to the matching
/// sources that *also* conform to that contract — resolved against `base_dir` and
/// validated by [`engine::validate`], the same machinery [`conformance`] runs — so
/// the allowed set is drawn only from the right *kind* of thing. A source that trips
/// any finding is dropped before the set is built; a non-resolving `conforms_to` is
/// admissibility's to report (like [`conformance`]'s skip), so the membership check
/// is skipped rather than run against an unconstrained source.
fn out_of_set(
    requirement: &Requirement,
    selector: &MatchSelector,
    membership: &Membership,
    candidates: &[&Features],
    by_kind: &BTreeMap<&str, &[Features]>,
    base_dir: &Path,
) -> Vec<Diagnostic> {
    let source = by_kind
        .get(membership.source_kind.as_str())
        .copied()
        .unwrap_or(&[]);
    let mut matched: Vec<&Features> = source
        .iter()
        .filter(|features| matches(&membership.source_selector, features))
        .collect();

    // A typed reference constrains S₂ to sources conforming to contract C: keep only
    // the matched sources that validate clean against it, reusing `conformance`'s
    // resolve + validate machinery. A non-resolving `conforms_to` is admissibility's
    // finding, not ours — skip the check rather than draw a set off an unconstrained
    // source (mirroring how `conformance` skips a requirement whose contract does not
    // resolve).
    if let Some(source_contract) = &membership.source_contract {
        let Ok(contract) = source_contract.resolve(base_dir, &requirement.name) else {
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

    candidates
        .iter()
        .filter(|features| matches(selector, features))
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

/// The finding for an S₁ filler whose declared field falls outside the S₂-derived
/// set — naming the requirement, the constrained field, the source feature and kind
/// the allowed set is drawn from, the offending filler, and the value that is not a
/// member.
fn not_member(
    requirement: &Requirement,
    membership: &Membership,
    filler: &str,
    value: &str,
) -> Diagnostic {
    Diagnostic::error(
        REQUIREMENT_MEMBERSHIP_RULE,
        &requirement.name,
        format!(
            "requirement `{}` requires `{}` of each filler drawn from the `{}` feature of matching `{}` artifacts, but filler `{}` declares `{}` = `{}`, which is not in that set",
            requirement.name,
            membership.field,
            membership.source_feature,
            membership.source_kind,
            filler,
            membership.field,
            value
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use crate::check::Severity;
    use crate::compose::{AuthorLayer, Requirement};
    use crate::extract::Kind;
    use std::path::Path;

    /// A `Features` carrying a name (its `id`) and an optional `role:` marker —
    /// the two facts the selectors decide over.
    fn features(name: &str, role_marker: Option<&str>) -> Features {
        let mut fields = BTreeMap::new();
        if let Some(marker) = role_marker {
            fields.insert(
                "role".to_string(),
                FeatureValue::scalar(Kind::String, marker),
            );
        }
        Features {
            id: name.to_string(),
            fields,
            body_lines: 1,
            headings: Vec::new(),
            source_dir: Some(name.to_string()),
            companions: Vec::new(),
            satisfies: Vec::new(),
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

    /// A required, name-glob single-filler requirement over the `skill` kind.
    fn required_name_requirement(glob: &str) -> Requirement {
        requirement(
            &format!(
                "[requirement.planner]\n\
                 kind = \"skill\"\n\
                 contract = \"contracts/skill.anthropic.toml\"\n\
                 match = {{ name = \"{glob}\" }}\n\
                 required = true\n"
            ),
            "planner",
        )
    }

    /// Pack a roster of one requirement and a skill candidate set into the shapes
    /// [`check`] takes.
    fn run(req: Requirement, skills: &[Features]) -> Vec<Diagnostic> {
        let mut requirements = BTreeMap::new();
        requirements.insert(req.name.clone(), req);
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", skills)]);
        check(&requirements, &by_kind, Path::new(""))
    }

    /// The `fillers` helper takes `&[&Features]`; a candidate slice is borrowed into
    /// that shape for the two tests that drive selection directly.
    fn refs(features: &[Features]) -> Vec<&Features> {
        features.iter().collect()
    }

    #[test]
    fn glob_matches_the_star_cases() {
        // A bare `*` matches anything, including the empty string.
        assert!(glob_matches("*", "anything"));
        assert!(glob_matches("*", ""));
        // A leading/trailing star anchors the literal remainder.
        assert!(glob_matches("plan*", "plan"));
        assert!(glob_matches("plan*", "plan-tasks"));
        assert!(!glob_matches("plan*", "preplan"));
        assert!(glob_matches("*lint", "run-lint"));
        assert!(!glob_matches("*lint", "lint-run"));
        // An interior star, and an exact (star-free) pattern.
        assert!(glob_matches("a*z", "abcz"));
        assert!(glob_matches("a*z", "az"));
        assert!(!glob_matches("a*z", "abc"));
        assert!(glob_matches("exact", "exact"));
        assert!(!glob_matches("exact", "exactly"));
    }

    #[test]
    fn a_name_glob_picks_exactly_the_matching_fillers() {
        let req = required_name_requirement("plan*");
        let skills = [
            features("plan-tasks", None),
            features("lint-rust", None),
            features("plan-sprints", None),
        ];
        let selected = fillers(req.selector.as_ref().unwrap(), &refs(&skills));
        assert_eq!(selected, vec!["plan-tasks", "plan-sprints"]);
    }

    #[test]
    fn a_role_marker_picks_the_opting_in_artifact() {
        // The `role` marker selector matches the artifact's declared `role:` field,
        // not its name — the marker-opt-in form.
        let req = requirement(
            "[requirement.release]\n\
             kind = \"skill\"\n\
             contract = \"contracts/skill.anthropic.toml\"\n\
             match = { role = \"release\" }\n\
             required = true\n",
            "release",
        );
        let skills = [
            features("ship-it", Some("release")),
            features("plan-tasks", Some("planning")),
            features("no-marker", None),
        ];
        let selected = fillers(req.selector.as_ref().unwrap(), &refs(&skills));
        assert_eq!(selected, vec!["ship-it"]);
    }

    #[test]
    fn zero_one_and_many_map_to_error_clean_error_for_a_required_requirement() {
        let req = required_name_requirement("plan*");

        // Zero fillers ⇒ an error-severity finding.
        let none = run(req.clone(), &[features("lint-rust", None)]);
        assert_eq!(none.len(), 1);
        assert_eq!(none[0].severity, Severity::Error);
        assert_eq!(none[0].rule, REQUIREMENT_MATCH_RULE);
        assert_eq!(none[0].artifact, "planner");
        assert!(none[0].message.contains("no `skill` artifact"));

        // Exactly one filler ⇒ clean.
        let one = run(req.clone(), &[features("plan-tasks", None)]);
        assert!(one.is_empty());

        // Many fillers ⇒ an error naming the count and the colliding fillers.
        let many = run(
            req,
            &[features("plan-tasks", None), features("plan-sprints", None)],
        );
        assert_eq!(many.len(), 1);
        assert_eq!(many[0].severity, Severity::Error);
        assert!(many[0].message.contains("plan-tasks"));
        assert!(many[0].message.contains("plan-sprints"));
    }

    #[test]
    fn a_non_required_requirement_never_fires_at_any_count() {
        // No `required` flag (absent ⇒ false): neither zero nor many fillers is a
        // violation in this tier.
        let req = requirement(
            "[requirement.planner]\n\
             kind = \"skill\"\n\
             contract = \"contracts/skill.anthropic.toml\"\n\
             match = { name = \"plan*\" }\n",
            "planner",
        );
        assert!(run(req.clone(), &[]).is_empty());
        assert!(
            run(
                req,
                &[features("plan-tasks", None), features("plan-sprints", None)],
            )
            .is_empty()
        );
    }

    #[test]
    fn a_requirement_with_no_selector_is_left_to_coverage() {
        // A `required` requirement with no `match` selector is filled by opt-in
        // `satisfies` alone — the roster scope skips it (coverage gates it), so no
        // single-filler finding fires even with no matching artifact.
        let req = requirement(
            "[requirement.dev-standards]\n\
             means = \"the harness maintains dev standards\"\n\
             required = true\n",
            "dev-standards",
        );
        assert!(run(req, &[features("lint-rust", None)]).is_empty());
    }

    /// A `count = { min, max }` band requirement over the `skill` kind — the set-scope
    /// predicate, mutually exclusive with `required`.
    fn count_band_requirement(min: usize, max: usize) -> Requirement {
        requirement(
            &format!(
                "[requirement.agents]\n\
                 kind = \"skill\"\n\
                 contract = \"contracts/skill.anthropic.toml\"\n\
                 match = {{ name = \"agent-*\" }}\n\
                 count = {{ min = {min}, max = {max} }}\n"
            ),
            "agents",
        )
    }

    #[test]
    fn a_count_band_is_clean_inside_and_fires_outside() {
        // A `[1, 2]` band: one or two matching skills are clean, zero or three fire.
        let req = count_band_requirement(1, 2);
        let agent = |n: u8| features(&format!("agent-{n}"), None);

        // In band (one filler, and two fillers) ⇒ clean.
        assert!(run(req.clone(), &[agent(1)]).is_empty());
        assert!(run(req.clone(), &[agent(1), agent(2)]).is_empty());

        // Below the band (zero fillers — the non-matching skill is ignored) ⇒ fires.
        let below = run(req.clone(), &[features("lint-rust", None)]);
        assert_eq!(below.len(), 1);
        assert_eq!(below[0].severity, Severity::Error);
        assert_eq!(below[0].rule, REQUIREMENT_MATCH_RULE);
        assert_eq!(below[0].artifact, "agents");
        assert!(below[0].message.contains("[1, 2]"));

        // Above the band (three fillers) ⇒ fires, naming the colliding fillers.
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
        // with one filler is out of band.
        let one = run(count_band_requirement(2, 4), &[features("agent-1", None)]);
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].rule, REQUIREMENT_MATCH_RULE);
    }

    /// A requirement declaring `unique = ["model"]` over the `skill` kind — the
    /// set-scope uniqueness predicate over the matched fillers' `model` field.
    fn unique_model_requirement() -> Requirement {
        requirement(
            "[requirement.agents]\n\
             kind = \"skill\"\n\
             contract = \"contracts/skill.anthropic.toml\"\n\
             match = { name = \"agent-*\" }\n\
             unique = [\"model\"]\n",
            "agents",
        )
    }

    /// A `Features` carrying a name and an optional `model:` scalar field — the
    /// field the `unique` predicate groups the matched set by.
    fn skill_with_model(name: &str, model: Option<&str>) -> Features {
        let mut f = features(name, None);
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

        // Two matched fillers sharing a `model` value ⇒ one error naming the field,
        // the shared value, and the colliding fillers.
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

        // Every matched filler's `model` differs ⇒ silent.
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
    fn a_unique_field_missing_from_the_fillers_is_no_collision() {
        // Neither matched filler declares `model` — no extracted value to share, so
        // a missing field is no collision.
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
    fn unique_groups_only_the_matched_fillers() {
        // The non-matching `lint-rust` shares `model` with nothing in the matched
        // set, and only `agent-*` fillers are grouped — so a lone matched filler is
        // silent even though an out-of-set artifact carries the same `model`.
        let req = unique_model_requirement();
        let diags = run(
            req,
            &[
                skill_with_model("agent-1", Some("opus")),
                skill_with_model("lint-rust", Some("opus")),
            ],
        );
        assert!(diags.is_empty());
    }

    /// A requirement declaring `membership` of its fillers' `model` field in the
    /// `model` feature drawn from `source_kind` artifacts matching `approved-*` — the
    /// set-scope membership predicate, with a corpus-derived allowed set.
    fn membership_requirement(source_kind: &str) -> Requirement {
        requirement(
            &format!(
                "[requirement.agents]\n\
                 kind = \"skill\"\n\
                 contract = \"contracts/skill.anthropic.toml\"\n\
                 match = {{ name = \"agent-*\" }}\n\
                 membership = {{ field = \"model\", kind = \"{source_kind}\", match = {{ name = \"approved-*\" }}, feature = \"model\" }}\n"
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
        check(&requirements, &by_kind, Path::new(""))
    }

    #[test]
    fn a_membership_fires_outside_the_derived_set_and_is_silent_inside() {
        // S₁ (`agent-*`) and S₂ (`approved-*`) are both skills here. The allowed set
        // is { opus, sonnet } (the `model` of the two approved skills); `agent-2`'s
        // `gpt` is outside it, `agent-1`'s `opus` is inside.
        let req = membership_requirement("skill");
        let skills = [
            skill_with_model("agent-1", Some("opus")),
            skill_with_model("agent-2", Some("gpt")),
            skill_with_model("approved-a", Some("opus")),
            skill_with_model("approved-b", Some("sonnet")),
        ];
        let diags = run(req.clone(), &skills);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, REQUIREMENT_MEMBERSHIP_RULE);
        assert_eq!(diags[0].artifact, "agents");
        assert!(diags[0].message.contains("agent-2"));
        assert!(diags[0].message.contains("gpt"));
        assert!(diags[0].message.contains("model"));

        // Every matched filler's `model` is drawn from the approved set ⇒ silent.
        let clean = [
            skill_with_model("agent-1", Some("opus")),
            skill_with_model("agent-2", Some("sonnet")),
            skill_with_model("approved-a", Some("opus")),
            skill_with_model("approved-b", Some("sonnet")),
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
            skill_with_model("approved-a", Some("opus")),
            skill_with_model("approved-b", Some("sonnet")),
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
    fn a_membership_filler_missing_the_field_is_skipped() {
        // `agent-2` declares no `model`, so it carries no value to check — a missing
        // field is no membership violation. `agent-1`'s `opus` is in the set, so the
        // run is clean.
        let req = membership_requirement("skill");
        let skills = [
            skill_with_model("agent-1", Some("opus")),
            skill_with_model("agent-2", None),
            skill_with_model("approved-a", Some("opus")),
        ];
        assert!(run(req, &skills).is_empty());
    }

    /// A requirement whose `membership` carries a `conforms_to` typed-reference
    /// constraint (inline clauses, so no `base_dir` is needed): the source set is
    /// narrowed to `approved-*` skills that also declare a `tier` field before the
    /// allowed `model` set is drawn.
    fn typed_reference_requirement() -> Requirement {
        requirement(
            "[requirement.agents]\n\
             kind = \"skill\"\n\
             contract = \"contracts/skill.anthropic.toml\"\n\
             match = { name = \"agent-*\" }\n\
             \n\
             [requirement.agents.membership]\n\
             field = \"model\"\n\
             kind = \"skill\"\n\
             feature = \"model\"\n\
             match = { name = \"approved-*\" }\n\
             \n\
             [[requirement.agents.membership.conforms_to.clause]]\n\
             severity = \"required\"\n\
             predicate = \"required\"\n\
             field = \"tier\"\n",
            "agents",
        )
    }

    /// A `skill_with_model` also carrying a `tier:` scalar — the field the
    /// typed-reference `conforms_to` contract requires of a conforming source.
    fn skill_with_model_and_tier(name: &str, model: Option<&str>, tier: Option<&str>) -> Features {
        let mut f = skill_with_model(name, model);
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
        // the allowed set. The `agent-gpt` filler's `gpt` is therefore a non-member.
        let req = typed_reference_requirement();
        let skills = [
            skill_with_model_and_tier("agent-gpt", Some("gpt"), None),
            skill_with_model_and_tier("approved-a", Some("opus"), Some("official")),
            skill_with_model_and_tier("approved-b", Some("gpt"), None),
        ];
        let mut requirements = BTreeMap::new();
        requirements.insert(req.name.clone(), req);
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        // Inline `conforms_to` clauses need no template, so an empty base dir suffices.
        let diags = check(&requirements, &by_kind, Path::new(""));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, REQUIREMENT_MEMBERSHIP_RULE);
        assert!(diags[0].message.contains("agent-gpt"));
        assert!(diags[0].message.contains("gpt"));

        // Give `approved-b` a `tier` so it conforms too: now `gpt` is in the derived
        // set and the same filler is silent — the constraint was the only thing
        // excluding it.
        let conforming = [
            skill_with_model_and_tier("agent-gpt", Some("gpt"), None),
            skill_with_model_and_tier("approved-a", Some("opus"), Some("official")),
            skill_with_model_and_tier("approved-b", Some("gpt"), Some("official")),
        ];
        let mut requirements = BTreeMap::new();
        requirements.insert(
            typed_reference_requirement().name.clone(),
            typed_reference_requirement(),
        );
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &conforming[..])]);
        assert!(check(&requirements, &by_kind, Path::new("")).is_empty());
    }

    #[test]
    fn a_membership_with_an_empty_source_set_flags_every_valued_filler() {
        // S₂ (`approved-*`) matches nothing, so the derived set is empty — every
        // matched filler that declares `model` is genuinely a non-member, a true
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

    /// A requirement carrying an inline `max_len` contract on `name`, capped at `max`.
    fn inline_maxlen_requirement(max: usize) -> Requirement {
        requirement(
            &format!(
                "[requirement.planner]\n\
                 kind = \"skill\"\n\
                 match = {{ name = \"plan*\" }}\n\
                 required = true\n\
                 [[requirement.planner.clause]]\n\
                 severity = \"required\"\n\
                 predicate = \"max_len\"\n\
                 field = \"name\"\n\
                 max = {max}\n"
            ),
            "planner",
        )
    }

    /// `features` with a `name` scalar field equal to its id — the field the
    /// inline `max_len` contract measures (the engine validates extracted *fields*,
    /// not the bare diagnostic id).
    fn named_skill(name: &str) -> Features {
        let mut f = features(name, None);
        f.fields
            .insert("name".to_string(), FeatureValue::scalar(Kind::String, name));
        f
    }

    /// Pack a roster of one requirement and skill candidates and run the conformance
    /// pass — the inline-contract path needs no `base_dir`, so an empty one suffices.
    fn run_conformance(req: Requirement, skills: &[Features]) -> Vec<Diagnostic> {
        let mut requirements = BTreeMap::new();
        requirements.insert(req.name.clone(), req);
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", skills)]);
        conformance(&requirements, &by_kind, Path::new(""))
    }

    #[test]
    fn an_inline_contract_validates_its_selected_filler_only() {
        // The inline contract caps `name` at 3 chars; the matching filler
        // `plan-tasks` (10) breaks it, while the non-matching `lint-rust` is never
        // validated against the requirement's contract.
        let diags = run_conformance(
            inline_maxlen_requirement(3),
            &[named_skill("plan-tasks"), named_skill("lint-rust")],
        );
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, REQUIREMENT_CONFORMS_TO_RULE);
        assert_eq!(diags[0].artifact, "plan-tasks");
        // The message names the requirement whose contract the filler broke.
        assert!(diags[0].message.contains("planner"));
        assert!(diags[0].message.contains("does not conform"));
    }

    #[test]
    fn an_inline_contract_is_silent_when_the_filler_conforms() {
        // The same shape, but a generous cap the filler stays within ⇒ clean.
        assert!(
            run_conformance(inline_maxlen_requirement(64), &[named_skill("plan-tasks")]).is_empty()
        );
    }

    #[test]
    fn conformance_and_selection_decide_independently() {
        // Two fillers — `check` would flag the single-filler overfill — and *both*
        // break the inline cap. Conformance reports each, regardless of the count.
        let diags = run_conformance(
            inline_maxlen_requirement(3),
            &[named_skill("plan-tasks"), named_skill("plan-sprints")],
        );
        assert_eq!(diags.len(), 2);
        assert!(diags.iter().all(|d| d.rule == REQUIREMENT_CONFORMS_TO_RULE));
    }

    #[test]
    fn a_requirement_whose_template_does_not_resolve_is_skipped_not_reported() {
        // The template path resolves to no file under this base dir: conformance
        // skips it (a non-resolving template is admissibility's finding), so
        // nothing fires even though a filler matches.
        let req = required_name_requirement("plan*"); // contract = a template path
        let skills = [features("plan-tasks", None)];
        let mut requirements = BTreeMap::new();
        requirements.insert(req.name.clone(), req);
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        // A base dir with no `contracts/` tree, so the template cannot load.
        let diags = conformance(
            &requirements,
            &by_kind,
            Path::new("/no-such-temper-base-dir"),
        );
        assert!(diags.is_empty());
    }

    #[test]
    fn a_required_requirement_over_an_unmodeled_kind_finds_zero_and_fails() {
        // The requirement's `kind` is `command`, a kind the `by_kind` map does not
        // carry — zero candidates, so the required requirement fails honestly.
        let req = requirement(
            "[requirement.releaser]\n\
             kind = \"command\"\n\
             contract = \"contracts/skill.anthropic.toml\"\n\
             match = { name = \"release*\" }\n\
             required = true\n",
            "releaser",
        );
        let mut requirements = BTreeMap::new();
        requirements.insert(req.name.clone(), req);
        // Only `skill` candidates are present; `command` is absent.
        let skills = [features("release-it", None)];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        let diags = check(&requirements, &by_kind, Path::new(""));
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("no `command` artifact"));
    }

    // ---- admissibility ----------------------------------------------------

    /// Run the admissibility pass over a one-requirement roster against a `skill`-only
    /// `by_kind` (the modeled kinds are its keys; admissibility reads no fillers).
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
        // so a required requirement over it can never be filled — inadmissible. The
        // inline contract resolves, so the only finding is the satisfiability one.
        let req = requirement(
            "[requirement.releaser]\n\
             kind = \"command\"\n\
             match = { name = \"release*\" }\n\
             required = true\n\
             [[requirement.releaser.clause]]\n\
             severity = \"required\"\n\
             predicate = \"max_len\"\n\
             field = \"name\"\n\
             max = 64\n",
            "releaser",
        );
        let diags = run_admissibility(req, Path::new(""));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, REQUIREMENT_ADMISSIBILITY_RULE);
        assert_eq!(diags[0].artifact, "releaser");
        assert!(diags[0].message.contains("command"));
        assert!(diags[0].message.contains("never be filled"));
    }

    #[test]
    fn a_non_resolving_template_contract_is_inadmissible() {
        // The template path resolves to no file under this base dir — the
        // inadmissibility this pass owns (the case `conformance` skips).
        let req = required_name_requirement("plan*"); // contract = a template path
        let diags = run_admissibility(req, Path::new("/no-such-temper-base-dir"));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, REQUIREMENT_ADMISSIBILITY_RULE);
        assert_eq!(diags[0].artifact, "planner");
        assert!(diags[0].message.contains("does not resolve"));
    }

    #[test]
    fn an_inline_contract_with_an_empty_enum_is_inadmissible() {
        // `engine::admissibility` runs on the resolved contract, so a vacuous
        // `enum` clause is caught here exactly as in a floor contract.
        let req = requirement(
            "[requirement.planner]\n\
             kind = \"skill\"\n\
             match = { name = \"plan*\" }\n\
             required = true\n\
             [[requirement.planner.clause]]\n\
             severity = \"required\"\n\
             predicate = \"enum\"\n\
             field = \"status\"\n\
             values = []\n",
            "planner",
        );
        let diags = run_admissibility(req, Path::new(""));
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
             match = { name = \"plan*\" }\n\
             required = true\n\
             verified_by = \"tests/nope.rs\"\n\
             [[requirement.planner.clause]]\n\
             severity = \"required\"\n\
             predicate = \"max_len\"\n\
             field = \"name\"\n\
             max = 64\n",
            "planner",
        );
        let diags = run_admissibility(req, Path::new("/no-such-temper-base-dir"));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, REQUIREMENT_ADMISSIBILITY_RULE);
        assert!(diags[0].message.contains("verifier"));
        assert!(diags[0].message.contains("tests/nope.rs"));
    }

    #[test]
    fn an_empty_role_marker_selector_is_inadmissible() {
        // A `role` marker no artifact can declare (the empty string) admits
        // nothing, so the selector does not resolve.
        let req = requirement(
            "[requirement.planner]\n\
             kind = \"skill\"\n\
             match = { role = \"\" }\n\
             [[requirement.planner.clause]]\n\
             severity = \"required\"\n\
             predicate = \"max_len\"\n\
             field = \"name\"\n\
             max = 64\n",
            "planner",
        );
        let diags = run_admissibility(req, Path::new(""));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, REQUIREMENT_ADMISSIBILITY_RULE);
        assert!(diags[0].message.contains("empty"));
    }

    #[test]
    fn a_name_glob_selector_is_always_admissible() {
        // A `name` glob is well-formed under the in-crate matcher for any pattern —
        // even a bare `*` — so the selector clause never fires for it.
        let req = requirement(
            "[requirement.planner]\n\
             kind = \"skill\"\n\
             match = { name = \"*\" }\n\
             required = true\n\
             [[requirement.planner.clause]]\n\
             severity = \"required\"\n\
             predicate = \"max_len\"\n\
             field = \"name\"\n\
             max = 64\n",
            "planner",
        );
        assert!(run_admissibility(req, Path::new("")).is_empty());
    }

    #[test]
    fn a_fully_resolving_roster_is_admissible() {
        // A modeled kind, a well-formed name glob, an admissible inline contract,
        // and no verifier — nothing for admissibility to reject.
        let req = inline_maxlen_requirement(64);
        assert!(run_admissibility(req, Path::new("")).is_empty());
    }

    #[test]
    fn a_bare_requirement_is_admissible() {
        // A pure opt-in-coverage requirement (only `means` + `required`, no `kind`,
        // `match`, or `contract`) has no facet for admissibility to reject — coverage
        // gates its fill, not the roster.
        let req = requirement(
            "[requirement.dev-standards]\n\
             means = \"the harness maintains dev standards\"\n\
             required = true\n",
            "dev-standards",
        );
        assert!(run_admissibility(req, Path::new("")).is_empty());
    }

    #[test]
    fn an_inverted_count_bound_is_inadmissible() {
        // `min > max` admits no cardinality at all — a vacuous bound the author
        // cannot have meant, so the definition fails admissibility (mirroring
        // `range`'s `min > max` rejection).
        let req = requirement(
            "[requirement.agents]\n\
             kind = \"skill\"\n\
             match = { name = \"agent-*\" }\n\
             count = { min = 3, max = 1 }\n\
             [[requirement.agents.clause]]\n\
             severity = \"required\"\n\
             predicate = \"max_len\"\n\
             field = \"name\"\n\
             max = 64\n",
            "agents",
        );
        let diags = run_admissibility(req, Path::new(""));
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
             match = { name = \"agent-*\" }\n\
             count = { min = 1, max = 1 }\n\
             [[requirement.agents.clause]]\n\
             severity = \"required\"\n\
             predicate = \"max_len\"\n\
             field = \"name\"\n\
             max = 64\n",
            "agents",
        );
        assert!(run_admissibility(req, Path::new("")).is_empty());
    }

    #[test]
    fn a_membership_conforms_to_that_does_not_resolve_is_inadmissible() {
        // The `membership` typed reference names a template path that resolves to no
        // file — a `conforms_to` that never loads would silently drop the whole
        // membership check, so admissibility reports it (clause (f)), mirroring the
        // requirement's own contract-resolve clause.
        let req = requirement(
            "[requirement.agents]\n\
             kind = \"skill\"\n\
             match = { name = \"agent-*\" }\n\
             membership = { field = \"model\", kind = \"skill\", match = { name = \"approved-*\" }, feature = \"model\", conforms_to = \"contracts/nope.toml\" }\n\
             [[requirement.agents.clause]]\n\
             severity = \"required\"\n\
             predicate = \"max_len\"\n\
             field = \"name\"\n\
             max = 64\n",
            "agents",
        );
        let diags = run_admissibility(req, Path::new("/no-such-temper-base-dir"));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, REQUIREMENT_ADMISSIBILITY_RULE);
        assert_eq!(diags[0].artifact, "agents");
        assert!(diags[0].message.contains("conforms_to"));
        assert!(diags[0].message.contains("does not resolve"));
    }

    #[test]
    fn a_membership_conforms_to_with_an_empty_enum_is_inadmissible() {
        // An inline `conforms_to` contract carrying a vacuous `enum` is held to the
        // same admissibility bar as the requirement's own contract — caught by (f).
        let req = requirement(
            "[requirement.agents]\n\
             kind = \"skill\"\n\
             match = { name = \"agent-*\" }\n\
             [requirement.agents.membership]\n\
             field = \"model\"\n\
             kind = \"skill\"\n\
             feature = \"model\"\n\
             match = { name = \"approved-*\" }\n\
             [[requirement.agents.membership.conforms_to.clause]]\n\
             severity = \"required\"\n\
             predicate = \"enum\"\n\
             field = \"tier\"\n\
             values = []\n\
             [[requirement.agents.clause]]\n\
             severity = \"required\"\n\
             predicate = \"max_len\"\n\
             field = \"name\"\n\
             max = 64\n",
            "agents",
        );
        let diags = run_admissibility(req, Path::new(""));
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
             match = { name = \"release*\" }\n\
             [[requirement.releaser.clause]]\n\
             severity = \"required\"\n\
             predicate = \"max_len\"\n\
             field = \"name\"\n\
             max = 64\n",
            "releaser",
        );
        assert!(run_admissibility(req, Path::new("")).is_empty());
    }
}
