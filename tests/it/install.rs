//! `temper install` — the one on-ramp.
//!
//! Drives the library `install::discover` / `install::run` / `install::gate_installed`
//! (plus the real `temper` binary for the CLI-observable bits — the one-question
//! prompt, `--yes`/`--no-represent`, and `guard`'s lock-grounded enforcement mode) and proves:
//!
//! - **discovery** — the report counts members by kind before anything is written;
//! - **no-path** — declining wires the `SessionStart` reporter alone, Node-free,
//!   never creating `.temper/`;
//! - **yes-path** — the lift scaffolds a member module per discovered artifact, a
//!   **whole conversion** (0016): every present frontmatter field hoists into a
//!   typed property and prose moves module-side (inline or a module-adjacent
//!   file, never a `file()` back-reference to the original `.claude/` path),
//!   plus `harness.ts`, and the first real `emit` (over the built SDK, `node` and
//!   all) regenerates every composed kind's artifact as a canonical projection
//!   and produces a lock;
//! - **registration members** — a host manifest's collection entries are the
//!   members discovery counts and the lift converts, one module per entry beside
//!   the container member holding the file — never the file standing in for them;
//! - **no own-path** — every scaffolded member is emit-owned from its first
//!   emit, so the guard/managed-by note claim it immediately — never an
//!   own-path passthrough;
//! - **idempotence** — converges on the first run, never re-scaffolding or
//!   duplicating the guard;
//! - **dependency-before-lift** — a spawn failure ensuring the SDK dependency
//!   leaves no half-scaffolded `.temper/` program behind it;
//! - **the lock, not the retired manifest, grounds `guard`'s enforcement mode**.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use temper::compose::{self, EnforcementMode};
use temper::drift::{self, ApplyOutcome, ClauseRow, KindFactRow};
use temper::install::{self, InstallOutcome, KindMembers, Represent};

use crate::common;
use crate::common::fresh_clause;

/// The command a represented harness's gate hook keyed under `event` projects when its
/// member reaches no tool: the temper package's bin at the workspace's anchored `.bin` path.
fn represented_gate_command(event: &str) -> String {
    temper::install::gate_hook_command(event, "./.temper/node_modules/.bin/temper").unwrap()
}

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

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

/// [`NON_CANONICAL_SETTINGS`], but with a `SessionStart` array already populated by a
/// different, non-temper tool — `session_start_present` reads `false` (the command
/// isn't temper's), so the merge must append temper's own group after this sibling
/// entry rather than take the fresh-key `insert_member` path.
const NON_CANONICAL_SETTINGS_WITH_SIBLING_HOOK: &str = "{\n    \"zeta\": \"first\",\n    \"hooks\": {\n        \"SessionStart\": [\n            { \"hooks\": [ { \"type\": \"command\", \"command\": \"other-tool check\" } ] }\n        ]\n    }\n}\n";

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
    let root = common::tmpdir(label);
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

/// The outcome `install` reported for the placement, asserting it
/// is unique.
fn outcome_of(outcome: &InstallOutcome, placement: temper::install::Placement) -> ApplyOutcome {
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

/// Whether `outcome` carries any entry for the placement.
fn has_entry(outcome: &InstallOutcome, placement: temper::install::Placement) -> bool {
    outcome.entries.iter().any(|e| e.placement == placement)
}

/// The self-verify driven the way `check` drives it: the harness's parsed lock read once
/// and threaded into the face, so the twelve arms below carry one expression rather than
/// twelve copies of the read.
fn gate_installed(root: &Path) -> Vec<temper::check::Diagnostic> {
    let lock = drift::read_lock_document(&root.join(".temper")).unwrap();
    install::gate_installed(root, &lock)
}

/// The message of the one advisory `gate_installed` folds every unplaced placement into,
/// asserting it really is one: a second diagnostic would let an arm below match a needle in
/// a finding it was not reasoning about.
fn one_gate_finding(findings: &[temper::check::Diagnostic]) -> &str {
    assert_eq!(
        findings.len(),
        1,
        "the self-verify folds into exactly one advisory, got: {findings:?}"
    );
    &findings[0].message
}

// ---------------------------------------------------------------------------
// discovery
// ---------------------------------------------------------------------------

/// A project root carrying a stand-in temper at `./bin/temper` that prints the directory
/// it ran in and its arguments, plus a `sub/` directory carrying its own stand-in that
/// prints `sub` first — so the output names which binary ran, and where.
#[cfg(unix)]
fn gate_command_project(label: &str) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;
    let root = common::tmpdir(label).canonicalize().unwrap();
    for (at, tag) in [(root.clone(), "root"), (root.join("sub"), "sub")] {
        let bin = at.join("bin").join("temper");
        fs::create_dir_all(bin.parent().unwrap()).unwrap();
        fs::write(&bin, format!("#!/bin/sh\necho \"{tag} $PWD $*\"\n")).unwrap();
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o755)).unwrap();
    }
    root
}

/// Runs `command` through `sh -c` in `cwd`, as Claude Code runs a hook, with
/// `CLAUDE_PROJECT_DIR` at `project_dir` or unset; returns stdout, asserting success.
#[cfg(unix)]
fn run_gate_command(command: &str, cwd: &Path, project_dir: Option<&Path>) -> String {
    let mut sh = Command::new("/bin/sh");
    sh.arg("-c")
        .arg(command)
        .current_dir(cwd)
        .env_remove("CLAUDE_PROJECT_DIR")
        .stdin(std::process::Stdio::null());
    if let Some(dir) = project_dir {
        sh.env("CLAUDE_PROJECT_DIR", dir);
    }
    let output = sh.output().unwrap();
    assert!(
        output.status.success(),
        "{command}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

/// Each gate hook's event and the arguments its rendered command passes temper.
#[cfg(unix)]
const GATE_EVENTS: [(&str, &str); 3] = [
    ("SessionStart", "check . --reporter session-start"),
    ("PreToolUse", "guard ."),
    ("PostToolUse", "guard ."),
];

#[cfg(unix)]
#[test]
fn a_rendered_gate_command_run_from_a_subdirectory_invokes_temper_at_the_project_root() {
    let root = gate_command_project("gate-anchored-root");
    for (event, args) in GATE_EVENTS {
        let command = install::gate_hook_command(event, "./bin/temper").unwrap();
        let stdout = run_gate_command(&command, &root.join("sub"), Some(&root));
        assert_eq!(
            stdout.trim_end(),
            format!("root {} {args}", root.display()),
            "{event}: the relative binary and `.` resolve at the project root"
        );
    }
}

#[cfg(unix)]
#[test]
fn a_rendered_gate_command_with_the_project_dir_unset_runs_where_it_is_invoked() {
    let root = gate_command_project("gate-anchored-unset");
    let sub = root.join("sub");
    for (event, args) in GATE_EVENTS {
        let command = install::gate_hook_command(event, "./bin/temper").unwrap();
        let stdout = run_gate_command(&command, &sub, None);
        assert_eq!(stdout.trim_end(), format!("sub {} {args}", sub.display()));
    }
}

#[test]
fn a_gate_command_in_the_unanchored_form_reads_as_no_rendered_gate() {
    // A settings document wiring temper's session-start command in the spelling from
    // before the project anchor: the probe and the verb, no `cd`. It is not the engine's
    // rendering, so the gate reads as not installed and install wires the anchored one.
    let anchored = temper::install::SESSION_START_COMMAND;
    let unanchored = "command -v temper >/dev/null 2>&1 || { echo \"temper: command not found\" >&2; exit 127; } && temper check . --reporter session-start";
    let root = write_harness("gate-unanchored", false);
    let settings_path = root.join(".claude").join("settings.json");
    let unanchored_settings = serde_json::to_string_pretty(&serde_json::json!({
        "hooks": { "SessionStart": [ { "hooks": [ { "type": "command", "command": unanchored } ] } ] }
    }))
    .unwrap();
    fs::write(&settings_path, format!("{unanchored_settings}\n")).unwrap();
    fs::create_dir_all(root.join(".temper")).unwrap();

    let message = one_gate_finding(&gate_installed(&root)).to_string();
    assert!(
        message.contains("session-start hook"),
        "an unanchored command is no installed gate, got: {message}"
    );

    let discovery = install::discover(&root).unwrap();
    let outcome = install::run(&root, &discovery, Represent::No, false).unwrap();
    assert_eq!(
        outcome_of(&outcome, temper::install::Placement::SessionStart),
        ApplyOutcome::Applied
    );
    let json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&settings_path).unwrap()).unwrap();
    let commands: Vec<&str> = json["hooks"]["SessionStart"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|group| group["hooks"][0]["command"].as_str())
        .collect();
    assert_eq!(commands, [unanchored, anchored]);
    assert!(gate_installed(&root).is_empty());
}

#[test]
fn the_gate_commands_render_from_a_binary_and_the_default_is_todays_command() {
    use temper::install::{
        DEFAULT_GATE_BINARY, GUARD_COMMAND, SESSION_START_COMMAND, gate_hook_command,
    };
    // At the default binary the renderer is the one home of today's bytes.
    assert_eq!(DEFAULT_GATE_BINARY, "temper");
    assert_eq!(
        gate_hook_command("SessionStart", DEFAULT_GATE_BINARY).as_deref(),
        Some(SESSION_START_COMMAND)
    );
    for event in ["PreToolUse", "PostToolUse"] {
        assert_eq!(
            gate_hook_command(event, DEFAULT_GATE_BINARY).as_deref(),
            Some(GUARD_COMMAND)
        );
    }

    // A path renders as written, in the probe, the refusal and the run alike.
    assert_eq!(
        gate_hook_command("PreToolUse", "./node_modules/.bin/temper").as_deref(),
        Some(
            "cd \"${CLAUDE_PROJECT_DIR:-.}\" || exit 1; command -v ./node_modules/.bin/temper >/dev/null 2>&1 || { echo \"./node_modules/.bin/temper: command not found\" >&2; exit 127; } && ./node_modules/.bin/temper guard ."
        )
    );

    // A binary the shell would split or expand is one quoted word everywhere it appears.
    assert_eq!(
        gate_hook_command("SessionStart", "/opt/my tools/it's/temper").as_deref(),
        Some(
            "cd \"${CLAUDE_PROJECT_DIR:-.}\" || exit 1; command -v '/opt/my tools/it'\\''s/temper' >/dev/null 2>&1 || { echo '/opt/my tools/it'\\''s/temper: command not found' >&2; exit 127; } && '/opt/my tools/it'\\''s/temper' check . --reporter session-start"
        )
    );

    // An event temper's gate does not ride renders no command at all.
    assert_eq!(gate_hook_command("Stop", DEFAULT_GATE_BINARY), None);
}

#[test]
fn discover_reports_member_counts_by_kind() {
    let root = write_harness("discover", false);
    let report = install::discover(&root).unwrap();
    assert_eq!(report.members.get("skill").map(KindMembers::len), Some(1));
    assert_eq!(report.members.get("rule").map(KindMembers::len), Some(2));
    assert_eq!(report.total(), 3);

    let rendered = install::render_discovery(&report, None, &[], &Default::default());
    assert!(rendered.contains("skill"));
    assert!(rendered.contains("rule"));
}

#[test]
fn an_empty_project_reports_no_members_found() {
    let root = common::tmpdir("discover-empty");
    let report = install::discover(&root).unwrap();
    assert_eq!(report.total(), 0);
    assert!(
        install::render_discovery(&report, None, &[], &Default::default())
            .contains("no members found")
    );
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
    assert_eq!(report.members.get("memory").map(KindMembers::len), Some(2));
    let memory = report.members.get("memory").unwrap().files();
    assert!(!memory.iter().any(|p| p.starts_with(root.join(".temper"))));
}

/// A vendored sub-harness carrying its own `.temper/lock.toml` is a nested governed
/// root — its members are its own corpus, so its `CLAUDE.md` adds no `memory` member
/// to the enclosing walk. The parent's own root `CLAUDE.md` is still discovered.
#[test]
fn discovery_fences_a_nested_governed_root() {
    let root = write_harness("discover-nested-root-fence", false);
    fs::write(root.join("CLAUDE.md"), "# Root\n").unwrap();

    let vendored = root.join("vendor").join("sub-harness");
    fs::create_dir_all(vendored.join(".temper")).unwrap();
    fs::write(vendored.join(".temper").join("lock.toml"), "").unwrap();
    fs::write(vendored.join("CLAUDE.md"), "# Vendored\n").unwrap();

    let report = install::discover(&root).unwrap();
    assert_eq!(report.members.get("memory").map(KindMembers::len), Some(1));
    let memory = report.members.get("memory").unwrap().files();
    assert!(!memory.iter().any(|p| p.starts_with(root.join("vendor"))));
}

/// A primary checkout and a linked worktree of it, made by a real `git worktree add` so
/// the pointer file and `commondir` are git's own layout, with the common git dir's
/// `info/exclude` naming `/.claude/rules/excluded.md`. Returns the worktree's root.
fn linked_worktree_excluding_a_rule(name: &str) -> PathBuf {
    let tmp = common::tmpdir(name);
    let primary = tmp.join("primary");
    fs::create_dir_all(&primary).unwrap();
    let git = |args: &[&str]| {
        let output = Command::new("git")
            .args(args)
            .current_dir(&primary)
            .env_remove("GIT_DIR")
            .env_remove("GIT_WORK_TREE")
            .env_remove("GIT_INDEX_FILE")
            .output()
            .expect("git runs");
        assert!(output.status.success(), "git {args:?}: {output:?}");
    };
    git(&["init", "--quiet", "--initial-branch=main"]);
    let worktree = tmp.join("linked");
    git(&["worktree", "add", "--quiet", worktree.to_str().unwrap()]);
    assert!(
        worktree.join(".git").is_file(),
        "a linked worktree's .git is a file"
    );
    fs::write(
        primary.join(".git").join("info").join("exclude"),
        "/.claude/rules/excluded.md\n",
    )
    .unwrap();
    worktree
}

/// git reads `info/exclude` from the common git dir a linked worktree's pointer chains
/// to, so a rule that exclude names is no member of the worktree's harness.
#[test]
fn the_discovery_walk_in_a_linked_worktree_skips_a_path_the_common_git_dirs_info_exclude_names() {
    let worktree = linked_worktree_excluding_a_rule("discover-worktree-exclude");
    let rules = worktree.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(rules.join("kept.md"), "# Kept\n").unwrap();
    fs::write(rules.join("excluded.md"), "# Excluded\n").unwrap();

    let report = install::discover(&worktree).unwrap();
    let discovered = report.members.get("rule").unwrap().files();
    assert_eq!(discovered.len(), 1, "{discovered:?}");
    assert!(discovered[0].ends_with("kept.md"), "{discovered:?}");
}

/// The guard's single-path answer for a pending write reads the same common exclude the
/// walk does: a write the exclude names is no member `check` would judge, so a
/// block-mode guard allows it, while an unexcluded sibling stays bound.
#[test]
fn the_single_path_query_agrees_with_the_discovery_walk_in_a_linked_worktree() {
    let worktree = linked_worktree_excluding_a_rule("guard-worktree-exclude");
    common::GuardLock::declaring("block")
        .member("rule", "safety", ".claude/rules/safety.md", "abc", "abc")
        .write(&worktree);

    let (code, stderr) = common::run_guard(
        &worktree,
        &common::guard_write_payload(".claude/rules/excluded.md"),
    );
    assert_eq!(
        code,
        Some(0),
        "a path the common exclude names is no member `check` judges, got: {stderr}"
    );

    let (stray_code, _) = common::run_guard(
        &worktree,
        &common::guard_write_payload(".claude/rules/stray.md"),
    );
    assert_eq!(stray_code, Some(2), "an unexcluded sibling stays bound");
}

/// The walk reads `.gitignore` without requiring a git checkout around it.
#[test]
fn a_gitignore_still_prunes_the_discovery_walk_in_a_tree_with_no_git() {
    let root = common::tmpdir("discover-gitignore-no-git");
    assert!(!root.join(".git").exists());
    let rules = root.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(root.join(".gitignore"), "*.local.md\n").unwrap();
    fs::write(rules.join("kept.md"), "# Kept\n").unwrap();
    fs::write(rules.join("mine.local.md"), "# Mine\n").unwrap();

    let report = install::discover(&root).unwrap();
    let discovered = report.members.get("rule").unwrap().files();
    assert_eq!(discovered.len(), 1, "{discovered:?}");
    assert!(discovered[0].ends_with("kept.md"), "{discovered:?}");
}

// ---------------------------------------------------------------------------
// the no-path — the session-start reporter alone, Node-free
// ---------------------------------------------------------------------------

#[test]
fn declining_wires_the_session_start_reporter_and_neither_guard_nor_note_and_never_creates_temper_dir()
 {
    let root = write_harness("no-represent", true);
    let discovery = install::discover(&root).unwrap();

    let outcome = install::run(&root, &discovery, Represent::No, false).unwrap();
    assert!(!outcome.represented);
    assert_eq!(outcome.scaffolded, 0);
    assert!(outcome.emit.is_none());

    // The SessionStart hook is projected — no guard, no note: those bind emit-owned
    // paths, and there is no lock to declare one.
    assert_eq!(
        outcome_of(&outcome, temper::install::Placement::SessionStart),
        ApplyOutcome::Applied
    );
    assert!(!has_entry(&outcome, temper::install::Placement::GuardHook));
    assert!(!has_entry(&outcome, temper::install::Placement::Note));

    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    assert_eq!(
        json["hooks"]["SessionStart"][0]["hooks"][0]["command"],
        temper::install::SESSION_START_COMMAND
    );
    assert!(json["hooks"].get("PreToolUse").is_none());
    assert_eq!(
        json["permissions"]["allow"][0], "Bash(cargo test:*)",
        "the merge must preserve the human's existing settings"
    );

    // The project is never represented: no workspace, no lock, and a frontmatterless
    // source takes no modeline, so it is untouched.
    assert!(!root.join(".temper").exists());
    assert_eq!(
        fs::read_to_string(root.join(".claude").join("rules").join("collaboration.md")).unwrap(),
        COLLAB_RULE
    );
}

/// The modeline an unrepresented install seats for `kind`: the SDK package's own schema
/// at the engine's version, on unpkg — spelled out here rather than read back off the
/// engine, so a respelled URL fails this suite.
fn cdn_modeline(kind: &str) -> String {
    format!(
        "# yaml-language-server: $schema=https://unpkg.com/@dtmd/temper@{}/schemas/{kind}.json",
        temper::VERSION
    )
}

/// Every modeline entry `outcome` reported, as (path, outcome) pairs.
fn modeline_entries(outcome: &InstallOutcome) -> Vec<(PathBuf, ApplyOutcome)> {
    outcome
        .entries
        .iter()
        .filter(|e| e.placement == temper::install::Placement::Modeline)
        .map(|e| (e.path.clone(), e.outcome))
        .collect()
}

#[test]
fn an_unrepresented_install_places_a_version_pinned_cdn_schema_modeline_in_a_governed_frontmatter_artifact()
 {
    let root = write_harness("no-represent-modeline", false);
    let discovery = install::discover(&root).unwrap();
    let outcome = install::run(&root, &discovery, Represent::No, false).unwrap();

    let skill_path = root
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md");
    let rule_path = root.join(".claude").join("rules").join("rust.md");
    let skill = fs::read_to_string(&skill_path).unwrap();
    let rule = fs::read_to_string(&rule_path).unwrap();

    // The modeline is the leading frontmatter line and the only inserted bytes.
    assert_eq!(
        skill,
        format!("---\n{}\n{}", cdn_modeline("skill"), &SKILL[4..])
    );
    assert_eq!(
        rule,
        format!("---\n{}\n{}", cdn_modeline("rule"), &RULE[4..])
    );

    // Discovery reports kinds in name order, so `rule` precedes `skill`.
    let placed = modeline_entries(&outcome);
    let expected = vec![
        (rule_path, ApplyOutcome::Applied),
        (skill_path, ApplyOutcome::Applied),
    ];
    assert_eq!(
        placed, expected,
        "one modeline per governed frontmatter artifact, none for the frontmatterless rule"
    );
}

#[test]
fn install_re_run_leaves_a_current_modeline_unchanged() {
    let root = write_harness("no-represent-modeline-rerun", false);
    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::No, false).unwrap();
    let skill_path = root
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md");
    let first = fs::read_to_string(&skill_path).unwrap();
    assert!(
        first.contains(&cdn_modeline("skill")),
        "the first run seats the CDN modeline, got:\n{first}"
    );

    let discovery = install::discover(&root).unwrap();
    let second = install::run(&root, &discovery, Represent::No, false).unwrap();
    assert_eq!(fs::read_to_string(&skill_path).unwrap(), first);
    let placed = modeline_entries(&second);
    assert_eq!(placed.len(), 2, "got: {placed:?}");
    assert!(
        placed
            .iter()
            .all(|(_, outcome)| *outcome == ApplyOutcome::Unchanged),
        "a re-run converges, got: {placed:?}"
    );
}

#[test]
fn unrepresented_install_re_run_re_points_a_stale_version_cdn_modeline_at_the_current_version() {
    let root = write_harness("no-represent-modeline-stale", false);
    let skill_path = root
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md");
    // A modeline an older engine seated: same package, retired version.
    let stale =
        "# yaml-language-server: $schema=https://unpkg.com/@dtmd/temper@0.0.1/schemas/skill.json";
    fs::write(&skill_path, format!("---\n{stale}\n{}", &SKILL[4..])).unwrap();

    let discovery = install::discover(&root).unwrap();
    let outcome = install::run(&root, &discovery, Represent::No, false).unwrap();

    // Only the modeline line's bytes are rewritten.
    assert_eq!(
        fs::read_to_string(&skill_path).unwrap(),
        format!("---\n{}\n{}", cdn_modeline("skill"), &SKILL[4..])
    );
    let placed = modeline_entries(&outcome);
    assert!(
        placed.contains(&(skill_path.clone(), ApplyOutcome::Applied)),
        "the stale modeline is re-placed, got: {placed:?}"
    );

    // Converged: a further run is Unchanged and byte-identical.
    let converged = fs::read_to_string(&skill_path).unwrap();
    let discovery = install::discover(&root).unwrap();
    let again = install::run(&root, &discovery, Represent::No, false).unwrap();
    assert_eq!(fs::read_to_string(&skill_path).unwrap(), converged);
    assert!(
        modeline_entries(&again)
            .iter()
            .all(|(_, outcome)| *outcome == ApplyOutcome::Unchanged),
        "got: {:?}",
        modeline_entries(&again)
    );
}

#[test]
fn an_unrepresented_install_writes_no_schema_file_to_the_tree() {
    let root = write_harness("no-represent-no-schema", false);
    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::No, false).unwrap();

    // The only JSON install writes is the settings document it merges into.
    let json_files: Vec<PathBuf> = walkdir::WalkDir::new(&root)
        .into_iter()
        .map(|entry| entry.unwrap().into_path())
        .filter(|path| path.is_file() && path.extension().is_some_and(|ext| ext == "json"))
        .collect();
    assert_eq!(
        json_files,
        vec![root.join(".claude").join("settings.json")],
        "an unrepresented install writes no schema file"
    );
    assert!(!root.join(".temper").exists());
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
        temper::install::SESSION_START_COMMAND
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

#[test]
fn the_session_start_merge_appends_after_a_sibling_tools_existing_hook() {
    let root = write_harness("format-preserving-append", false);
    let settings_path = root.join(".claude").join("settings.json");
    fs::write(&settings_path, NON_CANONICAL_SETTINGS_WITH_SIBLING_HOOK).unwrap();

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::No, false).unwrap();

    let after = fs::read_to_string(&settings_path).unwrap();
    assert_one_hunk_diff(NON_CANONICAL_SETTINGS_WITH_SIBLING_HOOK, &after);

    let json: serde_json::Value = serde_json::from_str(&after).unwrap();
    assert_eq!(
        json["hooks"]["SessionStart"][0]["hooks"][0]["command"], "other-tool check",
        "the sibling tool's entry survives untouched, in its original position"
    );
    assert_eq!(
        json["hooks"]["SessionStart"][1]["hooks"][0]["command"],
        temper::install::SESSION_START_COMMAND,
        "temper's own group is appended after the sibling entry, never before or in place of it"
    );
    assert_eq!(
        json["zeta"], "first",
        "the human's non-canonical key survives"
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

/// The scaffold declares its typecheck and it passes: a real `tsc` over the program
/// install just wrote, under the project and devDependencies it wrote beside it.
#[cfg(unix)]
#[test]
fn a_fresh_scaffold_declares_its_typecheck_and_passes_it() {
    let root = write_harness("represent-typecheck", true);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));
    common::link_typescript_bin(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false)
        .expect("the scaffolded program typechecks under its own project");

    let program = fs::read_to_string(temper_dir.join("harness.ts")).unwrap();
    assert!(
        program.contains(
            "starter({ typecheck: { project: input(import.meta.url, \"./tsconfig.json\") } })"
        ),
        "the program declares its typecheck through the starter: {program}"
    );
    assert!(
        temper_dir.join("tsconfig.json").is_file(),
        "the project file is scaffolded"
    );
    let manifest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(temper_dir.join("package.json")).unwrap())
            .unwrap();
    for package in ["typescript", "@types/node"] {
        assert!(
            manifest["devDependencies"][package].is_string(),
            "the manifest declares `{package}` for the typecheck: {manifest}"
        );
    }
    let lock = fs::read_to_string(temper_dir.join("lock.toml")).unwrap();
    assert!(
        lock.contains("source_path = \".temper/tsconfig.json\""),
        "the project rides the lock as an input"
    );
}

