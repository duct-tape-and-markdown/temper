//! Acceptance for `temper bundle`.
//!
//! Proves the three properties the entry names:
//!
//! - **plugin tree** — one run over an imported surface produces the operate-the-gate
//!   skill and the `SessionStart` hook in its own `hooks.json`, and carries no
//!   curated clause embeds (`bundle` delivers the gate, never clauses — clauses
//!   publish through the SDK, channel 1);
//! - **marketplace** — a well-formed `marketplace.json` listing the plugin;
//! - **kinded manifests** — both `.claude-plugin` manifests are members of the kinds that
//!   type them, rendered by their kind's write face and passing their kind's contract when
//!   the real gate reads the published tree back;
//! - **determinism** — a second run reproduces an identical tree, byte for byte
//!   (an `insta` snapshot pins the shape, and the two manifests' bytes are pinned whole);
//! - **CLI** — the real `temper bundle` binary composes the plugin across the process
//!   boundary (where `main`'s dispatch and the default `--out` are observable).

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

mod common;

use temper::bundle;

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

/// A surface path to compose the plugin "over" — `bundle` never reads it (channel 3
/// ships the operate-the-gate skill and the `SessionStart` hook unconditionally), so
/// an empty directory stands in for a real imported surface just as well as one.
fn imported_surface(label: &str) -> PathBuf {
    common::tmpdir(label)
}

#[test]
fn bundle_emits_the_plugin_tree_and_marketplace() {
    let surface = imported_surface("tree");
    let out = common::tmpdir("tree-out");

    let report = bundle::run(&surface, &out).unwrap();

    // 1. The operate-the-gate skill — under skills/temper/, with frontmatter.
    let skill = out.join("skills").join("temper").join("SKILL.md");
    assert!(skill.is_file(), "the bundled skill must be written");
    let skill_md = fs::read_to_string(&skill).unwrap();
    assert!(
        skill_md.starts_with("---\nname: temper\n"),
        "the skill carries its frontmatter, got:\n{skill_md}"
    );
    // Mechanics, not taste: it teaches operating the gate and challenging the
    // contract, never what a good harness is.
    assert!(skill_md.contains("temper check"));
    assert!(skill_md.contains("Challenge the contract"));

    // 2. The SessionStart hook, in its own hooks.json — the exec-form command.
    let hooks = out.join("hooks").join("hooks.json");
    assert!(hooks.is_file(), "hooks.json must be written");
    let hooks_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&hooks).unwrap()).unwrap();
    assert_eq!(
        hooks_json["hooks"]["SessionStart"][0]["hooks"][0]["command"],
        "temper check . --reporter session-start"
    );

    // 3. No curated clause embeds: `bundle` delivers the gate, never clauses.
    assert!(
        !out.join("packages").exists(),
        "the plugin must not carry curated clause embeds"
    );

    // 4. A well-formed marketplace.json listing the plugin.
    let market = out.join(".claude-plugin").join("marketplace.json");
    assert!(market.is_file(), "marketplace.json must be written");
    let market_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&market).unwrap()).unwrap();
    assert_eq!(market_json["plugins"][0]["name"], "temper");
    assert_eq!(market_json["plugins"][0]["source"], "./");

    // The plugin manifest identifies the plugin.
    let plugin = out.join(".claude-plugin").join("plugin.json");
    let plugin_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&plugin).unwrap()).unwrap();
    assert_eq!(plugin_json["name"], "temper");

    // The report names what the bundle ships — the operate-the-gate skill and the
    // hook events it wires — never the composed-over surface's member count (the
    // surface here carries one skill and one rule, and the report says nothing
    // about either).
    assert_eq!(report.skill_name, "temper");
    assert_eq!(report.hook_events, vec!["SessionStart"]);
}

