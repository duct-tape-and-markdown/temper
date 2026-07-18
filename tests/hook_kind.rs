//! The `hook` built-in kind: a `settings.json` `hooks.<Event>` registration member
//! (`specs/builtins.md`, "The coverage bar"; 0021, "the manifest authoring surface").
//!
//! The first manifest kind — fields-only, discovered off the `.claude/settings.json`
//! manifest at the `hooks.<Event>` collection address and read through the JSON manifest
//! adapter, never a file tree of its own. Driven over fixtures mirroring the real Claude
//! Code layout (`.claude/settings.json`, per `.claude/rules/rust.md`): the read that turns
//! a `hooks.<Event>` entry into a member, and the shipped default contract's one decidable
//! clause — the event must be a documented lifecycle event — firing on a broken hook and
//! passing a clean one, end to end through the `check --harness` gate.

mod common;

use common::{check_harness, write_settings};

use temper::builtin_kind;
use temper::builtin_lock;
use temper::json_manifest::Manifest;
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

fn hook_kind() -> temper::kind::CustomKind {
    builtin_kind::definition("hook")
        .unwrap()
        .expect("hook is embedded")
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

    let reads =
        Manifest::read_kind(&temper::import::Discovery::new(&harness), &hook_kind()).unwrap();
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

    let reads =
        Manifest::read_kind(&temper::import::Discovery::new(&harness), &hook_kind()).unwrap();
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

    assert!(
        common::findings_for(&findings, "hook.enum.event").is_empty(),
        "every documented event passes the clause, got: {findings:#?}"
    );
}

#[test]
fn a_settings_json_stays_an_unmodeled_surface_until_its_container_is_represented() {
    // The hook kind governs only the `hooks` segment of `settings.json`, never the whole
    // container — so a file carrying an ungoverned key (`permissions`) alongside its `hooks`
    // keeps its unmodeled-surface finding until it is a represented member (phase 2). The
    // hook kind landing must not prematurely retire that finding.
    let harness = common::tmpdir("settings-still-unmodeled");
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
            .any(|line| line.contains(".claude/settings.json")),
        "settings.json stays flagged until its container is represented, got: {findings:#?}"
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
