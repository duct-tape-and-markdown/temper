//! `temper install` — project the gate's wiring into the harness.
//!
//! Implements the `install` verb of `specs/architecture/50-distribution.md` ("Decision:
//! `install` projects the gate's wiring; drift keeps it synced"). Distribution is
//! not a second product; it is *placing the one gate* at every moment a harness is
//! authored, changes, or is used. `install` writes the placements a plugin cannot
//! carry — they live in *your* repo, not a shipped bundle — and wires the gate for
//! anyone who runs the binary without the plugin. Three placements:
//!
//! 1. the **`SessionStart` hook** (the exec-form `temper session-start` command)
//!    merged into `<root>/.claude/settings.json`;
//! 2. the **CI job** written to `<root>/.github/workflows/temper.yml` — the gate on
//!    human change, where humans collaborate;
//! 3. the **schema modeline** (`# yaml-language-server: $schema=…`) inserted into
//!    each artifact's frontmatter header — the gate at keystroke.
//!
//! Plus the assembly's **surface-authority** enforcement artifacts, wired per the
//! composed `authority` posture (`specs/architecture/20-surface.md`; `Shared` when the
//! `temper.toml` is absent or declares none): a managed-by **note** rides the
//! modeline machinery onto every non-memory frontmatter projection (a comment in a
//! memory `CLAUDE.md` costs context every session), and a `PreToolUse` **guard hook**
//! at Claude Code's write boundary informs-and-routes under `shared` / blocks under
//! `surface`. Both are placed and self-audited exactly like the three above.
//!
//! Each placement is projected as an ordinary artifact **under the three-state
//! drift engine** ([`drift::place`]) rather than a bespoke writer: `install`
//! computes each placement's desired bytes as an *idempotent merge* of temper's
//! wiring into the file as it stands, then hands the write to the engine. So a
//! re-run lands [`Unchanged`](drift::ApplyOutcome::Unchanged) (idempotent), a
//! `--dry-run` writes nothing, and a placement a human deleted is re-created (the
//! re-add direction). Because desired is a pure function of the current file,
//! `install` keeps no last-applied fingerprint of its own — it passes `None` to
//! [`drift::place`], and the merge re-derives the invariant every run.
//!
//! [`gate_installed`] is the read-only shadow: the same placements evaluated
//! dry-run, collapsed to **one** advisory [`Diagnostic`] carrying the missing/drifted
//! counts (so a real target's ~20 modelines don't bury the artifact findings).
//! `check` folds it in so temper verifies *its own* gate is wired —
//! "the harness checking that its self-check is wired" (`specs/architecture/50-distribution.md`).
//!
//! **Fail-loud** (`specs/architecture/50-distribution.md`, "Fail-loud delivery"): a placement
//! that cannot be written is a hard [`InstallError`] / [`drift::DriftError`], never
//! a silent skip — the gate's transport inherits the gate's soundness bar.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value as JsonValue, json};

use crate::check::Diagnostic;
use crate::compose::{AuthorLayer, Authority};
use crate::drift::{self, ApplyOutcome};
use crate::import;

/// The exec-form command Claude Code runs at session start: the `temper` binary
/// itself, checking the project root (`specs/architecture/50-distribution.md`, "the hook is the
/// temper binary itself"). The `.` is the harness under the running project.
const SESSION_START_COMMAND: &str = "temper session-start .";

/// The CI workflow `install` writes verbatim to `.github/workflows/temper.yml` —
/// the gate on human change, running `temper check` on pull requests. A file
/// temper owns wholesale, so it is placed rather than merged.
const CI_WORKFLOW: &str = "\
# Managed by `temper install` — the gate on human change (specs/architecture/50-distribution.md).
name: temper

on:
  pull_request:
  push:
    branches: [main]

jobs:
  gate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run the temper gate
        run: |
          temper import . --into .temper
          temper check .temper --reporter github
";

/// The placement label carried in the report and the self-verify diagnostics.
const SESSION_START: &str = "session-start hook";
/// The placement label for the CI job.
const CI_JOB: &str = "ci job";
/// The placement label for a schema modeline.
const MODELINE: &str = "schema modeline";
/// The placement label for the `PreToolUse` surface-authority guard hook.
const GUARD_HOOK: &str = "guard hook";
/// The placement label for a managed-by note.
const NOTE: &str = "managed-by note";

