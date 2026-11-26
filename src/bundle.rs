//! `temper bundle` — compose the imported surface into a publishable plugin.
//!
//! Implements the `bundle` verb ("The plugin — the
//! Claude-Code-native delivery"). Distribution is *placing the one gate*; the plugin
//! is the Claude-Code-native placement, and `bundle` is the verb that produces it.
//! `temper` ships as a plugin bundling two things:
//!
//! 1. the **skill** — the SDK's harness reference, emitted over the built-in program:
//!    how to *operate the gate* (`install` / `check`, read a diagnostic, author a
//!    custom kind's layout, and when to challenge the contract versus fix the
//!    artifact), then one topic file per built-in kind beside it. It is
//!    mechanics, never taste (the Decision "the skill is mechanics, never taste"):
//!    the opinions live in the SDK's floors, never in skill prose;
//! 2. the **`SessionStart` hook**, in its own `hooks.json` — the advisory
//!    session-start gate (`temper check . --reporter session-start`, the
//!    exec-form command Claude Code spawns).
//!
//! Alongside the plugin tree it emits a `marketplace.json` listing the plugin, the
//! channel it is distributed through. `bundle` delivers the *gate*, never clauses.
//!
//! The plugin is a **vendored, generated surface** — itself an instance of what
//! `temper` projects, so it is **byte-faithful where it carries prose**:
//! the skill and its topics are copied from the embedded SDK emit
//! ([`BUNDLED_REFERENCE`]), never re-rendered — only emit's managed-projection note is
//! dropped, since its edit-the-source remedy is false in an installed plugin. `plugin.json` and `marketplace.json` are members of
//! the `plugin-manifest` and `marketplace` kinds, rendered by the one write dispatch their
//! declared format names ([`crate::drift::project_bytes`]) — so what `bundle` publishes is
//! what `temper check` reads back off the same kinds, and no encoder is chosen here.
//! `hooks.json` is a plugin-cache artifact of neither kind; it stays on the canonical
//! whole-manifest write ([`crate::json_manifest::write_manifest`]).
//!
//! `bundle` ships channel 3's assets — the operate-the-gate skill and the
//! `SessionStart` hook — unconditionally: it never reads the surface it composes
//! over (`no compose.rs`/`contract.rs` edits). The
//! bundled contents are `temper`'s own gate-delivery assets, so `temper bundle` over
//! `temper`'s own surface produces `temper`'s own plugin — the dogfood target.
//!
//! Determinism: every written byte is a pure function of the embedded assets and the
//! crate version, so re-running `bundle` reproduces an identical tree — the vendored
//! plugin is reviewable and diff-stable.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::{Value as JsonValue, json};

/// The plugin's name — the `plugin.json` `name`, the marketplace entry name, and the
/// directory the operate-the-gate skill lands under (`skills/temper/`). `temper` is
/// outside the `deny`ed skill names (`anthropic`, `claude`), so the bundled skill
/// passes `temper`'s own skill contract — the dogfood holds.
/// Plugin skill locations (`skills/<name>/SKILL.md`) are plugin-root-relative, distinct
/// from the installed-harness skill locus (`**/.claude/skills/*/SKILL.md` per the `skill`
/// kind's governs; a plugin's contributed surfaces lie outside the corpus, unmodeled).
/// Both are documented plugin layouts
/// (code.claude.com/docs/en/plugins-reference, retrieved 2026-07-20).
const PLUGIN_NAME: &str = "temper";

/// The plugin/marketplace description — what the gate delivers, not what a good
/// harness is.
const PLUGIN_DESCRIPTION: &str = "The temper gate for a Claude Code harness: install it, check it against the \
     active contract, and run the advisory session-start gate — with the std-lib \
     default contracts embedded.";

