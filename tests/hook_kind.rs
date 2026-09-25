//! The `hook` built-in kind: a `settings.json` `hooks.<Event>` registration member
//! (`specs/builtins.md`, "The coverage bar"; 0021, "the manifest authoring surface").
//!
//! The first manifest kind — fields-only, discovered off the `.claude/settings.json`
//! manifest at the `hooks.<Event>` collection address and read through the JSON manifest
//! adapter, never a file tree of its own. Driven over fixtures mirroring the real Claude
//! Code layout (`.claude/settings.json`, per `.claude/rules/rust.md`): the read that turns
//! a `hooks.<Event>` entry into a member, and the shipped default contract firing on a
//! broken hook and passing a clean one, end to end through the `check --harness` gate.
//!
//! The contract spans both levels the member covers, because the read flattens both onto
//! it: the lifecycle event carried off the collection key, and the handler's own schema —
//! its `type`, and the fields the documented table marks required for that `type`, gated
//! by one `when` guard per handler kind.

mod common;

use std::fs;

use common::{check_harness, write_rule, write_settings};

use temper::builtin_kind;
use temper::builtin_lock;
use temper::kind::{CollectionAddress, CollectionKeyPath, Content, Registration};

/// A `.claude/settings.json` carrying two hooks — one under the documented `PreToolUse`
/// event, one under an undocumented `NotARealEvent` — in the real Claude Code shape (the
/// event maps to an array of matcher groups, each carrying a `hooks` handler list).
const BROKEN_SETTINGS: &str = r#"{
  "hooks": {
    "PreToolUse": [
      { "matcher": "Bash", "hooks": [ { "type": "command", "command": "echo guard" } ] }
    ],
    "NotARealEvent": [
      { "hooks": [ { "type": "command", "command": "echo nope" } ] }
    ]
  }
}"#;

/// A `.claude/settings.json` whose two hooks both key under documented events.
const CLEAN_SETTINGS: &str = r#"{
  "hooks": {
    "PreToolUse": [
      { "matcher": "Bash", "hooks": [ { "type": "command", "command": "echo guard" } ] }
    ],
    "SessionStart": [
      { "hooks": [ { "type": "command", "command": "echo hello" } ] }
    ]
  }
}"#;

/// A `.claude/settings.json` keying under the three events the docs grew after the
/// allowlist's first retrieval — `DirectoryAdded` (a mid-session `/add-dir`),
/// `PreModelSwitch` and `PostModelSwitch` (code.claude.com/docs/en/hooks, "Hook events",
/// retrieved 2026-09-25). Documented events, so the clause has nothing to say about them.
const NEWLY_DOCUMENTED_SETTINGS: &str = r#"{
  "hooks": {
    "DirectoryAdded": [
      { "hooks": [ { "type": "command", "command": "echo added" } ] }
    ],
    "PreModelSwitch": [
      { "hooks": [ { "type": "command", "command": "echo before" } ] }
    ],
    "PostModelSwitch": [
      { "hooks": [ { "type": "command", "command": "echo after" } ] }
    ]
  }
}"#;

/// A `.claude/settings.json` whose four hooks all key under documented events and each
/// break their handler's own documented schema: a `command` handler with nothing to run,
/// an `http` handler with nowhere to POST, a handler naming a `type` outside the
/// documented five, and one naming no `type` at all — the common-fields table marks it
/// required and gives it no default (code.claude.com/docs/en/hooks, "Hook handler
/// fields"/"Common fields", retrieved 2026-09-25).
const BROKEN_HANDLER_SETTINGS: &str = r#"{
  "hooks": {
    "PreToolUse": [
      { "matcher": "Bash", "hooks": [ { "type": "command" } ] }
    ],
    "PostToolUse": [
      { "matcher": "Write", "hooks": [ { "type": "http" } ] }
    ],
    "Stop": [
      { "hooks": [ { "type": "webhook", "url": "https://example.test/stop" } ] }
    ],
    "SessionStart": [
      { "hooks": [ { "command": "echo untyped" } ] }
    ]
  }
}"#;