#[test]
fn the_skill_teaches_custom_kind_layout_authoring_as_mechanics() {
    // Layout / custom-kind authoring had no written home; it is mechanics, so it
    // lives in the operate-the-gate skill (never a README/docs horizon page). The
    // bundled skill body must carry the authoring guidance — declare a layout when a
    // host mixes prose and members — spoken in the kernel nouns.
    let surface = imported_surface("custom-kind");
    let out = common::tmpdir("custom-kind-out");

    bundle::run(&surface, &out).unwrap();

    let skill_md = fs::read_to_string(out.join("skills").join("temper").join("SKILL.md")).unwrap();
    assert!(
        skill_md.contains("declare a **layout**"),
        "the skill must teach declaring a layout when a host mixes prose and members, got:\n{skill_md}"
    );
    assert!(
        skill_md.contains("member collection") && skill_md.contains("field section"),
        "the skill must name the layout primitives, got:\n{skill_md}"
    );
}

#[test]
fn shipped_strings_teach_install_not_the_retired_import_verb() {
    // The CLI has no Import subcommand (Install/Check/Emit/Schema/Guard/Bundle/
    // Explain) — `install` is the single on-ramp. No shipped bundle string may
    // still teach the retired `import` verb, and the skill's on-ramp bullet must
    // name `temper install`.
    let surface = imported_surface("install-verb");
    let out = common::tmpdir("install-verb-out");

    bundle::run(&surface, &out).unwrap();

    let plugin_json = fs::read_to_string(out.join(".claude-plugin").join("plugin.json")).unwrap();
    let market_json =
        fs::read_to_string(out.join(".claude-plugin").join("marketplace.json")).unwrap();
    let skill_md = fs::read_to_string(out.join("skills").join("temper").join("SKILL.md")).unwrap();

    for (label, text) in [
        ("plugin.json", &plugin_json),
        ("marketplace.json", &market_json),
        ("SKILL.md", &skill_md),
    ] {
        assert!(
            !text.contains("temper import") && !text.contains("`import`"),
            "{label} must not teach the retired `import` verb, got:\n{text}"
        );
        assert!(
            !text.contains("floor"),
            "{label} must not use the retired `floor` vocabulary, got:\n{text}"
        );
    }

    assert!(
        skill_md.contains("temper install"),
        "the skill's on-ramp bullet must name `temper install`, got:\n{skill_md}"
    );
}

#[test]
fn bundle_report_names_shipped_artifacts_over_an_empty_surface() {
    // The field report: a large surface produced "surface: 0 skills, 0 rules" because
    // the report counted composed-over members instead of naming what channel 3
    // actually ships. An empty surface is the sharpest case — the report must still
    // name the shipped skill and hook, never a member tally.
    let surface = common::tmpdir("empty-surface");
    let out = common::tmpdir("empty-out");

    let report = bundle::run(&surface, &out).unwrap();
    assert_eq!(report.skill_name, "temper");
    assert_eq!(report.hook_events, vec!["SessionStart"]);

    let rendered = bundle::render(&report);
    assert!(
        rendered.contains("ships: skill `temper`, SessionStart hook"),
        "got:\n{rendered}"
    );
    assert!(
        !rendered.contains("0 skill") && !rendered.contains("0 rule"),
        "the report must never read as a member tally, got:\n{rendered}"
    );
}

