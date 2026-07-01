//! `temper bundle` — compose the imported surface into a publishable plugin.
//!
//! Implements the `bundle` verb of `specs/50-distribution.md` ("The plugin — the
//! Claude-Code-native delivery"). Distribution is *placing the one gate*; the plugin
//! is the Claude-Code-native placement, and `bundle` is the verb that produces it.
//! `temper` ships as a plugin bundling three things:
//!
//! 1. the **skill** — how to *operate the gate* (`import` / `check`, read a
//!    diagnostic, and when to challenge the contract versus fix the artifact). It is
//!    mechanics, never taste (the Decision "the skill is mechanics, never taste"):
//!    the opinions live in the contract templates, never in skill prose;
//! 2. the **`SessionStart` hook**, in its own `hooks.json` — the advisory
//!    session-start gate (`temper session-start .`, the exec-form command Claude
//!    Code spawns);
//! 3. the **shipped contract templates** — the std-lib, embedded byte-faithful, so
//!    an installed `temper` has something to check against.
//!
//! Alongside the plugin tree it emits a `marketplace.json` listing the plugin, the
//! channel it is distributed through.
//!
//! The plugin is a **vendored, generated surface** — itself an instance of what
//! `temper` projects, so it is **byte-faithful where it carries prose** (`00-intent.md`
//! law 5): the skill body and the contract templates are copied verbatim from their
//! embedded sources, never re-rendered. The structured manifests (`plugin.json`,
//! `marketplace.json`, `hooks.json`) are built through `serde_json`, so they are
//! well-formed by construction — the binary owns the output contract, no
//! hand-escaping (mirroring [`crate::reporter`]).
//!
//! `bundle` **reads the imported surface** it is given (via [`Workspace::load`], an
//! existing public API — no `compose.rs`/`contract.rs` edits) both to fail loud on a
//! malformed surface and to report the harness the plugin was composed over. The
//! three bundled contents are `temper`'s own gate-delivery assets, so `temper bundle`
//! over `temper`'s own surface self-packages `temper`'s plugin — the dogfood target.
//!
//! Determinism: every written byte is a pure function of the embedded assets and the
//! crate version, so re-running `bundle` reproduces an identical tree — the vendored
//! plugin is reviewable and diff-stable.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value as JsonValue, json};

use crate::check::Workspace;

/// The plugin's name — the `plugin.json` `name`, the marketplace entry name, and the
/// directory the operate-the-gate skill lands under (`skills/temper/`). `temper` is
/// outside the `deny`ed skill names (`anthropic`, `claude`), so the bundled skill
/// passes `temper`'s own skill contract — the dogfood holds.
const PLUGIN_NAME: &str = "temper";

/// The plugin/marketplace description — what the gate delivers, not what a good
/// harness is (law 2: taste lives in the contract templates, never here).
const PLUGIN_DESCRIPTION: &str = "The temper gate for a Claude Code harness: import the harness into a typed \
     surface, check it against the active contract, and run the advisory \
     session-start gate — with the std-lib contract templates embedded.";

/// The exec-form command the bundled `SessionStart` hook runs: the `temper` binary
/// itself, checking the project it is installed into (`specs/50-distribution.md`,
/// "the hook is the temper binary itself"). Matches the wiring `temper install`
/// projects into `.claude/settings.json`, so the plugin and `install` deliver the
/// same gate.
const SESSION_START_COMMAND: &str = "temper session-start .";

/// The bundled **operate-the-gate skill**, embedded byte-faithful (law 5). Mechanics
/// only — how to run the checker and when to challenge the contract — never advice on
/// what a good harness is (the Decision "the skill is mechanics, never taste"). Its
/// `name` is `temper` and it lands under `skills/temper/`, so `name-matches-dir` holds
/// and the skill passes `temper`'s own skill contract.
const OPERATE_SKILL: &str = "\
---
name: temper
description: Use when operating the temper gate on a Claude Code harness — importing a harness into the typed surface, running `temper check` against the active contract, reading a temper diagnostic, or deciding whether to fix the artifact or challenge the contract.
---
# Operating the temper gate

`temper` is one gate over a Claude Code harness, placed wherever the harness is
authored, changed, or used. This skill is how to *operate* that gate. It carries no
opinion about what a good harness is — that lives in the contract templates
(`contracts/`), which are data you adopt, extend, or fork.

## Run the gate

- `temper import <harness-path> --into .temper` — scan the harness (skills, rules,
  and any custom kinds a `temper.toml` declares) into the typed surface under
  `.temper`. Re-importing an unchanged harness is a no-op.
- `temper check .temper` — the gate. It validates two greens: every built-in
  contract is itself admissible, and every artifact conforms to the contract for its
  kind. It exits non-zero on a `required`-severity violation; add `--deny-advisories`
  to also fail on advisory ones.
- `temper schema --kind <kind>` — emit the active contract as an editor JSON Schema,
  the same gate shifted to the keystroke.

## Read a diagnostic

