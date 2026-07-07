//! End-to-end CLI acceptance over the documented surface.
//!
//! Spawns the built `temper` binary via `CARGO_BIN_EXE_temper` and drives `temper
//! check --harness <path>` — the one-shot wedge that lints a raw harness directly off
//! disk, no on-ramp step — asserting the exit semantics: zero on a clean skill,
//! non-zero once a `required`-severity contract clause is violated. The `init`/`lift`
//! on-ramp verbs retired with the `[[member]]` manifest codec (`CODEC-RETIRE`); `install`
//! is the on-ramp going forward, not yet
//! shipped. A `--deny-advisories` case pins the strict policy.
//!
//! These checks live here (not in a `src` unit test) precisely because the exit
//! code is observable only across a real process boundary — `process::ExitCode`
//! is surfaced by `main`, not returned by the library.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-cli-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A skill that trips no `error`-severity rule: the `name` is valid and matches
/// its directory, the description is present and short, and the body references
/// no files. (`when` / `not` keep even the description advisories quiet.)
const CLEAN_SKILL: &str = "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// A skill that violates `required` clauses: the uppercase `name` is outside
/// `[a-z0-9-]` (the `allowed_chars` clause) and no longer equals its directory
/// (the `name-matches-dir` clause). Both are required ⇒ a non-zero exit.
const ERROR_SKILL: &str = "---\n\
name: Coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// A skill clean but for its over-budget body: every `required` clause holds
/// (lowercase `name` matching its directory, a present short description, no
/// forbidden keys), and the only violation is the advisory `max_lines` budget
/// (warn). That isolates the `--deny-advisories` promotion.
fn advisory_only_skill() -> String {
    let mut body = String::from("# Coordinate\n");
    for line in 1..=600 {
        body.push_str(&format!("Line {line} of an over-budget body.\n"));
    }
    format!(
        "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
{body}"
    )
}

/// Write a one-skill harness at `<root>/.claude/skills/<name>/SKILL.md`.
fn write_harness(root: &Path, name: &str, skill_md: &str) {
    let dir = root.join(".claude").join("skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("SKILL.md"), skill_md).unwrap();
}

/// A rule that trips no `required` clause: `paths:`-only frontmatter (Claude
/// Code's real scoping key) and a short body — the clean shape of `rust.md`.
const CLEAN_RULE: &str = "---\n\
paths:\n\
  - \"src/**/*.rs\"\n\
---\n\
# Rust conventions\n\
\n\
Prefer a clone over a lifetime fight.\n";

/// A rule that violates the `forbidden_keys` clause: a Cursor `.mdc` key
/// (`globs`) Claude Code silently ignores — the exact mistake the rule contract
/// exists to catch. That clause is `required` ⇒ a non-zero exit.
const FORBIDDEN_KEY_RULE: &str = "---\n\
globs: \"**/*.rs\"\n\
alwaysApply: true\n\
---\n\
# Rust conventions\n\
\n\
This frontmatter loads nothing in Claude Code.\n";

/// Write a one-rule harness at `<root>/.claude/rules/<name>.md` — the location
/// `init` scans for the rule kind.
fn write_rule_harness(root: &Path, name: &str, rule_md: &str) {
    let dir = root.join(".claude").join("rules");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join(format!("{name}.md")), rule_md).unwrap();
}

/// Run `temper check --harness <harness> [extra…]` (the one-shot wedge — no on-ramp
/// step, no workspace) and return `(exit-zero, stdout)`.
fn run_check_harness(harness: &Path, extra: &[&str]) -> (bool, String) {
    let output = Command::new(BIN)
        .arg("check")
        .arg("--harness")
        .arg(harness)
        .args(extra)
        .output()
        .unwrap();
    (
        output.status.success(),
        String::from_utf8(output.stdout).unwrap(),
    )
}

/// Run `temper check --harness <harness>` and return whether it exited zero.
fn check_harness_succeeds(harness: &Path) -> bool {
    run_check_harness(harness, &[]).0
}

#[test]
fn check_is_clean_for_a_well_formed_skill() {
    let harness = tmpdir("clean-src");
    write_harness(&harness, "coordinate", CLEAN_SKILL);

    assert!(
        check_harness_succeeds(&harness),
        "a clean skill must exit zero (no error-severity diagnostics)"
    );
}

#[test]
fn check_exits_non_zero_when_an_error_rule_fires() {
    let harness = tmpdir("error-src");
    // Directory `coordinate` but `name: Coordinate` — trips name-format and
    // name-matches-dir, both error severity.
    write_harness(&harness, "coordinate", ERROR_SKILL);

    assert!(
        !check_harness_succeeds(&harness),
        "an error-severity diagnostic must make check exit non-zero"
    );
}

