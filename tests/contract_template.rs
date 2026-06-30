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

use std::collections::BTreeSet;
use std::path::Path;

use temper::contract::{Charset, Clause, Contract, Predicate, Severity};
use temper::engine;

/// The typed model `contracts/skill.anthropic.toml` must deserialize into — the
/// surviving decidable clauses, in declaration order, each at the severity the
/// template author declared. Mirrors the data file clause-for-clause.
fn expected_template() -> Contract {
    Contract {
        // No top-level `name` in the data file: the label derives from the stem.
        name: "skill.anthropic".to_string(),
        clauses: vec![
            // name: required, non-empty, charset, length cap, reserved words,
            // matches its directory.
            Clause {
                severity: Severity::Required,
                predicate: Predicate::Required {
                    field: "name".to_string(),
                },
            },
            Clause {
                severity: Severity::Required,
                predicate: Predicate::MinLen {
                    field: "name".to_string(),
                    min: 1,
                },
            },
            Clause {
                severity: Severity::Required,
                predicate: Predicate::AllowedChars {
                    field: "name".to_string(),
                    charset: Charset {
                        ranges: vec![('a', 'z'), ('0', '9')],
                        chars: BTreeSet::from(['-']),
                    },
                },
            },
            Clause {
                severity: Severity::Required,
                predicate: Predicate::MaxLen {
                    field: "name".to_string(),
                    max: 64,
                },
            },
            Clause {
                severity: Severity::Required,
                predicate: Predicate::Deny {
                    field: "name".to_string(),
                    values: vec!["anthropic".to_string(), "claude".to_string()],
                },
            },
            Clause {
                severity: Severity::Required,
                predicate: Predicate::NameMatchesDir,
            },
            // description: required, non-empty, length cap.
            Clause {
                severity: Severity::Required,
                predicate: Predicate::Required {
                    field: "description".to_string(),
                },
            },
            Clause {
                severity: Severity::Required,
                predicate: Predicate::MinLen {
                    field: "description".to_string(),
                    min: 1,
                },
            },
            Clause {
                severity: Severity::Required,
                predicate: Predicate::MaxLen {
                    field: "description".to_string(),
                    max: 1024,
                },
            },
            // body: progressive-disclosure budget — advisory, recommend not gate.
            Clause {
                severity: Severity::Advisory,
                predicate: Predicate::MaxLines { max: 500 },
            },
            // no Cursor frontmatter keys Claude Code ignores.
            Clause {
                severity: Severity::Required,
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
