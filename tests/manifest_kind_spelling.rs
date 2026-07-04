//! The gate live-extracts in-place members from their committed landscape files and
//! dispatches each by its bare built-in kind (`specs/architecture/20-surface.md`, "The seam —
//! one implementation"; "In-place").
//!
//! An in-place `[[member]]` names a committed file and a bare built-in `kind`; the gate reads
//! the file, runs that kind's extractor, and dispatches to its package. These end-to-end
//! checks pin two halves: a bare `rule` member reaches the rule contract and is **checked**,
//! and a member whose kind resolves to no built-in surfaces a **loud error** rather than a
//! silent `checked 0` (`.claude/rules/collaboration.md`, "a silent skip reads as done").
//!
//! Driven across the real process boundary because the dispatch happens inside `check`'s gate
//! and its effect is observable only in the rendered diagnostics + exit code.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A rule carrying Cursor `.mdc` keys (`globs`/`alwaysApply`) Claude Code silently
/// ignores — the exact mistake the rule contract's `required` `forbidden_keys` clause
/// catches. If the member reaches the `rule` package, `check` fires and exits non-zero.
const FORBIDDEN_KEY_RULE: &str = "---\n\
globs: \"**/*.rs\"\n\
alwaysApply: true\n\
---\n\
# Rust conventions\n\
\n\
This frontmatter loads nothing in Claude Code.\n";

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "manifest-kind-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// The outcome of a `check` run: whether it exited zero and its combined stdout+stderr.
struct CheckRun {
    ok: bool,
    output: String,
}

/// Run `temper check` from `root` (so its `temper.toml` is discovered), capturing the
/// exit status and rendered diagnostics.
fn check_in(root: &Path) -> CheckRun {
    let out = Command::new(BIN)
        .current_dir(root)
        .arg("check")
        .output()
        .unwrap();
    let mut output = String::from_utf8_lossy(&out.stdout).into_owned();
    output.push_str(&String::from_utf8_lossy(&out.stderr));
    CheckRun {
        ok: out.status.success(),
        output,
    }
}

/// `init` a one-rule harness in place (`FORBIDDEN_KEY_RULE`), leaving an in-place `[[member]]`
/// over the committed landscape file — no copy tree. `spelling`, when given, rewrites the
/// member's bare `kind = "rule"` to drive the gate's dispatch. Returns the harness root.
fn in_place_rule(label: &str, spelling: Option<&str>) -> PathBuf {
    let root = tmpdir(label);
    let rules = root.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(rules.join("rust.md"), FORBIDDEN_KEY_RULE).unwrap();

    let status = Command::new(BIN).arg("init").arg(&root).status().unwrap();
    assert!(status.success(), "init should succeed: {status}");

    if let Some(spelling) = spelling {
        let manifest_path = root.join("temper.toml");
        let manifest = fs::read_to_string(&manifest_path).unwrap();
        let respelled = manifest.replace("kind = \"rule\"", &format!("kind = \"{spelling}\""));
        assert_ne!(
            manifest, respelled,
            "the init'd manifest must carry a bare `kind = \"rule\"` to re-spell"
        );
        fs::write(&manifest_path, respelled).unwrap();
    }
    root
}

#[test]
fn a_bare_builtin_in_place_member_reaches_its_package() {
    // The in-place member names the bare built-in `rule`; the gate live-extracts the committed
    // file and dispatches it to the rule contract, so the forbidden keys fire.
    let root = in_place_rule("bare-rule", None);
    let run = check_in(&root);

    assert!(
        !run.ok,
        "a `rule` member carrying forbidden keys must fail — proof it reached the rule \
         contract, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("forbidden_keys"),
        "the finding names the rule clause the member tripped, got:\n{}",
        run.output
    );
    // The coverage note counts the member under its resolved built-in identity — checked,
    // never a silent zero.
    assert!(
        run.output.contains("claude-code.rule (1)"),
        "the member is counted checked under its kind, got:\n{}",
        run.output
    );
}

#[test]
fn an_unrecognized_in_place_kind_is_a_loud_error_not_a_silent_zero() {
    // A member kind resolving to no built-in must be reported, never dropped to a silent
    // `checked 0` (`.claude/rules/collaboration.md`). In-place carriage is built-in-kind only,
    // so a non-built-in kind is a hard load error naming the offending kind.
    let root = in_place_rule("unknown-kind", Some("claude-code.widget"));
    let run = check_in(&root);

    assert!(
        !run.ok,
        "an unrecognized in-place kind must fail the run, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("non-built-in kind") && run.output.contains("claude-code.widget"),
        "the finding names the unresolved kind so the author knows what to fix, got:\n{}",
        run.output
    );
}
