//! Acceptance for `temper bundle` (`specs/50-distribution.md`, "The plugin — the
//! Claude-Code-native delivery").
//!
//! Proves the four properties the entry names:
//!
//! - **plugin tree** — one run over an imported surface produces the operate-the-gate
//!   skill, the `SessionStart` hook in its own `hooks.json`, and the shipped built-in
//!   packages embedded byte-faithful;
//! - **marketplace** — a well-formed `marketplace.json` listing the plugin;
//! - **determinism** — a second run reproduces an identical tree, byte for byte
//!   (an `insta` snapshot pins the shape);
//! - **CLI** — the real `temper bundle` binary composes the plugin across the process
//!   boundary (where `main`'s dispatch and the default `--out` are observable).

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

use temper::bundle;

/// The binary under test, located by Cargo at compile time.
const BIN: &str = env!("CARGO_BIN_EXE_temper");

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// A fresh, empty temp directory unique to this test run.
fn tmpdir(label: &str) -> PathBuf {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!(
        "author-bundle-{}-{}-{}",
        std::process::id(),
        id,
        label
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

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

/// Build a one-skill, one-rule harness and import it into a surface workspace,
/// returning the surface path.
fn imported_surface(label: &str) -> PathBuf {
    let harness = tmpdir(&format!("{label}-harness"));
    let skill = harness.join(".claude").join("skills").join("coordinate");
    fs::create_dir_all(&skill).unwrap();
    fs::write(skill.join("SKILL.md"), SKILL).unwrap();
    let rules = harness.join(".claude").join("rules");
    fs::create_dir_all(&rules).unwrap();
    fs::write(rules.join("rust.md"), RULE).unwrap();

    let surface = tmpdir(&format!("{label}-surface"));
    temper::import::run(&harness, &surface).unwrap();
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
    let out = tmpdir("tree-out");

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
        "temper session-start ."
    );

    // 3. The shipped built-in packages, embedded byte-faithful — the same
    //    `packages/<name>/PACKAGE.md` authored as product source, shipped verbatim.
    let skill_package = out
        .join("packages")
        .join("skill.anthropic")
        .join("PACKAGE.md");
    let rule_package = out
        .join("packages")
        .join("rule.anthropic")
        .join("PACKAGE.md");
    assert_eq!(
        fs::read_to_string(&skill_package).unwrap(),
        include_str!("../packages/skill.anthropic/PACKAGE.md"),
        "the embedded skill package must be byte-faithful"
    );
    assert_eq!(
        fs::read_to_string(&rule_package).unwrap(),
        include_str!("../packages/rule.anthropic/PACKAGE.md"),
        "the embedded rule package must be byte-faithful"
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

    // The report names the surface it composed over (one skill, one rule).
    assert_eq!(report.skills, 1);
    assert_eq!(report.rules, 1);
}

#[test]
fn bundle_is_deterministic() {
    // Two runs into the same output tree must not change a single byte — the vendored
    // plugin is diff-stable.
    let surface = imported_surface("determinism");
    let out = tmpdir("determinism-out");

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
    let out = tmpdir("cli-out");

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