#[test]
fn deny_advisories_promotes_a_warn_only_run_to_a_failure() {
    let harness = tmpdir("advisory-src");
    // The only clause this skill violates is the advisory `max_lines` budget.
    write_harness(&harness, "coordinate", &advisory_only_skill());

    // Default policy: an advisory-only run is clean — warn does not gate.
    assert!(
        check_harness_succeeds(&harness),
        "an advisory-only violation must exit zero without --deny-advisories"
    );
    // Strict policy: --deny-advisories promotes the warn to a blocking failure.
    assert!(
        !run_check_harness(&harness, &["--deny-advisories"]).0,
        "an advisory-only violation must exit non-zero under --deny-advisories"
    );
}

#[test]
fn check_dispatches_the_rule_kind_to_the_rule_contract() {
    // A clean rule (`paths:`-only) trips no `required` clause ⇒ check is zero.
    let clean = tmpdir("rule-clean-src");
    write_rule_harness(&clean, "rust", CLEAN_RULE);
    assert!(
        check_harness_succeeds(&clean),
        "a clean rule must exit zero — the rule contract has no `required` violation"
    );

    // A forbidden Cursor key (`globs`/`alwaysApply`) trips the `forbidden_keys`
    // clause, which is `required` ⇒ check is non-zero. This proves `check`
    // dispatches the rule kind to the rule contract, not the skill one.
    let forbidden = tmpdir("rule-forbidden-src");
    write_rule_harness(&forbidden, "rust", FORBIDDEN_KEY_RULE);
    assert!(
        !check_harness_succeeds(&forbidden),
        "a forbidden-key rule must exit non-zero (the rule contract's required clause)"
    );
}

#[test]
fn check_harness_one_shot_lints_a_raw_harness_without_a_workspace() {
    // The zero-config wedge: a raw harness is linted directly, no `init` step, and
    // no surface workspace is written. A forbidden Cursor key trips a `required`
    // clause ⇒ non-zero, and the finding is on stdout.
    let harness = tmpdir("one-shot-src");
    write_rule_harness(&harness, "rust", FORBIDDEN_KEY_RULE);

    let (ok, stdout) = run_check_harness(&harness, &[]);
    assert!(
        !ok,
        "check --harness must exit non-zero on a required-clause violation"
    );
    assert!(
        stdout.contains("forbidden_keys"),
        "the finding must reach stdout, got:\n{stdout}"
    );
    // One-shot means no workspace ceremony: the harness is imported internally into a
    // scratch dir, so no `.temper` surface is left beside it.
    assert!(
        !harness.join(".temper").exists(),
        "check --harness must not write a surface workspace beside the harness"
    );

    // A clean harness over the same one-shot path exits zero.
    let clean = tmpdir("one-shot-clean");
    write_rule_harness(&clean, "rust", CLEAN_RULE);
    let (ok, _) = run_check_harness(&clean, &[]);
    assert!(ok, "check --harness over a clean harness must exit zero");
}

#[test]
fn check_rejects_a_harness_and_workspace_together() {
    // `--harness` and the positional workspace are the two mutually-exclusive routes
    // into the gate; supplying both is a usage error, not a silent precedence pick.
    let ws = tmpdir("conflict-ws");
    let harness = tmpdir("conflict-harness");
    let status = Command::new(BIN)
        .arg("check")
        .arg(&ws)
        .arg("--harness")
        .arg(&harness)
        .status()
        .unwrap();
    assert!(
        !status.success(),
        "check <workspace> --harness <path> must be a usage error"
    );
}

#[test]
fn self_host_check_is_clean_over_tempers_own_surface() {
    // The bootstrap proof: `temper check` over temper's
    // OWN committed surface — its `.temper/` document-carried rules plus its lock-declared
    // assembly (spec kinds, requirements) — lints clean. `CARGO_MANIFEST_DIR` is the
    // crate root; a bare `check` reads the committed surface there, read-only, so the
    // repo is never mutated (the flume `temper check (self)` gate's exact invocation).
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let status = Command::new(BIN)
        .current_dir(repo_root)
        .arg("check")
        .status()
        .unwrap();
    assert!(
        status.success(),
        "temper must lint its own committed surface clean — the self-hosting finish line"
    );
}

