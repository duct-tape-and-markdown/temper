//! Pins the shipped Anthropic skill template (`specs/10-contracts.md`,
//! "Templates — best practices as data").
//!
//! `contracts/skill.anthropic.toml` is the std-lib skill contract — curated,
//! human-authored guidance the build phase *embeds* but never writes. This test
//! loads it through [`Contract::load`] (the real on-disk path, not an inline
//! fixture) and pins the exact clause vector it deserializes into.
//!
//! Whole-vector equality is the point: it proves both halves of the
//! registry-kill decision at once. The surviving *decidable* clauses are present
//! at their declared severities, and the *dropped* heuristics
//! (third-person, has-trigger, companion-refs — semantic or weak proxies, out of
//! the closed algebra entirely) are absent, because any extra or missing clause
//! breaks the equality. The template carries no internal `name`, so its display
//! label must derive to `skill.anthropic` from the file stem
//! (`specs/10-contracts.md`: a contract is identified by its path, not a name).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use temper::contract::{Charset, Clause, Contract, Predicate, Severity};
use temper::engine;
use temper::extract::Features;
use temper::schema;

/// The typed model `contracts/skill.anthropic.toml` must deserialize into — the
/// surviving decidable clauses, in declaration order, each at the severity the
/// template author declared. Mirrors the data file clause-for-clause.
fn expected_template() -> Contract {
    Contract {
        // No top-level `name` in the data file: the label derives from the stem.
        name: "skill.anthropic".to_string(),
        guidance: None,
        clauses: vec![
            // name: required, non-empty, charset, length cap, reserved words,
            // matches its directory.
            Clause {
                source: None,
                severity: Severity::Required,
                guidance: None,
                predicate: Predicate::Required {
                    field: "name".to_string(),
                },
            },
            Clause {
                source: None,
                severity: Severity::Required,
                guidance: None,
                predicate: Predicate::MinLen {
                    field: "name".to_string(),
                    min: 1,
                },
            },
            Clause {
                source: None,
                severity: Severity::Required,
                guidance: None,
                predicate: Predicate::AllowedChars {
                    field: "name".to_string(),
                    charset: Charset {
                        ranges: vec![('a', 'z'), ('0', '9')],
                        chars: BTreeSet::from(['-']),
                    },
                },
            },
            Clause {
                source: None,
                severity: Severity::Required,
                guidance: None,
                predicate: Predicate::MaxLen {
                    field: "name".to_string(),
                    max: 64,
                },
            },
            Clause {
                source: None,
                severity: Severity::Required,
                guidance: None,
                predicate: Predicate::Deny {
                    field: "name".to_string(),
                    values: vec!["anthropic".to_string(), "claude".to_string()],
                },
            },
            Clause {
                source: None,
                severity: Severity::Required,
                guidance: None,
                predicate: Predicate::NameMatchesDir,
            },
            // description: required, non-empty, length cap.
            Clause {
                source: None,
                severity: Severity::Required,
                guidance: None,
                predicate: Predicate::Required {
                    field: "description".to_string(),
                },
            },
            Clause {
                source: None,
                severity: Severity::Required,
                guidance: None,
                predicate: Predicate::MinLen {
                    field: "description".to_string(),
                    min: 1,
                },
            },
            Clause {
                source: None,
                severity: Severity::Required,
                guidance: None,
                predicate: Predicate::MaxLen {
                    field: "description".to_string(),
                    max: 1024,
                },
            },
            // body: progressive-disclosure budget — advisory, recommend not gate.
            Clause {
                source: None,
                severity: Severity::Advisory,
                guidance: None,
                predicate: Predicate::MaxLines { max: 500 },
            },
            // no Cursor frontmatter keys Claude Code ignores.
            Clause {
                source: None,
                severity: Severity::Required,
                guidance: None,
                predicate: Predicate::ForbiddenKeys {
                    keys: vec!["globs".to_string(), "alwaysApply".to_string()],
                },
            },
        ],
    }
}