/// The bundled **harness reference** — the SDK's `reference()` value emitted over the
/// memberless built-in program (the same program `src/builtin_lock.toml` is that program's
/// lock of), embedded byte-faithful: `SKILL.md` first, then its topic files beside it, each
/// a `(file name, bytes)` pair. Never hand-authored here: `tests/it/bundle.rs` re-emits the
/// reference through the built SDK and byte-compares every file, so the plugin's skill is
/// the very value a harness that declares `reference()` emits as its own member. Its
/// `name` is `temper` and it lands under `skills/temper/`, so `name-matches-dir` holds and
/// the skill passes `temper`'s own skill contract.
pub const BUNDLED_REFERENCE: &[(&str, &str)] = &[
    ("SKILL.md", include_str!("bundled_reference/SKILL.md")),
    (
        "kind-agent.md",
        include_str!("bundled_reference/kind-agent.md"),
    ),
    (
        "kind-command.md",
        include_str!("bundled_reference/kind-command.md"),
    ),
    (
        "kind-handler.md",
        include_str!("bundled_reference/kind-handler.md"),
    ),
    (
        "kind-hook.md",
        include_str!("bundled_reference/kind-hook.md"),
    ),
    (
        "kind-installed-plugin.md",
        include_str!("bundled_reference/kind-installed-plugin.md"),
    ),
    (
        "kind-known-marketplace.md",
        include_str!("bundled_reference/kind-known-marketplace.md"),
    ),
    (
        "kind-marketplace.md",
        include_str!("bundled_reference/kind-marketplace.md"),
    ),
    (
        "kind-mcp-server.md",
        include_str!("bundled_reference/kind-mcp-server.md"),
    ),
    ("kind-mcp.md", include_str!("bundled_reference/kind-mcp.md")),
    (
        "kind-memory.md",
        include_str!("bundled_reference/kind-memory.md"),
    ),
    (
        "kind-output-style.md",
        include_str!("bundled_reference/kind-output-style.md"),
    ),
    (
        "kind-plugin-manifest.md",
        include_str!("bundled_reference/kind-plugin-manifest.md"),
    ),
    (
        "kind-requirement.md",
        include_str!("bundled_reference/kind-requirement.md"),
    ),
    (
        "kind-rule.md",
        include_str!("bundled_reference/kind-rule.md"),
    ),
    (
        "kind-settings.md",
        include_str!("bundled_reference/kind-settings.md"),
    ),
    (
        "kind-skill.md",
        include_str!("bundled_reference/kind-skill.md"),
    ),
    (
        "kind-supporting-doc.md",
        include_str!("bundled_reference/kind-supporting-doc.md"),
    ),
    (
        "kind-tool.md",
        include_str!("bundled_reference/kind-tool.md"),
    ),
];

/// Errors raised while composing the plugin tree — the write side `bundle` owns.
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
enum BundleError {
    /// A plugin file or directory could not be written. Fail-loud:
    /// a placement that cannot be
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

    /// A manifest names a kind the embedded roster does not carry, so there is no declared
    /// format to render it through. Refused loud rather than falling back to an encoder
    /// picked here — a bundle written past its kind is exactly the ungated output this
    /// verb no longer produces.
    #[error("no embedded kind named `{kind}` to render a bundled manifest through")]
    #[diagnostic(code(temper::bundle::missing_kind))]
    MissingKind {
        /// The kind the roster does not carry.
        kind: &'static str,
    },

    /// A bundled manifest's kind declares a read-face-only format, so the write dispatch
    /// has nothing to render it through. Refused loud for the same reason a missing kind
    /// is: a bundle written past its kind's declared format is ungated output.
    #[error("embedded kind `{kind}` declares a read-only format — no write face renders it")]
    #[diagnostic(code(temper::bundle::unwritable_format))]
    UnwritableFormat {
        /// The kind whose declared format names no write face.
        kind: &'static str,
    },

    /// A kind passed to write_member must declare a Governs locus so the write path can
    /// be derived — a kind with no governs cannot be bundled. Refused loud since the kind's
    /// schema is fixed at build time, never authored by the user.
    #[error("embedded kind `{kind}` declares no Governs locus — cannot derive write path")]
    #[diagnostic(code(temper::bundle::no_governs))]
    NoGoverns {
        /// The kind that declares no governs.
        kind: &'static str,
    },
}

/// The typed result of a [`run`]: every file the plugin tree carries (relative to
/// the output root, sorted), plus the identity of what the bundle ships — channel 3
/// is gate-delivery only (the skill and the hook), never member delivery, so the
/// report names those shipped artifacts rather than counting the composed-over
/// surface's members.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleReport {
    /// The files written under the output root, as paths relative to it, sorted for
    /// a stable, reviewable report.
    pub files: Vec<PathBuf>,
    /// The name of the shipped operate-the-gate skill.
    pub skill_name: &'static str,
    /// The hook events the bundle wires, in emit order (currently just
    /// `SessionStart`).
    pub hook_events: Vec<&'static str>,
}

