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

use temper::drift::ApplyOutcome;
use temper::install::{self, InstallOutcome, Represent};

mod common;

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

/// [`NON_CANONICAL_SETTINGS`], but with the `SessionStart` hook already merged in —
/// the starting point for a second merge that only has the `PreToolUse` guard left
/// to graft.
const NON_CANONICAL_SETTINGS_WITH_HOOK: &str = "{\n    \"zeta\": \"first\",\n    \"hooks\": {\n        \"SessionStart\": [\n            { \"hooks\": [ { \"type\": \"command\", \"command\": \"command -v temper >/dev/null 2>&1 || { echo \\\"temper: command not found\\\" >&2; exit 127; } && temper check . --reporter session-start\" } ] }\n        ]\n    }\n}\n";

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

    let rendered = install::render_discovery(&report, None);
    assert!(rendered.contains("skill"));
    assert!(rendered.contains("rule"));
}

#[test]
fn an_empty_project_reports_no_members_found() {
    let root = common::tmpdir("discover-empty");
    let report = install::discover(&root).unwrap();
    assert_eq!(report.total(), 0);
    assert!(install::render_discovery(&report, None).contains("no members found"));
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
    assert_eq!(report.members.get("memory").map(Vec::len), Some(1));
    let memory = report.members.get("memory").unwrap();
    assert!(!memory.iter().any(|p| p.starts_with(root.join("vendor"))));
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
        outcome_of(&outcome, temper::install::Placement::SessionStart),
        ApplyOutcome::Applied
    );
    assert!(!has_entry(&outcome, temper::install::Placement::GuardHook));
    assert!(!has_entry(&outcome, temper::install::Placement::Note));
    assert!(!has_entry(&outcome, temper::install::Placement::Modeline));

    let settings = fs::read_to_string(root.join(".claude").join("settings.json")).unwrap();
    let json: serde_json::Value = serde_json::from_str(&settings).unwrap();
    assert_eq!(
        json["hooks"]["SessionStart"][0]["hooks"][0]["command"],
        "command -v temper >/dev/null 2>&1 || { echo \"temper: command not found\" >&2; exit 127; } && temper check . --reporter session-start"
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
        "command -v temper >/dev/null 2>&1 || { echo \"temper: command not found\" >&2; exit 127; } && temper check . --reporter session-start"
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
        "command -v temper >/dev/null 2>&1 || { echo \"temper: command not found\" >&2; exit 127; } && temper check . --reporter session-start",
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

#[test]
fn representing_hoists_every_field_and_regenerates_every_member_as_a_guard_claimed_projection() {
    let root = write_harness("represent", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));

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
        "command -v temper >/dev/null 2>&1 || { echo \"temper: command not found\" >&2; exit 127; } && temper guard ."
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
        coordinate_md.contains("# temper: managed projection"),
        "a scaffolded member's own frontmatter-bearing projection is note-claimed, got:\n{coordinate_md}"
    );
    let rust_md = fs::read_to_string(root.join(".claude").join("rules").join("rust.md")).unwrap();
    assert!(rust_md.contains("# temper: managed projection"));
    // `collaboration` has no frontmatter to carry the `#` note, so it takes the
    // block-level HTML-comment banner heading its body instead — a frontmatterless
    // markdown projection is note-claimed, never left bannerless.
    let collaboration_md =
        fs::read_to_string(root.join(".claude").join("rules").join("collaboration.md")).unwrap();
    assert!(
        collaboration_md.starts_with("<!-- temper: managed projection"),
        "got:\n{collaboration_md}"
    );
    assert!(!collaboration_md.contains("# temper: managed projection"));
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
    common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));

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
    common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));

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

#[test]
fn re_representing_never_re_scaffolds_and_settles_on_the_first_run() {
    let root = write_harness("re-represent", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();
    let after_first = common::tree_bytes(&root);

    // `evaluate_placements` writes each emit-owned target's managed-by note
    // *after* the first `emit` already stamped the lock's fingerprints from
    // the pre-placement bytes; a re-stamping `emit` inside `install::run`
    // must fold those placements back in before the run returns, so the lock
    // already matches this run's own output with no second run required.
    assert!(
        temper::drift::config_stale(&temper_dir).is_empty(),
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
    common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();

    let claude_md = fs::read_to_string(root.join("CLAUDE.md")).unwrap();
    assert!(
        claude_md.starts_with("<!-- temper: managed projection"),
        "the frontmatterless memory projection heads its body with the HTML-comment banner, got:\n{claude_md}"
    );
    // The banner form only — never the `#` frontmatter note (there is no frontmatter).
    assert!(!claude_md.contains("# temper: managed projection"));
    assert!(claude_md.contains("Memory for the agents."));

    let skill_md = fs::read_to_string(skill.join("SKILL.md")).unwrap();
    assert!(
        skill_md.contains("# temper: managed projection"),
        "a frontmatter kind keeps the `#` note, got:\n{skill_md}"
    );
    assert!(!skill_md.contains("<!-- temper: managed projection"));

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
            .matches("<!-- temper: managed projection")
            .count(),
        1
    );
    assert!(temper::drift::config_stale(&temper_dir).is_empty());
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
    common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));

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
        !module_contents.contains("<!-- temper: managed projection"),
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
        claude_md.matches("<!-- temper: managed projection").count(),
        1,
        "emit must place exactly one banner, not duplicate; got:\n{claude_md}"
    );
}