/// The rule id the self-verify diagnostics ([`gate_installed`]) carry.
const GATE_RULE: &str = "install.gate-installed";

/// The tool-name matcher the guard hook binds — Claude Code's own write boundary.
/// The guard binds *this provider's* writes only; the stated, unsolved limit
/// (`specs/architecture/20-surface.md`) is that other consumers of a shared file are not
/// instrumented by it.
const GUARD_MATCHER: &str = "Write|Edit|MultiEdit";

/// The stable token every guard command carries, whatever its degree — so a re-install
/// (or a posture change: `shared`↔`surface`) *replaces* the existing temper guard in
/// place rather than appending a second one.
const GUARD_MARKER: &str = "temper-managed projection";

/// The message the guard prints on a hit — carrying [`GUARD_MARKER`] and stating the
/// limit verbatim: the guard binds only this provider's writes, so other tools writing
/// a shared file are not bound by it. Deliberately apostrophe-free — the command
/// single-quotes it for the shell.
const NOTE_GUARD_MESSAGE: &str = "temper-managed projection: .claude/ is projected from the .temper/ surface — edit the surface and re-emit (temper re-add lifts a direct edit back). This guard binds only Claude Code writes; other tools writes are not bound by it.";

/// The extended-regex the guard command greps the `PreToolUse` payload for: a
/// `file_path` value under a `.claude/` projection locus. Matching the field (not the
/// whole payload) keeps a write whose *content* merely mentions `.claude/` from
/// tripping the guard. Kept deliberately conservative — a false negative routes to CI
/// (the backstop wall), a false positive would block honest work.
const GUARD_PATH_MATCH: &str = r#""file_path"[[:space:]]*:[[:space:]]*"[^"]*\.claude/"#;

/// The managed-by note's stable marker — the comment prefix its idempotence keys on,
/// so a second `install` neither duplicates nor rewrites the note.
const NOTE_MARKER: &str = "# temper: managed projection";

/// The managed-by note itself: a frontmatter comment stating the file is generated and
/// pointing at the surface. Cost-free metadata YAML frontmatter tolerates — never
/// stamped by `apply` (law 5 keeps the projection content-faithful; the note is
/// install's, not the surface body's).
const NOTE_COMMENT: &str = "# temper: managed projection — edit the .temper/ surface, not this generated file (temper re-add lifts a direct edit back).";

/// Errors raised while projecting the gate wiring — the read/parse side `install`
/// owns before it hands a placement's bytes to [`drift::place`] (whose own write
/// failures surface as [`drift::DriftError`]).
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum InstallError {
    /// A placement's existing file could not be read to merge into.
    #[error("failed to read {path}")]
    #[diagnostic(code(temper::install::read))]
    Read {
        /// The file whose read failed.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// `.claude/settings.json` exists but is not valid JSON, so the hook cannot be
    /// merged into it without clobbering the human's file.
    #[error("{path} is not valid JSON")]
    #[diagnostic(code(temper::install::settings_json))]
    Settings {
        /// The settings file that failed to parse.
        path: PathBuf,
        /// The JSON parse error.
        #[source]
        source: serde_json::Error,
    },

    /// `.claude/settings.json` parses but is not a JSON object, so there is no
    /// `hooks` map to merge the `SessionStart` entry into.
    #[error("{path} is not a JSON object")]
    #[diagnostic(code(temper::install::settings_shape))]
    SettingsShape {
        /// The settings file whose top level is not an object.
        path: PathBuf,
    },
}

/// One placement's outcome from [`run`]: which placement, at which path, and what
/// the three-state merge decided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallEntry {
    /// The placement label — the `SessionStart` hook, the CI job, or a modeline.
    pub placement: &'static str,
    /// The file the placement targets.
    pub path: PathBuf,
    /// What `install` did (or would do, under `--dry-run`) for this placement.
    pub outcome: ApplyOutcome,
}

