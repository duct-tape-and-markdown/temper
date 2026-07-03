//! The generic contract engine — evaluate a [`Contract`]'s clauses over
//! extracted [`Features`].
//!
//! Implements `specs/architecture/10-contracts.md` ("Decision: kill the heuristic rule
//! registry"): rules no longer live in a hardcoded `all_rules()` registry with
//! the tool's opinions buried in `if` statements. Instead an author-declared
//! contract (a closed set of decidable clauses) is validated by *this* one
//! generic engine. The engine knows no artifact kind and no rule name — it reads
//! only the declared clauses and the deterministically-extracted features, so
//! there is nowhere to hardcode an opinion (`00-intent.md`: one engine, every
//! layer an instance).
//!
//! For each artifact's [`Features`], [`validate`] evaluates every clause as a
//! decidable predicate and, on a false predicate, emits a [`check::Diagnostic`]:
//!
//! - **severity** is the clause's *declared* weight — `required` ⇒ [`Error`],
//!   `advisory` ⇒ [`Warn`] — never a tool-baked split (`specs/architecture/10-contracts.md`,
//!   "Severity is declared, not baked").
//! - **rule** is the clause key (the predicate's TOML discriminator, e.g.
//!   `max_len`), so a finding names the clause that produced it.
//! - **artifact** is the features' `id`.
//!
//! ## The honest bound (`verified_by` philosophy)
//!
//! One predicate in the vocabulary — `dependency-exists` — is **held back**: it
//! names no decidable reference syntax or extractor yet (a declared-dependency
//! model the current [`Features`] projection does not carry), so the engine
//! could only ever return *indeterminate* for it — a silent no-op law 1 forbids.
//! Rather than fabricate a pass or degrade to that no-op, [`admissibility`]
//! **fences it**: a contract carrying a `dependency-exists` clause fails
//! admissibility, exactly as the full `pattern` primitive is held back, so a
//! hand-authored clause fails loudly instead of quietly deciding nothing. The
//! decidable members the spec keeps — name format,
//! lengths, forbidden keys, required fields, `name-matches-dir`, body
//! `max_lines`, `require_sections` over the extracted headings, and
//! `section_contains` over the extracted sections — are evaluated here in full.
//!
//! [`Error`]: check::Severity::Error
//! [`Warn`]: check::Severity::Warn

use std::collections::BTreeSet;

use crate::check::{self, Diagnostic};
use crate::contract::{self, Contract, Predicate};
use crate::extract::{FeatureValue, Features, Kind};

/// Validate every artifact's [`Features`] against the contract's clauses,
/// collecting a [`Diagnostic`] per violation at the clause's declared severity.
///
/// The artifact slice is passed whole because cross-artifact clauses (e.g.
/// `unique-name`) decide over the set, not one unit — the whole-workspace shape
/// the [`Workspace`](crate::check::Workspace) IR carries.
#[must_use]
pub fn validate(contract: &Contract, artifacts: &[Features]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for features in artifacts {
        for clause in &contract.clauses {
            for message in evaluate(&clause.predicate, features, artifacts) {
                diagnostics.push(
                    Diagnostic::new(
                        severity_of(clause.severity),
                        clause.predicate.key(),
                        &features.id,
                        message,
                    )
                    // The clause's colocated guidance rides its own violation — the
                    // just-in-time teaching moment (`specs/architecture/10-contracts.md`).
                    .with_guidance(clause.guidance.clone()),
                );
            }
        }
    }
    diagnostics
}

/// Validate a contract against **the definition** — the closed algebra itself —
/// returning an error-severity [`Diagnostic`] per inadmissible clause. This is
/// *admissibility* (`specs/architecture/10-contracts.md`, "Decision: the contract is itself
/// checked — admissibility"): the contract earns trust the way a harness does,
/// by passing a check, before it is used to check anything.
///
/// Admissibility composes *on top* of loading, never re-doing it. Closed-
/// vocabulary rejection (an unknown predicate) and charset-range validity are
/// already enforced as load errors in [`crate::contract`]; a [`Contract`] that
/// reached this engine has cleared both. The only admissibility clause decidable
/// today over the current algebra is **list non-emptiness**: an `enum` or `deny`
/// with no values, or a `forbidden_keys` / `require_sections` with no entries, is
/// a vacuous clause that can never decide anything — inadmissible. (The
/// `pattern`-compiles and `verified_by`-resolves clauses the spec also names extend
/// this same pass when those primitives land.)
///
/// Every finding is [`check::Severity::Error`]: an inadmissible contract must
/// fail the run, exactly as a `required` conformance violation does — there is no
/// "advisory" admissibility, because a contract that cannot be trusted cannot be
/// used. The diagnostic's `artifact` is the contract's display label so a finding
/// names the contract it indicts.
#[must_use]
pub fn admissibility(contract: &Contract) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for clause in &contract.clauses {
        for message in inadmissibilities(&clause.predicate) {
            diagnostics.push(Diagnostic::error(
                clause.predicate.key(),
                &contract.name,
                message,
            ));
        }
    }
    diagnostics
}

