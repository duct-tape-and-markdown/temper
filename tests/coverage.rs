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
//! - a typo'd link yields the paired UNFILLED+DANGLING — exact-match precision, not one
//!   folding into the other (⇒ non-zero, both rules named);
//! - a duplicated `satisfies` entry emits exactly ONE dangling finding — the loop dedups
//!   per artifact (⇒ non-zero, one finding);
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

/// Author the `[satisfies.<requirement>]` opt-in modules on an imported skill's
/// surface `SKILL.md` document — the binding the coverage resolver reads. `import`
/// never writes them (they are surface-authored, not frontmatter), so a case adds
/// them exactly as a human editing the member document would, via the same
/// projection the tool uses.
fn author_satisfies(root: &Path, name: &str, requirements: &[&str]) {
    let dir = root.join(".temper").join("skills").join(name);
    let mut skill = temper::skill::Skill::from_dir(&dir).unwrap();
    skill.satisfies = requirements
        .iter()
        .map(|r| temper::document::Satisfies::new(*r))
        .collect();
    fs::write(dir.join("SKILL.md"), skill.to_document().emit()).unwrap();
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

/// Run `temper check --reporter github` from `root`, so findings render as one
/// `::error` workflow-command line per diagnostic — a stable substrate for counting
/// how many findings a case emits.
fn check_github(root: &Path) -> CheckRun {
    let out = Command::new(BIN)
        .current_dir(root)
        .arg("check")
        .arg("--reporter")
        .arg("github")
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
fn a_typo_in_a_satisfies_link_yields_paired_unfilled_and_dangling() {
    let root = tmpdir("typo");
    import_skill(&root, "dev-standards", CLEAN_SKILL);
    // The link misspells the requirement name. `satisfies` is exact-string matched,
    // never folded, so the real requirement goes UNFILLED (nothing resolves to it)
    // *and* the typo'd name DANGLES (it names no declared requirement). Both are true
    // positives — the pair is what pins exact-match precision, not one masking the
    // other.
    author_satisfies(&root, "dev-standards", &["dev-standatds"]);
    write_temper_toml(
        &root,
        "[requirement.dev-standards]\n\
means = \"the harness has a skill that maintains development standards\"\n\
required = true\n",
    );

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a typo'd link must block on both counts ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("requirement.unfilled"),
        "the real requirement is unfilled, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("requirement.dangling"),
        "the misspelled link dangles, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("dev-standatds"),
        "the dangling finding names the typo, got:\n{}",
        run.output
    );
}

#[test]
fn a_duplicated_satisfies_entry_emits_exactly_one_dangling() {
    let root = tmpdir("dup");
    import_skill(&root, "dev-standards", CLEAN_SKILL);
    // The skill covers the declared requirement (so no UNFILLED) and repeats the same
    // undeclared link. The coverage check dedups each artifact's `satisfies` before
    // the dangling loop, so the single unresolvable name yields exactly ONE
    // diagnostic — a duplicated link is not a doubled fault.
    author_satisfies(
        &root,
        "dev-standards",
        &["dev-standards", "ghost-requirement", "ghost-requirement"],
    );
    write_temper_toml(
        &root,
        "[requirement.dev-standards]\n\
means = \"the harness has a skill that maintains development standards\"\n\
required = true\n",
    );

    // The github reporter renders one `::error` line per diagnostic, a stable count.
    let run = check_github(&root);
    assert!(
        !run.ok,
        "a dangling link must block ⇒ non-zero, got:\n{}",
        run.output
    );
    let dangling = run.output.matches("requirement.dangling").count();
    assert_eq!(
        dangling, 1,
        "a duplicated `satisfies` must emit exactly one dangling finding, got {dangling} in:\n{}",
        run.output
    );
    // And no spurious unfilled — the requirement is covered by the first link.
    assert_eq!(
        run.output.matches("requirement.unfilled").count(),
        0,
        "the covered requirement must not fire unfilled, got:\n{}",
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

#[test]
fn a_means_less_required_requirement_still_gates() {
    let root = tmpdir("means-less");
    import_skill(&root, "dev-standards", CLEAN_SKILL);
    // The unified requirement makes `means` optional (`specs/10-contracts.md`, "all
    // facets optional except its name"), but coverage keys off `required`, not
    // `means`: a `required` requirement with no `means` and nothing opting in still
    // fires UNFILLED and blocks the run.
    write_temper_toml(
        &root,
        "[requirement.dev-standards]\n\
required = true\n",
    );

    let run = check_in(&root);
    assert!(
        !run.ok,
        "a means-less required requirement left unfilled must block ⇒ non-zero, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("requirement.unfilled"),
        "the finding names the unfilled rule, got:\n{}",
        run.output
    );
}