/// The typed result of an [`run`]: every placement's outcome, in placement order
/// (the hook, the CI job, then one entry per modeled artifact's modeline).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallReport {
    /// Every projected placement.
    pub entries: Vec<InstallEntry>,
}

/// Project temper's gate wiring into the harness rooted at `root`, writing the
/// three placements through the three-state drift engine.
///
/// `root` is the project root — beside the `.claude/` and `.github/` the placements
/// land in (the current directory, by CLI default). Nothing is written under
/// `dry_run`; every outcome is still computed and reported. See the module header
/// for the per-placement projection and the idempotence / re-add guarantees.
pub fn run(root: &Path, dry_run: bool) -> miette::Result<InstallReport> {
    plan(root, dry_run)
}

/// Report whether temper's own gate is installed and undrifted at `root` — the
/// `check` self-verify (`specs/architecture/50-distribution.md`, "the harness checking that its
/// self-check is wired").
///
/// Evaluates the same three placements dry-run and folds every placement the merge
/// would write (missing, or drifted away from temper's wiring) into **one advisory**
/// [`Diagnostic`] carrying the counts — never one warn per placement, which on a
/// real target (~20 modelines) would bury the artifact findings the gate is there to
/// surface. Always `warn`, never `error`, so a not-yet-installed gate nudges without
/// failing `check`; empty when every placement is `Unchanged`. Best-effort: a hard
/// read/parse error is surfaced by `install` itself, not this self-verify, so an
/// unreadable placement here yields no diagnostic rather than aborting the
/// surrounding `check`.
#[must_use]
pub fn gate_installed(root: &Path) -> Vec<Diagnostic> {
    let Ok(report) = plan(root, true) else {
        return Vec::new();
    };
    // Tally the missing/drifted placements by kind. The hook, guard, and CI job are
    // single placements; modelines and managed-by notes are one per modeled artifact,
    // so they collapse to a count.
    let (mut hook, mut guard, mut ci, mut modelines, mut notes) = (false, false, false, 0u32, 0u32);
    for entry in &report.entries {
        if entry.outcome == ApplyOutcome::Unchanged {
            continue;
        }
        match entry.placement {
            SESSION_START => hook = true,
            GUARD_HOOK => guard = true,
            CI_JOB => ci = true,
            NOTE => notes += 1,
            _ => modelines += 1,
        }
    }
    if !hook && !guard && !ci && modelines == 0 && notes == 0 {
        return Vec::new();
    }

    let mut parts = Vec::new();
    if hook {
        parts.push(SESSION_START.to_string());
    }
    if guard {
        parts.push(GUARD_HOOK.to_string());
    }
    if ci {
        parts.push(CI_JOB.to_string());
    }
    if modelines > 0 {
        let plural = if modelines == 1 { "" } else { "s" };
        parts.push(format!("{modelines} schema modeline{plural}"));
    }
    if notes > 0 {
        let plural = if notes == 1 { "" } else { "s" };
        parts.push(format!("{notes} managed-by note{plural}"));
    }

    vec![Diagnostic::warn(
        GATE_RULE,
        root.to_string_lossy().into_owned(),
        format!(
            "temper's gate is not installed or has drifted — run `temper install` (missing or drifted: {})",
            parts.join(", ")
        ),
    )]
}

