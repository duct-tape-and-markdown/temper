//! `temper install` — projecting the gate's wiring under the three-state drift
//! engine (`specs/architecture/50-distribution.md`, "Decision: `install` projects the gate's
//! wiring; drift keeps it synced").
//!
//! Drives the library `install::run` / `install::gate_installed` over a real
//! harness and proves the four properties the entry names:
//!
//! - **projection** — one run writes the `SessionStart` hook into
//!   `.claude/settings.json` (merged, preserving what was there), the CI job into
//!   `.github/workflows/`, and the schema modeline into each artifact's frontmatter;
//! - **idempotence** — a second run lands every placement `Unchanged` and touches
//!   not a byte;
//! - **dry-run** — `--dry-run` reports every outcome but writes nothing;
//! - **self-verify** — a hand-drifted placement is flagged by `gate_installed`.
//!
//! A final CLI case drives the real `temper install` binary across the process
//! boundary, since `main`'s dispatch (and the default `.` root) is observable only
//! there.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::drift::ApplyOutcome;
use temper::install::{self, InstallReport};

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-install-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// A skill with frontmatter — the modeline placement inserts a modeline into it.
const SKILL: &str = "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// A rule with `paths:` frontmatter — carries a modeline too.
const RULE: &str = "---\n\
paths:\n\
  - \"src/**/*.rs\"\n\
---\n\
# Rust conventions\n\
\n\
Prefer a clone over a lifetime fight.\n";

/// A rule with no frontmatter — the modeline placement skips it (nothing to
/// validate), so a human's frontmatter-free file is never rewritten.
const COLLAB_RULE: &str = "# Collaboration\n\nPushback is the point.\n";

/// A pre-existing `.claude/settings.json` carrying an unrelated hook, so the merge
/// can be proven additive — the human's content survives the `SessionStart` graft.
const EXISTING_SETTINGS: &str =
    "{\n  \"permissions\": {\n    \"allow\": [\"Bash(cargo test:*)\"]\n  }\n}\n";

/// Build a harness with a skill, two rules (one frontmatter-free), and a
/// pre-existing settings file, and return its root.
fn write_harness(label: &str, with_settings: bool) -> PathBuf {
    let root = tmpdir(label);
    let skill = root.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(&skill).unwrap();
    fs::write(skill.join("SKILL.md"), SKILL).unwrap();

    let rules = root.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(rules.join("rust.md"), RULE).unwrap();
    fs::write(rules.join("collaboration.md"), COLLAB_RULE).unwrap();

    if with_settings {
        fs::write(
            root.join(".claude").join("settings.json"),
            EXISTING_SETTINGS,
        )
        .unwrap();
    }
    root
}

/// Snapshot every file under `dir` as a sorted map of relative path -> bytes.
fn tree_bytes(dir: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut out = BTreeMap::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        for entry in fs::read_dir(&current).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let rel = path.strip_prefix(dir).unwrap().to_path_buf();
                out.insert(rel, fs::read(&path).unwrap());
            }
        }
    }
    out
}

/// The outcome `install` reported for the placement at the file named `file`
/// (a path suffix match), asserting it is unique.
fn outcome_for(report: &InstallReport, file: &str) -> ApplyOutcome {
    let mut matches = report
        .entries
        .iter()
        .filter(|e| e.path.to_string_lossy().ends_with(file));
    let found = matches
        .next()
        .unwrap_or_else(|| panic!("no entry for {file}"));
    assert!(
        matches.next().is_none(),
        "entry for {file} should be unique"
    );
    found.outcome
}

/// The outcome `install` reported for the placement labeled `placement`, asserting it
/// is unique — the by-label lookup for placements that share a file (the SessionStart
/// hook and the guard both land in `settings.json`).
fn outcome_of(report: &InstallReport, placement: &str) -> ApplyOutcome {
    let mut matches = report.entries.iter().filter(|e| e.placement == placement);
    let found = matches
        .next()
        .unwrap_or_else(|| panic!("no entry for placement {placement}"));
    assert!(
        matches.next().is_none(),
        "placement {placement} should be unique"
    );
    found.outcome
}

