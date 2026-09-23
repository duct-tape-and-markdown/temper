//! The `installed-plugin` built-in kind: a `settings.json` `enabledPlugins` registration
//! member (`specs/builtins.md`, "The shipped kinds"; 0031).
//!
//! The third manifest kind, and the first whose entries are **scalars** — fields-only,
//! discovered off the `.claude/settings.json` manifest at the `enabledPlugins.*`
//! collection address and read through the JSON manifest adapter, each entry's bare
//! boolean carried as the member's one declared `enabled` field. Driven over fixtures
//! mirroring the real Claude Code layout (`.claude/settings.json` carrying an
//! `enabledPlugins` map, per `.claude/rules/rust.md`): the read that turns an enablement
//! entry into a member, the documented `false` that gates the member off its one channel,
//! and the settings file that declares no plugins at all.

mod common;

use common::{check_harness, write_settings};

use serde_json::Value as JsonValue;
use temper::builtin_kind;
use temper::builtin_lock;
use temper::kind::{CollectionAddress, CollectionKeyPath, Content, Registration};

/// A `.claude/settings.json` carrying two enabled plugins and one explicitly disabled, in
/// the real Claude Code shape: `enabledPlugins` maps a `<plugin>@<marketplace>` identity
/// to a bare boolean (code.claude.com/docs/en/plugins-reference, retrieved 2026-07-16).
/// `permissions` rides alongside — `settings.json` is not wholly this collection.
const SETTINGS: &str = r#"{
  "permissions": { "allow": ["Bash(git status)"] },
  "enabledPlugins": {
    "formatter@my-marketplace": true,
    "linter@my-marketplace": true,
    "legacy@my-marketplace": false
  }
}"#;

/// A `.claude/settings.json` that declares no plugins at all — the overwhelmingly common
/// real settings file, and the one that must surface no member and no finding.
const SETTINGS_NO_PLUGINS: &str = r#"{
  "permissions": { "allow": ["Bash(git status)"] }
}"#;

fn installed_plugin_kind() -> temper::kind::CustomKind {
    builtin_kind::definition("installed-plugin").expect("installed-plugin is embedded")
}

/// A `.claude/settings.json` where two enabled plugins name a marketplace the registry
/// declares (`acme`) and one names a marketplace no registration carries (`ghost-mk`) — the
/// resolving edge and the dangling one side by side, in the real Claude Code shape.
const SETTINGS_WITH_MARKETPLACES: &str = r#"{
  "enabledPlugins": {
    "formatter@acme": true,
    "linter@acme": true,
    "stray@ghost-mk": true
  },
  "extraKnownMarketplaces": {
    "acme": { "source": { "source": "github", "repo": "acme/marketplace" } }
  }
}"#;

#[test]
fn the_installed_plugin_kind_is_a_fields_only_manifest_kind_at_the_enabled_plugins_address() {
    let plugin = installed_plugin_kind();
    assert_eq!(plugin.content, Content::Fields);
    assert_eq!(
        plugin.collection_address,
        Some(CollectionAddress {
            manifest: "settings.json".to_string(),
            key_path: CollectionKeyPath::EnabledPlugins,
            entry_shape: temper::kind::EntryShape::Scalar {
                field: "enabled".to_string(),
            },
        })
    );
    // The entry's own presence is the channel, and the field names the gate.
    assert_eq!(
        plugin.registration,
        vec![Registration::Enablement {
            field: "enabled".to_string(),
        }]
    );
}

