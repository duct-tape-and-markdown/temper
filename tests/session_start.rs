//! Acceptance for the advisory session-start gate.
//!
//! Two surfaces of the same gate. Session-start is a **reporter of `check`, not a
//! verb**: `temper check <harness>
//! --reporter session-start` reads the path as a harness root and is driven across the
//! real process boundary (the exit code and the stdout payload are observable only
//! there) — a failing contract yields a payload carrying the verdict plus the
//! notify-and-approve instruction, a clean harness yields the quiet payload, the output
//! is valid JSON under the 10k cap, and the run exits zero regardless (advisory, never
//! blocking). The **reporter** itself (`temper::reporter`) is exercised directly through
//! the library for the cap invariant, where a synthetic flood of diagnostics is easier
//! to construct than to provoke through a harness.

use std::fs;
use std::path::Path;
use std::process::Command;

mod common;

use temper::check::Diagnostic;
use temper::reporter::{self, ADDITIONAL_CONTEXT_CAP};

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

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

/// Run `temper check <harness> --reporter session-start` and return `(exit-zero, parsed
/// payload)`. The session-start reporter reads the positional path as a harness root.
fn run_session_start(harness: &Path) -> (bool, serde_json::Value) {
    let output = Command::new(BIN)
        .arg("check")
        .arg(harness)
        .arg("--reporter")
        .arg("session-start")
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
    let harness = common::tmpdir("failing-src");
    common::write_skill(&harness, "coordinate", ERROR_SKILL);

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
    let harness = common::tmpdir("clean-src");
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);

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
fn stray_custom_kind_shaped_fixtures_never_disturb_a_clean_session_start() {
    // Custom-kind registration retired along with the manifest that once carried it
    // (`TEMPER-TOML-ZERO`) and the `KIND.md` file format retired earlier still
    // — there is no
    // longer any author-facing way to register one. This pins that a harness carrying
    // such shaped-but-inert fixture files (nothing reads them) alongside a real skill
    // still resolves to a clean, quiet session-start payload.
    let harness = common::tmpdir("custom-kind-src");

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
    common::write_skill(&harness, "coordinate", CLEAN_SKILL);

    let (ok, payload) = run_session_start(&harness);

    assert!(ok, "the session-start gate must exit zero");
    let hook = &payload["hookSpecificOutput"];
    assert_eq!(hook["hookEventName"], "SessionStart");
    // The stray fixture files contribute no members and no findings, so the payload
    // is quiet — the clean skill is the only thing the gate resolves.
    assert!(
        hook["additionalContext"].is_null(),
        "stray custom-kind-shaped fixtures must not disturb a clean payload, got: {hook}"
    );
}

#[test]
fn an_authored_surface_resolves_its_satisfies_fill_with_no_blocking_findings() {
    // The inbox false positive, repro'd: a harness carrying the lock's declared
    // `required` requirement plus a `[[declaration.satisfies]]` row hand-edited onto
    // the committed lock (the real SDK-emit shape a converted harness carries) must
    // emit ZERO blocking findings at session-start — session-start itself never
    // re-imports, so the lock-declared row is the sole source naming the member as a
    // filler.
    let harness = common::tmpdir("authored-surface-src");

    // The committed landscape file a prior `import` would have discovered — the gate
    // walks the lock's governs locus straight off the harness, so the member must exist here too, not just declared in
    // the lock below.
    let rules = harness.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(
        rules.join("rust.md"),
        "---\n\
         paths:\n\
         \x20\x20- \"src/**/*.rs\"\n\
 ---\n\
         # Rust conventions\n\
 \n\
         The engineering bar.\n",
    )
    .unwrap();

    // The gate reads the assembly's requirements, and each member's `satisfies` fill,
    // off the lock's declaration rows — the fixture stands in for a prior `import`
    // having already written both — session-start itself still never re-imports.
    let temper_dir = harness.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(
        temper_dir.join("lock.toml"),
        "[[declaration.requirement]]\n\
         name = \"engineering-standards\"\n\
         kind = \"rule\"\n\
         required = true\n\
 \n\
         [[declaration.satisfies]]\n\
         member = \"rust\"\n\
         requirement = \"engineering-standards\"\n",
    )
    .unwrap();

    let (ok, payload) = run_session_start(&harness);

    // Advisory, and — the point — the required requirement resolves via the
    // lock-declared `satisfies` row, so no blocking verdict is injected.
    assert!(ok, "the session-start gate must exit zero");
    let hook = &payload["hookSpecificOutput"];
    assert_eq!(hook["hookEventName"], "SessionStart");
    assert!(
        hook["additionalContext"].is_null(),
        "the lock-declared `satisfies` must fill the requirement ⇒ quiet payload, got: {hook}"
    );
}