/// Serializes the one test below that shadows the process-wide `PATH` — no other
/// test in this suite spawns a real `npm` (every other yes-path test vendors the
/// dependency via [`common::vendor_sdk`], so `dependency_resolves` short-circuits before
/// ever reaching a spawn), but a shared `PATH` is process state, not per-test.
static PATH_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn a_dependency_spawn_failure_leaves_no_half_scaffolded_state() {
    // Force the SDK's own `npm run build` (real `npm`, gated by a `Once`) to finish
    // first — otherwise a concurrently first-triggered `ensure_sdk_built` elsewhere
    // could race the shadowed `npm` this test installs below.
    common::ensure_sdk_built();

    let root = write_harness("dependency-spawn-failure", false);
    let temper_dir = root.join(".temper");
    let discovery = install::discover(&root).unwrap();

    let guard = PATH_MUTEX.lock().unwrap();
    let original_path = std::env::var_os("PATH").unwrap_or_default();

    // A shadow `npm`/`npm.cmd` on `PATH` ahead of the real one, always failing —
    // standing in for "only npm.cmd exists and this Windows spawn can't find it"
    // without actually requiring a Windows host to prove the ordering.
    let fake_bin = common::tmpdir("dependency-spawn-failure-fake-npm");
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
    let before = common::tree_bytes(&root);

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
    common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));

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
        "command -v temper >/dev/null 2>&1 || { echo \"temper: command not found\" >&2; exit 127; } && temper guard ."
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
        .contains("# temper: managed projection"),
        "a scaffolded member's projection is note-claimed exactly like a hand-deepened one"
    );

    // No `.temper/schema/skill.json` exists yet, so no modeline is placed even on
    // the emit-owned target — a modeline pointing at nothing is worse than none.
    assert!(!extra_md.contains("# yaml-language-server:"));
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
        extra_md_after.starts_with(
            "---\n# yaml-language-server: $schema=../../../.temper/schema/skill.json\n"
        ),
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
        coordinate_md_after.starts_with(
            "---\n# yaml-language-server: $schema=../../../.temper/schema/skill.json\n"
        ),
        "got:\n{coordinate_md_after}"
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
    common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));

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
         export const extra = skill({\n  name: \"extra\",\n  description: \"An extra skill authored by hand.\",\n  prose: file(import.meta.url, \"./extra.md\"),\n});\n",
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
    assert_eq!(
        outcome_of(&outcome, temper::install::Placement::GuardHook),
        ApplyOutcome::Applied
    );

    let after = fs::read_to_string(&settings_path).unwrap();
    assert_one_hunk_diff(NON_CANONICAL_SETTINGS_WITH_HOOK, &after);

    let json: serde_json::Value = serde_json::from_str(&after).unwrap();
    assert_eq!(
        json["hooks"]["PreToolUse"][0]["hooks"][0]["command"],
        "command -v temper >/dev/null 2>&1 || { echo \"temper: command not found\" >&2; exit 127; } && temper guard ."
    );
    assert_eq!(
        json["hooks"]["SessionStart"][0]["hooks"][0]["command"],
        "command -v temper >/dev/null 2>&1 || { echo \"temper: command not found\" >&2; exit 127; } && temper check . --reporter session-start"
    );
    assert_eq!(
        json["zeta"], "first",
        "the human's non-canonical key survives"
    );

    // Re-running converges: both hooks are already in their desired shape.
    let second = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(
        outcome_of(&second, temper::install::Placement::GuardHook),
        ApplyOutcome::Unchanged
    );
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

    // Foreign harness with no .temper/ directory: gate_installed is silent.
    let before = install::gate_installed(&root);
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
    assert!(install::gate_installed(&root).is_empty());

    // Represent for real: the gate stays clean immediately after (no emit-owned
    // targets to nudge for a pure lift).
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));
    install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert!(
        install::gate_installed(&root).is_empty(),
        "got: {:?}",
        install::gate_installed(&root)
    );
}