/// A `.claude/settings.json` carrying one well-formed handler of every documented kind —
/// five members, since a member is a handler and not a matcher group. Each carries exactly
/// the fields its kind's table marks required (same source).
const EVERY_HANDLER_KIND_SETTINGS: &str = r#"{
  "hooks": {
    "PreToolUse": [
      { "matcher": "Bash", "hooks": [ { "type": "command", "command": "echo guard" } ] }
    ],
    "PostToolUse": [
      { "matcher": "Write", "hooks": [
        { "type": "http", "url": "https://example.test/audit" },
        { "type": "mcp_tool", "server": "my_server", "tool": "security_scan" }
      ] }
    ],
    "Stop": [
      { "hooks": [
        { "type": "prompt", "prompt": "Did the work finish?" },
        { "type": "agent", "prompt": "Verify the tree is clean." }
      ] }
    ]
  }
}"#;

/// Every finding line a `hook` clause raised, at any depth — a guarded clause reports under
/// its own body address (`hook.when.type=command.required.command`), so a prefix read is
/// what makes "no clause fired" a claim over the whole contract rather than over one label.
fn hook_findings(findings: &[String]) -> Vec<&String> {
    findings
        .iter()
        .filter(|line| line.contains("title=hook."))
        .collect()
}

fn hook_kind() -> temper::kind::CustomKind {
    builtin_kind::definition("hook").expect("hook is embedded")
}

#[test]
fn the_hook_kind_is_a_fields_only_manifest_kind_at_the_hooks_collection_address() {
    let hook = hook_kind();
    assert_eq!(hook.content, Content::Fields);
    assert_eq!(
        hook.collection_address,
        Some(CollectionAddress {
            manifest: "settings.json".to_string(),
            key_path: CollectionKeyPath::HooksEvent,
            entry_shape: temper::kind::EntryShape::GroupArray {
                member_key: "hooks".to_string(),
                lifted_fields: vec!["matcher".to_string()],
            },
        })
    );
    assert_eq!(
        hook.registration,
        vec![Registration::Event {
            field: "event".to_string()
        }]
    );
}

#[test]
fn a_settings_json_hooks_event_entry_reads_as_a_hook_member() {
    let harness = common::tmpdir("read-hook-members");
    write_settings(&harness, BROKEN_SETTINGS);

    let reads = common::manifest_members(&harness, &hook_kind());
    assert_eq!(
        reads.len(),
        1,
        "the one settings.json manifest is read once"
    );

    // One member per `hooks.<Event>` entry, keyed by its lifecycle event, in the
    // collection's own sorted key order — each surfacing in the `hooks` collection.
    let members: Vec<(&str, &str)> = reads[0]
        .members
        .iter()
        .map(|m| (m.collection.as_str(), m.key.as_str()))
        .collect();
    assert_eq!(
        members,
        vec![("hooks", "NotARealEvent"), ("hooks", "PreToolUse")]
    );
    // A `hooks.<Event>` value is Claude Code's array of matcher groups: the read decomposes
    // each handler into the flat {matcher?, type, command} fields the write face re-nests,
    // so a hook member carries its handler's own fields plus the group's `matcher` when one
    // is present. The tool-scoped `PreToolUse` lifts its `matcher`; the matcher-less
    // `NotARealEvent` carries only the handler's own.
    let pre = &reads[0].members[1];
    assert_eq!(pre.key, "PreToolUse");
    assert_eq!(pre.fields.get("matcher"), Some(&serde_json::json!("Bash")));
    assert_eq!(
        pre.fields.get("command"),
        Some(&serde_json::json!("echo guard"))
    );
    assert_eq!(pre.fields.get("type"), Some(&serde_json::json!("command")));
    assert!(!reads[0].members[0].fields.contains_key("matcher"));
}

#[test]
fn an_unrepresented_settings_json_still_infers_its_hook_members() {
    // The read is driven by the address, not by the manifest being modeled as a member:
    // a settings.json temper carries no representation for still surfaces its hooks.
    let harness = common::tmpdir("infer-unrepresented");
    write_settings(&harness, CLEAN_SETTINGS);

    let reads = common::manifest_members(&harness, &hook_kind());
    assert_eq!(reads[0].members.len(), 2);
    // The `hooks` collection is consumed into members, never left as an opaque field.
    assert!(!reads[0].opaque_fields.contains_key("hooks"));
}

