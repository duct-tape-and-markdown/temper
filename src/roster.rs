//! Roster checks — selection, conformance, and admissibility over the parsed
//! harness contract.
//!
//! Implements the role tier of `specs/10-contracts.md` ("Roles and matching"): a
//! harness contract binds an abstract **role** to whichever concrete artifact
//! fills it, and *which* artifact is itself a decidable selector, never a guess.
//! The roster — the `[role.<name>]` tables parsed onto
//! [`AuthorLayer`](crate::compose::AuthorLayer) by the shipped parse foundation —
//! reaches this module as typed [`Role`]s.
//!
//! ## Three passes over one roster
//!
//! Three decidable passes read the same parsed roster, each owning one clause of
//! the `role` primitive:
//!
//! - [`check`] — **selection**: each role's [`MatchSelector`] is evaluated against
//!   the workspace artifacts of the role's `artifact` kind, and a **`required`
//!   single-filler role filled by zero or by many artifacts is a conformance
//!   error**, reported precisely (`specs/10-contracts.md`: "When zero or many
//!   artifacts match a `required` single-filler role, that is a conformance
//!   error").
//! - [`conformance`] — **`conforms-to`**: each selected filler is validated against
//!   the role's resolved contract.
//! - [`admissibility`] — **the contract is itself checked** (`specs/10-contracts.md`,
//!   "Decision: the contract is itself checked — admissibility"): before the roster
//!   is trusted to judge a harness, each role's own definition is held to the
//!   definition — its `match` selector resolves, a `required` role's artifact kind
//!   is satisfiable, its contract resolves and is itself admissible, and any
//!   `verified_by` resolves to a real path.
//!
//! A non-`required` role never fires a *selection* gate (`temper` never fabricates
//! a gate the author did not declare, `00-intent.md` law 4), but every role's
//! definition is held to admissibility regardless.
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
//! - [`MatchSelector::Role`] — the artifact *opts in* by declaring the role it
//!   fills in a `role:` frontmatter field, read off [`Features`] like any other
//!   field. The spec's preferred form: the artifact declares its role rather than
//!   the contract reaching out to guess.
//!
//! Candidates come from the loaded kinds (`skill`, `rule`). A `required` role over
//! a kind the surface does not model finds zero candidates and fails — honest: an
//! unfilled required role is a true violation, not a reason to stay silent.

use std::collections::BTreeMap;
use std::path::Path;

use crate::check::Diagnostic;
use crate::compose::{CountBound, MatchSelector, Role};
use crate::engine;
use crate::extract::{FeatureValue, Features};

/// The diagnostic `rule` id every roster finding reports under — the role
/// match-selection "clause", the harness-contract analogue of the artifact-clause
/// keys [`crate::engine`] emits.
const ROLE_RULE: &str = "role";

/// The diagnostic `rule` id every role-conformance finding reports under — the
/// `conforms-to` clause of the `role` primitive (`specs/10-contracts.md`: a
/// filler is `present`, `conforms-to` contract C, and is selected by `match`).
const ROLE_CONFORMS_TO_RULE: &str = "role.conforms-to";

/// The diagnostic `rule` id every roster-admissibility finding reports under — the
/// admissibility clause of the `role` primitive (`specs/10-contracts.md`,
/// "Decision: the contract is itself checked — admissibility"): the role's own
/// definition is well-formed against the definition, before the roster is trusted
/// to judge a harness.
const ROLE_ADMISSIBILITY_RULE: &str = "role.admissibility";

