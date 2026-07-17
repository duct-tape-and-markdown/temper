//! The reporter family — machine formats over one diagnostic source.
//!
//! Two layers of proof. First, the library reporters ([`temper::reporter::github`]
//! / [`temper::reporter::sarif`]) are driven over a hand-built diagnostic set: the
//! `github` reporter emits one `::error`/`::warning::` workflow-command line per
//! finding carrying the rule as `title=`, and the `sarif` reporter emits a SARIF
//! 2.1.0 log with driver `temper` and one `results` entry per diagnostic with the
//! severity mapped `error`->`error` / `warn`->`warning` and the rule as `ruleId`.
//! Both derive purely from the diagnostic set — they re-judge nothing.
//!
//! Second, a CLI-level assertion (over the real process boundary, as `tests/cli.rs`
//! does) pins the verdict invariant: `check --reporter sarif` on a *failing*
//! surface prints SARIF and *still exits non-zero* — a reporter reshapes
//! presentation, never the gate's exit code.
//!
//! Third, also at the CLI, the **announcement**: a run judged by an input the
//! committed harness does not carry names it — every active local member, every
//! dialed clause, every joined lock — and a run judged by the committed harness
//! alone says nothing extra.

mod common;

use std::process::Command;

use temper::check::{Announcement, Diagnostic};
use temper::drift::Declarations;
use temper::reporter;

/// A mixed diagnostic set — one blocking error and one advisory warn — so a single
/// render exercises both severity mappings.
fn diagnostic_set() -> Vec<Diagnostic> {
    vec![
        Diagnostic::error(
            "allowed_chars",
            "Coordinate",
            "name has characters outside [a-z0-9-]",
        ),
        Diagnostic::warn("max_lines", "coordinate", "body is over the line budget"),
    ]
}

#[test]
fn github_emits_one_workflow_command_line_per_finding_with_the_rule_as_title() {
    let diagnostics = diagnostic_set();
    let rendered = reporter::github(&diagnostics, &Announcement::default());

    let lines: Vec<&str> = rendered.lines().collect();
    assert_eq!(
        lines.len(),
        diagnostics.len(),
        "one workflow-command line per finding, got:\n{rendered}"
    );

    // The error maps to `::error` and the warn to `::warning`, each carrying its
    // rule as the annotation `title=` and its message in the body.
    assert!(
        lines[0].starts_with("::error title=allowed_chars::"),
        "an error finding is an ::error line titled with its rule, got: {}",
        lines[0]
    );
    assert!(
        lines[0].contains("name has characters outside [a-z0-9-]"),
        "the message rides the command body, got: {}",
        lines[0]
    );
    assert!(
        lines[1].starts_with("::warning title=max_lines::"),
        "a warn finding is a ::warning line titled with its rule, got: {}",
        lines[1]
    );
}

#[test]
fn github_escapes_workflow_command_metacharacters() {
    // A message with a newline and a `%` must not break out of its single line.
    let diagnostics = vec![Diagnostic::error(
        "rule",
        "artifact",
        "first line\nsecond line 100%",
    )];
    let rendered = reporter::github(&diagnostics, &Announcement::default());

    assert_eq!(
        rendered.lines().count(),
        1,
        "an embedded newline is escaped, never spilled onto a new line: {rendered}"
    );
    assert!(rendered.contains("%0A"), "the newline is percent-encoded");
    assert!(
        rendered.contains("%25"),
        "the percent sign is percent-encoded"
    );
}

#[test]
fn sarif_is_valid_json_with_the_temper_driver_and_one_result_per_diagnostic() {
    let diagnostics = diagnostic_set();
    let log: serde_json::Value =
        serde_json::from_str(&reporter::sarif(&diagnostics, &Announcement::default()))
            .expect("SARIF must be valid JSON");

    assert_eq!(log["version"], "2.1.0");
    assert_eq!(
        log["runs"][0]["tool"]["driver"]["name"], "temper",
        "the SARIF driver names the tool"
    );

    let results = log["runs"][0]["results"]
        .as_array()
        .expect("results is an array");
    assert_eq!(
        results.len(),
        diagnostics.len(),
        "one results entry per diagnostic"
    );

    // The error diagnostic: ruleId from the rule, level mapped error->error,
    // message text and the artifact location all carried from the diagnostic.
    assert_eq!(results[0]["ruleId"], "allowed_chars");
    assert_eq!(results[0]["level"], "error");
    assert_eq!(
        results[0]["message"]["text"],
        "name has characters outside [a-z0-9-]"
    );
    assert_eq!(
        results[0]["locations"][0]["physicalLocation"]["artifactLocation"]["uri"],
        "Coordinate"
    );

    // The warn diagnostic maps warn->warning.
    assert_eq!(results[1]["ruleId"], "max_lines");
    assert_eq!(results[1]["level"], "warning");
}

#[test]
fn both_reporters_are_pure_presentations_of_the_diagnostic_set() {
    // An empty diagnostic set: github emits nothing, and sarif is still a valid
    // one-run log with an empty results array — neither invents a finding.
    assert_eq!(reporter::github(&[], &Announcement::default()), "");

    let log: serde_json::Value =
        serde_json::from_str(&reporter::sarif(&[], &Announcement::default()))
            .expect("empty SARIF is valid JSON");
    assert_eq!(log["runs"][0]["tool"]["driver"]["name"], "temper");
    assert_eq!(
        log["runs"][0]["results"]
            .as_array()
            .expect("results is an array")
            .len(),
        0,
        "no diagnostics ⇒ no results"
    );
}

// --- CLI-level: presentation changes, verdict does not -----------------------