/// Compose a publishable plugin tree under `out`, alongside a `marketplace.json`
/// listing it.
///
/// `surface` names the harness the plugin is nominally composed over but is not
/// read: channel 3 ships the skill and the hook unconditionally, regardless of the
/// surface's contents. Emits the manifest, the operate-the-gate skill, the
/// `SessionStart` hook in its own `hooks.json`, and the marketplace listing. See the
/// module header for the byte-faithfulness and determinism guarantees.
pub fn run(_surface: &Path, out: &Path) -> miette::Result<BundleReport> {
    let mut files = Vec::new();

    // The plugin manifest and the marketplace listing it — each one member of the kind
    // that types it, rendered by that kind's declared format.
    write_member(out, "plugin-manifest", &plugin_manifest(), &mut files)?;
    write_member(out, "marketplace", &marketplace_manifest(), &mut files)?;

    // The harness reference — the SDK's emit, byte-faithful: `SKILL.md` and its topics.
    // Plugin skill location: `skills/<name>/SKILL.md`, supporting files beside it
    // (code.claude.com/docs/en/plugins-reference, retrieved 2026-07-20).
    // `/`-spelled like every other recorded path, never joined with the host separator.
    for (name, contents) in BUNDLED_REFERENCE {
        let relative = PathBuf::from(format!("skills/{PLUGIN_NAME}/{name}"));
        write_text(out, &relative, &strip_managed_note(contents), &mut files)?;
    }

    // The `SessionStart` hook, in its own `hooks.json`.
    // Plugin hooks location: `hooks/hooks.json` (code.claude.com/docs/en/plugins-reference, retrieved 2026-07-20).
    write_json(
        out,
        Path::new("hooks/hooks.json"),
        &hooks_manifest(),
        &mut files,
    )?;

    files.sort();
    Ok(BundleReport {
        files,
        skill_name: PLUGIN_NAME,
        hook_events: vec![BUNDLED_HOOK_EVENT],
    })
}

/// Drops emit's managed-projection note — the frontmatter `#` note or the leading
/// banner — from one bundled reference file. The note tells its reader to edit a
/// `.temper/` source and re-run emit, which an installed plugin's reader has neither of,
/// so the plugin ships the reference's content without it. Every other byte is kept.
pub fn strip_managed_note(contents: &str) -> String {
    use crate::placement::{BANNER, NOTE_COMMENT};
    let note_open = format!("---\n{NOTE_COMMENT}\n");
    if let Some(rest) = contents.strip_prefix(&note_open) {
        return format!("---\n{rest}");
    }
    if contents.starts_with(BANNER) {
        return crate::placement::strip_leading_banner(contents).to_string();
    }
    contents.to_string()
}

/// The `plugin.json` manifest (`.claude-plugin/plugin.json`): the plugin's identity
/// Claude Code reads to install it. Version tracks the crate so the plugin and the
/// binary it delivers move together.
fn plugin_manifest() -> BTreeMap<String, JsonValue> {
    BTreeMap::from([
        ("name".to_string(), json!(PLUGIN_NAME)),
        ("version".to_string(), json!(crate::VERSION)),
        ("description".to_string(), json!(PLUGIN_DESCRIPTION)),
        (
            "keywords".to_string(),
            json!(["claude", "claude-code", "harness", "linter", "gate"]),
        ),
    ])
}

/// The `marketplace.json` manifest listing this one plugin — the channel `temper` is
/// distributed through. The plugin's `source` is `./`: the marketplace root *is* the plugin
/// root, so the generated tree is a self-contained, installable bundle.
fn marketplace_manifest() -> BTreeMap<String, JsonValue> {
    BTreeMap::from([
        ("name".to_string(), json!(PLUGIN_NAME)),
        ("owner".to_string(), json!({ "name": PLUGIN_NAME })),
        (
            "plugins".to_string(),
            json!([
                {
                    "name": PLUGIN_NAME,
                    "source": "./",
                    "description": PLUGIN_DESCRIPTION,
                }
            ]),
        ),
    ])
}

/// The one event the bundled `hooks.json` registers a group on.
const BUNDLED_HOOK_EVENT: &str = "SessionStart";

/// The `hooks.json` manifest carrying the advisory `SessionStart` hook — the
/// exec-form `temper check . --reporter session-start` command Claude Code spawns at
/// session start. Same shape `temper install` merges into `.claude/settings.json`, so the
/// plugin and `install` deliver the identical gate.
/// Plugin hooks configuration resides at `hooks/hooks.json` in the plugin root
/// (code.claude.com/docs/en/plugins-reference, retrieved 2026-07-20).
fn hooks_manifest() -> BTreeMap<String, JsonValue> {
    BTreeMap::from([(
        "hooks".to_string(),
        json!({
            BUNDLED_HOOK_EVENT: [crate::install::session_start_group()]
        }),
    )])
}