#[test]
fn a_custom_kind_synthesized_from_the_lock_resolves_its_requirement_with_no_false_admissibility_finding()
 {
    // The cascade field report, repro'd: a harness whose committed lock declares a
    // custom kind (`spec`) and a `required` requirement naming it must NOT emit the
    // false `requirement.admissibility` "does not model" finding roster.rs used to
    // raise when `by_kind` carried only `skill`/`rule` — the custom kind's own row is
    // synthesized into the same corpus, so the requirement resolves against it, its
    // member is walked and counted, and the run stays quiet.
    let harness = common::tmpdir("custom-kind-lock-src");

    let temper_dir = harness.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(
        temper_dir.join("lock.toml"),
        "[[declaration.kind]]\n\
         name = \"spec\"\n\
         governs_root = \"specs\"\n\
         governs_glob = \"*.md\"\n\
 \n\
         [[declaration.clause]]\n\
         kind = \"spec\"\n\
         predicate = \"max_lines\"\n\
         severity = \"advisory\"\n\
         bound = { max = 20 }\n\
 \n\
         [[declaration.requirement]]\n\
         name = \"spec-coverage\"\n\
         kind = \"spec\"\n\
         required = true\n\
 \n\
         [[declaration.satisfies]]\n\
         member = \"00-intent\"\n\
         requirement = \"spec-coverage\"\n",
    )
    .unwrap();

    // The real member on disk, at the lock-declared `governs` locus — the gate walks
    // this straight off the harness, exactly as it does a built-in's members. Its
    // `satisfies` fill is the lock-declared row above — the real SDK-emit shape a
    // converted harness carries.
    let specs = harness.join("specs");
    fs::create_dir_all(&specs).unwrap();
    fs::write(specs.join("00-intent.md"), "# Intent\n\nThe north star.\n").unwrap();

    let (ok, payload) = run_session_start(&harness);

    assert!(ok, "the session-start gate must exit zero");
    let hook = &payload["hookSpecificOutput"];
    assert_eq!(hook["hookEventName"], "SessionStart");
    assert!(
        hook["additionalContext"].is_null(),
        "a lock-synthesized custom kind's requirement must resolve with no blocking \
         finding, got: {hook}"
    );
}

#[test]
fn a_custom_kinds_required_floor_clause_blocks_a_violating_member() {
    // The other half of the fix: a custom kind's floor is no longer a no-op. Its
    // clause rows now dispatch through the same admissibility/conformance the
    // built-in loop runs, so a `required` clause the member violates fires a real
    // blocking finding — proof that conformance runs, not just that resolution does.
    let harness = common::tmpdir("custom-kind-floor-src");

    let temper_dir = harness.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(
        temper_dir.join("lock.toml"),
        "[[declaration.kind]]\n\
         name = \"spec\"\n\
         governs_root = \"specs\"\n\
         governs_glob = \"*.md\"\n\
 \n\
         [[declaration.clause]]\n\
         kind = \"spec\"\n\
         predicate = \"required\"\n\
         field = \"owner\"\n\
         severity = \"required\"\n",
    )
    .unwrap();

    // The on-disk member never declares `owner` — a real violation of the lock's own
    // custom-kind floor clause.
    let specs = harness.join("specs");
    fs::create_dir_all(&specs).unwrap();
    fs::write(specs.join("00-intent.md"), "# Intent\n\nThe north star.\n").unwrap();

    let (ok, payload) = run_session_start(&harness);

    assert!(
        ok,
        "the session-start gate must exit zero even on a failure"
    );
    let hook = &payload["hookSpecificOutput"];
    assert_eq!(hook["hookEventName"], "SessionStart");
    let context = hook["additionalContext"]
        .as_str()
        .expect("the violated custom-kind floor clause must carry additionalContext");
    assert!(
        context.contains("required") && context.contains("owner"),
        "the verdict must name the failing custom-kind clause, got:\n{context}"
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
