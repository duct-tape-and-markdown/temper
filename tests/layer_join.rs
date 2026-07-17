//! The policy layer the invocation joins: `check --layer <lock>`.
//!
//! An org authors an ordinary temper corpus, emits its lock, and distributes it; a
//! consumer names it at check and is gated by it. The lock is the whole interchange —
//! there is no second format, no payload as input, and no trust model: whoever owns the
//! invocation owns the top of the layer stack.
//!
//! Four faces of the one join:
//!
//! - **join** — the named lock's clause rows range over *this* corpus's selections, keyed
//!   by kind name, and the run gates on them.
//! - **fail-closed** — a joined lock that fails admissibility fails the check, whether or
//!   not this corpus declares the kind its clauses name; a layer that was named and could
//!   not be read refuses rather than gating nothing.
//! - **hardening binds** — a joined clause the host declares no counterpart to is an
//!   ordinary clause of the kind it names.
//! - **softening is inert** — a joined clause cannot displace the host's reviewed one, so
//!   an attempt to weaken the gate reports beside the clause it failed to replace and
//!   changes no verdict.

use std::fs;
use std::path::{Path, PathBuf};

use temper::drift::{BoundRow, ClauseRow, Declarations};

mod common;

/// The host corpus every case gates: one floor-clean skill whose description is far
/// longer than the bound the layers below declare, under a lock declaring nothing of its
/// own. Clean on its own — so a finding here is the layer's, and only the layer's.
fn host(slug: &str) -> PathBuf {
    let harness = common::tmpdir(slug);
    common::write_skill(&harness, "packing", &common::clean_skill("packing"));
    common::write_lock(&harness, Declarations::default());
    harness
}

/// An org corpus carrying `clauses` on its emitted lock — the artifact it distributes.
/// Returns the org root; its lock sits at the ordinary `.temper/lock.toml` under it.
fn org(slug: &str, clauses: Vec<ClauseRow>) -> PathBuf {
    let org = common::tmpdir(slug);
    common::write_lock(
        &org,
        Declarations {
            clauses,
            ..Default::default()
        },
    );
    org
}

/// The lock file inside an org corpus — the path a consumer names.
fn lock_of(org: &Path) -> PathBuf {
    org.join(".temper").join("lock.toml")
}

/// A `max_len` clause on `skill`'s `description`, bounded far under any real
/// description's length, at `severity` — the one policy every case here states, so what
/// varies between them is the join and never the check.
fn description_bound(severity: &str) -> ClauseRow {
    ClauseRow {
        kind: Some("skill".to_string()),
        field: Some("description".to_string()),
        bound: Some(BoundRow {
            min: None,
            max: Some(10),
        }),
        ..common::clause("max_len", severity)
    }
}

/// Run `check` over `harness` under the github reporter, joining each of `layers`.
fn check_joining(harness: &Path, layers: &[&Path]) -> (Vec<String>, bool) {
    let mut args = vec!["--harness".to_string(), harness.display().to_string()];
    for layer in layers {
        args.push("--layer".to_string());
        args.push(layer.display().to_string());
    }
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let run = common::check_in(harness, &args, Some("github"));
    (run.findings(), run.ok)
}

/// The finding lines whose rule carries `needle` — a joined clause's address ends in the
/// layer that produced it, which no `title=<rule>::` equality can spell.
fn findings_naming<'a>(findings: &'a [String], needle: &str) -> Vec<&'a String> {
    findings
        .iter()
        .filter(|line| line.contains(needle))
        .collect()
}

#[test]
fn a_joined_lock_gates_the_host_corpus_and_an_unjoined_one_does_not() {
    let harness = host("layer-join");
    let org = org("layer-join-org", vec![description_bound("required")]);

    // The consumer's own corpus is clean: nothing here bounds a description's length.
    let (_, ok) = check_joining(&harness, &[]);
    assert!(ok, "the host corpus gates green on its own");

    // Naming the org's lock joins its clause over this corpus's own skills.
    let (findings, ok) = check_joining(&harness, &[&lock_of(&org)]);
    assert!(!ok, "a joined required clause gates the run: {findings:?}");
    let joined = findings_naming(&findings, "skill.max_len.description@");
    assert_eq!(
        joined.len(),
        1,
        "the joined clause fires over the host's own skill: {findings:?}"
    );
    assert!(
        joined[0].starts_with("::error"),
        "a joined `required` clause is required here too: {joined:?}"
    );
    assert!(
        joined[0].contains("packing"),
        "the finding names the host's member, not the layer's: {joined:?}"
    );
}

