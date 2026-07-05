//! `temper install` — projecting the gate's wiring under the three-state drift
//! engine (`specs/architecture/50-distribution.md`, "Decision: `install` is two
//! placements, one mechanism").
//!
//! Drives the library `install::run` / `install::gate_installed` over a real
//! harness and proves the four properties the entry names:
//!
//! - **projection** — one run writes the `SessionStart` hook into
//!   `.claude/settings.json` (merged, preserving what was there) and the schema
//!   modeline into each artifact's frontmatter; it writes no CI workflow file
//!   (`50-distribution.md` rejects an install-managed workflow — CI is a
//!   user-authored job);
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
fn install_projects_the_placements() {
    let root = write_harness("projects", true);

    let report = install::run(&root, false).unwrap();

    // 1. The SessionStart hook is merged into settings.json — additively, so the
    //    pre-existing permissions block survives beside the grafted hook.
    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    assert_eq!(
        json["hooks"]["SessionStart"][0]["hooks"][0]["command"],
        "temper check . --reporter session-start"
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

    // 2. No CI workflow file is written — `50-distribution.md` rejects an
    //    install-managed workflow; CI is a documented user-authored job.
    assert!(
        !root
            .join(".github")
            .join("workflows")
            .join("temper.yml")
            .exists(),
        "install must write no CI workflow file"
    );
    assert!(
        !root.join(".github").exists(),
        "install must not create .github/ at all"
    );
    assert!(
        report
            .entries
            .iter()
            .all(|e| !e.path.to_string_lossy().ends_with("temper.yml")),
        "no placement entry names a CI workflow file"
    );

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

    // Hand-drift one placement: a human strips the schema modeline off a rule.
    let rust_path = root.join(".claude").join("rules").join("rust.md");
    fs::write(&rust_path, RULE).unwrap();

    let after = install::gate_installed(&root);
    assert_eq!(
        after.len(),
        1,
        "a single drifted placement still yields one summary advisory, got: {after:?}"
    );
    assert!(
        after[0].message.contains("schema modeline"),
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

/// The guard command the `PreToolUse` hook carries after an install at `root` — now the
/// posture-independent `temper guard .` subcommand invocation (the note/warn/block
/// behavior lives in the subcommand, driven by [`run_guard`]).
fn guard_command(root: &Path) -> String {
    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    json["hooks"]["PreToolUse"][0]["hooks"][0]["command"]
        .as_str()
        .expect("the guard hook must carry a command")
        .to_string()
}

/// A `PreToolUse` payload naming a `.claude/` projection `file_path` — the write the guard
/// binds on.
const CLAUDE_WRITE_PAYLOAD: &str =
    "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\".claude/skills/x/SKILL.md\"}}";

/// Drive `temper guard <root>` across the process boundary with `payload` on stdin —
/// the subcommand reads the declared posture from `<root>/temper.toml`. Returns the exit
/// code and the stderr the guard prints on a projection hit.
fn run_guard(root: &Path, payload: &str) -> (Option<i32>, String) {
    use std::io::Write;
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
    let out = child.wait_with_output().unwrap();
    (
        out.status.code(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
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

    // The installed hook is the posture-independent subcommand invocation.
    assert_eq!(
        guard_command(&root),
        "temper guard .",
        "the guard hook runs the `temper guard` subcommand"
    );

    // Driven over a `.claude/` write under `surface`, the guard blocks (exit 2) and its
    // message states the limit verbatim.
    let (code, stderr) = run_guard(&root, CLAUDE_WRITE_PAYLOAD);
    assert_eq!(code, Some(2), "the surface guard must block, got: {code:?}");
    assert!(
        stderr.contains("other tools writes are not bound by it"),
        "the guard states the limit verbatim, got: {stderr}"
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

    // Driven over a `.claude/` write under `shared`, the guard warns-and-routes: it prints
    // the managed-by message but exits zero, never blocking the write.
    let (code, stderr) = run_guard(&root, CLAUDE_WRITE_PAYLOAD);
    assert_eq!(
        code,
        Some(0),
        "the shared guard must not block, got: {code:?}"
    );
    assert!(
        stderr.contains("temper-managed projection"),
        "the shared guard states the managed-by message, got: {stderr}"
    );
    // A write that touches no `.claude/` projection is allowed silently under either posture.
    let (allow_code, allow_stderr) = run_guard(
        &root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\"src/main.rs\"}}",
    );
    assert_eq!(allow_code, Some(0));
    assert!(
        allow_stderr.is_empty(),
        "a non-projection write is allowed silently, got: {allow_stderr}"
    );

    // The note is the universal layer — present under `shared` too.
    let rust_md = fs::read_to_string(root.join(".claude").join("rules").join("rust.md")).unwrap();
    assert!(rust_md.contains(NOTE_MARKER), "the note lands under shared");
}

#[test]
fn a_posture_change_takes_effect_without_re_wiring_and_never_duplicates_the_guard() {
    // The guard command is posture-independent — `temper guard` reads the declared posture
    // live — so flipping the assembly `shared`→`surface` changes the guard's *behavior*
    // with no re-install, and a re-install never appends a second guard.
    let root = write_harness("posture-change", true);
    install::run(&root, false).unwrap();
    // Default `shared`: the guard warns without blocking.
    assert_eq!(
        run_guard(&root, CLAUDE_WRITE_PAYLOAD).0,
        Some(0),
        "starts advisory"
    );

    // Flip the posture — no re-wiring — and the same installed hook now blocks.
    fs::write(root.join("temper.toml"), "authority = \"surface\"\n").unwrap();
    assert_eq!(
        run_guard(&root, CLAUDE_WRITE_PAYLOAD).0,
        Some(2),
        "the flipped posture blocks, read live by the subcommand"
    );

    // A re-install leaves a single `PreToolUse` group Unchanged — the constant command
    // is already in place, so it is never duplicated.
    let report = install::run(&root, false).unwrap();
    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    assert_eq!(
        json["hooks"]["PreToolUse"].as_array().unwrap().len(),
        1,
        "a re-install never appends a second guard"
    );
    assert_eq!(
        outcome_of(&report, "guard hook"),
        ApplyOutcome::Unchanged,
        "the posture-independent guard is already in place ⇒ Unchanged"
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

    // The guard's message rides the subcommand now — drive it over a `.claude/` write and
    // read the routed remedy off stderr.
    let (_code, guard) = run_guard(&root, CLAUDE_WRITE_PAYLOAD);
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

#[test]
fn a_reworded_managed_by_note_re_places_and_a_current_one_stays_unchanged() {
    // Content-drift awareness (`specs/architecture/50-distribution.md`, "drift keeps it
    // synced"): presence-based keying returned any marked note verbatim, so a note
    // reworded after INSTALL-DRIFT-STRINGS never refreshed and `gate_installed` passed
    // the stale bytes forever. The fix keys idempotence on the note's bytes.
    let root = write_harness("note-content-drift", true);
    install::run(&root, false).unwrap();

    let rust_path = root.join(".claude").join("rules").join("rust.md");
    let placed = fs::read_to_string(&rust_path).unwrap();
    // Capture the current note line verbatim so the stale variant is a genuine reword
    // of THIS wording, not a guess at what install writes.
    let current_note = placed
        .lines()
        .find(|l| l.trim_start().starts_with(NOTE_MARKER))
        .expect("install placed a marked note")
        .to_string();

    // Hand-drift the on-disk note to a retired wording — a marked line whose body
    // differs from the current NOTE_COMMENT. This is the post-reword state install
    // used to leave untouched.
    let stale_note = format!("{NOTE_MARKER} — retired wording, re-run the old re-add verb.");
    let drifted = placed.replacen(&current_note, &stale_note, 1);
    fs::write(&rust_path, &drifted).unwrap();

    let report = install::run(&root, false).unwrap();
    let note_entry = |file: &str| {
        report
            .entries
            .iter()
            .find(|e| e.placement == "managed-by note" && e.path.to_string_lossy().ends_with(file))
            .unwrap_or_else(|| panic!("no managed-by note entry for {file}"))
    };
    // The reworded note re-places (not Unchanged); the already-current note on the
    // skill stays Unchanged — a matching body is left byte-verbatim.
    assert_ne!(
        note_entry("rust.md").outcome,
        ApplyOutcome::Unchanged,
        "a reworded note must re-place, not report Unchanged"
    );
    assert_eq!(
        note_entry("SKILL.md").outcome,
        ApplyOutcome::Unchanged,
        "an already-current note must stay Unchanged"
    );

    // The splice restores the exact current projection — the stale body is gone and no
    // other byte (modeline, fields, body) shifted.
    let after = fs::read_to_string(&rust_path).unwrap();
    assert_eq!(
        after, placed,
        "the reworded note re-places to the current projection byte-for-byte"
    );
    assert!(
        !after.contains("retired wording"),
        "the retired note body must be gone, got:\n{after}"
    );
    // With the note re-synced, the self-verify no longer flags it as drifted.
    assert!(
        install::gate_installed(&root).is_empty(),
        "a re-placed note leaves the gate undrifted, got: {:?}",
        install::gate_installed(&root)
    );

    // A second install is a byte-for-byte no-op: every placement Unchanged.
    let before_second = tree_bytes(&root);
    let second = install::run(&root, false).unwrap();
    assert!(
        second
            .entries
            .iter()
            .all(|e| e.outcome == ApplyOutcome::Unchanged),
        "a second install is idempotent, got: {:?}",
        second.entries
    );
    assert_eq!(
        before_second,
        tree_bytes(&root),
        "the idempotent re-run must not change a single byte"
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
        root.join(".claude").join("settings.json").is_file(),
        "the CLI install must write the SessionStart hook into settings.json"
    );
    assert!(
        !root.join(".github").exists(),
        "the CLI install must write no CI workflow file"
    );
}
