//! Roster conformance — role match-selection over the parsed harness contract.
//!
//! Implements the selection half of `specs/10-contracts.md` ("Roles and
//! matching"): a harness contract binds an abstract **role** to whichever
//! concrete artifact fills it, and *which* artifact is itself a decidable
//! selector, never a guess. The roster — the `[role.<name>]` tables parsed onto
//! [`AuthorLayer`](crate::compose::AuthorLayer) by the shipped parse foundation —
//! reaches this pass as typed [`Role`]s; here each role's [`MatchSelector`] is
//! evaluated against the workspace artifacts of the role's `artifact` kind, and a
//! **`required` single-filler role filled by zero or by many artifacts is a
//! conformance error**, reported precisely (`specs/10-contracts.md`: "When zero or
//! many artifacts match a `required` single-filler role, that is a conformance
//! error").
//!
//! ## Selection only — `conforms-to` and admissibility stay frontier
//!
//! This tier decides *which* artifacts fill a role and whether a `required`
//! single-filler role is satisfiably filled — nothing more. The `role` primitive
//! also asks that the filler `conforms-to` the role's contract, and admissibility
//! asks that the `match` selector and any `verified_by` *resolve*; both are
//! follow-on entries. A non-`required` role never fires here: `temper` never
//! fabricates a gate the author did not declare (`00-intent.md` law 4).
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
use crate::compose::{MatchSelector, Role};
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
        // `temper` never fabricates a gate the author did not declare: a
        // non-`required` role's filler count is not a violation in this tier.
        if !role.required {
            continue;
        }
        let candidates = by_kind.get(role.artifact.as_str()).copied().unwrap_or(&[]);
        let fillers = fillers(&role.selector, candidates);
        match fillers.as_slice() {
            // Exactly one filler — the single-filler role is satisfied.
            [_] => {}
            [] => diagnostics.push(unfilled(role)),
            many => diagnostics.push(overfilled(role, many)),
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
}