#[test]
fn gate_installed_names_stale_noted_files() {
    let root = write_harness("gate-drifted", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));

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
        install::gate_installed(&root).is_empty(),
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
    let findings = install::gate_installed(&root);
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
fn gate_installed_does_not_report_superseded_by_member() {
    // Regression test: gate_installed must exclude SupersededByMember outcomes
    // from its tally, so a correctly-authored hook member that supersedes a
    // synthesized placement is not flagged as needing install.
    //
    // This verifies the fix for: b7b2a7a6 added SupersededByMember but
    // gate_installed wasn't updated to skip it alongside Unchanged.
    common::ensure_sdk_built();
    let root = write_harness("hook-supersede", false);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));

    // Represent and install: places the synthesized SessionStart hook.
    let discovery = install::discover(&root).unwrap();
    let first = install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert_eq!(
        outcome_of(&first, temper::install::Placement::SessionStart),
        ApplyOutcome::Applied,
        "first install must apply the SessionStart placement"
    );

    // Verify gate is clean after install.
    assert!(
        install::gate_installed(&root).is_empty(),
        "gate must be clean after successful install"
    );

    // Now manually add an authored hook member to the harness program that
    // claims SessionStart. This simulates what would happen if a user wrote:
    // `import { sessionStartHook } from "./hooks/session_start.ts";`
    // and added it to the harness members array.
    fs::create_dir_all(temper_dir.join("hooks")).unwrap();
    fs::write(
        temper_dir.join("hooks").join("session_start.ts"),
        "import { hook } from \"@dtmd/temper/claude-code\";\n\
         export const sessionStartHook = hook({\n  \
         name: \"SessionStart\",\n  \
         type: \"command\",\n  \
         command: \"echo test\",\n  \
         });\n",
    )
    .unwrap();

    // Update the harness to include the hook member by inserting it into the
    // members array. The array may be formatted differently than we expect, so we
    // use a more flexible approach: insert before the closing bracket.
    let original_harness = fs::read_to_string(temper_dir.join("harness.ts")).unwrap();

    // First add the import if not already present
    let mut updated_harness = if original_harness.contains("sessionStartHook") {
        original_harness.clone()
    } else {
        let import_line = "import { sessionStartHook } from \"./hooks/session_start.ts\";\n";
        // Insert after the last import statement
        if let Some(last_import_pos) = original_harness.rfind("import {") {
            let eol = original_harness[last_import_pos..].find('\n').unwrap_or(0);
            let insert_pos = last_import_pos + eol + 1;
            let mut result = original_harness.clone();
            result.insert_str(insert_pos, import_line);
            result
        } else {
            original_harness.clone()
        }
    };

    // Then add to members array - look for closing bracket and add before it
    if !updated_harness.contains(", sessionStartHook") {
        updated_harness = updated_harness.replace("members: [", "members: [sessionStartHook, ");
    }
    fs::write(temper_dir.join("harness.ts"), updated_harness).unwrap();

    // Simulate a drift scenario: remove the placed hook from settings.json so the
    // next run will try to re-apply it. But now there's an authored hook member
    // that will claim it, so it should be marked SupersededByMember.
    let settings_path = root.join(".claude").join("settings.json");
    let settings = fs::read_to_string(&settings_path).unwrap();
    let drifted = settings.replace("\"SessionStart\": [", "\"SessionStartRemoved\": [");
    fs::write(&settings_path, drifted).unwrap();

    // Re-run install: evaluate_placements will see the SessionStart hook is missing (needs Applied).
    // emit will read the authored hook and register it in the lock.
    // detect_hook_member_conflicts will find the "SessionStart" registration and mark
    // the placement as SupersededByMember since its outcome would be Applied.
    let second = install::run(&root, &discovery, Represent::Yes, false).unwrap();

    assert_eq!(
        outcome_of(&second, temper::install::Placement::SessionStart),
        ApplyOutcome::SupersededByMember,
        "SessionStart should be marked SupersededByMember when an authored hook member claims it"
    );

    // Verify gate_installed does not report SessionStart as missing/drifted.
    // With the fix, SupersededByMember outcomes are skipped like Unchanged,
    // so the superseded SessionStart placement should not appear in the finding.
    let gate_findings = install::gate_installed(&root);
    if !gate_findings.is_empty() {
        // Check that SessionStart is NOT in the message (since it's superseded)
        let has_session_start = gate_findings
            .iter()
            .any(|d| d.message.contains("session-start hook"));
        assert!(
            !has_session_start,
            "SupersededByMember SessionStart should not be reported as missing/drifted"
        );
    }
}