/// The admissibility violations of a single clause's predicate — empty when the
/// clause is well-formed over the definition. Two decidable checks live here
/// today: (1) a value/key list is non-empty — a list-bearing predicate with an
/// empty list is vacuous (an `enum` over no values admits nothing;
/// `forbidden_keys` over no keys forbids nothing), which the author cannot have
/// meant; and (2) no **held-back** predicate is used as a working clause —
/// `dependency-exists` names no decidable reference syntax or extractor, so it is
/// inadmissible until it does (`specs/architecture/10-contracts.md`, "The primitive algebra").
fn inadmissibilities(predicate: &Predicate) -> Vec<String> {
    match predicate {
        // `dependency-exists` is held back — like the full `pattern` primitive.
        // It names no decidable reference syntax or extractor, so the engine
        // could only return `Indeterminate` for it (a silent no-op law 1
        // forbids). A hand-authored clause must therefore fail admissibility, not
        // degrade to a working no-op.
        Predicate::DependencyExists => {
            vec![
                "`dependency-exists` is held back: it names no decidable reference \
                 syntax or extractor, so it is inadmissible as a contract clause"
                    .to_string(),
            ]
        }
        Predicate::Enum { field, values } if values.is_empty() => {
            vec![format!("`enum` clause on field `{field}` lists no values")]
        }
        Predicate::Deny { field, values } if values.is_empty() => {
            vec![format!("`deny` clause on field `{field}` lists no values")]
        }
        Predicate::ForbiddenKeys { keys } if keys.is_empty() => {
            vec!["`forbidden_keys` clause lists no keys".to_string()]
        }
        Predicate::RequireSections { sections } if sections.is_empty() => {
            vec!["`require_sections` clause lists no sections".to_string()]
        }
        // An empty `section_contains` marker is a substring of every body, so the
        // clause can never fire — vacuous, and inadmissible like an empty-list
        // clause. An empty *heading* prefix is not vacuous (it governs every
        // section, a meaningful "every section carries the marker"), so it stands.
        Predicate::SectionContains { heading, marker } if marker.is_empty() => {
            vec![format!(
                "`section_contains` clause on heading `{heading}` names an empty marker"
            )]
        }
        // An inverted bound (`min > max`) admits no value at all — a vacuous
        // clause the author cannot have meant, so the contract carrying it fails
        // admissibility (`specs/architecture/45-governance.md`, "reject min>max").
        Predicate::Range { field, min, max } if min > max => {
            vec![format!(
                "`range` clause on field `{field}` has min {min} greater than max {max}"
            )]
        }
        _ => Vec::new(),
    }
}

/// Evaluate one predicate over one artifact's features, returning a message per
/// violation (empty ⇒ the clause holds, or could not be decided over this
/// projection — see [`Outcome`]).
fn evaluate(predicate: &Predicate, features: &Features, all: &[Features]) -> Vec<String> {
    match decide(predicate, features, all) {
        Outcome::Holds | Outcome::Indeterminate => Vec::new(),
        Outcome::Violated(messages) => messages,
    }
}

/// The result of testing a predicate against features. `Indeterminate` is the
/// honest third state for a clause whose backing feature the current projection
/// does not carry — distinct from `Holds`, so the engine never *claims* to have
/// checked what it could not.
enum Outcome {
    /// The predicate is true of the features.
    Holds,
    /// The predicate is false; each string is one violation to report.
    Violated(Vec<String>),
    /// The feature this predicate names is absent from the projection, so the
    /// clause cannot be decided here (no pass, no finding).
    Indeterminate,
}

impl Outcome {
    /// A single-message violation.
    fn violated(message: String) -> Self {
        Outcome::Violated(vec![message])
    }

    /// `Holds` when `ok`, else a single-message violation.
    fn check(ok: bool, message: impl FnOnce() -> String) -> Self {
        if ok {
            Outcome::Holds
        } else {
            Outcome::violated(message())
        }
    }
}

