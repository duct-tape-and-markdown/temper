//! End-to-end acceptance over requirement coverage — the referential shadow of the
//! meaningful contract (`specs/10-contracts.md`, "Requirements and `satisfies` — the
//! meaningful contract").
//!
//! Drives the built `temper` binary so the whole path is pinned: `temper.toml`
//! discovery at the project root, the `[requirement.<name>]` tables parsed onto the
//! author layer, the authored `[representation].satisfies` opt-in reconstructed off
//! each surface artifact, and the coverage gate's exit code. Mirrors
//! `tests/temper_toml.rs`: each case sets the process working directory to a project
//! root that carries a `temper.toml`, exactly as a real invocation would.
//!
//! The cases mirror the entry's acceptance:
//! - a `required` requirement with a skill declaring a resolving `satisfies` stays
//!   silent (covered ⇒ zero);
//! - a `required` requirement with no satisfying artifact fires UNFILLED (⇒ non-zero);
//! - a skill whose `satisfies` names no declared requirement fires DANGLING (⇒ non-zero);
//! - a non-`required` requirement left unfilled does not block (⇒ zero).

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
        "author-coverage-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A clean skill that trips no floor clause — the isolated subject for the coverage
/// gate, so the only findings a case sees are coverage ones.
const CLEAN_SKILL: &str = "---\n\
name: dev-standards\n\
description: Use when maintaining development standards across the harness.\n\
---\n\
# Dev standards\n\
\n\
Keep the bar high.\n";

/// Project a one-skill harness into `<root>/.temper` via the real `import` verb, so
/// the workspace `check` reads is built exactly as a user's would be.
fn import_skill(root: &Path, name: &str, skill_md: &str) {
    let harness = tmpdir("harness");
    let dir = harness.join("skills").join(name);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("SKILL.md"), skill_md).unwrap();

    let status = Command::new(BIN)
        .arg("import")
        .arg(&harness)
        .arg("--into")
        .arg(root.join(".temper"))
        .status()
        .unwrap();
    assert!(status.success(), "import should succeed: {status}");
}

/// Author the `[representation].satisfies` opt-in on an imported skill's surface
/// `meta.toml` — the binding the coverage resolver reads. `import` never writes it
/// (it is surface-authored, not frontmatter), so a case appends the table exactly as
/// a human editing the surface would.
fn author_satisfies(root: &Path, name: &str, requirements: &[&str]) {
    let meta = root
        .join(".temper")
        .join("skills")
        .join(name)
        .join("meta.toml");
    let mut contents = fs::read_to_string(&meta).unwrap();
    let list = requirements
        .iter()
        .map(|r| format!("\"{r}\""))
        .collect::<Vec<_>>()
        .join(", ");
    contents.push_str(&format!("\n[representation]\nsatisfies = [{list}]\n"));
    fs::write(&meta, contents).unwrap();
}

/// Write `<root>/temper.toml`.
fn write_temper_toml(root: &Path, contents: &str) {
    fs::write(root.join("temper.toml"), contents).unwrap();
}

/// The outcome of a `check` run: whether it exited zero and its combined
/// stdout+stderr (diagnostics render to stdout, a load error to stderr).
struct CheckRun {
    ok: bool,
    output: String,
}

/// Run `temper check` from `root` (so a `temper.toml` there is discovered) against
/// the default `./.temper` workspace, capturing the result.
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

#[test]
fn a_required_requirement_with_a_resolving_satisfies_stays_silent() {
    let root = tmpdir("covered");
    import_skill(&root, "dev-standards", CLEAN_SKILL);
    // The skill opts into the requirement, so its intent has a resolving home.
    author_satisfies(&root, "dev-standards", &["dev-standards"]);
    write_temper_toml(
        &root,
        "[requirement.dev-standards]\n\
means = \"the harness has a skill that maintains development standards\"\n\
required = true\n",
    );

    let run = check_in(&root);
    assert!(
        run.ok,
        "a covered required requirement must not block ⇒ zero, got:\n{}",
        run.output
    );
}

#[test]
fn a_required_requirement_with_no_satisfying_artifact_fires_unfilled() {
    let root = tmpdir("unfilled");
    import_skill(&root, "dev-standards", CLEAN_SKILL);
    // No `[representation].satisfies` authored: nothing opts into the requirement.
    write_temper_toml(
        &root,
        "[requirement.dev-standards]\n\
means = \"the harness has a skill that maintains development standards\"\n\
required = true\n",
    );

    let run = check_in(&root);
    assert!(
        !run.ok,
        "an unfilled required requirement must block ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("requirement.unfilled"),
        "the finding names the unfilled rule, got:\n{}",
        run.output
    );
}

#[test]
fn a_satisfies_naming_no_requirement_fires_dangling() {
    let root = tmpdir("dangling");
    import_skill(&root, "dev-standards", CLEAN_SKILL);
    // The skill opts into the required requirement (so no UNFILLED) *and* a second,
    // undeclared one — the link that dangles.
    author_satisfies(
        &root,
        "dev-standards",
        &["dev-standards", "ghost-requirement"],
    );
    write_temper_toml(
        &root,
        "[requirement.dev-standards]\n\
means = \"the harness has a skill that maintains development standards\"\n\
required = true\n",
    );

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a dangling `satisfies` must block ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("requirement.dangling"),
        "the finding names the dangling rule, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("ghost-requirement"),
        "the finding names the unresolvable link, got:\n{}",
        run.output
    );
}

#[test]
fn a_non_required_unfilled_requirement_does_not_block() {
    let root = tmpdir("advisory");
    import_skill(&root, "dev-standards", CLEAN_SKILL);
    // Nothing opts into it, but the requirement is advisory intent (no `required`),
    // so `temper` never fabricates a gate the author did not declare.
    write_temper_toml(
        &root,
        "[requirement.nice-to-have]\n\
means = \"an optional convenience the harness may provide\"\n",
    );

    let run = check_in(&root);
    assert!(
        run.ok,
        "a non-required unfilled requirement must not block ⇒ zero, got:\n{}",
        run.output
    );
}
