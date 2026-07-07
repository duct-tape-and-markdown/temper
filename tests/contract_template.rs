//! Pins the shipped Anthropic skill built-in floor (`specs/architecture/10-contracts.md`,
//! "Packages — best practices as data").
//!
//! The `skill` floor is a projection of the embedded built-in lock's clause rows
//! (`specs/architecture/50-distribution.md`, "Decision: the built-in lock is derived
//! from the SDK module, never transcribed"). This test loads it through the same
//! embedded path the shipped `check` uses ([`temper::builtin::contract`]) and pins
//! the exact decidable clause vector it carries.
//!
//! Pinning the vector proves both halves of the registry-kill decision at once. The
//! surviving *decidable* clauses are present at their declared severities, and the
//! *dropped* heuristics (third-person, has-trigger, companion-refs — semantic or
//! weak proxies, out of the closed algebra entirely) are absent, because any extra
//! or missing clause breaks the equality. A floor is named for its kind, not a
//! package — the built-in's display label is `skill`.
//! The clause *vocabulary* is pinned; the guidance/citation prose is product
//! territory, so it is asserted present, not pinned verbatim.

use std::collections::BTreeSet;
use std::path::Path;

use temper::contract::{Charset, Contract, Predicate, Severity};
use temper::engine;
use temper::schema;

/// The built-in skill contract, resolved from the embedded built-in lock the same
/// way the shipped tool resolves it.
fn skill_builtin() -> Contract {
    temper::builtin::contract("skill")
        .expect("the embedded skill floor should project")
        .expect("the skill floor is embedded")
}

/// A contract's decidable `(severity, predicate)` vector, in declaration order —
/// the structural pin, excluding the per-clause guidance/`source` prose (product
/// territory, asserted present elsewhere).
fn predicate_vector(contract: &Contract) -> Vec<(Severity, Predicate)> {
    contract
        .clauses
        .iter()
        .map(|clause| (clause.severity, clause.predicate.clone()))
        .collect()
}

/// The `[a-z0-9-]` slug charset the `name` `allowed_chars` clause declares.
fn slug_charset() -> Charset {
    Charset {
        ranges: vec![('a', 'z'), ('0', '9')],
        chars: BTreeSet::from(['-']),
    }
}

/// The decidable clause vector the shipped skill built-in must carry, in declaration
/// order: name required/non-empty/charset/length/deny + matches-dir; description
/// required/non-empty/length; the optional `compatibility` cap; the advisory body
/// budget; the forbidden Cursor keys.
fn expected_skill_clauses() -> Vec<(Severity, Predicate)> {
    vec![
        (
            Severity::Required,
            Predicate::Required {
                field: "name".to_string(),
            },
        ),
        (
            Severity::Required,
            Predicate::MinLen {
                field: "name".to_string(),
                min: 1,
            },
        ),
        (
            Severity::Required,
            Predicate::AllowedChars {
                field: "name".to_string(),
                charset: slug_charset(),
            },
        ),
        (
            Severity::Required,
            Predicate::MaxLen {
                field: "name".to_string(),
                max: 64,
            },
        ),
        (
            Severity::Required,
            Predicate::Deny {
                field: "name".to_string(),
                values: vec!["anthropic".to_string(), "claude".to_string()],
            },
        ),
        (Severity::Required, Predicate::NameMatchesDir),
        (
            Severity::Required,
            Predicate::Required {
                field: "description".to_string(),
            },
        ),
        (
            Severity::Required,
            Predicate::MinLen {
                field: "description".to_string(),
                min: 1,
            },
        ),
        (
            Severity::Required,
            Predicate::MaxLen {
                field: "description".to_string(),
                max: 1024,
            },
        ),
        (
            Severity::Required,
            Predicate::MaxLen {
                field: "compatibility".to_string(),
                max: 500,
            },
        ),
        (Severity::Advisory, Predicate::MaxLines { max: 500 }),
        (
            Severity::Required,
            Predicate::ForbiddenKeys {
                keys: vec!["globs".to_string(), "alwaysApply".to_string()],
            },
        ),
    ]
}

