//! Acceptance for `temper bundle`.
//!
//! Proves the three properties the entry names:
//!
//! - **plugin tree** — one run over an imported surface produces the operate-the-gate
//!   skill and the `SessionStart` hook in its own `hooks.json`, and carries no
//!   curated clause embeds (`bundle` delivers the gate, never clauses — clauses
//!   publish through the SDK, channel 1);
//! - **marketplace** — a well-formed `marketplace.json` listing the plugin;
//! - **determinism** — a second run reproduces an identical tree, byte for byte
//!   (an `insta` snapshot pins the shape);
//! - **CLI** — the real `temper bundle` binary composes the plugin across the process
//!   boundary (where `main`'s dispatch and the default `--out` are observable).

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

mod common;

use temper::bundle;

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

/// A skill with frontmatter — a real harness artifact for the surface `bundle` reads.
const SKILL: &str = "---\n\
name: coordinate\n\
description: Use when coordinating agents across axes; not for single-axis work.\n\
---\n\
# Coordinate\n\
\n\
Drive the team through the playbook.\n";

/// A rule with `paths:` frontmatter — so the surface carries both built-in kinds.
const RULE: &str = "---\n\
paths:\n\
  - \"src/**/*.rs\"\n\
---\n\
# Rust conventions\n\
\n\
Prefer a clone over a lifetime fight.\n";

/// Build a one-skill, one-rule surface workspace directly — the shape `bundle` reads
/// via [`temper::check::Workspace::load`] (`<surface>/<kind's surface subdir>/<id>/<member
/// doc>`) — by projecting each member through the same generic frontmatter adapter
/// `import` used, no scratch harness or import verb needed.
fn imported_surface(label: &str) -> PathBuf {
    let skill_kind = temper::builtin_kind::definition("skill").unwrap().unwrap();
    let rule_kind = temper::builtin_kind::definition("rule").unwrap().unwrap();

    let src = common::tmpdir(&format!("{label}-src"));
    let skill_src = src.join("SKILL.md");
    fs::write(&skill_src, SKILL).unwrap();
    let skill = temper::frontmatter::Member::from_source(&skill_kind, &skill_src).unwrap();

    let rule_src = src.join("rust.md");
    fs::write(&rule_src, RULE).unwrap();
    let rule = temper::frontmatter::Member::from_source(&rule_kind, &rule_src).unwrap();

    let surface = common::tmpdir(&format!("{label}-surface"));
    let skill_dir = surface.join("skills").join("coordinate");
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(skill_dir.join("SKILL.md"), skill.to_document().emit()).unwrap();

    let rule_dir = surface.join("rules").join("rust");
    fs::create_dir_all(&rule_dir).unwrap();
    fs::write(rule_dir.join("RULE.md"), rule.to_document().emit()).unwrap();

    surface
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
    assert_eq!(market_json["plugins"][0]["source"], ".");

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

#[test]
fn bundle_is_deterministic() {
    // Two runs into the same output tree must not change a single byte — the vendored
    // plugin is diff-stable.
    let surface = imported_surface("determinism");
    let out = common::tmpdir("determinism-out");

    bundle::run(&surface, &out).unwrap();
    let first = tree_bytes(&out);

    bundle::run(&surface, &out).unwrap();
    let second = tree_bytes(&out);

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