#[test]
fn a_layer_is_named_as_its_lock_file_or_as_the_corpus_holding_it() {
    let harness = host("layer-spelling");
    let org = org("layer-spelling-org", vec![description_bound("required")]);

    // The lock is the interchange; naming the directory that holds one reaches the same
    // lock, so a consumer handed a whole org corpus needs no path lore to join it.
    let (by_file, file_ok) = check_joining(&harness, &[&lock_of(&org)]);
    let (by_dir, dir_ok) = check_joining(&harness, &[&org.join(".temper")]);

    assert!(!file_ok && !dir_ok, "both spellings gate: {by_file:?}");
    assert_eq!(
        by_file.len(),
        by_dir.len(),
        "both spellings join the same lock: {by_file:?} vs {by_dir:?}"
    );
}

#[test]
fn a_joined_clause_that_would_soften_the_hosts_gate_is_inert_and_still_visible() {
    // This host declares the bound itself, reviewed and required — the committed gate a
    // layer is about to try to talk down.
    let harness = host("layer-soften");
    common::write_lock(
        &harness,
        Declarations {
            clauses: vec![description_bound("required")],
            ..Default::default()
        },
    );
    let org = org("layer-soften-org", vec![description_bound("advisory")]);

    let (findings, ok) = check_joining(&harness, &[&lock_of(&org)]);

    // The layer states the same clause at a weaker severity. It cannot displace the
    // host's — review is the price of softening — so the host's error stands and the run
    // still fails.
    assert!(
        !ok,
        "a joined layer never talks the host's gate down: {findings:?}"
    );
    let host_finding = common::findings_for(&findings, "skill.max_len.description");
    assert_eq!(
        host_finding.len(),
        1,
        "the host's own clause still fires: {findings:?}"
    );
    assert!(
        host_finding[0].starts_with("::error"),
        "at the severity the host reviewed: {host_finding:?}"
    );

    // Inert on the verdict, but never silent: the layer's own clause reports at the
    // severity the layer declared, so what it asked for is visible rather than swallowed.
    let joined = findings_naming(&findings, "skill.max_len.description@");
    assert_eq!(joined.len(), 1, "the joined clause reports: {findings:?}");
    assert!(
        joined[0].starts_with("::warning"),
        "at its own advisory severity: {joined:?}"
    );
}

#[test]
fn two_layers_declaring_one_clause_each_keep_their_own_addresses() {
    let harness = host("layer-two");
    let first = org("layer-two-a", vec![description_bound("required")]);
    let second = org("layer-two-b", vec![description_bound("advisory")]);

    let (findings, ok) = check_joining(&harness, &[&lock_of(&first), &lock_of(&second)]);

    // Two locks compiled the same address in their own corpora. Each joined clause is
    // addressed under the layer that carried it, so neither collides with the other and
    // both judge — a clause's address stays the name its own finding prints.
    assert!(!ok, "the required layer gates: {findings:?}");
    let joined = findings_naming(&findings, "skill.max_len.description@");
    assert_eq!(joined.len(), 2, "both layers' clauses fire: {findings:?}");
    assert_eq!(
        joined.iter().filter(|f| f.starts_with("::error")).count(),
        1,
        "each at the severity its own layer declared: {joined:?}"
    );
}

#[test]
fn one_lock_named_twice_joins_once() {
    let harness = host("layer-repeat");
    let org = org("layer-repeat-org", vec![description_bound("required")]);
    let lock = lock_of(&org);

    let (findings, ok) = check_joining(&harness, &[&lock, &lock]);

    // The same lock is the same layer. Joining it twice would collide its every address
    // with its own copy and refuse the run as malformed — a benign repetition is not a
    // malformed layer.
    assert!(!ok, "the layer still gates: {findings:?}");
    assert_eq!(
        findings_naming(&findings, "skill.max_len.description@").len(),
        1,
        "the repeated lock joins once: {findings:?}"
    );
    assert!(
        common::findings_for(&findings, "clause.label-collision").is_empty(),
        "and collides with nothing: {findings:?}"
    );
}

#[test]
fn a_joined_lock_that_fails_admissibility_fails_the_check() {
    let harness = host("layer-inadmissible");
    // An `enum` over no values admits nothing — a vacuous clause, and a malformed lock
    // wherever it was compiled.
    let org = org(
        "layer-inadmissible-org",
        vec![ClauseRow {
            kind: Some("skill".to_string()),
            field: Some("status".to_string()),
            values: Some(Vec::new()),
            ..common::clause("enum", "required")
        }],
    );

    let (findings, ok) = check_joining(&harness, &[&lock_of(&org)]);

    assert!(!ok, "a malformed layer fails the check: {findings:?}");
    let joined = findings_naming(&findings, "skill.enum.status@");
    assert_eq!(joined.len(), 1, "loud, naming the clause: {findings:?}");
    assert!(
        joined[0].starts_with("::error"),
        "admissibility findings are errors: {joined:?}"
    );
}