/// A skill that violates `required` clauses (uppercase `name` outside `[a-z0-9-]`,
/// and no longer equal to its directory) — a failing surface, so `check` exits
/// non-zero whichever reporter renders it.
const ERROR_SKILL: &str = "---\n\
name: Coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

#[test]
fn check_reporter_sarif_prints_sarif_and_still_exits_non_zero_on_a_failing_surface() {
    let harness = common::tmpdir("sarif-src");
    common::write_skill(&harness, "coordinate", ERROR_SKILL);

    // CWD is the harness root, carrying no adopted lock, so `check .` reads built-in
    // kind members live off harness disk and no ambient project assembly at the
    // process CWD — e.g. temper's own — can leak in. Mirrors the `schema`/`cli` tests'
    // isolation.
    let run = common::check_in(&harness, &["."], Some("sarif"));

    // Presentation: stdout is a valid SARIF log naming the temper driver.
    let log: serde_json::Value = serde_json::from_str(run.stdout.trim())
        .expect("--reporter sarif must print valid SARIF JSON");
    assert_eq!(log["runs"][0]["tool"]["driver"]["name"], "temper");
    assert!(
        log["runs"][0]["results"]
            .as_array()
            .expect("results is an array")
            .iter()
            .any(|r| r["level"] == "error"),
        "the failing surface's error finding is present in the SARIF results"
    );

    // Verdict: the reporter reshapes presentation, never the exit code — a failing
    // surface still exits non-zero.
    assert!(
        !run.ok,
        "check --reporter sarif on a failing surface must still exit non-zero"
    );
}

// --- The announcement: which inputs judged this run ---------------------------

/// A dial naming a clause the shipped `skill` contract really carries, so the entry
/// reaches one and is announced — a label that reached nothing is a refusal, and
/// nothing was judged through it.
const DIAL: &str = "name = \"workstation\"\n\
\n\
[[clause]]\n\
label = \"skill.max_lines\"\n\
severity = \"required\"\n";

#[test]
fn a_run_announces_every_local_member_dialed_clause_and_joined_lock() {
    // A clean surface: what the run announces is independent of its verdict, so the
    // three announced inputs here are all there is to read.
    let harness = common::tmpdir("announce-all-three");
    common::write_skill(&harness, "coordinate", &common::clean_skill("coordinate"));
    common::write_lock(&harness, Declarations::default());
    // The dial document is both thirds this machine contributes: a local member of the
    // shipped `dial` kind, and the source of the dialed clause.
    common::write_sibling(&harness, ".temper/dial.toml", DIAL);

    // An org corpus whose lock the invocation joins. It declares no clause of its own:
    // the announcement names the lock that was joined, never the clauses it carried, and
    // an empty one joined the run just the same.
    let org = common::tmpdir("announce-org");
    common::write_lock(&org, Declarations::default());
    let layer = org.join(".temper").join("lock.toml");

    let run = common::check_in(
        &harness,
        &[
            "--harness",
            harness.to_str().unwrap(),
            "--layer",
            layer.to_str().unwrap(),
        ],
        Some("github"),
    );
    let announced = run.announcements().join("\n");

    assert!(
        announced.contains("local member: dial:workstation"),
        "the local member's document is uncommitted, so the run says it read it, \
         got:\n{announced}"
    );
    assert!(
        announced.contains("dialed clause: skill.max_lines"),
        "the machine re-weighed a clause, so the run says which, got:\n{announced}"
    );
    assert!(
        announced.contains(&format!("joined lock: {}", layer.display())),
        "the invocation joined a lock, so the run says so — spelled as the `--layer` \
         argument named it, got:\n{announced}"
    );
}

#[test]
fn a_run_judged_by_the_committed_harness_alone_announces_nothing() {
    let harness = common::tmpdir("announce-nothing");
    common::write_skill(&harness, "coordinate", &common::clean_skill("coordinate"));
    common::write_lock(&harness, Declarations::default());

    let run = common::check_harness_in(&harness, Some("github"));

    assert!(
        run.announcements().is_empty(),
        "no local member, no dial, no joined lock ⇒ nothing beyond the committed \
         harness judged this run, and nothing extra is said:\n{}",
        run.output
    );
}

// ---- CLAUSE-LABEL-IS-AN-ADDRESS: one identity, printed on both faces ---------

/// The `temper` binary, located by Cargo at compile time — `explain` has no
/// library-level driver here, so this suite reaches it over the process boundary the
/// way `tests/cli.rs` does.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

#[test]
fn a_clause_finding_prints_its_label_and_explain_narrates_the_same_one() {
    // The whole point of the address: read the annoying finding, read its label, and
    // find that exact label on the member's contract in `explain` — with no lookup
    // table in between. So the two faces must print one string, not two spellings.
    let harness = common::tmpdir("clause-label-faces");
    common::write_skill(&harness, "coordinate", ERROR_SKILL);
    common::write_lock(&harness, temper::drift::Declarations::default());

    let findings = common::check_in(&harness, &[], Some("github")).findings();
    let charset = findings
        .iter()
        .find(|line| line.contains("characters outside"))
        .expect("the uppercase name trips the skill floor's charset clause");
    assert!(
        charset.starts_with("::error title=skill.allowed_chars.name::"),
        "the finding's diagnostic code is the clause's address, not the bare \
         predicate key, got: {charset}"
    );

    let explained = Command::new(BIN)
        .current_dir(&harness)
        .args(["explain", "coordinate"])
        .output()
        .unwrap();
    let narration = String::from_utf8_lossy(&explained.stdout).into_owned();
    assert!(
        narration.contains("skill.allowed_chars.name"),
        "`explain` narrates the governing contract's clauses by the same address the \
         finding printed, got:\n{narration}"
    );
}
