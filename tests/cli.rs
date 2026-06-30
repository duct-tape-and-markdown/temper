//! End-to-end CLI acceptance over the documented surface (`specs/20-surface.md`,
//! "CLI surface"; `specs/10-contracts.md`, the contract engine `check` runs).
//!
//! Spawns the built `temper` binary via `CARGO_BIN_EXE_temper` and drives the
//! documented round trip — `temper import <harness> --into <tmp>` then
//! `temper check <tmp>` — asserting the exit semantics: zero on a clean skill,
//! non-zero once a `required`-severity contract clause is violated. A
//! `--deny-advisories` case pins the strict policy: an advisory-only run exits
//! zero by default but non-zero under the flag. A final case pins the default
//! workspace: with `--into` / the `check` argument omitted, both resolve to
//! `./.temper` under the process's working directory.
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

/// Write a one-skill harness at `<root>/skills/<name>/SKILL.md`.
fn write_harness(root: &Path, name: &str, skill_md: &str) {
    let dir = root.join("skills").join(name);
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
/// `import` scans for the rule kind (`specs/20-surface.md`).
fn write_rule_harness(root: &Path, name: &str, rule_md: &str) {
    let dir = root.join(".claude").join("rules");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join(format!("{name}.md")), rule_md).unwrap();
}

/// Run `temper import <harness> --into <into>` and assert it succeeded.
fn import(harness: &Path, into: &Path) {
    let status = Command::new(BIN)
        .arg("import")
        .arg(harness)
        .arg("--into")
        .arg(into)
        .status()
        .unwrap();
    assert!(status.success(), "import should succeed: {status}");
}

/// Run `temper check <workspace> [extra…]` and return whether it exited zero.
fn run_check(workspace: &Path, extra: &[&str]) -> bool {
    Command::new(BIN)
        .arg("check")
        .arg(workspace)
        .args(extra)
        .status()
        .unwrap()
        .success()
}

/// Run `temper check <workspace>` and return whether it exited zero.
fn check_succeeds(workspace: &Path) -> bool {
    run_check(workspace, &[])
}

#[test]
fn import_then_check_is_clean_for_a_well_formed_skill() {
    let harness = tmpdir("clean-src");
    write_harness(&harness, "coordinate", CLEAN_SKILL);
    let into = tmpdir("clean-into");

    import(&harness, &into);
    assert!(
        check_succeeds(&into),
        "a clean skill must exit zero (no error-severity diagnostics)"
    );
}

#[test]
fn check_exits_non_zero_when_an_error_rule_fires() {
    let harness = tmpdir("error-src");
    // Directory `coordinate` but `name: Coordinate` — trips name-format and
    // name-matches-dir, both error severity.
    write_harness(&harness, "coordinate", ERROR_SKILL);
    let into = tmpdir("error-into");

    import(&harness, &into);
    assert!(
        !check_succeeds(&into),
        "an error-severity diagnostic must make check exit non-zero"
    );
}

#[test]
fn deny_advisories_promotes_a_warn_only_run_to_a_failure() {
    let harness = tmpdir("advisory-src");
    // The only clause this skill violates is the advisory `max_lines` budget.
    write_harness(&harness, "coordinate", &advisory_only_skill());
    let into = tmpdir("advisory-into");

    import(&harness, &into);
    // Default policy: an advisory-only run is clean — warn does not gate.
    assert!(
        check_succeeds(&into),
        "an advisory-only violation must exit zero without --deny-advisories"
    );
    // Strict policy: --deny-advisories promotes the warn to a blocking failure.
    assert!(
        !run_check(&into, &["--deny-advisories"]),
        "an advisory-only violation must exit non-zero under --deny-advisories"
    );
}

#[test]
fn import_then_check_dispatches_the_rule_kind_to_the_rule_contract() {
    // A clean rule (`paths:`-only) trips no `required` clause ⇒ check is zero.
    let clean_src = tmpdir("rule-clean-src");
    write_rule_harness(&clean_src, "rust", CLEAN_RULE);
    let clean_into = tmpdir("rule-clean-into");
    import(&clean_src, &clean_into);
    assert!(
        check_succeeds(&clean_into),
        "a clean rule must exit zero — the rule contract has no `required` violation"
    );

    // A forbidden Cursor key (`globs`/`alwaysApply`) trips the `forbidden_keys`
    // clause, which is `required` ⇒ check is non-zero. This proves `check`
    // dispatches the rule kind to the rule contract, not the skill one.
    let forbidden_src = tmpdir("rule-forbidden-src");
    write_rule_harness(&forbidden_src, "rust", FORBIDDEN_KEY_RULE);
    let forbidden_into = tmpdir("rule-forbidden-into");
    import(&forbidden_src, &forbidden_into);
    assert!(
        !check_succeeds(&forbidden_into),
        "a forbidden-key rule must exit non-zero (the rule contract's required clause)"
    );
}

#[test]
fn self_host_check_is_clean_over_tempers_own_rules() {
    // The bootstrap proof (`specs/00-intent.md`): import `temper`'s OWN repo —
    // whose `.claude/rules/` carries `rust.md` (`paths:`) and `collaboration.md`
    // (no frontmatter) — and `check` its own house clean. `CARGO_MANIFEST_DIR` is
    // the crate root, the harness root `import` scans for `.claude/rules/`.
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let into = tmpdir("self-host-into");
    import(repo_root, &into);
    assert!(
        check_succeeds(&into),
        "temper must lint its own .claude/rules/ clean — the self-hosting finish line"
    );
}

#[test]
fn into_and_workspace_default_to_dot_author() {
    // With `--into` omitted, import writes to `./.temper` relative to the
    // process CWD; with the `check` argument omitted, check reads the same path.
    let cwd = tmpdir("default-cwd");
    let harness = tmpdir("default-src");
    write_harness(&harness, "coordinate", CLEAN_SKILL);

    let import_status = Command::new(BIN)
        .current_dir(&cwd)
        .arg("import")
        .arg(&harness)
        .status()
        .unwrap();
    assert!(
        import_status.success(),
        "default-into import should succeed"
    );

    // The default surface landed under `<cwd>/.temper`.
    let default_ws = cwd.join(".temper");
    assert!(
        default_ws.join("author.toml").is_file(),
        "import without --into must resolve to ./.temper"
    );
    assert!(
        default_ws
            .join("skills")
            .join("coordinate")
            .join("meta.toml")
            .is_file()
    );

    // `check` with no argument reads that same `./.temper` and finds it clean.
    let check_status = Command::new(BIN)
        .current_dir(&cwd)
        .arg("check")
        .status()
        .unwrap();
    assert!(
        check_status.success(),
        "check without an argument must lint ./.temper and exit zero"
    );
}
