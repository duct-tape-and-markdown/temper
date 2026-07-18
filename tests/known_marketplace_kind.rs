//! The `known-marketplace` built-in kind: a `settings.json` `extraKnownMarketplaces`
//! registration member (`specs/builtins.md`, "The shipped kinds"; 0039).
//!
//! The fourth registration member, the consumer half of the plugin-distribution graph:
//! fields-only, discovered off the `.claude/settings.json` manifest at the
//! `extraKnownMarketplaces.*` collection address and read through the JSON manifest adapter,
//! each entry's object folded into its member (the documented `source` union and
//! `autoUpdate`). Distinct from the publisher-side `marketplace` catalog — two documents, two
//! owners. Driven over fixtures mirroring the real Claude Code layout (`.claude/settings.json`
//! carrying an `extraKnownMarketplaces` map, per `.claude/rules/rust.md`): the read that turns
//! a registry entry into a member, the `registry` channel that is never provably dead
//! (presence is the channel, as a connection's is), and the settings file that names no
//! marketplace at all.

use std::path::Path;

mod common;

use common::{check_harness, write_settings};

use serde_json::Value as JsonValue;
use temper::builtin_kind;
use temper::builtin_lock;
use temper::extract::Features;
use temper::json_manifest::Manifest;
use temper::kind::{CollectionAddress, CollectionKeyPath, Content, Registration};

/// A `.claude/settings.json` carrying two registered marketplaces in the real Claude Code
/// shape: `extraKnownMarketplaces` maps a marketplace name to an object naming where to fetch
/// it (`source`, the same union a `marketplace.json` lists its plugins by) and whether the
/// harness keeps it current (`autoUpdate`) (code.claude.com/docs/en/plugin-marketplaces,
/// retrieved 2026-07-17). `enabledPlugins` rides alongside — `settings.json` is not wholly
/// this collection.
const SETTINGS: &str = r#"{
  "enabledPlugins": { "formatter@acme": true },
  "extraKnownMarketplaces": {
    "acme": { "source": { "source": "github", "repo": "acme/marketplace" }, "autoUpdate": true },
    "local-mk": { "source": "./vendor/marketplace" }
  }
}"#;

/// A `.claude/settings.json` that registers no marketplace at all — the overwhelmingly common
/// real settings file, and the one that must surface no member and no finding.
const SETTINGS_NO_MARKETPLACES: &str = r#"{
  "permissions": { "allow": ["Bash(git status)"] }
}"#;

fn known_marketplace_kind() -> temper::kind::CustomKind {
    builtin_kind::definition("known-marketplace")
        .unwrap()
        .expect("known-marketplace is embedded")
}

/// The kind's members projected through the shared read-time fold — the same `Features` a
/// clause and the reachability gate range over.
fn features(harness: &Path) -> Vec<Features> {
    let kind = known_marketplace_kind();
    let reads = Manifest::read_kind(&temper::import::Discovery::new(harness), &kind).unwrap();
    let address = kind.collection_address.clone().unwrap();
    let source = harness.join(".claude/settings.json");
    reads
        .iter()
        .flat_map(|manifest| &manifest.members)
        .map(|member| builtin_kind::features(&kind, &member.to_unit(&address, &source), &[]))
        .collect()
}

#[test]
fn the_known_marketplace_kind_is_a_fields_only_manifest_kind_at_the_extra_known_marketplaces_address()
 {
    let market = known_marketplace_kind();
    assert_eq!(market.content, Content::Fields);
    assert_eq!(
        market.collection_address,
        Some(CollectionAddress {
            manifest: "settings.json".to_string(),
            key_path: CollectionKeyPath::ExtraKnownMarketplaces,
        })
    );
    // The registry entry's own presence is the channel — fieldless, as a connection's is.
    assert_eq!(market.registration, vec![Registration::Registry]);
}