#[test]
fn install_projects_the_three_placements() {
    let root = write_harness("projects", true);

    let report = install::run(&root, false).unwrap();

    // 1. The SessionStart hook is merged into settings.json — additively, so the
    //    pre-existing permissions block survives beside the grafted hook.
    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    assert_eq!(
        json["hooks"]["SessionStart"][0]["hooks"][0]["command"],
        "temper session-start ."
    );
    assert_eq!(
        json["permissions"]["allow"][0], "Bash(cargo test:*)",
        "the merge must preserve the human's existing settings"
    );
    // The SessionStart hook and the guard share settings.json — each reported by label.
    assert_eq!(
        outcome_of(&report, "session-start hook"),
        ApplyOutcome::Applied
    );
    assert_eq!(outcome_of(&report, "guard hook"), ApplyOutcome::Applied);

    // 2. The CI job lands as a whole file under .github/workflows/.
    let ci = root.join(".github").join("workflows").join("temper.yml");
    assert!(ci.is_file(), "the CI job must be written");
    assert!(
        fs::read_to_string(&ci).unwrap().contains("temper check"),
        "the CI job must run the gate"
    );
    assert_eq!(outcome_for(&report, "temper.yml"), ApplyOutcome::Applied);

    // 3. The schema modeline is inserted as the first frontmatter line of each
    //    artifact that HAS frontmatter — the skill and the `paths:` rule.
    let skill_md = fs::read_to_string(
        root.join(".claude")
            .join("skills")
            .join("coordinate")
            .join("SKILL.md"),
    )
    .unwrap();
    assert!(
        skill_md.starts_with(
            "---\n# yaml-language-server: $schema=../../../.temper/schema/skill.json\n"
        ),
        "the skill modeline must lead the frontmatter, got:\n{skill_md}"
    );
    // The body and the other frontmatter fields are preserved byte-for-byte.
    assert!(skill_md.contains("name: coordinate\n"));
    assert!(skill_md.ends_with("Drive the team through the playbook.\n"));

    let rust_md = fs::read_to_string(root.join(".claude").join("rules").join("rust.md")).unwrap();
    assert!(
        rust_md.contains("# yaml-language-server: $schema=../../.temper/schema/rule.json"),
        "the rule modeline must reference the rule schema, got:\n{rust_md}"
    );

    // A frontmatter-free rule is left untouched — nothing to validate, so no header
    // is synthesised and no placement entry is emitted for it.
    let collab =
        fs::read_to_string(root.join(".claude").join("rules").join("collaboration.md")).unwrap();
    assert_eq!(
        collab, COLLAB_RULE,
        "a frontmatter-free rule must be untouched"
    );
    assert!(
        report
            .entries
            .iter()
            .all(|e| !e.path.to_string_lossy().ends_with("collaboration.md")),
        "no modeline placement is emitted for a frontmatter-free artifact"
    );
}

#[test]
fn a_second_run_is_unchanged_and_writes_nothing() {
    let root = write_harness("idempotent", true);

    install::run(&root, false).unwrap();
    let after_first = tree_bytes(&root);

    // The second run finds every placement already in its desired state.
    let report = install::run(&root, false).unwrap();
    assert!(
        report
            .entries
            .iter()
            .all(|e| e.outcome == ApplyOutcome::Unchanged),
        "a re-install must land every placement Unchanged, got: {:?}",
        report.entries
    );
    assert_eq!(
        after_first,
        tree_bytes(&root),
        "the idempotent re-run must not change a single byte"
    );
}

#[test]
fn dry_run_reports_outcomes_but_writes_nothing() {
    let root = write_harness("dry", true);
    let before = tree_bytes(&root);

    let report = install::run(&root, true).unwrap();
    // The report still says every placement *would* be written...
    assert!(
        report
            .entries
            .iter()
            .all(|e| e.outcome == ApplyOutcome::Applied),
        "a first dry run reports Applied for every placement, got: {:?}",
        report.entries
    );
    // ...but nothing landed: no .github, no modeline, no hook in settings.
    assert_eq!(before, tree_bytes(&root), "--dry-run must write nothing");
    assert!(!root.join(".github").exists());
}