// ---------------------------------------------------------------------------
// guard — the lock, not the retired manifest, grounds the enforcement mode
// ---------------------------------------------------------------------------

/// A `PreToolUse` payload naming a `.claude/` projection `file_path` — the write the
/// guard binds on.
const CLAUDE_WRITE_PAYLOAD: &str =
    "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\".claude/skills/x/SKILL.md\"}}";

/// A minimal lock row declaring `.claude/skills/x/SKILL.md` (the [`CLAUDE_WRITE_PAYLOAD`]
/// target) an emit-owned projection — real enforcement-mode tests bind against a declared
/// member, never a lock with no member rows at all.
const CLAUDE_WRITE_LOCK_ROW: &str = "[[skill]]\nname = \"x\"\nsource_path = \".claude/skills/x/SKILL.md\"\nsource_hash = \"abc\"\nemit_hash = \"abc\"\n";

#[test]
fn guard_reads_the_block_mode_from_the_lock_not_the_retired_manifest() {
    let root = common::tmpdir("lock-mode-block");
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(
        temper_dir.join("lock.toml"),
        format!("[[declaration.assembly]]\nfact = \"mode\"\nvalue = \"block\"\n\n{CLAUDE_WRITE_LOCK_ROW}"),
    )
    .unwrap();
    // A stray retired manifest naming the opposite enforcement mode must be ignored entirely —
    // the manifest is never read at all, by this or any other verb.
    fs::write(
        root.join(format!("temper{}toml", '.')),
        "authority = \"warn\"\n",
    )
    .unwrap();

    let (code, stderr) = common::run_guard(&root, CLAUDE_WRITE_PAYLOAD);
    assert_eq!(code, Some(2), "the lock's `block` mode must block");
    assert!(stderr.contains("direct Bash/PowerShell writes are not bound by it"));
}

/// With no `lock.toml` at all there is no declared projection set to consult — unlike
/// a represented harness (below), the guard falls back to binding any `.claude/` write
/// at the default enforcement mode rather than silently allowing everything: absent evidence
/// must never *suppress* a guard claim, only ever fail to forge one.
#[test]
fn guard_defaults_to_warn_when_the_lock_is_absent() {
    let root = common::tmpdir("lock-mode-absent");
    let (code, stderr) = common::run_guard(&root, CLAUDE_WRITE_PAYLOAD);
    assert_eq!(
        code,
        Some(0),
        "no lock ⇒ default warn, warns but never blocks"
    );
    assert!(stderr.contains("temper-managed projection"));

    let (allow_code, allow_stderr) = common::run_guard(
        &root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\"src/main.rs\"}}",
    );
    assert_eq!(allow_code, Some(0));
    assert!(allow_stderr.is_empty());
}

/// When the lock declares an emit-owned target outside `.claude/`, the guard binds
/// writes to that path just as it does for `.claude/` projections — the filter
/// derives from lock-declared targets, not a hardcoded `.claude/` regex.
#[test]
fn guard_binds_declared_locus_targets_outside_claude() {
    let root = common::tmpdir("lock-declared-outside-claude");
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();

    // A lock with a `block` mode and a single emit-owned target outside `.claude/`
    // (e.g., a layout kind that governs `.rules/` directly).
    fs::write(
        temper_dir.join("lock.toml"),
        "[[declaration.assembly]]\nfact = \"mode\"\nvalue = \"block\"\n\n[[rule]]\nname = \"safety\"\nsource_path = \".rules/safety.md\"\nsource_hash = \"def\"\nemit_hash = \"def\"\n"
    )
    .unwrap();

    // A write targeting the declared `.rules/safety.md` path should be bound by the
    // guard, not silently allowed (the bug the entry fixes).
    let (code, stderr) = common::run_guard(
        &root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\".rules/safety.md\"}}",
    );
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
    let (undeclared_code, undeclared_stderr) = common::run_guard(
        &root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\".claude/skills/x/SKILL.md\"}}",
    );
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
    let (other_code, other_stderr) = common::run_guard(
        &root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\"src/main.rs\"}}",
    );
    assert_eq!(other_code, Some(0));
    assert!(other_stderr.is_empty());
}