/// Run role match-selection over the parsed roster, returning an error-severity
/// [`Diagnostic`] per `required` single-filler role that is filled by zero or by
/// many artifacts.
///
/// `by_kind` maps an artifact kind (`skill`, `rule`, …) to the workspace
/// [`Features`] of that kind; a role whose `artifact` names a kind absent from the
/// map finds zero candidates (an unmodeled kind), so a `required` role over it
/// fails honestly. Roles iterate in name order (the roster is a [`BTreeMap`]) and
/// each kind's candidates arrive name-sorted, so the finding set is stable across
/// runs.
///
/// Selection only (`specs/10-contracts.md`, "Roles and matching"): this decides
/// which artifacts fill a role and whether a `required` single-filler role is
/// satisfiably filled. `conforms-to` the role's contract and `verified_by`
/// admissibility are follow-on passes; a non-`required` role never fires here.
#[must_use]
pub fn check(
    roles: &BTreeMap<String, Role>,
    by_kind: &BTreeMap<&str, &[Features]>,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for role in roles.values() {
        let candidates = by_kind.get(role.artifact.as_str()).copied().unwrap_or(&[]);
        let fillers = fillers(&role.selector, candidates);
        match &role.count {
            // A declared `count` bound quantifies over the matched set: the
            // cardinality must land in `[min, max]`, generalizing the single-filler
            // zero/one/many arms below (`specs/45-governance.md`, "The set scope").
            // `count` is itself an author-declared gate, so it fires regardless of
            // `required` (which it is mutually exclusive with).
            Some(bound) => {
                if !(bound.min..=bound.max).contains(&fillers.len()) {
                    diagnostics.push(out_of_band(role, bound, &fillers));
                }
            }
            // No `count`: the single-filler form. `temper` never fabricates a gate
            // the author did not declare, so a non-`required` role's filler count is
            // not a violation; a `required` one needs exactly one filler.
            None => {
                if !role.required {
                    continue;
                }
                match fillers.as_slice() {
                    [_] => {}
                    [] => diagnostics.push(unfilled(role)),
                    many => diagnostics.push(overfilled(role, many)),
                }
            }
        }
    }
    diagnostics
}

/// Run the `conforms-to` half of the `role` primitive over the parsed roster
/// (`specs/10-contracts.md`, "Roles and matching"): for each role, validate the
/// artifact(s) its selector picks against the role's resolved contract, retagging
/// every conformance finding under [`ROLE_CONFORMS_TO_RULE`] and naming the role
/// the filler broke.
///
/// `base_dir` is the `temper.toml` directory a template-path contract resolves
/// against; `by_kind` is the same workspace-features map [`check`] reads. A role
/// whose contract does not resolve — a missing or malformed template — is skipped
/// here rather than reported: a non-resolving template is the roster-admissibility
/// follow-on entry's finding, and double-reporting it would be noise.
///
/// Conformance and selection decide *independently*: this pass validates whichever
/// artifacts match (zero, one, or many), so a filler that violates the contract is
/// reported even when the same role also trips its single-filler gate in [`check`].
/// A role over a kind absent from `by_kind` finds no fillers, so [`engine::validate`]
/// over the empty set is silent — nothing to conform.
#[must_use]
pub fn conformance(
    roles: &BTreeMap<String, Role>,
    by_kind: &BTreeMap<&str, &[Features]>,
    base_dir: &Path,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for role in roles.values() {
        // A non-resolving/malformed template is admissibility's to report, not
        // ours — skip the conformance check rather than double-report it.
        let Ok(contract) = role.contract.resolve(base_dir, &role.name) else {
            continue;
        };
        let candidates = by_kind.get(role.artifact.as_str()).copied().unwrap_or(&[]);
        let fillers: Vec<Features> = candidates
            .iter()
            .filter(|features| matches(&role.selector, features))
            .cloned()
            .collect();
        for finding in engine::validate(&contract, &fillers) {
            diagnostics.push(conformance_finding(role, &finding));
        }
    }
    diagnostics
}