#[test]
fn the_hook_default_contract_fires_on_an_undocumented_event() {
    let harness = common::tmpdir("hook-broken-event");
    write_settings(&harness, BROKEN_SETTINGS);

    let (findings, ok) = check_harness(&harness);

    // The strictest-documented-profile clause fires exactly once — on the undocumented
    // `NotARealEvent` hook, never on the documented `PreToolUse` one.
    let fired = common::findings_for(&findings, "hook.enum.event");
    assert_eq!(
        fired.len(),
        1,
        "exactly the undocumented event fires the clause, got: {findings:#?}"
    );
    assert!(
        fired[0].contains("NotARealEvent"),
        "the finding names the undocumented event, got: {}",
        fired[0]
    );
    assert!(
        !ok,
        "an undocumented event is a required-severity finding — the run fails, got: {findings:#?}"
    );
}

#[test]
fn the_hook_default_contract_passes_documented_events() {
    let harness = common::tmpdir("hook-clean-events");
    write_settings(&harness, CLEAN_SETTINGS);

    let (findings, _ok) = check_harness(&harness);

    // The vacuity pin: both `CLEAN_SETTINGS` hooks were read as members and judged, so the
    // silence below is a verdict and not an empty selection (an empty harness reports
    // `hook (0)` and emits the same zero findings).
    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(
        checked.len(),
        1,
        "expected exactly one checked summary, got: {findings:#?}"
    );
    assert!(
        checked[0].contains("hook (2)"),
        "both documented-event hooks are checked, got: {}",
        checked[0]
    );
    assert!(
        common::findings_for(&findings, "hook.enum.event").is_empty(),
        "every documented event passes the clause, got: {findings:#?}"
    );
}

#[test]
fn the_hook_default_contract_passes_events_the_docs_added_after_the_first_retrieval() {
    // The allowlist is an external fact with a retrieval date, so it goes stale under the
    // page it cites: an event documented *after* that date is dead configuration by the
    // clause's reckoning and live configuration in fact. A required-severity clause over a
    // stale allowlist forges a finding on a correct harness — the false positive
    // `specs/intent.md` invariant 2 names as how a gate gets disabled — so the three events
    // the page grew are pinned here, not merely added to the array.
    let harness = common::tmpdir("hook-newly-documented-events");
    write_settings(&harness, NEWLY_DOCUMENTED_SETTINGS);

    let (findings, ok) = check_harness(&harness);

    // The vacuity pin: all three hooks were read as members and judged, so the silence
    // below is a verdict and not an empty selection.
    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(
        checked.len(),
        1,
        "expected exactly one checked summary, got: {findings:#?}"
    );
    assert!(
        checked[0].contains("hook (3)"),
        "all three newly-documented-event hooks are checked, got: {}",
        checked[0]
    );
    assert!(
        common::findings_for(&findings, "hook.enum.event").is_empty(),
        "an event the docs document passes the clause, got: {findings:#?}"
    );
    assert!(
        ok,
        "a harness registering only documented events gates clean, got: {findings:#?}"
    );
}