/// The embedded skill built-in carries exactly the decidable clause vector at its
/// declared severities, and its display name is its bare kind label, `skill`.
#[test]
fn skill_builtin_carries_the_decidable_clause_vector() {
    let contract = skill_builtin();
    assert_eq!(contract.name, "skill");
    assert_eq!(predicate_vector(&contract), expected_skill_clauses());
}

/// A built-in package's clauses are *cited* and carry guidance — each pairs a
/// `source` provenance of taste with the hover-sized why (`specs/architecture/10-contracts.md`,
/// "a built-in package's clauses ... it is the expected posture"). Pinning presence
/// (not text) keeps the update ritual honest — walk the clauses, re-check their
/// citations — without coupling the build test to product prose.
#[test]
fn every_skill_builtin_clause_is_guided_and_cited() {
    for clause in &skill_builtin().clauses {
        assert!(
            clause.guidance.is_some(),
            "a built-in clause must carry its guidance, got: {:?}",
            clause.predicate,
        );
        assert!(
            clause.source.is_some(),
            "a built-in clause must carry its source citation, got: {:?}",
            clause.predicate,
        );
    }
}

/// No dropped heuristic survives as a clause. Whole-vector equality above already
/// implies this, but pinning the predicate kind set explicitly documents the
/// registry-kill decision: the built-in encodes *only* decidable predicates — every
/// clause is a true/false fact over the artifact, never a semantic guess
/// (third-person / has-trigger / companion-refs were undecidable and stay prose
/// guidance, `specs/architecture/10-contracts.md`).
#[test]
fn skill_builtin_encodes_only_decidable_clauses() {
    let kinds: BTreeSet<&str> = skill_builtin()
        .clauses
        .iter()
        .map(|clause| clause.predicate.key())
        .collect();

    assert_eq!(
        kinds,
        BTreeSet::from([
            "required",
            "min_len",
            "max_len",
            "deny",
            "allowed_chars",
            "max_lines",
            "name-matches-dir",
            "forbidden_keys",
        ]),
        "the built-in must carry only its declared decidable predicates",
    );
}

/// Both shipped built-in packages are themselves admissible — they pass the second
/// green (`specs/architecture/10-contracts.md`, "Decision: the contract is itself checked —
/// admissibility"). Every embedded package parses (closed vocabulary + charset
/// ranges enforced at load) and carries no vacuous list clause, so
/// `engine::admissibility` returns no findings.
#[test]
fn the_shipped_built_in_packages_are_admissible() {
    let builtins = temper::builtin::contracts().expect("the embedded packages should parse");
    assert!(
        !builtins.is_empty(),
        "at least the skill and rule built-ins must be embedded"
    );
    for (name, contract) in &builtins {
        let diagnostics = engine::admissibility(contract);
        assert!(
            diagnostics.is_empty(),
            "{name} should be admissible, got: {diagnostics:?}",
        );
    }
}

/// A contract text carrying `guidance` on a field clause parses it onto the clause
/// (`specs/architecture/50-distribution.md`, "The gate at keystroke"), and it plays *no part* in
/// admissibility — it is advisory-only, never a gate input (`00-intent.md` law 3).
/// The same contract's `guidance` projects to the emitted schema's property
/// `description`, strictly beside the validation keywords and never mixed into them.
#[test]
fn guidance_parses_is_advisory_only_and_projects_to_description() {
    let toml = r#"
[[clause]]
severity = "required"
predicate = "max_len"
field = "name"
max = 64
guidance = "Keep the name short and slug-like."

[[clause]]
severity = "required"
predicate = "min_len"
field = "description"
min = 1
"#;
    let contract = Contract::parse(toml, Path::new("skill.toml")).unwrap();

    // Parses onto the clause; a clause without a `guidance` key carries `None`.
    assert_eq!(
        contract.clauses[0].guidance.as_deref(),
        Some("Keep the name short and slug-like.")
    );
    assert!(contract.clauses[1].guidance.is_none());

    // Advisory-only: guidance is not a gate input, so admissibility is unaffected —
    // the contract is exactly as admissible as it would be without it.
    assert!(
        engine::admissibility(&contract).is_empty(),
        "guidance must play no part in admissibility",
    );

    // Projects to the docs channel: `name`'s property carries both its validation
    // keyword and the `description`; the un-guided `description` field carries none.
    let json = schema::emit(&contract);
    assert_eq!(
        json["properties"]["name"]["description"],
        "Keep the name short and slug-like."
    );
    assert_eq!(json["properties"]["name"]["maxLength"], 64);
    assert!(
        json["properties"]["description"]
            .get("description")
            .is_none()
    );
}

