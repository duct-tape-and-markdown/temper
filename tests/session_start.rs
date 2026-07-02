//! Acceptance for the advisory session-start gate (`specs/50-distribution.md`,
//! "Decision: the session-start gate is advisory, not blocking").
//!
//! Two surfaces of the same gate. The **one-shot verb** — `temper session-start
//! <harness>` — is driven across the real process boundary (the exit code and the
//! stdout payload are observable only there): a failing contract yields a payload
//! carrying the verdict plus the notify-and-approve instruction, a clean harness
//! yields the quiet payload, the output is valid JSON under the 10k cap, and the
//! run exits zero regardless (advisory, never blocking). The **reporter** itself
//! (`temper::reporter`) is exercised directly through the library for the cap
//! invariant, where a synthetic flood of diagnostics is easier to construct than
//! to provoke through a harness.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::check::Diagnostic;
use temper::reporter::{self, ADDITIONAL_CONTEXT_CAP};

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-session-start-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A skill that trips no `error`-severity clause: lowercase `name` matching its
/// directory, a present short description, a short body.
const CLEAN_SKILL: &str = "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// A skill that violates a `required` clause: the uppercase `name` is outside the
/// `[a-z0-9-]` `allowed_chars` set — a failing contract.
const ERROR_SKILL: &str = "---\n\
name: Coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// Write a one-skill harness at `<root>/skills/<name>/SKILL.md`.
fn write_harness(root: &Path, name: &str, skill_md: &str) {
    let dir = root.join("skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("SKILL.md"), skill_md).unwrap();
}

/// Run `temper session-start <harness>` and return `(exit-zero, parsed payload)`.
fn run_session_start(harness: &Path) -> (bool, serde_json::Value) {
    let output = Command::new(BIN)
        .arg("session-start")
        .arg(harness)
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    // The gate owns its output contract: stdout is always valid JSON.
    let payload = serde_json::from_str(&stdout)
        .unwrap_or_else(|e| panic!("session-start stdout must be valid JSON ({e}):\n{stdout}"));
    (output.status.success(), payload)
}

#[test]
fn a_failing_harness_emits_the_verdict_and_exits_zero() {
    let harness = tmpdir("failing-src");
    write_harness(&harness, "coordinate", ERROR_SKILL);

    let (ok, payload) = run_session_start(&harness);

    // Advisory: a failing contract never blocks the session.
    assert!(
        ok,
        "the session-start gate must exit zero even on a failure"
    );

    let hook = &payload["hookSpecificOutput"];
    assert_eq!(hook["hookEventName"], "SessionStart");
    let context = hook["additionalContext"]
        .as_str()
        .expect("a failing contract must carry additionalContext");
    // The verdict routes through the human, and names the offending clause.
    assert!(
        context.contains("approval before continuing"),
        "the verdict must carry the notify-and-approve instruction, got:\n{context}"
    );
    assert!(
        context.contains("allowed_chars"),
        "the verdict must name the failing clause, got:\n{context}"
    );
    // Under the 10k cap.
    assert!(context.chars().count() <= ADDITIONAL_CONTEXT_CAP);
}

#[test]
fn a_clean_harness_emits_the_quiet_payload_and_exits_zero() {
    let harness = tmpdir("clean-src");
    write_harness(&harness, "coordinate", CLEAN_SKILL);

    let (ok, payload) = run_session_start(&harness);

    assert!(ok, "a clean harness must exit zero");
    let hook = &payload["hookSpecificOutput"];
    assert_eq!(hook["hookEventName"], "SessionStart");
    // Quiet: no context is injected into the session.
    assert!(
        hook["additionalContext"].is_null(),
        "a clean harness must emit no additionalContext, got: {hook}"
    );
}

#[test]
fn a_registered_custom_kind_resolves_from_the_harness_temper_dir() {
    // The dogfood bug (inbox): `session-start` over a project registering a custom
    // kind must resolve the kind's authored KIND.md + bound package from the
    // harness's own `.temper/` — beside its `temper.toml` — not the throwaway scratch
    // surface the members import into. Before the fix the definition dangled as
    // `kind::missing_definition` because `gate` read `kinds`/`packages` from the
    // scratch, which never carries them.
    let harness = tmpdir("custom-kind-src");

    // The assembly registers a custom `spec` kind and binds its package by name.
    fs::write(
        harness.join("temper.toml"),
        "[kind.spec]\npackage = \"spec\"\n",
    )
    .unwrap();

    // The authored kind definition under `.temper/kinds/spec/KIND.md`: a member is a
    // `specs/*.md` file, extracting a line count (a decidable, trivially-satisfied
    // feature).
    let kind_dir = harness.join(".temper").join("kinds").join("spec");
    fs::create_dir_all(&kind_dir).unwrap();
    fs::write(
        kind_dir.join("KIND.md"),
        "+++\n\
         governs = { root = \"specs\", glob = \"*.md\" }\n\
         \n\
         [[extraction]]\n\
         primitive = \"line_count\"\n\
         +++\n\
         # The spec kind\n\
         \n\
         temper's own governing documents.\n",
    )
    .unwrap();

    // The bound package under `.temper/packages/spec/PACKAGE.md`: no clauses, so its
    // members conform trivially — this fixture pins the *resolution*, not the engine.
    let pkg_dir = harness.join(".temper").join("packages").join("spec");
    fs::create_dir_all(&pkg_dir).unwrap();
    fs::write(
        pkg_dir.join("PACKAGE.md"),
        "+++\n+++\n# The spec package\n\nNo clauses — resolution is what this pins.\n",
    )
    .unwrap();

    // A member source at the `governs` root and a clean skill, so the harness carries
    // a real custom-kind member alongside a built-in one.
    let specs = harness.join("specs");
    fs::create_dir_all(&specs).unwrap();
    fs::write(specs.join("00-intent.md"), "# Intent\n\nThe north star.\n").unwrap();
    write_harness(&harness, "coordinate", CLEAN_SKILL);

    let (ok, payload) = run_session_start(&harness);

    // Advisory and, crucially, *emits a payload at all*: before the fix `gate`
    // propagated a hard `kind::missing_definition` error (KIND.md absent from the
    // scratch), so `session-start` exited non-zero with no JSON on stdout — and
    // `run_session_start` would have panicked parsing it.
    assert!(ok, "the session-start gate must exit zero");
    let hook = &payload["hookSpecificOutput"];
    assert_eq!(hook["hookEventName"], "SessionStart");
    // The registered custom kind resolves cleanly from the harness's `.temper/`, so
    // the payload is quiet — no verdict, and never the `missing_definition` a
    // scratch-surface resolution would raise.
    assert!(
        hook["additionalContext"].is_null(),
        "the custom kind must resolve from the harness's .temper/ ⇒ quiet payload, got: {hook}"
    );
}

#[test]
fn an_authored_surface_resolves_its_satisfies_fill_with_no_blocking_findings() {
    // The inbox false positive, repro'd: a harness carrying an authored surface
    // (a `temper.toml` declaring a `required` requirement + a surface member whose
    // document header `satisfies` it) must emit ZERO blocking findings at
    // session-start. The surface path resolves the fill; a fresh import would
    // discard the authored `satisfies` recognition and report the requirement
    // unfilled — the law-3 false positive the spec's surface-present clause forbids.
    let harness = tmpdir("authored-surface-src");

    // The assembly declares a required `rule`-kind requirement.
    fs::write(
        harness.join("temper.toml"),
        "[requirement.engineering-standards]\n\
         means = \"the harness carries a rule maintaining engineering standards\"\n\
         kind = \"rule\"\n\
         required = true\n",
    )
    .unwrap();

    // The authored surface member: a `rule` whose imported document header opts in
    // via `satisfies.engineering-standards` — the recognition a fresh import drops.
    let rule_dir = harness.join(".temper").join("rules").join("rust");
    fs::create_dir_all(&rule_dir).unwrap();
    fs::write(
        rule_dir.join("RULE.md"),
        "+++\n\
         [clause.paths]\n\
         value = [\"src/**/*.rs\"]\n\
         \n\
         [satisfies.engineering-standards]\n\
         rationale = \"the path-scoped home for the Rust engineering bar\"\n\
         \n\
         [provenance]\n\
         source_path = \"./.claude/rules/rust.md\"\n\
         import_hash = \"0000000000000000000000000000000000000000000000000000000000000000\"\n\
         +++\n\
         # Rust conventions\n\
         \n\
         The engineering bar.\n",
    )
    .unwrap();

    let (ok, payload) = run_session_start(&harness);

    // Advisory, and — the point — the required requirement resolves via the
    // surface's `satisfies`, so no blocking verdict is injected.
    assert!(ok, "the session-start gate must exit zero");
    let hook = &payload["hookSpecificOutput"];
    assert_eq!(hook["hookEventName"], "SessionStart");
    assert!(
        hook["additionalContext"].is_null(),
        "the authored `satisfies` must fill the requirement ⇒ quiet payload, got: {hook}"
    );
}

#[test]
fn the_reporter_caps_additional_context_at_10k() {
    // A synthetic flood far larger than the cap — easier to construct directly
    // than to provoke through a harness, and the cap is the reporter's own
    // invariant. Each finding carries a long message so the total overruns 10k.
    let long = "x".repeat(300);
    let diagnostics: Vec<Diagnostic> = (0..1000)
        .map(|i| Diagnostic::error("required", format!("artifact-{i}"), &long))
        .collect();

    let rendered = reporter::session_start(&diagnostics);
    let payload: serde_json::Value =
        serde_json::from_str(&rendered).expect("even a capped payload is valid JSON");
    let context = payload["hookSpecificOutput"]["additionalContext"]
        .as_str()
        .unwrap();

    assert_eq!(
        context.chars().count(),
        ADDITIONAL_CONTEXT_CAP,
        "an over-long verdict is truncated to exactly the cap"
    );
    // The notify-and-approve instruction leads the verdict, so truncation of a
    // long finding list never drops it.
    assert!(context.contains("approval before continuing"));
}