/// A represented harness whose lock declares one `rule` projection and nothing else —
/// the fixture the governed-locus binding is judged on. `mode` is the caller's, so one
/// writer serves the block/warn/note arms.
fn represented_rule_harness(name: &str, mode: &str) -> std::path::PathBuf {
    let root = common::tmpdir(name);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(
        temper_dir.join("lock.toml"),
        format!(
            "[[declaration.assembly]]\nfact = \"mode\"\nvalue = \"{mode}\"\n\n[[rule]]\nname = \"safety\"\nsource_path = \".claude/rules/safety.md\"\nsource_hash = \"abc\"\nemit_hash = \"abc\"\n"
        ),
    )
    .unwrap();
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
    // declare-and-re-emit remedy `check`'s `locus.undeclared-member` names.
    let block_root = represented_rule_harness("guard-undeclared-locus-block", "block");
    let (code, stderr) = common::run_guard(
        &block_root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\".claude/rules/stray.md\"}}",
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
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\".claude/agents/stray.md\"}}",
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
    let (warn_code, warn_stderr) = common::run_guard(
        &warn_root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\".claude/rules/stray.md\"}}",
    );
    assert_eq!(warn_code, Some(0), "warn mode allows the write");
    assert!(
        warn_stderr.contains("`rule` kind's governed locus"),
        "warn surfaces the finding in-band, got: {warn_stderr}"
    );

    // note — allowed, and nothing reaches the live session.
    let note_root = represented_rule_harness("guard-undeclared-locus-note", "note");
    let (note_code, note_stderr) = common::run_guard(
        &note_root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\".claude/rules/stray.md\"}}",
    );
    assert_eq!(note_code, Some(0), "note mode allows the write");
    assert!(
        note_stderr.is_empty(),
        "note records out-of-band only, got: {note_stderr}"
    );
}

/// The arms the governed-locus binding must leave exactly where they were: a declared
/// projection keeps the projection wording, a `local`-commitment locus is never bound by
/// it, a manifest locus stays the manifest arm's, and an unrepresented harness keeps the
/// `.claude/` fallback.
#[test]
fn the_governed_locus_binding_leaves_the_neighbouring_guard_arms_alone() {
    let root = represented_rule_harness("guard-locus-neighbours", "block");

    // A declared projection is drift, not an undeclared member — the projection binding
    // is consulted first and its wording is what the author reads.
    let (declared_code, declared_stderr) = common::run_guard(
        &root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\".claude/rules/safety.md\"}}",
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
        let (local_code, local_stderr) = common::run_guard(
            &root,
            &format!(
                "{{\"tool_name\":\"Write\",\"tool_input\":{{\"file_path\":\"{local_path}\"}}}}"
            ),
        );
        assert_eq!(
            local_code,
            Some(0),
            "a local-commitment locus is never bound by the locus binding ({local_path}), got: {local_stderr}"
        );
        assert!(local_stderr.is_empty());
    }

    // A manifest kind's locus stays `manifest_write_findings`'s: `.claude/settings.json`
    // is not emit-owned in this lock (no registration rows), and the `collection_address`
    // exclusion keeps the locus binding off it, so the write is allowed exactly as before.
    let (manifest_code, manifest_stderr) = common::run_guard(
        &root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\".claude/settings.json\"}}",
    );
    assert_eq!(
        manifest_code,
        Some(0),
        "a manifest locus is the manifest arm's, not the locus binding's, got: {manifest_stderr}"
    );
    assert!(manifest_stderr.is_empty());

    // `memory` governs `.` with `**/CLAUDE.md`, and the guard has no ignore reader where
    // discovery prunes by the repo's ignore rules — so the `.`-rooted locus is excluded
    // by construction rather than judging every CLAUDE.md in the tree.
    let (memory_code, memory_stderr) = common::run_guard(
        &root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\"vendor/dep/CLAUDE.md\"}}",
    );
    assert_eq!(
        memory_code,
        Some(0),
        "a `.`-rooted locus is excluded, got: {memory_stderr}"
    );
    assert!(memory_stderr.is_empty());

    // No lock at all: the `.claude/` fallback is untouched, and it is a substring match
    // on `.claude/`, so it binds `settings.local.json` too — "never bound" is a claim
    // about the locus binding under a lock, never about the fallback.
    let bare_root = common::tmpdir("guard-locus-no-lock");
    fs::create_dir_all(&bare_root).unwrap();
    for bound in [".claude/rules/stray.md", ".claude/settings.local.json"] {
        let (code, stderr) = common::run_guard(
            &bare_root,
            &format!("{{\"tool_name\":\"Write\",\"tool_input\":{{\"file_path\":\"{bound}\"}}}}"),
        );
        assert_eq!(code, Some(0), "the no-lock default mode is warn ({bound})");
        assert!(
            stderr.contains("temper-managed projection"),
            "the no-lock fallback binds any `.claude/` path ({bound}), got: {stderr}"
        );
    }
    let (outside_code, outside_stderr) = common::run_guard(
        &bare_root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\"specs/intent.md\"}}",
    );
    assert_eq!(outside_code, Some(0));
    assert!(
        outside_stderr.is_empty(),
        "with no lock nothing outside `.claude/` binds, got: {outside_stderr}"
    );
}

/// .claude/settings.json is composed from registration-member kinds (hook,
/// installed-plugin, known-marketplace) and becomes emit-owned when any of
/// them exist in the lock. The guard must bind writes to it, not silently allow them.
#[test]
fn guard_binds_settings_json_when_registration_members_compose() {
    let root = common::tmpdir("guard-settings-json-emit-owned");
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();

    // A lock with `block` mode and a hook member (which composes into settings.json).
    fs::write(
        temper_dir.join("lock.toml"),
        "[[declaration.assembly]]\nfact = \"mode\"\nvalue = \"block\"\n\n[[declaration.registration]]\nkind = \"hook\"\nkey = \"SessionStart\"\nmanifest = \"settings.json\"\nkey_path = \"hooks.<Event>\"\n"
    )
    .unwrap();

    // A pending write to .claude/settings.json should be bound by the guard
    // (not silently allowed), since the hook member composes into it.
    let (code, stderr) = common::run_guard(
        &root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\".claude/settings.json\"}}",
    );
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
    let warn_temper_dir = warn_root.join(".temper");
    fs::create_dir_all(&warn_temper_dir).unwrap();
    fs::write(
        warn_temper_dir.join("lock.toml"),
        "[[declaration.assembly]]\nfact = \"mode\"\nvalue = \"warn\"\n\n[[declaration.registration]]\nkind = \"hook\"\nkey = \"SessionStart\"\nmanifest = \"settings.json\"\nkey_path = \"hooks.<Event>\"\n"
    )
    .unwrap();

    let (warn_code, warn_stderr) = common::run_guard(
        &warn_root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\".claude/settings.json\"}}",
    );
    assert_eq!(
        warn_code,
        Some(0),
        "warn mode allows the write but surfaces the finding"
    );
    assert!(
        warn_stderr.contains("temper-managed projection"),
        "the warning must be in-band"
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
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();

    // A lock with `block` mode and the root `CLAUDE.md` as a projected member
    // (the memory kind's single-segment projection).
    fs::write(
        temper_dir.join("lock.toml"),
        "[[declaration.assembly]]\nfact = \"mode\"\nvalue = \"block\"\n\n[[memory]]\nname = \"root\"\nsource_path = \"CLAUDE.md\"\nsource_hash = \"abc\"\nemit_hash = \"abc\"\n"
    )
    .unwrap();

    let payload_for = |path: &Path| {
        serde_json::json!({
            "tool_name": "Write",
            "tool_input": { "file_path": path.to_string_lossy().as_ref() }
        })
        .to_string()
    };

    // A write to the actual root CLAUDE.md projection (absolute path) should be blocked.
    let (code, stderr) = common::run_guard(&root, &payload_for(&root.join("CLAUDE.md")));
    assert_eq!(
        code,
        Some(2),
        "the actual CLAUDE.md projection (absolute path) should be blocked"
    );
    assert!(stderr.contains("temper-managed projection"));

    // Every other path ending in `CLAUDE.md` is a different file, not the projection:
    // neither one deeper in the tree nor one a single segment over may be blocked.
    for unrelated in [".temper/memory/CLAUDE.md", "reference/CLAUDE.md"] {
        let (code, stderr) = common::run_guard(&root, &payload_for(&root.join(unrelated)));
        assert_eq!(
            code,
            Some(0),
            "`{unrelated}` is not the declared projection and must be allowed"
        );
        assert!(stderr.is_empty(), "and surfaces nothing: {stderr}");
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
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(
        temper_dir.join("lock.toml"),
        format!(
            "[[declaration.assembly]]\nfact = \"mode\"\nvalue = \"block\"\n\n\
             [[memory]]\nname = \"root\"\nsource_path = \"CLAUDE.md\"\nsource_hash = \"abc\"\nemit_hash = \"abc\"\n\n\
             {CLAUDE_WRITE_LOCK_ROW}"
        ),
    )
    .unwrap();

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
        // Undeclared paths, absolute — including one whose bare name matches a projection.
        (
            root.join("reference/CLAUDE.md")
                .to_string_lossy()
                .into_owned(),
            Some(0),
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
        let payload = serde_json::json!({
            "tool_name": "Write",
            "tool_input": { "file_path": &file_path }
        })
        .to_string();
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

/// A `PreToolUse` `Write` payload landing whole-file `content` at `file_path` — the shape
/// the manifest guard reads a pending manifest's members off (a partial `Edit` carries no
/// full manifest, so the guard checks only a whole-file write).
fn write_payload(file_path: &str, content: &str) -> String {
    serde_json::json!({
        "tool_name": "Write",
        "tool_input": { "file_path": file_path, "content": content },
    })
    .to_string()
}

/// A `block` harness whose lock also declares the [`CLAUDE_WRITE_PAYLOAD`] target an
/// emit-owned projection — so one lock exercises both bindings the guard now runs.
fn manifest_guard_harness(slug: &str) -> PathBuf {
    let root = common::tmpdir(slug);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(
        temper_dir.join("lock.toml"),
        format!("[[declaration.assembly]]\nfact = \"mode\"\nvalue = \"block\"\n\n{CLAUDE_WRITE_LOCK_ROW}"),
    )
    .unwrap();
    root
}

#[test]
fn guard_flags_a_represented_manifest_whose_member_violates_its_contract() {
    let root = manifest_guard_harness("guard-manifest-block");

    // An `.mcp.json` write whose `gmail` server declares an undocumented transport violates
    // the `mcp-server` contract's `type` enum — flagged, and under `block` the write is denied.
    let (code, stderr) = common::run_guard(
        &root,
        &write_payload(
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
        &write_payload(
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
    let (proj_code, proj_stderr) = common::run_guard(&root, CLAUDE_WRITE_PAYLOAD);
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
        let temper_dir = root.join(".temper");
        fs::create_dir_all(&temper_dir).unwrap();
        fs::write(
            temper_dir.join("lock.toml"),
            format!("[[declaration.assembly]]\nfact = \"mode\"\nvalue = \"{mode}\"\n"),
        )
        .unwrap();

        let (code, stderr) = common::run_guard(
            &root,
            &write_payload(
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
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();

    // A lock with `block` mode declaring an MCP server and a hook member.
    fs::write(
        temper_dir.join("lock.toml"),
        "[[declaration.assembly]]\nfact = \"mode\"\nvalue = \"block\"\n\n\
         [[declaration.registration]]\nkind = \"mcp-server\"\nkey = \"gmail\"\nmanifest = \".mcp.json\"\nkey_path = \"mcpServers.*\"\n\n\
         [[declaration.registration]]\nkind = \"hook\"\nkey = \"SessionStart\"\nmanifest = \"settings.json\"\nkey_path = \"hooks.<Event>\"\n"
    )
    .unwrap();

    // A write to `.mcp.json` that omits the `gmail` server (empty mcpServers object)
    // must be flagged, even though what's there parses correctly.
    let (code, stderr) =
        common::run_guard(&root, &write_payload(".mcp.json", r#"{"mcpServers":{}}"#));
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
    let warn_temper_dir = warn_root.join(".temper");
    fs::create_dir_all(&warn_temper_dir).unwrap();
    fs::write(
        warn_temper_dir.join("lock.toml"),
        "[[declaration.assembly]]\nfact = \"mode\"\nvalue = \"warn\"\n\n\
         [[declaration.registration]]\nkind = \"mcp-server\"\nkey = \"gmail\"\nmanifest = \".mcp.json\"\nkey_path = \"mcpServers.*\"\n"
    )
    .unwrap();

    let (warn_code, warn_stderr) = common::run_guard(
        &warn_root,
        &write_payload(".mcp.json", r#"{"mcpServers":{}}"#),
    );
    assert_eq!(
        warn_code,
        Some(0),
        "warn mode allows the write but surfaces the finding"
    );
    assert!(
        warn_stderr.contains("lock declares member"),
        "the warning must be in-band"
    );
}

/// A `PreToolUse` `Edit` payload — the partial shape carrying replacement strings rather
/// than the whole file, which the guard reconstructs against the on-disk manifest.
fn edit_payload(file_path: &str, old_string: &str, new_string: &str) -> String {
    serde_json::json!({
        "tool_name": "Edit",
        "tool_input": {
            "file_path": file_path,
            "old_string": old_string,
            "new_string": new_string,
        },
    })
    .to_string()
}

/// A `block` harness whose lock declares the `SessionStart` hook member — which makes
/// `.claude/settings.json` both a represented manifest and an emit-owned projection, the
/// exact overlap an `Edit` to it has to be judged under.
fn settings_manifest_harness(slug: &str, settings: &str) -> PathBuf {
    let root = common::tmpdir(slug);
    let temper_dir = root.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(
        temper_dir.join("lock.toml"),
        "[[declaration.assembly]]\nfact = \"mode\"\nvalue = \"block\"\n\n\
         [[declaration.registration]]\nkind = \"hook\"\nkey = \"SessionStart\"\nmanifest = \"settings.json\"\nkey_path = \"hooks.<Event>\"\n",
    )
    .unwrap();
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
        &edit_payload(
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
        &edit_payload(
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
        &edit_payload(
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
    let temper_dir = bare.join(".temper");
    fs::create_dir_all(&temper_dir).unwrap();
    fs::write(
        temper_dir.join("lock.toml"),
        "[[declaration.assembly]]\nfact = \"mode\"\nvalue = \"block\"\n\n\
         [[declaration.registration]]\nkind = \"hook\"\nkey = \"SessionStart\"\nmanifest = \"settings.json\"\nkey_path = \"hooks.<Event>\"\n",
    )
    .unwrap();

    let (absent_code, absent_stderr) = common::run_guard(
        &bare,
        &edit_payload(".claude/settings.json", "\"a\": 1", "\"a\": 2"),
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

/// A `block` harness's lock is one line different per enforcement mode — the manifest
/// binding's verdict is a function of the mode alone, so the unparseable case exercises
/// all three off one lock body.
fn settings_manifest_lock(mode: &str) -> String {
    format!(
        "[[declaration.assembly]]\nfact = \"mode\"\nvalue = \"{mode}\"\n\n\
         [[declaration.registration]]\nkind = \"hook\"\nkey = \"SessionStart\"\nmanifest = \"settings.json\"\nkey_path = \"hooks.<Event>\"\n"
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
        &write_payload(".claude/settings.json", UNPARSEABLE_SETTINGS),
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
        let temper_dir = root.join(".temper");
        fs::create_dir_all(&temper_dir).unwrap();
        fs::write(temper_dir.join("lock.toml"), settings_manifest_lock(mode)).unwrap();
        common::write_settings(&root, CO_OWNED_SETTINGS);

        let (code, stderr) = common::run_guard(
            &root,
            &write_payload(".claude/settings.json", UNPARSEABLE_SETTINGS),
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
        &write_payload(".claude/settings.json", CO_OWNED_SETTINGS),
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
        &edit_payload(
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
            teardown: false,
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
    common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));

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
    common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));
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
    common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));

    // Convert all projections to CRLF.
    convert_tree_to_crlf(&root);

    let discovery = install::discover(&root).unwrap();
    install::run(&root, &discovery, Represent::Yes, false).unwrap();

    // gate_installed must be quiet (no stale config drift).
    assert!(
        temper::drift::config_stale(&temper_dir).is_empty(),
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
        skill_md.contains("# temper: managed projection"),
        "skill projection must retain managed-by note on CRLF"
    );

    let rule_md = fs::read_to_string(root.join(".claude").join("rules").join("rust.md")).unwrap();
    assert!(
        rule_md.contains("# temper: managed projection"),
        "rule projection must retain managed-by note on CRLF"
    );

    // The frontmatterless rule should have the banner instead.
    let collab_md =
        fs::read_to_string(root.join(".claude").join("rules").join("collaboration.md")).unwrap();
    assert!(
        collab_md.contains("<!-- temper: managed projection"),
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
    common::vendor_sdk(&lf_temper_dir.join("node_modules").join("@dtmd"));

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
    common::vendor_sdk(&crlf_temper_dir.join("node_modules").join("@dtmd"));

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
        temper::drift::config_stale(&lf_temper_dir).is_empty(),
        "LF harness gate must be clean"
    );
    assert!(
        temper::drift::config_stale(&crlf_temper_dir).is_empty(),
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
        common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));

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
            skill_after_install.contains("# temper: managed projection"),
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
            skill_after_emit.contains("# temper: managed projection"),
            "skill must retain managed-by note after standalone emit (LF)"
        );
        assert!(
            skill_after_emit.contains("Updated by author."),
            "skill must carry the authored edit after emit (LF)"
        );

        // Gate must be quiet (no drift).
        assert!(
            temper::drift::config_stale(&temper_dir).is_empty(),
            "gate must report no drift after install→edit→emit (LF)"
        );
    };

    let test_crlf = |name: &str| {
        let root = write_harness(name, false);
        convert_tree_to_crlf(&root);

        let temper_dir = root.join(".temper");
        fs::create_dir_all(&temper_dir).unwrap();
        common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));

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
            skill_after_install.contains("# temper: managed projection"),
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
            skill_after_emit.contains("# temper: managed projection"),
            "skill must retain managed-by note after standalone emit (CRLF)"
        );
        assert!(
            skill_after_emit.contains("Updated by author."),
            "skill must carry the authored edit after emit (CRLF)"
        );

        // Gate must be quiet (no drift).
        assert!(
            temper::drift::config_stale(&temper_dir).is_empty(),
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
    let payload = r#"{"tool_name":"Write","tool_input":{"file_path":".claude/test.md"}}"#;

    // Run the command through a shell with an empty PATH so temper cannot be found.
    let cmd = temper::install::GUARD_COMMAND;
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(format!("echo '{payload}' | {cmd}"))
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
