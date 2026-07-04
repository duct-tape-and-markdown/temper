//! The gate reads the SDK-emitted `roster.toml`/`bindings.toml` as the assembly source
//! (`specs/architecture/20-surface.md`, "the bindings, the roster — are emitted as small
//! committed temper-owned artifacts"; GATE-READS-ASSEMBLY).
//!
//! `emit` compiles a **members-only** `temper.toml` and lands the requirement roster and
//! the kind bindings in two locus-less temper-owned files beside it. A member's
//! `satisfies` names a requirement that lives in `roster.toml`, not the manifest — so
//! before this fix the gate saw no such requirement and reported a spurious
//! `requirement.dangling`. These end-to-end checks pin both halves: with the artifacts
//! present the run is green (the roster's requirements gate as declared), and with the
//! roster removed the same manifest goes red with the dangling finding the fix removes —
//! proof the roster is what resolves the join.
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

/// Build the SDK-shaped scenario: `import` a floor-clean skill+rule harness into a
/// persistent manifest, then re-spell each member's `kind` to the **qualified** identity
/// the SDK stamps and graft on the `satisfies` recognition, so the manifest reads exactly
/// as an SDK members-only emit does. The surface trees are stripped so the manifest
/// members are the sole corpus — a roster requirement resolves off the artifacts or not
/// at all. Returns the harness root (its `temper.toml` carries the members; the caller
/// decides which artifacts sit beside it).
fn members_only_manifest(label: &str) -> PathBuf {
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

    // `import` writes each member under its bare `kind` and unrecognized (no `satisfies`).
    // Re-spell to the SDK's qualified identity and graft the recognition — the member-side
    // join the roster resolves. `kind = "rule"`/`kind = "skill"` are the member-header
    // lines only (never a field), so the replace is unambiguous.
    let manifest_path = root.join("temper.toml");
    let manifest = fs::read_to_string(&manifest_path).unwrap();
    let respelled = manifest
        .replace(
            "kind = \"rule\"",
            "kind = \"claude-code.rule\"\nsatisfies = [\"engineering-standards\"]",
        )
        .replace(
            "kind = \"skill\"",
            "kind = \"claude-code.skill\"\nsatisfies = [\"agent-playbook\"]",
        );
    assert_ne!(
        manifest, respelled,
        "the imported manifest must carry bare `kind` lines to re-spell"
    );
    fs::write(&manifest_path, respelled).unwrap();

    // Strip the surface so the manifest members are the only corpus — no copy-tree
    // fallback masking whether the roster resolved the join.
    fs::remove_dir_all(root.join(".claude")).unwrap();
    fs::remove_dir_all(root.join(".temper").join("skills")).unwrap();
    fs::remove_dir_all(root.join(".temper").join("rules")).unwrap();
    root
}

#[test]
fn a_members_only_manifest_with_the_artifacts_checks_green() {
    // The roster + bindings sit beside the members-only manifest. The gate reads them as
    // the assembly source, so each member's `satisfies` resolves to a declared requirement
    // and the required one is filled — a clean run, no spurious dangling.
    let root = members_only_manifest("with-artifacts");
    fs::write(root.join("roster.toml"), ROSTER).unwrap();
    fs::write(root.join("bindings.toml"), BINDINGS).unwrap();

    let run = check_in(&root);
    assert!(
        run.ok,
        "an SDK members-only manifest with roster/bindings beside it must check green, got:\n{}",
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
fn the_same_manifest_without_the_roster_dangles() {
    // The control: the identical manifest, but no `roster.toml` beside it. With no assembly
    // source to read, each member's `satisfies` names a requirement that exists nowhere —
    // the spurious dangling the fix removes. This proves the roster is what resolves it.
    let root = members_only_manifest("without-roster");

    let run = check_in(&root);
    assert!(
        !run.ok,
        "with no roster, the members-only manifest's `satisfies` links must dangle, got:\n{}",
        run.output
    );
    assert!(
        run.output.contains("requirement.dangling"),
        "the missing roster leaves every `satisfies` dangling, got:\n{}",
        run.output
    );
}