/// The shared projection both [`run`] and [`gate_installed`] drive: compute each
/// placement's desired bytes and route the write through [`drift::place`]. With
/// `dry_run` set nothing lands, so the self-verify reuses it read-only.
fn plan(root: &Path, dry_run: bool) -> miette::Result<InstallReport> {
    let authority = load_authority(root)?;
    let mut entries = Vec::new();

    // 1. The SessionStart hook and the PreToolUse guard, both merged into one
    //    .claude/settings.json. A single write carries both; the per-hook presence
    //    drives two placement outcomes so `gate_installed` audits each independently.
    let settings_path = root.join(".claude").join("settings.json");
    let existing = read_optional(&settings_path)?;
    let settings = project_settings(&settings_path, existing.as_deref(), authority)?;
    drift::place(&settings_path, &settings.desired, None, dry_run)?;
    entries.push(InstallEntry {
        placement: SESSION_START,
        outcome: placement_outcome(settings.hook_present),
        path: settings_path.clone(),
    });
    entries.push(InstallEntry {
        placement: GUARD_HOOK,
        outcome: placement_outcome(settings.guard_present),
        path: settings_path,
    });

    // 2. The CI job — a file temper owns wholesale, placed verbatim.
    let ci_path = root.join(".github").join("workflows").join("temper.yml");
    entries.push(InstallEntry {
        placement: CI_JOB,
        outcome: drift::place(&ci_path, CI_WORKFLOW, None, dry_run)?,
        path: ci_path,
    });

    // 3. Per frontmatter projection: the managed-by note, then the schema modeline.
    //    The note is applied first so the modeline stays the leading frontmatter line,
    //    and SKIPS memory projections — a comment in a CLAUDE.md costs context every
    //    session (`specs/architecture/20-surface.md`).
    for target in modeline_targets(root)? {
        let source = fs::read_to_string(&target.path).map_err(|source| InstallError::Read {
            path: target.path.clone(),
            source,
        })?;
        let mut current = source;

        if !target.is_memory
            && let Some(desired) = project_note(&current)
        {
            let outcome = drift::place(&target.path, &desired, None, dry_run)?;
            entries.push(InstallEntry {
                placement: NOTE,
                outcome,
                path: target.path.clone(),
            });
            current = desired;
        }

        // An artifact with no frontmatter has nothing to validate — skip it rather
        // than synthesise a header a human never wrote.
        if let Some(desired) = project_modeline(&current, &target.schema_ref) {
            entries.push(InstallEntry {
                placement: MODELINE,
                outcome: drift::place(&target.path, &desired, None, dry_run)?,
                path: target.path,
            });
        }
    }

    Ok(InstallReport { entries })
}

/// The assembly's declared surface-authority posture, read from `<root>/temper.toml`
/// (`specs/architecture/20-surface.md`, "surface authority is a declared posture").
/// [`Authority::Shared`] when the file is absent or declares no `authority` — the
/// default: temper fabricates no enforcement the author did not ask for.
fn load_authority(root: &Path) -> miette::Result<Authority> {
    Ok(AuthorLayer::load(&root.join("temper.toml"))?
        .map(|layer| layer.authority())
        .unwrap_or_default())
}

/// Map "was this placement already in its desired state" onto the settings outcomes.
/// The settings file carries no baseline fingerprint (idempotent placement), so a
/// placement is only ever [`Applied`](ApplyOutcome::Applied) (absent/drifted) or
/// [`Unchanged`](ApplyOutcome::Unchanged) — never `Conflicted`.
fn placement_outcome(present: bool) -> ApplyOutcome {
    if present {
        ApplyOutcome::Unchanged
    } else {
        ApplyOutcome::Applied
    }
}

/// Read a file that may not exist, distinguishing "absent" (`Ok(None)`) from a
/// real read failure. The absent case is normal — a harness with no
/// `.claude/settings.json` yet is exactly what `install` is for.
fn read_optional(path: &Path) -> Result<Option<String>, InstallError> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(Some(text)),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(source) => Err(InstallError::Read {
            path: path.to_path_buf(),
            source,
        }),
    }
}

/// The desired `.claude/settings.json` plus whether each temper hook was already in
/// its desired state before the merge — so `install` reports the `SessionStart` hook
/// and the `PreToolUse` guard as distinct placements though they share one file.
struct SettingsProjection {
    /// The re-emitted settings JSON (canonical pretty, trailing newline).
    desired: String,
    /// Whether the `SessionStart` hook was already present.
    hook_present: bool,
    /// Whether the guard hook was already present *at the desired degree* — a posture
    /// change (`shared`↔`surface`) leaves a differing command, so this reads `false`
    /// and the guard reports as (re)applied.
    guard_present: bool,
}

