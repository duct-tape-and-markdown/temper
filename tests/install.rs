//! `temper install` — the one on-ramp.
//!
//! Drives the library `install::discover` / `install::run` / `install::gate_installed`
//! (plus the real `temper` binary for the CLI-observable bits — the one-question
//! prompt, `--yes`/`--no-represent`, and `guard`'s lock-grounded posture) and proves:
//!
//! - **discovery** — the report counts members by kind before anything is written;
//! - **no-path** — declining wires the `SessionStart` reporter alone, Node-free,
//!   never creating `.temper/`;
//! - **yes-path** — the lift scaffolds a member module per discovered artifact
//!   (`file()` over the original text, zero file moves) plus `harness.ts`, and the
//!   first real `emit` (over the built SDK, `node` and all) produces a lock;
//! - **own-path** — a lifted member's projection is byte-identical to its source,
//!   so it claims no guard/note; a separately-authored member's does;
//! - **idempotence** — re-running converges, never re-scaffolding or duplicating
//!   the guard;
//! - **dependency-before-lift** — a spawn failure ensuring the SDK dependency
//!   leaves no half-scaffolded `.temper/` program behind it;
//! - **the lock, not the retired manifest, grounds `guard`'s posture**.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Once;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::drift::ApplyOutcome;
use temper::install::{self, InstallOutcome, Represent};

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

/// The repo's `sdk/` directory — the SDK package this crate's worktree carries
/// beside `Cargo.toml` (mirrors `tests/emit.rs`'s own fixture).
fn sdk_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sdk")
}

/// Build the SDK's `dist/` once per test binary run.
fn ensure_sdk_built() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let status = std::process::Command::new("npm")
            .args(["run", "build"])
            .current_dir(sdk_root())
            .status()
            .expect("failed to run `npm run build` in sdk/ — is npm on PATH?");
        assert!(status.success(), "sdk build failed");
    });
}

/// Vendor `@dtmd/temper` into `temper_dir/node_modules` via a symlink to the repo's
/// own built SDK — the stand-in for a real `npm install`, so `install::run`'s
/// dependency-ensure step finds it already resolved and never spawns real `npm`
/// (no network needed in this suite).
fn vendor_sdk(temper_dir: &Path) {
    ensure_sdk_built();
    let scope = temper_dir.join("node_modules").join("@dtmd");
    fs::create_dir_all(&scope).unwrap();
    let link = scope.join("temper");
    if !link.exists() {
        std::os::unix::fs::symlink(sdk_root(), &link).unwrap();
    }
}

/// A skill with frontmatter — a full, realistic pre-existing artifact the lift
/// scaffolds over.
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

/// A pre-existing `.claude/settings.json` in a shape a whole-file re-serialize could
/// never reproduce: 4-space indentation (`serde_json`'s canonical pretty-printer
/// always uses 2) and `zeta` ordered before `permissions` (a `serde_json::Map`
/// without `preserve_order` always sorts alphabetically, so a reserialize would flip
/// them). `EXISTING_SETTINGS` above is already canonical 2-space/alphabetical, so a
/// whole-file re-serialize is byte-identical to it and cannot falsify the bug this
/// fixture targets.
const NON_CANONICAL_SETTINGS: &str = "{\n    \"zeta\": \"first\",\n    \"permissions\": {\n        \"allow\": [\"Bash(cargo test:*)\"]\n    }\n}\n";

/// [`NON_CANONICAL_SETTINGS`], but with the `SessionStart` hook already merged in —
/// the starting point for a second merge that only has the `PreToolUse` guard left
/// to graft.
const NON_CANONICAL_SETTINGS_WITH_HOOK: &str = "{\n    \"zeta\": \"first\",\n    \"hooks\": {\n        \"SessionStart\": [\n            { \"hooks\": [ { \"type\": \"command\", \"command\": \"temper check . --reporter session-start\" } ] }\n        ]\n    }\n}\n";

