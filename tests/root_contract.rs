//! The root member's own contract: the clause rows that name no kind.
//!
//! Three facts hold this scope together. A top-level clause row absent its `kind` column
//! lifts into the **root** contract where a kind-named row does not; the root selection
//! ranges over every discovered member of every kind; and a root clause addresses as
//! `root.<predicate>` — the label its findings report under and the one an author spells
//! back into a dial entry. `reachable` is the consumer the mechanism ships with: opt-in,
//! so a lock naming no root row asks the graph nothing.

use std::path::Path;

use temper::compose;
use temper::contract::{Predicate, Severity};
use temper::drift::{ClauseRow, Declarations};

mod common;

/// A rule scoped to a glob no file in the harness matches — a `paths-match` registration
/// channel that is provably dead, so the harness activates the rule never.
const UNMATCHED_GLOB: &str = "never/**/*.xyz";

/// The root member's `reachable` clause row at `severity`. `kind` stays `None`: that
/// absence at the top level is what makes the row the root's. Payload-shaped — `label`
/// is emit's to stamp.
fn root_reachable(severity: &str) -> ClauseRow {
    common::clause("reachable", severity)
}

/// A lock-shaped copy of `row`, its address spelled — the shape the lift reads, since a
/// row emit never stamped is a row emit never wrote.
fn stamped(label: &str, row: ClauseRow) -> ClauseRow {
    ClauseRow {
        label: Some(label.to_string()),
        ..row
    }
}

/// A harness whose one rule is dead — its `paths` glob matches no file — plus a
/// floor-clean skill, so the corpus the root selection ranges over is more than the one
/// member under test.
fn dead_registration_harness(slug: &str) -> std::path::PathBuf {
    let root = common::scaffold(slug);
    common::write_sibling(
        &root,
        ".claude/rules/style.md",
        &common::scoped_rule(Some(UNMATCHED_GLOB)),
    );
    common::write_skill(&root, "standards", &common::clean_skill("standards"));
    root
}

/// The label emit stamped onto the lock's one root clause row, read back off disk and
/// lifted through the root contract — so a case asserting on a finding's `rule` id
/// asserts on the address the pipeline actually compiled, never a literal beside it.
fn locked_root_label(root: &Path) -> String {
    let declarations = temper::drift::read_declarations(&root.join(".temper")).unwrap();
    let contract = compose::root_contract_from_rows(&declarations.clauses).unwrap();
    assert_eq!(
        contract.clauses.len(),
        1,
        "the lock declares exactly one root clause, got {:?}",
        contract.clauses
    );
    contract.clauses[0].label.clone()
}

#[test]
fn a_kind_less_top_level_row_lifts_into_the_root_contract_and_a_kind_named_row_does_not() {
    // The discriminator is the `kind` column alone: the root's row leaves it absent, the
    // skill's names a kind, and the lift keeps exactly the first.
    let rows = vec![
        stamped("root.reachable", root_reachable("advisory")),
        stamped(
            "skill.required.description",
            ClauseRow {
                kind: Some("skill".to_string()),
                field: Some("description".to_string()),
                ..common::clause("required", "required")
            },
        ),
    ];

    let root = compose::root_contract_from_rows(&rows).unwrap();
    assert_eq!(root.name, temper::contract::ROOT_OWNER);
    assert_eq!(root.clauses.len(), 1, "got {:?}", root.clauses);
    assert_eq!(root.clauses[0].predicate, Predicate::Reachable);
    assert_eq!(root.clauses[0].severity, Severity::Advisory);

    // The same rows through the by-kind lift: the root's row is invisible to it, exactly
    // as the skill's row is invisible above.
    let skill = compose::default_contract_from_rows(&rows, &[], "skill").unwrap();
    assert_eq!(skill.clauses.len(), 1, "got {:?}", skill.clauses);
    assert_eq!(
        skill.clauses[0].predicate,
        Predicate::Required {
            field: "description".to_string()
        }
    );
}