/// The decision table — one arm per primitive. Every arm is decidable *given the
/// feature it names*; the one held-back predicate whose feature the projection
/// omits (`dependency-exists`) returns [`Outcome::Indeterminate`] rather than a
/// fabricated pass — though [`admissibility`] fences it before a valid run ever
/// reaches conformance, so that arm is a defensive floor, not a working clause.
fn decide(predicate: &Predicate, features: &Features, all: &[Features]) -> Outcome {
    match predicate {
        // A value/presence predicate is the *only* owner of its field's
        // presence; the other field predicates stay silent when the field is
        // absent so one missing field yields one finding, not a cascade.
        Predicate::Required { field } => Outcome::check(features.has_field(field), || {
            format!("required field `{field}` is absent")
        }),

        // `optional` records that a key is part of the declared schema; it is
        // always satisfied — its presence or absence is never a violation.
        Predicate::Optional { .. } => Outcome::Holds,

        // `type` compares the field's *preserved source kind* to the declared
        // one. An absent field is the `required` clause's concern, so `type`
        // stays silent on absence (like the other field predicates).
        Predicate::Type { field, kind } => match features.field(field).map(FeatureValue::kind) {
            None => Outcome::Holds,
            Some(actual) => Outcome::check(actual == *kind, || {
                format!(
                    "field `{field}` is `{}` but the contract declares `{}`",
                    actual.name(),
                    kind.name()
                )
            }),
        },

        Predicate::MinLen { field, min } => match scalar(features, field) {
            None => Outcome::Holds,
            Some(value) => {
                let len = value.chars().count();
                Outcome::check(len >= *min, || {
                    format!("field `{field}` is {len} characters (min {min})")
                })
            }
        },

        Predicate::MaxLen { field, max } => match scalar(features, field) {
            None => Outcome::Holds,
            Some(value) => {
                let len = value.chars().count();
                Outcome::check(len <= *max, || {
                    format!("field `{field}` is {len} characters (max {max})")
                })
            }
        },

        // `range` bounds a *numeric* field to `[min, max]`. It fires only when the
        // field is present, parsed as `integer`/`number`, and falls outside the
        // bound; it stays silent on absence (the `required` clause's concern) and
        // on a non-numeric kind (a `type` clause owns that mismatch) so one wrong
        // field yields one finding, not a cascade.
        Predicate::Range { field, min, max } => match features.field(field) {
            Some(value) if matches!(value.kind(), Kind::Integer | Kind::Number) => {
                match value.as_scalar().and_then(|text| text.parse::<f64>().ok()) {
                    Some(n) => Outcome::check((*min..=*max).contains(&n), || {
                        format!("field `{field}` value {n} is outside the range [{min}, {max}]")
                    }),
                    // Kind says numeric but the text would not parse — don't
                    // fabricate a finding over a value we cannot read.
                    None => Outcome::Holds,
                }
            }
            // Absent, or a non-numeric kind: not this predicate's concern.
            _ => Outcome::Holds,
        },

        Predicate::Enum { field, values } => match scalar(features, field) {
            None => Outcome::Holds,
            Some(value) => Outcome::check(values.iter().any(|v| v == value), || {
                format!(
                    "field `{field}` value `{value}` is not one of [{}]",
                    values.join(", ")
                )
            }),
        },

        Predicate::Deny { field, values } => match scalar(features, field) {
            None => Outcome::Holds,
            Some(value) => Outcome::check(!values.iter().any(|v| v == value), || {
                format!("field `{field}` value `{value}` is denied")
            }),
        },

        // One finding per offending key, so each forbidden key points at itself.
        Predicate::ForbiddenKeys { keys } => {
            let present: Vec<String> = keys
                .iter()
                .filter(|key| features.has_field(key))
                .map(|key| format!("forbidden key `{key}` is present"))
                .collect();
            if present.is_empty() {
                Outcome::Holds
            } else {
                Outcome::Violated(present)
            }
        }

        Predicate::AllowedChars { field, charset } => match scalar(features, field) {
            None => Outcome::Holds,
            Some(value) => {
                let bad: BTreeSet<char> = value.chars().filter(|&c| !charset.allows(c)).collect();
                Outcome::check(bad.is_empty(), || {
                    let rendered: String = bad.iter().collect();
                    format!("field `{field}` has characters outside the allowed set: {rendered}")
                })
            }
        },

        Predicate::MaxLines { max } => Outcome::check(features.body_lines <= *max, || {
            format!("body is {} lines (max {max})", features.body_lines)
        }),

        // `require_sections` decides over the extracted body headings: one
        // finding per named section with no matching heading.
        Predicate::RequireSections { sections } => {
            let missing: Vec<String> = sections
                .iter()
                .filter(|section| !features.headings.iter().any(|h| h == *section))
                .map(|section| format!("required section `{section}` is absent from the body"))
                .collect();
            if missing.is_empty() {
                Outcome::Holds
            } else {
                Outcome::Violated(missing)
            }
        }

        // `must_define` over a frontmatter marker (e.g. `disable-model-invocation`)
        // is decidable as field presence.
        Predicate::MustDefine { marker } => Outcome::check(features.has_field(marker), || {
            format!("marker `{marker}` is not defined")
        }),

        // `section_contains` decides over the extracted body sections: every
        // section whose heading *starts with* the declared prefix must carry the
        // declared marker (a substring of its body). One finding per bare section,
        // so each offending section points at itself.
        Predicate::SectionContains { heading, marker } => {
            let bare: Vec<String> = features
                .sections
                .iter()
                .filter(|section| section.heading.starts_with(heading.as_str()))
                .filter(|section| !section.body.contains(marker.as_str()))
                .map(|section| {
                    format!(
                        "section `{}` does not carry the required marker `{marker}`",
                        section.heading
                    )
                })
                .collect();
            if bare.is_empty() {
                Outcome::Holds
            } else {
                Outcome::Violated(bare)
            }
        }

        Predicate::NameMatchesDir => {
            match (scalar(features, "name"), features.source_dir.as_deref()) {
                (Some(name), Some(dir)) => Outcome::check(name == dir, || {
                    format!("name `{name}` does not match its directory `{dir}`")
                }),
                // No name field, or no known source directory: nothing to compare.
                _ => Outcome::Holds,
            }
        }

        Predicate::UniqueName => {
            let shared = all.iter().filter(|other| other.id == features.id).count();
            Outcome::check(shared <= 1, || {
                format!(
                    "name `{}` is not unique ({shared} artifacts share it)",
                    features.id
                )
            })
        }

        // `dependency-exists` is held back — [`admissibility`] rejects any
        // contract carrying it, so a valid conformance run never reaches this arm.
        // It stays `Indeterminate` as a defensive floor (never a fabricated pass)
        // and to light up with no engine change once the extractor grows a
        // declared-dependency model.
        Predicate::DependencyExists => Outcome::Indeterminate,
    }
}