#[test]
fn a_settings_extra_known_marketplaces_map_surfaces_one_member_per_entry_keyed_by_name() {
    let harness = common::tmpdir("read-known-marketplaces");
    write_settings(&harness, SETTINGS);

    let reads = Manifest::read_kind(
        &temper::import::Discovery::new(&harness),
        &known_marketplace_kind(),
    )
    .unwrap();
    assert_eq!(
        reads.len(),
        1,
        "the one settings.json manifest is read once"
    );

    // One member per `extraKnownMarketplaces` entry, identity the collection key, in the
    // collection's own sorted key order.
    let members: Vec<(&str, &str)> = reads[0]
        .members
        .iter()
        .map(|m| (m.collection.as_str(), m.key.as_str()))
        .collect();
    assert_eq!(
        members,
        vec![
            ("extraKnownMarketplaces", "acme"),
            ("extraKnownMarketplaces", "local-mk"),
        ]
    );

    // Unlike an installed plugin (bare boolean value), a registry entry is an object, so its
    // own fields fold into the member — the `source` union and `autoUpdate`, carried raw for
    // the shared read-time projection.
    let acme = &reads[0].members[0];
    assert_eq!(acme.fields.get("autoUpdate"), Some(&JsonValue::Bool(true)));
    assert_eq!(
        acme.fields.get("source"),
        Some(&serde_json::json!({ "source": "github", "repo": "acme/marketplace" }))
    );

    // The string form of the `source` union folds just as faithfully — a `./`-relative path.
    let local = &reads[0].members[1];
    assert_eq!(
        local.fields.get("source"),
        Some(&JsonValue::from("./vendor/marketplace"))
    );

    // `enabledPlugins` is another kind's collection, not this one's, so it never surfaces as a
    // known-marketplace member.
    assert!(
        reads[0]
            .members
            .iter()
            .all(|m| m.collection == "extraKnownMarketplaces"),
        "only extraKnownMarketplaces entries surface, got: {:#?}",
        reads[0].members
    );
}

#[test]
fn the_registry_channel_is_never_provably_dead() {
    let harness = common::tmpdir("known-marketplace-reach");
    write_settings(&harness, SETTINGS);

    let members = features(&harness);
    let channels = vec![Registration::Registry];
    let by_kind = std::collections::BTreeMap::from([("known-marketplace", members.as_slice())]);
    let registrations = std::collections::BTreeMap::from([("known-marketplace", channels)]);

    let findings = temper::graph::reachable(
        &registrations,
        &by_kind,
        &[],
        &[],
        temper::check::Severity::Error,
    );

    // Whether the marketplace the entry names actually resolves is a fetch-time fact temper
    // cannot decide, so — like a connection — the channel is never provably dead: every
    // registered marketplace stays live, none is ever called unreachable on a guess.
    assert!(
        findings.is_empty(),
        "a registered marketplace is never provably dead, got: {findings:#?}"
    );
}

#[test]
fn a_settings_file_with_no_known_marketplaces_surfaces_no_member_and_no_finding() {
    let harness = common::tmpdir("known-marketplaces-absent");
    write_settings(&harness, SETTINGS_NO_MARKETPLACES);

    let reads = Manifest::read_kind(
        &temper::import::Discovery::new(&harness),
        &known_marketplace_kind(),
    )
    .unwrap();
    assert_eq!(reads.len(), 1, "the settings.json manifest is still read");
    assert!(
        reads[0].members.is_empty(),
        "no `extraKnownMarketplaces` key is no members — never a fabricated one, got: {:#?}",
        reads[0].members
    );

    let (findings, _ok) = check_harness(&harness);
    let fired = common::findings_for(&findings, "known-marketplace");
    assert!(
        fired.is_empty(),
        "a settings file naming no marketplace is not a finding, got: {findings:#?}"
    );
}

#[test]
fn the_embedded_builtin_lock_carries_the_known_marketplace_kind_and_no_clause() {
    // The embedded side of the built-in lock (`tests/builtin_lock_frozen.rs` pins its
    // byte-equality with the SDK module's own memberless emit).
    let declarations = builtin_lock::declarations();

    let market = declarations
        .kinds
        .iter()
        .find(|k| k.name == "known-marketplace")
        .expect("the known-marketplace kind fact is embedded");
    assert_eq!(market.shape.as_deref(), Some("fields"));
    assert_eq!(market.registration, vec!["registry".to_string()]);
    let address = market
        .collection_address
        .as_ref()
        .expect("the known-marketplace kind carries its collection address");
    assert_eq!(address.manifest, "settings.json");
    assert_eq!(address.key_path, "extraKnownMarketplaces.*");

    // The empty default contract is an assertion, not an omission: the `source` union and
    // `autoUpdate` are the type's to hold and the key is the member's identity, so no decidable
    // clause survives that the shape does not already enforce.
    assert!(
        !declarations
            .clauses
            .iter()
            .any(|c| c.kind.as_deref() == Some("known-marketplace")),
        "the known-marketplace default contract ships empty"
    );
}