/// A shipped contract path, resolved off the crate root so the test is
/// cwd-independent (`.claude/rules/rust.md`, mirroring `tests/rules.rs`).
fn contract_path(relative: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

/// The path to the shipped skill template.
fn template_path() -> std::path::PathBuf {
    contract_path("contracts/skill.anthropic.toml")
}

/// The shipped template loads into the Contract model carrying exactly the
/// surviving decidable clause vector — name format/length/deny + name-matches-dir,
/// description presence/length, the advisory body budget, the forbidden keys —
/// at their declared severities, with the display label derived from the stem.
#[test]
fn shipped_template_loads_into_the_decidable_clause_vector() {
    let contract = Contract::load(&template_path()).expect("the shipped template should load");
    assert_eq!(contract, expected_template());
}

/// No dropped heuristic survives as a clause. Whole-vector equality above already
/// implies this, but pinning it explicitly documents the registry-kill decision:
/// the template encodes *only* decidable predicates — every clause is a true/false
/// fact over the artifact, never a semantic guess (third-person / has-trigger /
/// companion-refs were undecidable and stay prose guidance, `specs/10-contracts.md`).
#[test]
fn template_encodes_only_decidable_clauses() {
    let contract = Contract::load(&template_path()).expect("the shipped template should load");

    // Every predicate the template uses is a member of the closed algebra — there
    // is no syntax for an undecidable clause, so a survivor would have to be one
    // of these decidable kinds. Asserting the kind set keeps the intent legible.
    let kinds: BTreeSet<&str> = contract
        .clauses
        .iter()
        .map(|clause| match &clause.predicate {
            Predicate::Required { .. } => "required",
            Predicate::Optional { .. } => "optional",
            Predicate::Type { .. } => "type",
            Predicate::MinLen { .. } => "min_len",
            Predicate::MaxLen { .. } => "max_len",
            Predicate::Range { .. } => "range",
            Predicate::Enum { .. } => "enum",
            Predicate::Deny { .. } => "deny",
            Predicate::ForbiddenKeys { .. } => "forbidden_keys",
            Predicate::AllowedChars { .. } => "allowed_chars",
            Predicate::MaxLines { .. } => "max_lines",
            Predicate::RequireSections { .. } => "require_sections",
            Predicate::MustDefine { .. } => "must_define",
            Predicate::NameMatchesDir => "name-matches-dir",
            Predicate::UniqueName => "unique-name",
            Predicate::DependencyExists => "dependency-exists",
        })
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
        "the template must carry only its declared decidable predicates",
    );
}

/// Both curated built-in contracts are themselves admissible — they pass the
/// second green (`specs/10-contracts.md`, "Decision: the contract is itself
/// checked — admissibility"). They load without error (closed vocabulary +
/// charset ranges are enforced there) and carry no vacuous list clause, so
/// `engine::admissibility` returns no findings.
#[test]
fn the_shipped_built_in_contracts_are_admissible() {
    for relative in ["contracts/skill.anthropic.toml", "contracts/rule.toml"] {
        let contract =
            Contract::load(&contract_path(relative)).expect("the shipped contract should load");
        let diagnostics = engine::admissibility(&contract);
        assert!(
            diagnostics.is_empty(),
            "{relative} should be admissible, got: {diagnostics:?}",
        );
    }
}

/// The shipped templates carry no `guidance` today — the docs channel is authored
/// separately (a human `chore(harness)`, since `contracts/` is not build-writable),
/// so every clause parses with `guidance: None`. Pinning this documents that the
/// mechanism is live but the prose is not yet authored: absent guidance ⇒ no
/// `description` in the emitted schema.
#[test]
fn the_shipped_templates_carry_no_guidance_yet() {
    for relative in ["contracts/skill.anthropic.toml", "contracts/rule.toml"] {
        let contract =
            Contract::load(&contract_path(relative)).expect("the shipped contract should load");
        assert!(
            contract
                .clauses
                .iter()
                .all(|clause| clause.guidance.is_none()),
            "{relative} carries no guidance until it is human-authored",
        );
    }
}

/// A contract text carrying `guidance` on a field clause parses it onto the clause
/// (`specs/50-distribution.md`, "The gate at keystroke"), and it plays *no part* in
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
/// of taste* a built-in package's clauses are expected to record (`specs/10-contracts.md`,
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

// --- PACKAGE.md — a package authored as one fenced document -------------------
//
// A package is authored the same way as any member (`specs/20-surface.md`): one
// `PACKAGE.md` whose fenced header carries the `[[clause]]` tables and whose body
// is the package-level guidance. It loads straight into the [`Contract`] model
// (`specs/10-contracts.md`, "Packages" — the resolved PACKAGE-MODEL-RECONCILE
// fold), beside the embedded `contracts/*.toml` floor. These cases grow the
// TOML-file ones above to the document form.

/// A fixture package under `tests/fixtures/.temper/packages/<name>/`, resolved off
/// the crate root so the test is cwd-independent.
fn package_path(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/.temper/packages")
        .join(name)
        .join("PACKAGE.md")
}