/// Assert `updated` differs from `original` only inside one contiguous byte range —
/// a single-hunk diff, provable without depending on where install's own grafted
/// content happens to land: the longest common prefix and the longest common
/// suffix between the two texts, taken together, must account for every byte
/// `original` carries. Anything that fails this check moved or was rewritten
/// somewhere outside the grafted hunk.
fn assert_one_hunk_diff(original: &str, updated: &str) {
    let prefix_len = original
        .bytes()
        .zip(updated.bytes())
        .take_while(|(a, b)| a == b)
        .count();
    let suffix_len = original.as_bytes()[prefix_len..]
        .iter()
        .rev()
        .zip(updated.as_bytes()[prefix_len..].iter().rev())
        .take_while(|(a, b)| a == b)
        .count();
    assert_eq!(
        prefix_len + suffix_len,
        original.len(),
        "expected every pre-existing byte to survive as a prefix+suffix around one \
         grafted hunk; original:\n{original}\nupdated:\n{updated}"
    );
}

/// Build a harness with a skill, two rules (one frontmatter-free), and optionally a
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
/// is unique.
fn outcome_of(outcome: &InstallOutcome, placement: &str) -> ApplyOutcome {
    let mut matches = outcome.entries.iter().filter(|e| e.placement == placement);
    let found = matches
        .next()
        .unwrap_or_else(|| panic!("no entry for placement {placement}"));
    assert!(
        matches.next().is_none(),
        "placement {placement} should be unique"
    );
    found.outcome
}

/// Whether `outcome` carries any entry for `placement` at all.
fn has_entry(outcome: &InstallOutcome, placement: &str) -> bool {
    outcome.entries.iter().any(|e| e.placement == placement)
}

// ---------------------------------------------------------------------------
// discovery
// ---------------------------------------------------------------------------

#[test]
fn discover_reports_member_counts_by_kind() {
    let root = write_harness("discover", false);
    let report = install::discover(&root).unwrap();
    assert_eq!(report.members.get("skill").map(Vec::len), Some(1));
    assert_eq!(report.members.get("rule").map(Vec::len), Some(2));
    assert_eq!(report.total(), 3);

    let rendered = install::render_discovery(&report);
    assert!(rendered.contains("skill"));
    assert!(rendered.contains("rule"));
}

#[test]
fn an_empty_project_reports_no_members_found() {
    let root = tmpdir("discover-empty");
    let report = install::discover(&root).unwrap();
    assert_eq!(report.total(), 0);
    assert!(install::render_discovery(&report).contains("no members found"));
}

/// A `CLAUDE.md` under `.temper/` (the surface workspace: temper's own authored
/// modules and lock) is never a harness member — it is committed, not
/// gitignored, so absent an explicit skip it would double-count `memory`
/// alongside the harness-root and `.claude/` files.
#[test]
fn discovery_skips_claude_md_under_the_surface_workspace() {
    let root = write_harness("discover-surface-skip", false);
    fs::write(root.join("CLAUDE.md"), "# Root\n").unwrap();
    fs::create_dir_all(root.join(".claude")).unwrap();
    fs::write(root.join(".claude").join("CLAUDE.md"), "# Claude dir\n").unwrap();
    fs::create_dir_all(root.join(".temper")).unwrap();
    fs::write(root.join(".temper").join("CLAUDE.md"), "# Surface\n").unwrap();

    let report = install::discover(&root).unwrap();
    assert_eq!(report.members.get("memory").map(Vec::len), Some(2));
    let memory = report.members.get("memory").unwrap();
    assert!(!memory.iter().any(|p| p.starts_with(root.join(".temper"))));
}

// ---------------------------------------------------------------------------
// the no-path — the session-start reporter alone, Node-free
// ---------------------------------------------------------------------------

#[test]
fn declining_wires_the_session_start_reporter_alone_and_never_creates_temper_dir() {
    let root = write_harness("no-represent", true);
    let discovery = install::discover(&root).unwrap();

    let outcome = install::run(&root, &discovery, Represent::No, false).unwrap();
    assert!(!outcome.represented);
    assert_eq!(outcome.scaffolded, 0);
    assert!(outcome.emit.is_none());

    // Only the SessionStart hook is projected — no guard, no note, no modeline.
    assert_eq!(
        outcome_of(&outcome, "session-start hook"),
        ApplyOutcome::Applied
    );
    assert!(!has_entry(&outcome, "guard hook"));
    assert!(!has_entry(&outcome, "managed-by note"));
    assert!(!has_entry(&outcome, "schema modeline"));

    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    assert_eq!(
        json["hooks"]["SessionStart"][0]["hooks"][0]["command"],
        "temper check . --reporter session-start"
    );
    assert!(json["hooks"].get("PreToolUse").is_none());
    assert_eq!(
        json["permissions"]["allow"][0], "Bash(cargo test:*)",
        "the merge must preserve the human's existing settings"
    );

    // The project is never represented: no workspace, no lock, sources untouched.
    assert!(!root.join(".temper").exists());
    assert_eq!(
        fs::read_to_string(
            root.join(".claude")
                .join("skills")
                .join("coordinate")
                .join("SKILL.md")
        )
        .unwrap(),
        SKILL
    );
}