Each finding names its clause (the `code`, e.g. `skill.forbidden_keys`), the artifact
it is about, and what failed. A `required` finding blocks; an `advisory` one is a
recommendation. The clause id is the contract clause that fired — read it against the
contract for that kind to see exactly what was checked.

## Fix the artifact, or challenge the contract

A finding is a true positive by construction — the clause is decidable, so it never
guesses. That leaves two honest responses, never a third:

- **Fix the artifact** when the finding is right — the artifact really does violate a
  contract you stand behind.
- **Challenge the contract** when the clause is wrong for this harness — too strict,
  mis-scoped, or checking something you do not want gated. The contract is data: tune
  the clause's severity, change its parameters, or drop it in your `temper.toml`
  layer. Do not silence a finding by contorting the artifact around a clause you
  disagree with — change the clause.

Never paper over a gap. If the contract and the artifact disagree and you are unsure
which is wrong, surface it rather than guessing which way to bend.
";

/// The shipped skill contract template — the curated Anthropic std-lib default,
/// embedded at build time and copied byte-faithful into the plugin's `contracts/`
/// so an installed `temper` has a contract to check skills against
/// (`specs/50-distribution.md`, "the shipped contract templates ... embedded").
const SKILL_CONTRACT: &str = include_str!("../contracts/skill.anthropic.toml");

/// The shipped rule contract template — the curated default for the `rule` kind,
/// embedded beside the skill one and copied byte-faithful into the plugin.
const RULE_CONTRACT: &str = include_str!("../contracts/rule.toml");

/// Errors raised while composing the plugin tree — the write side `bundle` owns.
/// A surface that fails to load bubbles as its own [`WorkspaceError`](crate::check::WorkspaceError);
/// this covers the emit half.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum BundleError {
    /// A plugin file or directory could not be written. Fail-loud
    /// (`specs/50-distribution.md`, "Fail-loud delivery"): a placement that cannot be
    /// written is a hard error, never a silent skip.
    #[error("failed to write {path}")]
    #[diagnostic(code(temper::bundle::write))]
    Write {
        /// The destination path that failed.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },
}

/// The typed result of a [`run`]: every file the plugin tree carries (relative to
/// the output root, sorted), plus the size of the surface it was composed over.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleReport {
    /// The files written under the output root, as paths relative to it, sorted for
    /// a stable, reviewable report.
    pub files: Vec<PathBuf>,
    /// How many skills the imported surface carried — reported so `bundle` names the
    /// harness the plugin was composed over.
    pub skills: usize,
    /// How many rules the imported surface carried.
    pub rules: usize,
}

/// Compose the imported surface at `surface` into a publishable plugin tree under
/// `out`, alongside a `marketplace.json` listing it.
///
/// Reads the surface via [`Workspace::load`] (fail-loud on a malformed one), then
/// emits the plugin: the manifest, the operate-the-gate skill, the `SessionStart`
/// hook in its own `hooks.json`, the embedded contract templates, and the
/// marketplace listing. See the module header for the byte-faithfulness and
/// determinism guarantees.
pub fn run(surface: &Path, out: &Path) -> miette::Result<BundleReport> {
    // Read the imported surface: fail loud if it is not a valid workspace, and carry
    // its size into the report so `bundle` names what it composed over.
    let ws = Workspace::load(surface)?;

    let mut files = Vec::new();

    // The plugin manifest and the marketplace listing it — structured, built through
    // `serde_json` so they are well-formed by construction.
    write_json(
        out,
        Path::new(".claude-plugin/plugin.json"),
        &plugin_manifest(),
        &mut files,
    )?;
    write_json(
        out,
        Path::new(".claude-plugin/marketplace.json"),
        &marketplace_manifest(),
        &mut files,
    )?;

    // The operate-the-gate skill — embedded prose, byte-faithful (law 5).
    write_text(
        out,
        Path::new("skills/temper/SKILL.md"),
        OPERATE_SKILL,
        &mut files,
    )?;

    // The `SessionStart` hook, in its own `hooks.json`.
    write_json(
        out,
        Path::new("hooks/hooks.json"),
        &hooks_manifest(),
        &mut files,
    )?;

    // The shipped contract templates (the std-lib), embedded byte-faithful.
    write_text(
        out,
        Path::new("contracts/skill.anthropic.toml"),
        SKILL_CONTRACT,
        &mut files,
    )?;
    write_text(
        out,
        Path::new("contracts/rule.toml"),
        RULE_CONTRACT,
        &mut files,
    )?;

    files.sort();
    Ok(BundleReport {
        files,
        skills: ws.skills.len(),
        rules: ws.rules.len(),
    })
}

/// The `plugin.json` manifest (`.claude-plugin/plugin.json`): the plugin's identity
/// Claude Code reads to install it. Version tracks the crate so the plugin and the
/// binary it delivers move together.
fn plugin_manifest() -> JsonValue {
    json!({
        "name": PLUGIN_NAME,
        "version": crate::VERSION,
        "description": PLUGIN_DESCRIPTION,
        "keywords": ["claude", "claude-code", "harness", "linter", "gate"],
    })
}