/// A `PACKAGE.md` whose header clauses mirror `contracts/skill.anthropic.toml`
/// loads through the document primitive into the *same* clause vector the TOML
/// floor deserializes into — the two authoring forms decide identically, because
/// both parse the same closed-vocabulary header through [`temper::contract`].
#[test]
fn a_package_document_loads_the_same_clauses_as_the_toml_floor() {
    let package =
        Contract::load_package(&package_path("skill.anthropic")).expect("the package should load");
    let toml_floor = Contract::load(&template_path()).expect("the floor should load");

    // Identical clauses ⇒ identical decisions over any surface: the medium changed
    // (fenced document vs bare `.toml`), the algebra did not.
    assert_eq!(package.clauses, toml_floor.clauses);

    // And the package is admissible for the same reason the floor is — it carries
    // only closed-vocabulary clauses with no vacuous list.
    assert!(
        engine::admissibility(&package).is_empty(),
        "the package must pass admissibility beside the floor",
    );
}

/// A package is identified by *where it lives*: its display name derives from the
/// containing directory's stem, never an internal `name` field
/// (`specs/10-contracts.md`, "Decision: a package is identified by its binding").
#[test]
fn a_package_display_name_is_its_directory_stem() {
    let package =
        Contract::load_package(&package_path("skill.anthropic")).expect("the package should load");
    assert_eq!(package.name, "skill.anthropic");
}

/// The document body is the package-level guidance channel — the always-on prose
/// the clauses cannot encode, carried verbatim onto the [`Contract`]. It never
/// gates (admissibility above is unaffected); it is the authoring agent's channel.
#[test]
fn a_package_body_is_carried_as_package_level_guidance() {
    let package =
        Contract::load_package(&package_path("skill.anthropic")).expect("the package should load");
    let guidance = package
        .guidance
        .as_deref()
        .expect("the package body is its guidance");
    assert!(guidance.contains("Anthropic skill package"));
    assert!(guidance.contains("hover docs"));
}

/// An unknown predicate in a package header fails to load — the same closed-
/// vocabulary rejection the TOML floor makes, one medium over. A package reaches
/// the definition check through the document primitive exactly as a bare contract
/// does; the trapdoor stays shut.
#[test]
fn an_unknown_predicate_in_a_package_header_fails_to_load() {
    let err = Contract::load_package(&package_path("bad-predicate")).unwrap_err();
    assert!(
        matches!(err, temper::contract::ContractError::UnknownPredicate { ref predicate, .. } if predicate == "word_count"),
        "an out-of-vocabulary package clause must be rejected at load, got: {err:?}",
    );
}

/// A malformed fenced document (no closing `+++`) is a load error surfaced through
/// the document primitive — distinct from a `.toml` parse error, because a package
/// is authored in the surface's fenced medium.
#[test]
fn a_malformed_package_document_is_a_load_error() {
    let err = Contract::parse_package(
        "+++\n[[clause]]\nno closing fence\n",
        Path::new("PACKAGE.md"),
    )
    .unwrap_err();
    assert!(matches!(
        err,
        temper::contract::ContractError::PackageDocument { .. }
    ));
}

/// A failing clause's colocated `guidance` rides its diagnostic — the just-in-time
/// teaching moment (`specs/10-contracts.md`, "Packages"). The `guided` package's
/// `required name` clause carries guidance; a member missing `name` trips it, and
/// the emitted [`Diagnostic`](temper::check::Diagnostic) carries that guidance so
/// the violation teaches. A clause is a gate; its guidance is never one — the
/// prose explains the finding, it did not decide it.
#[test]
fn a_failing_clause_diagnostic_carries_its_colocated_guidance() {
    let package =
        Contract::load_package(&package_path("guided")).expect("the guided package should load");

    // A member with no `name` frontmatter field: the `required name` clause fails.
    let member = Features {
        id: "nameless".to_string(),
        fields: BTreeMap::new(),
        body_lines: 3,
        headings: Vec::new(),
        source_dir: Some("nameless".to_string()),
        satisfies: Vec::new(),
    };

    let diagnostics = engine::validate(&package, std::slice::from_ref(&member));
    let required = diagnostics
        .iter()
        .find(|diagnostic| diagnostic.rule == "required")
        .expect("the missing name must fire the `required` clause");
    assert_eq!(
        required.guidance.as_deref(),
        Some("Every skill declares a `name` — it is the slug the harness binds to."),
        "the diagnostic must carry the clause's colocated guidance",
    );
}