/// Project the desired `.claude/settings.json` — the existing settings with the
/// `SessionStart` hook and the `PreToolUse` guard (at `authority`'s degree) merged in,
/// or a fresh document when the file is absent or empty. Idempotent: an already-present
/// temper hook at its desired shape is left alone, so re-merging reproduces the bytes.
///
/// JSON carries no comments and its object order is not semantically meaningful, so
/// the merge parses, adds/updates each hook, and re-emits canonical pretty JSON —
/// there is no format-preserving JSON editor to round-trip through the way
/// `toml_edit` round-trips the TOML surface.
fn project_settings(
    path: &Path,
    existing: Option<&str>,
    authority: Authority,
) -> Result<SettingsProjection, InstallError> {
    let mut root = match existing {
        Some(text) if !text.trim().is_empty() => {
            serde_json::from_str::<JsonValue>(text).map_err(|source| InstallError::Settings {
                path: path.to_path_buf(),
                source,
            })?
        }
        _ => json!({}),
    };

    let object = root
        .as_object_mut()
        .ok_or_else(|| InstallError::SettingsShape {
            path: path.to_path_buf(),
        })?;

    // Presence *before* the merge — the per-hook outcome install reports.
    let guard = guard_command(authority);
    let hook_present = session_start_present(object);
    let guard_present = guard_present(object, &guard);

    // `hooks` is an object of event-name -> array-of-groups.
    let hooks = object
        .entry("hooks")
        .or_insert_with(|| json!({}))
        .as_object_mut()
        .ok_or_else(|| InstallError::SettingsShape {
            path: path.to_path_buf(),
        })?;

    // Ensure a `SessionStart` group whose command is the temper binary.
    let session_start = hooks
        .entry("SessionStart")
        .or_insert_with(|| json!([]))
        .as_array_mut()
        .ok_or_else(|| InstallError::SettingsShape {
            path: path.to_path_buf(),
        })?;
    if !hooks_session_start(session_start) {
        session_start.push(json!({
            "hooks": [
                { "type": "command", "command": SESSION_START_COMMAND }
            ]
        }));
    }

    // Ensure the `PreToolUse` guard at the posture's degree — replacing any existing
    // temper guard (its command may carry a stale degree) rather than appending.
    let pre_tool_use = hooks
        .entry("PreToolUse")
        .or_insert_with(|| json!([]))
        .as_array_mut()
        .ok_or_else(|| InstallError::SettingsShape {
            path: path.to_path_buf(),
        })?;
    upsert_guard(pre_tool_use, &guard);

    // A trailing newline keeps the file POSIX-clean; pretty JSON is deterministic,
    // so the whole projection is idempotent.
    let desired = format!(
        "{}\n",
        serde_json::to_string_pretty(&root).map_err(|source| InstallError::Settings {
            path: path.to_path_buf(),
            source,
        })?
    );
    Ok(SettingsProjection {
        desired,
        hook_present,
        guard_present,
    })
}

/// The guard hook's command at `authority`'s degree. Both greps the `PreToolUse`
/// payload for a `.claude/` `file_path` and, on a hit, states the managed-by message —
/// then routes (`shared`, advisory: exit 0) or blocks (`surface`, required: exit 2).
/// Degree maps onto the severity vocabulary (advisory / required) so temper never
/// escalates past the declared posture (`specs/architecture/20-surface.md`). The message states
/// the limit verbatim: the guard binds only this provider's writes.
fn guard_command(authority: Authority) -> String {
    match authority {
        // Advisory: inform and route to `re-add`, never block.
        Authority::Shared => {
            format!("grep -qE '{GUARD_PATH_MATCH}' && echo '{NOTE_GUARD_MESSAGE}' >&2; exit 0")
        }
        // Required: block the write (exit 2) so the projection is not clobbered.
        Authority::Surface => format!(
            "grep -qE '{GUARD_PATH_MATCH}' && {{ echo '{NOTE_GUARD_MESSAGE}' >&2; exit 2; }}; exit 0"
        ),
    }
}

/// Whether a `SessionStart` group carrying temper's exec-form command is already
/// present — the idempotence check, so a second `install` neither duplicates the hook
/// nor clobbers a human's other `SessionStart` groups.
fn session_start_present(object: &serde_json::Map<String, JsonValue>) -> bool {
    object
        .get("hooks")
        .and_then(|hooks| hooks.get("SessionStart"))
        .and_then(JsonValue::as_array)
        .is_some_and(|groups| hooks_session_start(groups))
}

