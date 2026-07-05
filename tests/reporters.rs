//! The reporter family — machine formats over one diagnostic source
//! (`specs/architecture/50-distribution.md`, "Outward seams — Reporters").
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

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::check::Diagnostic;
use temper::reporter;

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-reporters-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

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
    let rendered = reporter::github(&diagnostics);

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
    let rendered = reporter::github(&diagnostics);

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
        serde_json::from_str(&reporter::sarif(&diagnostics)).expect("SARIF must be valid JSON");

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
    assert_eq!(reporter::github(&[]), "");

    let log: serde_json::Value =
        serde_json::from_str(&reporter::sarif(&[])).expect("empty SARIF is valid JSON");
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

/// Write a one-skill harness at `<root>/skills/<name>/SKILL.md`.
fn write_harness(root: &Path, name: &str, skill_md: &str) {
    let dir = root.join(".claude").join("skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("SKILL.md"), skill_md).unwrap();
}

/// Import `harness` into `into` via the library — the retired `temper import` verb's
/// exact two steps (surface projection + manifest), driven in-process now that the CLI
/// on-ramp is `init`/`check --harness`.
fn import(harness: &Path, into: &Path) {
    temper::import::run(harness, into).unwrap();
    temper::import::emit_manifest(harness, into).unwrap();
}

#[test]
fn check_reporter_sarif_prints_sarif_and_still_exits_non_zero_on_a_failing_surface() {
    let harness = tmpdir("sarif-src");
    write_harness(&harness, "coordinate", ERROR_SKILL);
    let into = tmpdir("sarif-into");
    import(&harness, &into);

    // CWD-isolated to the workspace (which carries no `temper.toml`) so an ambient
    // project layer at the process CWD — e.g. temper's own, registering the `spec`
    // custom kind whose definition this foreign workspace lacks — can't leak in and
    // abort the load. Mirrors the `schema`/`cli` tests' isolation.
    let output = Command::new(BIN)
        .current_dir(&into)
        .arg("check")
        .arg(&into)
        .arg("--reporter")
        .arg("sarif")
        .output()
        .unwrap();

    // Presentation: stdout is a valid SARIF log naming the temper driver.
    let stdout = String::from_utf8(output.stdout).unwrap();
    let log: serde_json::Value =
        serde_json::from_str(stdout.trim()).expect("--reporter sarif must print valid SARIF JSON");
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
        !output.status.success(),
        "check --reporter sarif on a failing surface must still exit non-zero"
    );
}