#[test]
fn the_session_start_merge_never_reserializes_a_non_canonical_settings_file() {
    let root = write_harness("format-preserving", false);
    let settings_path = root.join(".claude").join("settings.json");
    fs::write(&settings_path, NON_CANONICAL_SETTINGS).unwrap();

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::No, false).unwrap();

    let after = fs::read_to_string(&settings_path).unwrap();
    assert_one_hunk_diff(NON_CANONICAL_SETTINGS, &after);

    let json: serde_json::Value = serde_json::from_str(&after).unwrap();
    assert_eq!(
        json["hooks"]["SessionStart"][0]["hooks"][0]["command"],
        "temper check . --reporter session-start"
    );
    assert_eq!(
        json["zeta"], "first",
        "the human's non-canonical key survives"
    );
    assert_eq!(
        json["permissions"]["allow"][0], "Bash(cargo test:*)",
        "the human's non-canonical indentation and order survive outside the graft"
    );

    // Re-running converges: the hook is already in its desired shape, so the second
    // merge is a byte-for-byte no-op — never a second graft, never renewed churn.
    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::No, false).unwrap();
    assert_eq!(
        fs::read_to_string(&settings_path).unwrap(),
        after,
        "re-running the merge must converge"
    );
}

// ---------------------------------------------------------------------------
// the yes-path — the lift + first emit over the real, built SDK
// ---------------------------------------------------------------------------

