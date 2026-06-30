//! End-to-end CLI acceptance for slice 1 (`spec/RELEASE-v0.1.md`, "Surface").
//!
//! Spawns the built `temper` binary via `CARGO_BIN_EXE_author` and drives the
//! documented round trip — `temper import <harness> --into <tmp>` then
//! `temper check <tmp>` — asserting the exit semantics: zero on a clean skill,
//! non-zero once an `error`-severity rule fires. A third case pins the default
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
const BIN: &str = env!("CARGO_BIN_EXE_author");

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

/// A skill that trips `error` rules: the uppercase `name` is outside `[a-z0-9-]`
/// (`skill.name-format`) and no longer equals its directory (`skill.name-matches-dir`).
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

/// Run `temper check <workspace>` and return whether it exited zero.
fn check_succeeds(workspace: &Path) -> bool {
    Command::new(BIN)
        .arg("check")
        .arg(workspace)
        .status()
        .unwrap()
        .success()
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