#[test]
fn the_hook_default_contract_fires_on_a_handler_breaking_its_kinds_documented_schema() {
    let harness = common::tmpdir("hook-broken-handler");
    write_settings(&harness, BROKEN_HANDLER_SETTINGS);

    let (findings, ok) = check_harness(&harness);

    // The vacuity pin: all four hooks were read as members and judged, so each count
    // below is a verdict over a member and not an empty selection.
    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(
        checked.len(),
        1,
        "expected exactly one checked summary, got: {findings:#?}"
    );
    assert!(
        checked[0].contains("hook (4)"),
        "all four broken-handler hooks are checked, got: {}",
        checked[0]
    );

    // A `command` handler with nothing to run fires its own guard's body, and only it —
    // the guard is keyed on `type`, so the `http` and `webhook` members never enter it.
    let no_command = common::findings_for(&findings, "hook.when.type=command.required.command");
    assert_eq!(
        no_command.len(),
        1,
        "exactly the command handler missing its `command` fires, got: {findings:#?}"
    );

    // An `http` handler with nowhere to POST fires its own.
    let no_url = common::findings_for(&findings, "hook.when.type=http.required.url");
    assert_eq!(
        no_url.len(),
        1,
        "exactly the http handler missing its `url` fires, got: {findings:#?}"
    );

    // A `type` outside the documented five fires the enum — and no guard, since no guard
    // ranges over a value the allowlist refuses.
    let bad_type = common::findings_for(&findings, "hook.enum.type");
    assert_eq!(
        bad_type.len(),
        1,
        "exactly the undocumented handler type fires the enum, got: {findings:#?}"
    );
    assert!(
        bad_type[0].contains("webhook"),
        "the finding names the undocumented handler type, got: {}",
        bad_type[0]
    );

    // A handler naming no `type` fires the presence clause — and only it. The enum stays
    // silent: an allowlist refuses values, and an absent field offers none. `type` is
    // documented required with no default, so this is the one break the enum beside it
    // could never see.
    let no_type = common::findings_for(&findings, "hook.required.type");
    assert_eq!(
        no_type.len(),
        1,
        "exactly the handler naming no `type` fires the presence clause, got: {findings:#?}"
    );
    assert_eq!(
        bad_type.len(),
        1,
        "the absent `type` adds nothing to the enum's count, got: {findings:#?}"
    );

    // The lifecycle-event clause stays silent: all three key under documented events, so
    // the handler half fires on its own evidence and not on the event half's.
    assert!(
        common::findings_for(&findings, "hook.enum.event").is_empty(),
        "every event here is documented, got: {findings:#?}"
    );
    assert!(
        !ok,
        "a handler breaking its documented schema is a required-severity finding — the run fails, got: {findings:#?}"
    );
}

#[test]
fn the_hook_default_contract_passes_a_well_formed_handler_of_every_documented_kind() {
    let harness = common::tmpdir("hook-every-handler-kind");
    write_settings(&harness, EVERY_HANDLER_KIND_SETTINGS);

    let (findings, ok) = check_harness(&harness);

    // The vacuity pin: one member per handler, five handlers, so the silence below is a
    // verdict passed by every documented kind — including the two that share a guard.
    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(
        checked.len(),
        1,
        "expected exactly one checked summary, got: {findings:#?}"
    );
    assert!(
        checked[0].contains("hook (5)"),
        "one member per documented handler kind is checked, got: {}",
        checked[0]
    );

    assert!(
        hook_findings(&findings).is_empty(),
        "a well-formed handler of every documented kind passes the whole contract, got: {findings:#?}"
    );
    assert!(
        ok,
        "a harness whose every handler is well-formed gates clean, got: {findings:#?}"
    );
}

#[test]
fn the_hook_kind_checks_its_segment_while_the_container_governs_the_whole_file() {
    // The hook kind governs the `hooks` segment of `settings.json` and never the file:
    // whole-file governance is the `settings` container's, and it shipping does not absorb
    // the segment — the registration members are still discovered and checked as hooks,
    // beside the one container member. With every segment of the file covered, nothing is
    // left ungoverned to flag. (How a *partial* verdict reads when no container kind is in
    // scope is `tests/coverage_note.rs`'s case, not this kind's.)
    let harness = common::tmpdir("settings-container-governed");
    write_settings(
        &harness,
        r#"{
  "permissions": { "allow": ["Bash(git status)"] },
  "hooks": {
    "PreToolUse": [
      { "matcher": "Bash", "hooks": [ { "type": "command", "command": "echo guard" } ] }
    ]
  }
}"#,
    );

    let (findings, _ok) = check_harness(&harness);

    let unmodeled = common::findings_for(&findings, "coverage.unmodeled-surface");
    assert!(
        unmodeled
            .iter()
            .all(|line| !line.contains(".claude/settings.json")),
        "the settings container governs the file whole, got: {findings:#?}"
    );
    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(
        checked.len(),
        1,
        "expected exactly one checked summary, got: {findings:#?}"
    );
    assert!(
        checked[0].contains("hook (1)") && checked[0].contains("settings (1)"),
        "the hook member is checked beside the container member, got: {}",
        checked[0]
    );
}