#[test]
fn representing_hoists_every_field_and_regenerates_every_member_as_a_guard_claimed_projection() {
    let root = write_harness("represent", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();

    assert!(outcome.represented);
    assert_eq!(outcome.scaffolded, 3, "one skill + two rules");
    assert!(temper_dir.join("harness.ts").is_file());
    assert!(temper_dir.join("skills").join("coordinate.ts").is_file());
    assert!(temper_dir.join("rules").join("rust.ts").is_file());
    assert!(temper_dir.join("rules").join("collaboration.ts").is_file());
    assert!(temper_dir.join("lock.toml").is_file());

    // Every present field hoists into a typed property (0016, whole
    // conversion) — `description` (a skill's required field) and `paths` (a
    // hoisted non-description field on a rule); a rule carries no description
    // line, since its source declares none.
    assert!(
        fs::read_to_string(temper_dir.join("skills").join("coordinate.ts"))
            .unwrap()
            .contains(
                "description: \"Use when coordinating agents across axes; not for single-axis work.\","
            ),
        "the scaffolded skill module must hoist the source's required description forward"
    );
    let rust_module = fs::read_to_string(temper_dir.join("rules").join("rust.ts")).unwrap();
    assert!(
        rust_module.contains("paths: [\"src/**/*.rs\"],"),
        "a hoisted non-description field, got:\n{rust_module}"
    );
    assert!(
        !rust_module.contains("description:"),
        "a rule has no description-trigger field, so its module carries no description line"
    );

    // Every fixture body here is at or under the SDK's three-line inline
    // threshold, so prose moves module-side as an inline `text` literal —
    // never a `file()` back-reference to the original `.claude/` path, and no
    // module-adjacent document either (a separate test covers that split).
    for rel in [
        "skills/coordinate.ts",
        "rules/rust.ts",
        "rules/collaboration.ts",
    ] {
        let module = fs::read_to_string(temper_dir.join(rel)).unwrap();
        assert!(module.contains("prose: text`"), "got:\n{module}");
        assert!(!module.contains(".claude/"), "got:\n{module}");
    }
    assert!(!temper_dir.join("skills").join("coordinate.md").exists());
    assert!(!temper_dir.join("rules").join("rust.md").exists());

    // The first emit regenerates every composed kind's artifact as a canonical
    // projection — never an own-path passthrough. `collaboration` carries no
    // frontmatter fields at all, so its canonical projection is its
    // byte-faithful body headed by the managed-projection banner emit places
    // on every frontmatterless markdown projection — the banner is what makes
    // the hand-authored source change; `rust`/`coordinate` declare fields, so
    // their frontmatter is rewritten into canonical form.
    let emit = outcome.emit.as_ref().expect("the yes-path ran a real emit");
    let outcome_for = |name: &str| {
        emit.entries
            .iter()
            .find(|e| e.name == name)
            .unwrap_or_else(|| panic!("no emit entry for {name}"))
            .outcome
    };
    assert_eq!(
        outcome_for("collaboration"),
        temper::drift::EmitOutcome::Emitted
    );
    assert_eq!(outcome_for("rust"), temper::drift::EmitOutcome::Emitted);
    assert_eq!(
        outcome_for("coordinate"),
        temper::drift::EmitOutcome::Emitted
    );

    // Every scaffolded member is emit-owned from its first emit — the guard
    // and the frontmatter-bearing members' managed-by notes claim it
    // immediately, never waiting on a hand-deepened member.
    assert_eq!(
        outcome_of(&outcome, temper::install::Placement::GuardHook),
        ApplyOutcome::Applied
    );
    assert_eq!(
        outcome_of(&outcome, temper::install::Placement::SessionStart),
        ApplyOutcome::Applied
    );
    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    assert_eq!(
        json["hooks"]["PreToolUse"][0]["hooks"][0]["command"],
        represented_gate_command("PreToolUse")
    );
    assert!(
        !has_entry(&outcome, temper::install::Placement::Modeline),
        "no schema artifact exists yet"
    );

    let coordinate_md = fs::read_to_string(
        root.join(".claude")
            .join("skills")
            .join("coordinate")
            .join("SKILL.md"),
    )
    .unwrap();
    assert!(
        coordinate_md.contains(temper::placement::NOTE_MARKER),
        "a scaffolded member's own frontmatter-bearing projection is note-claimed, got:\n{coordinate_md}"
    );
    let rust_md = fs::read_to_string(root.join(".claude").join("rules").join("rust.md")).unwrap();
    assert!(rust_md.contains(temper::placement::NOTE_MARKER));
    // `collaboration` has no frontmatter to carry the `#` note, so it takes the
    // block-level HTML-comment banner heading its body instead — a frontmatterless
    // markdown projection is note-claimed, never left bannerless.
    let collaboration_md =
        fs::read_to_string(root.join(".claude").join("rules").join("collaboration.md")).unwrap();
    assert!(
        collaboration_md.starts_with(temper::placement::BANNER_MARKER),
        "got:\n{collaboration_md}"
    );
    assert!(!collaboration_md.contains(temper::placement::NOTE_MARKER));
}

/// A `.claude/commands/deploy.md` — no required frontmatter fields.
const COMMAND_DEPLOY: &str = "---\n\
description: Deploy the service.\n\
---\n\
# Deploy\n\
\n\
Ship the command surface's build.\n";

/// A `.claude/agents/deploy.md` sharing the command above's member id — its
/// identity comes from the `name` field, never the filename, but it still
/// lands on `deploy`.
const AGENT_DEPLOY: &str = "---\n\
name: deploy\n\
description: Use to deploy the agent surface's build.\n\
---\n\
# Deploy\n\
\n\
Ship the agent surface's build.\n";

#[test]
fn a_command_and_an_agent_sharing_a_member_id_scaffold_to_distinct_paths() {
    let root = common::tmpdir("member-dir-widen");
    let commands = root.join(".claude").join("commands");
    fs::create_dir_all(&commands).unwrap();
    fs::write(commands.join("deploy.md"), COMMAND_DEPLOY).unwrap();

    let agents = root.join(".claude").join("agents");
    fs::create_dir_all(&agents).unwrap();
    fs::write(agents.join("deploy.md"), AGENT_DEPLOY).unwrap();

    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();

    assert!(outcome.represented);
    assert_eq!(outcome.scaffolded, 2, "one command + one agent");

    let command_module = temper_dir.join("command").join("deploy.ts");
    let agent_module = temper_dir.join("agent").join("deploy.ts");
    assert!(
        command_module.is_file(),
        "the command must scaffold into its own kind-named directory"
    );
    assert!(
        agent_module.is_file(),
        "the agent must scaffold into its own kind-named directory, never overwriting the command's file"
    );
    assert_ne!(
        fs::read_to_string(&command_module).unwrap(),
        fs::read_to_string(&agent_module).unwrap(),
        "distinct directories must carry each kind's own content rather than one clobbering the other"
    );
}

#[test]
fn a_nested_skill_lifts_under_its_prefixed_name_and_re_emits_where_it_was_found() {
    let root = common::tmpdir("nested-skill-lift");
    let nested = root.join("apps/web/.claude/skills/deploy/SKILL.md");
    fs::create_dir_all(nested.parent().unwrap()).unwrap();
    fs::write(&nested, SKILL).unwrap();

    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(outcome.scaffolded, 1);

    let module = fs::read_to_string(temper_dir.join("skills").join("apps-web-deploy.ts")).unwrap();
    assert!(module.contains("name: \"apps:web:deploy\""), "{module}");
    // The first emit places it back under its own subdirectory, never at the root's
    // `.claude/skills/deploy/`.
    assert!(!root.join(".claude/skills/deploy/SKILL.md").exists());
    assert!(
        fs::read_to_string(&nested)
            .unwrap()
            .ends_with("# Coordinate\n\nDrive the team through the playbook.\n")
    );
}

/// A nested skill lifted and emitted once: the root it lives in, and its re-emitted
/// `SKILL.md`.
fn nested_skill_emitted(label: &str) -> (PathBuf, PathBuf) {
    let root = common::tmpdir(label);
    let nested = root.join("apps/web/.claude/skills/deploy/SKILL.md");
    fs::create_dir_all(nested.parent().unwrap()).unwrap();
    fs::write(&nested, SKILL).unwrap();

    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();
    (root, nested)
}

#[test]
fn a_nested_skill_emits_its_frontmatter_name_as_the_leaf_of_its_identity() {
    let (root, nested) = nested_skill_emitted("nested-skill-leaf-name");
    let module = fs::read_to_string(root.join(".temper/skills/apps-web-deploy.ts")).unwrap();
    assert!(module.contains("name: \"apps:web:deploy\""), "{module}");
    let emitted = fs::read_to_string(&nested).unwrap();
    assert!(
        emitted.contains("\nname: \"deploy\"\n"),
        "the frontmatter name is the leaf, never the placement-prefixed identity: {emitted}"
    );
}

#[test]
fn an_emitted_nested_skill_draws_no_name_finding_from_the_skill_contract() {
    let (root, _) = nested_skill_emitted("nested-skill-leaf-check");
    let run = common::check_in(&root, &[], None);
    assert!(
        !run.output.contains("name-matches-dir") && !run.output.contains("allowed_chars"),
        "{}",
        run.output
    );
    assert!(run.ok, "{}", run.output);
}

#[test]
fn a_nested_command_lifts_under_its_path_name_and_re_emits_where_it_was_found() {
    let root = common::tmpdir("nested-command-lift");
    for dir in ["frontend", "backend"] {
        let commands = root.join(".claude").join("commands").join(dir);
        fs::create_dir_all(&commands).unwrap();
        fs::write(commands.join("deploy.md"), COMMAND_DEPLOY).unwrap();
    }

    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(
        outcome.scaffolded, 2,
        "two same-stemmed commands, two members"
    );

    // `:` is reserved in a Windows file name, so the module is named with `-`; the member
    // keeps the `:`-joined path name `check` reads it under.
    for (stem, name) in [
        ("frontend-deploy", "frontend:deploy"),
        ("backend-deploy", "backend:deploy"),
    ] {
        let module = fs::read_to_string(temper_dir.join("command").join(format!("{stem}.ts")))
            .unwrap_or_else(|err| panic!("the `{name}` command scaffolds as `{stem}.ts`: {err}"));
        assert!(
            module.contains(&format!("name: \"{name}\"")),
            "the module names the member by its path, got:\n{module}"
        );
    }
    // The first emit places each back at its own nested path, never a flat `deploy.md`.
    assert!(!root.join(".claude/commands/deploy.md").exists());
    for dir in ["frontend", "backend"] {
        assert!(
            root.join(".claude/commands")
                .join(dir)
                .join("deploy.md")
                .is_file()
        );
    }
}

#[test]
fn lift_declares_a_location_for_a_named_field_member_whose_field_differs_from_its_file_stem() {
    let root = common::tmpdir("named-field-location-lift");
    let agents = root.join(".claude").join("agents");
    fs::create_dir_all(&agents).unwrap();
    let source = "---\n\
name: teaching-mode\n\
description: Use to teach the codebase.\n\
---\n\
# Teach\n\
\n\
Walk the reader through the module.\n";
    fs::write(agents.join("teach.md"), source).unwrap();

    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(outcome.scaffolded, 1);

    let module = fs::read_to_string(temper_dir.join("agent").join("teaching-mode.ts")).unwrap();
    assert!(
        module.contains("\"teach.md\""),
        "the lift declares its read path: {module}"
    );
    // The first emit rewrites no path: the agent stays where it was found.
    assert!(!agents.join("teaching-mode.md").exists());
    assert!(
        fs::read_to_string(agents.join("teach.md"))
            .unwrap()
            .ends_with("# Teach\n\nWalk the reader through the module.\n")
    );
    let run = common::check_in(&root, &[], None);
    assert!(run.ok, "{}", run.output);
}

#[test]
fn install_lifts_two_nested_memory_files_as_two_members() {
    let root = common::tmpdir("nested-memory-lift");
    fs::write(root.join("CLAUDE.md"), "# Root\n\nRoot memory.\n").unwrap();
    for dir in ["apps/web", "apps/api"] {
        fs::create_dir_all(root.join(dir)).unwrap();
        fs::write(
            root.join(dir).join("CLAUDE.md"),
            format!("# {dir}\n\nMemory for {dir}.\n"),
        )
        .unwrap();
    }

    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(
        outcome.scaffolded, 3,
        "three CLAUDE.md files, three members"
    );

    let modules: Vec<String> = walkdir::WalkDir::new(&temper_dir)
        .into_iter()
        .filter_entry(|entry| entry.file_name() != "node_modules")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "ts"))
        .map(|entry| fs::read_to_string(entry.path()).unwrap())
        .collect();
    for name in ["CLAUDE", "apps:web:CLAUDE", "apps:api:CLAUDE"] {
        assert!(
            modules
                .iter()
                .any(|module| module.contains(&format!("name: \"{name}\""))),
            "a module names the `{name}` memory member"
        );
    }

    // The first emit regenerates each at its own path, never collapsing onto the root.
    let root_md = fs::read_to_string(root.join("CLAUDE.md")).unwrap();
    assert!(root_md.contains("Root memory."), "{root_md}");
    for dir in ["apps/web", "apps/api"] {
        let nested = fs::read_to_string(root.join(dir).join("CLAUDE.md")).unwrap();
        assert!(nested.contains(&format!("Memory for {dir}.")), "{nested}");
        assert!(
            nested.starts_with(temper::placement::BANNER_MARKER),
            "`{dir}/CLAUDE.md` is regenerated as a projection, got:\n{nested}"
        );
    }
}

/// A skill whose body is a document — well past the SDK's three-line inline
/// threshold (`sdk/src/prose.ts`) — the lift's other prose placement: a
/// module-adjacent file, never a `file()` back-reference to the original
/// `.claude/` path (0016).
const DOCUMENT_SKILL: &str = "---\n\
name: deepdive\n\
description: Use for a documented multi-paragraph walkthrough.\n\
---\n\
# Deepdive\n\
\n\
This member's body is long enough that it must live in a module-adjacent\n\
document rather than inline in the module source.\n\
\n\
A third paragraph keeps it safely past the three-line threshold.\n";

#[test]
fn a_document_body_scaffolds_to_a_module_adjacent_file_never_the_original_path() {
    let root = common::tmpdir("document-prose");
    let skill = root.join(".claude").join("skills").join("deepdive");
    fs::create_dir_all(&skill).unwrap();
    fs::write(skill.join("SKILL.md"), DOCUMENT_SKILL).unwrap();

    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();

    let module = fs::read_to_string(temper_dir.join("skills").join("deepdive.ts")).unwrap();
    assert!(
        module.contains("prose: file(import.meta.url, \"./deepdive.md\"),"),
        "got:\n{module}"
    );
    assert!(!module.contains("text`"));
    assert!(!module.contains(".claude/"));

    let expected_body = "# Deepdive\n\nThis member's body is long enough that it must live in a module-adjacent\ndocument rather than inline in the module source.\n\nA third paragraph keeps it safely past the three-line threshold.\n";
    assert_eq!(
        fs::read_to_string(temper_dir.join("skills").join("deepdive.md")).unwrap(),
        expected_body,
        "the document body is copied module-adjacent, byte-faithfully"
    );

    // The first emit still resolves the module-adjacent document back into
    // the canonical projection's body, byte-faithfully.
    let projected = fs::read_to_string(skill.join("SKILL.md")).unwrap();
    assert!(projected.ends_with(expected_body), "got:\n{projected}");
}

// ---------------------------------------------------------------------------
// the whole-document formats — a `json-document` kind's artifact is its fields
// ---------------------------------------------------------------------------

/// A `.claude/settings.json` whose top-level keys are the `settings` member's own
/// fields — the whole file is one JSON document, not frontmatter over a body.
const SETTINGS_DOCUMENT: &str = "{\n  \"model\": \"opus\",\n  \"permissions\": {\n    \"allow\": [\"Bash(cargo test:*)\"]\n  }\n}\n";

/// A `.claude-plugin/plugin.json`, a `json-document` kind whose identity is the
/// top-level `name` rather than the file stem — the stem is `plugin` for every
/// manifest ever written.
const PLUGIN_MANIFEST: &str = "{\n  \"name\": \"demo-pack\",\n  \"description\": \"A demo plugin.\",\n  \"version\": \"1.2.3\"\n}\n";

/// A `.claude/settings.local.json` — the machine's own overlay, at the `local`
/// commitment class.
const SETTINGS_LOCAL: &str = "{\n  \"model\": \"haiku\"\n}\n";

/// A harness carrying only whole-document artifacts — the committed settings, a plugin
/// manifest, and a local overlay — with the built SDK vendored so the first emit runs
/// for real. Returns the root and its `.temper` directory.
fn write_document_harness(label: &str) -> (PathBuf, PathBuf) {
    common::ensure_sdk_built();
    let root = common::tmpdir(label);
    let claude = root.join(".claude");
    fs::create_dir_all(&claude).unwrap();
    fs::write(claude.join("settings.json"), SETTINGS_DOCUMENT).unwrap();
    fs::write(claude.join("settings.local.json"), SETTINGS_LOCAL).unwrap();
    common::write_plugin_json(&root, PLUGIN_MANIFEST);

    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);
    (root, temper_dir)
}

/// Assert `.claude/settings.json` under `root` wires all three of temper's gate hook
/// groups — the whole gate, read the way Claude Code reads it: a `hooks.<Event>` matcher
/// group whose handler carries the exact command `install` declares. The grain is asserted
/// with it: exactly one group per event runs temper's command, carrying exactly one
/// handler, which is what a gate `hook` member now authors (`gate_hook_fields`). `context`
/// names the moment being asserted, since the same claim is made before and after a
/// re-emit.
fn assert_gate_hooks_wired(root: &Path, context: &str) {
    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    for (event, matcher, command) in [
        (
            "SessionStart",
            None,
            represented_gate_command("SessionStart"),
        ),
        (
            "PreToolUse",
            Some("Write|Edit|MultiEdit"),
            represented_gate_command("PreToolUse"),
        ),
        // The post edge runs the *same* guard command the pre edge does: the payload's own
        // `hook_event_name` says which edge fired, so one command serves both rows.
        (
            "PostToolUse",
            Some("Bash"),
            represented_gate_command("PostToolUse"),
        ),
    ] {
        let groups = json["hooks"][event]
            .as_array()
            .unwrap_or_else(|| panic!("{context}: no `hooks.{event}` array, got:\n{settings}"));
        let running: Vec<&serde_json::Value> = groups
            .iter()
            .filter(|group| {
                group["hooks"]
                    .as_array()
                    .is_some_and(|handlers| handlers.iter().any(|h| h["command"] == command))
            })
            .collect();
        assert_eq!(
            running.len(),
            1,
            "{context}: exactly one `{event}` group runs temper's command, got:\n{settings}"
        );
        let group = running[0];
        assert_eq!(
            group["hooks"].as_array().map(Vec::len),
            Some(1),
            "{context}: the `{event}` gate group fires its one handler, got:\n{settings}"
        );
        match matcher {
            Some(matcher) => assert_eq!(
                group["matcher"], matcher,
                "{context}: the `{event}` group binds its documented matcher"
            ),
            None => assert!(
                group.get("matcher").is_none(),
                "{context}: `{event}` carries no tool, so its group declares no matcher"
            ),
        }
    }
}