#[test]
fn a_joined_lock_fails_admissibility_even_where_this_corpus_selects_nothing() {
    let harness = host("layer-inadmissible-unknown");
    // The clause names a kind this corpus declares none of, so nothing here selects it.
    // Fail-closed does not turn on whether the host happened to give a layer something to
    // range over: a malformed layer is malformed either way.
    let org = org(
        "layer-inadmissible-unknown-org",
        vec![ClauseRow {
            kind: Some("ritual".to_string()),
            field: Some("status".to_string()),
            values: Some(Vec::new()),
            ..common::clause("enum", "required")
        }],
    );

    let (findings, ok) = check_joining(&harness, &[&lock_of(&org)]);

    assert!(!ok, "a malformed layer fails the check: {findings:?}");
    assert_eq!(
        findings_naming(&findings, "ritual.enum.status@").len(),
        1,
        "loud, though no member of `ritual` exists here: {findings:?}"
    );
}

#[test]
fn a_layer_that_cannot_be_read_refuses_rather_than_gating_nothing() {
    let harness = host("layer-absent");
    let absent = harness.join("no-such-org").join("lock.toml");

    let run = common::check_in(
        harness.as_path(),
        &[
            "--harness",
            harness.to_str().unwrap(),
            "--layer",
            absent.to_str().unwrap(),
        ],
        None,
    );

    // A layer the invocation named and that is not there gated nothing — the one outcome
    // fail-closed forbids. (The host's own lock is the opposite case: an unadopted harness
    // legitimately has none, so that read tolerates absence and this one cannot.)
    assert!(!run.ok, "an unreadable layer refuses: {}", run.output);
    assert!(
        run.output.contains("layer") && run.output.contains("lock.toml"),
        "naming what could not be read: {}",
        run.output
    );
}

#[test]
fn a_joined_clause_binds_to_a_custom_kind_by_name() {
    let harness = common::tmpdir("layer-custom-kind");
    fs::create_dir_all(harness.join(".claude").join("playbooks")).unwrap();
    common::write_sibling(
        &harness,
        ".claude/playbooks/rollback.md",
        "---\nsummary: Roll the deploy back to the last green tag.\n---\n\nBody.\n",
    );
    // The host declares the kind; the kind is all it declares. A layer's clauses range
    // over the host's own selections, so what it can reach is what this corpus models.
    common::write_lock(
        &harness,
        Declarations {
            kinds: vec![common::kind_facts("playbook", ".claude/playbooks", "*.md")],
            ..Default::default()
        },
    );
    let org = org(
        "layer-custom-kind-org",
        vec![ClauseRow {
            kind: Some("playbook".to_string()),
            field: Some("owner".to_string()),
            ..common::clause("required", "required")
        }],
    );

    let (findings, ok) = check_joining(&harness, &[&lock_of(&org)]);

    // Kinds travel by name: the org never saw this harness's playbooks, and its clause
    // judges them anyway.
    assert!(!ok, "the joined clause gates the custom kind: {findings:?}");
    let joined = findings_naming(&findings, "playbook.required.owner@");
    assert_eq!(joined.len(), 1, "over the host's own member: {findings:?}");
    assert!(joined[0].contains("rollback"), "naming it: {joined:?}");
}

#[test]
fn a_layer_never_tips_a_builtin_off_its_embedded_default() {
    // This host declares no clause of its own, so `skill` gates on the embedded default
    // contract — the floor a real harness stands on.
    let harness = host("layer-embedded-default");
    common::write_skill(
        &harness,
        "broken",
        "---\nname: WRONG\ndescription: Use when a name disagrees with its directory.\n---\n\nBody.\n",
    );
    let org = org(
        "layer-embedded-default-org",
        vec![description_bound("required")],
    );

    let (with_layer, _) = check_joining(&harness, &[&lock_of(&org)]);
    let (without_layer, _) = check_joining(&harness, &[]);

    // A layer's row is not this harness declaring one. Were the two folded into one set,
    // the layer's single clause would read as "the lock declares skill's contract" and the
    // embedded default would fall away — the whole floor softened by a layer that only
    // ever asked to add to it.
    let floor =
        |findings: &[String]| common::findings_for(findings, "skill.name-matches-dir").len();
    assert_eq!(
        floor(&with_layer),
        floor(&without_layer),
        "the embedded default still judges under a layer: {with_layer:?} vs {without_layer:?}"
    );
    assert_eq!(floor(&with_layer), 1, "and it fires: {with_layer:?}");
}