#[test]
fn gate_installed_summarizes_missing_then_drifted_placements() {
    let root = write_harness("self-verify", true);

    // Before install: every placement is missing, but the self-verify collapses them
    // to ONE summary advisory carrying the counts — never one warn per placement,
    // which on a real target would bury the artifact findings the gate exists to show.
    let before = install::gate_installed(&root);
    assert_eq!(
        before.len(),
        1,
        "an uninstalled gate yields exactly one summary advisory, got: {before:?}"
    );
    let summary = &before[0];
    assert_eq!(
        summary.severity,
        temper::check::Severity::Warn,
        "the self-verify is advisory — never error"
    );
    assert!(
        summary.message.contains("temper install"),
        "the summary points at the fix, got: {}",
        summary.message
    );
    // The counts name every missing placement kind — the per-placement detail folded
    // into the message body, not sibling diagnostics.
    assert!(
        summary.message.contains("session-start hook")
            && summary.message.contains("ci job")
            && summary.message.contains("schema modeline"),
        "the summary carries the missing-placement counts, got: {}",
        summary.message
    );

    // After a full install the gate is clean — nothing to report.
    install::run(&root, false).unwrap();
    assert!(
        install::gate_installed(&root).is_empty(),
        "a fully installed gate reports no drift, got: {:?}",
        install::gate_installed(&root)
    );

    // Hand-drift one placement: a human edits the CI job out from under temper.
    let ci = root.join(".github").join("workflows").join("temper.yml");
    fs::write(&ci, "name: not-temper\n").unwrap();

    let after = install::gate_installed(&root);
    assert_eq!(
        after.len(),
        1,
        "a single drifted placement still yields one summary advisory, got: {after:?}"
    );
    assert!(
        after[0].message.contains("ci job"),
        "the summary names the drifted placement, got: {}",
        after[0].message
    );
    assert!(
        after[0].message.contains("temper install"),
        "the diagnostic points at the fix, got: {}",
        after[0].message
    );
}

/// The managed-by note's marker — the frontmatter comment prefix `install` writes and
/// `emit` must never reproduce (kept in step with `install.rs`'s `NOTE_MARKER`).
const NOTE_MARKER: &str = "# temper: managed projection";

/// The guard command the `PreToolUse` hook carries after an install at `root`.
fn guard_command(root: &Path) -> String {
    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    json["hooks"]["PreToolUse"][0]["hooks"][0]["command"]
        .as_str()
        .expect("the guard hook must carry a command")
        .to_string()
}

#[test]
fn surface_posture_authors_a_blocking_guard_and_notes_non_memory() {
    let root = write_harness("surface", true);
    // The assembly declares the enforcing posture; a memory projection sits at the root.
    fs::write(root.join("temper.toml"), "authority = \"surface\"\n").unwrap();
    let memory = "# Project memory\n\nHand-authored, no frontmatter.\n";
    fs::write(root.join("CLAUDE.md"), memory).unwrap();

    // Before install: the self-verify audits BOTH new placements beside the old three.
    let before = install::gate_installed(&root);
    assert_eq!(before.len(), 1, "one summary advisory, got: {before:?}");
    assert!(
        before[0].message.contains("guard hook") && before[0].message.contains("managed-by note"),
        "gate-installed audits the guard and the note, got: {}",
        before[0].message
    );

    install::run(&root, false).unwrap();

    // The guard blocks (exit 2) under `surface`, and its message states the limit.
    let guard = guard_command(&root);
    assert!(
        guard.contains("exit 2"),
        "the surface guard must block, got: {guard}"
    );
    assert!(
        guard.contains("other tools writes are not bound by it"),
        "the guard states the limit verbatim, got: {guard}"
    );

    // The managed-by note lands on the non-memory frontmatter projections...
    let skill_md = fs::read_to_string(
        root.join(".claude")
            .join("skills")
            .join("coordinate")
            .join("SKILL.md"),
    )
    .unwrap();
    let rust_md = fs::read_to_string(root.join(".claude").join("rules").join("rust.md")).unwrap();
    assert!(
        skill_md.contains(NOTE_MARKER),
        "the note lands on the skill, got:\n{skill_md}"
    );
    assert!(
        rust_md.contains(NOTE_MARKER),
        "the note lands on the rule, got:\n{rust_md}"
    );

    // ...but NEVER on the memory projection — a comment there costs context every session.
    let claude_md = fs::read_to_string(root.join("CLAUDE.md")).unwrap();
    assert_eq!(
        claude_md, memory,
        "the memory projection must be untouched — no note"
    );
}