/// The scalar text of a named field, or `None` if it is absent or a list — the
/// generic accessor every value predicate (`min_len`, `enum`, …) reads through.
fn scalar<'a>(features: &'a Features, field: &str) -> Option<&'a str> {
    features.field(field).and_then(FeatureValue::as_scalar)
}

/// Map a clause's *declared* severity onto the engine's diagnostic severity:
/// `required` blocks (`Error`), `advisory` reports (`Warn`). The engine never
/// chooses — it only translates what the author declared.
///
/// `pub` so an assembly-scope dial that shares the author's `required`/`advisory`
/// vocabulary — the reachability severity (`specs/architecture/45-governance.md`) — maps through
/// the one translation, never a second copy that could drift.
#[must_use]
pub fn severity_of(severity: contract::Severity) -> check::Severity {
    match severity {
        contract::Severity::Required => check::Severity::Error,
        contract::Severity::Advisory => check::Severity::Warn,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use crate::check::{Severity, any_error};
    use crate::contract::{Charset, Clause, Severity as ClauseSeverity};
    use crate::extract::Kind;

    /// Build a `Features` with the given name-keyed scalar fields, body line
    /// count, and source directory.
    fn features(
        id: &str,
        fields: &[(&str, FeatureValue)],
        body_lines: usize,
        source_dir: Option<&str>,
    ) -> Features {
        let fields = fields
            .iter()
            .map(|(k, v)| ((*k).to_string(), v.clone()))
            .collect::<BTreeMap<_, _>>();
        Features {
            id: id.to_string(),
            fields,
            body_lines,
            headings: Vec::new(),
            sections: Vec::new(),
            source_dir: source_dir.map(str::to_string),
            directives: Vec::new(),
            satisfies: Vec::new(),
            published_requirements: Vec::new(),
        }
    }

    /// `features` with the given body headings — for the `require_sections`
    /// tests, which decide over headings rather than fields.
    fn features_with_headings(id: &str, headings: &[&str]) -> Features {
        let mut f = features(id, &[], 1, None);
        f.headings = headings.iter().map(|h| (*h).to_string()).collect();
        f
    }

    /// A scalar field value (kind `string`; the existing scalar predicates read
    /// only the text, so the kind is incidental to these tests).
    fn scalar(text: &str) -> FeatureValue {
        FeatureValue::scalar(Kind::String, text)
    }

    /// A one-clause contract carrying `predicate` at `severity`.
    fn contract(severity: ClauseSeverity, predicate: Predicate) -> Contract {
        Contract {
            name: "skill".to_string(),
            guidance: None,
            clauses: vec![Clause {
                source: None,
                severity,
                predicate,
                guidance: None,
            }],
        }
    }

    /// The `[a-z0-9-]` charset, the `allowed_chars` workhorse.
    fn slug_charset() -> Charset {
        Charset {
            ranges: vec![('a', 'z'), ('0', '9')],
            chars: BTreeSet::from(['-']),
        }
    }

    /// Validate a single artifact against a single required clause and return the
    /// diagnostics — the common shape of the per-primitive tests below.
    fn run(predicate: Predicate, artifact: Features) -> Vec<Diagnostic> {
        validate(
            &contract(ClauseSeverity::Required, predicate),
            std::slice::from_ref(&artifact),
        )
    }

    #[test]
    fn required_fires_on_an_absent_field_and_is_silent_when_present() {
        let absent = features("demo", &[], 1, None);
        let diags = run(
            Predicate::Required {
                field: "name".to_string(),
            },
            absent,
        );
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "required");
        assert_eq!(diags[0].artifact, "demo");

        let present = features("demo", &[("name", scalar("demo"))], 1, None);
        assert!(
            run(
                Predicate::Required {
                    field: "name".to_string()
                },
                present
            )
            .is_empty()
        );
    }

    #[test]
    fn optional_never_fires() {
        let any = features("demo", &[], 1, None);
        assert!(
            run(
                Predicate::Optional {
                    field: "license".to_string()
                },
                any
            )
            .is_empty()
        );
    }

    #[test]
    fn type_fires_on_a_kind_mismatch_and_is_silent_on_match_and_absence() {
        let predicate = || Predicate::Type {
            field: "count".to_string(),
            kind: Kind::Integer,
        };

        // The field's preserved source kind differs from the declared one: fires.
        let mismatch = features(
            "demo",
            &[("count", FeatureValue::scalar(Kind::String, "7"))],
            1,
            None,
        );
        let diags = run(predicate(), mismatch);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "type");
        // The message names both the actual and the declared lattice kind.
        assert!(diags[0].message.contains("string"));
        assert!(diags[0].message.contains("integer"));

        // The kind matches the declaration: silent.
        let matched = features(
            "demo",
            &[("count", FeatureValue::scalar(Kind::Integer, "7"))],
            1,
            None,
        );
        assert!(run(predicate(), matched).is_empty());

        // An absent field is the `required` clause's concern, not `type`'s.
        let absent = features("demo", &[], 1, None);
        assert!(run(predicate(), absent).is_empty());

        // A container kind is decided the same way — a list where a map is
        // declared fires.
        let container = Predicate::Type {
            field: "tags".to_string(),
            kind: Kind::Map,
        };
        let as_list = features(
            "demo",
            &[("tags", FeatureValue::List(vec!["a".to_string()]))],
            1,
            None,
        );
        assert_eq!(run(container, as_list).len(), 1);
    }

    #[test]
    fn max_len_fires_only_past_the_bound() {
        let predicate = || Predicate::MaxLen {
            field: "name".to_string(),
            max: 3,
        };
        let over = features("demo", &[("name", scalar("toolong"))], 1, None);
        let diags = run(predicate(), over);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "max_len");

        let within = features("demo", &[("name", scalar("ok"))], 1, None);
        assert!(run(predicate(), within).is_empty());
        // An absent field is the `required` clause's concern, not this one's.
        let absent = features("demo", &[], 1, None);
        assert!(run(predicate(), absent).is_empty());
    }

    #[test]
    fn range_fires_only_when_a_numeric_field_falls_outside_the_bound() {
        let predicate = || Predicate::Range {
            field: "score".to_string(),
            min: 0.0,
            max: 100.0,
        };

        // A numeric field past the upper bound fires once, naming the clause.
        let over = features(
            "demo",
            &[("score", FeatureValue::scalar(Kind::Integer, "150"))],
            1,
            None,
        );
        let diags = run(predicate(), over);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "range");

        // Below the lower bound fires too — a fractional `number` is in scope.
        let under = features(
            "demo",
            &[("score", FeatureValue::scalar(Kind::Number, "-0.5"))],
            1,
            None,
        );
        assert_eq!(run(predicate(), under).len(), 1);

        // Within the inclusive bound (and exactly on each edge): silent.
        let within = features(
            "demo",
            &[("score", FeatureValue::scalar(Kind::Integer, "42"))],
            1,
            None,
        );
        assert!(run(predicate(), within).is_empty());
        for edge in ["0", "100"] {
            let at_edge = features(
                "demo",
                &[("score", FeatureValue::scalar(Kind::Integer, edge))],
                1,
                None,
            );
            assert!(run(predicate(), at_edge).is_empty(), "edge {edge} holds");
        }

        // An absent field is the `required` clause's concern, not this one's.
        let absent = features("demo", &[], 1, None);
        assert!(run(predicate(), absent).is_empty());

        // A non-numeric kind is a `type` clause's concern: `range` stays silent
        // rather than fire on a value it does not own — no cascade.
        let non_numeric = features(
            "demo",
            &[("score", FeatureValue::scalar(Kind::String, "150"))],
            1,
            None,
        );
        assert!(run(predicate(), non_numeric).is_empty());
    }

    #[test]
    fn an_inverted_range_is_inadmissible() {
        // A `min > max` bound admits no value — vacuous, so the contract carrying
        // it fails admissibility (an error, exit non-zero).
        let inverted = contract(
            ClauseSeverity::Required,
            Predicate::Range {
                field: "score".to_string(),
                min: 100.0,
                max: 0.0,
            },
        );
        let diags = admissibility(&inverted);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "range");
        assert_eq!(diags[0].severity, Severity::Error);
        assert_eq!(diags[0].artifact, "skill");
        assert!(any_error(&diags));

        // A well-formed `min <= max` bound (equal endpoints included) is admissible.
        for (min, max) in [(0.0, 100.0), (5.0, 5.0)] {
            let ok = contract(
                ClauseSeverity::Required,
                Predicate::Range {
                    field: "score".to_string(),
                    min,
                    max,
                },
            );
            assert!(
                admissibility(&ok).is_empty(),
                "[{min}, {max}] is admissible"
            );
        }
    }

    #[test]
    fn min_len_fires_only_below_the_bound() {
        let predicate = || Predicate::MinLen {
            field: "description".to_string(),
            min: 5,
        };
        let under = features("demo", &[("description", scalar("hi"))], 1, None);
        assert_eq!(run(predicate(), under).len(), 1);

        let ok = features("demo", &[("description", scalar("plenty"))], 1, None);
        assert!(run(predicate(), ok).is_empty());
    }

    #[test]
    fn enum_fires_off_the_permitted_set() {
        let predicate = || Predicate::Enum {
            field: "status".to_string(),
            values: vec!["draft".to_string(), "active".to_string()],
        };
        let bad = features("demo", &[("status", scalar("retired"))], 1, None);
        assert_eq!(run(predicate(), bad).len(), 1);

        let good = features("demo", &[("status", scalar("active"))], 1, None);
        assert!(run(predicate(), good).is_empty());
    }

    #[test]
    fn deny_fires_on_a_forbidden_value() {
        let predicate = || Predicate::Deny {
            field: "name".to_string(),
            values: vec!["anthropic".to_string(), "claude".to_string()],
        };
        let reserved = features("claude", &[("name", scalar("claude"))], 1, None);
        assert_eq!(run(predicate(), reserved).len(), 1);

        let fine = features("demo", &[("name", scalar("demo"))], 1, None);
        assert!(run(predicate(), fine).is_empty());
    }

    #[test]
    fn forbidden_keys_fire_once_per_present_key() {
        let predicate = || Predicate::ForbiddenKeys {
            keys: vec!["globs".to_string(), "alwaysApply".to_string()],
        };
        let legacy = features(
            "legacy",
            &[
                ("globs", scalar("**/*.rs")),
                ("alwaysApply", scalar("true")),
            ],
            1,
            None,
        );
        let diags = run(predicate(), legacy);
        assert_eq!(diags.len(), 2);
        assert!(diags.iter().all(|d| d.rule == "forbidden_keys"));

        let clean = features("clean", &[("name", scalar("clean"))], 1, None);
        assert!(run(predicate(), clean).is_empty());
    }

    #[test]
    fn allowed_chars_fires_on_a_character_outside_the_set() {
        let predicate = || Predicate::AllowedChars {
            field: "name".to_string(),
            charset: slug_charset(),
        };
        let shouty = features("Demo_1", &[("name", scalar("Demo_1"))], 1, None);
        let diags = run(predicate(), shouty);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "allowed_chars");
        // The offending characters, deduped and sorted, ride in the message.
        assert!(diags[0].message.contains('D'));
        assert!(diags[0].message.contains('_'));

        let slug = features("demo-1", &[("name", scalar("demo-1"))], 1, None);
        assert!(run(predicate(), slug).is_empty());
    }

    #[test]
    fn max_lines_fires_only_past_the_budget() {
        let predicate = || Predicate::MaxLines { max: 500 };
        let long = features("demo", &[], 501, None);
        assert_eq!(run(predicate(), long).len(), 1);

        // Exactly at the bound is "at most max" — it holds.
        let at_bound = features("demo", &[], 500, None);
        assert!(run(predicate(), at_bound).is_empty());
    }

    #[test]
    fn must_define_fires_when_the_marker_is_absent() {
        let predicate = || Predicate::MustDefine {
            marker: "disable-model-invocation".to_string(),
        };
        let missing = features("demo", &[("name", scalar("demo"))], 1, None);
        assert_eq!(run(predicate(), missing).len(), 1);

        let defined = features(
            "demo",
            &[("disable-model-invocation", scalar("true"))],
            1,
            None,
        );
        assert!(run(predicate(), defined).is_empty());
    }

    #[test]
    fn name_matches_dir_fires_on_a_mismatch() {
        let mismatch = features("demo", &[("name", scalar("demo"))], 1, Some("other"));
        let diags = run(Predicate::NameMatchesDir, mismatch);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "name-matches-dir");

        let aligned = features("demo", &[("name", scalar("demo"))], 1, Some("demo"));
        assert!(run(Predicate::NameMatchesDir, aligned).is_empty());
    }

    #[test]
    fn unique_name_fires_for_each_colliding_artifact() {
        let a = features("dup", &[("name", scalar("dup"))], 1, None);
        let b = features("dup", &[("name", scalar("dup"))], 1, None);
        let c = features("solo", &[("name", scalar("solo"))], 1, None);
        let diags = validate(
            &contract(ClauseSeverity::Required, Predicate::UniqueName),
            &[a, b, c],
        );
        // Both `dup` artifacts report; `solo` does not.
        assert_eq!(diags.len(), 2);
        assert!(diags.iter().all(|d| d.artifact == "dup"));
    }

    #[test]
    fn require_sections_fires_per_missing_heading_and_is_silent_when_all_present() {
        let predicate = || Predicate::RequireSections {
            sections: vec!["Usage".to_string(), "Examples".to_string()],
        };

        // One finding per named section with no matching heading.
        let missing = features_with_headings("demo", &["Usage"]);
        let diags = run(predicate(), missing);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "require_sections");
        assert_eq!(diags[0].artifact, "demo");
        assert!(diags[0].message.contains("Examples"));

        // Every named heading present (order and extras are irrelevant): silent.
        let complete = features_with_headings("demo", &["Examples", "Intro", "Usage"]);
        assert!(run(predicate(), complete).is_empty());
    }

    #[test]
    fn dependency_exists_is_inadmissible() {
        // `dependency-exists` is held back — it names no decidable reference
        // syntax or extractor, so a hand-authored clause must fail admissibility
        // loudly rather than silently decide `Indeterminate` (a no-op law 1
        // forbids). The fence is mirrored on the full `pattern` primitive.
        let held = contract(ClauseSeverity::Required, Predicate::DependencyExists);
        let diags = admissibility(&held);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "dependency-exists");
        assert_eq!(diags[0].severity, Severity::Error);
        // The finding names the contract it indicts.
        assert_eq!(diags[0].artifact, "skill");
        assert!(any_error(&diags));

        // A clause's declared severity is irrelevant: it is inadmissible even
        // when marked advisory, because an inadmissible contract cannot be used.
        let advisory = contract(ClauseSeverity::Advisory, Predicate::DependencyExists);
        assert_eq!(admissibility(&advisory).len(), 1);
    }

    #[test]
    fn an_empty_enum_clause_is_inadmissible() {
        // A clause that lists no values can never decide anything — vacuous, so
        // the contract carrying it fails admissibility (an error, exit non-zero).
        let empty_enum = contract(
            ClauseSeverity::Required,
            Predicate::Enum {
                field: "status".to_string(),
                values: Vec::new(),
            },
        );
        let diags = admissibility(&empty_enum);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].rule, "enum");
        assert_eq!(diags[0].severity, Severity::Error);
        // The finding names the contract it indicts.
        assert_eq!(diags[0].artifact, "skill");
        assert!(any_error(&diags));
    }

    #[test]
    fn an_empty_list_clause_of_every_list_kind_is_inadmissible() {
        // Each list-bearing predicate is inadmissible when its list is empty; the
        // finding's `rule` names the offending clause.
        for (predicate, key) in [
            (
                Predicate::Deny {
                    field: "name".to_string(),
                    values: Vec::new(),
                },
                "deny",
            ),
            (
                Predicate::ForbiddenKeys { keys: Vec::new() },
                "forbidden_keys",
            ),
            (
                Predicate::RequireSections {
                    sections: Vec::new(),
                },
                "require_sections",
            ),
        ] {
            let diags = admissibility(&contract(ClauseSeverity::Required, predicate));
            assert_eq!(diags.len(), 1, "{key} with an empty list should fire once");
            assert_eq!(diags[0].rule, key);
            assert_eq!(diags[0].severity, Severity::Error);
        }
    }

    #[test]
    fn a_well_formed_contract_is_admissible() {
        // Non-empty lists, and the non-list primitives, carry nothing for
        // admissibility to reject — the multi-clause representative is admissible.
        let clauses = vec![
            Clause {
                source: None,
                severity: ClauseSeverity::Required,
                guidance: None,
                predicate: Predicate::Enum {
                    field: "status".to_string(),
                    values: vec!["draft".to_string(), "active".to_string()],
                },
            },
            Clause {
                source: None,
                severity: ClauseSeverity::Required,
                guidance: None,
                predicate: Predicate::Deny {
                    field: "name".to_string(),
                    values: vec!["claude".to_string()],
                },
            },
            Clause {
                source: None,
                severity: ClauseSeverity::Required,
                guidance: None,
                predicate: Predicate::ForbiddenKeys {
                    keys: vec!["globs".to_string()],
                },
            },
            Clause {
                source: None,
                severity: ClauseSeverity::Advisory,
                guidance: None,
                predicate: Predicate::RequireSections {
                    sections: vec!["Usage".to_string()],
                },
            },
            Clause {
                source: None,
                severity: ClauseSeverity::Required,
                guidance: None,
                predicate: Predicate::Required {
                    field: "name".to_string(),
                },
            },
            Clause {
                source: None,
                severity: ClauseSeverity::Required,
                guidance: None,
                predicate: Predicate::NameMatchesDir,
            },
        ];
        let contract = Contract {
            name: "skill".to_string(),
            guidance: None,
            clauses,
        };
        assert!(admissibility(&contract).is_empty());
    }

    #[test]
    fn declared_severity_maps_required_to_error_and_advisory_to_warn() {
        let violating = features("demo", &[], 1, None);
        let predicate = || Predicate::Required {
            field: "name".to_string(),
        };

        let required = validate(
            &contract(ClauseSeverity::Required, predicate()),
            std::slice::from_ref(&violating),
        );
        assert_eq!(required[0].severity, Severity::Error);

        let advisory = validate(
            &contract(ClauseSeverity::Advisory, predicate()),
            std::slice::from_ref(&violating),
        );
        assert_eq!(advisory[0].severity, Severity::Warn);
    }

    #[test]
    fn an_all_advisory_run_yields_no_error() {
        // Every clause advisory; the artifact violates all of them.
        let clauses = vec![
            Clause {
                source: None,
                severity: ClauseSeverity::Advisory,
                guidance: None,
                predicate: Predicate::Required {
                    field: "name".to_string(),
                },
            },
            Clause {
                source: None,
                severity: ClauseSeverity::Advisory,
                guidance: None,
                predicate: Predicate::MaxLines { max: 10 },
            },
        ];
        let contract = Contract {
            name: "skill".to_string(),
            guidance: None,
            clauses,
        };
        let violating = features("demo", &[], 99, None);

        let diags = validate(&contract, std::slice::from_ref(&violating));
        assert_eq!(diags.len(), 2);
        assert!(diags.iter().all(|d| d.severity == Severity::Warn));
        // The whole point of `advisory`: it reports without blocking the gate.
        assert!(!any_error(&diags));
    }

    #[test]
    fn a_conforming_artifact_against_a_multi_clause_contract_is_clean() {
        let clauses = vec![
            Clause {
                source: None,
                severity: ClauseSeverity::Required,
                guidance: None,
                predicate: Predicate::Required {
                    field: "name".to_string(),
                },
            },
            Clause {
                source: None,
                severity: ClauseSeverity::Required,
                guidance: None,
                predicate: Predicate::MaxLen {
                    field: "name".to_string(),
                    max: 64,
                },
            },
            Clause {
                source: None,
                severity: ClauseSeverity::Required,
                guidance: None,
                predicate: Predicate::AllowedChars {
                    field: "name".to_string(),
                    charset: slug_charset(),
                },
            },
            Clause {
                source: None,
                severity: ClauseSeverity::Required,
                guidance: None,
                predicate: Predicate::ForbiddenKeys {
                    keys: vec!["globs".to_string()],
                },
            },
            Clause {
                source: None,
                severity: ClauseSeverity::Advisory,
                guidance: None,
                predicate: Predicate::MaxLines { max: 500 },
            },
            Clause {
                source: None,
                severity: ClauseSeverity::Required,
                guidance: None,
                predicate: Predicate::NameMatchesDir,
            },
        ];
        let contract = Contract {
            name: "skill".to_string(),
            guidance: None,
            clauses,
        };
        let conforming = features("demo", &[("name", scalar("demo"))], 12, Some("demo"));

        assert!(validate(&contract, std::slice::from_ref(&conforming)).is_empty());
    }
}