/// Validate the harness roster against **the definition** — admissibility
/// (`specs/10-contracts.md`, "Decision: the contract is itself checked —
/// admissibility"). Each role earns trust the way a harness does, by passing a
/// check, *before* the roster is used to judge anything; every finding is
/// [`Diagnostic::error`] (an inadmissible role cannot be trusted, so it must fail
/// the run) and names the role it indicts.
///
/// Five decidable clauses, mirroring the spec's admissibility list for the role
/// primitive:
///
/// - **(a) the `match` selector resolves** — a [`MatchSelector::Role`] marker is
///   non-empty (an empty marker no artifact can declare admits nothing). A
///   [`MatchSelector::Name`] glob is always well-formed under the in-crate matcher
///   ([`glob_matches`] accepts any pattern), so it never fails here.
/// - **(b) a `required` single-filler role is satisfiable** — `role.artifact`
///   names a kind `temper` models (a key of `by_kind`); a required role over an
///   unmodeled kind can *never* be filled, so its definition is inadmissible
///   regardless of the surface. A non-`required` role over an unmodeled kind is
///   merely never filled, which the author may have meant — so this clause gates
///   on `required`.
/// - **(c) its contract resolves and is itself admissible** —
///   [`RoleContract::resolve`](crate::compose::RoleContract::resolve)
///   succeeds (a non-resolving template path is the inadmissibility this pass owns,
///   the case [`conformance`] skips) and the resolved [`Contract`](crate::contract::Contract)
///   passes [`engine::admissibility`] (so an empty `enum` in a role's inline
///   contract is caught here, exactly as in a floor contract).
/// - **(d) any `verified_by` resolves** — the named path exists relative to
///   `base_dir` (the referential clause; a dangling verifier is a silent no-op,
///   the very failure `00-intent.md` law 1 forbids).
/// - **(e) a declared `count` bound is satisfiable** — `min <= max`; an inverted
///   bound admits no cardinality at all, so the role's definition is inadmissible,
///   mirroring `range`'s `min > max` rejection (`specs/45-governance.md`).
///
/// `by_kind` is the same workspace-features map [`check`] and [`conformance`] read
/// — admissibility uses only its *keys* (the modeled kinds), never the fillers.
/// `base_dir` is the `temper.toml` directory a template path or `verified_by` path
/// resolves against.
#[must_use]
pub fn admissibility(
    roles: &BTreeMap<String, Role>,
    by_kind: &BTreeMap<&str, &[Features]>,
    base_dir: &Path,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for role in roles.values() {
        // (a) The `match` selector resolves. Only a `role` marker can be vacuous:
        // an empty marker is one no artifact can opt into.
        if let MatchSelector::Role { marker } = &role.selector
            && marker.is_empty()
        {
            diagnostics.push(Diagnostic::error(
                ROLE_ADMISSIBILITY_RULE,
                &role.name,
                format!(
                    "role `{}` selects by an empty `role` marker, which no artifact can declare",
                    role.name
                ),
            ));
        }

        // (b) A `required` single-filler role is satisfiable: its artifact kind is
        // one `temper` models, else no artifact of that kind can ever fill it.
        if role.required && !by_kind.contains_key(role.artifact.as_str()) {
            diagnostics.push(Diagnostic::error(
                ROLE_ADMISSIBILITY_RULE,
                &role.name,
                format!(
                    "required role `{}` names artifact kind `{}`, which `temper` does not model — it can never be filled",
                    role.name, role.artifact
                ),
            ));
        }

        // (c) The role's contract resolves, and the resolved contract is itself
        // admissible. A non-resolving template is this pass's finding (the case
        // `conformance` skips to avoid double-reporting).
        match role.contract.resolve(base_dir, &role.name) {
            Err(error) => diagnostics.push(Diagnostic::error(
                ROLE_ADMISSIBILITY_RULE,
                &role.name,
                format!("role `{}` contract does not resolve: {error}", role.name),
            )),
            Ok(contract) => {
                for finding in engine::admissibility(&contract) {
                    diagnostics.push(Diagnostic::error(
                        ROLE_ADMISSIBILITY_RULE,
                        &role.name,
                        format!(
                            "role `{}` contract is inadmissible: {}",
                            role.name, finding.message
                        ),
                    ));
                }
            }
        }

        // (d) Any `verified_by` resolves to a real path under the project — a
        // dangling verifier is a silent no-op.
        if let Some(verifier) = &role.verified_by
            && !base_dir.join(verifier).exists()
        {
            diagnostics.push(Diagnostic::error(
                ROLE_ADMISSIBILITY_RULE,
                &role.name,
                format!(
                    "role `{}` names verifier `{verifier}`, which does not resolve to a path under the project — a dangling verifier is a silent no-op",
                    role.name
                ),
            ));
        }

        // (e) A declared `count` bound is satisfiable: `min <= max`. An inverted
        // bound admits no cardinality at all — a vacuous clause the author cannot
        // have meant — so the role's definition is inadmissible, mirroring `range`'s
        // `min > max` rejection (`specs/45-governance.md`, "reject min>max").
        if let Some(bound) = &role.count
            && bound.min > bound.max
        {
            diagnostics.push(Diagnostic::error(
                ROLE_ADMISSIBILITY_RULE,
                &role.name,
                format!(
                    "role `{}` declares an inverted count bound (min {} greater than max {}), which no matched set can satisfy",
                    role.name, bound.min, bound.max
                ),
            ));
        }
    }
    diagnostics
}

