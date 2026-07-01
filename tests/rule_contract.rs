//! Pins the shipped rule template (`specs/10-contracts.md`,
//! "Templates — best practices as data").
//!
//! `contracts/rule.toml` is the std-lib contract for the `rule` artifact kind
//! (`.claude/rules/*.md`) — curated, human-authored guidance the build phase
//! *embeds* (RULE-CHECK) but never writes. This test loads it through
//! [`Contract::load`] (the real on-disk path, not an inline fixture) and pins the
//! exact clause vector it deserializes into, mirroring `tests/contract_template.rs`
//! for the skill template.
//!
//! Whole-vector equality is the point: the rule template, like the skill one,
//! carries *only* decidable predicates and no semantic guess. The surviving
//! clauses — `optional` on `paths` (advisory), `forbidden_keys` (required),
//! `max_lines` (advisory) — are present at their declared severities, and any
//! extra or missing clause breaks the equality. The template carries no internal
//! `name`, so its display label must derive to `rule` from the file stem
//! (`specs/10-contracts.md`: a contract is identified by its path, not a name).

use std::collections::BTreeSet;
use std::path::Path;

use temper::contract::{Clause, Contract, Predicate, Severity};

/// The typed model `contracts/rule.toml` must deserialize into — its decidable
/// clauses, in declaration order, each at the severity the template author
/// declared. Mirrors the data file clause-for-clause.
fn expected_template() -> Contract {
    Contract {
        // No top-level `name` in the data file: the label derives from the stem.
        name: "rule".to_string(),
        guidance: None,
        clauses: vec![
            // `paths` is Claude Code's real scoping key — declared so it is part
            // of the known schema. `optional` always holds; it documents, never
            // fails, hence advisory.
            Clause {
                severity: Severity::Advisory,
                guidance: None,
                predicate: Predicate::Optional {
                    field: "paths".to_string(),
                },
            },
            // The Cursor `.mdc` keys Claude Code silently ignores — forbidding
            // them catches the inert-rule mistake `temper` exists to prevent, so
            // it is required (gate-blocking).
            Clause {
                severity: Severity::Required,
                guidance: None,
                predicate: Predicate::ForbiddenKeys {
                    keys: vec![
                        "description".to_string(),
                        "globs".to_string(),
                        "alwaysApply".to_string(),
                    ],
                },
            },
            // Rules are always-on context paid each tick — keep them lean.
            // Advisory, mirroring the CLAUDE.md <200-line guidance.
            Clause {
                severity: Severity::Advisory,
                guidance: None,
                predicate: Predicate::MaxLines { max: 200 },
            },
        ],
    }
}

/// The path to the shipped template, resolved off the crate root so the test is
/// cwd-independent (mirroring `tests/contract_template.rs`).
fn template_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("contracts/rule.toml")
}

/// The shipped rule template loads into the Contract model carrying exactly the
/// optional-`paths` / forbidden-keys / max-lines clause vector at their declared
/// severities, with the display label derived from the file stem.
#[test]
fn shipped_template_loads_into_the_decidable_clause_vector() {
    let contract = Contract::load(&template_path()).expect("the shipped rule template should load");
    assert_eq!(contract, expected_template());
}

/// No undecidable clause survives. Whole-vector equality above already implies
/// this, but pinning the predicate kind set explicitly documents that the rule
/// template — like the skill one — encodes *only* decidable predicates: every
/// clause is a true/false fact over the artifact, never a semantic guess
/// (`specs/10-contracts.md`, "best practices as data").
#[test]
fn template_encodes_only_decidable_clauses() {
    let contract = Contract::load(&template_path()).expect("the shipped rule template should load");

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
        BTreeSet::from(["optional", "forbidden_keys", "max_lines"]),
        "the rule template must carry only its declared decidable predicates",
    );
}
