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
const NON_CANONICAL_SETTINGS_WITH_HOOK: &str = "{\n    \"zeta\": \"first\",\n    \"hooks\": {\n        \"SessionStart\": [\n            { \"hooks\": [ { \"type\": \"command\", \"command\": \"temper check . --reporter session-start\" } ] }\n        ]\n    }\n}\n";

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
        "temper check . --reporter session-start",
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
    // byte-faithful body alone, already matching its hand-authored source;
    // `rust`/`coordinate` declare fields, so their frontmatter is rewritten
    // into canonical form and the projection changes.
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
        temper::drift::EmitOutcome::Unchanged
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
        "temper guard ."
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
    common::vendor_sdk(&temper_dir.join("node_modules").join("@dtmd"));
    install::run(&root, &discovery, Represent::Yes, false).unwrap();
    assert!(
        install::gate_installed(&root).is_empty(),
        "got: {:?}",
        install::gate_installed(&root)
    );
}

// ---------------------------------------------------------------------------
// guard — the lock, not the retired manifest, grounds the enforcement mode
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

    let (code, stderr) = run_guard(&root, CLAUDE_WRITE_PAYLOAD);
    assert_eq!(code, Some(2), "the lock's `block` mode must block");
    assert!(stderr.contains("other tools writes are not bound by it"));
}

/// With no `lock.toml` at all there is no declared projection set to consult — unlike
/// a represented harness (below), the guard falls back to binding any `.claude/` write
/// at the default enforcement mode rather than silently allowing everything: absent evidence
/// must never *suppress* a guard claim, only ever fail to forge one.
#[test]
fn guard_defaults_to_warn_when_the_lock_is_absent() {
    let root = common::tmpdir("lock-mode-absent");
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
    let (code, stderr) = run_guard(
        &root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\".rules/safety.md\"}}",
    );
    assert_eq!(
        code,
        Some(2),
        "a declared-locus target outside .claude/ must be bound (block mode)"
    );
    assert!(stderr.contains("temper-managed projection"));

    // A write to a `.claude/` path with no corresponding declared target should
    // still be allowed (the fallback check only applies when no targets exist).
    let (allow_code, allow_stderr) = run_guard(
        &root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\".claude/skills/x/SKILL.md\"}}",
    );
    assert_eq!(
        allow_code,
        Some(0),
        "an undeclared .claude/ path is allowed when targets exist"
    );
    assert!(allow_stderr.is_empty());

    // A write to an entirely different path should be allowed.
    let (other_code, other_stderr) = run_guard(
        &root,
        "{\"tool_name\":\"Write\",\"tool_input\":{\"file_path\":\"src/main.rs\"}}",
    );
    assert_eq!(other_code, Some(0));
    assert!(other_stderr.is_empty());
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
    let (code, stderr) = run_guard(
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
    let (ok_code, ok_stderr) = run_guard(
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
    let (proj_code, proj_stderr) = run_guard(&root, CLAUDE_WRITE_PAYLOAD);
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

        let (code, stderr) = run_guard(
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