#[test]
fn shared_posture_authors_a_warning_guard() {
    // No temper.toml ⇒ the default `shared` posture: the guard informs and routes,
    // never blocks.
    let root = write_harness("shared", true);
    install::run(&root, false).unwrap();

    let guard = guard_command(&root);
    assert!(
        !guard.contains("exit 2"),
        "the shared guard must not block, got: {guard}"
    );
    assert!(
        guard.contains("temper-managed projection") && guard.contains("exit 0"),
        "the shared guard warns-and-routes (advisory), got: {guard}"
    );
    // The note is the universal layer — present under `shared` too.
    let rust_md = fs::read_to_string(root.join(".claude").join("rules").join("rust.md")).unwrap();
    assert!(rust_md.contains(NOTE_MARKER), "the note lands under shared");
}

#[test]
fn a_posture_change_rewrites_the_guard_in_place() {
    // Install under the default `shared`, then flip the assembly to `surface`: the
    // guard is REPLACED (block for warn), never a second one appended.
    let root = write_harness("posture-change", true);
    install::run(&root, false).unwrap();
    assert!(!guard_command(&root).contains("exit 2"), "starts advisory");

    fs::write(root.join("temper.toml"), "authority = \"surface\"\n").unwrap();
    let report = install::run(&root, false).unwrap();

    assert!(
        guard_command(&root).contains("exit 2"),
        "the flipped posture blocks"
    );
    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    assert_eq!(
        json["hooks"]["PreToolUse"].as_array().unwrap().len(),
        1,
        "a posture change rewrites the guard, never appends a second"
    );
    assert_eq!(
        outcome_of(&report, "guard hook"),
        ApplyOutcome::Applied,
        "the rewritten guard reports as (re)applied"
    );
}

#[test]
fn the_note_and_guard_name_the_ratified_drift_remedy() {
    // READD-RETIRE deleted the `re-add` verb; the managed-by note and guard message
    // must name the ratified remedy — edit the owning `.temper/` source and re-run
    // `temper emit` — never the retired verb (`specs/architecture/20-surface.md`,
    // `re-add` retired). Re-placing the reworded strings stays idempotent.
    let root = write_harness("drift-strings", true);
    install::run(&root, false).unwrap();

    let guard = guard_command(&root);
    assert!(
        guard.contains("re-run temper emit"),
        "the guard routes to `temper emit`, got: {guard}"
    );
    assert!(
        !guard.contains("re-add"),
        "the guard must not name the retired `re-add` verb, got: {guard}"
    );

    let rust_md = fs::read_to_string(root.join(".claude").join("rules").join("rust.md")).unwrap();
    assert!(
        rust_md.contains("re-run temper emit"),
        "the managed-by note routes to `temper emit`, got:\n{rust_md}"
    );
    assert!(
        !rust_md.contains("re-add"),
        "the note must not name the retired `re-add` verb, got:\n{rust_md}"
    );

    // Placing the reworded strings over their prior placement is a no-op: every guard
    // and managed-by-note entry lands Unchanged, so no reword reintroduces churn.
    let report = install::run(&root, false).unwrap();
    assert!(
        report
            .entries
            .iter()
            .filter(|e| e.placement == "guard hook" || e.placement == "managed-by note")
            .all(|e| e.outcome == ApplyOutcome::Unchanged),
        "a re-install leaves the reworded note and guard Unchanged, got: {:?}",
        report.entries
    );
}

/// The schema modeline's marker — the frontmatter comment `install` places and `emit`
/// round-trips (kept in step with `install.rs`'s `MODELINE_MARKER`).
const MODELINE_MARKER: &str = "# yaml-language-server:";

/// The skill and `paths:`-rule projections a frontmatter-carrying harness exposes to
/// the modeline/note placements — relative to the harness root.
fn frontmatter_projections() -> [PathBuf; 2] {
    [
        PathBuf::from(".claude")
            .join("skills")
            .join("coordinate")
            .join("SKILL.md"),
        PathBuf::from(".claude").join("rules").join("rust.md"),
    ]
}

/// Import `harness` into a fresh surface and emit it back onto the harness sources —
/// the round-trip the two projectors share. Returns nothing; the caller re-reads the
/// projections `emit` wrote.
fn import_then_emit(harness: &Path, label: &str) {
    let into = tmpdir(label);
    temper::import::run(harness, &into).unwrap();
    let ws = temper::check::Workspace::load(&into).unwrap();
    temper::drift::emit(
        &ws,
        &into,
        temper::drift::EmitOptions {
            dry_run: false,
            frozen: false,
        },
    )
    .unwrap();
}