#[test]
fn schema_kind_skill_emits_the_skill_floor_decidable_clauses() {
    // Run in a fresh CWD with no adopted lock, so the emitted schema is the pure
    // skill floor (no clause overrides) and the assertions are deterministic.
    let cwd = tmpdir("schema-skill");
    let output = Command::new(BIN)
        .current_dir(&cwd)
        .arg("schema")
        .arg("--kind")
        .arg("skill")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "temper schema --kind skill must exit zero"
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    // A JSON object over frontmatter.
    assert!(
        stdout.contains("\"type\": \"object\""),
        "schema must be an object schema, got:\n{stdout}"
    );
    // The skill floor requires `name` (required → required[]).
    assert!(
        stdout.contains("\"required\""),
        "the skill floor's `required` clause must project a required[] array"
    );
    // `allowed_chars` on `name` → a generated `pattern` charclass.
    assert!(
        stdout.contains("\"pattern\""),
        "the skill floor's allowed_chars clause must project a `pattern`"
    );
    // `forbidden_keys` (globs / alwaysApply) → a `not`/`required` combinator per key.
    assert!(
        stdout.contains("globs") && stdout.contains("alwaysApply"),
        "the skill floor's forbidden_keys must appear as forbidden-key combinators"
    );

    // It parses back as JSON — a well-formed schema, not just a string that looks
    // like one.
    serde_json::from_str::<serde_json::Value>(&stdout).expect("emitted schema must be valid JSON");
}

#[test]
fn schema_kind_skill_emits_guidance_as_the_docs_channel_description() {
    // The docs (hover) channel of the emitted schema: a field clause's `guidance` prose rides its JSON
    // Schema property's `description`, strictly alongside the validation keywords.
    // The embedded `skill.anthropic` built-in now carries guidance on its clauses,
    // so
    // the pure floor — no clause overrides — already exercises both channels.
    let cwd = tmpdir("schema-guidance");
    let output = Command::new(BIN)
        .current_dir(&cwd)
        .arg("schema")
        .arg("--kind")
        .arg("skill")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "temper schema --kind skill must exit zero"
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    // Docs channel: the floor's `name` guidance rides its `description`, beside its
    // validation keywords (`maxLength`/`pattern` et al.), never mixed into them.
    assert!(
        json["properties"]["name"]["description"].is_string(),
        "the floor's name guidance must ride the property description, got:\n{stdout}"
    );
    assert_eq!(json["properties"]["name"]["maxLength"], 64);
    assert!(json["properties"]["name"]["pattern"].is_string());

    // The prose stays in the docs channel — it never became a validation keyword.
    assert!(json["properties"]["name"].get("enum").is_none());
    assert!(json["properties"]["name"].get("const").is_none());
    // Guidance never leaks to the schema root, only onto property `description`s.
    assert!(json.get("description").is_none());
}

#[test]
fn schema_without_kind_maps_every_modeled_kind() {
    let cwd = tmpdir("schema-all");
    let output = Command::new(BIN)
        .current_dir(&cwd)
        .arg("schema")
        .output()
        .unwrap();
    assert!(output.status.success(), "temper schema must exit zero");

    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    // A by-kind map keyed by each kind's bare row label: each resolves to its own schema.
    assert_eq!(json["skill"]["type"], "object");
    assert_eq!(json["rule"]["type"], "object");
}

#[test]
fn schema_rejects_an_unknown_kind() {
    let cwd = tmpdir("schema-unknown");
    let status = Command::new(BIN)
        .current_dir(&cwd)
        .arg("schema")
        .arg("--kind")
        .arg("nonesuch")
        .status()
        .unwrap();
    assert!(
        !status.success(),
        "an unknown kind must be a hard error, not a silent empty schema"
    );
}

#[test]
fn the_cli_surface_is_check_emit_install_bundle_schema_guard_explain() {
    // The collapsed surface: five nouns
    // plus `guard`, plus `explain` — the one read verb (EXPLAIN-UNIFY) — landed once its
    // fork-gate (`explain-target-disambiguation`) resolved. `--help` lists exactly
    // these; the migration-era verbs, and `init`/`lift` (retired into `install`,
    // `CODEC-RETIRE`), are gone.
    let help = Command::new(BIN).arg("--help").output().unwrap();
    assert!(help.status.success(), "temper --help must exit zero");
    let stdout = String::from_utf8(help.stdout).unwrap();
    // The "Commands:" section lists each surviving noun (a leading-whitespace entry, so a
    // retired verb merely *mentioned* in a description does not count as present).
    for command in [
        "check", "emit", "install", "bundle", "schema", "guard", "explain",
    ] {
        assert!(
            stdout
                .lines()
                .any(|line| line.trim_start().starts_with(command)),
            "temper --help must list `{command}`, got:\n{stdout}"
        );
    }

    // Every retired verb is rejected as an unknown subcommand — the surface no longer
    // carries `init`/`import`/`diff`/`session-start`/`why`/`requirements`/`impact`/`context`,
    // `init` retired into `install` (`CODEC-RETIRE`), the rest collapsed into `explain`
    // at EXPLAIN-UNIFY.
    for retired in [
        "init",
        "import",
        "diff",
        "session-start",
        "why",
        "requirements",
        "impact",
        "context",
    ] {
        let status = Command::new(BIN).arg(retired).arg("x").status().unwrap();
        assert!(
            !status.success(),
            "`temper {retired}` must be a rejected (unknown) subcommand"
        );
    }
}