/// Guidance is admitted by the closed-vocabulary parser without widening the gate:
/// a clause carrying `guidance` alongside an *unknown* predicate still fails to
/// load, so `guidance` is not an escape hatch — it rides beside the algebra, never
/// relaxes it.
#[test]
fn guidance_does_not_admit_an_unknown_predicate() {
    let toml = r#"
[[clause]]
severity = "required"
predicate = "word_count"
field = "description"
guidance = "should be concise"
"#;
    assert!(
        Contract::parse(toml, Path::new("c.toml")).is_err(),
        "guidance must not turn an unknown predicate into an admissible clause",
    );
}

/// A non-string `guidance` is a load error, mirroring every other mistyped clause
/// key — the docs channel is prose, never a structured value.
#[test]
fn a_non_string_guidance_is_a_load_error() {
    let toml = r#"
[[clause]]
severity = "advisory"
predicate = "max_lines"
max = 500
guidance = 42
"#;
    assert!(Contract::parse(toml, Path::new("c.toml")).is_err());
}

/// A clause may carry a `source` citation beside its `guidance` — the *provenance
/// of taste* a built-in package's clauses are expected to record (`specs/architecture/10-contracts.md`,
/// "Decision: a built-in package is named for its source, and cited to it"). The
/// loader parses and *preserves* it verbatim; a clause without one still loads,
/// carrying `None`. `source` is preserved metadata, not a predicate — admitting it
/// leaves admissibility untouched.
#[test]
fn source_parses_is_preserved_and_advisory_only() {
    let toml = r#"
[[clause]]
severity = "required"
predicate = "max_len"
field = "name"
max = 64
guidance = "Keep the name short and slug-like."
source = "https://docs.claude.com/skills#naming (retrieved 2026-07-01)"

[[clause]]
severity = "required"
predicate = "min_len"
field = "description"
min = 1
"#;
    let contract = Contract::parse(toml, Path::new("skill.toml")).unwrap();

    // Preserved verbatim on the clause that declares it; absent ⇒ `None`.
    assert_eq!(
        contract.clauses[0].source.as_deref(),
        Some("https://docs.claude.com/skills#naming (retrieved 2026-07-01)")
    );
    assert!(contract.clauses[1].source.is_none());

    // Preserved metadata, not a gate input: admitting `source` neither adds nor
    // relaxes any admissibility check.
    assert!(
        engine::admissibility(&contract).is_empty(),
        "source must play no part in admissibility",
    );
}

/// `source` rides *beside* the algebra, never widens it: a clause pairing `source`
/// with a genuinely unknown predicate still fails to load, so a stray key is no
/// escape hatch. And absent `source` (every clause on disk today) stays admissible.
#[test]
fn source_does_not_admit_a_stray_key_or_unknown_predicate() {
    // `source` is not a blanket "accept any key" — an unrelated stray key still rejects.
    let stray = r#"
[[clause]]
severity = "required"
predicate = "max_len"
field = "name"
max = 64
source = "https://example.test"
nonsense = "still rejected"
"#;
    assert!(
        Contract::parse(stray, Path::new("c.toml")).is_err(),
        "a stray key must still reject even when `source` is present",
    );

    // Nor does `source` launder an unknown predicate into an admissible clause.
    let unknown_predicate = r#"
[[clause]]
severity = "required"
predicate = "word_count"
field = "description"
source = "https://example.test"
"#;
    assert!(
        Contract::parse(unknown_predicate, Path::new("c.toml")).is_err(),
        "source must not turn an unknown predicate into an admissible clause",
    );
}

/// A non-string `source` is a load error, mirroring `guidance` and every other
/// mistyped clause key — the citation channel is prose, never a structured value.
#[test]
fn a_non_string_source_is_a_load_error() {
    let toml = r#"
[[clause]]
severity = "advisory"
predicate = "max_lines"
max = 500
source = 42
"#;
    assert!(Contract::parse(toml, Path::new("c.toml")).is_err());
}