#[test]
fn emit_never_stamps_the_managed_by_note() {
    // The managed-by note (and the schema modeline) ride `install`, not the surface
    // body — a YAML comment is not authored content (law 5). So an `emit` over a
    // projection that carries no note never invents one: emit preserves install's
    // placements, it does not originate them. (The complementary direction — an
    // install-placed note SURVIVES emit — is `install_placements_survive_a_subsequent_emit`.)
    let harness = write_harness("emit-no-note", false);
    // No `install` runs, so the sources carry neither the note nor the modeline.
    import_then_emit(&harness, "emit-no-note-into");

    for rel in frontmatter_projections() {
        let projected = fs::read_to_string(harness.join(&rel)).unwrap();
        assert!(
            !projected.contains(NOTE_MARKER),
            "emit must never stamp the managed-by note, got in {}:\n{projected}",
            rel.display()
        );
        assert!(
            !projected.contains(MODELINE_MARKER),
            "emit must never stamp the schema modeline, got in {}:\n{projected}",
            rel.display()
        );
    }
}

#[test]
fn install_placements_survive_a_subsequent_emit() {
    // The two-projectors seam (`specs/architecture/20-surface.md`): `install` places the
    // schema modeline + managed-by note as frontmatter comments; `emit` re-emits the
    // whole projection from the surface. A whole-file re-emit must carry those
    // install-placed metadata lines through — never drop or reflow them — so the
    // `gate_installed` re-nudge loop that papered over emit dropping them is gone.
    let harness = write_harness("survive-emit", false);
    install::run(&harness, false).unwrap();

    // Precondition: both placements really are on disk, so a survival assertion below
    // is emit preserving them — not them never having been placed. The modeline is the
    // exact per-artifact line, checked verbatim so a reflow would be caught.
    let skill_rel = &frontmatter_projections()[0];
    let skill_modeline = "# yaml-language-server: $schema=../../../.temper/schema/skill.json";
    let before = fs::read_to_string(harness.join(skill_rel)).unwrap();
    assert!(before.contains(NOTE_MARKER) && before.contains(skill_modeline));

    import_then_emit(&harness, "survive-emit-into");

    // Both install-placed lines round-trip the re-emit verbatim, on every frontmatter
    // projection — the modeline's exact bytes (no reflow) and the note's marker.
    let after = fs::read_to_string(harness.join(skill_rel)).unwrap();
    assert!(
        after.contains(skill_modeline),
        "emit must preserve the modeline verbatim, got:\n{after}"
    );
    assert!(
        after.contains(NOTE_MARKER),
        "emit must preserve the managed-by note, got:\n{after}"
    );
    let rust = fs::read_to_string(harness.join(&frontmatter_projections()[1])).unwrap();
    assert!(
        rust.contains(MODELINE_MARKER) && rust.contains(NOTE_MARKER),
        "emit must preserve the rule's placements too, got:\n{rust}"
    );

    // The re-nudge loop is gone: with the placements preserved, the gate now reads
    // every one as Unchanged, so its self-verify has nothing left to nudge.
    assert!(
        install::gate_installed(&harness).is_empty(),
        "a preserved-placement projection leaves the gate clean, got: {:?}",
        install::gate_installed(&harness)
    );
}

#[test]
fn a_fresh_install_settings_document_is_stable() {
    // A fresh install (no pre-existing settings) yields a deterministic single-hook
    // document — a reviewable golden for the merge's output shape.
    let root = write_harness("golden", false);
    install::run(&root, false).unwrap();
    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    insta::assert_snapshot!(settings);
}

#[test]
fn the_cli_install_verb_projects_and_dry_runs() {
    let root = write_harness("cli", true);

    // A dry run over the real binary reports the pending placements but writes none.
    let before = tree_bytes(&root);
    let output = Command::new(BIN)
        .arg("install")
        .arg(&root)
        .arg("--dry-run")
        .output()
        .unwrap();
    assert!(output.status.success(), "install --dry-run must exit zero");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("dry run") && stdout.contains("applied"),
        "the dry run must report the pending install, got:\n{stdout}"
    );
    assert_eq!(before, tree_bytes(&root), "the CLI dry run writes nothing");

    // The real run lands the placements and exits zero.
    let status = Command::new(BIN)
        .arg("install")
        .arg(&root)
        .status()
        .unwrap();
    assert!(status.success(), "install must exit zero");
    assert!(
        root.join(".github")
            .join("workflows")
            .join("temper.yml")
            .is_file(),
        "the CLI install must write the CI job"
    );
}