/// The `marketplace.json` manifest listing this one plugin — the channel `temper` is
/// distributed through (`specs/50-distribution.md`, "distributed through a
/// marketplace"). The plugin's `source` is `.`: the marketplace root *is* the plugin
/// root, so the generated tree is a self-contained, installable bundle.
fn marketplace_manifest() -> JsonValue {
    json!({
        "name": PLUGIN_NAME,
        "owner": { "name": PLUGIN_NAME },
        "plugins": [
            {
                "name": PLUGIN_NAME,
                "source": ".",
                "description": PLUGIN_DESCRIPTION,
            }
        ],
    })
}

/// The `hooks.json` manifest carrying the advisory `SessionStart` hook — the
/// exec-form `temper session-start .` command Claude Code spawns at session start.
/// Same shape `temper install` merges into `.claude/settings.json`, so the plugin and
/// `install` deliver the identical gate.
fn hooks_manifest() -> JsonValue {
    json!({
        "hooks": {
            "SessionStart": [
                {
                    "hooks": [
                        { "type": "command", "command": SESSION_START_COMMAND }
                    ]
                }
            ]
        }
    })
}

/// Write a structured manifest as pretty JSON under `<out>/<relative>`, recording the
/// relative path in `files`. A trailing newline keeps the file POSIX-clean; pretty
/// JSON over a deterministic value is reproducible, so the whole emit is deterministic.
fn write_json(
    out: &Path,
    relative: &Path,
    value: &JsonValue,
    files: &mut Vec<PathBuf>,
) -> Result<(), BundleError> {
    // `serde_json::to_string_pretty` over a `Value` never fails (no custom
    // `Serialize` in play), but map the error rather than unwrap to keep the module
    // panic-free on every path.
    let json = serde_json::to_string_pretty(value).map_err(|source| BundleError::Write {
        path: out.join(relative),
        source: std::io::Error::other(source),
    })?;
    write_text(out, relative, &format!("{json}\n"), files)
}

/// Write text bytes verbatim to `<out>/<relative>`, creating parent directories and
/// recording the relative path in `files`. Byte-faithful: the bytes are written
/// exactly as given (law 5), never re-rendered.
fn write_text(
    out: &Path,
    relative: &Path,
    contents: &str,
    files: &mut Vec<PathBuf>,
) -> Result<(), BundleError> {
    let path = out.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| BundleError::Write {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    fs::write(&path, contents.as_bytes()).map_err(|source| BundleError::Write {
        path: path.clone(),
        source,
    })?;
    files.push(relative.to_path_buf());
    Ok(())
}

/// Render a bundle report for the terminal: one `wrote  <path>` line per file, then a
/// one-line tally naming the surface it was composed over — mirroring
/// [`crate::install::render`].
#[must_use]
pub fn render(report: &BundleReport) -> String {
    let mut out = String::new();
    for file in &report.files {
        out.push_str(&format!("wrote  {}\n", file.display()));
    }
    out.push_str(&format!(
        "\nbundled {} file{} (surface: {} skill{}, {} rule{})\n",
        report.files.len(),
        plural(report.files.len()),
        report.skills,
        plural(report.skills),
        report.rules,
        plural(report.rules),
    ));
    out
}

/// The plural suffix for a count — `""` for one, `"s"` otherwise.
fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    /// A fresh, empty temp directory unique to this test run.
    fn tmpdir(label: &str) -> PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "author-bundle-unit-{}-{}-{}",
            std::process::id(),
            id,
            label
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn the_bundled_skill_passes_tempers_own_skill_contract() {
        // The dogfood: the operate-the-gate skill temper ships must itself satisfy
        // temper's skill contract — lowercase name matching its dir, present
        // description under the cap, no forbidden keys. Import it and gate it.
        let src = tmpdir("skill-src");
        let skill_dir = src.join("skills").join("temper");
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(skill_dir.join("SKILL.md"), OPERATE_SKILL).unwrap();

        let skill = crate::skill::Skill::from_source_dir(&skill_dir).unwrap();
        assert_eq!(skill.name, "temper");
        assert!(!skill.description.is_empty());
        assert!(skill.description.len() <= 1024);
        // No Cursor keys leaked into the frontmatter.
        assert!(!skill.extra.contains_key("globs"));
        assert!(!skill.extra.contains_key("alwaysApply"));
    }

    #[test]
    fn manifests_are_well_formed_json() {
        // The structured manifests are valid JSON with the shape Claude Code reads.
        let plugin = plugin_manifest();
        assert_eq!(plugin["name"], "temper");
        assert_eq!(plugin["version"], crate::VERSION);

        let market = marketplace_manifest();
        assert_eq!(market["plugins"][0]["name"], "temper");
        assert_eq!(market["plugins"][0]["source"], ".");

        let hooks = hooks_manifest();
        assert_eq!(
            hooks["hooks"]["SessionStart"][0]["hooks"][0]["command"],
            SESSION_START_COMMAND
        );
    }
}