/// See [`session_start_present`] — the same check specialized to the array itself,
/// used mid-merge where only the `SessionStart` array is in hand.
fn hooks_session_start(groups: &[JsonValue]) -> bool {
    groups
        .iter()
        .any(|group| group_has_command(group, |command| command == SESSION_START_COMMAND))
}

/// Whether a `PreToolUse` group carrying *this exact* guard command is already present.
/// A differing command (a posture change) reads `false`, so the guard reports as
/// (re)applied and [`upsert_guard`] rewrites it.
fn guard_present(object: &serde_json::Map<String, JsonValue>, guard: &str) -> bool {
    object
        .get("hooks")
        .and_then(|hooks| hooks.get("PreToolUse"))
        .and_then(JsonValue::as_array)
        .is_some_and(|groups| {
            groups
                .iter()
                .any(|group| group_has_command(group, |command| command == guard))
        })
}

/// Insert or update temper's guard in a `PreToolUse` groups array: an existing temper
/// guard (identified by [`GUARD_MARKER`], whatever its degree) has its command set to
/// `guard`, so a posture change rewrites in place; absent, a fresh group is appended.
fn upsert_guard(groups: &mut Vec<JsonValue>, guard: &str) {
    for group in groups.iter_mut() {
        if !group_has_command(group, |command| command.contains(GUARD_MARKER)) {
            continue;
        }
        if let Some(hooks) = group.get_mut("hooks").and_then(JsonValue::as_array_mut) {
            for hook in hooks.iter_mut() {
                let is_guard = hook
                    .get("command")
                    .and_then(JsonValue::as_str)
                    .is_some_and(|command| command.contains(GUARD_MARKER));
                if is_guard {
                    hook["command"] = json!(guard);
                }
            }
        }
        return;
    }
    groups.push(json!({
        "matcher": GUARD_MATCHER,
        "hooks": [
            { "type": "command", "command": guard }
        ]
    }));
}

/// Whether a hook group carries a `command` string satisfying `pred` — the shared
/// spine of the `SessionStart` and guard presence checks.
fn group_has_command(group: &JsonValue, pred: impl Fn(&str) -> bool) -> bool {
    group
        .get("hooks")
        .and_then(JsonValue::as_array)
        .is_some_and(|hooks| {
            hooks.iter().any(|hook| {
                hook.get("command")
                    .and_then(JsonValue::as_str)
                    .is_some_and(&pred)
            })
        })
}

/// One artifact the modeline placement targets: its source path, the `$schema`
/// reference the modeline points at (relative to the artifact's own directory), and
/// whether it is a **memory** projection — which takes the modeline but skips the
/// managed-by note (a comment in a `CLAUDE.md` costs context every session).
struct ModelineTarget {
    path: PathBuf,
    schema_ref: String,
    is_memory: bool,
}

/// Discover the modeled artifacts the modeline placement covers — every skill
/// `SKILL.md` and rule `*.md` under `root` — reusing `import`'s own per-kind
/// discovery so the set matches exactly what `check` validates. Each carries the
/// relative `$schema` reference its modeline points at.
fn modeline_targets(root: &Path) -> miette::Result<Vec<ModelineTarget>> {
    let mut targets = Vec::new();
    // Iterate the qualified built-in set and thread each parsed kind through discovery,
    // never re-resolving a bare name at the scan (`specs/architecture/15-kinds.md`,
    // "Decision: kind identity carries a provider axis"). Covers every embedded built-in,
    // not just `skill`/`rule`.
    for kind in crate::builtin_kind::definitions()?.values() {
        for source in import::discover_builtin(root, kind)? {
            let schema_ref = schema_ref(root, &source, &kind.name);
            targets.push(ModelineTarget {
                path: source,
                schema_ref,
                is_memory: kind.name == "memory",
            });
        }
    }
    Ok(targets)
}