/// The on-disk manifest's top-level keys, as the fields one member of its kind carries.
fn manifest_fields(path: &std::path::Path) -> BTreeMap<String, serde_json::Value> {
    let written = fs::read_to_string(path).unwrap();
    let value: serde_json::Value = serde_json::from_str(&written).unwrap();
    value
        .as_object()
        .expect("a manifest top level is an object")
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

#[test]
fn the_kinded_manifests_are_rendered_by_their_kinds_write_face() {
    // Consolidation: `plugin.json` and `marketplace.json` are members of the kinds that
    // type them, so their bytes are their kind's `json-document` write face's output —
    // never a serde_json path spelled beside the kind. Re-rendering the on-disk fields
    // through that face must reproduce them exactly, proving no second encoder survives
    // here to drift from the one `check` reads them back through.
    let surface = imported_surface("kinded-write");
    let out = common::tmpdir("kinded-write-out");

    bundle::run(&surface, &out).unwrap();

    for relative in [
        ".claude-plugin/plugin.json",
        ".claude-plugin/marketplace.json",
    ] {
        let written = fs::read_to_string(out.join(relative)).unwrap();
        let rendered = temper::json_manifest::write_document(&manifest_fields(&out.join(relative)));
        assert_eq!(
            written, rendered,
            "{relative} must be byte-identical to its kind's document write face"
        );
    }

    // `hooks.json` is a plugin-cache artifact of neither kind, so it keeps riding the
    // canonical whole-manifest write as pure residue — the one encoder for what no kind
    // types, never a hand-rolled pretty-printer.
    let hooks = out.join("hooks/hooks.json");
    let written = fs::read_to_string(&hooks).unwrap();
    assert_eq!(
        written,
        temper::json_manifest::write_manifest(&[], &manifest_fields(&hooks)),
        "hooks.json must be byte-identical to the canonical manifest write"
    );
}

#[test]
fn the_published_manifests_pass_the_contracts_temper_ships_for_them() {
    // The gap decision 0031 named: the product wrote these two files and refused to check
    // them. Now they are members of shipped kinds at the loci those kinds govern, so the
    // real gate reads them off the bundled tree — temper checks what it publishes.
    let surface = imported_surface("gated");
    let out = common::tmpdir("gated-out");

    bundle::run(&surface, &out).unwrap();

    let (findings, ok) = common::check_harness(&out);

    assert!(
        ok,
        "the bundle temper publishes must pass the contract temper ships for it: {findings:?}"
    );
    // And both manifests are really being read and checked, not silently skipped past —
    // a clean run over a tree the gate never looked at would prove nothing.
    for counted in ["plugin-manifest (1)", "marketplace (1)"] {
        assert!(
            findings.iter().any(|f| f.contains(counted)),
            "the gate must count the published manifest as a member: {findings:?}"
        );
    }
}

#[test]
fn the_published_manifests_bytes_are_pinned() {
    // The two `.claude-plugin` manifests are what a user installs temper by, so their
    // exact bytes are the reviewable artifact — pinned here rather than left to whichever
    // write face happens to render them. `version` tracks the crate, so it is elided to a
    // placeholder: a release bump is not a change to the manifest's shape.
    let surface = imported_surface("manifest-bytes");
    let out = common::tmpdir("manifest-bytes-out");

    bundle::run(&surface, &out).unwrap();

    for relative in [
        ".claude-plugin/plugin.json",
        ".claude-plugin/marketplace.json",
    ] {
        let written = fs::read_to_string(out.join(relative)).unwrap();
        let pinned = written.replace(temper::VERSION, "<version>");
        insta::assert_snapshot!(relative, pinned);
    }
}

#[test]
fn bundle_is_deterministic() {
    // Two runs into the same output tree must not change a single byte — the vendored
    // plugin is diff-stable.
    let surface = imported_surface("determinism");
    let out = common::tmpdir("determinism-out");

    bundle::run(&surface, &out).unwrap();
    let first = common::tree_bytes(&out);

    bundle::run(&surface, &out).unwrap();
    let second = common::tree_bytes(&out);

    assert_eq!(first, second, "re-running bundle must be byte-identical");

    // Pin the plugin shape: the sorted set of files the tree carries.
    let files: Vec<String> = first
        .keys()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .collect();
    insta::assert_debug_snapshot!(files);
}

#[test]
fn the_cli_bundle_verb_composes_the_plugin() {
    let surface = imported_surface("cli");
    let out = common::tmpdir("cli-out");

    let output = Command::new(BIN)
        .arg("bundle")
        .arg(&surface)
        .arg("--out")
        .arg(&out)
        .output()
        .unwrap();
    assert!(output.status.success(), "bundle must exit zero");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("bundled") && stdout.contains("marketplace.json"),
        "the run must report the composed plugin, got:\n{stdout}"
    );
    assert!(
        stdout.contains("ships: skill `temper`, SessionStart hook"),
        "the report must name the shipped artifacts, got:\n{stdout}"
    );

    // The plugin tree landed on disk.
    assert!(
        out.join(".claude-plugin")
            .join("marketplace.json")
            .is_file(),
        "the CLI bundle must write the marketplace"
    );
    assert!(
        out.join("skills").join("temper").join("SKILL.md").is_file(),
        "the CLI bundle must write the operate-the-gate skill"
    );
}