/// Write `fields` as one member of the embedded `kind` under the path its declared Governs
/// locus names ([`crate::kind::locus_path`], so the path this records is `/`-spelled on
/// every host and a root-locus kind's `.` is dropped — one spelling across the whole
/// [`BundleReport::files`] listing, never the host's at one entry and the literal `/` at
/// the next). The bytes are whatever the
/// kind's declared format renders through the one write dispatch — `bundle` names the kind and
/// never picks an encoder, so the manifest it publishes is byte-for-byte the artifact `check`
/// reads back off that same kind. A manifest carries no body and no install metadata, so both
/// ride empty.
///
/// # Errors
///
/// Returns [`BundleError::MissingKind`] if the embedded roster carries no such kind,
/// [`BundleError::NoGoverns`] if the kind declares no Governs locus, and [`BundleError::Write`]
/// if the file cannot be written.
fn write_member(
    out: &Path,
    kind: &'static str,
    fields: &BTreeMap<String, JsonValue>,
    files: &mut Vec<PathBuf>,
) -> Result<(), BundleError> {
    let Some(definition) = crate::builtin_kind::definition(kind) else {
        return Err(BundleError::MissingKind { kind });
    };
    let Some(governs) = definition.governs else {
        return Err(BundleError::NoGoverns { kind });
    };
    let relative = crate::kind::locus_path(&governs.root, &governs.glob);
    let fields: Vec<(String, JsonValue)> = fields.clone().into_iter().collect();
    let Some(rendered) = crate::drift::project_bytes(definition.format, &fields, "", &[]) else {
        return Err(BundleError::UnwritableFormat { kind });
    };
    write_text(out, relative.as_ref(), &rendered, files)
}

/// Write a structured manifest as canonical pretty JSON under `<out>/<relative>`,
/// recording the relative path in `files`. Its top-level keys carry no declared
/// collection, so they route through the canonical whole-manifest write as pure residue —
/// the one encoder (trailing LF and all), never a second serde_json pretty-printer.
fn write_json(
    out: &Path,
    relative: &Path,
    residue: &BTreeMap<String, JsonValue>,
    files: &mut Vec<PathBuf>,
) -> Result<(), BundleError> {
    let json = crate::json_manifest::write_manifest(&[], residue);
    write_text(out, relative, &json, files)
}

/// Write text bytes verbatim to `<out>/<relative>`, creating parent directories and
/// recording the relative path in `files`. Byte-faithful: the bytes are written
/// exactly as given, never re-rendered.
fn write_text(
    out: &Path,
    relative: &Path,
    contents: &str,
    files: &mut Vec<PathBuf>,
) -> Result<(), BundleError> {
    let path = out.join(relative);
    crate::fs_util::write_creating_parents(&path, contents.as_bytes()).map_err(|source| {
        BundleError::Write {
            path: path.clone(),
            source,
        }
    })?;
    files.push(relative.to_path_buf());
    Ok(())
}

/// Render a bundle report for the terminal: one `wrote  <path>` line per file, then a
/// one-line tally naming what the bundle **ships** — the operate-the-gate skill and
/// the hook events it wires — mirroring [`crate::install::render`]. Channel 3 is
/// gate-delivery only, so this names shipped artifacts, never the composed-over
/// surface's member count.
#[must_use]
pub fn render(report: &BundleReport) -> String {
    let mut out = String::new();
    for file in &report.files {
        out.push_str(&format!("wrote  {}\n", file.display()));
    }
    out.push_str(&format!(
        "\nbundled {} file{} (ships: skill `{}`, {} hook{})\n",
        report.files.len(),
        crate::display::plural(report.files.len()),
        report.skill_name,
        report.hook_events.join(", "),
        crate::display::plural(report.hook_events.len()),
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::tmpdir;
    use std::fs;

    #[test]
    fn the_bundled_skill_passes_tempers_own_skill_contract() {
        // The dogfood: the operate-the-gate skill temper ships must itself satisfy
        // temper's skill contract — lowercase name matching its dir, present
        // description under the cap, no forbidden keys. Import it and gate it.
        let src = tmpdir("skill-src");
        let skill_dir = src.join("skills").join("temper");
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(skill_dir.join("SKILL.md"), BUNDLED_REFERENCE[0].1).unwrap();

        let kind = crate::builtin_kind::definition("skill").unwrap();
        let member =
            crate::frontmatter::Member::from_source(&kind, &skill_dir.join("SKILL.md")).unwrap();
        assert_eq!(member.id, "temper");
        let description = member
            .field("description")
            .and_then(|v| v.as_str())
            .expect("the operate-the-gate skill declares a description");
        assert!(!description.is_empty());
        assert!(description.len() <= 1024);
        // No Cursor keys leaked into the frontmatter.
        assert!(!member.has_field("globs"));
        assert!(!member.has_field("alwaysApply"));
    }

    #[test]
    fn manifests_are_well_formed_json() {
        // The structured manifests are valid JSON with the shape Claude Code reads.
        let plugin = plugin_manifest();
        assert_eq!(plugin["name"], "temper");
        assert_eq!(plugin["version"], crate::VERSION);

        let market = marketplace_manifest();
        assert_eq!(market["plugins"][0]["name"], "temper");
        assert_eq!(market["plugins"][0]["source"], "./");

        let hooks = hooks_manifest();
        assert_eq!(
            hooks["hooks"]["SessionStart"][0]["hooks"][0]["command"],
            crate::install::SESSION_START_COMMAND
        );
    }
}