#[test]
fn a_settings_enabled_plugins_map_surfaces_one_member_per_entry_keyed_by_plugin_identity() {
    let harness = common::tmpdir("read-enabled-plugins");
    write_settings(&harness, SETTINGS);

    let reads = common::manifest_members(&harness, &installed_plugin_kind());
    assert_eq!(
        reads.len(),
        1,
        "the one settings.json manifest is read once"
    );

    // One member per `enabledPlugins` entry, identity the collection key, in the
    // collection's own sorted key order.
    let members: Vec<(&str, &str)> = reads[0]
        .members
        .iter()
        .map(|m| (m.collection.as_str(), m.key.as_str()))
        .collect();
    assert_eq!(
        members,
        vec![
            ("enabledPlugins", "formatter@my-marketplace"),
            ("enabledPlugins", "legacy@my-marketplace"),
            ("enabledPlugins", "linter@my-marketplace"),
        ]
    );

    // Unlike a hook (array value) or an MCP server (object value), an entry's value is a
    // bare scalar, so there is no object to fold: the member carries exactly the one
    // declared `enabled` field off its value.
    let formatter = &reads[0].members[0];
    assert_eq!(
        formatter.fields.get("enabled"),
        Some(&JsonValue::Bool(true))
    );
    assert_eq!(formatter.fields.len(), 1, "one field, off the scalar value");

    let legacy = &reads[0].members[1];
    assert_eq!(legacy.fields.get("enabled"), Some(&JsonValue::Bool(false)));

    // `permissions` is no address's, so it stays opaque residue on the container.
    assert!(reads[0].opaque_fields.contains_key("permissions"));
}

/// The root selection binding one `reachable` clause at `required` — the opt-in the
/// judge locates before it walks anything, and the declaration its findings report
/// under. `members` stays empty: the predicate ranges over `by_kind`.
fn root_reachable_binding() -> Vec<temper::engine::Selection<'static>> {
    vec![temper::engine::Selection {
        selector: temper::engine::Selector::Root,
        clauses: vec![temper::contract::Clause {
            label: "root.reachable".to_string(),
            severity: temper::contract::Severity::Required,
            predicate: temper::contract::Predicate::Reachable,
            guidance: None,
            source: None,
        }],
        members: Vec::new(),
    }]
}

#[test]
fn a_false_valued_entry_gates_its_member_off_every_channel() {
    let harness = common::tmpdir("enabled-plugins-gate");
    write_settings(&harness, SETTINGS);

    let members = common::kind_features(&harness, &installed_plugin_kind());
    let channels = vec![Registration::Enablement {
        field: "enabled".to_string(),
    }];
    let by_kind = std::collections::BTreeMap::from([("installed-plugin", members.as_slice())]);
    let registrations = std::collections::BTreeMap::from([("installed-plugin", channels)]);

    let findings = temper::graph::reachable(
        &root_reachable_binding(),
        &registrations,
        &by_kind,
        &[],
        &[],
        &[],
    );

    // The gate rides the declared field's documented semantics, never a second channel
    // entry: `false` is the one value the harness documents as not loaded, so exactly the
    // disabled plugin is dead — the two enabled ones stay live.
    assert_eq!(
        findings.len(),
        1,
        "exactly the `false` entry is unreachable, got: {findings:#?}"
    );
    let rendered = format!("{:#?}", findings[0]);
    assert!(
        rendered.contains("legacy@my-marketplace"),
        "the finding names the disabled plugin, got: {rendered}"
    );
    assert!(
        rendered.contains("enabled"),
        "the finding names the field carrying the gate, got: {rendered}"
    );
}

#[test]
fn a_settings_file_with_no_enabled_plugins_surfaces_no_member_and_no_finding() {
    let harness = common::tmpdir("enabled-plugins-absent");
    write_settings(&harness, SETTINGS_NO_PLUGINS);

    let reads = common::manifest_members(&harness, &installed_plugin_kind());
    assert_eq!(reads.len(), 1, "the settings.json manifest is still read");
    assert!(
        reads[0].members.is_empty(),
        "no `enabledPlugins` key is no members — never a fabricated one, got: {:#?}",
        reads[0].members
    );

    let (findings, _ok) = check_harness(&harness);
    let fired = common::findings_for(&findings, "installed-plugin");
    assert!(
        fired.is_empty(),
        "a settings file declaring no plugins is not a finding, got: {findings:#?}"
    );
}