#[test]
fn the_embedded_builtin_lock_carries_the_hook_kind_and_its_event_clause() {
    // The embedded side of the built-in lock (`tests/builtin_lock_frozen.rs` pins its
    // byte-equality with the SDK module's own memberless emit): the hook kind fact and
    // its one enum clause are present, so SDK-less checking ships the same hook contract.
    let declarations = builtin_lock::declarations();

    let hook = declarations
        .kinds
        .iter()
        .find(|k| k.name == "hook")
        .expect("the hook kind fact is embedded");
    assert_eq!(hook.shape.as_deref(), Some("fields"));
    let address = hook
        .collection_address
        .as_ref()
        .expect("the hook kind carries its collection address");
    assert_eq!(address.manifest, "settings.json");
    assert_eq!(address.key_path, "hooks.<Event>");

    let clause = declarations
        .clauses
        .iter()
        .find(|c| c.kind.as_deref() == Some("hook") && c.predicate == "enum")
        .expect("the hook default contract's event clause is embedded");
    assert_eq!(clause.field.as_deref(), Some("event"));
    let values = clause.values.as_ref().expect("the enum carries its values");
    assert!(
        values.iter().any(|v| v == "PreToolUse"),
        "the documented-event allowlist carries the events, got: {values:?}"
    );
}

#[test]
fn two_kinds_at_the_same_collection_address_trip_collision_loud() {
    // Regression: two fields-only kinds declaring the same collectionAddress are
    // accepted silently, the corpus unions their selections and cross-applies contracts.
    // The check gate must refuse the collision loud with a named admissibility finding.
    // The case is rooted decidably: the fixture harness is a *child* of the temp dir,
    // and a real `rule` member is planted in that parent. The gate discovers it only if
    // the run roots above the fixture — so `rule (0)` below pins the run's corpus root
    // to the fixture itself.
    let parent = common::tmpdir("collection-address-collision");
    let harness = parent.join("collision");
    write_rule(&parent, "planted");

    // Write a lock that declares two kinds at the same collection address. The first is
    // the built-in `hook` kind at `settings.json#hooks.<Event>`; the second is a custom
    // kind also at that same address.
    let lock = r#"[declaration]

[[declaration.kind]]
name = "hook"
shape = "fields"
collection_address = { manifest = "settings.json", key_path = "hooks.<Event>", entry_shape = "group-array(hooks;matcher)" }

[[declaration.kind]]
name = "custom_hook"
shape = "fields"
collection_address = { manifest = "settings.json", key_path = "hooks.<Event>" }
"#;
    let workspace = harness.join(".temper");
    fs::create_dir_all(&workspace).expect("create workspace");
    fs::write(workspace.join("lock.toml"), lock).expect("write lock");

    let (findings, ok) = check_harness(&harness);

    // The collision finding fires exactly once, naming both kinds and the address.
    let collisions = common::findings_for(&findings, "kind.collection-address-collision");
    assert_eq!(
        collisions.len(),
        1,
        "exactly one collection-address-collision finding, got: {findings:#?}"
    );
    let collision = &collisions[0];
    assert!(
        collision.contains("hook"),
        "the collision names the built-in hook kind, got: {collision}"
    );
    assert!(
        collision.contains("custom_hook"),
        "the collision names the custom kind, got: {collision}"
    );
    assert!(
        collision.contains("settings.json"),
        "the collision names the manifest, got: {collision}"
    );
    assert!(
        collision.contains("hooks"),
        "the collision references the hooks key path, got: {collision}"
    );
    assert!(
        !ok,
        "a collection-address collision is a required-severity finding — the run fails, got: {findings:#?}"
    );

    // The gate walked the fixture, never the directory enclosing it: the rule planted
    // beside the fixture is not in the checked corpus.
    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(
        checked.len(),
        1,
        "expected exactly one checked summary, got: {findings:#?}"
    );
    assert!(
        checked[0].contains("rule (0)"),
        "the run gates the fixture, so the rule planted in its parent is never discovered, got: {}",
        checked[0]
    );
}