#[test]
fn a_settings_document_lifts_into_a_fields_bearing_member_whose_emit_re_renders_its_keys() {
    let (root, temper_dir) = write_document_harness("lift-settings-document");
    let discovery = install::discover(&root).unwrap();
    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(
        outcome.scaffolded, 2,
        "the settings document and the plugin manifest — never the local overlay"
    );

    // Read under the format the kind declares, the document's top-level keys land as
    // typed properties; read as frontmatter instead, the module carried no fields at
    // all and the whole JSON became a prose body emit has no home for.
    let module = fs::read_to_string(temper_dir.join("settings").join("settings.ts")).unwrap();
    assert!(
        module.contains("import { settings } from \"@dtmd/temper/claude-code\";"),
        "got:\n{module}"
    );
    assert!(module.contains("model: \"opus\","), "got:\n{module}");
    assert!(
        module.contains("permissions: {\"allow\":[\"Bash(cargo test:*)\"]},"),
        "got:\n{module}"
    );
    assert!(
        !module.contains("prose:"),
        "a whole-document member is its fields — there is no body slot to fill, got:\n{module}"
    );
    assert!(
        !temper_dir.join("settings").join("settings.md").exists(),
        "no prose sidecar holding the raw JSON"
    );

    // The first emit re-renders the document from the program — every authored key
    // survives the round trip.
    let emit = outcome.emit.as_ref().expect("the yes-path ran a real emit");
    assert!(
        emit.entries
            .iter()
            .any(|e| e.kind == "settings" && e.name == "settings"),
        "the settings member is an emit target, got:\n{:#?}",
        emit.entries
    );
    let projected: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join(".claude").join("settings.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(projected["model"], "opus");
    assert_eq!(projected["permissions"]["allow"][0], "Bash(cargo test:*)");

    // The gate rides the same projection. Its three hook groups are `hook` **members**
    // the starter composes, so they land in the file emit renders whole — where a splice
    // beside emit reported `applied` and was erased by the very next re-render, leaving
    // this file with no `hooks` key at all.
    assert_gate_hooks_wired(&root, "after install");
    assert!(
        fs::read_to_string(temper_dir.join("harness.ts"))
            .unwrap()
            .contains("members: [...base.members, "),
        "the entry point composes the starter's members, the gate hooks among them"
    );

    // And a second emit keeps them — the assertion the splice could never pass: the
    // re-stamp emit that follows `install`'s placements re-renders this same file from
    // the program, so a writer other than emit is erased here by construction.
    temper::drift::emit_program(&temper_dir, temper::drift::EmitOptions::default()).unwrap();
    assert_gate_hooks_wired(&root, "after a second emit");
    let re_projected: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join(".claude").join("settings.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(
        re_projected, projected,
        "the settings projection is byte-stable across a second emit"
    );
}

#[test]
fn a_plugin_manifest_lifts_under_its_name_key_with_the_sdks_camel_case_constructor() {
    let (root, temper_dir) = write_document_harness("lift-plugin-manifest");
    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();

    // Identity is the document's declared `name` key; reading the JSON as frontmatter
    // found none and aborted the whole lift `frontmatter::no_named_field_id`.
    let module =
        fs::read_to_string(temper_dir.join("plugin-manifest").join("demo-pack.ts")).unwrap();
    assert!(
        module.contains("import { pluginManifest } from \"@dtmd/temper/claude-code\";"),
        "a hyphenated kind reaches the module as the SDK's camelCase export, never the \
         row label — `import {{ plugin-manifest }}` is not TS; got:\n{module}"
    );
    assert!(
        module.contains("export const plugin_manifest_demo_pack = pluginManifest({"),
        "the kind prefix folds into the identifier exactly as the name does, got:\n{module}"
    );
    assert!(module.contains("name: \"demo-pack\","), "got:\n{module}");
    assert!(module.contains("version: \"1.2.3\","), "got:\n{module}");
    assert!(
        fs::read_to_string(temper_dir.join("harness.ts"))
            .unwrap()
            .contains("plugin_manifest_demo_pack"),
        "the entry point composes the lifted manifest"
    );

    let projected: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join(".claude-plugin").join("plugin.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(projected["name"], "demo-pack");
    assert_eq!(projected["description"], "A demo plugin.");
    assert_eq!(projected["version"], "1.2.3");
}

#[test]
fn a_local_commitment_artifact_is_counted_in_discovery_and_converted_into_nothing() {
    let (root, temper_dir) = write_document_harness("lift-local-skip");
    let discovery = install::discover(&root).unwrap();
    assert_eq!(
        discovery
            .members
            .get("settings-local")
            .map(KindMembers::len),
        Some(1),
        "the report counts what the walk found"
    );

    install::run(&root, &discovery, Represent::Yes, false).unwrap();
    // A local artifact is read in place at check and is never an emit input or target,
    // so the lift — which converts an artifact into a committed module whose artifact
    // is a projection — writes nothing for it.
    assert!(
        !temper_dir.join("settings-local").exists(),
        "a local-commitment artifact becomes no member module"
    );
    assert!(
        !fs::read_to_string(temper_dir.join("harness.ts"))
            .unwrap()
            .contains("settings_local"),
        "and the entry point composes nothing for it"
    );
    assert_eq!(
        fs::read_to_string(root.join(".claude").join("settings.local.json")).unwrap(),
        SETTINGS_LOCAL,
        "its authored bytes are untouched"
    );
}

#[test]
fn re_representing_never_re_scaffolds_and_settles_on_the_first_run() {
    let root = write_harness("re-represent", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();
    let after_first = common::tree_bytes(&root);

    // `evaluate_placements` writes each emit-owned target's managed-by note
    // *after* the first `emit` already stamped the lock's fingerprints from
    // the pre-placement bytes; a re-stamping `emit` inside `install::run`
    // must fold those placements back in before the run returns, so the lock
    // already matches this run's own output with no second run required.
    assert!(
        temper::drift::config_stale(&temper_dir, &fresh_clause()).is_empty(),
        "the first install run must leave the lock's fingerprints matching the placement-inclusive bytes"
    );

    // Re-representing with no authored change converges to a byte-for-byte no-op.
    let second = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(second.scaffolded, 0, "the lift never re-scaffolds");
    assert_eq!(
        outcome_of(&second, temper::install::Placement::SessionStart),
        ApplyOutcome::Unchanged
    );
    assert_eq!(
        outcome_of(&second, temper::install::Placement::GuardHook),
        ApplyOutcome::Unchanged
    );
    assert_eq!(
        after_first,
        common::tree_bytes(&root),
        "a re-representation with no authored change is a byte-for-byte no-op once settled on the first run"
    );
}

#[test]
fn a_frontmatterless_memory_projection_carries_the_html_banner_and_a_re_run_converges() {
    // A memory `CLAUDE.md` has no frontmatter to hold the `#` note, so install grows
    // the block-level HTML-comment banner heading its body — while the frontmatter
    // skill beside it keeps the `#` form. Both stay content-keyed and idempotent.
    let root = common::tmpdir("memory-banner");
    fs::write(
        root.join("CLAUDE.md"),
        "# Project\n\nMemory for the agents.\n",
    )
    .unwrap();
    let skill = root.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(&skill).unwrap();
    fs::write(skill.join("SKILL.md"), SKILL).unwrap();

    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();

    let claude_md = fs::read_to_string(root.join("CLAUDE.md")).unwrap();
    assert!(
        claude_md.starts_with(temper::placement::BANNER_MARKER),
        "the frontmatterless memory projection heads its body with the HTML-comment banner, got:\n{claude_md}"
    );
    // The banner form only — never the `#` frontmatter note (there is no frontmatter).
    assert!(!claude_md.contains(temper::placement::NOTE_MARKER));
    assert!(claude_md.contains("Memory for the agents."));

    let skill_md = fs::read_to_string(skill.join("SKILL.md")).unwrap();
    assert!(
        skill_md.contains(temper::placement::NOTE_MARKER),
        "a frontmatter kind keeps the `#` note, got:\n{skill_md}"
    );
    assert!(!skill_md.contains(temper::placement::BANNER_MARKER));

    // Content-keyed idempotence across a full re-run: exactly one banner, no duplicate,
    // and the lock already matches its own placement-inclusive output (no drift).
    install::run(&root, &discovery, Represent::Yes, false).unwrap();
    let claude_md_again = fs::read_to_string(root.join("CLAUDE.md")).unwrap();
    assert_eq!(
        claude_md_again, claude_md,
        "a re-run is a byte-for-byte no-op"
    );
    assert_eq!(
        claude_md_again
            .matches(temper::placement::BANNER_MARKER)
            .count(),
        1
    );
    assert!(temper::drift::config_stale(&temper_dir, &fresh_clause()).is_empty());
}

#[test]
fn lifting_a_banner_carrying_markdown_member_scaffolds_with_no_embedded_banner() {
    // A regression for the double-banner bug: when lifting an already-banner-carrying
    // frontmatterless member (e.g., a CLAUDE.md that has been through emit once already),
    // the scaffolded module body must exclude the banner so the next emit places exactly
    // one banner, not one baked into the module-adjacent prose plus one newly placed.
    let root = common::tmpdir("lift-banner-carrying-memory");
    // Create a CLAUDE.md that already carries the banner (simulating a prior emit).
    fs::write(
        root.join("CLAUDE.md"),
        format!(
            "{}\n\n# Project\n\nMemory for the agents.\n",
            temper::placement::BANNER
        ),
    )
    .unwrap();
    // Also create a skill so the emit has members to place.
    let skill = root.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(&skill).unwrap();
    fs::write(skill.join("SKILL.md"), SKILL).unwrap();

    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    // Install (lift) the existing CLAUDE.md that already carries the banner.
    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();

    // The scaffolded memory module body must not contain the banner.
    let memory_module = temper_dir.join("memory").join("CLAUDE.ts");
    assert!(
        memory_module.is_file(),
        "the memory/CLAUDE.ts module should be scaffolded"
    );
    let module_contents = fs::read_to_string(&memory_module).unwrap();
    // The module prose must not contain the banner — it's authored prose only.
    assert!(
        !module_contents.contains(temper::placement::BANNER_MARKER),
        "the scaffold module prose must not embed the banner; got:\n{module_contents}"
    );
    assert!(
        module_contents.contains("# Project"),
        "the scaffold module prose must preserve the authored body"
    );

    // The next emit (already ran as part of install::run) must place exactly one banner,
    // not duplicate it. Verify the root CLAUDE.md still has exactly one banner.
    let claude_md = fs::read_to_string(root.join("CLAUDE.md")).unwrap();
    assert_eq!(
        claude_md.matches(temper::placement::BANNER_MARKER).count(),
        1,
        "emit must place exactly one banner, not duplicate; got:\n{claude_md}"
    );
}

/// Write an executable `name` script into `bin` whose body is `script`.
#[cfg(unix)]
fn fake_manager(bin: &Path, name: &str, script: &str) {
    use std::os::unix::fs::PermissionsExt;
    fs::create_dir_all(bin).unwrap();
    let path = bin.join(name);
    fs::write(&path, format!("#!/bin/sh\n{script}")).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
}

/// Run `temper install --yes` at `root` with `bin` ahead of the inherited `PATH` — the
/// child's own environment, so no other test's spawn sees the shadow.
#[cfg(unix)]
fn install_with_shadowed_path(root: &Path, bin: &Path) -> std::process::Output {
    let mut path = std::ffi::OsString::from(bin);
    path.push(":");
    path.push(std::env::var_os("PATH").unwrap_or_default());
    Command::new(env!("CARGO_BIN_EXE_temper"))
        .args(["install", "--yes"])
        .arg(root)
        .env("PATH", path)
        .output()
        .unwrap()
}

#[cfg(unix)]
#[test]
fn a_dependency_spawn_failure_leaves_no_half_scaffolded_state() {
    let root = write_harness("dependency-spawn-failure", false);
    let temper_dir = root.join(".temper");

    // An npm that reports its version but always fails to install.
    let fake_bin = common::tmpdir("dependency-spawn-failure-fake-npm");
    fake_manager(
        &fake_bin,
        "npm",
        "[ \"$1\" = --version ] && { echo 10.9.0; exit 0; }\nexit 1\n",
    );
    let output = install_with_shadowed_path(&root, &fake_bin);

    assert!(
        !output.status.success(),
        "the shadowed, always-failing npm must fail the install"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("npm install"),
        "the refusal names the spawn: {}",
        String::from_utf8_lossy(&output.stderr)
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

#[cfg(unix)]
#[test]
fn a_root_package_manager_field_naming_pnpm_is_copied_and_install_runs_pnpm() {
    let root = write_harness("root-pnpm-manager", false);
    fs::write(
        root.join("package.json"),
        "{ \"private\": true, \"packageManager\": \"pnpm@9.15.0\" }\n",
    )
    .unwrap();
    let temper_dir = root.join(".temper");
    // The SDK resolves, the compiler does not: install must spawn the manager.
    common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));

    // A pnpm whose install records itself and lays out the compiler; no npm at all.
    let fake_bin = common::tmpdir("root-pnpm-manager-fake-bin");
    fake_manager(
        &fake_bin,
        "pnpm",
        "echo \"$@\" > pnpm-ran\nmkdir -p node_modules/.bin\nprintf '#!/bin/sh\\nexit 0\\n' > node_modules/.bin/tsc\nchmod +x node_modules/.bin/tsc\n",
    );
    fake_manager(&fake_bin, "npm", "exit 1\n");
    let output = install_with_shadowed_path(&root, &fake_bin);
    assert!(
        output.status.success(),
        "install succeeds under pnpm: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let manifest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(temper_dir.join("package.json")).unwrap())
            .unwrap();
    assert_eq!(
        manifest["packageManager"], "pnpm@9.15.0",
        "the scaffold copies the root's declaration"
    );
    assert_eq!(
        fs::read_to_string(temper_dir.join("pnpm-ran"))
            .unwrap()
            .trim(),
        "install",
        "install spawned the declared manager in the workspace"
    );
}

#[test]
fn a_fresh_dry_run_scaffolds_and_writes_nothing() {
    // No `vendor_sdk`, no real `node_modules` — a fresh dry run over the yes-path
    // must never touch disk, so it needs neither the dependency nor a real emit.
    let root = write_harness("dry-fresh", true);
    let discovery = install::discover(&root).unwrap();
    let before = common::tree_bytes(&root);

    let outcome = install::run(&root, &discovery, Represent::Yes, true).unwrap();
    assert_eq!(
        outcome.scaffolded, 4,
        "the preview still counts what would lift — the skill, the two rules, and the \
         `.claude/settings.json` the settings container governs"
    );
    assert!(
        outcome.emit.is_none(),
        "nothing was scaffolded for real to emit over"
    );
    assert!(
        outcome.entries.is_empty(),
        "no lock yet to ground placements against"
    );
    assert_eq!(
        before,
        common::tree_bytes(&root),
        "--dry-run must write nothing"
    );
    assert!(!root.join(".temper").exists());
}

#[test]
fn a_hand_deepened_member_is_emit_owned_exactly_like_a_scaffolded_one() {
    let root = write_harness("deepen", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    let first = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    // Every scaffolded member is emit-owned from its first emit (0016, whole
    // conversion) — the guard already has a constituency before any hand
    // deepening happens; the lifted/deepened distinction the retired own-path
    // lift drew has collapsed.
    assert_eq!(
        outcome_of(&first, temper::install::Placement::GuardHook),
        ApplyOutcome::Applied
    );

    // Deepen by hand: a brand-new member with its own separate asset.
    fs::write(
        temper_dir.join("skills").join("extra.md"),
        "# Extra\n\nDeepened by hand.\n",
    )
    .unwrap();
    fs::write(
        temper_dir.join("skills").join("extra.ts"),
        "import { file, skill } from \"@dtmd/temper/claude-code\";\n\n\
         export const extra = skill({\n  name: \"extra\",\n  description: \"An extra skill authored by hand.\",\n  prose: file(import.meta.url, \"./extra.md\"),\n});\n",
 )
.unwrap();
    fs::write(
        temper_dir.join("harness.ts"),
        "import { emit, harness, input, tool } from \"@dtmd/temper\";\n\
         import { skill_coordinate } from \"./skills/coordinate.ts\";\n\
         import { rule_rust } from \"./rules/rust.ts\";\n\
         import { rule_collaboration } from \"./rules/collaboration.ts\";\n\
         import { extra } from \"./skills/extra.ts\";\n\n\
         const tsc = tool({ package: \"typescript\", bin: \"tsc\" });\n\n\
         const program = harness({\n  members: [skill_coordinate, rule_rust, rule_collaboration, extra, tsc],\n  typecheck: { tool: tsc, project: input(import.meta.url, \"./tsconfig.json\") },\n});\n\n\
         process.stdout.write(emit(program).seam);\n",
 )
.unwrap();

    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(
        outcome.scaffolded, 0,
        "an already-represented project is never re-lifted"
    );

    // The guard was already applied on the first run — a new emit-owned
    // constituent changes nothing about its own placement.
    assert_eq!(
        outcome_of(&outcome, temper::install::Placement::GuardHook),
        ApplyOutcome::Unchanged
    );
    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    assert_eq!(
        json["hooks"]["PreToolUse"][0]["hooks"][0]["command"],
        represented_gate_command("PreToolUse")
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
        extra_md.contains(temper::placement::NOTE_MARKER),
        "the emit-owned extra skill gets the managed-by note, got:\n{extra_md}"
    );

    // The scaffolded (lifted) `coordinate` member is note-claimed exactly like
    // the hand-deepened `extra` one — no own-path exemption survives.
    assert!(
        fs::read_to_string(
            root.join(".claude")
                .join("skills")
                .join("coordinate")
                .join("SKILL.md")
        )
        .unwrap()
        .contains(temper::placement::NOTE_MARKER),
        "a scaffolded member's projection is note-claimed exactly like a hand-deepened one"
    );

    // No `.temper/schema/skill.json` exists yet, so no modeline is placed even on
    // the emit-owned target — a modeline pointing at nothing is worse than none.
    assert!(!extra_md.contains(temper::placement::MODELINE_MARKER));
    assert!(!has_entry(&outcome, temper::install::Placement::Modeline));

    // Once the schema artifact exists, a re-run places the modeline on every
    // frontmatter-bearing `skill` target — the hand-deepened one and the
    // scaffolded one alike.
    fs::create_dir_all(temper_dir.join("schema")).unwrap();
    fs::write(temper_dir.join("schema").join("skill.json"), "{}").unwrap();
    let third = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    let modeline_targets: Vec<&PathBuf> = third
        .entries
        .iter()
        .filter(|e| e.placement == temper::install::Placement::Modeline)
        .map(|e| &e.path)
        .collect();
    assert_eq!(modeline_targets.len(), 2, "got: {modeline_targets:?}");
    let extra_md_after = fs::read_to_string(
        root.join(".claude")
            .join("skills")
            .join("extra")
            .join("SKILL.md"),
    )
    .unwrap();
    assert!(
        extra_md_after.starts_with(&format!(
            "---\n{} $schema=../../../.temper/schema/skill.json\n",
            temper::placement::MODELINE_MARKER
        )),
        "got:\n{extra_md_after}"
    );
    let coordinate_md_after = fs::read_to_string(
        root.join(".claude")
            .join("skills")
            .join("coordinate")
            .join("SKILL.md"),
    )
    .unwrap();
    assert!(
        coordinate_md_after.starts_with(&format!(
            "---\n{} $schema=../../../.temper/schema/skill.json\n",
            temper::placement::MODELINE_MARKER
        )),
        "got:\n{coordinate_md_after}"
    );

    // A subsequent real `emit` (over the modified `harness.ts`) preserves both
    // install-placed lines — the two-projectors seam.
    let emit_again =
        temper::drift::emit_program(&temper_dir, temper::drift::EmitOptions::default()).unwrap();
    assert!(
        !emit_again.entries.is_empty(),
        "the second emit must judge a populated entry set — an empty one satisfies the \
         all-Unchanged claim below without proving the emit ran"
    );
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
    assert!(extra_md_final.contains(temper::placement::MODELINE_MARKER));
    assert!(extra_md_final.contains(temper::placement::NOTE_MARKER));
}

#[test]
fn represented_install_re_run_re_points_a_modeline_whose_schema_ref_differs_from_the_locks() {
    let root = write_harness("represented-modeline-stale", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);
    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();
    fs::create_dir_all(temper_dir.join("schema")).unwrap();
    fs::write(temper_dir.join("schema").join("skill.json"), "{}").unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();

    let skill_path = root
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md");
    let current = fs::read_to_string(&skill_path).unwrap();
    let desired = format!(
        "{} $schema=../../../.temper/schema/skill.json",
        temper::placement::MODELINE_MARKER
    );
    assert!(current.contains(&desired), "got:\n{current}");

    // Point the modeline somewhere the lock's schema ref does not.
    let stale = format!(
        "{} $schema=https://example.invalid/skill.json",
        temper::placement::MODELINE_MARKER
    );
    fs::write(&skill_path, current.replacen(&desired, &stale, 1)).unwrap();

    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(fs::read_to_string(&skill_path).unwrap(), current);
    let placed: Vec<ApplyOutcome> = outcome
        .entries
        .iter()
        .filter(|e| e.placement == temper::install::Placement::Modeline && e.path == skill_path)
        .map(|e| e.outcome)
        .collect();
    assert_eq!(placed, vec![ApplyOutcome::Applied]);

    // An already-current modeline is Unchanged and byte-identical.
    let again = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(fs::read_to_string(&skill_path).unwrap(), current);
    assert!(
        again
            .entries
            .iter()
            .filter(|e| e.placement == temper::install::Placement::Modeline)
            .all(|e| e.outcome == ApplyOutcome::Unchanged),
        "got: {:?}",
        again.entries
    );
}

#[test]
fn a_represented_harness_re_projects_its_gate_hooks_instead_of_splicing_them_back() {
    // The represented path's `.claude/settings.json` is a projection the program owns
    // whole, so the gate rides it as three `hook` members rather than a splice beside
    // emit. The splice reported `applied` and the re-stamp emit that follows it in the
    // same `install` run re-rendered the file from the program — which declared no hooks
    // — so the run wired nothing. Here `emit` is the file's one writer, which is what
    // makes a hand-stripped gate come back.
    let root = write_harness("gate-hooks-projected", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    for placement in [
        temper::install::Placement::SessionStart,
        temper::install::Placement::GuardHook,
        temper::install::Placement::PostToolUseHook,
    ] {
        assert_eq!(
            outcome_of(&outcome, placement),
            ApplyOutcome::Applied,
            "the lift wires {placement} through the program"
        );
    }
    assert_gate_hooks_wired(&root, "after install");

    // The starter supplies all three as members of the program, composed into the entry
    // point.
    let harness = fs::read_to_string(temper_dir.join("harness.ts")).unwrap();
    assert!(harness.contains("...base.members"), "got:\n{harness}");
    // Strip the guard group by hand, the drift the `PreToolUse` guard itself exists to
    // catch. A plain `emit` — no `install` behind it — puts it back, because the member
    // is where the hook lives.
    let settings_path = root.join(".claude").join("settings.json");
    let projected = fs::read_to_string(&settings_path).unwrap();
    let mut stripped: serde_json::Value = serde_json::from_str(&projected).unwrap();
    stripped["hooks"]
        .as_object_mut()
        .unwrap()
        .remove("PreToolUse")
        .expect("the guard group was there to strip");
    fs::write(
        &settings_path,
        serde_json::to_string_pretty(&stripped).unwrap(),
    )
    .unwrap();

    temper::drift::emit_program(&temper_dir, temper::drift::EmitOptions::default()).unwrap();
    assert_gate_hooks_wired(&root, "after a re-emit over a hand-stripped gate");
    assert_eq!(
        fs::read_to_string(&settings_path).unwrap(),
        projected,
        "the re-emit reproduces the install-time projection byte for byte"
    );
}

#[test]
fn the_post_tool_use_row_runs_the_guard_the_pre_tool_use_row_runs() {
    // One command at both edges of a tool call: the payload's own `hook_event_name` is
    // what splits them, so the wiring carries no second command to drift and no second
    // reporter to stamp the wrong event's name in its output.
    let root = write_harness("post-edge-runs-the-guard", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_gate_hooks_wired(&root, "after install");

    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    let command = json["hooks"]["PostToolUse"][0]["hooks"][0]["command"]
        .as_str()
        .unwrap_or_else(|| panic!("the `PostToolUse` row carries a command, got:\n{settings}"));
    assert_eq!(
        command,
        represented_gate_command("PostToolUse"),
        "the post edge runs the guard, got:\n{settings}"
    );

    // The self-verify shadow `check` folds in reads the whole gate — the post row
    // included — as installed and undrifted.
    let findings = gate_installed(&root);
    assert!(
        findings.is_empty(),
        "the wired gate must read as installed, got: {findings:?}"
    );
}

// ---------------------------------------------------------------------------
// registration members — a manifest's collection entries, not the file holding them
// ---------------------------------------------------------------------------

/// A `.claude/settings.json` carrying all three of its registration segments beside the
/// container member's own keys: a `hooks.SessionStart` matcher group running a command
/// of the author's own (never temper's gate command), an `enabledPlugins` enablement,
/// and the `extraKnownMarketplaces` entry that enablement's `<plugin>@<marketplace>` key
/// names.
const REGISTERING_SETTINGS: &str = r#"{
  "model": "opus",
  "hooks": {
    "SessionStart": [
      { "hooks": [ { "type": "command", "command": "echo authored" } ] }
    ]
  },
  "enabledPlugins": {
    "formatter@acme": true
  },
  "extraKnownMarketplaces": {
    "acme": { "source": "acme/market" }
  }
}
"#;

/// The registration-entry count a kind reports, or `None` when the kind is missing from
/// the report entirely — the two failures a bare `unwrap()` would conflate.
fn reported(discovery: &install::DiscoveryReport, kind: &str) -> Option<usize> {
    discovery.members.get(kind).map(KindMembers::len)
}

#[test]
fn a_manifests_registration_entries_each_lift_to_their_own_module_and_survive_the_first_emit() {
    common::ensure_sdk_built();
    let root = common::tmpdir("lift-registration-members");
    common::write_settings(&root, REGISTERING_SETTINGS);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    // Discovery counts the *entries* at each kind's collection address. The count used to
    // be the host manifest's file count, so every one of these read `1` no matter what the
    // file carried — and the one `settings` container was reported four times over.
    let discovery = install::discover(&root).unwrap();
    assert_eq!(reported(&discovery, "hook"), Some(1));
    assert_eq!(reported(&discovery, "installed-plugin"), Some(1));
    assert_eq!(reported(&discovery, "known-marketplace"), Some(1));
    assert_eq!(
        reported(&discovery, "settings"),
        Some(1),
        "the file itself is still the container member's"
    );

    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(
        outcome.scaffolded, 4,
        "the settings container plus one module per registration entry"
    );

    // One module per registration member, each named by its registration key and carrying
    // that key as its identity — a hook's as its `event`, never a `name` — the entry's own
    // fields hoisted beside it.
    let hook_module =
        fs::read_to_string(temper_dir.join("hooks").join("SessionStart-2.ts")).unwrap();
    assert!(
        hook_module.contains("event: \"SessionStart\",") && !hook_module.contains("name:"),
        "got:\n{hook_module}"
    );
    // The group's handler array survives the lift as read, at every handler count: the
    // single-handler hoist that used to flatten these keys onto the module retired with
    // the authoring surface that demanded it (0075).
    assert!(
        hook_module.contains("hooks: [{\"command\":\"echo authored\",\"type\":\"command\"}],"),
        "got:\n{hook_module}"
    );

    let plugin_module = fs::read_to_string(
        temper_dir
            .join("installed-plugin")
            .join("formatter@acme.ts"),
    )
    .unwrap();
    assert!(
        plugin_module.contains("import { installedPlugin } from \"@dtmd/temper/claude-code\";"),
        "got:\n{plugin_module}"
    );
    assert!(
        plugin_module.contains("name: \"formatter@acme\","),
        "got:\n{plugin_module}"
    );
    assert!(
        plugin_module.contains("enabled: true,"),
        "the scalar entry's value rides its declared field, got:\n{plugin_module}"
    );

    // And the container member stops at its *own* keys: a segment a registration kind
    // claims is that kind's members' to carry, so the container lifting it too would have
    // emit write the segment twice — landing a manifest with the key repeated and the
    // gate's own groups shadowed by the duplicate.
    let container = fs::read_to_string(temper_dir.join("settings").join("settings.ts")).unwrap();
    assert!(container.contains("model: \"opus\","), "got:\n{container}");
    for claimed in ["hooks:", "enabledPlugins:", "extraKnownMarketplaces:"] {
        assert!(
            !container.contains(claimed),
            "`{claimed}` is a registration kind's collection, not the container's field, got:\n{container}"
        );
    }

    let marketplace_module =
        fs::read_to_string(temper_dir.join("known-marketplace").join("acme.ts")).unwrap();
    assert!(
        marketplace_module.contains("name: \"acme\","),
        "got:\n{marketplace_module}"
    );
    assert!(
        marketplace_module.contains("source: \"acme/market\","),
        "got:\n{marketplace_module}"
    );

    // The first emit re-renders the manifest from the program with every segment intact —
    // the container's own keys, and each registration segment beside them. Dropped by the
    // lift, a segment reached no member and the container's first emit shed it.
    let projected: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join(".claude").join("settings.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(projected["model"], "opus");
    assert_eq!(projected["enabledPlugins"]["formatter@acme"], true);
    assert_eq!(
        projected["extraKnownMarketplaces"]["acme"]["source"],
        "acme/market"
    );

    // The author's own group survives the round trip, and temper's gate is wired beside
    // it — two members on one event, two groups in the projected array.
    let commands: Vec<&str> = projected["hooks"]["SessionStart"]
        .as_array()
        .expect("the SessionStart collection is an array of matcher groups")
        .iter()
        .filter_map(|group| group["hooks"][0]["command"].as_str())
        .collect();
    assert!(
        commands.contains(&"echo authored"),
        "the author's own group survives, got: {commands:?}"
    );
    assert!(
        commands.contains(&represented_gate_command("SessionStart").as_str()),
        "temper's gate is wired beside it, got: {commands:?}"
    );
    assert_gate_hooks_wired(&root, "after install");
}

/// The same manifest, with the marketplace key carrying `|` — a character Windows
/// reserves in a file name. Every other segment is unchanged, so the lift's refusal is
/// the only difference between this run and the arm above.
const UNPORTABLY_REGISTERING_SETTINGS: &str = r#"{
  "model": "opus",
  "enabledPlugins": {
    "formatter@ac|me": true
  },
  "extraKnownMarketplaces": {
    "ac|me": { "source": "acme/market" }
  }
}
"#;

#[test]
fn a_registration_key_windows_reserves_refuses_the_lift_before_it_writes_a_module() {
    common::ensure_sdk_built();
    let root = common::tmpdir("lift-unportable-registration-key");
    common::write_settings(&root, UNPORTABLY_REGISTERING_SETTINGS);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    // Discovery reads the keys off the manifest like any others — the judgment is the
    // lift's, at the one point a key becomes a file name.
    let discovery = install::discover(&root).unwrap();
    assert_eq!(reported(&discovery, "known-marketplace"), Some(1));

    // `InstallError::MemberFileName`, by the code an adopter reads rather than the
    // variant's Rust name.
    let err = install::run(&root, &discovery, Represent::Yes, false).unwrap_err();
    assert_eq!(
        err.code().map(|code| code.to_string()).as_deref(),
        Some("temper::install::member_file_name"),
        "the lift refuses a key that cannot name a module file, got: {err}"
    );
    // The remedy states the rule as well as the rename, so an author hits the refusal
    // once rather than per reserved character.
    let help = err
        .help()
        .expect("the refusal carries its remedy")
        .to_string();
    for stated in [
        "rename the entry",
        "control character",
        "trailing space or dot",
    ] {
        assert!(
            help.contains(stated),
            "the help states `{stated}`, got: {help}"
        );
    }

    // The invariant the refusal serves: `run_represented` pre-flights before the lift
    // writes, so a refused key leaves no half-scaffolded program — not the offending
    // module, not the portable ones beside it, not the entry point importing them.
    for module in [
        temper_dir.join("harness.ts"),
        temper_dir.join("known-marketplace").join("ac|me.ts"),
        temper_dir
            .join("installed-plugin")
            .join("formatter@ac|me.ts"),
        temper_dir.join("settings").join("settings.ts"),
    ] {
        assert!(
            !module.exists(),
            "the refusal lands before the lift writes, got: {}",
            module.display()
        );
    }
}

#[test]
fn an_authored_group_on_a_gate_event_never_takes_the_gate_hooks_remedy_path() {
    common::ensure_sdk_built();
    let root = common::tmpdir("lift-registration-no-clobber");
    common::write_settings(&root, REGISTERING_SETTINGS);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();

    // The starter composes the gate hooks, so the scaffold writes no module for them — but
    // a missing one's remedy cites the fixed `hooks/<Event>.ts`, so the authored group
    // takes the next free name rather than the path the remedy tells an author to write.
    assert!(
        !temper_dir.join("hooks").join("SessionStart.ts").exists(),
        "the gate's remedy path stays free"
    );
    let authored_module =
        fs::read_to_string(temper_dir.join("hooks").join("SessionStart-2.ts")).unwrap();
    assert!(
        authored_module.contains("echo authored"),
        "the authored group has a module of its own, got:\n{authored_module}"
    );

    let harness = fs::read_to_string(temper_dir.join("harness.ts")).unwrap();
    assert!(
        harness.contains("import { hook_SessionStart_2 } from \"./hooks/SessionStart-2.ts\";")
            && harness.contains(", hook_SessionStart_2"),
        "the authored group is imported from its own module and composed, got:\n{harness}"
    );
    // The authored group rides beside the gate, never in its place.
    assert_gate_hooks_wired(&root, "beside an authored group on the gate's event");
}

#[test]
fn a_manifest_carrying_no_registration_segment_reports_zero_for_those_kinds() {
    // The vacuity guard on the count above: the manifest is found, read, and reported —
    // and every registration kind whose segment it does not carry reports zero. A file
    // count would read `1` for each of the three, over a file holding none of them.
    let root = common::tmpdir("discover-registration-empty");
    common::write_settings(&root, SETTINGS_DOCUMENT);

    let discovery = install::discover(&root).unwrap();
    assert_eq!(reported(&discovery, "settings"), Some(1));
    for kind in ["hook", "installed-plugin", "known-marketplace"] {
        assert_eq!(
            reported(&discovery, kind),
            Some(0),
            "`{kind}` has no entry in a settings.json carrying no such segment"
        );
    }

    let rendered = install::render_discovery(&discovery, None, &[], &Default::default());
    assert!(rendered.contains("settings"), "got:\n{rendered}");
    assert!(
        !rendered.contains("hook"),
        "an empty kind prints no row at all, got:\n{rendered}"
    );
}

// ---------------------------------------------------------------------------
// gate_installed — the read-only self-verify shadow
// ---------------------------------------------------------------------------

#[test]
fn gate_installed_never_scaffolds_and_reflects_represented_vs_not() {
    let root = write_harness("gate", false);

    // Foreign harness with no .temper/ directory: gate_installed is silent.
    let before = gate_installed(&root);
    assert_eq!(
        before.len(),
        0,
        "gate_installed must be silent on foreign harness with no .temper/, got: {before:?}"
    );
    assert!(
        !root.join(".temper").exists(),
        "gate_installed must never scaffold or adopt"
    );

    // Decline: the hook lands, the gate is clean, still unrepresented.
    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::No, false).unwrap();
    assert!(install::represented_by(&root).is_none());
    assert!(gate_installed(&root).is_empty());

    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    // A workspace dir and no program is still unrepresented, and there the session-start
    // hook is install's own write — so the remedy the advisory names is the verb, never a
    // member module in a program the harness does not have.
    let settings_path = root.join(".claude").join("settings.json");
    fs::write(&settings_path, "{}\n").unwrap();
    let message = one_gate_finding(&gate_installed(&root)).to_string();
    assert!(
        message.contains("session-start hook missing: run `temper install`"),
        "an unrepresented harness's gate is the verb's to place, got: {message}"
    );
    assert!(
        !message.contains("harness.ts"),
        "and there is no program for it to import a member into, got: {message}"
    );
    fs::remove_file(&settings_path).unwrap();

    // Represent for real: the gate stays clean immediately after (no emit-owned
    // targets to nudge for a pure lift).
    install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert!(install::represented_by(&root).is_some());
    assert!(
        gate_installed(&root).is_empty(),
        "got: {:?}",
        gate_installed(&root)
    );
}

#[test]
fn gate_installed_names_stale_noted_files() {
    let root = write_harness("gate-drifted", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    // Represent and install: the managed-by notes land on the skill and rules.
    let discovery = install::discover(&root).unwrap();
    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();

    // Check what notes were placed.
    let has_notes = outcome
        .entries
        .iter()
        .any(|e| e.placement == temper::install::Placement::Note);
    assert!(has_notes, "install must place managed-by notes");

    // Verify gate is clean after initial install.
    assert!(
        gate_installed(&root).is_empty(),
        "gate_installed must be clean after successful install"
    );

    // Simulate the one drift this gate still owns: a *stale note wording*, the marked
    // line carrying a retired phrasing. Deleting the note outright is not this gate's
    // case — emit writes the note with the rest of the projection's bytes, so an absent
    // note is a hand-edit the drift hash catches, exactly as it is for the banner.
    let skill_file = root
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md");
    let content = fs::read_to_string(&skill_file).unwrap();
    const RETIRED_NOTE: &str = "# temper: managed projection — do not edit.";
    assert!(
        content.contains(temper::placement::NOTE_COMMENT),
        "the emitted skill must carry the current note, got: {content}"
    );
    let modified = content.replacen(temper::placement::NOTE_COMMENT, RETIRED_NOTE, 1);
    fs::write(&skill_file, modified).unwrap();

    // gate_installed should now report the stale placement.
    let findings = gate_installed(&root);
    assert_eq!(
        findings.len(),
        1,
        "gate_installed must report one drifted placement, got: {findings:?}"
    );
    let msg = &findings[0].message;
    assert!(
        msg.contains("managed-by note"),
        "message must mention managed-by note, got: {msg}"
    );
    assert!(
        msg.contains("SKILL.md"),
        "message must name the specific stale-noted file, got: {msg}"
    );
}

#[test]
fn an_unreadable_emit_owned_projection_makes_the_installed_gate_check_report_rather_than_pass_silently()
 {
    let root = write_harness("gate-unreadable", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);
    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert!(gate_installed(&root).is_empty());

    // A directory where the lock names a projection file: the read fails on every host,
    // privileged or not, where a permission bit would not.
    let skill_file = root
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md");
    fs::remove_file(&skill_file).unwrap();
    fs::create_dir(&skill_file).unwrap();

    let message = one_gate_finding(&gate_installed(&root)).to_string();
    assert!(
        message.contains("could not be evaluated") && message.contains("SKILL.md"),
        "the failed read is the finding, naming the file, got: {message}"
    );
}

/// A retired spelling of temper's session-start command: the bytes before the
/// PATH-resolvability guard joined them. Exactly the shape 0073's Context describes — a
/// scaffolded module froze a literal, the command was respelled, and the member kept
/// running yesterday's bytes.
const RETIRED_SESSION_START_COMMAND: &str = "temper check . --reporter session-start";

#[test]
fn gate_installed_names_a_gate_hook_member_running_a_retired_command_as_stale() {
    // The member holds temper's gate address — `(SessionStart, no matcher)` — and runs
    // something else. That is a *stale* gate hook, not a settled one: skipping it is what
    // let a represented harness run a retired command spelling and read green forever
    // (0073), so the advisory names it and names the module to fix.
    //
    // The fixture replaces the starter's `SessionStart` gate hook rather than adding a
    // second member beside it: a hook authored *alongside* temper's leaves the gate wired,
    // which is the unchanged case, not this one.
    common::ensure_sdk_built();
    let root = write_harness("hook-supersede", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    let first = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(
        outcome_of(&first, temper::install::Placement::SessionStart),
        ApplyOutcome::Applied,
        "the lift wires the SessionStart hook through the program"
    );
    assert!(
        gate_installed(&root).is_empty(),
        "gate must be clean after a successful install"
    );

    // Replace the starter's gate hook with a member at the same identity and address, the
    // retired command spelling.
    fs::create_dir_all(temper_dir.join("hooks")).unwrap();
    fs::write(
        temper_dir.join("hooks").join("SessionStart.ts"),
        format!(
            "import {{ hook }} from \"@dtmd/temper/claude-code\";\n\n\
             export const hook_SessionStart = hook({{\n  \
             event: \"SessionStart\",\n  \
             hooks: [{{ type: \"command\", command: {RETIRED_SESSION_START_COMMAND:?} }}],\n\
             }});\n"
        ),
    )
    .unwrap();
    replace_starter_gate_hook(
        &temper_dir,
        "SessionStart",
        "./hooks/SessionStart.ts",
        &["hook_SessionStart"],
    );

    let second = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(
        outcome_of(&second, temper::install::Placement::SessionStart),
        ApplyOutcome::SupersededByMember,
        "a member seated at the gate hook's own address, running something else, is stale"
    );
    // The guard and the post-tool-use hook are untouched members, so they stay wired —
    // the vacuity guard on the claim above: staleness is per-address, not a whole-file
    // verdict that would pass over an empty `hooks` collection.
    for placement in [
        temper::install::Placement::GuardHook,
        temper::install::Placement::PostToolUseHook,
    ] {
        assert_eq!(
            outcome_of(&second, placement),
            ApplyOutcome::Unchanged,
            "{placement} is still projected by its own member"
        );
    }
    let settings_path = root.join(".claude").join("settings.json");
    let settings = fs::read_to_string(&settings_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    assert_eq!(
        json["hooks"]["SessionStart"][0]["hooks"][0]["command"], RETIRED_SESSION_START_COMMAND,
        "install never re-adds the gate hook — the member's own bytes are what emit projects, \
         got:\n{settings}"
    );

    // And `check`'s self-verify says so, with the remedy the author owes: the module, and
    // the typed constructor that states a binary instead of a command that can go stale.
    let findings = gate_installed(&root);
    let message = one_gate_finding(&findings);
    for needle in [
        "session-start hook stale",
        ".temper/hooks/SessionStart.ts",
        "gateHook({ event: \"SessionStart\" })",
    ] {
        assert!(
            message.contains(needle),
            "the stale report must name `{needle}`, got: {message}"
        );
    }
    assert_eq!(
        fs::read_to_string(&settings_path).unwrap(),
        settings,
        "the self-verify is read-only — it never writes the projection"
    );
}

/// Install a represented harness under `label`, then replace the starter's guard with the
/// `hook_PreToolUse` that `module` exports and re-emit, so the projection is that module's.
fn reinstall_with_guard_module(label: &str, module: &str) -> PathBuf {
    common::ensure_sdk_built();
    let root = write_harness(label, false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);
    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert!(gate_installed(&root).is_empty());

    fs::create_dir_all(temper_dir.join("hooks")).unwrap();
    fs::write(temper_dir.join("hooks").join("PreToolUse.ts"), module).unwrap();
    replace_starter_gate_hook(
        &temper_dir,
        "PreToolUse",
        "./hooks/PreToolUse.ts",
        &["hook_PreToolUse"],
    );
    temper::drift::emit_program(&temper_dir, temper::drift::EmitOptions::default()).unwrap();
    root
}

#[test]
fn a_fresh_represented_install_scaffolds_a_program_that_calls_starter() {
    common::ensure_sdk_built();
    let root = write_harness("fresh-install-calls-starter", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);
    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();

    // The default is the starter's: the gate hooks and the typecheck come from one call,
    // and the lifted members compose after its members, each still its own module.
    let program = fs::read_to_string(temper_dir.join("harness.ts")).unwrap();
    assert_eq!(
        program,
        "import { emit, harness, input } from \"@dtmd/temper\";\n\
         import { starter } from \"@dtmd/temper/starter\";\n\
         import { rule_collaboration } from \"./rules/collaboration.ts\";\n\
         import { rule_rust } from \"./rules/rust.ts\";\n\
         import { skill_coordinate } from \"./skills/coordinate.ts\";\n\n\
         const base = starter({ typecheck: { project: input(import.meta.url, \"./tsconfig.json\") } });\n\n\
         const program = harness({\n  ...base,\n  \
         members: [...base.members, rule_collaboration, rule_rust, skill_coordinate],\n});\n\n\
         process.stdout.write(emit(program).seam);\n"
    );
    // What the starter supplies the scaffold no longer writes, or the names would collide.
    assert!(
        !temper_dir.join("hooks").exists(),
        "no gate hook module is scaffolded beside the starter's gate hooks"
    );
    assert!(
        !program.contains("tool("),
        "no compiler tool is declared beside the starter's"
    );
}

#[test]
fn a_fresh_represented_install_wires_the_three_gate_hooks() {
    common::ensure_sdk_built();
    let root = write_harness("fresh-install-wires-gates", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);
    let discovery = install::discover(&root).unwrap();
    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    for placement in [
        temper::install::Placement::SessionStart,
        temper::install::Placement::GuardHook,
        temper::install::Placement::PostToolUseHook,
    ] {
        assert_eq!(
            outcome_of(&outcome, placement),
            ApplyOutcome::Applied,
            "the first emit wires {placement}"
        );
    }
    assert_gate_hooks_wired(&root, "after a fresh represented install");
    assert!(
        gate_installed(&root).is_empty(),
        "the installed-gate check reads every gate hook wired"
    );
}

/// Take temper's gate hook at `event` (a hook member's name is its event, then `:` and any
/// matcher) out of the starter's members in `harness.ts` and
/// compose `idents`, imported from `module`, in its place — how an author replaces one of
/// the starter's defaults by ordinary code. An empty `idents` drops the gate hook whole.
fn replace_starter_gate_hook(temper_dir: &Path, event: &str, module: &str, idents: &[&str]) {
    let path = temper_dir.join("harness.ts");
    let program = fs::read_to_string(&path).unwrap();
    assert!(program.contains("...base.members"), "{program}");
    let mut composed = format!(
        "...base.members.filter((member) => !(member.kind === \"hook\" && member.name.split(\":\")[0] === {event:?}))"
    );
    for ident in idents {
        composed.push_str(", ");
        composed.push_str(ident);
    }
    let mut out = String::new();
    if !idents.is_empty() {
        out.push_str(&format!(
            "import {{ {} }} from {module:?};\n",
            idents.join(", ")
        ));
    }
    out.push_str(&program.replacen("...base.members", &composed, 1));
    fs::write(&path, out).unwrap();
}

#[test]
fn a_gate_hook_reaching_a_declared_tool_is_reported_installed() {
    common::ensure_sdk_built();
    let root = write_harness("gate-hook-declared-tool", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);
    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();

    // The guard's module states the tool it runs, and the program composes that tool in
    // place of the starter's guard.
    fs::create_dir_all(temper_dir.join("hooks")).unwrap();
    fs::write(
        temper_dir.join("hooks").join("PreToolUse.ts"),
        "import { gateHook, tool } from \"@dtmd/temper/claude-code\";\n\n\
         export const tool_temper = tool({ system: \"temper\" });\n\
         export const hook_PreToolUse = gateHook({ event: \"PreToolUse\", \
         matcher: \"Write|Edit|MultiEdit\", tool: tool_temper });\n",
    )
    .unwrap();
    replace_starter_gate_hook(
        &temper_dir,
        "PreToolUse",
        "./hooks/PreToolUse.ts",
        &["tool_temper", "hook_PreToolUse"],
    );
    temper::drift::emit_program(&temper_dir, temper::drift::EmitOptions::default()).unwrap();

    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    assert_eq!(
        json["hooks"]["PreToolUse"][0]["hooks"][0]["command"].as_str(),
        Some(temper::install::GUARD_COMMAND),
        "the guard runs the system tool it reaches, got:\n{settings}"
    );
    let findings = gate_installed(&root);
    assert!(
        findings.is_empty(),
        "a gate hook member is installed at whatever tool it reaches, got: {findings:?}"
    );
}

#[test]
fn an_unrepresented_harnesss_gate_hook_runs_temper_on_the_path() {
    // With no program there is no package bin to anchor: the session-start hook the
    // unrepresented path writes resolves `temper` on the PATH.
    let root = write_harness("unrepresented-path-temper", true);
    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::No, false).unwrap();

    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    let command = json["hooks"]["SessionStart"][0]["hooks"][0]["command"]
        .as_str()
        .unwrap_or_else(|| panic!("the session-start hook carries a command, got:\n{settings}"));
    assert_eq!(
        Some(command),
        temper::install::gate_hook_command("SessionStart", "temper").as_deref()
    );
    assert!(
        command.contains("command -v temper ") && !command.contains("node_modules"),
        "{command}"
    );
}

#[test]
fn a_hook_at_a_gate_address_that_is_not_a_gate_hook_member_is_reported_stale() {
    let root = reinstall_with_guard_module(
        "hook-at-gate-address-not-gate",
        "import { hook } from \"@dtmd/temper/claude-code\";\n\n\
         export const hook_PreToolUse = hook({\n  event: \"PreToolUse\",\n  \
         matcher: \"Write|Edit|MultiEdit\",\n  \
         hooks: [{ type: \"command\", command: \"echo not the guard\" }],\n});\n",
    );
    let findings = gate_installed(&root);
    let message = one_gate_finding(&findings);
    for needle in [
        "guard hook stale",
        ".temper/hooks/PreToolUse.ts",
        "is not a gate hook",
        "gateHook({ event: \"PreToolUse\", matcher: \"Write|Edit|MultiEdit\" })",
    ] {
        assert!(
            message.contains(needle),
            "the stale report must name `{needle}`, got: {message}"
        );
    }
    assert!(
        !message.contains("session-start") && !message.contains("post-tool-use"),
        "the untouched gate hooks stay installed, got: {message}"
    );
}

#[test]
fn gate_installed_names_a_deleted_gate_hook_member_with_its_module_and_import_line() {
    // The author deleted the member — install never re-adds it (0073), so the advisory is
    // the whole intervention and owes both halves of the remedy: the module to author *and*
    // the `harness.ts` import line that reaches it. Naming only the module would have the
    // author write the unreached member `reached-from` indicts.
    common::ensure_sdk_built();
    let root = write_harness("hook-deleted", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert!(gate_installed(&root).is_empty());

    // Drop the starter's guard from the program, so it declares no hook at
    // `(PreToolUse, Write|Edit|MultiEdit)`.
    replace_starter_gate_hook(&temper_dir, "PreToolUse", "", &[]);

    let second = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(
        outcome_of(&second, temper::install::Placement::GuardHook),
        ApplyOutcome::Conflicted,
        "nothing is seated at the guard hook's address, so the member is missing"
    );
    let settings_path = root.join(".claude").join("settings.json");
    let settings = fs::read_to_string(&settings_path).unwrap();
    assert!(
        !settings.contains("Write|Edit|MultiEdit"),
        "install never re-adds the deleted gate hook — the author may have dropped it on \
         purpose, got:\n{settings}"
    );

    let findings = gate_installed(&root);
    let message = one_gate_finding(&findings);
    for needle in [
        "guard hook missing",
        "author `.temper/hooks/PreToolUse.ts`",
        "import { hook_PreToolUse } from \"./hooks/PreToolUse.ts\";",
        ".temper/harness.ts",
    ] {
        assert!(
            message.contains(needle),
            "the missing report must name `{needle}`, got: {message}"
        );
    }
    assert_eq!(
        fs::read_to_string(&settings_path).unwrap(),
        settings,
        "the self-verify is read-only — it never writes the projection"
    );
}

// ---------------------------------------------------------------------------
// guard — the lock, not the retired manifest, grounds the enforcement mode
// ---------------------------------------------------------------------------

/// The `.claude/` projection path the enforcement-mode arms bind a write to, fed to
/// [`common::guard_write_payload`] at each of them.
const CLAUDE_WRITE_PATH: &str = ".claude/skills/x/SKILL.md";

/// A lock at `mode` declaring [`CLAUDE_WRITE_PATH`] an emit-owned projection — real
/// enforcement-mode tests bind against a declared member, never a lock with no member
/// rows at all. Callers compose their own further rows onto it before writing.
fn claude_write_lock(mode: &str) -> common::GuardLock {
    common::GuardLock::declaring(mode).member("skill", "x", CLAUDE_WRITE_PATH, "abc", "abc")
}

/// Every guard fixture's lock is built by one writer, and this is the pin that the
/// writer's enforcement mode really lands in the bytes. `mode_from_declarations` reads an
/// absent — or unreadably-spelled — `mode` fact as `EnforcementMode::default()`, which is
/// `warn`: a `warn` fixture over a lock that declares nothing behaves exactly like one
/// that declares everything, so every warn arm below would stay green over a lock saying
/// nothing at all. Asserting the declared row *and* the mode the engine reads back off it
/// closes both halves for all three modes at once.
#[test]
fn a_guard_fixtures_lock_declares_its_enforcement_mode_rather_than_inheriting_the_default() {
    for (mode, resolved) in [
        ("note", EnforcementMode::Note),
        ("warn", EnforcementMode::Warn),
        ("block", EnforcementMode::Block),
    ] {
        let root = common::tmpdir(&format!("guard-lock-declares-{mode}"));
        common::GuardLock::declaring(mode)
            .member("rule", "safety", ".claude/rules/safety.md", "abc", "abc")
            .write(&root);

        // Parsed off the bytes rather than read through `drift::read_declarations`, whose
        // hoisting counters `check_cost` pins exact deltas on.
        let path = root.join(temper::WORKSPACE_DIR).join(temper::LOCK_FILENAME);
        let text = fs::read_to_string(&path).unwrap();
        let declarations = drift::parse_declarations(&path, &text).unwrap();

        let declared: Vec<&str> = declarations
            .assembly
            .iter()
            .filter(|row| row.fact == "mode")
            .filter_map(|row| row.value.as_deref())
            .collect();
        assert_eq!(
            declared,
            [mode],
            "the fixture lock must carry the `mode` fact itself, never leave it to the \
             default:\n{text}"
        );
        assert_eq!(
            compose::mode_from_declarations(&declarations).unwrap(),
            resolved,
            "and the engine must read that fact back as the mode declared:\n{text}"
        );
    }
}

#[test]
fn a_member_declaring_block_is_blocked_while_the_harness_is_in_warn() {
    let root = common::tmpdir("member-mode-block");
    claude_write_lock("warn")
        .member_mode("skill:x", "block")
        .write(&root);
    let (code, stderr) = common::run_guard(&root, &common::guard_write_payload(CLAUDE_WRITE_PATH));
    assert_eq!(
        code,
        Some(2),
        "the member's own `block` outranks the harness's `warn`"
    );
    assert!(
        stderr.contains("temper-managed projection"),
        "got: {stderr}"
    );
}

#[test]
fn a_kind_declaring_block_blocks_its_members_while_the_harness_is_in_warn() {
    let root = common::tmpdir("kind-mode-block");
    claude_write_lock("warn")
        .kind_mode("skill", "block")
        .write(&root);
    let (code, stderr) = common::run_guard(&root, &common::guard_write_payload(CLAUDE_WRITE_PATH));
    assert_eq!(
        code,
        Some(2),
        "the kind's `block` outranks the harness's `warn`"
    );
    assert!(
        stderr.contains("temper-managed projection"),
        "got: {stderr}"
    );
}

/// Overrides naming another member and another kind leave this member at the root's mode,
/// and a member's own mode outranks its kind's in either direction.
#[test]
fn with_no_member_override_the_harness_mode_governs() {
    let root = common::tmpdir("member-mode-absent");
    claude_write_lock("warn")
        .member("skill", "y", ".claude/skills/y/SKILL.md", "abc", "abc")
        .member_mode("skill:y", "block")
        .kind_mode("rule", "block")
        .write(&root);
    let (code, output) = common::run_guard(&root, &common::guard_write_payload(CLAUDE_WRITE_PATH));
    assert_eq!(
        code,
        Some(0),
        "the harness's `warn` governs an unoverridden member"
    );
    assert!(
        output.contains("temper-managed projection"),
        "got: {output}"
    );

    let softened = common::tmpdir("member-mode-softens-kind");
    claude_write_lock("block")
        .kind_mode("skill", "block")
        .member_mode("skill:x", "warn")
        .write(&softened);
    let (code, _) = common::run_guard(&softened, &common::guard_write_payload(CLAUDE_WRITE_PATH));
    assert_eq!(code, Some(0), "the member's own mode is the most specific");
}

/// The skill beside [`CLAUDE_WRITE_PATH`] that declares no mode of its own.
const SIBLING_WRITE_PATH: &str = ".claude/skills/y/SKILL.md";

/// A lock at `mode` declaring two emit-owned skill projections, `skill:x` at
/// [`CLAUDE_WRITE_PATH`] (its own mode `member_mode`) and `skill:y` at
/// [`SIBLING_WRITE_PATH`] (none), with both projections on disk drifted from the
/// fingerprint the lock recorded — the tree a shell call left.
fn drifted_pair_harness(label: &str, mode: &str, member_mode: &str) -> PathBuf {
    let root = common::tmpdir(label);
    claude_write_lock(mode)
        .member("skill", "y", SIBLING_WRITE_PATH, "abc", "abc")
        .member_mode("skill:x", member_mode)
        .write(&root);
    for path in [CLAUDE_WRITE_PATH, SIBLING_WRITE_PATH] {
        common::write_sibling(&root, path, "---\nname: drifted\n---\n");
    }
    root
}

#[test]
fn a_shell_edge_finding_on_a_member_declaring_block_refuses_the_result_under_a_warn_root() {
    let root = drifted_pair_harness("member-mode-shell-block", "warn", "block");
    let (code, output) = common::run_guard(&root, SHELL_CALL_PAYLOAD);
    assert_eq!(
        code,
        Some(0),
        "the post edge refuses in-band, got: {output}"
    );
    let reason = common::post_edge_refusal(&output);
    assert!(
        reason.contains(CLAUDE_WRITE_PATH),
        "the blocking member's drift is the refusal, got: {reason}"
    );
    assert!(
        reason.contains(SIBLING_WRITE_PATH),
        "the warned sibling still reaches the session, got: {reason}"
    );
    assert!(
        reason.find(CLAUDE_WRITE_PATH) < reason.find(SIBLING_WRITE_PATH),
        "the blocking finding leads the refusal, got: {reason}"
    );

    // A member declaring `note` under a `block` root is recorded only: the root's
    // sibling refuses, and the noted member is named nowhere in-band.
    let root = drifted_pair_harness("member-mode-shell-note", "block", "note");
    let (code, output) = common::run_guard(&root, SHELL_CALL_PAYLOAD);
    assert_eq!(code, Some(0), "got: {output}");
    let reason = common::post_edge_refusal(&output);
    assert!(reason.contains(SIBLING_WRITE_PATH), "got: {reason}");
    assert!(
        !reason.contains(CLAUDE_WRITE_PATH),
        "the noted member's finding rides no in-band surface, got: {reason}"
    );
}

#[test]
fn a_manifest_write_finding_follows_the_members_declared_mode() {
    // A write dropping the `gmail` server: its own `block` refuses the write under a
    // `warn` root.
    let root = common::tmpdir("member-mode-manifest-block");
    common::GuardLock::declaring("warn")
        .registration("mcp-server", "gmail", ".mcp.json", "mcpServers.*")
        .member_mode("mcp-server:gmail", "block")
        .write(&root);
    let dropped = common::guard_write_payload_with_content(".mcp.json", r#"{"mcpServers":{}}"#);
    let (code, stderr) = common::run_guard(&root, &dropped);
    assert_eq!(
        code,
        Some(2),
        "the member's own `block` denies it: {stderr}"
    );
    assert!(stderr.contains("gmail"), "got: {stderr}");

    // A contract violation on the same member is decided at the same mode.
    let violating = common::guard_write_payload_with_content(
        ".mcp.json",
        r#"{"mcpServers":{"gmail":{"type":"carrier-pigeon","command":"npx"}}}"#,
    );
    let (code, stderr) = common::run_guard(&root, &violating);
    assert_eq!(code, Some(2), "got: {stderr}");
    assert!(stderr.contains("carrier-pigeon"), "got: {stderr}");

    // A member declaring `note` under a `block` root is recorded only.
    let noted = common::tmpdir("member-mode-manifest-note");
    common::GuardLock::declaring("block")
        .registration("mcp-server", "gmail", ".mcp.json", "mcpServers.*")
        .member_mode("mcp-server:gmail", "note")
        .write(&noted);
    for payload in [&dropped, &violating] {
        let (code, output) = common::run_guard(&noted, payload);
        assert_eq!(code, Some(0), "note allows the write: {output}");
        assert!(output.is_empty(), "and surfaces nothing in-band: {output}");
    }
}

#[test]
fn guard_reads_the_block_mode_from_the_lock_not_the_retired_manifest() {
    let root = common::tmpdir("lock-mode-block");
    claude_write_lock("block").write(&root);
    // A stray retired manifest naming the opposite enforcement mode must be ignored entirely —
    // the manifest is never read at all, by this or any other verb.
    fs::write(
        root.join(format!("temper{}toml", '.')),
        "authority = \"warn\"\n",
    )
    .unwrap();

    let (code, stderr) = common::run_guard(&root, &common::guard_write_payload(CLAUDE_WRITE_PATH));
    assert_eq!(code, Some(2), "the lock's `block` mode must block");
    // The in-band surface carries the binding limit whole, naming the tools the guard's
    // `PreToolUse` row binds — the sentence reaches the user, tool list included.
    assert!(
        stderr.contains(install::binding_limit()),
        "the guard's in-band message must carry the binding limit, got: {stderr}"
    );
}

/// With no `lock.toml` at all the lock names no projection, so every file is an
/// authored source: a write to a `.claude/` skill draws no projection warning.
#[test]
fn a_no_lock_write_to_a_skill_draws_no_projection_warning() {
    let root = common::tmpdir("lock-absent-skill-write");
    let (code, stderr) = common::run_guard(&root, &common::guard_write_payload(CLAUDE_WRITE_PATH));
    assert_eq!(code, Some(0), "with no lock the guard never blocks");
    assert!(
        stderr.is_empty(),
        "with no lock nothing is a projection, got: {stderr}"
    );

    let (allow_code, allow_stderr) =
        common::run_guard(&root, &common::guard_write_payload("src/main.rs"));
    assert_eq!(allow_code, Some(0));
    assert!(allow_stderr.is_empty());
}

/// The lock alone names projections: under one, a write to the path it names is still
/// guarded at the lock's enforcement mode.
#[test]
fn with_a_lock_a_write_to_a_lock_named_projection_is_still_guarded() {
    let root = common::tmpdir("lock-named-projection-guarded");
    claude_write_lock("warn").write(&root);
    let (code, output) = common::run_guard(&root, &common::guard_write_payload(CLAUDE_WRITE_PATH));
    assert_eq!(code, Some(0), "warn surfaces but never blocks");
    assert!(
        output.contains("temper-managed projection"),
        "a lock-named projection write must be guarded, got: {output}"
    );
}

/// When the lock declares an emit-owned target outside `.claude/`, the guard binds
/// writes to that path just as it does for `.claude/` projections — the filter
/// derives from lock-declared targets, not a hardcoded `.claude/` regex.
#[test]
fn guard_binds_declared_locus_targets_outside_claude() {
    let root = common::tmpdir("lock-declared-outside-claude");

    // A lock with a `block` mode and a single emit-owned target outside `.claude/`
    // (e.g., a layout kind that governs `.rules/` directly).
    common::GuardLock::declaring("block")
        .member("rule", "safety", ".rules/safety.md", "def", "def")
        .write(&root);

    // A write targeting the declared `.rules/safety.md` path should be bound by the
    // guard, not silently allowed (the bug the entry fixes).
    let (code, stderr) = common::run_guard(&root, &common::guard_write_payload(".rules/safety.md"));
    assert_eq!(
        code,
        Some(2),
        "a declared-locus target outside .claude/ must be bound (block mode)"
    );
    assert!(stderr.contains("temper-managed projection"));

    // A `.claude/` path with no declared target is not a projection — but it lands in
    // the `skill` kind's governed locus and the lock declares no member there, so the
    // locus binding catches it. Representation must not loosen the boundary the no-lock
    // fallback would have held.
    let (undeclared_code, undeclared_stderr) =
        common::run_guard(&root, &common::guard_write_payload(CLAUDE_WRITE_PATH));
    assert_eq!(
        undeclared_code,
        Some(2),
        "an undeclared document at a governed locus is bound, not allowed (block mode)"
    );
    assert!(
        undeclared_stderr.contains("`skill` kind's governed locus"),
        "the finding must name the kind whose locus the write landed in, got: {undeclared_stderr}"
    );

    // A write to an entirely different path should be allowed.
    let (other_code, other_stderr) =
        common::run_guard(&root, &common::guard_write_payload("src/main.rs"));
    assert_eq!(other_code, Some(0));
    assert!(other_stderr.is_empty());
}

/// A represented harness whose lock declares one `rule` projection and nothing else —
/// the fixture the governed-locus binding is judged on. `mode` is the caller's, so one
/// writer serves the block/warn/note arms.
fn represented_rule_harness(name: &str, mode: &str) -> std::path::PathBuf {
    let root = common::tmpdir(name);
    common::GuardLock::declaring(mode)
        .member("rule", "safety", ".claude/rules/safety.md", "abc", "abc")
        .write(&root);
    root
}

/// A pending write creating a document inside a represented committed kind's governed
/// locus that the lock declares no member for is bound at the author's declared mode —
/// `emit` will never maintain the file and Claude Code loads it anyway. Before this, a
/// path that was not an exact lock row was allowed outright, so representation *loosened*
/// the boundary the no-lock fallback held.
#[test]
fn guard_binds_an_undeclared_write_inside_a_governed_locus() {
    // block — the write is denied and the finding names the governing kind and the
    // declare-and-re-emit remedy `check`'s root `locus-declared` clause names.
    let block_root = represented_rule_harness("guard-undeclared-locus-block", "block");
    let (code, stderr) = common::run_guard(
        &block_root,
        &common::guard_write_payload(".claude/rules/stray.md"),
    );
    assert_eq!(
        code,
        Some(2),
        "block mode must deny an undeclared document at the `rule` locus, got: {stderr}"
    );
    assert!(
        stderr.contains("`rule` kind's governed locus"),
        "the finding must name the kind whose locus it landed in, got: {stderr}"
    );
    assert!(
        stderr.contains("declare the member in the program and re-emit"),
        "the finding must name the same remedy `check` names, got: {stderr}"
    );

    // `.claude/agents/` is the arm a lock-row-sourced locus set would have missed: a
    // `[[declaration.kind]]` row exists only for a kind the program uses, and this lock
    // carries none. The locus set is embedded kind data, so the binding holds anyway.
    let (agent_code, agent_stderr) = common::run_guard(
        &block_root,
        &common::guard_write_payload(".claude/agents/stray.md"),
    );
    assert_eq!(
        agent_code,
        Some(2),
        "a kind the lock carries no row for still binds its locus, got: {agent_stderr}"
    );
    assert!(
        agent_stderr.contains("`agent` kind's governed locus"),
        "the finding must name the `agent` kind, got: {agent_stderr}"
    );

    // warn — allowed, with the finding surfaced in-band.
    let warn_root = represented_rule_harness("guard-undeclared-locus-warn", "warn");
    let (warn_code, warn_output) = common::run_guard(
        &warn_root,
        &common::guard_write_payload(".claude/rules/stray.md"),
    );
    assert_eq!(warn_code, Some(0), "warn mode allows the write");
    let in_band = common::guard_in_band(&warn_output, "PreToolUse");
    assert!(
        in_band.contains("`rule` kind's governed locus"),
        "warn surfaces the finding in-band, got: {in_band}"
    );

    // note — allowed, and nothing reaches the live session.
    let note_root = represented_rule_harness("guard-undeclared-locus-note", "note");
    let (note_code, note_output) = common::run_guard(
        &note_root,
        &common::guard_write_payload(".claude/rules/stray.md"),
    );
    assert_eq!(note_code, Some(0), "note mode allows the write");
    assert!(
        note_output.is_empty(),
        "note records out-of-band only, got: {note_output}"
    );
}

/// A pending write at a gitignored path inside a governed locus names a document
/// discovery never reaches — the repo declares it not authored here — so `check` judges
/// no member there and the guard binds nothing either. A sibling the ignore rules leave
/// alone is still bound, so the allowance is the ignore rule's, not the locus's.
#[test]
fn the_guard_does_not_judge_a_pending_write_to_a_gitignored_locus_path() {
    let root = represented_rule_harness("guard-gitignored-locus", "block");
    fs::write(root.join(".gitignore"), ".claude/rules/local.md\n").unwrap();

    let (code, stderr) = common::run_guard(
        &root,
        &common::guard_write_payload(".claude/rules/local.md"),
    );
    assert_eq!(
        code,
        Some(0),
        "a gitignored locus path is no member `check` judges, got: {stderr}"
    );
    assert!(stderr.is_empty(), "a pass prints nothing, got: {stderr}");

    let (stray_code, _) = common::run_guard(
        &root,
        &common::guard_write_payload(".claude/rules/stray.md"),
    );
    assert_eq!(stray_code, Some(2), "an unignored sibling stays bound");
}

/// The arms the governed-locus binding must leave exactly where they were: a declared
/// projection keeps the projection wording, a `local`-commitment locus is never bound by
/// it, a registration kind's `collection_address` keeps it off its host manifest, and an
/// unrepresented harness keeps the `.claude/` fallback. Its neighbour: a container kind's
/// own locus binds wherever it is rooted, `.claude/settings.json` and `.mcp.json` included.
#[test]
fn the_governed_locus_binding_leaves_the_neighbouring_guard_arms_alone() {
    let root = represented_rule_harness("guard-locus-neighbours", "block");

    // A declared projection is drift, not an undeclared member — the projection binding
    // is consulted first and its wording is what the author reads.
    let (declared_code, declared_stderr) = common::run_guard(
        &root,
        &common::guard_write_payload(".claude/rules/safety.md"),
    );
    assert_eq!(declared_code, Some(2));
    assert!(
        declared_stderr.contains("temper-managed projection"),
        "a declared projection keeps the projection finding, got: {declared_stderr}"
    );

    // A `local`-commitment kind's documents are the author's own by declaration — never
    // an emit input or target — so no member is ever declared at their loci and the
    // binding must not fire there. `settings.local.json` and the dial are the two.
    for local_path in [".claude/settings.local.json", ".temper/dial.toml"] {
        let (local_code, local_stderr) =
            common::run_guard(&root, &common::guard_write_payload(local_path));
        assert_eq!(
            local_code,
            Some(0),
            "a local-commitment locus is never bound by the locus binding ({local_path}), got: {local_stderr}"
        );
        assert!(local_stderr.is_empty());
    }

    // `.mcp.json` is not emit-owned in this lock (no registration rows), so no manifest
    // arm claims it: the `mcp` container governs the file at the harness root and binds
    // it like any container locus, while `mcp-server`'s `collection_address` keeps the
    // registration kind off it — one finding, the container's.
    let (manifest_code, manifest_stderr) =
        common::run_guard(&root, &common::guard_write_payload(".mcp.json"));
    assert_eq!(
        manifest_code,
        Some(2),
        "the `mcp` container's locus binds, got: {manifest_stderr}"
    );
    assert!(
        manifest_stderr.contains("`mcp` kind's governed locus")
            && !manifest_stderr.contains("`mcp-server` kind"),
        "the finding names the container, never the registration kind, got: {manifest_stderr}"
    );

    // `.claude/settings.json` is the other side of that exclusion: the `settings`
    // container governs the file and carries no collection address, so its locus binds
    // like any other — the guard saying what `check` already says about the same file.
    let (settings_code, settings_stderr) =
        common::run_guard(&root, &common::guard_write_payload(".claude/settings.json"));
    assert_eq!(
        settings_code,
        Some(2),
        "the settings container's locus binds, got: {settings_stderr}"
    );
    assert!(
        settings_stderr.contains("`settings` kind's governed locus"),
        "the finding names the `settings` kind, got: {settings_stderr}"
    );

    // No lock at all: nothing is a projection and no locus declares a member, so no
    // write binds — `.claude/` or otherwise.
    let bare_root = common::tmpdir("guard-locus-no-lock");
    fs::create_dir_all(&bare_root).unwrap();
    for path in [
        ".claude/rules/stray.md",
        ".claude/settings.local.json",
        "specs/intent.md",
    ] {
        let (code, stderr) = common::run_guard(&bare_root, &common::guard_write_payload(path));
        assert_eq!(code, Some(0), "with no lock the guard allows ({path})");
        assert!(
            stderr.is_empty(),
            "with no lock nothing binds ({path}), got: {stderr}"
        );
    }
}

/// A manifest a lock's registration rows key inside is emit-owned — `emit` regenerates
/// the file whole — so the guard must bind writes to it, not silently allow them. The
/// fixture declares no kind row of its own: the path resolves off the built-in floor's
/// facts for the registration kind the row names.
#[test]
fn guard_binds_settings_json_when_registration_members_compose() {
    let root = common::tmpdir("guard-settings-json-emit-owned");

    // A lock with `block` mode and a hook member (which composes into settings.json).
    common::GuardLock::declaring("block")
        .registration("hook", "SessionStart", "settings.json", "hooks.<Event>")
        .write(&root);

    // A pending write to .claude/settings.json should be bound by the guard
    // (not silently allowed), since the hook member composes into it.
    let (code, stderr) =
        common::run_guard(&root, &common::guard_write_payload(".claude/settings.json"));
    assert_eq!(
        code,
        Some(2),
        "a pending write to .claude/settings.json must be bound when registration members exist (block mode)"
    );
    assert!(
        stderr.contains("temper-managed projection"),
        "the guard message must indicate it's a managed projection"
    );

    // Verify that under `warn` mode, the same write is allowed but surfaces the finding.
    let warn_root = common::tmpdir("guard-settings-json-warn");
    common::GuardLock::declaring("warn")
        .registration("hook", "SessionStart", "settings.json", "hooks.<Event>")
        .write(&warn_root);

    let (warn_code, warn_output) = common::run_guard(
        &warn_root,
        &common::guard_write_payload(".claude/settings.json"),
    );
    assert_eq!(
        warn_code,
        Some(0),
        "warn mode allows the write but surfaces the finding"
    );
    assert!(
        common::guard_in_band(&warn_output, "PreToolUse").contains("temper-managed projection"),
        "the warning must be in-band"
    );
}

/// `settings.json` is not the whole constituency: an `mcp-server` row keys inside
/// `.mcp.json`, and that manifest is emit-owned on exactly the same ground — the kind
/// facts resolve the registration's manifest to its governed path, whichever manifest it
/// names.
#[test]
fn guard_binds_a_registration_s_own_manifest_outside_settings_json() {
    let root = common::tmpdir("guard-mcp-json-emit-owned");

    common::GuardLock::declaring("block")
        .registration("mcp-server", "docs", ".mcp.json", "mcpServers.*")
        .write(&root);

    let (code, stderr) = common::run_guard(&root, &common::guard_write_payload(".mcp.json"));
    assert_eq!(
        code,
        Some(2),
        "a pending write to the `.mcp.json` the lock's servers compose must be bound, got: {stderr}"
    );
    assert!(
        stderr.contains("temper-managed projection"),
        "the finding names it a managed projection, got: {stderr}"
    );
    assert!(
        stderr.contains("`mcp` member") && stderr.contains(".mcp.json"),
        "the owner line names the container kind governing the path, got: {stderr}"
    );
}

/// A container member's own row already owns its manifest's path, so the derived set adds
/// no second entry for it: the path appears exactly once, under the member's real name.
/// Two entries would double the path's `install` placement and let the container label
/// outrank the member in a path-keyed index.
#[test]
fn an_emit_owned_manifest_is_named_once_by_its_container_member() {
    let root = common::tmpdir("emit-owned-manifest-dedupe");

    common::GuardLock::declaring("block")
        .member("settings", "project", ".claude/settings.json", "abc", "abc")
        .registration("hook", "SessionStart", "settings.json", "hooks.<Event>")
        .registration("hook", "PreToolUse", "settings.json", "hooks.<Event>")
        .write(&root);

    let targets = drift::emit_owned_targets(&root.join(temper::WORKSPACE_DIR));
    let settings: Vec<&drift::EmitOwnedEntry> = targets
        .iter()
        .filter(|target| &*target.path == ".claude/settings.json")
        .collect();
    assert_eq!(
        settings.len(),
        1,
        "`.claude/settings.json` is one emit-owned target, not one per row that keys in it"
    );
    assert_eq!(
        (settings[0].kind.as_str(), settings[0].name.as_str()),
        ("settings", "project"),
        "the container member's own row names the path, never a synthetic label"
    );

    // And the guard's refusal names that member — the owner a drift finding owes the author.
    let (code, stderr) =
        common::run_guard(&root, &common::guard_write_payload(".claude/settings.json"));
    assert_eq!(code, Some(2), "block mode denies the write, got: {stderr}");
    assert!(
        stderr.contains("the `settings` member `project`"),
        "the owner line names the container member, got: {stderr}"
    );
}

/// The guard matches a projection by path equality, never by suffix: a `file_path`
/// ending in a projected file's bare name but living anywhere other than the declared
/// path resolves Allow, not Block — whether it sits deeper in the tree
/// (`.temper/memory/CLAUDE.md`) or one segment over in a sibling directory
/// (`reference/CLAUDE.md`). Uses absolute paths as Claude Code actually sends them.
#[test]
fn guard_matches_a_projection_by_path_equality_not_suffix() {
    let root = common::tmpdir("guard-path-equality");

    // A lock with `block` mode and the root `CLAUDE.md` as a projected member
    // (the memory kind's single-segment projection).
    common::GuardLock::declaring("block")
        .member("memory", "root", "CLAUDE.md", "abc", "abc")
        .write(&root);

    // A write to the actual root CLAUDE.md projection (absolute path) should be blocked.
    let (code, stderr) = common::run_guard(
        &root,
        &common::guard_write_payload(&root.join("CLAUDE.md").to_string_lossy()),
    );
    assert_eq!(
        code,
        Some(2),
        "the actual CLAUDE.md projection (absolute path) should be blocked"
    );
    assert!(stderr.contains("temper-managed projection"));

    // Every other path ending in `CLAUDE.md` is a different file, not the projection.
    // One under the surface workspace is fenced from discovery, so nothing binds it.
    let (code, stderr) = common::run_guard(
        &root,
        &common::guard_write_payload(&root.join(".temper/memory/CLAUDE.md").to_string_lossy()),
    );
    assert_eq!(
        code,
        Some(0),
        "a fenced `CLAUDE.md` is no projection: {stderr}"
    );
    assert!(stderr.is_empty(), "and surfaces nothing: {stderr}");

    // One a single segment over is an undeclared document at `memory`'s locus — bound,
    // but as the locus finding, never as this projection.
    let (code, stderr) = common::run_guard(
        &root,
        &common::guard_write_payload(&root.join("reference/CLAUDE.md").to_string_lossy()),
    );
    assert_eq!(
        code,
        Some(2),
        "block denies the undeclared document: {stderr}"
    );
    assert!(
        !stderr.contains("temper-managed projection")
            && stderr.contains("`memory` kind's governed locus"),
        "`reference/CLAUDE.md` is the locus finding, not the projection's: {stderr}"
    );
}

/// A declared projection whose lock row carries the older `./` spelling is bound exactly
/// as the bare spelling is. An engine before the workspace path was lexically normalized
/// wrote a root-`.` member as `./CLAUDE.md`, and both sides of the projection compare
/// normalize at read time (decision 0024) — through
/// [`temper::path::to_canonical_spelling`], the one home for the spelling. Left
/// un-canonicalized, the candidate matched no relativized `file_path` and the write to a
/// live projection passed unchecked.
///
/// The memory kind's `.`-rooted locus binds an undeclared `CLAUDE.md` too, so only the
/// owner line proves the projection compare spoke rather than the locus arm.
#[test]
fn guard_binds_a_projection_whose_lock_row_carries_the_older_dot_slash_spelling() {
    let root = common::tmpdir("guard-dot-slash-row");
    common::GuardLock::declaring("block")
        .member("memory", "root", "./CLAUDE.md", "abc", "abc")
        .write(&root);

    // Vacuous unless the lock on disk really carries the older spelling — the fixture
    // writes the row verbatim, and a writer that canonicalized it would leave this case
    // asserting the bare spelling a second time.
    let lock = fs::read_to_string(root.join(temper::WORKSPACE_DIR).join(temper::LOCK_FILENAME))
        .expect("the fixture lock must be readable");
    assert!(
        lock.contains(r#"source_path = "./CLAUDE.md""#),
        "the fixture must declare the older `./` spelling: {lock}"
    );

    // The write Claude Code actually sends: the projection's absolute path.
    let (code, stderr) = common::run_guard(
        &root,
        &common::guard_write_payload(&root.join("CLAUDE.md").to_string_lossy()),
    );
    assert_eq!(
        code,
        Some(2),
        "the `./`-spelled row names a live projection and must block the write: {stderr}"
    );
    assert!(
        stderr.contains("the `memory` member `root`"),
        "and block through the projection compare, which names the row's owner: {stderr}"
    );
    assert!(
        stderr.contains("owns `CLAUDE.md`"),
        "the owner line spells the path canonically, never the row's older `./`: {stderr}"
    );

    // And the compare still names nothing it does not spell: canonicalizing a row widens
    // no match beyond the one file it spells, so a sibling `CLAUDE.md` is the locus
    // arm's undeclared document, never this row's projection.
    let (_, stderr) = common::run_guard(
        &root,
        &common::guard_write_payload(&root.join("reference/CLAUDE.md").to_string_lossy()),
    );
    assert!(
        !stderr.contains("temper-managed projection")
            && stderr.contains("`memory` kind's governed locus"),
        "an undeclared sibling is no projection: {stderr}"
    );
}

/// A declared projection whose path needs a JSON escape to ride the payload is bound like
/// any other. The guard reads `file_path` off the decoded payload, so the value it compares
/// is the path the tool named; the retired field-scoped grep captured `[^"]*` and stopped at
/// the escape's own quote, handing the compare a truncated prefix that matched no row — and
/// the write through.
///
/// The fixture's root contract binds `reachable` alone, so `locus-declared` is unbound and
/// the projection compare is the one binding in play: the governed-locus arm cannot catch
/// what the compare misses. The allowed sibling below is that fact's own pin — with the
/// locus binding live it would block, and this case would stay green over the wrong judge.
#[test]
fn guard_binds_a_projection_whose_file_path_value_carries_a_json_escape() {
    // An embedded quote: legal in a POSIX path, and a two-character escape on the wire.
    const ESCAPED_PATH: &str = ".claude/skills/x\"y/SKILL.md";

    let root = common::tmpdir("guard-escaped-file-path");
    common::GuardLock::declaring("block")
        .member("skill", "x", ESCAPED_PATH, "abc", "abc")
        .clause_row(ClauseRow {
            label: Some("root.reachable".to_string()),
            ..common::clause("reachable", "advisory")
        })
        .write(&root);

    let payload = common::guard_write_payload(ESCAPED_PATH);
    assert!(
        payload.contains("x\\\"y"),
        "the pin is vacuous unless the payload really escapes the path value: {payload}"
    );

    let (code, stderr) = common::run_guard(&root, &payload);
    assert_eq!(
        code,
        Some(2),
        "the declared projection must be bound however its path spells on the wire: {stderr}"
    );
    assert!(
        stderr.contains("temper-managed projection"),
        "and bound by the projection compare, which names the row's owner: {stderr}"
    );

    // No `locus-declared` clause ⇒ no governed locus to fall through to, so the arm above
    // is the only one that could have blocked.
    let (stray_code, stray_output) = common::run_guard(
        &root,
        &common::guard_write_payload(".claude/rules/stray.md"),
    );
    assert_eq!(
        stray_code,
        Some(0),
        "the fixture binds no governed locus: {stray_output}"
    );
    assert!(
        stray_output.is_empty(),
        "and surfaces nothing: {stray_output}"
    );
}

/// Stdin that is not a hook envelope refuses loud and judges nothing. The guard's input is
/// machine-written, so arbitrary text means the hook row is mis-wired — a condition that
/// disables the boundary wholesale, and so exactly what invariant 6 forbids degrading
/// silently over. The retired grep instead scraped a `file_path` out of whatever text
/// arrived and rendered a verdict on it, which is the shape pinned against here.
#[test]
fn guard_refuses_stdin_that_is_not_a_hook_payload() {
    let root = common::tmpdir("guard-unparseable-payload");
    claude_write_lock("block").write(&root);

    // Prose that happens to mention the field, with the declared projection's path beside
    // it — the grep read both and blocked a write no hook announced.
    let payload = format!("mis-wired: \"file_path\": \"{CLAUDE_WRITE_PATH}\"\n");
    let (code, output) = common::run_guard(&root, &payload);
    assert_eq!(
        code,
        Some(1),
        "a payload that is not JSON is a hard error, not a verdict: {output}"
    );
    assert!(
        !output.contains("temper-managed projection"),
        "no verdict is rendered over text the guard could not decode: {output}"
    );

    // miette word-wraps the rendered message across its own gutter, so a multi-word
    // phrase never appears contiguously in the bytes: match against the render flattened
    // back to one line rather than pinning the terminal width the wrap happened at.
    let flattened = output
        .replace(['│', '×'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    for named in ["`PreToolUse`/`PostToolUse` hook row", "`temper install`"] {
        assert!(
            flattened.contains(named),
            "the refusal must name the mis-wired hook and its remedy ({named}): {output}"
        );
    }
}

/// `temper guard .` — the invocation shape `install` writes into every settings.json
/// hook command — must reach the identical verdict as `temper guard <absolute root>`.
/// A relative root that is never resolved against the working directory normalizes to
/// the empty path, whose prefix strips trivially off every absolute `file_path`, leaving
/// it unrelativized and every projection unbound.
#[test]
fn guard_reaches_the_same_verdict_from_a_dot_root_as_from_an_absolute_root() {
    let root = common::tmpdir("guard-dot-root");
    claude_write_lock("block")
        .member("memory", "root", "CLAUDE.md", "abc", "abc")
        .write(&root);

    // (file_path as the payload spells it, the verdict both invocation forms owe it)
    let cases: Vec<(String, Option<i32>)> = vec![
        // The declared projections, absolute — what Claude Code actually sends.
        (
            root.join("CLAUDE.md").to_string_lossy().into_owned(),
            Some(2),
        ),
        (
            root.join(".claude/skills/x/SKILL.md")
                .to_string_lossy()
                .into_owned(),
            Some(2),
        ),
        // Undeclared paths, absolute — one at `memory`'s locus, bound as undeclared.
        (
            root.join("reference/CLAUDE.md")
                .to_string_lossy()
                .into_owned(),
            Some(2),
        ),
        (
            root.join("src/main.rs").to_string_lossy().into_owned(),
            Some(0),
        ),
        // Out of tree entirely — relativizes against neither root spelling.
        (
            common::tmpdir("guard-dot-root-elsewhere")
                .join("CLAUDE.md")
                .to_string_lossy()
                .into_owned(),
            Some(0),
        ),
        // Already harness-relative — the form the fixtures speak.
        ("CLAUDE.md".to_string(), Some(2)),
        (".claude/skills/x/SKILL.md".to_string(), Some(2)),
        ("README.md".to_string(), Some(0)),
    ];

    for (file_path, expected) in cases {
        let payload = common::guard_write_payload(&file_path);
        let (abs_code, abs_stderr) = common::run_guard(&root, &payload);
        let (dot_code, dot_stderr) = common::run_guard_from_root(&root, &payload);
        assert_eq!(
            abs_code, expected,
            "`guard <absolute root>` verdict for {file_path}: {abs_stderr}"
        );
        assert_eq!(
            dot_code, abs_code,
            "`guard .` must reach the same verdict as `guard <absolute root>` for {file_path}: {dot_stderr}"
        );
        assert_eq!(
            dot_stderr, abs_stderr,
            "and surface the same finding for {file_path}"
        );
    }
}

// ---------------------------------------------------------------------------
// guard — represented-manifest member contract (entry 4/5), beside the
// `.claude/`-projection-drift binding it extends
// ---------------------------------------------------------------------------

/// A `block` harness whose lock also declares [`CLAUDE_WRITE_PATH`] an emit-owned
/// projection — so one lock exercises both bindings the guard now runs.
fn manifest_guard_harness(slug: &str) -> PathBuf {
    let root = common::tmpdir(slug);
    claude_write_lock("block").write(&root);
    root
}

#[test]
fn guard_flags_a_represented_manifest_whose_member_violates_its_contract() {
    let root = manifest_guard_harness("guard-manifest-block");

    // An `.mcp.json` write whose `gmail` server declares an undocumented transport violates
    // the `mcp-server` contract's `type` enum — flagged, and under `block` the write is denied.
    let (code, stderr) = common::run_guard(
        &root,
        &common::guard_write_payload_with_content(
            ".mcp.json",
            r#"{"mcpServers":{"gmail":{"type":"carrier-pigeon","command":"npx"}}}"#,
        ),
    );
    assert_eq!(
        code,
        Some(2),
        "a `block` harness denies a contract-violating manifest write"
    );
    assert!(
        stderr.contains("temper-governed manifest"),
        "the finding names the broken contract, not the file edited: {stderr}"
    );
    assert!(
        stderr.contains("type") && stderr.contains("carrier-pigeon"),
        "the finding surfaces the offending field and value: {stderr}"
    );

    // The same file with a documented transport conforms — the guard passes it silently,
    // never blanket-blocking a manifest the way it does a `.claude/` projection.
    let (ok_code, ok_stderr) = common::run_guard(
        &root,
        &common::guard_write_payload_with_content(
            ".mcp.json",
            r#"{"mcpServers":{"gmail":{"type":"stdio","command":"npx"}}}"#,
        ),
    );
    assert_eq!(ok_code, Some(0), "a conforming manifest write is allowed");
    assert!(
        ok_stderr.is_empty(),
        "a conforming write surfaces nothing: {ok_stderr}"
    );

    // The existing `.claude/` projection-drift binding is unaffected: a direct edit to a
    // projected path still blocks under the same lock, at the same enforcement mode.
    let (proj_code, proj_stderr) =
        common::run_guard(&root, &common::guard_write_payload(CLAUDE_WRITE_PATH));
    assert_eq!(
        proj_code,
        Some(2),
        "the projection binding still denies a projection write"
    );
    assert!(proj_stderr.contains("temper-managed projection"));
}

#[test]
fn guard_follows_the_declared_mode_for_a_manifest_violation() {
    // The manifest binding acts at the same three-valued enforcement mode the projection
    // binding does: `warn` surfaces the finding in-band but allows the write (exit 0), and
    // `note` allows it with no in-band message at all — the finding rides the next report.
    for (mode, expect_stderr) in [("warn", true), ("note", false)] {
        let root = common::tmpdir(&format!("guard-manifest-{mode}"));
        common::GuardLock::declaring(mode).write(&root);

        let (code, stderr) = common::run_guard(
            &root,
            &common::guard_write_payload_with_content(
                ".mcp.json",
                r#"{"mcpServers":{"gmail":{"type":"carrier-pigeon","command":"npx"}}}"#,
            ),
        );
        assert_eq!(code, Some(0), "`{mode}` allows the write, never blocks");
        assert_eq!(
            !stderr.is_empty(),
            expect_stderr,
            "`{mode}` in-band surfacing mismatch: {stderr}"
        );
    }
}

#[test]
fn guard_flags_manifest_write_that_omits_lock_declared_member() {
    // A manifest write that omits a member the lock declares is flagged at the declared
    // enforcement mode. This regression test ensures the guard checks the lock's expected
    // member roster, not just the members present in the pending write.
    let root = common::tmpdir("guard-manifest-dropped-member-block");

    // A lock with `block` mode declaring an MCP server and a hook member.
    common::GuardLock::declaring("block")
        .registration("mcp-server", "gmail", ".mcp.json", "mcpServers.*")
        .registration("hook", "SessionStart", "settings.json", "hooks.<Event>")
        .write(&root);

    // A write to `.mcp.json` that omits the `gmail` server (empty mcpServers object)
    // must be flagged, even though what's there parses correctly.
    let (code, stderr) = common::run_guard(
        &root,
        &common::guard_write_payload_with_content(".mcp.json", r#"{"mcpServers":{}}"#),
    );
    assert_eq!(
        code,
        Some(2),
        "a manifest write omitting a lock-declared member must be blocked in `block` mode"
    );
    assert!(
        stderr.contains("lock declares member"),
        "the finding must reference the lock declaration: {stderr}"
    );
    assert!(
        stderr.contains("gmail"),
        "the finding must name the missing member: {stderr}"
    );

    // Verify that under `warn` mode, the same write is allowed but surfaces the finding.
    let warn_root = common::tmpdir("guard-manifest-dropped-member-warn");
    common::GuardLock::declaring("warn")
        .registration("mcp-server", "gmail", ".mcp.json", "mcpServers.*")
        .write(&warn_root);

    let (warn_code, warn_output) = common::run_guard(
        &warn_root,
        &common::guard_write_payload_with_content(".mcp.json", r#"{"mcpServers":{}}"#),
    );
    assert_eq!(
        warn_code,
        Some(0),
        "warn mode allows the write but surfaces the finding"
    );
    assert!(
        common::guard_in_band(&warn_output, "PreToolUse").contains("lock declares member"),
        "the warning must be in-band"
    );
}

/// A `block` harness whose lock declares the `SessionStart` hook member — which makes
/// `.claude/settings.json` both a represented manifest and an emit-owned projection, the
/// exact overlap an `Edit` to it has to be judged under.
fn settings_manifest_harness(slug: &str, settings: &str) -> PathBuf {
    let root = common::tmpdir(slug);
    settings_manifest_lock("block").write(&root);
    common::write_settings(&root, settings);
    root
}

/// A `.claude/settings.json` carrying the lock-declared `SessionStart` hook beside residue
/// no kind models (`autoMemoryEnabled`) — the co-owned shape the manifest binding exists for.
const CO_OWNED_SETTINGS: &str = r#"{
  "autoMemoryEnabled": false,
  "hooks": {
    "SessionStart": [
      { "hooks": [ { "type": "command", "command": "temper check . --reporter session-start" } ] }
    ]
  }
}
"#;

#[test]
fn guard_allows_an_edit_touching_only_unmodeled_manifest_residue() {
    // `.claude/settings.json` is co-owned: the lock governs its `hooks` segment, and its
    // `autoMemoryEnabled`/`permissions` residue is the author's. An `Edit` touching only that
    // residue is legitimate — the guard reconstructs the manifest the edit would land, finds
    // every governed member intact and conforming, and passes it, never falling through to
    // the blanket `.claude/`-is-projected denial the same path earns for a projection write.
    let root = settings_manifest_harness("guard-manifest-edit-residue", CO_OWNED_SETTINGS);

    let (code, stderr) = common::run_guard(
        &root,
        &common::guard_edit_payload(
            ".claude/settings.json",
            "\"autoMemoryEnabled\": false",
            "\"autoMemoryEnabled\": true",
        ),
    );
    assert_eq!(
        code,
        Some(0),
        "an edit touching only unmodeled residue passes even under `block`: {stderr}"
    );
    assert!(
        stderr.is_empty(),
        "a conforming edit surfaces nothing: {stderr}"
    );

    // The same reconstruction judges an edit that *drops* a governed member: replacing the
    // `hooks` segment wholesale denies, at the manifest's own wording — one rule for `Write`
    // and `Edit` alike.
    let (dropped_code, dropped_stderr) = common::run_guard(
        &root,
        &common::guard_edit_payload(
            ".claude/settings.json",
            "\"SessionStart\"",
            "\"NotARealEvent\"",
        ),
    );
    assert_eq!(
        dropped_code,
        Some(2),
        "an edit dropping a lock-declared member is denied"
    );
    assert!(
        dropped_stderr.contains("lock declares member") && dropped_stderr.contains("SessionStart"),
        "the denial names the dropped member: {dropped_stderr}"
    );
}

/// A `block` harness whose lock declares the `settings` **container** member that projects
/// `.claude/settings.json` whole: the kind row naming the container (a locus equal to the
/// manifest's own path, keying no collection of its own) and the provenance row that member's
/// projection rides. The `SessionStart` hook registration keys inside the same file, so one
/// lock carries exactly the overlap a container-owned manifest presents to the guard.
fn container_owned_settings_harness(slug: &str) -> PathBuf {
    let root = common::tmpdir(slug);
    settings_manifest_lock("block")
        .member(
            "settings",
            "settings",
            ".claude/settings.json",
            "abc",
            "abc",
        )
        .kind_row(KindFactRow {
            format: Some("json-document".to_string()),
            unit_shape: Some("file".to_string()),
            ..common::kind_facts("settings", ".claude", "settings.json")
        })
        .write(&root);
    common::write_settings(&root, CO_OWNED_SETTINGS);
    root
}

/// Every other `fresh_clause` case in this suite weighs the judge's verdict over bytes
/// `install` wrote. This one pins the commitment that keeps that reading honest: the
/// guard's enforcement mode is a separate decision, untouched by the clause.
#[test]
fn the_guards_enforcement_mode_decides_a_guarded_write_whatever_the_fresh_clause_binds() {
    // Two verdicts over one path, and only one of them is the clause's. `check`'s
    // projection-drift finding is the root `fresh` clause's to weigh — declare no such
    // clause and the finding is gone. The *guard*'s refusal is the enforcement mode's, at
    // the boundary, before any byte lands: it binds the path because the lock names it a
    // projection, and no contract clause enters that decision. A harness that opted out of
    // the freshness finding has not opted out of the boundary.
    let root = common::tmpdir("guard-mode-outlives-a-clauseless-fresh");
    // A root contract binding `reachable` alone: the lock declares kind-less rows, so
    // these rows *are* the root's whole contract and no `fresh` clause is composed.
    claude_write_lock("block")
        .clause_row(ClauseRow {
            label: Some("root.reachable".to_string()),
            ..common::clause("reachable", "advisory")
        })
        .write(&root);
    // The projection on disk, drifted from the `emit_hash` the row above records — so the
    // only reason `check` can stay silent is the absent clause.
    let skill = root.join(".claude").join("skills").join("x");
    fs::create_dir_all(&skill).unwrap();
    fs::write(skill.join("SKILL.md"), SKILL).unwrap();

    let (findings, _ok) = common::check_harness(&root);
    assert!(
        common::findings_for(&findings, "root.fresh").is_empty(),
        "with no `fresh` clause bound, the drifted projection draws no freshness finding: \
         {findings:#?}"
    );

    let (code, stderr) = common::run_guard(&root, &common::guard_write_payload(CLAUDE_WRITE_PATH));
    assert_eq!(
        code,
        Some(2),
        "the guard still denies the write at the lock's `block` mode: {stderr}"
    );
    assert!(
        stderr.contains("temper-managed projection"),
        "and speaks as the projection binding, which reads the lock's rows and no clause: \
         {stderr}"
    );
}

#[test]
fn guard_refuses_a_residue_only_write_to_a_container_owned_manifest() {
    // The co-ownership allowance reverses where a container member projects the manifest
    // whole. Every byte is then emit's — the part-authored file co-ownership assumes cannot
    // arise — so there is no opaque residue a hand write may touch, and `check`'s root
    // `fresh` clause gives these very bytes a projection-drift verdict. A boundary that
    // waved the edit through would
    // contradict the gate, so the write earns the projection refusal instead.
    let root = container_owned_settings_harness("guard-manifest-container-owned");
    let pending = CO_OWNED_SETTINGS.replace(": false", ": true");

    let (code, stderr) = common::run_guard(
        &root,
        &common::guard_write_payload_with_content(".claude/settings.json", &pending),
    );
    assert_eq!(
        code,
        Some(2),
        "a `block` harness denies a residue-only write to a container-owned manifest: {stderr}"
    );
    assert!(
        stderr.contains("temper-managed projection"),
        "the refusal speaks as the projection binding, not the co-owned one: {stderr}"
    );
    assert!(
        !stderr.contains("temper-governed manifest"),
        "a container-owned manifest never earns the co-owned wording: {stderr}"
    );
    assert!(
        stderr.contains("`settings` member `settings`") && stderr.contains(".claude/settings.json"),
        "the refusal names the member that owns the bytes: {stderr}"
    );

    // The control that isolates the container as the discriminator: same path, same pending
    // bytes, same `block` mode, same hook registration — and `.claude/settings.json` is an
    // emit-owned target in both locks. Only the container rows are gone, and the co-owned
    // verdict survives.
    let co_owned = settings_manifest_harness("guard-manifest-container-absent", CO_OWNED_SETTINGS);
    let (ok_code, ok_stderr) = common::run_guard(
        &co_owned,
        &common::guard_write_payload_with_content(".claude/settings.json", &pending),
    );
    assert_eq!(
        ok_code,
        Some(0),
        "with no container member the residue-only write still conforms: {ok_stderr}"
    );
    assert!(
        ok_stderr.is_empty(),
        "a conforming co-owned write surfaces nothing: {ok_stderr}"
    );

    // And the same discriminator read off a second manifest, to show it is the **lock's**
    // rows and not the kind vocabulary that decides: the `mcp` container kind ships embedded
    // and could project `.mcp.json` whole, but this lock declares no member of it, so the
    // file is composed from registrations alone here and stays co-owned.
    let (mcp_code, mcp_stderr) = common::run_guard(
        &co_owned,
        &common::guard_write_payload_with_content(
            ".mcp.json",
            r#"{"mcpServers":{"gmail":{"type":"stdio","command":"npx"}},"someResidue":true}"#,
        ),
    );
    assert_eq!(
        mcp_code,
        Some(0),
        "a residue-carrying write to a container-less manifest conforms: {mcp_stderr}"
    );
    assert!(
        mcp_stderr.is_empty(),
        "a conforming co-owned write surfaces nothing: {mcp_stderr}"
    );
}

#[test]
fn guard_denies_an_unreconstructable_manifest_edit_with_the_manifest_message() {
    // An edit whose `old_string` is not on disk cannot be honestly applied, so no member was
    // checked. That is a denial — but a manifest-specific one, naming the governed members
    // and the write shape the guard can read. The blanket projection wording would tell the
    // author to re-emit a file they co-own, which is false.
    let root =
        settings_manifest_harness("guard-manifest-edit-unreconstructable", CO_OWNED_SETTINGS);

    let (code, stderr) = common::run_guard(
        &root,
        &common::guard_edit_payload(
            ".claude/settings.json",
            "\"neverOnDisk\": 1",
            "\"neverOnDisk\": 2",
        ),
    );
    assert_eq!(code, Some(2), "an unreconstructable edit is denied");
    assert!(
        stderr.contains("temper-governed manifest"),
        "the denial speaks as the manifest binding: {stderr}"
    );
    assert!(
        !stderr.contains("temper-managed projection"),
        "a co-owned manifest never earns the blanket projection denial: {stderr}"
    );
    assert!(
        stderr.contains("SessionStart") && stderr.contains("Write"),
        "the denial names the governed members and points at a whole-file write: {stderr}"
    );

    // An absent file is unreconstructable for the same reason, and denies the same way.
    let bare = common::tmpdir("guard-manifest-edit-absent-file");
    settings_manifest_lock("block").write(&bare);

    let (absent_code, absent_stderr) = common::run_guard(
        &bare,
        &common::guard_edit_payload(".claude/settings.json", "\"a\": 1", "\"a\": 2"),
    );
    assert_eq!(
        absent_code,
        Some(2),
        "an edit to an unreadable manifest is denied"
    );
    assert!(
        absent_stderr.contains("temper-governed manifest")
            && !absent_stderr.contains("temper-managed projection"),
        "an unreadable manifest still speaks as the manifest binding: {absent_stderr}"
    );
}

/// A lock at `mode` declaring the `SessionStart` hook registration — the manifest
/// binding's verdict is a function of the mode alone, so one builder serves the `block`
/// harness above and the unparseable case's three arms alike.
fn settings_manifest_lock(mode: &str) -> common::GuardLock {
    common::GuardLock::declaring(mode).registration(
        "hook",
        "SessionStart",
        "settings.json",
        "hooks.<Event>",
    )
}

/// The pending content that reproduces the reported failure: a `.claude/settings.json`
/// that is decidably not JSON.
const UNPARSEABLE_SETTINGS: &str = "{ not json";

#[test]
fn guard_refuses_a_write_that_would_leave_a_represented_manifest_unparseable() {
    // The write the guard used to wave through: the bytes reconstruct in full and are not a
    // manifest. Deferring it to a later placement assumed one exists — for this file class
    // none does. An unparseable `.claude/settings.json` makes the harness unloadable, so the
    // next `check` aborts before any reporter runs, and the `SessionStart` hook that would
    // have carried the verdict is declared in the file that no longer parses. The boundary
    // is the only placement left.
    let root = settings_manifest_harness("guard-manifest-unparseable-block", CO_OWNED_SETTINGS);

    let (code, stderr) = common::run_guard(
        &root,
        &common::guard_write_payload_with_content(".claude/settings.json", UNPARSEABLE_SETTINGS),
    );
    assert_eq!(
        code,
        Some(2),
        "a `block` harness denies a write that would leave the manifest unparseable: {stderr}"
    );
    assert!(
        stderr.contains("guard.manifest-unparseable"),
        "the denial carries its own rule id: {stderr}"
    );
    assert!(
        stderr.contains(".claude/settings.json") && stderr.contains("line 1"),
        "the finding names the manifest path and the parse fault: {stderr}"
    );
    assert!(
        !stderr.contains("temper-managed projection"),
        "a co-owned manifest never earns the blanket projection denial: {stderr}"
    );
    assert_eq!(
        stderr.matches("guard.manifest-unparseable").count(),
        1,
        "the three collection addresses sharing settings.json report one finding, not three: {stderr}"
    );
}

#[test]
fn guard_follows_the_declared_mode_for_an_unparseable_manifest_write() {
    // The refusal is a finding like any other: it rides main.rs's existing three-valued
    // dispatch rather than escalating past the mode the lock declares. `warn` surfaces it
    // in-band and allows the write; `note` allows it with no in-band message at all.
    for (mode, expect_stderr) in [("warn", true), ("note", false)] {
        let root = common::tmpdir(&format!("guard-manifest-unparseable-{mode}"));
        settings_manifest_lock(mode).write(&root);
        common::write_settings(&root, CO_OWNED_SETTINGS);

        let (code, stderr) = common::run_guard(
            &root,
            &common::guard_write_payload_with_content(
                ".claude/settings.json",
                UNPARSEABLE_SETTINGS,
            ),
        );
        assert_eq!(code, Some(0), "`{mode}` allows the write, never blocks");
        assert_eq!(
            stderr.contains("guard.manifest-unparseable"),
            expect_stderr,
            "`{mode}` in-band surfacing mismatch: {stderr}"
        );
    }
}

#[test]
fn guard_reserves_the_unparseable_refusal_for_content_that_is_not_a_manifest() {
    // The new refusal is decided on the reconstructed bytes alone, so it must not swallow
    // either neighbour: a write landing a well-formed manifest still passes silently, and an
    // edit the guard cannot reconstruct still earns the reconstruction rule — nothing was
    // reconstructed there, so nothing can be said about whether it parses.
    let root =
        settings_manifest_harness("guard-manifest-unparseable-neighbours", CO_OWNED_SETTINGS);

    let (ok_code, ok_stderr) = common::run_guard(
        &root,
        &common::guard_write_payload_with_content(".claude/settings.json", CO_OWNED_SETTINGS),
    );
    assert_eq!(
        ok_code,
        Some(0),
        "a parseable, conforming write is still allowed: {ok_stderr}"
    );
    assert!(
        ok_stderr.is_empty(),
        "and still surfaces nothing: {ok_stderr}"
    );

    let (edit_code, edit_stderr) = common::run_guard(
        &root,
        &common::guard_edit_payload(
            ".claude/settings.json",
            "\"neverOnDisk\": 1",
            "\"neverOnDisk\": 2",
        ),
    );
    assert_eq!(
        edit_code,
        Some(2),
        "an unreconstructable edit is still denied"
    );
    assert!(
        edit_stderr.contains("guard.manifest-edit-unreconstructable")
            && !edit_stderr.contains("guard.manifest-unparseable"),
        "an edit that reconstructs nothing keeps its own rule: {edit_stderr}"
    );
}

// ---------------------------------------------------------------------------
// emit's own note/modeline discipline — unrelated to install, still exercised
// directly over a hand-built payload.
// ---------------------------------------------------------------------------

fn skill_rule_kind_facts() -> Vec<temper::drift::KindFactRow> {
    vec![
        common::rule_kind_facts(None, &[]),
        common::skill_kind_facts(None, &[]),
    ]
}

fn payload_from_harness(harness: &Path) -> temper::drift::Payload {
    let skill_kind = temper::builtin_kind::definition("skill").unwrap();
    let rule_kind = temper::builtin_kind::definition("rule").unwrap();

    let skill_path = harness
        .join(".claude")
        .join("skills")
        .join("coordinate")
        .join("SKILL.md");
    let skill = temper::frontmatter::Member::from_source(&skill_kind, &skill_path).unwrap();
    let mut members = vec![temper::drift::PayloadMember {
        kind: "skill".to_string(),
        name: skill.id.clone(),
        host: None,
        location: None,
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
            host: None,
            location: None,
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
    // The hand-built payload is a narrower program than install's: it composes no
    // typecheck tool, so after an install it drops the root's embedded layer, on purpose.
    temper::drift::emit(
        &payload,
        &into,
        temper::drift::EmitOptions {
            dry_run: false,
            frozen: false,
            teardown: true,
        },
    )
    .unwrap();
}

#[test]
fn emit_stamps_the_managed_by_note_but_never_the_schema_modeline() {
    // The ownership split, pinned from install's side: emit writes a projection's bytes
    // whole, note included, so a fresh emit needs no install run to converge it. The
    // yaml-language-server modeline stays install's — it names a schema artifact that
    // may not exist yet, so only install can decide it is safe to point at.
    let harness = write_harness("emit-note", false);
    emit_from_harness(&harness);

    for rel in [
        PathBuf::from(".claude")
            .join("skills")
            .join("coordinate")
            .join("SKILL.md"),
        PathBuf::from(".claude").join("rules").join("rust.md"),
    ] {
        let projected = fs::read_to_string(harness.join(&rel)).unwrap();
        assert!(
            projected.starts_with(&format!("---\n{}\n", temper::placement::NOTE_COMMENT)),
            "{}: emit stamps the note as the leading frontmatter line, got: {projected}",
            rel.display()
        );
        assert!(!projected.contains(temper::placement::MODELINE_MARKER));
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

/// Run `temper install` over `root`, answering the one question `n`, and return stdout.
fn install_prompted(root: &Path) -> String {
    use std::io::Write as _;
    let mut child = Command::new(BIN)
        .arg("install")
        .arg(root)
        .arg("--dry-run")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(b"n\n").unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn the_install_discovery_report_shows_a_missing_skill_description_before_the_represent_question() {
    let root = common::tmpdir("cli-report-contract");
    let skill = root.join(".claude").join("skills").join("undescribed");
    fs::create_dir_all(&skill).unwrap();
    fs::write(
        skill.join("SKILL.md"),
        "---\nname: undescribed\n---\nBody.\n",
    )
    .unwrap();

    let stdout = install_prompted(&root);
    let finding = stdout
        .find("skill.required.description")
        .unwrap_or_else(|| panic!("no description finding in the report:\n{stdout}"));
    let question = stdout.find(install::REPRESENT_QUESTION).unwrap();
    assert!(finding < question, "finding after the question:\n{stdout}");
}

#[test]
fn the_install_discovery_report_prints_a_count_per_kind() {
    let root = write_harness("cli-report-counts", false);
    let stdout = install_prompted(&root);
    let question = stdout.find(install::REPRESENT_QUESTION).unwrap();
    let report = &stdout[..question];
    let count = |kind: &str| {
        report
            .lines()
            .find_map(|line| {
                let mut words = line.split_whitespace();
                (words.next() == Some(kind)).then(|| words.next().map(str::to_owned))
            })
            .flatten()
    };
    assert_eq!(count("skill").as_deref(), Some("1"), "{report}");
    assert_eq!(count("rule").as_deref(), Some("2"), "{report}");
}

#[test]
fn the_cli_install_verb_represents_on_yes_and_dry_runs_a_re_represent() {
    let root = write_harness("cli-yes", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    let status = Command::new(BIN)
        .arg("install")
        .arg(&root)
        .arg("--yes")
        .status()
        .unwrap();
    assert!(status.success());
    assert!(temper_dir.join("harness.ts").is_file());
    assert!(temper_dir.join("lock.toml").is_file());

    let before = common::tree_bytes(&root);
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
        common::tree_bytes(&root),
        "a re-represent dry run writes nothing"
    );
}

// ---------------------------------------------------------------------------
// the lock on disk resolves install's path argument
// ---------------------------------------------------------------------------

/// A represented root: the lift run for real, so `.temper/lock.toml` is a true emit
/// product rather than a fixture stand-in.
fn represent_for_real(label: &str) -> PathBuf {
    let root = write_harness(label, false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);
    let status = Command::new(BIN)
        .arg("install")
        .arg(&root)
        .arg("--yes")
        .status()
        .unwrap();
    assert!(status.success());
    assert!(temper_dir.join("lock.toml").is_file());
    root
}

#[test]
fn a_represented_root_converges_on_its_lock_without_re_asking_the_question() {
    let root = represent_for_real("cli-settled");

    // No flag, and no tty behind stdin — the exact shape whose conservative `No`
    // default would place the session-start hook alone against a root whose lock
    // already justifies the full placement set.
    let output = Command::new(BIN)
        .arg("install")
        .arg(&root)
        .arg("--dry-run")
        .stdin(std::process::Stdio::null())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(
        !stdout.contains(install::REPRESENT_QUESTION),
        "the lock has already answered the question: {stdout}"
    );
    assert!(
        stdout.contains("already represented"),
        "a skipped question is visible in the report: {stdout}"
    );
    assert!(
        stdout.contains("represented — ") && !stdout.contains("not represented"),
        "the represented path is taken, not the unattended default: {stdout}"
    );
    // The placements the lock justifies — the guard and the per-artifact notes the
    // `No` default would have left unplaced.
    assert!(stdout.contains("guard hook"), "{stdout}");
    assert!(stdout.contains("managed-by note"), "{stdout}");
}

#[test]
fn no_represent_against_a_represented_root_refuses_loud() {
    let root = write_harness("cli-settled-denied", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(temper_dir.join("lock.toml"), "").unwrap();

    let output = Command::new(BIN)
        .arg("install")
        .arg(&root)
        .arg("--no-represent")
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "asserting the false half of a settled fork is a usage error"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("lock.toml") && stderr.contains("--no-represent"),
        "the refusal names the lock that settled the fork: {stderr}"
    );
}

#[test]
fn no_represent_against_a_program_that_has_never_been_emitted_refuses_loud() {
    // A harness carrying an authored program and no lock is represented — never emitted.
    // Reading the fork off the lock alone let this shape answer No, and the unrepresented
    // writer then spliced the `SessionStart` row into a file the program owns whole: install
    // re-adding a gate hook to a represented harness, which 0073 forbids outright.
    let root = write_harness("cli-program-no-lock", true);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(temper_dir.join("harness.ts"), "export default {};\n").unwrap();
    assert!(!temper_dir.join("lock.toml").exists());

    let settings_path = root.join(".claude").join("settings.json");
    let before = fs::read_to_string(&settings_path).unwrap();

    let output = Command::new(BIN)
        .arg("install")
        .arg(&root)
        .arg("--no-represent")
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "asserting the false half of a settled fork is a usage error"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("harness.ts") && stderr.contains("--no-represent"),
        "the refusal names the program that settled the fork: {stderr}"
    );
    assert_eq!(
        fs::read_to_string(&settings_path).unwrap(),
        before,
        "and nothing is spliced into the projection the program owns"
    );
}

#[test]
fn a_workspace_passed_as_the_path_refuses_instead_of_scaffolding_inside_it() {
    let root = write_harness("cli-workspace-arg", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(temper_dir.join("lock.toml"), "").unwrap();

    let output = Command::new(BIN)
        .arg("install")
        .arg(&temper_dir)
        .arg("--no-represent")
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "a workspace is not a harness root install can be aimed at"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains(&root.display().to_string()),
        "the refusal names the enclosing root the argument meant: {stderr}"
    );
    assert!(
        !temper_dir.join(".claude").exists(),
        "nothing is scaffolded inside the workspace"
    );
}

#[test]
fn package_json_written_when_dependency_resolves_via_ancestor() {
    let root = write_harness("pkg-json-ancestor", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();

    // Vendor the SDK into the root's node_modules instead of .temper's, so
    // dependency_resolves finds it via ancestor walk-up and npm install is
    // skipped — but .temper/package.json should still be written.
    common::vendor_sdk(&root.join("node_modules").join("@dtmd"));
    // The compiler is the workspace's own: the typecheck runs its anchored `.bin`.
    let bin = temper_dir.join("node_modules").join(".bin");
    fs::create_dir_all(&bin).unwrap();
    common::write_stub_compiler(&bin.join(if cfg!(windows) { "tsc.cmd" } else { "tsc" }));

    let discovery = install::discover(&root).unwrap();
    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();

    assert!(outcome.represented);
    let package_json_path = temper_dir.join("package.json");
    assert!(
        package_json_path.is_file(),
        ".temper/package.json must exist even when SDK resolves via ancestor"
    );

    let package_json = fs::read_to_string(&package_json_path).unwrap();
    let parsed: serde_json::Value =
        serde_json::from_str(&package_json).expect(".temper/package.json must be valid JSON");

    assert_eq!(
        parsed["type"], "module",
        ".temper/package.json must declare type: module"
    );
    assert!(
        parsed["dependencies"]["@dtmd/temper"].is_string(),
        ".temper/package.json must declare @dtmd/temper dependency"
    );
}

#[test]
fn an_existing_package_json_stays_byte_identical_on_re_install() {
    let root = write_harness("pkg-json-byte-identity", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    // Unsorted keys, a field temper never writes, two-column indent, no trailing
    // newline, and neither typecheck devDependency — any serializer round-trip
    // would move at least one of these bytes.
    let authored = "{\n  \"type\": \"module\",\n  \"name\": \"my-harness\",\n  \"private\": true,\n  \"packageManager\": \"pnpm@9.0.0\",\n  \"dependencies\": { \"@dtmd/temper\": \"*\" }\n}";
    let manifest = temper_dir.join("package.json");
    fs::write(&manifest, authored).unwrap();

    let discovery = install::discover(&root).unwrap();
    let first = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(fs::read_to_string(&manifest).unwrap(), authored);
    let missing: Vec<&str> = first
        .missing_dependencies
        .iter()
        .map(|dep| dep.package.as_str())
        .collect();
    assert_eq!(missing, ["typescript", "@types/node"]);
    assert!(first.missing_dependencies.iter().all(|dep| dep.dev));
    let rendered = install::render(&first);
    assert!(
        rendered.contains("lacks `typescript@") && rendered.contains("lacks `@types/node@"),
        "the missing dependencies are named for the author's manager: {rendered}"
    );

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(
        fs::read_to_string(&manifest).unwrap(),
        authored,
        "re-running install never rewrites the program's package.json"
    );
}

#[test]
fn a_fresh_install_writes_the_package_manager_field_into_the_scaffold() {
    let root = write_harness("pkg-json-manager-field", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();

    let version = Command::new(install::npm_program())
        .arg("--version")
        .output()
        .unwrap();
    let expected = format!("npm@{}", String::from_utf8_lossy(&version.stdout).trim());
    let parsed: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(temper_dir.join("package.json")).unwrap())
            .unwrap();
    assert_eq!(
        parsed["packageManager"], *expected,
        "with no root declaration the scaffold names the npm install ran, at its version"
    );
}

#[test]
fn a_program_package_json_with_no_package_manager_field_refuses_naming_the_remedy() {
    let root = write_harness("pkg-json-no-manager", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);
    let authored = "{ \"type\": \"module\", \"dependencies\": { \"@dtmd/temper\": \"*\" } }\n";
    let manifest = temper_dir.join("package.json");
    fs::write(&manifest, authored).unwrap();

    let discovery = install::discover(&root).unwrap();
    let err = install::run(&root, &discovery, Represent::Yes, false).unwrap_err();
    let rendered = format!("{err:?}");
    assert!(
        rendered.contains("declares no `packageManager`")
            && rendered.contains("add `\"packageManager\": \"npm@<version>\"`"),
        "the refusal names the field and the remedy: {rendered}"
    );
    assert_eq!(
        fs::read_to_string(&manifest).unwrap(),
        authored,
        "the refusal never writes the author's manifest"
    );
    assert!(
        !temper_dir.join("harness.ts").exists(),
        "the refusal lands before the lift"
    );
}

#[test]
fn a_first_install_scaffolds_the_package_json() {
    let root = write_harness("pkg-json-first-install", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);
    let manifest = temper_dir.join("package.json");
    assert!(!manifest.exists());

    let discovery = install::discover(&root).unwrap();
    let outcome = install::run(&root, &discovery, Represent::Yes, false).unwrap();

    let parsed: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&manifest).unwrap()).unwrap();
    assert_eq!(parsed["type"], "module");
    assert!(parsed["dependencies"]["@dtmd/temper"].is_string());
    for package in ["typescript", "@types/node"] {
        assert!(parsed["devDependencies"][package].is_string(), "{parsed}");
    }
    assert!(outcome.missing_dependencies.is_empty());
}

// ---------------------------------------------------------------------------
// Fail-loud invariant: hooks fail loud when temper is not on PATH
// ---------------------------------------------------------------------------

#[test]
fn session_start_hook_fails_loud_when_temper_not_on_path() {
    // Run the SESSION_START_COMMAND with temper not on PATH to verify it fails
    // with a clear message naming temper as the missing binary.
    let tmpdir = tempfile::Builder::new()
        .prefix("hook-missing-temper-session-start")
        .tempdir()
        .unwrap();
    let harness = tmpdir.path().to_path_buf();
    let _ = tmpdir.keep();

    // Create a minimal lock so the command can run.
    let temper_dir = harness.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(temper_dir.join("lock.toml"), "").unwrap();

    // Run the command through a shell with an empty PATH so temper cannot be found.
    // Use the command that checks for temper on PATH, but remove temper from PATH.
    let cmd = temper::install::SESSION_START_COMMAND;
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(cmd)
        .current_dir(&harness)
        .env_clear()
        .output()
        .unwrap();

    // The command must fail (non-zero exit) when temper is not on PATH.
    assert!(
        !output.status.success(),
        "session-start must fail when temper is not on PATH"
    );

    // The error message must name temper as the missing binary.
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("temper: command not found"),
        "stderr must contain 'temper: command not found' to name the missing binary, got:\n{stderr}"
    );
}

/// Convert a file's content to CRLF line endings.
fn convert_to_crlf(path: &Path) {
    let content = fs::read_to_string(path).unwrap();
    let crlf = content.replace("\r\n", "\n").replace('\n', "\r\n");
    fs::write(path, crlf).unwrap();
}

/// Recursively convert all markdown and TypeScript files to CRLF line endings,
/// but skip the `.temper` directory (which may be scaffolded after conversion).
fn convert_tree_to_crlf(root: &Path) {
    for entry in walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        // Skip .temper directory — it's scaffolded after conversion.
        if path.components().any(|c| {
            if let std::path::Component::Normal(os_str) = c {
                os_str == ".temper"
            } else {
                false
            }
        }) {
            continue;
        }
        if entry.file_type().is_file()
            && path
                .extension()
                .is_some_and(|ext| ext == "md" || ext == "json")
        {
            convert_to_crlf(path);
        }
    }
}

#[test]
fn crlf_checkouts_preserve_managed_projections_through_install_emit_cycle() {
    // A represented harness with CRLF line endings must have its managed-by
    // notes preserved across install and emit, with no drift reported.
    let root = write_harness("crlf-preserve", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);

    // Convert all projections to CRLF.
    convert_tree_to_crlf(&root);

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();

    // gate_installed must be quiet (no stale config drift).
    assert!(
        temper::drift::config_stale(&temper_dir, &fresh_clause()).is_empty(),
        "gate_installed must be quiet on CRLF projections after install"
    );

    // Verify each frontmatter projection still carries its managed-by note.
    let skill_md = fs::read_to_string(
        root.join(".claude")
            .join("skills")
            .join("coordinate")
            .join("SKILL.md"),
    )
    .unwrap();
    assert!(
        skill_md.contains(temper::placement::NOTE_MARKER),
        "skill projection must retain managed-by note on CRLF"
    );

    let rule_md = fs::read_to_string(root.join(".claude").join("rules").join("rust.md")).unwrap();
    assert!(
        rule_md.contains(temper::placement::NOTE_MARKER),
        "rule projection must retain managed-by note on CRLF"
    );

    // The frontmatterless rule should have the banner instead.
    let collab_md =
        fs::read_to_string(root.join(".claude").join("rules").join("collaboration.md")).unwrap();
    assert!(
        collab_md.contains(temper::placement::BANNER_MARKER),
        "frontmatterless rule must have banner on CRLF"
    );

    // Re-install must converge (no changes).
    let second_install = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(
        second_install.scaffolded, 0,
        "CRLF re-install must not re-scaffold"
    );
    assert_eq!(
        outcome_of(&second_install, temper::install::Placement::GuardHook),
        ApplyOutcome::Unchanged
    );
}

#[test]
fn crlf_lf_twins_both_converge_on_install() {
    // Both LF and CRLF checkouts of the same harness must converge identically:
    // the same managed notes inserted, gate clean, and re-install idempotent.
    // This pins that the delimiter handling is format-agnostic.

    // Create and install an LF harness.
    let lf_root = write_harness("crlf-lf-twin-lf", false);
    let lf_temper_dir = lf_root.join(".temper");
    fs::create_dir_all(&lf_temper_dir).unwrap();
    common::vendor_install_workspace(&lf_temper_dir);

    let lf_discovery = install::discover(&lf_root).unwrap();
    install::run(&lf_root, &lf_discovery, Represent::Yes, false).unwrap();

    // Read the LF result.
    let lf_skill = fs::read_to_string(
        lf_root
            .join(".claude")
            .join("skills")
            .join("coordinate")
            .join("SKILL.md"),
    )
    .unwrap();

    // Create and install a CRLF harness (same structure, converted to CRLF).
    let crlf_root = write_harness("crlf-lf-twin-crlf", false);
    convert_tree_to_crlf(&crlf_root);

    let crlf_temper_dir = crlf_root.join(".temper");
    fs::create_dir_all(&crlf_temper_dir).unwrap();
    common::vendor_install_workspace(&crlf_temper_dir);

    let crlf_discovery = install::discover(&crlf_root).unwrap();
    install::run(&crlf_root, &crlf_discovery, Represent::Yes, false).unwrap();

    // Read the CRLF result.
    let crlf_skill = fs::read_to_string(
        crlf_root
            .join(".claude")
            .join("skills")
            .join("coordinate")
            .join("SKILL.md"),
    )
    .unwrap();

    // Normalize line endings for comparison (the projection content must be identical).
    let lf_normalized = lf_skill.replace("\r\n", "\n");
    let crlf_normalized = crlf_skill.replace("\r\n", "\n");
    assert_eq!(
        lf_normalized, crlf_normalized,
        "LF and CRLF harnesses must project identically"
    );

    // Both must report gate clean.
    assert!(
        temper::drift::config_stale(&lf_temper_dir, &fresh_clause()).is_empty(),
        "LF harness gate must be clean"
    );
    assert!(
        temper::drift::config_stale(&crlf_temper_dir, &fresh_clause()).is_empty(),
        "CRLF harness gate must be clean"
    );
}

#[test]
fn install_then_edit_then_standalone_emit_preserves_managed_projections_at_both_eols() {
    // The verb-composition regression: install::run, then an authored module edit,
    // then a standalone emit_from_harness (no second install::run). The projection
    // must carry the authored change and survive the managed-by note/modeline;
    // the lock must converge with no drift reported. Pinned at both LF and CRLF.

    // Ensure the SDK is built once before both test cases run.
    common::ensure_sdk_built();

    let test_lf = |name: &str| {
        let root = write_harness(name, false);
        let temper_dir = root.join(".temper");
        fs::create_dir_all(&temper_dir).unwrap();
        common::vendor_install_workspace(&temper_dir);

        // First install::run.
        let discovery = install::discover(&root).unwrap();
        install::run(&root, &discovery, Represent::Yes, false).unwrap();

        // Verify managed-by note is present after install.
        let skill_path = root
            .join(".claude")
            .join("skills")
            .join("coordinate")
            .join("SKILL.md");
        let skill_after_install = fs::read_to_string(&skill_path).unwrap();
        assert!(
            skill_after_install.contains(temper::placement::NOTE_MARKER),
            "skill must have managed-by note after install (LF)"
        );

        // Author edits the skill's body (simulate a module edit).
        let edited_body = skill_after_install.replace(
            "Drive the team through the playbook.",
            "Drive the team through the playbook.\n\nUpdated by author.",
        );
        fs::write(&skill_path, edited_body).unwrap();

        // Standalone emit_from_harness without second install::run.
        emit_from_harness(&root);

        // Verify managed-by note and modeline survive.
        let skill_after_emit = fs::read_to_string(&skill_path).unwrap();
        assert!(
            skill_after_emit.contains(temper::placement::NOTE_MARKER),
            "skill must retain managed-by note after standalone emit (LF)"
        );
        assert!(
            skill_after_emit.contains("Updated by author."),
            "skill must carry the authored edit after emit (LF)"
        );

        // Gate must be quiet (no drift).
        assert!(
            temper::drift::config_stale(&temper_dir, &fresh_clause()).is_empty(),
            "gate must report no drift after install→edit→emit (LF)"
        );
    };

    let test_crlf = |name: &str| {
        let root = write_harness(name, false);
        convert_tree_to_crlf(&root);

        let temper_dir = root.join(".temper");
        fs::create_dir_all(&temper_dir).unwrap();
        common::vendor_install_workspace(&temper_dir);

        // First install::run.
        let discovery = install::discover(&root).unwrap();
        install::run(&root, &discovery, Represent::Yes, false).unwrap();

        // Verify managed-by note is present after install.
        let skill_path = root
            .join(".claude")
            .join("skills")
            .join("coordinate")
            .join("SKILL.md");
        let skill_after_install = fs::read_to_string(&skill_path).unwrap();
        assert!(
            skill_after_install.contains(temper::placement::NOTE_MARKER),
            "skill must have managed-by note after install (CRLF)"
        );

        // Author edits the skill's body (simulate a module edit).
        let edited_body = skill_after_install.replace(
            "Drive the team through the playbook.",
            "Drive the team through the playbook.\n\nUpdated by author.",
        );
        fs::write(&skill_path, edited_body).unwrap();

        // Standalone emit_from_harness without second install::run.
        emit_from_harness(&root);

        // Verify managed-by note and modeline survive.
        let skill_after_emit = fs::read_to_string(&skill_path).unwrap();
        assert!(
            skill_after_emit.contains(temper::placement::NOTE_MARKER),
            "skill must retain managed-by note after standalone emit (CRLF)"
        );
        assert!(
            skill_after_emit.contains("Updated by author."),
            "skill must carry the authored edit after emit (CRLF)"
        );

        // Gate must be quiet (no drift).
        assert!(
            temper::drift::config_stale(&temper_dir, &fresh_clause()).is_empty(),
            "gate must report no drift after install→edit→emit (CRLF)"
        );
    };

    test_lf("install-edit-emit-lf");
    test_crlf("install-edit-emit-crlf");
}

#[test]
fn guard_hook_fails_loud_when_temper_not_on_path() {
    // Run the GUARD_COMMAND with temper not on PATH to verify it fails
    // with a clear message naming temper as the missing binary.
    let tmpdir = tempfile::Builder::new()
        .prefix("hook-missing-temper-guard")
        .tempdir()
        .unwrap();
    let harness = tmpdir.path().to_path_buf();
    let _ = tmpdir.keep();

    // Create a minimal lock so the command can parse it.
    let temper_dir = harness.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(temper_dir.join("lock.toml"), "").unwrap();

    // Build a minimal guard payload.
    let payload = common::guard_write_payload(".claude/test.md");

    // Run the command through a shell with an empty PATH so temper cannot be found.
    let cmd = temper::install::GUARD_COMMAND;
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(format!("echo '{payload}' | {{ {cmd}; }}"))
        .current_dir(&harness)
        .env_clear()
        .output()
        .unwrap();

    // The command must fail (non-zero exit) when temper is not on PATH.
    assert!(
        !output.status.success(),
        "guard must fail when temper is not on PATH"
    );

    // The error message must name temper as the missing binary.
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("temper: command not found"),
        "stderr must contain 'temper: command not found' to name the missing binary, got:\n{stderr}"
    );
}

/// A `PostToolUse` payload after a Bash call — the event, the tool, and a `tool_response`
/// naming no path (code.claude.com/docs/en/hooks, "PostToolUse input", retrieved
/// 2026-09-25). The shell edge's whole input: the subject is the tree the call left.
const SHELL_CALL_PAYLOAD: &str = "{\"session_id\":\"s\",\
     \"hook_event_name\":\"PostToolUse\",\"tool_name\":\"Bash\",\
     \"tool_input\":{\"command\":\"true\"},\
     \"tool_response\":{\"stdout\":\"\",\"stderr\":\"\",\"interrupted\":false}}";

/// A represented harness declaring one top-level layout document — a `guide` at
/// `specs/guide.md` whose body opens with a prose region importing `specs/included.md`
/// and closes with an `intent` field section — emitted, so the lock carries the member's
/// declaration row and the import's fingerprint and no provenance row (a layout document
/// is a source emit reads, never a projection it writes). Rows-or-default: no clause row,
/// so the shipped root `fresh` and `locus-declared` clauses bind, under `warn`.
fn declared_layout_harness(label: &str) -> PathBuf {
    let harness = common::scaffold(label);
    emit_guide_layout(&harness, vec![common::layout_member("guide")]);
    harness
}

/// Emit [`declared_layout_harness`]'s `guide` kind into `harness`, its documents written
/// and its lock carrying exactly `members`' declaration rows.
fn emit_guide_layout(harness: &Path, members: Vec<drift::PayloadMember>) {
    fs::write(
        harness.join("specs/guide.md"),
        "# Intent\ntemper makes a harness good.\n",
    )
    .unwrap();
    fs::write(harness.join("specs/included.md"), "shared prose.\n").unwrap();
    let payload = drift::Payload {
        version: drift::SEAM_VERSION,
        declarations: drift::Declarations {
            kinds: vec![common::layout_kind_facts(
                "guide",
                vec![
                    drift::LayoutRegionRow {
                        region: "prose".to_string(),
                        import: Some("included.md".to_string()),
                        slot: None,
                        member_kind: None,
                        key: None,
                    },
                    common::field_region("intent"),
                ],
            )],
            ..Default::default()
        },
        members,
    };
    drift::emit(
        &payload,
        &harness.join(".temper"),
        drift::EmitOptions::default(),
    )
    .unwrap();
}

/// A declared layout document is a declared member — its lock evidence is the
/// declaration row `emit` lowered it into, never a provenance row — so `check` raises no
/// `locus-declared` finding over it, and neither may the guard at either edge.
///
/// The silence is proven non-vacuous first: with the member's declaration absent, the
/// guide's starless `specs/guide.md` locus draws the guard's finding at both edges.
#[test]
fn the_guard_raises_no_locus_finding_for_a_declared_layout_document() {
    let harness = common::scaffold("guard-declared-layout");
    emit_guide_layout(&harness, Vec::new());
    assert_the_guard_flags_the_undeclared_guide(&harness);

    emit_guide_layout(&harness, vec![common::layout_member("guide")]);
    let (findings, _) = common::check_harness(&harness);
    assert!(
        common::findings_for(&findings, "root.locus-declared").is_empty(),
        "the premise: `check` calls the declared layout document declared, got: {findings:?}"
    );

    let (code, output) = common::run_guard(&harness, SHELL_CALL_PAYLOAD);
    assert_eq!(code, Some(0));
    assert!(
        output.is_empty(),
        "a shell call over a clean tree with a declared layout document passes silently, \
         got: {output}"
    );

    let (code, output) =
        common::run_guard(&harness, &common::guard_write_payload("specs/guide.md"));
    assert_eq!(code, Some(0));
    assert!(
        output.is_empty(),
        "an edit to a declared layout document is no undeclared member, got: {output}"
    );
}

/// The populated subject of the declared-layout silence: the guard binds the guide's
/// locus, flagging `specs/guide.md` at the post edge and in a pending write while no
/// declaration row names it.
fn assert_the_guard_flags_the_undeclared_guide(harness: &Path) {
    let (code, output) = common::run_guard(harness, SHELL_CALL_PAYLOAD);
    assert_eq!(code, Some(0), "warn allows the call, got: {output}");
    let in_band = common::guard_in_band(&output, "PostToolUse");
    assert_check_raises_the_guard_s_layout_finding(harness, &in_band, "specs/guide.md");

    let (code, output) = common::run_guard(harness, &common::guard_write_payload("specs/guide.md"));
    assert_eq!(code, Some(0), "warn allows the write, got: {output}");
    let in_band = common::guard_in_band(&output, "PreToolUse");
    assert_check_raises_the_guard_s_layout_finding(harness, &in_band, "specs/guide.md");
}

#[test]
fn the_guard_flags_the_guide_layout_document_when_its_declaration_is_absent() {
    let harness = common::scaffold("guard-undeclared-guide");
    emit_guide_layout(&harness, Vec::new());
    assert_the_guard_flags_the_undeclared_guide(&harness);
}

/// The shell edge enumerates its loci the way `check`'s discovery does: an ignored file
/// is by declaration not authored here, so a document the repo's root `.gitignore`
/// names is no site, even inside a governed locus — while an unignored stray beside it
/// still is.
#[test]
fn the_guard_s_locus_walk_honours_ignore_rules() {
    let harness = common::tmpdir("guard-locus-walk-ignore");
    let rules = harness.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(rules.join("scratch.md"), "# Scratch\n").unwrap();
    fs::write(rules.join("stray.md"), "# Stray\n").unwrap();
    fs::write(harness.join(".gitignore"), ".claude/rules/scratch.md\n").unwrap();
    common::write_lock(
        &harness,
        drift::Declarations {
            clauses: vec![common::clause("locus-declared", "required")],
            ..drift::Declarations::default()
        },
    );

    let (findings, _) = common::check_harness(&harness);
    let check_locus = common::findings_for(&findings, "root.locus-declared");
    assert!(
        check_locus.iter().all(|line| !line.contains("scratch.md")),
        "the premise: `check` never discovers the ignored document, got: {findings:?}"
    );

    let (code, output) = common::run_guard(&harness, SHELL_CALL_PAYLOAD);
    assert_eq!(code, Some(0));
    let in_band = common::guard_in_band(&output, "PostToolUse");
    assert!(
        in_band.contains(".claude/rules/stray.md"),
        "the unignored stray is still a site, got: {in_band}"
    );
    assert!(
        !in_band.contains("scratch.md"),
        "an ignored document is no site of the guard's walk, got: {in_band}"
    );
}

/// The post edge runs the whole of the root `fresh` clause `check` runs: a layout
/// document's imported prose moving under an unmoved projection is source-dependency
/// drift, and the shell call that moved it hears so.
#[test]
fn the_guard_s_post_edge_reports_a_stale_layout_import() {
    let harness = declared_layout_harness("guard-stale-layout-import");
    fs::write(harness.join("specs/included.md"), "edited prose.\n").unwrap();

    let (findings, _) = common::check_harness(&harness);
    assert!(
        findings
            .iter()
            .any(|line| line.contains("title=root.fresh::") && line.contains("layout import")),
        "the premise: `check` calls the moved import stale, got: {findings:?}"
    );

    let (code, output) = common::run_guard(&harness, SHELL_CALL_PAYLOAD);
    assert_eq!(code, Some(0));
    let in_band = common::guard_in_band(&output, "PostToolUse");
    assert!(
        in_band.contains("temper source dependency"),
        "the stale import speaks under the source-dependency preamble, got: {in_band}"
    );
    assert!(
        in_band.contains("root.fresh: ") && in_band.contains("specs/included.md"),
        "and names the moved import under `check`'s own label, got: {in_band}"
    );
}

/// A represented harness declaring one top-level layout document at a wildcard locus — a
/// `charter` kind governing `docs/charters/*.md`, its one member `docs/charters/charter.md`
/// declared and emitted — plus an undeclared `docs/charters/stray.md` beside it. The
/// stray fits the layout, so `check` reads it as a member of the kind and finds no
/// `layout_source` row naming it. Rows-or-default: the shipped root `locus-declared`
/// clause binds, under `warn`.
fn undeclared_layout_member_harness(label: &str) -> PathBuf {
    let harness = common::scaffold(label);
    let charters = harness.join("docs/charters");
    fs::create_dir_all(&charters).unwrap();
    fs::write(
        charters.join("charter.md"),
        "# Intent\nThe gate is check.\n",
    )
    .unwrap();
    let payload = drift::Payload {
        version: drift::SEAM_VERSION,
        declarations: drift::Declarations {
            kinds: vec![drift::KindFactRow {
                content: Some(drift::LayoutRow {
                    regions: vec![common::field_region("intent")],
                }),
                ..common::kind_facts("charter", "docs/charters", "*.md")
            }],
            ..Default::default()
        },
        members: vec![common::layout_member("charter")],
    };
    drift::emit(
        &payload,
        &harness.join(".temper"),
        drift::EmitOptions::default(),
    )
    .unwrap();
    harness
}

/// The one `root.locus-declared` finding a guard's in-band surface speaks about the
/// layout document at `path`, asserted to be a finding `check` raises word for word over
/// the same tree — the guard is `check` at a narrower trigger, never a second reader.
fn assert_check_raises_the_guard_s_layout_finding(harness: &Path, in_band: &str, path: &str) {
    let spoken = in_band
        .lines()
        .filter_map(|line| line.strip_prefix("  root.locus-declared: "))
        .find(|message| message.contains(path) && message.starts_with("layout document"))
        .unwrap_or_else(|| {
            panic!("the guard raises the undeclared layout member `{path}`, got: {in_band}")
        });
    let (findings, _) = common::check_harness(harness);
    assert!(
        common::findings_for(&findings, "root.locus-declared")
            .iter()
            .any(|line| line.contains(spoken)),
        "the guard's `{spoken}` is a finding `check` raises on the same tree, got: \
         {findings:#?}"
    );
}

#[test]
fn the_guard_raises_an_undeclared_layout_member_at_the_post_edge() {
    let harness = undeclared_layout_member_harness("guard-layout-stray-post");
    fs::write(
        harness.join("docs/charters/stray.md"),
        "# Intent\nNobody declared me.\n",
    )
    .unwrap();

    let (code, output) = common::run_guard(&harness, SHELL_CALL_PAYLOAD);
    assert_eq!(code, Some(0), "warn allows the call, got: {output}");
    let in_band = common::guard_in_band(&output, "PostToolUse");
    assert_check_raises_the_guard_s_layout_finding(&harness, &in_band, "docs/charters/stray.md");
    assert!(
        !in_band.contains("docs/charters/charter.md`"),
        "the declared layout document is no stray, got: {in_band}"
    );
}

#[test]
fn the_guard_raises_an_undeclared_layout_member_in_a_pending_write() {
    let harness = undeclared_layout_member_harness("guard-layout-stray-pre");

    // The pending write to the declared document is no undeclared member.
    let (code, output) = common::run_guard(
        &harness,
        &common::guard_write_payload("docs/charters/charter.md"),
    );
    assert_eq!(code, Some(0));
    assert!(
        output.is_empty(),
        "a write to the declared layout document passes silently, got: {output}"
    );

    let (code, output) = common::run_guard(
        &harness,
        &common::guard_write_payload("docs/charters/stray.md"),
    );
    assert_eq!(code, Some(0), "warn allows the write, got: {output}");
    let in_band = common::guard_in_band(&output, "PreToolUse");

    // Land the write, and `check` raises the very finding the guard spoke.
    fs::write(
        harness.join("docs/charters/stray.md"),
        "# Intent\nNobody declared me.\n",
    )
    .unwrap();
    assert_check_raises_the_guard_s_layout_finding(&harness, &in_band, "docs/charters/stray.md");
}

/// A represented harness whose layout kind is a nested child — a `spec` governing no
/// locus, templated at `*.md` under each `guide` host's `docs/guides/<name>/` unit — with
/// one host `operate` and its declared child `representation`, emitted, plus an
/// undeclared `stray.md` beside it in the host's unit. Rows-or-default: the shipped root
/// `locus-declared` clause binds, under `warn`.
fn nested_layout_child_harness(label: &str) -> PathBuf {
    let harness = common::scaffold(label);
    let unit = harness.join("docs/guides/operate");
    fs::create_dir_all(&unit).unwrap();
    fs::write(unit.join("representation.md"), NESTED_SPEC_DOC).unwrap();
    let payload = drift::Payload {
        version: drift::SEAM_VERSION,
        declarations: drift::Declarations {
            kinds: vec![
                KindFactRow {
                    unit_shape: Some("directory".to_string()),
                    templates: vec![drift::TemplateRow {
                        kind: "spec".to_string(),
                        path: Some("*.md".to_string()),
                    }],
                    ..common::kind_facts("guide", "docs/guides", "*/GUIDE.md")
                },
                KindFactRow {
                    governs_root: None,
                    governs_glob: Vec::new(),
                    unit_shape: Some("file".to_string()),
                    content: Some(drift::LayoutRow {
                        regions: vec![common::field_region("intent")],
                    }),
                    ..common::kind_facts("spec", "", "")
                },
            ],
            ..Default::default()
        },
        members: vec![
            common::payload_member("guide", "operate", None, "# Operate\n"),
            common::payload_member("spec", "representation", Some("guide:operate"), ""),
        ],
    };
    drift::emit(
        &payload,
        &harness.join(".temper"),
        drift::EmitOptions::default(),
    )
    .unwrap();
    harness
}

/// A `spec` document fitting its layout: one `# Intent` field section.
const NESTED_SPEC_DOC: &str = "# Intent\nThe gate is check.\n";

/// The undeclared child of [`nested_layout_child_harness`]'s `operate` host.
const NESTED_STRAY: &str = "docs/guides/operate/stray.md";

#[test]
fn the_guard_names_an_undeclared_nested_layout_child_at_the_post_edge() {
    let harness = nested_layout_child_harness("guard-nested-layout-post");
    let (code, output) = common::run_guard(&harness, SHELL_CALL_PAYLOAD);
    assert_eq!(code, Some(0));
    assert!(
        output.is_empty(),
        "the declared child alone is no stray, got: {output}"
    );

    fs::write(harness.join(NESTED_STRAY), NESTED_SPEC_DOC).unwrap();
    let before = temper::import::walk_count();
    let declarations = drift::read_declarations(&harness.join(".temper")).unwrap();
    let report = install::shell_edge_findings(
        &harness.join(".temper"),
        &declarations,
        &[],
        &temper::compose::modes_from_declarations(&declarations).unwrap(),
    )
    .unwrap()
    .message;
    assert!(!report.is_empty(), "the undeclared child reports");
    assert!(report.contains(NESTED_STRAY), "got: {report}");
    let walks = temper::import::walk_count() - before;
    assert!(
        walks <= 2,
        "the hosts are read through one discovery walk per flavor per call, never one per \
         kind or host, got: {walks}"
    );

    let (code, output) = common::run_guard(&harness, SHELL_CALL_PAYLOAD);
    assert_eq!(code, Some(0), "warn allows the call, got: {output}");
    let in_band = common::guard_in_band(&output, "PostToolUse");
    assert_check_raises_the_guard_s_layout_finding(&harness, &in_band, NESTED_STRAY);
    assert!(
        !in_band.contains("representation.md"),
        "the declared child is no stray, got: {in_band}"
    );
}

#[test]
fn the_guard_names_an_undeclared_nested_layout_child_at_the_pre_edge() {
    let harness = nested_layout_child_harness("guard-nested-layout-pre");
    let (code, output) = common::run_guard(
        &harness,
        &common::guard_write_payload("docs/guides/operate/representation.md"),
    );
    assert_eq!(code, Some(0));
    assert!(
        output.is_empty(),
        "a write to the declared child passes silently, got: {output}"
    );

    // The undeclared child already sits in the host's unit; the pending write edits it.
    fs::write(harness.join(NESTED_STRAY), NESTED_SPEC_DOC).unwrap();
    let (code, output) = common::run_guard(&harness, &common::guard_write_payload(NESTED_STRAY));
    assert_eq!(code, Some(0), "warn allows the write, got: {output}");
    let in_band = common::guard_in_band(&output, "PreToolUse");
    assert_check_raises_the_guard_s_layout_finding(&harness, &in_band, NESTED_STRAY);
}

#[test]
fn the_guard_names_a_pending_write_that_creates_an_undeclared_nested_layout_child() {
    let harness = nested_layout_child_harness("guard-nested-layout-create");
    assert!(!harness.join(NESTED_STRAY).exists());
    let (code, output) = common::run_guard(
        &harness,
        &common::guard_write_payload_with_content(NESTED_STRAY, NESTED_SPEC_DOC),
    );
    assert_eq!(code, Some(0), "warn allows the write, got: {output}");
    let in_band = common::guard_in_band(&output, "PreToolUse");

    // A path outside the host template's pattern places no child, so it binds nothing.
    let (code, output) = common::run_guard(
        &harness,
        &common::guard_write_payload("docs/guides/operate/deeper/stray.txt"),
    );
    assert_eq!(code, Some(0));
    assert!(output.is_empty(), "no placement, no binding, got: {output}");

    // Once the write lands, `check` raises the finding the guard spoke before it.
    fs::write(harness.join(NESTED_STRAY), NESTED_SPEC_DOC).unwrap();
    assert_check_raises_the_guard_s_layout_finding(&harness, &in_band, NESTED_STRAY);
}

/// `memory`'s locus is `.`-rooted (`**/CLAUDE.md`): it reaches every directory, and the
/// pending-write edge binds it wherever discovery would reach the written path.
#[test]
fn the_pre_edge_names_an_undeclared_claude_md_written_under_a_dot_rooted_locus() {
    let root = represented_rule_harness("guard-dot-rooted-memory", "block");
    let (code, stderr) = common::run_guard(&root, &common::guard_write_payload("docs/CLAUDE.md"));
    assert_eq!(
        code,
        Some(2),
        "block denies an undeclared memory document, got: {stderr}"
    );
    assert!(
        stderr.contains("`memory` kind's governed locus"),
        "the finding names the `memory` kind, as `check` does, got: {stderr}"
    );
}

/// A gitignored `CLAUDE.md` is no member `check` judges, so the `.`-rooted locus binds
/// nothing there; an unignored sibling stays bound, so the allowance is the ignore rule's.
#[test]
fn the_pre_edge_binds_nothing_for_a_gitignored_claude_md_under_a_dot_rooted_locus() {
    let root = represented_rule_harness("guard-dot-rooted-ignored", "block");
    fs::write(root.join(".gitignore"), "vendor/\n").unwrap();

    let (code, stderr) =
        common::run_guard(&root, &common::guard_write_payload("vendor/dep/CLAUDE.md"));
    assert_eq!(
        code,
        Some(0),
        "a gitignored path binds nothing, got: {stderr}"
    );
    assert!(stderr.is_empty(), "a pass prints nothing, got: {stderr}");

    let (sibling_code, sibling_stderr) =
        common::run_guard(&root, &common::guard_write_payload("lib/dep/CLAUDE.md"));
    assert_eq!(
        sibling_code,
        Some(2),
        "an unignored sibling stays bound, got: {sibling_stderr}"
    );
}

/// [`undeclared_layout_member_harness`]'s `charter` kind, keyed by its frontmatter
/// `title` rather than its file stem; the declared document is titled `charter`.
fn named_field_layout_member_harness(label: &str) -> PathBuf {
    let harness = common::scaffold(label);
    let charters = harness.join("docs/charters");
    fs::create_dir_all(&charters).unwrap();
    fs::write(
        charters.join("charter.md"),
        "---\ntitle: charter\n---\n# Intent\nThe gate is check.\n",
    )
    .unwrap();
    let payload = drift::Payload {
        version: drift::SEAM_VERSION,
        declarations: drift::Declarations {
            kinds: vec![drift::KindFactRow {
                format: Some("yaml-frontmatter".to_string()),
                unit_shape: Some("named-field(title)".to_string()),
                content: Some(drift::LayoutRow {
                    regions: vec![common::field_region("intent")],
                }),
                ..common::kind_facts("charter", "docs/charters", "*.md")
            }],
            ..Default::default()
        },
        members: vec![common::layout_member("charter")],
    };
    drift::emit(
        &payload,
        &harness.join(".temper"),
        drift::EmitOptions::default(),
    )
    .unwrap();
    harness
}

#[test]
fn the_pre_edge_names_an_undeclared_named_field_layout_document_by_its_frontmatter_identity() {
    let harness = named_field_layout_member_harness("guard-named-field-layout-pre");

    // A rewrite of the declared document keeps its declared identity.
    let declared = "---\ntitle: charter\n---\n# Intent\nThe gate is still check.\n";
    let (code, output) = common::run_guard(
        &harness,
        &common::guard_write_payload_with_content("docs/charters/charter.md", declared),
    );
    assert_eq!(code, Some(0));
    assert!(
        output.is_empty(),
        "a write keeping the declared identity passes silently, got: {output}"
    );

    let stray = "---\ntitle: wayward\n---\n# Intent\nNobody declared me.\n";
    let (code, output) = common::run_guard(
        &harness,
        &common::guard_write_payload_with_content("docs/charters/stray.md", stray),
    );
    assert_eq!(code, Some(0), "warn allows the write, got: {output}");
    let in_band = common::guard_in_band(&output, "PreToolUse");
    assert!(
        in_band.contains("charter:wayward"),
        "the finding names the document by its frontmatter identity, got: {in_band}"
    );

    // Land the write, and `check` raises the very finding the guard spoke.
    fs::write(harness.join("docs/charters/stray.md"), stray).unwrap();
    assert_check_raises_the_guard_s_layout_finding(&harness, &in_band, "docs/charters/stray.md");
}

/// A represented harness whose nested layout child `spec` is templated at `*.md` under
/// each unit of a `local`-commitment `guide` host — `docs/guides/<name>/GUIDE.md`, the
/// whole locus gitignored as a per-machine corpus is. The host `operate` sits on disk
/// with no child declared under it. Rows-or-default: the root `locus-declared` binds,
/// under `warn`. `check`'s nested-file scan honors the host's commitment, so it reaches
/// every child in the ignored unit.
fn local_host_child_harness(label: &str) -> PathBuf {
    let harness = common::scaffold(label);
    let unit = harness.join("docs/guides/operate");
    fs::create_dir_all(&unit).unwrap();
    fs::write(unit.join("GUIDE.md"), "# Operate\n").unwrap();
    fs::write(harness.join(".gitignore"), "docs/guides/\n").unwrap();
    let payload = drift::Payload {
        version: drift::SEAM_VERSION,
        declarations: drift::Declarations {
            kinds: vec![
                KindFactRow {
                    commitment: Some("local".to_string()),
                    unit_shape: Some("directory".to_string()),
                    templates: vec![drift::TemplateRow {
                        kind: "spec".to_string(),
                        path: Some("*.md".to_string()),
                    }],
                    ..common::kind_facts("guide", "docs/guides", "*/GUIDE.md")
                },
                KindFactRow {
                    governs_root: None,
                    governs_glob: Vec::new(),
                    unit_shape: Some("file".to_string()),
                    content: Some(drift::LayoutRow {
                        regions: vec![common::field_region("intent")],
                    }),
                    ..common::kind_facts("spec", "", "")
                },
            ],
            ..Default::default()
        },
        members: Vec::new(),
    };
    drift::emit(
        &payload,
        &harness.join(".temper"),
        drift::EmitOptions::default(),
    )
    .unwrap();
    harness
}

/// A child of [`local_host_child_harness`]'s `operate` unit, under the gitignored locus.
const LOCAL_HOST_STRAY: &str = "docs/guides/operate/stray.md";

#[test]
fn the_guard_binds_a_pending_write_to_a_gitignored_member_at_a_committed_local_locus_kind() {
    let harness = local_host_child_harness("guard-local-host-edit");
    fs::write(harness.join(LOCAL_HOST_STRAY), NESTED_SPEC_DOC).unwrap();
    let (code, output) =
        common::run_guard(&harness, &common::guard_write_payload(LOCAL_HOST_STRAY));
    assert_eq!(code, Some(0), "warn allows the write, got: {output}");
    let in_band = common::guard_in_band(&output, "PreToolUse");
    assert_check_raises_the_guard_s_layout_finding(&harness, &in_band, LOCAL_HOST_STRAY);
}

#[test]
fn the_guard_places_a_pending_hosted_child_under_a_local_host_as_check_s_scan_places_it() {
    let harness = local_host_child_harness("guard-local-host-create");
    assert!(!harness.join(LOCAL_HOST_STRAY).exists());
    let (code, output) = common::run_guard(
        &harness,
        &common::guard_write_payload_with_content(LOCAL_HOST_STRAY, NESTED_SPEC_DOC),
    );
    assert_eq!(code, Some(0), "warn allows the write, got: {output}");
    let in_band = common::guard_in_band(&output, "PreToolUse");

    // The host's own entry is no child of its unit, whatever its flavor.
    let (code, output) = common::run_guard(
        &harness,
        &common::guard_write_payload("docs/guides/operate/GUIDE.md"),
    );
    assert_eq!(code, Some(0));
    assert!(
        output.is_empty(),
        "the host entry places no child, got: {output}"
    );

    fs::write(harness.join(LOCAL_HOST_STRAY), NESTED_SPEC_DOC).unwrap();
    assert_check_raises_the_guard_s_layout_finding(&harness, &in_band, LOCAL_HOST_STRAY);
}

#[test]
fn represented_install_refuses_a_discovered_skill_named_like_the_starter_reference_before_writing()
{
    common::ensure_sdk_built();
    let root = common::tmpdir("reference-name-collision");
    common::write_skill(
        &root,
        "temper",
        "---\nname: temper\ndescription: Use when operating our own temper wrapper.\n---\n\nAuthored here.\n",
    );
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_install_workspace(&temper_dir);
    let before = common::tree_bytes(&root);

    let output = Command::new(BIN)
        .arg("install")
        .arg(&root)
        .arg("--yes")
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "a skill named like the starter's reference refuses the represented install"
    );
    let stderr = String::from_utf8(output.stderr).unwrap();
    for stated in [
        "reference_name_collision",
        "\"temper\"",
        "rename your skill",
    ] {
        assert!(
            stderr.contains(stated),
            "the refusal states `{stated}`, got: {stderr}"
        );
    }
    assert_eq!(
        before,
        common::tree_bytes(&root),
        "the refusal lands before any write under .temper/"
    );
    assert!(!temper_dir.join("harness.ts").exists());
}
