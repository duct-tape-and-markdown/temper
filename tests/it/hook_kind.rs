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
//! A member is a whole **matcher group**, and the two levels it spans are two kinds: the
//! lifecycle event, carried off the collection key and gated by `hook`'s own contract, and
//! each entry of the group's `hooks` array — a `handler` member keyed by its position
//! (`…/handler/0`), gated by `handler`'s, whose `type` and per-`type` required fields are
//! fields of the member that carries them.

use std::collections::BTreeMap;
use std::fs;

use crate::common;

use crate::common::{check_harness, write_rule, write_settings};

use temper::builtin_kind;
use temper::builtin_lock;
use temper::drift::{
    self, CollectionAddressRow, Declarations, EmitOptions, KindFactRow, Payload, PayloadMember,
    RegistrationRow,
};
use temper::json_manifest::{self, CollectionSegment};
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

/// A `.claude/settings.json` whose one `PreToolUse` matcher group carries **two** handlers
/// — the shape the old per-handler grain read as two members at one address. The second is
/// an `http` handler with nowhere to POST, so exactly one handler-level clause has
/// something to say and its address is what says which handler.
const TWO_HANDLER_SETTINGS: &str = r#"{
  "hooks": {
    "PreToolUse": [
      { "matcher": "Bash", "hooks": [
        { "type": "command", "command": "echo guard" },
        { "type": "http" }
      ] }
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
/// three `hook` members, one per matcher group, and five `handler` members beneath them.
/// Each handler carries exactly the fields its kind's table marks required (same source).
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

/// Every finding line a `hook` **or** `handler` clause raised, at any depth — a guarded
/// clause reports under its own body address
/// (`handler.when.type=command.required.command`), so a prefix read is what makes "no
/// clause fired" a claim over both whole contracts rather than over one label. Both kinds,
/// because the hooks contract is split across them and a claim over one alone would leave
/// the other's silence unexamined.
fn hooks_findings(findings: &[String]) -> Vec<&String> {
    findings
        .iter()
        .filter(|line| line.contains("title=hook.") || line.contains("title=handler."))
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

    // One member per matcher GROUP, keyed by its lifecycle event, in the collection's own
    // sorted key order — each surfacing in the `hooks` collection.
    let members: Vec<(&str, &str)> = reads[0]
        .members
        .iter()
        .map(|m| (m.collection.as_str(), m.key.as_str()))
        .collect();
    assert_eq!(
        members,
        vec![("hooks", "NotARealEvent"), ("hooks", "PreToolUse")]
    );
    // A `hooks.<Event>` value is Claude Code's array of matcher groups, and the group is the
    // member: its own fields are the group's — the lifted `matcher` where one is present,
    // beside the handler array the write face re-nests. A handler's own keys are the
    // handler's, never the group's.
    let pre = &reads[0].members[1];
    assert_eq!(pre.key, "PreToolUse");
    assert_eq!(pre.fields.get("matcher"), Some(&serde_json::json!("Bash")));
    assert!(!pre.fields.contains_key("type"));
    assert!(!pre.fields.contains_key("command"));
    assert!(!reads[0].members[0].fields.contains_key("matcher"));

    // The handler rides the member's nested entries, keyed by its position in the group.
    assert_eq!(pre.members.len(), 1);
    assert_eq!(pre.members[0].0, 0);
    assert_eq!(
        pre.members[0].1.get("command"),
        Some(&serde_json::json!("echo guard"))
    );
}

#[test]
fn a_two_handler_group_is_one_hook_member_with_a_handler_child_per_position() {
    // The grain, end to end through the gate: ONE group carrying two handlers is one `hook`
    // member and two `handler` members — never two hook members at the one address the bare
    // event spells. The second handler is the broken one, so the finding that fires names
    // `…/handler/1`: the position is the identity, and nothing else in this harness could
    // have raised it.
    let harness = common::tmpdir("hook-two-handler-group");
    write_settings(&harness, TWO_HANDLER_SETTINGS);

    let (findings, ok) = check_harness(&harness);

    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(
        checked.len(),
        1,
        "expected exactly one checked summary, got: {findings:#?}"
    );
    assert!(
        checked[0].contains("hook (1)") && checked[0].contains("handler (2 embedded)"),
        "one group is one hook member, and each of its handlers is a handler member, got: {}",
        checked[0]
    );

    // The finding is the HANDLER's, addressed at the handler's own position — the hook's own
    // contract has nothing to say about a `url` it does not carry.
    let no_url = common::findings_for(&findings, "handler.when.type=http.required.url");
    assert_eq!(
        no_url.len(),
        1,
        "exactly the second handler, missing its `url`, fires, got: {findings:#?}"
    );
    assert!(
        no_url[0].contains("hook:PreToolUse:Bash/handler/1"),
        "the finding names the handler's own address, host name and all, got: {}",
        no_url[0]
    );
    assert!(
        common::findings_for(&findings, "handler.when.type=command.required.command").is_empty(),
        "the first handler carries its `command`, got: {findings:#?}"
    );
    assert!(
        !ok,
        "a handler breaking its documented schema is a required-severity finding, got: {findings:#?}"
    );
}

#[test]
fn a_two_handler_group_re_nests_to_the_settings_json_it_was_read_from() {
    // The read's inverse over a group Claude Code would load and the old grain could not
    // round-trip: two handlers under one matcher used to read as two members at one
    // address, so re-nesting them wrote two groups where the source had one. Here the whole
    // group is one member, its fields carry the whole handler array, and emit writes the
    // bytes back.
    //
    // The source is the canonical writer's own output, so the compare is a fixed point over
    // the one encoder rather than a hand-typed document's formatting.
    let (harness, into) = common::workspace("hook-group-round-trip");
    let group = serde_json::json!([{
        "matcher": "Bash",
        "hooks": [
            { "type": "command", "command": "echo guard" },
            { "type": "http", "url": "https://example.test/audit", "timeout": 30 }
        ]
    }]);
    let source = json_manifest::write_manifest(
        &[CollectionSegment {
            collection_key: "hooks".to_string(),
            entries: BTreeMap::from([("PreToolUse".to_string(), group)]),
        }],
        &BTreeMap::new(),
    );
    write_settings(&harness, &source);

    // Read the group back as a member, then hand its fields to emit exactly as the SDK's own
    // registration row would — the one write face, driven off the one read.
    let reads = common::manifest_members(&harness, &hook_kind());
    let member = &reads[0].members[0];
    assert_eq!(member.members.len(), 2, "the group carries two handlers");
    let payload = Payload {
        version: drift::SEAM_VERSION,
        declarations: Declarations {
            kinds: vec![
                common::hook_kind_facts(),
                common::kind_facts("settings", ".claude", "settings.json"),
            ],
            registrations: vec![RegistrationRow {
                kind: "hook".to_string(),
                key: member.key.clone(),
                manifest: "settings.json".to_string(),
                key_path: "hooks.<Event>".to_string(),
                fields: member.fields.clone().into_iter().collect(),
            }],
            ..Default::default()
        },
        members: vec![PayloadMember {
            kind: "settings".to_string(),
            name: "settings".to_string(),
            host: None,
            fields: Vec::new(),
            body: String::new(),
            source_path: None,
        }],
    };
    drift::emit(&payload, &into, EmitOptions::default()).unwrap();

    assert_eq!(
        fs::read_to_string(harness.join(".claude").join("settings.json")).unwrap(),
        source,
        "the group re-nests to the settings.json it was read from — handler count and order \
         included"
    );
}

/// A program authoring ONE `hook` member whose group fires two handlers — the authoring
/// twin of the read fixtures above. `hook()` types the **group**: the `matcher` is the
/// group's own field and the handlers are its ordered `hooks` array, so one call mints one
/// group whatever the handler count (0075).
const TWO_HANDLER_PROGRAM: &str = r#"
import { emit, harness } from "@dtmd/temper";
import { hook } from "@dtmd/temper/claude-code";

const guard = hook({
  name: "PreToolUse",
  matcher: "Bash",
  hooks: [
    { type: "command", command: "echo guard" },
    { type: "http", url: "https://example.test/audit", timeout: 30 },
  ],
});

process.stdout.write(emit(harness({ members: [guard] })).seam);
"#;

#[test]
fn a_hook_authoring_two_handlers_is_one_registration_row_and_one_projected_group() {
    // The write side of the grain, driven over the real SDK: two handlers under one matcher
    // are one member's array, so the program emits ONE `hooks.<Event>` registration and the
    // projection carries one group holding both — never a row or a group per handler.
    common::ensure_sdk_built();
    let (harness, into) = common::wire_sdk_harness("hook-authors-handlers", TWO_HANDLER_PROGRAM);

    drift::emit_program(&into, EmitOptions::default()).unwrap();

    let lock = fs::read_to_string(into.join("lock.toml")).unwrap();
    assert_eq!(
        lock.matches("[[declaration.registration]]").count(),
        1,
        "one call, one registration row: {lock}"
    );

    // The bytes Claude Code loads, compared against the canonical write face's own output
    // so the assertion pins the shape rather than a hand-typed document's formatting.
    let expected = json_manifest::write_manifest(
        &[CollectionSegment {
            collection_key: "hooks".to_string(),
            entries: BTreeMap::from([(
                "PreToolUse".to_string(),
                serde_json::json!([{
                    "matcher": "Bash",
                    "hooks": [
                        { "type": "command", "command": "echo guard" },
                        { "type": "http", "url": "https://example.test/audit", "timeout": 30 }
                    ]
                }]),
            )]),
        }],
        &BTreeMap::new(),
    );
    assert_eq!(
        fs::read_to_string(harness.join(".claude").join("settings.json")).unwrap(),
        expected,
        "one matcher group carrying both handlers, in authored order"
    );
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

/// A `.claude/settings.json` whose `PostToolUse` event carries **two** matcher groups —
/// the shape the bare-event name read as one address twice — beside a matcher-less
/// `Stop` group. Each group's one handler is an `http` with nowhere to POST, so every
/// group raises exactly one finding and the finding's address is what names the group.
const TWO_GROUP_SETTINGS: &str = r#"{
  "hooks": {
    "PostToolUse": [
      { "matcher": "Edit|Write", "hooks": [ { "type": "http" } ] },
      { "matcher": "Bash", "hooks": [ { "type": "http" } ] }
    ],
    "Stop": [
      { "hooks": [ { "type": "http" } ] }
    ]
  }
}"#;

#[test]
fn two_groups_on_one_event_are_two_members_at_distinct_addresses() {
    // A member's name is its event, then `:` and the matcher's authored bytes; a group
    // binding no matcher keeps the bare event. The event alone named all three members
    // here, and two of them at one address.
    let harness = common::tmpdir("hook-two-groups-one-event");
    write_settings(&harness, TWO_GROUP_SETTINGS);

    let address = hook_kind().collection_address.unwrap();
    let reads = common::manifest_members(&harness, &hook_kind());
    let names: Vec<(String, String)> = reads[0]
        .members
        .iter()
        .map(|member| (member.key.clone(), member.name(&address)))
        .collect();
    assert_eq!(
        names,
        vec![
            (
                "PostToolUse".to_string(),
                "PostToolUse:Edit|Write".to_string()
            ),
            ("PostToolUse".to_string(), "PostToolUse:Bash".to_string()),
            ("Stop".to_string(), "Stop".to_string()),
        ],
        "the key stays the bare event — `hooks.<Event>` is where the group still writes"
    );

    // The name is the unit's id, and the key-field value stays the bare event, so the
    // `hook.enum.event` clause still ranges over a documented value.
    let unit = reads[0].members[0].to_unit(&address, &reads[0].provenance.source_path);
    assert_eq!(unit.id, "PostToolUse:Edit|Write");
    assert_eq!(
        unit.frontmatter.get("event"),
        Some(&serde_json::json!("PostToolUse"))
    );

    // And the address the name spells round-trips: the grammar splits at the *first*
    // colon, so the kind comes off and the matcher rides on inside the name.
    let host = temper::member_address::host_address("hook", &unit.id);
    assert_eq!(
        temper::member_address::parse_host_address(&host),
        Some(("hook", "PostToolUse:Edit|Write"))
    );
}

#[test]
fn each_group_on_one_event_is_judged_and_addressed_as_its_own_member() {
    // The same three groups end to end through the gate: three `hook` members, three
    // `handler` members, and three findings whose addresses tell the two `PostToolUse`
    // groups apart. On the bare-event name the two of them shared one address.
    let harness = common::tmpdir("hook-two-groups-gate");
    write_settings(&harness, TWO_GROUP_SETTINGS);

    let (findings, ok) = check_harness(&harness);

    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(
        checked.len(),
        1,
        "expected exactly one checked summary, got: {findings:#?}"
    );
    assert!(
        checked[0].contains("hook (3)") && checked[0].contains("handler (3 embedded)"),
        "each group is its own member, got: {}",
        checked[0]
    );

    let no_url = common::findings_for(&findings, "handler.when.type=http.required.url");
    for address in [
        "hook:PostToolUse:Edit|Write/handler/0",
        "hook:PostToolUse:Bash/handler/0",
        "hook:Stop/handler/0",
    ] {
        assert_eq!(
            no_url
                .iter()
                .filter(|finding| finding.contains(address))
                .count(),
            1,
            "exactly one finding names `{address}`, got: {findings:#?}"
        );
    }
    assert_eq!(
        no_url.len(),
        3,
        "three groups, three handler findings, got: {findings:#?}"
    );
    assert!(
        common::findings_for(&findings, "hook.enum.event").is_empty(),
        "both events are documented — the matcher rides the name, never the key, got: {findings:#?}"
    );
    assert!(
        !ok,
        "a handler missing its `url` fails the run, got: {findings:#?}"
    );
}

#[test]
fn a_matcher_carrying_the_address_separator_refuses_loud() {
    // The name is the first segment of every address beneath the member, so a `/` in it
    // re-seats the group's handlers under an address naming nothing. Refused at the read,
    // where `to_unit` is still infallible for every caller below it.
    let address = hook_kind().collection_address.unwrap();
    let err = json_manifest::Manifest::parse(
        std::path::Path::new(".claude/settings.json"),
        r#"{
  "hooks": {
    "PostToolUse": [
      { "matcher": "Edit/Write", "hooks": [ { "type": "command", "command": "echo hi" } ] }
    ]
  }
}"#,
        &[&address],
    )
    .expect_err("a matcher carrying the address separator is refused");

    let rendered = err.to_string();
    assert!(
        rendered.contains("PostToolUse:Edit/Write") && rendered.contains("matcher"),
        "the refusal names the member the join would have spelled and the field it came \
         from, got: {rendered}"
    );

    // And the sibling group is not collateral: a `/`-free matcher reads as it always did.
    assert!(
        json_manifest::Manifest::parse(
            std::path::Path::new(".claude/settings.json"),
            r#"{
  "hooks": {
    "PostToolUse": [
      { "matcher": "Edit|Write", "hooks": [ { "type": "command", "command": "echo hi" } ] }
    ]
  }
}"#,
            &[&address],
        )
        .is_ok()
    );
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
fn the_handler_default_contract_fires_on_a_handler_breaking_its_kinds_documented_schema() {
    let harness = common::tmpdir("hook-broken-handler");
    write_settings(&harness, BROKEN_HANDLER_SETTINGS);

    let (findings, ok) = check_harness(&harness);

    // The vacuity pin: four groups were read as `hook` members and their four handlers as
    // `handler` members, so each count below is a verdict over a member and not an empty
    // selection.
    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(
        checked.len(),
        1,
        "expected exactly one checked summary, got: {findings:#?}"
    );
    assert!(
        checked[0].contains("hook (4)") && checked[0].contains("handler (4 embedded)"),
        "all four groups and all four of their handlers are checked, got: {}",
        checked[0]
    );

    // Every clause below is `handler`'s, and every finding names the handler's own address:
    // the schema is a fact about the member that carries it, and the hook member above it
    // carries none of these fields at all.
    //
    // A `command` handler with nothing to run fires its own guard's body, and only it — the
    // guard is keyed on `type`, so the `http` and `webhook` members never enter it.
    let no_command = common::findings_for(&findings, "handler.when.type=command.required.command");
    assert_eq!(
        no_command.len(),
        1,
        "exactly the command handler missing its `command` fires, got: {findings:#?}"
    );
    assert!(
        no_command[0].contains("hook:PreToolUse:Bash/handler/0"),
        "the finding names the handler, not its host hook, got: {}",
        no_command[0]
    );

    // An `http` handler with nowhere to POST fires its own.
    let no_url = common::findings_for(&findings, "handler.when.type=http.required.url");
    assert_eq!(
        no_url.len(),
        1,
        "exactly the http handler missing its `url` fires, got: {findings:#?}"
    );

    // A `type` outside the documented five fires the enum — and no guard, since no guard
    // ranges over a value the allowlist refuses.
    let bad_type = common::findings_for(&findings, "handler.enum.type");
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
    let no_type = common::findings_for(&findings, "handler.required.type");
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

    // And nothing fires under `hook`: the six handler-level clauses have one binding now,
    // and it is not the group's.
    assert!(
        common::findings_for(&findings, "hook.required.type").is_empty()
            && common::findings_for(&findings, "hook.enum.type").is_empty(),
        "no handler-level clause is left bound to `hook`, got: {findings:#?}"
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
fn the_handler_default_contract_passes_a_well_formed_handler_of_every_documented_kind() {
    let harness = common::tmpdir("hook-every-handler-kind");
    write_settings(&harness, EVERY_HANDLER_KIND_SETTINGS);

    let (findings, ok) = check_harness(&harness);

    // The vacuity pin: three groups, one `handler` member per handler, five handlers — so
    // the silence below is a verdict passed by every documented kind, including the two that
    // share a guard and the two pairs that share a group.
    let checked = common::findings_for(&findings, "coverage.checked");
    assert_eq!(
        checked.len(),
        1,
        "expected exactly one checked summary, got: {findings:#?}"
    );
    assert!(
        checked[0].contains("hook (3)") && checked[0].contains("handler (5 embedded)"),
        "one member per documented handler kind is checked, under three groups, got: {}",
        checked[0]
    );

    assert!(
        hooks_findings(&findings).is_empty(),
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
    // beside the one container member.
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

/// A fields-only kind row keyed at `settings.json#hooks.<Event>` and governing no file
/// locus of its own — the shape the collision case needs on both sides of the collision,
/// diverging only on the name and the declared `entry_shape`.
fn collides_at_hooks_event(name: &str, entry_shape: Option<&str>) -> KindFactRow {
    KindFactRow {
        governs_root: None,
        governs_glob: None,
        shape: Some("fields".to_string()),
        collection_address: Some(CollectionAddressRow {
            manifest: "settings.json".to_string(),
            key_path: "hooks.<Event>".to_string(),
            entry_shape: entry_shape.map(str::to_string),
        }),
        ..common::kind_facts(name, "", "")
    }
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
    // kind also at that same address. Both rows are spelled here rather than off
    // `common::hook_kind_facts`, which declares the real kind's `.claude/settings.json`
    // file locus: the collision this case judges is the *collection* address alone, and a
    // governed glob beside it would hand the gate a second, unrelated locus fact.
    common::write_lock(
        &harness,
        Declarations {
            kinds: vec![
                collides_at_hooks_event("hook", Some("group-array(hooks;matcher)")),
                collides_at_hooks_event("custom_hook", None),
            ],
            ..Declarations::default()
        },
    );

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
