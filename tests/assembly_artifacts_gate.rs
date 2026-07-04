//! The gate reads the SDK-emitted `roster.toml`/`bindings.toml` as the assembly source
//! (`specs/architecture/20-surface.md`, "the bindings, the roster — are emitted as small
//! committed temper-owned artifacts"; GATE-READS-ASSEMBLY).
//!
//! The members are read from the committed surface corpus (`specs/architecture/20-surface.md`,
//! "The seam"); a member's `satisfies` names a requirement that lives in `roster.toml`, not
//! in any member — so with no roster the gate reports `requirement.dangling`. These
//! end-to-end checks pin both halves: with the artifacts present the run is green (the
//! roster's requirements gate as declared), and with the roster removed the same corpus
//! goes red with the dangling finding — proof the roster is what resolves the join.
//!
//! Driven across the real process boundary because the read happens inside `check`'s gate
//! and its effect is observable only in the rendered diagnostics + exit code.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A floor-clean skill whose directory matches its name, satisfying the `name-matches-dir`
/// and description clauses so the only finding a case can produce is a roster one.
const SKILL: &str = "---\n\
    name: coordinate\n\
    description: Use when coordinating a complex task across a team of agents; not otherwise.\n\
    ---\n\
    # Coordinate\n\
    \n\
    Drive the team.\n";

/// A floor-clean rule — `paths` frontmatter, no Cursor `.mdc` keys the rule package forbids.
const RULE: &str = "---\n\
    paths: \"src/**/*.rs\"\n\
    ---\n\
    # Rust conventions\n\
    \n\
    Errors via miette/thiserror.\n";

/// The requirement roster `emit` lands beside a members-only `temper.toml` — each
/// requirement carrying its `means`/`kind`/`required`, the kind qualified as the SDK
/// stamps it (`sdk/src/assembly_artifacts.ts`, `serializeRoster`).
const ROSTER: &str = "[requirement.agent-playbook]\n\
    means = \"a shared agent playbook exists\"\n\
    kind = \"claude-code.skill\"\n\
    required = true\n\
    \n\
    [requirement.engineering-standards]\n\
    means = \"the repo carries a rule fixing the engineering bar\"\n\
    kind = \"claude-code.rule\"\n";

/// The kind→package bindings `emit` lands beside the manifest — the dotted kind quoted
/// into a single sub-key (`sdk/src/assembly_artifacts.ts`, `serializeBindings`).
const BINDINGS: &str = "[binding.\"claude-code.rule\"]\n\
    package = \"rule.anthropic\"\n\
    \n\
    [binding.\"claude-code.skill\"]\n\
    package = \"skill.anthropic\"\n";

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "assembly-artifacts-gate-{}-{}-{}",
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

/// Run `temper check` from `root` (so its `temper.toml` and the assembly-fact artifacts
/// beside it are discovered), capturing the exit status and rendered diagnostics.
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

/// Inject a `[satisfies.<requirement>]` opt-in into a committed surface document's `+++`
/// header — the member-side recognition the gate reads off the committed corpus and the
/// roster resolves. The document opens with a `+++` fence, so the block lands right after
/// it (`specs/architecture/20-surface.md`, "The member — satisfies").
fn inject_satisfies(doc_path: &Path, requirement: &str, rationale: &str) {
    let text = fs::read_to_string(doc_path).unwrap();
    let block = format!("[satisfies.{requirement}]\nrationale = \"{rationale}\"\n\n");
    let injected = text.replacen("+++\n", &format!("+++\n{block}"), 1);
    assert_ne!(
        text, injected,
        "the surface document must open with a `+++` header to inject satisfies into"
    );
    fs::write(doc_path, injected).unwrap();
}

/// Build the scenario: `import` a floor-clean skill+rule harness, then author each member's
/// `satisfies` recognition into its **committed surface document** — the corpus the gate
/// reads (`specs/architecture/20-surface.md`, "The seam"). A `satisfies` names a requirement
/// that lives in the roster beside the manifest, not in any member, so a roster requirement
/// resolves off the artifacts or the join dangles. Returns the harness root (the caller
/// decides which assembly-fact artifacts sit beside it).
fn surface_harness_with_satisfies(label: &str) -> PathBuf {
    let root = tmpdir(label);
    let skills = root.join(".claude").join("skills").join("coordinate");
    let rules = root.join(".claude").join("rules");
    fs::create_dir_all(&skills).unwrap();
    fs::create_dir_all(&rules).unwrap();
    fs::write(skills.join("SKILL.md"), SKILL).unwrap();
    fs::write(rules.join("rust.md"), RULE).unwrap();

    let status = Command::new(BIN)
        .arg("import")
        .arg(&root)
        .arg("--into")
        .arg(root.join(".temper"))
        .status()
        .unwrap();
    assert!(status.success(), "import should succeed: {status}");

    // The committed surface documents carry no mined `satisfies` (the source frontmatter
    // declares none); author the recognition into each so the gate reads it off the corpus.
    inject_satisfies(
        &root
            .join(".temper")
            .join("skills")
            .join("coordinate")
            .join("SKILL.md"),
        "agent-playbook",
        "the shared agent playbook",
    );
    inject_satisfies(
        &root
            .join(".temper")
            .join("rules")
            .join("rust")
            .join("RULE.md"),
        "engineering-standards",
        "the Rust engineering bar",
    );
    root
}

#[test]
fn a_committed_corpus_with_the_artifacts_checks_green() {
    // The roster + bindings sit beside the committed surface corpus. The gate reads them as
    // the assembly source, so each member's `satisfies` resolves to a declared requirement
    // and the required one is filled — a clean run, no spurious dangling.
    let root = surface_harness_with_satisfies("with-artifacts");
    fs::write(root.join("roster.toml"), ROSTER).unwrap();
    fs::write(root.join("bindings.toml"), BINDINGS).unwrap();

    let run = check_in(&root);
    assert!(
        run.ok,
        "a committed corpus with roster/bindings beside it must check green, got:\n{}",
        run.output
    );
    assert!(
        !run.output.contains("requirement.dangling"),
        "no `satisfies` may dangle — the roster declares every requirement, got:\n{}",
        run.output
    );
    // The required requirement gates as declared: it is filled, so no unfilled finding either.
    assert!(
        !run.output.contains("requirement.unfilled"),
        "the required `agent-playbook` is filled by the skill member, got:\n{}",
        run.output
    );
}

#[test]
fn the_same_corpus_without_the_roster_dangles() {
    // The control: the identical corpus, but no `roster.toml` beside it. With no assembly
    // source to read, each member's `satisfies` names a requirement that exists nowhere —
    // the dangling finding. This proves the roster is what resolves the join.
    let root = surface_harness_with_satisfies("without-roster");

    let run = check_in(&root);
    assert!(
        !run.ok,
        "with no roster, the committed corpus's `satisfies` links must dangle, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("requirement.dangling"),
        "the missing roster leaves every `satisfies` dangling, got:\n{}",
        run.output
    );
}