#[test]
fn the_embedded_builtin_lock_carries_the_installed_plugin_kind_and_no_clause() {
    // The embedded side of the built-in lock (`tests/builtin_lock_frozen.rs` pins its
    // byte-equality with the SDK module's own memberless emit).
    let declarations = builtin_lock::declarations();

    let plugin = declarations
        .kinds
        .iter()
        .find(|k| k.name == "installed-plugin")
        .expect("the installed-plugin kind fact is embedded");
    assert_eq!(plugin.shape.as_deref(), Some("fields"));
    assert_eq!(plugin.registration, vec!["enablement(enabled)".to_string()]);
    let address = plugin
        .collection_address
        .as_ref()
        .expect("the installed-plugin kind carries its collection address");
    assert_eq!(address.manifest, "settings.json");
    assert_eq!(address.key_path, "enabledPlugins.*");

    // The empty default contract is an assertion, not an omission: the format documents no
    // gateable schema, so an almost-empty format earns an almost-empty contract.
    assert!(
        !declarations
            .clauses
            .iter()
            .any(|c| c.kind.as_deref() == Some("installed-plugin")),
        "the installed-plugin default contract ships empty"
    );
}

#[test]
fn the_marketplace_half_of_an_enablement_key_is_a_declared_edge_to_known_marketplace() {
    // The edge is declared, not invented here: the built-in kind carries the marketplace
    // edge on its schema, and the reference graph reads it off each plugin's features — the
    // marketplace half split off the composite `<plugin>@<marketplace>` key.
    let plugin = installed_plugin_kind();
    assert_eq!(
        plugin.relationships,
        vec![temper::compose::Edge {
            field: "marketplace".to_string(),
            from: "installed-plugin".to_string(),
            to: vec!["known-marketplace".to_string()],
        }],
        "installed-plugin declares the marketplace edge to known-marketplace"
    );

    let harness = common::tmpdir("enabled-plugins-marketplace-edge");
    write_settings(&harness, SETTINGS_WITH_MARKETPLACES);

    let plugins = common::kind_features(&harness, &plugin);
    let marketplaces = common::kind_features(
        &harness,
        &builtin_kind::definition("known-marketplace").expect("known-marketplace is embedded"),
    );

    // Every plugin surfaces its marketplace half as the `marketplace` field the edge reads —
    // the same fold a frontmatter reference rides.
    let stray = plugins
        .iter()
        .find(|features| features.id == "stray@ghost-mk")
        .expect("the ghost-mk plugin is read");
    assert_eq!(
        stray.field("marketplace"),
        Some(temper::extract::FeatureValue::scalar(
            temper::extract::ValueType::String,
            "ghost-mk",
        )),
        "the marketplace half of the key is the field the edge resolves"
    );

    let by_kind = std::collections::BTreeMap::from([
        ("installed-plugin", plugins.as_slice()),
        ("known-marketplace", marketplaces.as_slice()),
    ]);
    let findings = temper::graph::check(&plugin.relationships, &by_kind);

    // `formatter@acme` and `linter@acme` resolve to the declared `acme` registry entry;
    // `stray@ghost-mk` names a marketplace no registration declares — exactly one dangling
    // edge, keyed to the offending plugin and naming the marketplace it wrote.
    assert_eq!(
        findings.len(),
        1,
        "exactly the undeclared marketplace dangles, got: {findings:#?}"
    );
    let rendered = format!("{:#?}", findings[0]);
    assert!(
        rendered.contains("stray@ghost-mk") && rendered.contains("ghost-mk"),
        "the finding names the enablement and the marketplace it named, got: {rendered}"
    );
    assert!(
        rendered.contains("known-marketplace"),
        "the finding names the target kind the marketplace half resolves within, got: {rendered}"
    );
}