#[test]
fn a_root_reachable_clause_reports_a_dead_registration_under_its_own_label() {
    let root = dead_registration_harness("root-reachable-fires");
    common::write_lock(
        &root,
        Declarations {
            clauses: vec![root_reachable("advisory")],
            ..Declarations::default()
        },
    );

    let label = locked_root_label(&root);
    assert_eq!(
        label, "root.reachable",
        "a root clause addresses under the root owner segment"
    );

    let (findings, ok) = common::check_harness(&root);
    let reported = common::findings_for(&findings, &label);
    assert_eq!(
        reported.len(),
        1,
        "the dead `paths-match` channel is one finding under `{label}`, got:\n{findings:#?}"
    );
    assert!(
        reported[0].contains("style") && reported[0].contains("world"),
        "the finding names the dead member and the world node it is unreached from, got: {}",
        reported[0]
    );
    assert!(
        reported[0].starts_with("::warning"),
        "the clause declared `advisory`, so the finding is a warning, got: {}",
        reported[0]
    );
    assert!(
        ok,
        "and an advisory finding does not fail the run, got:\n{findings:#?}"
    );
}

#[test]
fn a_root_reachable_clause_gates_at_the_severity_its_author_declared() {
    // The tool never decides error-versus-warning: the same dead registration under a
    // `required` root clause fails the run.
    let root = dead_registration_harness("root-reachable-required");
    common::write_lock(
        &root,
        Declarations {
            clauses: vec![root_reachable("required")],
            ..Declarations::default()
        },
    );

    let (findings, ok) = common::check_harness(&root);
    let reported = common::findings_for(&findings, "root.reachable");
    assert_eq!(reported.len(), 1, "got:\n{findings:#?}");
    assert!(
        reported[0].starts_with("::error"),
        "a `required` root clause blocks, got: {}",
        reported[0]
    );
    assert!(!ok, "and the run fails, got:\n{findings:#?}");
}

#[test]
fn the_dial_reaches_a_root_clause_by_its_label() {
    // The dial's site is the loop over selections, so the root selection reaches it with
    // no second dial site: the same `required` clause read at `advisory` on this machine.
    let root = dead_registration_harness("root-reachable-dialed");
    common::write_lock(
        &root,
        Declarations {
            clauses: vec![root_reachable("required")],
            ..Declarations::default()
        },
    );
    common::write_sibling(
        &root,
        ".temper/dial.toml",
        "name = \"workstation\"\n\n[[clause]]\nlabel = \"root.reachable\"\nseverity = \"advisory\"\n",
    );

    let (findings, ok) = common::check_harness(&root);
    let reported = common::findings_for(&findings, "root.reachable");
    assert_eq!(
        reported.len(),
        1,
        "a dialed clause still reports, got:\n{findings:#?}"
    );
    assert!(
        reported[0].starts_with("::warning"),
        "the dial softened the root clause to advisory, got: {}",
        reported[0]
    );
    assert!(
        ok,
        "so the dead registration no longer blocks, got:\n{findings:#?}"
    );
}

#[test]
fn a_lock_naming_no_root_row_falls_back_to_the_embedded_locks_root_rows() {
    // Rows-or-default, over the root member: a lock whose every clause row names a kind
    // declares no root contract of its own, so the embedded lock's root rows answer.
    let kind_named = vec![stamped(
        "skill.required.description",
        ClauseRow {
            kind: Some("skill".to_string()),
            field: Some("description".to_string()),
            ..common::clause("required", "required")
        },
    )];
    assert_eq!(
        compose::root_contract(&kind_named).unwrap(),
        temper::builtin::root_contract(),
        "a rowless root falls back to the embedded default, never to silence"
    );

    // And a root row displaces that default entirely, the same way a kind's rows do.
    let with_root = vec![stamped("root.reachable", root_reachable("advisory"))];
    assert_eq!(
        compose::root_contract(&with_root).unwrap(),
        compose::root_contract_from_rows(&with_root).unwrap()
    );
}

#[test]
fn a_harness_declaring_no_root_reachable_clause_asks_the_graph_nothing() {
    // `reachable` is opt-in exactly as `degree` and `mention-reachable` are: the same
    // dead registration is silent where no root clause binds. The cost half of that
    // claim — the closure itself never walking — is pinned in `tests/check_cost.rs`.
    let root = dead_registration_harness("root-reachable-opt-in");
    common::write_lock(&root, Declarations::default());

    let (findings, _) = common::check_harness(&root);
    assert!(
        common::findings_for(&findings, "root.reachable").is_empty(),
        "and the gate reports nothing under the root label, got:\n{findings:#?}"
    );
}