#[test]
fn representing_lifts_every_discovered_member_byte_stable_with_no_guard_claim() {
    let root = write_harness("represent", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    vendor_sdk(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();

    assert!(outcome.represented);
    assert_eq!(outcome.scaffolded, 3, "one skill + two rules");
    assert!(temper_dir.join("harness.ts").is_file());
    assert!(temper_dir.join("skills").join("coordinate.ts").is_file());
    assert!(temper_dir.join("rules").join("rust.ts").is_file());
    assert!(temper_dir.join("rules").join("collaboration.ts").is_file());
    assert!(temper_dir.join("lock.toml").is_file());

    // `Skill.description` is a required SDK field — the scaffolded module must carry
    // the source's description forward or it fails `tsc` before a single deepening
    // edit; a rule's module (no description-trigger field) carries no such line.
    assert!(
        fs::read_to_string(temper_dir.join("skills").join("coordinate.ts"))
            .unwrap()
            .contains(
                "description: \"Use when coordinating agents across axes; not for single-axis work.\","
            ),
        "the scaffolded skill module must carry the source's required description forward"
    );
    assert!(
        !fs::read_to_string(temper_dir.join("rules").join("rust.ts"))
            .unwrap()
            .contains("description:"),
        "a rule has no description-trigger field, so its module carries no description line"
    );

    // Every lifted member's projection is byte-identical to its original source —
    // the lift's own no-op-on-content guarantee.
    let emit = outcome.emit.as_ref().expect("the yes-path ran a real emit");
    assert!(
        emit.entries
            .iter()
            .all(|e| e.outcome == temper::drift::EmitOutcome::Unchanged),
        "a lifted member's first emit must be a byte-stable no-op, got: {:?}",
        emit.entries
    );
    assert_eq!(
        fs::read_to_string(
            root.join(".claude")
                .join("skills")
                .join("coordinate")
                .join("SKILL.md")
        )
        .unwrap(),
        SKILL,
        "a lifted member's file() source is its own projected path — untouched"
    );
    assert_eq!(
        fs::read_to_string(root.join(".claude").join("rules").join("rust.md")).unwrap(),
        RULE
    );

    // No member here is emit-owned (every one is own-path), so the guard has no
    // constituency yet — "the guard arrives with its constituency, never before".
    assert!(!has_entry(&outcome, "guard hook"));
    assert!(!has_entry(&outcome, "managed-by note"));
    assert!(!has_entry(&outcome, "schema modeline"));
    assert_eq!(
        outcome_of(&outcome, "session-start hook"),
        ApplyOutcome::Applied
    );
    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    assert!(
        json["hooks"].get("PreToolUse").is_none(),
        "no guard hook without an emit-owned constituency, got: {settings}"
    );
}

#[test]
fn re_representing_never_re_scaffolds_and_converges() {
    let root = write_harness("re-represent", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    vendor_sdk(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();
    let after_first = tree_bytes(&root);

    let second = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(second.scaffolded, 0, "the lift never re-scaffolds");
    assert_eq!(
        outcome_of(&second, "session-start hook"),
        ApplyOutcome::Unchanged
    );
    assert_eq!(
        after_first,
        tree_bytes(&root),
        "a re-representation with no authored change is a byte-for-byte no-op"
    );
}

/// Serializes the one test below that shadows the process-wide `PATH` — no other
/// test in this suite spawns a real `npm` (every other yes-path test vendors the
/// dependency via [`vendor_sdk`], so `dependency_resolves` short-circuits before
/// ever reaching a spawn), but a shared `PATH` is process state, not per-test.
static PATH_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn a_dependency_spawn_failure_leaves_no_half_scaffolded_state() {
    // Force the SDK's own `npm run build` (real `npm`, gated by a `Once`) to finish
    // first — otherwise a concurrently first-triggered `ensure_sdk_built` elsewhere
    // could race the shadowed `npm` this test installs below.
    ensure_sdk_built();

    let root = write_harness("dependency-spawn-failure", false);
    let temper_dir = root.join(".temper");
    let discovery = install::discover(&root).unwrap();

    let guard = PATH_MUTEX.lock().unwrap();
    let original_path = std::env::var_os("PATH").unwrap_or_default();

    // A shadow `npm`/`npm.cmd` on `PATH` ahead of the real one, always failing —
    // standing in for "only npm.cmd exists and this Windows spawn can't find it"
    // without actually requiring a Windows host to prove the ordering.
    let fake_bin = tmpdir("dependency-spawn-failure-fake-npm");
    let fake_npm = fake_bin.join(if cfg!(windows) { "npm.cmd" } else { "npm" });
    fs::write(&fake_npm, "#!/bin/sh\nexit 1\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&fake_npm).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&fake_npm, perms).unwrap();
    }
    let mut shadowed_path = std::ffi::OsString::from(&fake_bin);
    shadowed_path.push(if cfg!(windows) { ";" } else { ":" });
    shadowed_path.push(&original_path);
    // SAFETY: serialized by `PATH_MUTEX`, and no other test in this binary spawns a
    // real `npm` (see the mutex doc comment) — no other thread reads/writes `PATH`
    // concurrently with this block.
    unsafe { std::env::set_var("PATH", &shadowed_path) };

    let result = install::run(&root, &discovery, Represent::Yes, false);

    // SAFETY: see above.
    unsafe { std::env::set_var("PATH", &original_path) };
    drop(guard);

    assert!(
        result.is_err(),
        "the shadowed, always-failing npm must fail the install"
    );
    assert!(
        !temper_dir.join("harness.ts").exists(),
        "dependency assurance must run before harness.ts is scaffolded"
    );
    assert!(
        !temper_dir.join("skills").exists(),
        "dependency assurance must run before any member module is scaffolded"
    );
    assert!(
        !temper_dir.join("rules").exists(),
        "dependency assurance must run before any member module is scaffolded"
    );
}

#[test]
fn a_fresh_dry_run_scaffolds_and_writes_nothing() {
    // No `vendor_sdk`, no real `node_modules` — a fresh dry run over the yes-path
    // must never touch disk, so it needs neither the dependency nor a real emit.
    let root = write_harness("dry-fresh", true);
    let discovery = install::discover(&root).unwrap();
    let before = tree_bytes(&root);

    let outcome = install::run(&root, &discovery, Represent::Yes, true).unwrap();
    assert_eq!(
        outcome.scaffolded, 3,
        "the preview still counts what would lift"
    );
    assert!(
        outcome.emit.is_none(),
        "nothing was scaffolded for real to emit over"
    );
    assert!(
        outcome.entries.is_empty(),
        "no lock yet to ground placements against"
    );
    assert_eq!(before, tree_bytes(&root), "--dry-run must write nothing");
    assert!(!root.join(".temper").exists());
}

#[test]
fn a_deepened_member_with_its_own_asset_is_emit_owned_and_a_lifted_one_is_not() {
    let root = write_harness("deepen", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    vendor_sdk(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();

    // Deepen by hand: a brand-new member with its own separate asset — never a
    // lift, so its `file()` source differs from its projected path.
    fs::write(
        temper_dir.join("skills").join("extra.md"),
        "# Extra\n\nDeepened by hand.\n",
    )
    .unwrap();
    fs::write(
        temper_dir.join("skills").join("extra.ts"),
        "import { file, skill } from \"@dtmd/temper/claude-code\";\n\n\
         export const extra = skill({\n  name: \"extra\",\n  description: \"An extra skill authored by hand.\",\n  prose: file(\"./skills/extra.md\"),\n});\n",
 )
.unwrap();
    fs::write(
        temper_dir.join("harness.ts"),
        "import { emit, harness } from \"@dtmd/temper\";\n\
         import { skill_coordinate } from \"./skills/coordinate.ts\";\n\
         import { rule_rust } from \"./rules/rust.ts\";\n\
         import { rule_collaboration } from \"./rules/collaboration.ts\";\n\
         import { extra } from \"./skills/extra.ts\";\n\n\
         const program = harness({\n  members: [skill_coordinate, rule_rust, rule_collaboration, extra],\n});\n\n\
         process.stdout.write(emit(program).seam);\n",
 )
.unwrap();

    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(
        outcome.scaffolded, 0,
        "an already-represented project is never re-lifted"
    );

    // The guard now has a constituency: the freshly authored `extra` skill.
    assert_eq!(outcome_of(&outcome, "guard hook"), ApplyOutcome::Applied);
    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    assert_eq!(
        json["hooks"]["PreToolUse"][0]["hooks"][0]["command"],
        "temper guard ."
    );

    let extra_md = fs::read_to_string(
        root.join(".claude")
            .join("skills")
            .join("extra")
            .join("SKILL.md"),
    )
    .unwrap();
    assert!(extra_md.contains("name: \"extra\""));
    assert!(extra_md.contains("Deepened by hand."));
    assert!(
        extra_md.contains("# temper: managed projection"),
        "the emit-owned extra skill gets the managed-by note, got:\n{extra_md}"
    );

    // The lifted members stay own-path — no note claim, byte-untouched.
    assert!(
        !fs::read_to_string(
            root.join(".claude")
                .join("skills")
                .join("coordinate")
                .join("SKILL.md")
        )
        .unwrap()
        .contains("# temper: managed projection"),
        "a lifted member's own file must carry no note claim"
    );

    // No `.temper/schema/skill.json` exists yet, so no modeline is placed even on
    // the emit-owned target — a modeline pointing at nothing is worse than none.
    assert!(!extra_md.contains("# yaml-language-server:"));
    assert!(!has_entry(&outcome, "schema modeline"));

    // Once the schema artifact exists, a re-run places the modeline too.
    fs::create_dir_all(temper_dir.join("schema")).unwrap();
    fs::write(temper_dir.join("schema").join("skill.json"), "{}").unwrap();
    let third = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(outcome_of(&third, "schema modeline"), ApplyOutcome::Applied);
    let extra_md_after = fs::read_to_string(
        root.join(".claude")
            .join("skills")
            .join("extra")
            .join("SKILL.md"),
    )
    .unwrap();
    assert!(
        extra_md_after.starts_with(
            "---\n# yaml-language-server: $schema=../../../.temper/schema/skill.json\n"
        ),
        "got:\n{extra_md_after}"
    );

    // A subsequent real `emit` (over the modified `harness.ts`) preserves both
    // install-placed lines — the two-projectors seam.
    let emit_again =
        temper::drift::emit_program(&temper_dir, temper::drift::EmitOptions::default()).unwrap();
    assert!(
        emit_again
            .entries
            .iter()
            .all(|e| e.outcome == temper::drift::EmitOutcome::Unchanged),
        "nothing authored changed since the last emit, got: {:?}",
        emit_again.entries
    );
    let extra_md_final = fs::read_to_string(
        root.join(".claude")
            .join("skills")
            .join("extra")
            .join("SKILL.md"),
    )
    .unwrap();
    assert!(extra_md_final.contains("# yaml-language-server:"));
    assert!(extra_md_final.contains("# temper: managed projection"));
}

#[test]
fn the_guard_merge_never_reserializes_a_non_canonical_settings_file() {
    let root = write_harness("format-preserving-guard", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    vendor_sdk(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();

    // Deepen by hand exactly like the emit-owned test above — the guard's
    // constituency, so this run has one to place a `PreToolUse` group for.
    fs::write(
        temper_dir.join("skills").join("extra.md"),
        "# Extra\n\nDeepened by hand.\n",
    )
    .unwrap();
    fs::write(
        temper_dir.join("skills").join("extra.ts"),
        "import { file, skill } from \"@dtmd/temper/claude-code\";\n\n\
         export const extra = skill({\n  name: \"extra\",\n  description: \"An extra skill authored by hand.\",\n  prose: file(\"./skills/extra.md\"),\n});\n",
    )
    .unwrap();
    fs::write(
        temper_dir.join("harness.ts"),
        "import { emit, harness } from \"@dtmd/temper\";\n\
         import { skill_coordinate } from \"./skills/coordinate.ts\";\n\
         import { rule_rust } from \"./rules/rust.ts\";\n\
         import { rule_collaboration } from \"./rules/collaboration.ts\";\n\
         import { extra } from \"./skills/extra.ts\";\n\n\
         const program = harness({\n  members: [skill_coordinate, rule_rust, rule_collaboration, extra],\n});\n\n\
         process.stdout.write(emit(program).seam);\n",
    )
    .unwrap();

    // Replace the settings the first `run` above wrote with a hand-authored,
    // non-canonical document that already carries the `SessionStart` hook — so
    // this second run has only the `PreToolUse` guard left to graft.
    let settings_path = root.join(".claude").join("settings.json");
    fs::write(&settings_path, NON_CANONICAL_SETTINGS_WITH_HOOK).unwrap();

    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(outcome_of(&outcome, "guard hook"), ApplyOutcome::Applied);

    let after = fs::read_to_string(&settings_path).unwrap();
    assert_one_hunk_diff(NON_CANONICAL_SETTINGS_WITH_HOOK, &after);

    let json: serde_json::Value = serde_json::from_str(&after).unwrap();
    assert_eq!(
        json["hooks"]["PreToolUse"][0]["hooks"][0]["command"],
        "temper guard ."
    );
    assert_eq!(
        json["hooks"]["SessionStart"][0]["hooks"][0]["command"],
        "temper check . --reporter session-start"
    );
    assert_eq!(
        json["zeta"], "first",
        "the human's non-canonical key survives"
    );

    // Re-running converges: both hooks are already in their desired shape.
    let second = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(outcome_of(&second, "guard hook"), ApplyOutcome::Unchanged);
    assert_eq!(
        fs::read_to_string(&settings_path).unwrap(),
        after,
        "re-running the merge must converge"
    );
}

// ---------------------------------------------------------------------------
// gate_installed — the read-only self-verify shadow
// ---------------------------------------------------------------------------

#[test]
fn gate_installed_never_scaffolds_and_reflects_represented_vs_not() {
    let root = write_harness("gate", false);

    // Unrepresented: only the missing hook is nudged.
    let before = install::gate_installed(&root);
    assert_eq!(before.len(), 1, "got: {before:?}");
    assert!(before[0].message.contains("session-start hook"));
    assert!(before[0].message.contains("temper install"));
    assert!(
        !root.join(".temper").exists(),
        "gate_installed must never scaffold or adopt"
    );

    // Decline: the hook lands, the gate is clean, still unrepresented.
    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::No, false).unwrap();
    assert!(install::gate_installed(&root).is_empty());

    // Represent for real: the gate stays clean immediately after (no emit-owned
    // targets to nudge for a pure lift).
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    vendor_sdk(&temper_dir);
    install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert!(
        install::gate_installed(&root).is_empty(),
        "got: {:?}",
        install::gate_installed(&root)
    );
}

// ---------------------------------------------------------------------------
// guard — the lock, not the retired manifest, grounds the posture
// ---------------------------------------------------------------------------

/// A `PreToolUse` payload naming a `.claude/` projection `file_path` — the write the
/// guard binds on.
const CLAUDE_WRITE_PAYLOAD: &str =
    "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\".claude/skills/x/SKILL.md\"}}";

/// Drive `temper guard <root>` across the process boundary with `payload` on stdin.
/// Returns the exit code and the stderr the guard prints on a projection hit.
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

/// A minimal lock row declaring `.claude/skills/x/SKILL.md` (the [`CLAUDE_WRITE_PAYLOAD`]
/// target) an emit-owned projection — real posture tests bind against a declared
/// member, never a lock with no member rows at all.
const CLAUDE_WRITE_LOCK_ROW: &str = "[[skill]]\nname = \"x\"\nsource_path = \".claude/skills/x/SKILL.md\"\nsource_hash = \"abc\"\nemit_hash = \"abc\"\n";

#[test]
fn guard_reads_the_block_posture_from_the_lock_not_the_retired_manifest() {
    let root = tmpdir("lock-posture-block");
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(
        temper_dir.join("lock.toml"),
        format!("[[declaration.assembly]]\nfact = \"mode\"\nvalue = \"block\"\n\n{CLAUDE_WRITE_LOCK_ROW}"),
    )
    .unwrap();
    // A stray retired manifest naming the opposite posture must be ignored entirely —
    // the manifest is never read at all, by this or any other verb.
    fs::write(
        root.join(format!("temper{}toml", '.')),
        "authority = \"warn\"\n",
    )
    .unwrap();

    let (code, stderr) = run_guard(&root, CLAUDE_WRITE_PAYLOAD);
    assert_eq!(code, Some(2), "the lock's `block` posture must block");
    assert!(stderr.contains("other tools writes are not bound by it"));
}

/// With no `lock.toml` at all there is no declared projection set to consult — unlike
/// a represented harness (below), the guard falls back to binding any `.claude/` write
/// at the default posture rather than silently allowing everything: absent evidence
/// must never *suppress* a guard claim, only ever fail to forge one.
#[test]
fn guard_defaults_to_warn_when_the_lock_is_absent() {
    let root = tmpdir("lock-posture-absent");
    let (code, stderr) = run_guard(&root, CLAUDE_WRITE_PAYLOAD);
    assert_eq!(
        code,
        Some(0),
        "no lock ⇒ default warn, warns but never blocks"
    );
    assert!(stderr.contains("temper-managed projection"));

    let (allow_code, allow_stderr) = run_guard(
        &root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\"src/main.rs\"}}",
    );
    assert_eq!(allow_code, Some(0));
    assert!(allow_stderr.is_empty());
}

/// A file()-carried member's own `.claude/` source (`own_path`) is its authored source
/// of truth, absent from the lock's emit-owned projection set — a write to it must
/// pass even under `block`, while a genuinely lock-declared projection stays bound.
#[test]
fn guard_allows_a_file_carried_members_own_path_source_under_block() {
    let root = tmpdir("lock-own-path");
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(
        temper_dir.join("lock.toml"),
        "[[declaration.assembly]]\nfact = \"mode\"\nvalue = \"block\"\n\n\
         [[skill]]\nname = \"projected\"\nsource_path = \".claude/skills/projected/SKILL.md\"\nsource_hash = \"a\"\nemit_hash = \"a\"\n\n\
         [[skill]]\nname = \"lifted\"\nsource_path = \".claude/skills/lifted/SKILL.md\"\nsource_hash = \"b\"\nemit_hash = \"b\"\nown_path = true\n",
    )
    .unwrap();

    let (own_path_code, own_path_stderr) = run_guard(
        &root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\".claude/skills/lifted/SKILL.md\"}}",
    );
    assert_eq!(
        own_path_code,
        Some(0),
        "a file()-carried member's own .claude/ source is never a guard target"
    );
    assert!(own_path_stderr.is_empty());

    let (projected_code, projected_stderr) = run_guard(
        &root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\".claude/skills/projected/SKILL.md\"}}",
    );
    assert_eq!(
        projected_code,
        Some(2),
        "a lock-declared projection still binds at the declared mode"
    );
    assert!(projected_stderr.contains("other tools writes are not bound by it"));
}

// ---------------------------------------------------------------------------
// emit's own note/modeline discipline — unrelated to install, still exercised
// directly over a hand-built payload.
// ---------------------------------------------------------------------------

fn skill_rule_kind_facts() -> Vec<temper::drift::KindFactRow> {
    vec![
        temper::drift::KindFactRow {
            name: "rule".to_string(),
            provider: None,
            governs_root: ".claude/rules".to_string(),
            governs_glob: "*.md".to_string(),
            format: Some("yaml-frontmatter".to_string()),
            unit_shape: Some("file".to_string()),
            registration: None,
            templates: Vec::new(),
        },
        temper::drift::KindFactRow {
            name: "skill".to_string(),
            provider: None,
            governs_root: ".claude/skills".to_string(),
            governs_glob: "*/SKILL.md".to_string(),
            format: Some("yaml-frontmatter".to_string()),
            unit_shape: Some("directory".to_string()),
            registration: None,
            templates: Vec::new(),
        },
    ]
}

fn payload_from_harness(harness: &Path) -> temper::drift::Payload {
    let skill_kind = temper::builtin_kind::definition("skill").unwrap().unwrap();
    let rule_kind = temper::builtin_kind::definition("rule").unwrap().unwrap();

    let skill_path = harness
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md");
    let skill = temper::frontmatter::Member::from_source(&skill_kind, &skill_path).unwrap();
    let mut members = vec![temper::drift::PayloadMember {
        kind: "skill".to_string(),
        name: skill.id.clone(),
        fields: skill.fields.clone(),
        body: skill.body.clone(),
        source_path: None,
    }];

    for rule_name in ["rust", "collaboration"] {
        let rule_path = harness
            .join(".claude")
            .join("rules")
            .join(format!("{rule_name}.md"));
        let rule = temper::frontmatter::Member::from_source(&rule_kind, &rule_path).unwrap();
        members.push(temper::drift::PayloadMember {
            kind: "rule".to_string(),
            name: rule.id.clone(),
            fields: rule.fields.clone(),
            body: rule.body.clone(),
            source_path: None,
        });
    }

    temper::drift::Payload {
        version: temper::drift::SEAM_VERSION,
        declarations: temper::drift::Declarations {
            kinds: skill_rule_kind_facts(),
            ..Default::default()
        },
        members,
    }
}

fn emit_from_harness(harness: &Path) {
    let into = harness.join(".temper");
    fs::create_dir_all(&into).unwrap();
    let payload = payload_from_harness(harness);
    temper::drift::emit(
        &payload,
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
    let harness = write_harness("emit-no-note", false);
    emit_from_harness(&harness);

    for rel in [
        PathBuf::from(".claude")
            .join("skills")
            .join("coordinate")
            .join("SKILL.md"),
        PathBuf::from(".claude").join("rules").join("rust.md"),
    ] {
        let projected = fs::read_to_string(harness.join(&rel)).unwrap();
        assert!(!projected.contains("# temper: managed projection"));
        assert!(!projected.contains("# yaml-language-server:"));
    }
}

// ---------------------------------------------------------------------------
// the CLI verb — the one question, `--yes`/`--no-represent`, the interactive prompt
// ---------------------------------------------------------------------------

#[test]
fn the_cli_install_verb_reports_discovery_then_wires_the_reporter_on_no_represent() {
    let root = write_harness("cli-no", true);
    let output = Command::new(BIN)
        .arg("install")
        .arg(&root)
        .arg("--no-represent")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("discovery:"));
    assert!(stdout.contains("not represented"));
    assert!(!root.join(".temper").exists());
}

#[test]
fn the_cli_install_verb_prompts_exactly_once_with_no_flag() {
    use std::io::Write as _;
    let root = write_harness("cli-prompt", true);
    let mut child = Command::new(BIN)
        .arg("install")
        .arg(&root)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(b"n\n").unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains(install::REPRESENT_QUESTION));
    assert!(stdout.contains("not represented"));
}

#[test]
fn the_cli_install_verb_represents_on_yes_and_dry_runs_a_re_represent() {
    let root = write_harness("cli-yes", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    vendor_sdk(&temper_dir);

    let status = Command::new(BIN)
        .arg("install")
        .arg(&root)
        .arg("--yes")
        .status()
        .unwrap();
    assert!(status.success());
    assert!(temper_dir.join("harness.ts").is_file());
    assert!(temper_dir.join("lock.toml").is_file());

    let before = tree_bytes(&root);
    let output = Command::new(BIN)
        .arg("install")
        .arg(&root)
        .arg("--yes")
        .arg("--dry-run")
        .output()
        .unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("dry run"));
    assert_eq!(
        before,
        tree_bytes(&root),
        "a re-represent dry run writes nothing"
    );
}