#[test]
fn guard_reads_a_pretooluse_payload_and_acts_on_the_posture() {
    use std::io::Write;

    // `temper guard` reads the `PreToolUse` payload from stdin and acts at the
    // enforcement mode declared in the harness's lock. A `block` lock blocks a `.claude/` write (exit 2); a
    // non-projection write is allowed (exit 0). `warn`/`note` both allow a
    // projection write.
    let root = tmpdir("guard-block");
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(
        temper_dir.join("lock.toml"),
        "[[declaration.assembly]]\nfact = \"mode\"\nvalue = \"block\"\n\n\
         [[rule]]\nname = \"rust\"\nsource_path = \".claude/rules/rust.md\"\nsource_hash = \"abc\"\nemit_hash = \"abc\"\n",
    )
    .unwrap();

    let run_in = |root: &std::path::Path, payload: &str| {
        let mut child = Command::new(BIN)
            .arg("guard")
            .arg(root)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(payload.as_bytes())
            .unwrap();
        child.wait_with_output().unwrap()
    };
    let run = |payload: &str| run_in(&root, payload);

    let blocked = run("{\"tool_input\":{\"file_path\":\".claude/rules/rust.md\"}}");
    assert_eq!(
        blocked.status.code(),
        Some(2),
        "a block harness blocks a projection write"
    );
    assert!(
        String::from_utf8_lossy(&blocked.stderr).contains("temper-managed projection"),
        "the block states the managed-by message"
    );

    let allowed = run("{\"tool_input\":{\"file_path\":\"README.md\"}}");
    assert!(
        allowed.status.success(),
        "a non-projection write is allowed even under `block`"
    );

    for mode in ["warn", "note"] {
        let root = tmpdir(&format!("guard-{mode}"));
        let temper_dir = root.join(".temper");
        fs::create_dir_all(&temper_dir).unwrap();
        fs::write(
            temper_dir.join("lock.toml"),
            format!(
                "[[declaration.assembly]]\nfact = \"mode\"\nvalue = \"{mode}\"\n\n\
                 [[rule]]\nname = \"rust\"\nsource_path = \".claude/rules/rust.md\"\nsource_hash = \"abc\"\nemit_hash = \"abc\"\n"
            ),
        )
        .unwrap();

        let projection_write = run_in(
            &root,
            "{\"tool_input\":{\"file_path\":\".claude/rules/rust.md\"}}",
        );
        assert!(
            projection_write.status.success(),
            "a `{mode}` harness allows a projection write"
        );
    }
}

#[test]
fn help_text_speaks_the_current_enforcement_vocabulary_and_layout() {
    // HELP-TEXT-RECUT: no user-facing help/about string may still speak the
    // retired `shared`/`surface` enforcement-mode pair (EnforcementMode recut to
    // {note, warn, block}), and none may cite a retired `specs/architecture/*`
    // path (the layout recut under CITE-RETAG). `guard --help` must name all
    // three live modes.
    let mut all_help = String::new();
    for command in [
        "check", "schema", "emit", "guard", "install", "bundle", "explain",
    ] {
        let out = Command::new(BIN)
            .arg(command)
            .arg("--help")
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "`temper {command} --help` must exit zero"
        );
        all_help.push_str(&String::from_utf8(out.stdout).unwrap());
    }
    let top = Command::new(BIN).arg("--help").output().unwrap();
    assert!(top.status.success(), "temper --help must exit zero");
    all_help.push_str(&String::from_utf8(top.stdout).unwrap());

    assert!(
        !all_help.contains("specs/architecture"),
        "help text must not cite a retired specs/architecture/* path:\n{all_help}"
    );

    let guard_help = Command::new(BIN)
        .arg("guard")
        .arg("--help")
        .output()
        .unwrap();
    let guard_stdout = String::from_utf8(guard_help.stdout).unwrap();
    assert!(
        !guard_stdout.contains("shared") && !guard_stdout.contains("`surface`"),
        "`temper guard --help` must not speak the retired `shared`/`surface` mode pair, got:\n{guard_stdout}"
    );
    for mode in ["note", "warn", "block"] {
        assert!(
            guard_stdout.contains(mode),
            "`temper guard --help` must name the `{mode}` enforcement mode, got:\n{guard_stdout}"
        );
    }
}