/// Recast one [`engine::validate`] finding as a role-conformance finding: the
/// filler artifact and the clause's declared severity carry over unchanged, the
/// `rule` becomes [`ROLE_CONFORMS_TO_RULE`], and the message names the role whose
/// contract the filler broke so a reader knows which role indicted it.
fn conformance_finding(role: &Role, finding: &Diagnostic) -> Diagnostic {
    Diagnostic::new(
        finding.severity,
        ROLE_CONFORMS_TO_RULE,
        finding.artifact.as_str(),
        format!(
            "filler `{}` does not conform to role `{}`: {}",
            finding.artifact, role.name, finding.message
        ),
    )
}

/// The names of the artifacts that fill `role`'s selector, in candidate order. A
/// candidate fills the role when its name matches the [`MatchSelector::Name`] glob
/// or it declares the [`MatchSelector::Role`] marker in its `role` field.
fn fillers<'a>(selector: &MatchSelector, candidates: &'a [Features]) -> Vec<&'a str> {
    candidates
        .iter()
        .filter(|features| matches(selector, features))
        .map(|features| features.id.as_str())
        .collect()
}

/// Whether one artifact's [`Features`] fills the role's selector — the decidable
/// match at the heart of selection.
fn matches(selector: &MatchSelector, features: &Features) -> bool {
    match selector {
        MatchSelector::Name { glob } => glob_matches(glob, &features.id),
        // The artifact opts in by declaring the role in a `role:` frontmatter
        // field, read off `Features` like any other scalar field.
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

/// The finding for a `required` single-filler role no artifact fills — naming the
/// role, the kind it expected, and that a single-filler role needs exactly one.
fn unfilled(role: &Role) -> Diagnostic {
    Diagnostic::error(
        ROLE_RULE,
        &role.name,
        format!(
            "required role `{}` is filled by no `{}` artifact (a single-filler role needs exactly one)",
            role.name, role.artifact
        ),
    )
}

/// The finding for a `required` single-filler role that many artifacts fill —
/// naming the role, the count, the kind, and the colliding fillers.
fn overfilled(role: &Role, fillers: &[&str]) -> Diagnostic {
    Diagnostic::error(
        ROLE_RULE,
        &role.name,
        format!(
            "required role `{}` is filled by {} `{}` artifacts ({}); a single-filler role needs exactly one",
            role.name,
            fillers.len(),
            role.artifact,
            fillers.join(", ")
        ),
    )
}

/// The finding for a role whose matched-set cardinality falls outside its declared
/// `count` bound — naming the role, the count, the kind, the colliding fillers, and
/// the `[min, max]` bound it missed (`specs/45-governance.md`, "The set scope").
fn out_of_band(role: &Role, bound: &CountBound, fillers: &[&str]) -> Diagnostic {
    let listed = if fillers.is_empty() {
        String::new()
    } else {
        format!(" ({})", fillers.join(", "))
    };
    Diagnostic::error(
        ROLE_RULE,
        &role.name,
        format!(
            "role `{}` is filled by {} `{}` artifact(s){listed}, outside its declared count bound [{}, {}]",
            role.name,
            fillers.len(),
            role.artifact,
            bound.min,
            bound.max
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use crate::check::Severity;
    use crate::compose::{AuthorLayer, Role};
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
        }
    }

    /// Parse a single role out of a `temper.toml` fragment — the parse foundation
    /// is the only constructor for a [`Role`], so the unit tests drive it.
    fn role(toml: &str, name: &str) -> Role {
        AuthorLayer::parse(toml, Path::new("temper.toml"))
            .unwrap()
            .roles()
            .get(name)
            .expect("the fragment declares the role")
            .clone()
    }

    /// A required, name-glob single-filler role over the `skill` kind.
    fn required_name_role(glob: &str) -> Role {
        role(
            &format!(
                "[role.planner]\n\
                 artifact = \"skill\"\n\
                 contract = \"contracts/skill.anthropic.toml\"\n\
                 match = {{ name = \"{glob}\" }}\n\
                 required = true\n"
            ),
            "planner",
        )
    }

    /// Pack a roster of one role and a skill candidate set into the shapes
    /// [`check`] takes.
    fn run(role: Role, skills: &[Features]) -> Vec<Diagnostic> {
        let mut roles = BTreeMap::new();
        roles.insert(role.name.clone(), role);
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", skills)]);
        check(&roles, &by_kind)
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
        let role = required_name_role("plan*");
        let skills = [
            features("plan-tasks", None),
            features("lint-rust", None),
            features("plan-sprints", None),
        ];
        let fillers = fillers(&role.selector, &skills);
        assert_eq!(fillers, vec!["plan-tasks", "plan-sprints"]);
    }

    #[test]
    fn a_role_marker_picks_the_opting_in_artifact() {
        // The `role` selector matches the artifact's declared `role:` field, not
        // its name — the "artifact opts in" form.
        let role = role(
            "[role.release]\n\
             artifact = \"skill\"\n\
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
        let fillers = fillers(&role.selector, &skills);
        assert_eq!(fillers, vec!["ship-it"]);
    }

    #[test]
    fn zero_one_and_many_map_to_error_clean_error_for_a_required_role() {
        let role = required_name_role("plan*");

        // Zero fillers ⇒ an error-severity finding.
        let none = run(role.clone(), &[features("lint-rust", None)]);
        assert_eq!(none.len(), 1);
        assert_eq!(none[0].severity, Severity::Error);
        assert_eq!(none[0].rule, ROLE_RULE);
        assert_eq!(none[0].artifact, "planner");
        assert!(none[0].message.contains("no `skill` artifact"));

        // Exactly one filler ⇒ clean.
        let one = run(role.clone(), &[features("plan-tasks", None)]);
        assert!(one.is_empty());

        // Many fillers ⇒ an error naming the count and the colliding fillers.
        let many = run(
            role,
            &[features("plan-tasks", None), features("plan-sprints", None)],
        );
        assert_eq!(many.len(), 1);
        assert_eq!(many[0].severity, Severity::Error);
        assert!(many[0].message.contains("plan-tasks"));
        assert!(many[0].message.contains("plan-sprints"));
    }

    #[test]
    fn a_non_required_role_never_fires_at_any_count() {
        // No `required` flag (absent ⇒ false): neither zero nor many fillers is a
        // violation in this tier.
        let role = role(
            "[role.planner]\n\
             artifact = \"skill\"\n\
             contract = \"contracts/skill.anthropic.toml\"\n\
             match = { name = \"plan*\" }\n",
            "planner",
        );
        assert!(run(role.clone(), &[]).is_empty());
        assert!(
            run(
                role,
                &[features("plan-tasks", None), features("plan-sprints", None)],
            )
            .is_empty()
        );
    }

    /// A `count = { min, max }` band role over the `skill` kind — the set-scope
    /// predicate, mutually exclusive with `required`.
    fn count_band_role(min: usize, max: usize) -> Role {
        role(
            &format!(
                "[role.agents]\n\
                 artifact = \"skill\"\n\
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
        let role = count_band_role(1, 2);
        let agent = |n: u8| features(&format!("agent-{n}"), None);

        // In band (one filler, and two fillers) ⇒ clean.
        assert!(run(role.clone(), &[agent(1)]).is_empty());
        assert!(run(role.clone(), &[agent(1), agent(2)]).is_empty());

        // Below the band (zero fillers — the non-matching skill is ignored) ⇒ fires.
        let below = run(role.clone(), &[features("lint-rust", None)]);
        assert_eq!(below.len(), 1);
        assert_eq!(below[0].severity, Severity::Error);
        assert_eq!(below[0].rule, ROLE_RULE);
        assert_eq!(below[0].artifact, "agents");
        assert!(below[0].message.contains("[1, 2]"));

        // Above the band (three fillers) ⇒ fires, naming the colliding fillers.
        let above = run(role, &[agent(1), agent(2), agent(3)]);
        assert_eq!(above.len(), 1);
        assert!(above[0].message.contains("agent-1"));
        assert!(above[0].message.contains("agent-3"));
        assert!(above[0].message.contains("[1, 2]"));
    }

    #[test]
    fn a_count_role_fires_without_a_required_flag() {
        // `count` is an author-declared gate, so it fires independent of `required`
        // (with which it is mutually exclusive) — a `{ min = 2, max = 4 }` role with
        // one filler is out of band.
        let one = run(count_band_role(2, 4), &[features("agent-1", None)]);
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].rule, ROLE_RULE);
    }

    /// A role carrying an inline `max_len` contract on `name`, capped at `max`.
    fn inline_maxlen_role(max: usize) -> Role {
        role(
            &format!(
                "[role.planner]\n\
                 artifact = \"skill\"\n\
                 match = {{ name = \"plan*\" }}\n\
                 required = true\n\
                 [[role.planner.clause]]\n\
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

    /// Pack a roster of one role and skill candidates and run the conformance pass
    /// — the inline-contract path needs no `base_dir`, so an empty one suffices.
    fn run_conformance(role: Role, skills: &[Features]) -> Vec<Diagnostic> {
        let mut roles = BTreeMap::new();
        roles.insert(role.name.clone(), role);
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", skills)]);
        conformance(&roles, &by_kind, Path::new(""))
    }

    #[test]
    fn an_inline_role_contract_validates_its_selected_filler_only() {
        // The inline contract caps `name` at 3 chars; the matching filler
        // `plan-tasks` (10) breaks it, while the non-matching `lint-rust` is never
        // validated against the role's contract.
        let diags = run_conformance(
            inline_maxlen_role(3),
            &[named_skill("plan-tasks"), named_skill("lint-rust")],
        );
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, ROLE_CONFORMS_TO_RULE);
        assert_eq!(diags[0].artifact, "plan-tasks");
        // The message names the role whose contract the filler broke.
        assert!(diags[0].message.contains("planner"));
        assert!(diags[0].message.contains("does not conform"));
    }

    #[test]
    fn an_inline_role_contract_is_silent_when_the_filler_conforms() {
        // The same shape, but a generous cap the filler stays within ⇒ clean.
        assert!(run_conformance(inline_maxlen_role(64), &[named_skill("plan-tasks")]).is_empty());
    }

    #[test]
    fn conformance_and_selection_decide_independently() {
        // Two fillers — `check` would flag the single-filler overfill — and *both*
        // break the inline cap. Conformance reports each, regardless of the count.
        let diags = run_conformance(
            inline_maxlen_role(3),
            &[named_skill("plan-tasks"), named_skill("plan-sprints")],
        );
        assert_eq!(diags.len(), 2);
        assert!(diags.iter().all(|d| d.rule == ROLE_CONFORMS_TO_RULE));
    }

    #[test]
    fn a_role_whose_template_does_not_resolve_is_skipped_not_reported() {
        // The template path resolves to no file under this base dir: conformance
        // skips it (a non-resolving template is admissibility's finding), so
        // nothing fires even though a filler matches.
        let role = required_name_role("plan*"); // contract = a template path
        let skills = [features("plan-tasks", None)];
        let mut roles = BTreeMap::new();
        roles.insert(role.name.clone(), role);
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        // A base dir with no `contracts/` tree, so the template cannot load.
        let diags = conformance(&roles, &by_kind, Path::new("/no-such-temper-base-dir"));
        assert!(diags.is_empty());
    }

    #[test]
    fn a_required_role_over_an_unmodeled_kind_finds_zero_and_fails() {
        // The role's `artifact` is `command`, a kind the `by_kind` map does not
        // carry — zero candidates, so the required role fails honestly.
        let role = role(
            "[role.releaser]\n\
             artifact = \"command\"\n\
             contract = \"contracts/skill.anthropic.toml\"\n\
             match = { name = \"release*\" }\n\
             required = true\n",
            "releaser",
        );
        let mut roles = BTreeMap::new();
        roles.insert(role.name.clone(), role);
        // Only `skill` candidates are present; `command` is absent.
        let skills = [features("release-it", None)];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        let diags = check(&roles, &by_kind);
        assert_eq!(diags.len(), 1);
        assert!(diags[0].message.contains("no `command` artifact"));
    }

    // ---- admissibility ----------------------------------------------------

    /// Run the admissibility pass over a one-role roster against a `skill`-only
    /// `by_kind` (the modeled kinds are its keys; admissibility reads no fillers).
    fn run_admissibility(role: Role, base_dir: &Path) -> Vec<Diagnostic> {
        let mut roles = BTreeMap::new();
        roles.insert(role.name.clone(), role);
        let skills: [Features; 0] = [];
        let by_kind: BTreeMap<&str, &[Features]> = BTreeMap::from([("skill", &skills[..])]);
        admissibility(&roles, &by_kind, base_dir)
    }

    #[test]
    fn a_required_role_over_an_unmodeled_kind_is_inadmissible() {
        // `command` is not a kind `temper` models (only `skill` is in `by_kind`),
        // so a required role over it can never be filled — inadmissible. The inline
        // contract resolves, so the only finding is the satisfiability one.
        let role = role(
            "[role.releaser]\n\
             artifact = \"command\"\n\
             match = { name = \"release*\" }\n\
             required = true\n\
             [[role.releaser.clause]]\n\
             severity = \"required\"\n\
             predicate = \"max_len\"\n\
             field = \"name\"\n\
             max = 64\n",
            "releaser",
        );
        let diags = run_admissibility(role, Path::new(""));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, ROLE_ADMISSIBILITY_RULE);
        assert_eq!(diags[0].artifact, "releaser");
        assert!(diags[0].message.contains("command"));
        assert!(diags[0].message.contains("never be filled"));
    }

    #[test]
    fn a_non_resolving_template_contract_is_inadmissible() {
        // The template path resolves to no file under this base dir — the
        // inadmissibility this pass owns (the case `conformance` skips).
        let role = required_name_role("plan*"); // contract = a template path
        let diags = run_admissibility(role, Path::new("/no-such-temper-base-dir"));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, ROLE_ADMISSIBILITY_RULE);
        assert_eq!(diags[0].artifact, "planner");
        assert!(diags[0].message.contains("does not resolve"));
    }

    #[test]
    fn an_inline_role_contract_with_an_empty_enum_is_inadmissible() {
        // `engine::admissibility` runs on the resolved role contract, so a vacuous
        // `enum` clause is caught here exactly as in a floor contract.
        let role = role(
            "[role.planner]\n\
             artifact = \"skill\"\n\
             match = { name = \"plan*\" }\n\
             required = true\n\
             [[role.planner.clause]]\n\
             severity = \"required\"\n\
             predicate = \"enum\"\n\
             field = \"status\"\n\
             values = []\n",
            "planner",
        );
        let diags = run_admissibility(role, Path::new(""));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, ROLE_ADMISSIBILITY_RULE);
        assert_eq!(diags[0].artifact, "planner");
        assert!(diags[0].message.contains("inadmissible"));
        assert!(diags[0].message.contains("enum"));
    }

    #[test]
    fn a_dangling_verified_by_is_inadmissible() {
        // The `verified_by` path does not exist under the base dir — a dangling
        // verifier is a silent no-op, so it fails admissibility.
        let role = role(
            "[role.planner]\n\
             artifact = \"skill\"\n\
             match = { name = \"plan*\" }\n\
             required = true\n\
             verified_by = \"tests/nope.rs\"\n\
             [[role.planner.clause]]\n\
             severity = \"required\"\n\
             predicate = \"max_len\"\n\
             field = \"name\"\n\
             max = 64\n",
            "planner",
        );
        let diags = run_admissibility(role, Path::new("/no-such-temper-base-dir"));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, ROLE_ADMISSIBILITY_RULE);
        assert!(diags[0].message.contains("verifier"));
        assert!(diags[0].message.contains("tests/nope.rs"));
    }

    #[test]
    fn an_empty_role_marker_selector_is_inadmissible() {
        // A `role` marker no artifact can declare (the empty string) admits
        // nothing, so the selector does not resolve.
        let role = role(
            "[role.planner]\n\
             artifact = \"skill\"\n\
             match = { role = \"\" }\n\
             [[role.planner.clause]]\n\
             severity = \"required\"\n\
             predicate = \"max_len\"\n\
             field = \"name\"\n\
             max = 64\n",
            "planner",
        );
        let diags = run_admissibility(role, Path::new(""));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, ROLE_ADMISSIBILITY_RULE);
        assert!(diags[0].message.contains("empty"));
    }

    #[test]
    fn a_name_glob_selector_is_always_admissible() {
        // A `name` glob is well-formed under the in-crate matcher for any pattern —
        // even a bare `*` — so the selector clause never fires for it.
        let role = role(
            "[role.planner]\n\
             artifact = \"skill\"\n\
             match = { name = \"*\" }\n\
             required = true\n\
             [[role.planner.clause]]\n\
             severity = \"required\"\n\
             predicate = \"max_len\"\n\
             field = \"name\"\n\
             max = 64\n",
            "planner",
        );
        assert!(run_admissibility(role, Path::new("")).is_empty());
    }

    #[test]
    fn a_fully_resolving_roster_is_admissible() {
        // A modeled kind, a well-formed name glob, an admissible inline contract,
        // and no verifier — nothing for admissibility to reject.
        let role = inline_maxlen_role(64);
        assert!(run_admissibility(role, Path::new("")).is_empty());
    }

    #[test]
    fn an_inverted_count_bound_is_inadmissible() {
        // `min > max` admits no cardinality at all — a vacuous bound the author
        // cannot have meant, so the role's definition fails admissibility (mirroring
        // `range`'s `min > max` rejection).
        let role = role(
            "[role.agents]\n\
             artifact = \"skill\"\n\
             match = { name = \"agent-*\" }\n\
             count = { min = 3, max = 1 }\n\
             [[role.agents.clause]]\n\
             severity = \"required\"\n\
             predicate = \"max_len\"\n\
             field = \"name\"\n\
             max = 64\n",
            "agents",
        );
        let diags = run_admissibility(role, Path::new(""));
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].rule, ROLE_ADMISSIBILITY_RULE);
        assert_eq!(diags[0].artifact, "agents");
        assert!(diags[0].message.contains("inverted count bound"));
    }

    #[test]
    fn a_well_ordered_count_bound_is_admissible() {
        // `min <= max` (including a degenerate exactly-one `[1, 1]` band) is a
        // satisfiable bound — nothing for admissibility to reject.
        let role = role(
            "[role.agents]\n\
             artifact = \"skill\"\n\
             match = { name = \"agent-*\" }\n\
             count = { min = 1, max = 1 }\n\
             [[role.agents.clause]]\n\
             severity = \"required\"\n\
             predicate = \"max_len\"\n\
             field = \"name\"\n\
             max = 64\n",
            "agents",
        );
        assert!(run_admissibility(role, Path::new("")).is_empty());
    }

    #[test]
    fn a_non_required_role_over_an_unmodeled_kind_is_admissible() {
        // Satisfiability gates on `required`: a non-required role over an unmodeled
        // kind is merely never filled, which the author may have meant — not an
        // inadmissibility.
        let role = role(
            "[role.releaser]\n\
             artifact = \"command\"\n\
             match = { name = \"release*\" }\n\
             [[role.releaser.clause]]\n\
             severity = \"required\"\n\
             predicate = \"max_len\"\n\
             field = \"name\"\n\
             max = 64\n",
            "releaser",
        );
        assert!(run_admissibility(role, Path::new("")).is_empty());
    }
}