/// The relative `$schema` reference a modeline in `source` points at: the
/// conventional per-kind schema `temper schema` emits under `<root>/.temper/schema/`,
/// expressed relative to the artifact file's own directory so an editor resolves it
/// (`../../.temper/schema/rule.json` for `.claude/rules/rust.md`). `install` wires
/// the modeline; emitting the schema file it targets is `temper schema`'s job.
fn schema_ref(root: &Path, source: &Path, kind: &str) -> String {
    // How deep the artifact's directory sits below the root — one `..` per level to
    // climb back up to the `.temper/schema/` the modeline references.
    let depth = source
        .parent()
        .and_then(|dir| dir.strip_prefix(root).ok())
        .map(|rel| rel.components().count())
        .unwrap_or(0);
    let climb = "../".repeat(depth);
    format!("{climb}.temper/schema/{kind}.json")
}

/// Project an artifact source with the schema modeline inserted as the first line
/// of its frontmatter, or `None` when it has no frontmatter to validate (leave it
/// untouched rather than synthesise a header). Idempotent: an artifact already
/// carrying a `yaml-language-server` modeline is returned verbatim, so re-running
/// neither duplicates nor rewrites it — including one a human pointed elsewhere.
///
/// Byte-faithful: the modeline is the only inserted bytes; every other byte —
/// the other frontmatter fields, comments, and the whole body — is preserved
/// exactly (`.claude/rules/rust.md`, round-trip discipline).
fn project_modeline(source: &str, schema_ref: &str) -> Option<String> {
    let rest = source.strip_prefix("---\n")?;
    let inner = frontmatter_inner(rest)?;
    if inner
        .lines()
        .any(|line| line.trim_start().starts_with("# yaml-language-server:"))
    {
        return Some(source.to_string());
    }
    let modeline = format!("# yaml-language-server: $schema={schema_ref}");
    Some(format!("---\n{modeline}\n{rest}"))
}

/// Project an artifact source with the managed-by note inserted as a frontmatter
/// comment, or `None` when it has no frontmatter to carry it (a memory `CLAUDE.md`
/// has none, and the caller already skips memory besides). Applied *before* the
/// modeline so the modeline stays the leading line; idempotent by [`NOTE_MARKER`],
/// so re-running neither duplicates nor rewrites the note.
///
/// Byte-faithful (`.claude/rules/rust.md`, round-trip discipline): the note is the
/// only inserted bytes. The note rides `install`, never `apply` — a YAML comment is
/// not authored surface content, so the content-faithful projector (law 5) never
/// re-emits it (`specs/architecture/20-surface.md`).
fn project_note(source: &str) -> Option<String> {
    let rest = source.strip_prefix("---\n")?;
    let inner = frontmatter_inner(rest)?;
    if inner
        .lines()
        .any(|line| line.trim_start().starts_with(NOTE_MARKER))
    {
        return Some(source.to_string());
    }
    Some(format!("---\n{NOTE_COMMENT}\n{rest}"))
}

/// The frontmatter text between the delimiters of `rest` — everything after the
/// opening `---\n` (the caller's `rest`) up to the closing `---` line — or `None`
/// when there is no closing delimiter (an opening `---` that is really prose, not a
/// frontmatter block). Mirrors the delimiter detection the loaders use.
fn frontmatter_inner(rest: &str) -> Option<&str> {
    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        let content = line.strip_suffix('\n').unwrap_or(line);
        if content.trim_end() == "---" {
            return Some(&rest[..offset]);
        }
        offset += line.len();
    }
    None
}

/// Render an install report for the terminal: one `<outcome>  <placement>  <path>`
/// line per entry, then a one-line tally — mirroring [`drift::render_apply`].
#[must_use]
pub fn render(report: &InstallReport) -> String {
    let mut out = String::new();
    let (mut applied, mut unchanged, mut conflicted) = (0u32, 0u32, 0u32);
    for entry in &report.entries {
        match entry.outcome {
            ApplyOutcome::Applied => applied += 1,
            ApplyOutcome::Unchanged => unchanged += 1,
            ApplyOutcome::Conflicted => conflicted += 1,
        }
        out.push_str(&format!(
            "{:<10}  {:<18}  {}\n",
            entry.outcome.label(),
            entry.placement,
            entry.path.display()
        ));
    }
    out.push_str(&format!(
        "\n{applied} applied, {unchanged} unchanged, {conflicted} conflicted\n"
    ));
    out
}
