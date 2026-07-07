//! `temper install` — the one on-ramp (`specs/architecture/20-surface.md`, "install —
//! the front door; the lift, once").
//!
//! `install` opens with a discovery report ([`discover`]/[`render_discovery`]) —
//! findings first, ceremony after — then asks exactly one question via [`Represent`]:
//! represent this project as a temper program?
//!
//! - **No** wires the `SessionStart` reporter alone ([`place_settings_only`]) and
//!   stops — the stranger gate at session start, Node-free forever.
//! - **Yes** requires Node (checked up front, refused loud with instructions when
//!   absent), scaffolds the SDK program once if none exists yet — the lift
//!   ([`scaffold`]): a member module per discovered artifact, `file()` over the
//!   original text, zero file moves, plus a `harness.ts` skeleton — ensures the
//!   `@dtmd/temper` dependency ([`ensure_dependency`]), runs the first `emit` (the
//!   adoption moment, [`drift::emit_program`]), and places the guard hook / managed-by
//!   note / schema modeline only at paths the fresh lock declares **emit-owned**
//!   ([`drift::emit_owned_targets`], [`evaluate_placements`]) — a lifted member's own
//!   `file()` path is authored territory, never a guard claim.
//!
//! [`gate_installed`] is the read-only shadow `check` folds in: the same placement
//! evaluation, dry-run, collapsed to one advisory [`Diagnostic`]. It never scaffolds,
//! installs a dependency, or emits — `install` alone adopts (`specs/architecture/
//! 20-surface.md`, "the bare binary checks; it never adopts").
//!
//! **Fail-loud**: a placement, a scaffold write, or a dependency/emit step that
//! cannot complete is a hard [`InstallError`] / propagated [`miette::Report`], never
//! a silent skip.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use regex::Regex;
use serde_json::{Value as JsonValue, json};

use crate::builtin_kind;
use crate::check::Diagnostic;
use crate::compose::EnforcementMode;
use crate::drift::{self, ApplyOutcome, EmitReport};
use crate::import;
use crate::kind::UnitShape;

/// The workspace directory a represented project's SDK program lives under, beside
/// the harness it governs (`specs/architecture/20-surface.md`, "The surface").
const WORKSPACE_DIR: &str = ".temper";

/// The SDK program's entry file — scaffolded once by the lift, run by every
/// subsequent `emit` (`specs/architecture/20-surface.md`, "The seam").
const HARNESS_ENTRY: &str = "harness.ts";

/// The npm package name the yes-path ensures as a dependency.
const SDK_PACKAGE: &str = "@dtmd/temper";

/// The dependency range `install` writes when `.temper/package.json` does not
/// already declare [`SDK_PACKAGE`] — the current published line
/// (`docs/ledger.md`, "`@dtmd/temper@0.0.2` on npm").
const SDK_VERSION_RANGE: &str = "^0.0.2";

/// The exec-form command Claude Code runs at session start: the `temper` binary
/// itself, checking the project root under the advisory session-start reporter
/// (`specs/architecture/20-surface.md`, "session-start is a reporter of `check`, not a verb").
/// The `.` is the harness under the running project.
const SESSION_START_COMMAND: &str = "temper check . --reporter session-start";

/// The placement label carried in the report and the self-verify diagnostics.
const SESSION_START: &str = "session-start hook";
/// The placement label for a schema modeline.
const MODELINE: &str = "schema modeline";
/// The placement label for the `PreToolUse` enforcement-mode guard hook.
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

/// The exec-form command the `PreToolUse` guard hook runs: the `temper` binary's own
/// `guard` subcommand, reading the payload from stdin and deciding at the harness's
/// declared posture (`specs/architecture/20-surface.md`). The `.` roots the lock the
/// posture is read from — the project Claude Code runs the hook in.
const GUARD_COMMAND: &str = "temper guard .";

/// The stable token the guard command carries so a re-install *replaces* the existing
/// temper guard in place rather than appending a second one. The command is
/// posture-independent (the subcommand reads the posture live), so this is simply the
/// subcommand invocation.
const GUARD_MARKER: &str = "temper guard";

/// The message `temper guard` prints on a projection hit — stating the limit verbatim:
/// the guard binds only this provider's writes, so other tools writing a shared file are
/// not bound by it. Public so the `guard` subcommand ([`main`]) prints it whether it
/// warns (`shared`) or blocks (`surface`).
pub const GUARD_MESSAGE: &str = "temper-managed projection: .claude/ is projected from the .temper/ surface — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit. This guard binds only Claude Code writes; other tools writes are not bound by it.";

/// The extended-regex `temper guard` greps the `PreToolUse` payload for: a `file_path`
/// value under a `.claude/` projection locus. Matching the field (not the whole payload)
/// keeps a write whose *content* merely mentions `.claude/` from tripping the guard. Kept
/// deliberately conservative — a false negative routes to CI (the backstop wall), a false
/// positive would block honest work.
const GUARD_PATH_MATCH: &str = r#""file_path"[[:space:]]*:[[:space:]]*"[^"]*\.claude/"#;

/// The managed-by note's stable marker — the comment prefix that *locates* an already
/// placed note (so a second `install` never duplicates it); whether that note is then
/// left verbatim or re-placed keys on the line's bytes vs [`NOTE_COMMENT`], not this
/// prefix (`project_note`, content-drift-aware).
const NOTE_MARKER: &str = "# temper: managed projection";

/// The schema modeline's stable marker — the frontmatter comment prefix `install` keys
/// its idempotence on and `emit` keys its preservation on, so both projectors agree on
/// which line is the modeline.
const MODELINE_MARKER: &str = "# yaml-language-server:";

/// The managed-by note itself: a frontmatter comment stating the file is generated and
/// pointing at the surface. Cost-free metadata YAML frontmatter tolerates — never
/// stamped by `emit` (law 5 keeps the projection content-faithful; the note is
/// install's, not the surface body's).
const NOTE_COMMENT: &str = "# temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file.";

/// The one question `install` asks, exactly once, after the discovery report
/// (`specs/architecture/20-surface.md`, "install — the front door"): there is one
/// genuine fork in the world — a harness is represented or it is not.
pub const REPRESENT_QUESTION: &str = "Represent this project as a temper program? [y/N]";

/// Errors raised while projecting the gate wiring — the read/parse side `install`
/// owns before it hands a placement's bytes to [`drift::place`] (whose own write
/// failures surface as [`drift::DriftError`]), plus the yes-path's scaffold/
/// dependency/preflight failures.
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

    /// A scaffold file (a member module, `harness.ts`, or `package.json`) could not
    /// be written.
    #[error("failed to write {path}")]
    #[diagnostic(code(temper::install::write))]
    Write {
        /// The destination path that failed.
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

    /// Node.js was not found on `PATH` — the represented (yes) path requires it
    /// (`specs/architecture/20-surface.md`, "the represented path requires Node and
    /// `.temper/`, checked up front and refused with instructions when absent").
    #[error("Node.js is required to represent this project as a temper program")]
    #[diagnostic(
        code(temper::install::node_missing),
        help(
            "install Node.js (https://nodejs.org, or via nvm/fnm) and re-run `temper install --yes`"
        )
    )]
    NodeMissing,

    /// `.temper/package.json` exists but is not valid JSON.
    #[error("{path} is not valid JSON")]
    #[diagnostic(code(temper::install::package_json))]
    PackageJson {
        /// The package manifest that failed to parse.
        path: PathBuf,
        /// The JSON parse error.
        #[source]
        source: serde_json::Error,
    },

    /// `.temper/package.json` parses but is not a JSON object (or its
    /// `dependencies` key is not one).
    #[error("{path} is not a JSON object")]
    #[diagnostic(code(temper::install::package_json_shape))]
    PackageJsonShape {
        /// The package manifest whose shape is wrong.
        path: PathBuf,
    },

    /// `npm install` could not be spawned in the scaffolded `.temper/` workspace.
    #[error("failed to spawn \"npm install\" in {path}")]
    #[diagnostic(code(temper::install::dependency_spawn))]
    DependencySpawn {
        /// The workspace directory the spawn was attempted in.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },

    /// `npm install` exited non-zero while ensuring the `@dtmd/temper` dependency.
    #[error("\"npm install\" failed in {path}:\n{stderr}")]
    #[diagnostic(code(temper::install::dependency_install))]
    DependencyInstall {
        /// The workspace directory `npm install` ran in.
        path: PathBuf,
        /// The subprocess's captured stderr.
        stderr: String,
    },
}

/// The one question's answer (`specs/architecture/20-surface.md`, "install — the front
/// door"): there is one genuine fork in the world, so exactly one boolean fork here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Represent {
    /// Represent this project as a temper program — the lift + first emit path.
    Yes,
    /// Do not — wire the session-start reporter alone.
    No,
}

/// The discovery walk's findings — "what the walk found (members by kind...)"
/// (`specs/architecture/20-surface.md`, "install — the front door"), reported before
/// the one question and reused by the yes-path's scaffold so the lift lifts exactly
/// what was reported, never a re-walked, possibly-differing set.
#[derive(Debug, Clone, Default)]
pub struct DiscoveryReport {
    /// Discovered member source files, keyed by the kind's bare row label
    /// (`specs/architecture/15-kinds.md`) — every embedded built-in kind.
    pub members: BTreeMap<String, Vec<PathBuf>>,
}

impl DiscoveryReport {
    /// The total member count across every discovered kind.
    #[must_use]
    pub fn total(&self) -> usize {
        self.members.values().map(Vec::len).sum()
    }
}

/// Walk `root` for every embedded built-in kind's members — the discovery report's
/// data, computed once and shared by the printed report and the yes-path's scaffold.
///
/// # Errors
/// Returns a [`miette::Report`] if the embedded kind set fails to load or a kind's
/// discovery walk fails to read a directory.
pub fn discover(root: &Path) -> miette::Result<DiscoveryReport> {
    let mut members = BTreeMap::new();
    for kind in builtin_kind::definitions()?.values() {
        let files = import::discover_builtin(root, kind)?;
        members.insert(kind.name.clone(), files);
    }
    Ok(DiscoveryReport { members })
}

/// Render the discovery report for the terminal — findings first, ceremony after
/// (`specs/architecture/20-surface.md`): member counts by kind, or a plain
/// statement that nothing was found.
#[must_use]
pub fn render_discovery(report: &DiscoveryReport) -> String {
    let mut out = String::from("discovery:\n");
    if report.total() == 0 {
        out.push_str("  no members found under this project's known kinds\n");
        return out;
    }
    for (kind, files) in &report.members {
        if files.is_empty() {
            continue;
        }
        out.push_str(&format!("  {kind:<20} {}\n", files.len()));
    }
    out
}

/// One placement's outcome from [`run`]/[`gate_installed`]: which placement, at
/// which path, and what the three-state merge decided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallEntry {
    /// The placement label — the `SessionStart` hook, the guard, a modeline, or a note.
    pub placement: &'static str,
    /// The file the placement targets.
    pub path: PathBuf,
    /// What `install` did (or would do, under `--dry-run`) for this placement.
    pub outcome: ApplyOutcome,
}

/// The typed result of [`run`]: the represent decision, how many members the lift
/// scaffolded (`0` when already represented, declining, or previewing), the first
/// (or subsequent) `emit`'s report when the yes-path ran one, and every projected
/// placement in stable order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallOutcome {
    /// Whether this project is represented as a temper program after this run.
    pub represented: bool,
    /// Members the lift scaffolded this run.
    pub scaffolded: usize,
    /// The `emit` report from the yes-path's (first or subsequent) emit, when one ran.
    pub emit: Option<EmitReport>,
    /// Every projected placement.
    pub entries: Vec<InstallEntry>,
}

/// Run `install` at `root`: the one question already answered as `represent`, over
/// the `discovery` report the caller printed before asking it. Nothing is written
/// under `dry_run`; every outcome is still computed and reported where that is
/// possible without actually scaffolding, installing a dependency, or emitting (see
/// the module header for exactly what a fresh `--dry-run` yes-path can and cannot
/// preview).
///
/// # Errors
/// Returns a [`miette::Report`] on any read/write/subprocess failure along the
/// chosen path — see [`InstallError`] and [`drift::DriftError`].
pub fn run(
    root: &Path,
    discovery: &DiscoveryReport,
    represent: Represent,
    dry_run: bool,
) -> miette::Result<InstallOutcome> {
    match represent {
        Represent::No => Ok(InstallOutcome {
            represented: false,
            scaffolded: 0,
            emit: None,
            entries: place_settings_only(root, dry_run)?,
        }),
        Represent::Yes => run_represented(root, discovery, dry_run),
    }
}

/// The yes-path: require Node, scaffold once if unrepresented, ensure the SDK
/// dependency, run `emit`, then place the guard/note/modeline at the fresh lock's
/// emit-owned targets (`specs/architecture/20-surface.md`, "install — the front door").
fn run_represented(
    root: &Path,
    discovery: &DiscoveryReport,
    dry_run: bool,
) -> miette::Result<InstallOutcome> {
    ensure_node_available()?;

    let temper_dir = root.join(WORKSPACE_DIR);
    let harness_entry = temper_dir.join(HARNESS_ENTRY);
    let already_scaffolded = harness_entry.is_file();

    let scaffolded = if already_scaffolded {
        0
    } else {
        scaffold(root, &temper_dir, discovery, dry_run)?
    };

    if !dry_run && !dependency_resolves(&temper_dir) {
        ensure_dependency(&temper_dir)?;
    }

    // A fresh (never-scaffolded) `--dry-run` preview has no `harness.ts` on disk to
    // run `node` over — there is nothing real to emit yet, so the preview stops at
    // the scaffold count rather than inventing an emit report. Once the program
    // exists (already represented, or this very run just wrote it for real),
    // `emit_program` runs for real — its own `dry_run` governs whether it writes.
    let emit = if dry_run && !already_scaffolded {
        None
    } else {
        Some(drift::emit_program(
            &temper_dir,
            drift::EmitOptions {
                dry_run,
                frozen: false,
            },
        )?)
    };

    let entries = if emit.is_some() {
        evaluate_placements(root, &temper_dir, dry_run)?
    } else {
        Vec::new()
    };

    Ok(InstallOutcome {
        represented: true,
        scaffolded,
        emit,
        entries,
    })
}

/// Report whether temper's own gate is installed and undrifted at `root` — the
/// `check` self-verify (`specs/architecture/20-surface.md`, "the harness checking that its
/// self-check is wired").
///
/// Never scaffolds, installs a dependency, or emits — only [`run`] adopts
/// (`specs/architecture/20-surface.md`, "the bare binary checks; it never adopts").
/// Evaluates the placements a *represented* project's current lock justifies, or —
/// on an unrepresented project (no `.temper/harness.ts`) — the session-start hook
/// alone, both dry-run, folded into **one advisory** [`Diagnostic`] carrying the
/// missing/drifted counts. Always `warn`, never `error`; empty when every placement
/// is already in place.
#[must_use]
pub fn gate_installed(root: &Path) -> Vec<Diagnostic> {
    let temper_dir = root.join(WORKSPACE_DIR);
    let represented = temper_dir.join(HARNESS_ENTRY).is_file();
    let Ok(entries) = (if represented {
        evaluate_placements(root, &temper_dir, true)
    } else {
        place_settings_only(root, true)
    }) else {
        return Vec::new();
    };

    // Tally the missing/drifted placements by kind. The hook and guard are single
    // placements; modelines and managed-by notes are one per modeled artifact, so
    // they collapse to a count.
    let (mut hook, mut guard, mut modelines, mut notes) = (false, false, 0u32, 0u32);
    for entry in &entries {
        if entry.outcome == ApplyOutcome::Unchanged {
            continue;
        }
        match entry.placement {
            SESSION_START => hook = true,
            GUARD_HOOK => guard = true,
            NOTE => notes += 1,
            _ => modelines += 1,
        }
    }
    if !hook && !guard && modelines == 0 && notes == 0 {
        return Vec::new();
    }

    let mut parts = Vec::new();
    if hook {
        parts.push(SESSION_START.to_string());
    }
    if guard {
        parts.push(GUARD_HOOK.to_string());
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

/// Project only the `SessionStart` hook into `.claude/settings.json` — the no-path's
/// whole write (`specs/architecture/20-surface.md`: "No — install wires the
/// session-start reporter... and stops"). No guard, no note, no modeline: those bind
/// only paths a lock declares emit-owned, and an unrepresented project has no lock.
fn place_settings_only(root: &Path, dry_run: bool) -> miette::Result<Vec<InstallEntry>> {
    let settings_path = root.join(".claude").join("settings.json");
    let existing = read_optional(&settings_path)?;
    let settings = project_settings(&settings_path, existing.as_deref(), false)?;
    drift::place(&settings_path, &settings.desired, None, dry_run)?;
    Ok(vec![InstallEntry {
        placement: SESSION_START,
        outcome: placement_outcome(settings.hook_present),
        path: settings_path,
    }])
}

/// Project the `SessionStart` hook, the `PreToolUse` guard (only when emit-owned
/// targets exist — "the guard arrives with its constituency, never before"), and
/// each emit-owned target's managed-by note + schema modeline — the represented
/// project's whole placement set, lock-grounded via [`drift::emit_owned_targets`]
/// rather than a raw discovery walk (`specs/architecture/20-surface.md`, "surface
/// authority is a declared posture").
fn evaluate_placements(
    root: &Path,
    temper_dir: &Path,
    dry_run: bool,
) -> miette::Result<Vec<InstallEntry>> {
    let targets = drift::emit_owned_targets(temper_dir);

    let mut entries = Vec::new();
    let settings_path = root.join(".claude").join("settings.json");
    let existing = read_optional(&settings_path)?;
    let settings = project_settings(&settings_path, existing.as_deref(), !targets.is_empty())?;
    drift::place(&settings_path, &settings.desired, None, dry_run)?;
    entries.push(InstallEntry {
        placement: SESSION_START,
        outcome: placement_outcome(settings.hook_present),
        path: settings_path.clone(),
    });
    if !targets.is_empty() {
        entries.push(InstallEntry {
            placement: GUARD_HOOK,
            outcome: placement_outcome(settings.guard_present),
            path: settings_path,
        });
    }

    // The note is applied first so the modeline stays the leading frontmatter line.
    for target in targets {
        let is_memory = target.kind == "memory";
        let source = fs::read_to_string(&target.path).map_err(|source| InstallError::Read {
            path: target.path.clone(),
            source,
        })?;
        let mut current = source;

        if !is_memory && let Some(desired) = project_note(&current) {
            let outcome = drift::place(&target.path, &desired, None, dry_run)?;
            entries.push(InstallEntry {
                placement: NOTE,
                outcome,
                path: target.path.clone(),
            });
            current = desired;
        }

        // Never point an editor at a `$schema` reference with nothing on the other
        // end: the modeline lands only once its schema artifact actually exists.
        if schema_artifact_exists(root, &target.kind)
            && let Some(desired) =
                project_modeline(&current, &schema_ref(root, &target.path, &target.kind))
        {
            entries.push(InstallEntry {
                placement: MODELINE,
                outcome: drift::place(&target.path, &desired, None, dry_run)?,
                path: target.path,
            });
        }
    }

    Ok(entries)
}

/// Whether `<root>/.temper/schema/<kind>.json` exists — the schema modeline's own
/// target, generated by `temper schema` (`specs/architecture/20-surface.md`, "CLI
/// surface"). A modeline pointing at nothing is worse than no modeline.
fn schema_artifact_exists(root: &Path, kind: &str) -> bool {
    root.join(WORKSPACE_DIR)
        .join("schema")
        .join(format!("{kind}.json"))
        .is_file()
}

/// The verdict `temper guard` reaches over a `PreToolUse` payload at the root
/// member's declared enforcement mode (`specs/model/representation.md`, "The root
/// member"): whether Claude Code's pending write is allowed, informed-and-routed,
/// or blocked. temper never escalates past the mode the harness declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardVerdict {
    /// The write does not target a `.claude/` projection — allow it silently.
    Allow,
    /// A projection edit under the `shared` mode — inform and route to `emit`, exit 0.
    Warn,
    /// A projection edit under the `surface` mode — block the write (exit 2).
    Block,
}

/// Decide `temper guard`'s verdict over a raw `PreToolUse` `payload` at `mode`'s
/// enforcement posture. A write whose `file_path` targets a `.claude/` projection
/// maps onto the mode vocabulary — `shared` informs and routes
/// ([`GuardVerdict::Warn`]), `surface` blocks ([`GuardVerdict::Block`]). Any other
/// write, or a payload naming no `.claude/` `file_path`, is [`GuardVerdict::Allow`]:
/// the guard binds only projection edits.
#[must_use]
pub fn guard(payload: &str, mode: EnforcementMode) -> GuardVerdict {
    if !targets_projection(payload) {
        return GuardVerdict::Allow;
    }
    match mode {
        EnforcementMode::Shared => GuardVerdict::Warn,
        EnforcementMode::Surface => GuardVerdict::Block,
    }
}

/// Whether `payload` names a `.claude/` projection `file_path` — the conservative,
/// field-scoped match the guard binds on ([`GUARD_PATH_MATCH`]).
fn targets_projection(payload: &str) -> bool {
    // A compile-time-constant pattern: the only failure is a malformed literal, a build
    // invariant, so `expect` here can never fire on a real path.
    Regex::new(GUARD_PATH_MATCH)
        .expect("GUARD_PATH_MATCH is a valid regex")
        .is_match(payload)
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
    /// Whether the guard hook was already present (`false`, unchecked, when
    /// `include_guard` is `false` — there is no constituency to place it for).
    guard_present: bool,
}

/// Project the desired `.claude/settings.json` — the existing settings with the
/// `SessionStart` hook merged in, and the `PreToolUse` guard ([`GUARD_COMMAND`])
/// merged in too when `include_guard` is set ("the guard arrives with its
/// constituency, never before" — `specs/architecture/20-surface.md`) — or a fresh
/// document when the file is absent or empty. Idempotent: an already-present temper
/// hook at its desired shape is left alone, so re-merging reproduces the bytes.
///
/// JSON carries no comments and its object order is not semantically meaningful, so
/// the merge parses, adds/updates each hook, and re-emits canonical pretty JSON —
/// there is no format-preserving JSON editor to round-trip through the way
/// `toml_edit` round-trips the TOML surface.
fn project_settings(
    path: &Path,
    existing: Option<&str>,
    include_guard: bool,
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
    let hook_present = session_start_present(object);
    let guard_present = include_guard && guard_present(object, GUARD_COMMAND);

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

    // Ensure the `PreToolUse` guard — replacing any existing temper guard (identified by
    // [`GUARD_MARKER`]) rather than appending a second one. Only when this run has a
    // constituency to place it for.
    if include_guard {
        let pre_tool_use = hooks
            .entry("PreToolUse")
            .or_insert_with(|| json!([]))
            .as_array_mut()
            .ok_or_else(|| InstallError::SettingsShape {
                path: path.to_path_buf(),
            })?;
        upsert_guard(pre_tool_use, GUARD_COMMAND);
    }

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
/// A differing command reads `false`, so the guard reports as (re)applied and
/// [`upsert_guard`] rewrites it.
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
/// guard (identified by [`GUARD_MARKER`]) has its command set to `guard`; absent, a fresh
/// group is appended. So a re-install never duplicates the guard.
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

// ---------------------------------------------------------------------------
// the lift — scaffold the SDK program from the discovery report
// (`specs/architecture/20-surface.md`, "install — the front door; the lift, once")
// ---------------------------------------------------------------------------

/// The scaffold subdirectory a bare kind's member modules live under
/// (`specs/architecture/20-surface.md`, "The port scene": `.temper/skills/reviewer.ts`).
fn member_dir(kind: &str) -> &'static str {
    match kind {
        "skill" => "skills",
        "rule" => "rules",
        "memory" => "memory",
        _ => "members",
    }
}

/// One member the lift is about to scaffold.
struct ScaffoldedMember {
    ident: String,
    import_path: String,
}

/// Scaffold the SDK program from `discovery`'s findings — the lift's whole output:
/// a member module per discovered artifact, `file()` over the original text at its
/// original path (zero rewording, zero file moves — "recognition of the port scene
/// is the acceptance test"), and a `harness.ts` skeleton importing them all. Writes
/// nothing under `dry_run`, returning only the count a real run would scaffold.
///
/// # Errors
/// Returns a [`miette::Report`] if a member's name cannot be derived from its
/// discovered path, the path escapes `root`, or a scaffold file cannot be written.
fn scaffold(
    root: &Path,
    temper_dir: &Path,
    discovery: &DiscoveryReport,
    dry_run: bool,
) -> miette::Result<usize> {
    let kinds = builtin_kind::definitions()?;

    let mut lifted: Vec<(String, String, PathBuf)> = Vec::new();
    for (name, files) in &discovery.members {
        let Some(kind) = kinds.get(name) else {
            continue;
        };
        for file in files {
            lifted.push((name.clone(), member_name(kind, file)?, file.clone()));
        }
    }
    lifted.sort();

    if dry_run {
        return Ok(lifted.len());
    }

    let mut scaffolded = Vec::with_capacity(lifted.len());
    for (kind, name, source) in &lifted {
        let ident = member_ident(kind, name);
        let rel_path = relative_to_workspace(root, source)?;
        let import_path = format!("./{}/{name}.ts", member_dir(kind));
        let module_path = temper_dir.join(member_dir(kind)).join(format!("{name}.ts"));
        write_scaffold_file(
            &module_path,
            &member_module_source(kind, name, &ident, &rel_path),
        )?;
        scaffolded.push(ScaffoldedMember { ident, import_path });
    }

    write_scaffold_file(
        &temper_dir.join(HARNESS_ENTRY),
        &harness_entry_source(&scaffolded),
    )?;

    Ok(lifted.len())
}

/// Derive a discovered artifact's member name: a directory-unit kind's (`skill`)
/// name is its entry file's parent directory; a lone-file kind's (`rule`, `memory`)
/// is the file stem.
fn member_name(kind: &crate::kind::CustomKind, file: &Path) -> miette::Result<String> {
    let component = match kind.unit_shape {
        Some(UnitShape::Directory) => file.parent().and_then(Path::file_name),
        _ => file.file_stem(),
    };
    component
        .and_then(|c| c.to_str())
        .map(str::to_string)
        .ok_or_else(|| miette::miette!("cannot derive a member name from {}", file.display()))
}

/// A member module's TS identifier: kind-prefixed so a skill and a rule sharing a
/// name never collide, non-alphanumeric bytes folded to `_`.
fn member_ident(kind: &str, name: &str) -> String {
    let mut ident = format!("{kind}_");
    ident.extend(
        name.chars()
            .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' }),
    );
    ident
}

/// The `file()` path a scaffolded member module writes, relative to `harness.ts`'s
/// own directory (`.temper/`, always one level under `root` — the SDK resolves a
/// `file()` asset against the running program's cwd, which `emit_program` sets to
/// `harness.ts`'s directory, `specs/architecture/20-surface.md` "The seam").
fn relative_to_workspace(root: &Path, source: &Path) -> miette::Result<String> {
    let relative = source.strip_prefix(root).map_err(|_| {
        miette::miette!(
            "discovered path {} is not under the project root {}",
            source.display(),
            root.display()
        )
    })?;
    Ok(format!("../{}", relative.to_string_lossy()))
}

/// One lifted member's module source — no typed fields at all: the whole original
/// file (frontmatter included) rides through as the `file()` body verbatim, so the
/// projection is byte-identical to the source it came from (own-path,
/// `specs/architecture/20-surface.md`, "A member whose `file()` source is its own
/// projected path is authored territory"). Depth (`description`, `satisfies`, …)
/// accrues later, member by member, under the author's own pen — never scaffolded.
fn member_module_source(kind: &str, name: &str, ident: &str, rel_path: &str) -> String {
    format!(
        "import {{ file, {kind} }} from \"@dtmd/temper/claude-code\";\n\nexport const {ident} = {kind}({{\n  name: {name:?},\n  prose: file({rel_path:?}),\n}});\n"
    )
}

/// The `harness.ts` skeleton: import every scaffolded member, compose them into
/// `harness({ members: [...] })`, and print `emit`'s seam to stdout — the whole
/// program `emit_program` (`specs/architecture/20-surface.md`, "The seam") then runs.
fn harness_entry_source(members: &[ScaffoldedMember]) -> String {
    let mut out = String::from("import { emit, harness } from \"@dtmd/temper\";\n");
    for member in members {
        out.push_str(&format!(
            "import {{ {} }} from {:?};\n",
            member.ident, member.import_path
        ));
    }
    out.push_str("\nconst program = harness({\n  members: [");
    for member in members {
        out.push_str(&member.ident);
        out.push_str(", ");
    }
    out.push_str("],\n});\n\nprocess.stdout.write(emit(program).seam);\n");
    out
}

/// Write a scaffold file, creating any missing parent directories.
fn write_scaffold_file(path: &Path, contents: &str) -> Result<(), InstallError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| InstallError::Write {
            path: parent.to_path_buf(),
            source,
        })?;
    }
    fs::write(path, contents).map_err(|source| InstallError::Write {
        path: path.to_path_buf(),
        source,
    })
}

// ---------------------------------------------------------------------------
// Node + the `@dtmd/temper` dependency — the yes-path's preflight
// (`specs/architecture/20-surface.md`, "the represented path requires Node...")
// ---------------------------------------------------------------------------

/// Refuse loud, with instructions, when `node` is not on `PATH` — checked up front
/// so the yes-path never leaves a half-scaffolded state behind a missing toolchain.
fn ensure_node_available() -> Result<(), InstallError> {
    let available = Command::new("node")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success());
    if available {
        Ok(())
    } else {
        Err(InstallError::NodeMissing)
    }
}

/// Whether `@dtmd/temper` already resolves from `temper_dir` — walking up through
/// `node_modules` the way Node's own resolution would, so a project nested under an
/// already-`npm install`ed ancestor (or a test's pre-vendored fixture) is recognized
/// without a redundant `npm install`.
fn dependency_resolves(temper_dir: &Path) -> bool {
    let mut dir = Some(temper_dir);
    while let Some(candidate) = dir {
        if candidate
            .join("node_modules")
            .join("@dtmd")
            .join("temper")
            .exists()
        {
            return true;
        }
        dir = candidate.parent();
    }
    false
}

/// Ensure the `@dtmd/temper` dependency: declare it in `.temper/package.json`
/// (creating a minimal manifest when absent) and `npm install` it. Idempotent by
/// construction — [`run_represented`] only calls this when [`dependency_resolves`]
/// already reads `false`.
fn ensure_dependency(temper_dir: &Path) -> Result<(), InstallError> {
    ensure_package_json(temper_dir)?;
    let output = Command::new("npm")
        .arg("install")
        .current_dir(temper_dir)
        .output()
        .map_err(|source| InstallError::DependencySpawn {
            path: temper_dir.to_path_buf(),
            source,
        })?;
    if !output.status.success() {
        return Err(InstallError::DependencyInstall {
            path: temper_dir.to_path_buf(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }
    Ok(())
}

/// Ensure `<temper_dir>/package.json` declares [`SDK_PACKAGE`] as a dependency —
/// creating a minimal manifest when absent, adding the dependency key when the file
/// exists but lacks it, and leaving an already-declared dependency (any version
/// spec — a test's `file:` pin included) untouched.
fn ensure_package_json(temper_dir: &Path) -> Result<(), InstallError> {
    let path = temper_dir.join("package.json");
    let mut root = match fs::read_to_string(&path) {
        Ok(text) => serde_json::from_str::<JsonValue>(&text).map_err(|source| {
            InstallError::PackageJson {
                path: path.clone(),
                source,
            }
        })?,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => json!({
            "name": "temper-harness",
            "private": true,
            "type": "module",
        }),
        Err(source) => {
            return Err(InstallError::Read {
                path: path.clone(),
                source,
            });
        }
    };

    let object = root
        .as_object_mut()
        .ok_or_else(|| InstallError::PackageJsonShape { path: path.clone() })?;
    let dependencies = object
        .entry("dependencies")
        .or_insert_with(|| json!({}))
        .as_object_mut()
        .ok_or_else(|| InstallError::PackageJsonShape { path: path.clone() })?;
    dependencies
        .entry(SDK_PACKAGE)
        .or_insert_with(|| json!(SDK_VERSION_RANGE));

    let desired = format!(
        "{}\n",
        serde_json::to_string_pretty(&root).map_err(|source| InstallError::PackageJson {
            path: path.clone(),
            source,
        })?
    );
    write_scaffold_file(&path, &desired)
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
        .any(|line| line.trim_start().starts_with(MODELINE_MARKER))
    {
        return Some(source.to_string());
    }
    let modeline = format!("{MODELINE_MARKER} $schema={schema_ref}");
    Some(format!("---\n{modeline}\n{rest}"))
}

/// Project an artifact source with the managed-by note inserted as a frontmatter
/// comment, or `None` when it has no frontmatter to carry it (a memory `CLAUDE.md`
/// has none, and the caller already skips memory besides). Applied *before* the
/// modeline so the modeline stays the leading line.
///
/// **Content-drift-aware** (`specs/architecture/50-distribution.md`, "drift keeps it
/// synced"): idempotence keys on the note's *bytes*, not the bare [`NOTE_MARKER`]
/// prefix. A marked line whose body still matches [`NOTE_COMMENT`] is returned
/// verbatim (no churn); a marked line carrying a retired wording — the reword that
/// [`NOTE_COMMENT`] shipped — is *re-placed*, splicing the current [`NOTE_COMMENT`]
/// over the stale line so a changed placement re-places instead of reporting
/// `Unchanged`. Presence-only keying let a stale note pass `gate_installed` forever.
///
/// Byte-faithful (`.claude/rules/rust.md`, round-trip discipline): the note line is
/// the only rewritten bytes. The note rides `install`, never `emit` — a YAML comment
/// is not authored surface content, so the content-faithful projector (law 5) never
/// re-emits it (`specs/architecture/20-surface.md`).
fn project_note(source: &str) -> Option<String> {
    let rest = source.strip_prefix("---\n")?;
    let inner = frontmatter_inner(rest)?;
    if let Some(existing) = inner
        .lines()
        .find(|line| line.trim_start().starts_with(NOTE_MARKER))
    {
        if existing == NOTE_COMMENT {
            return Some(source.to_string());
        }
        // Stale wording: splice the current note over the marked line, leaving every
        // other byte — the modeline, the other fields, the body — untouched. The
        // marker is distinctive, so the first occurrence is this note line.
        return Some(source.replacen(existing, NOTE_COMMENT, 1));
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

/// The install-placed frontmatter comment lines present in `source`, in on-disk order —
/// the schema modeline and the managed-by note. `emit` round-trips these through its
/// whole-file re-emit so its content-faithful projection (law 5) carries install's
/// metadata instead of dropping it (`specs/architecture/20-surface.md`, the two-projectors
/// seam): install owns *placing and auditing* them, emit only *preserves* what is already
/// there. Empty when `source` has no frontmatter or carries neither line.
pub(crate) fn placement_lines(source: &str) -> Vec<String> {
    let Some(rest) = source.strip_prefix("---\n") else {
        return Vec::new();
    };
    let Some(inner) = frontmatter_inner(rest) else {
        return Vec::new();
    };
    inner
        .lines()
        .filter(|line| is_placement_comment(line))
        .map(str::to_string)
        .collect()
}

/// Whether `line` is one of install's managed metadata comments — the schema modeline
/// or the managed-by note. The single predicate install's idempotence and emit's
/// preservation share, so the two projectors never disagree on which lines are install's.
fn is_placement_comment(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with(MODELINE_MARKER) || trimmed.starts_with(NOTE_MARKER)
}

/// Render an install outcome for the terminal: the represent decision and any
/// scaffold/emit summary, then one `<outcome>  <placement>  <path>` line per
/// placement entry, then a one-line tally — mirroring [`drift::render_emit`].
#[must_use]
pub fn render(outcome: &InstallOutcome) -> String {
    let mut out = String::new();
    if outcome.represented {
        out.push_str(&format!(
            "represented — {} member(s) scaffolded\n",
            outcome.scaffolded
        ));
        if let Some(emit) = &outcome.emit {
            out.push_str(&drift::render_emit(emit));
        }
    } else {
        out.push_str("not represented — session-start reporter only\n");
    }

    let (mut applied, mut unchanged, mut conflicted) = (0u32, 0u32, 0u32);
    for entry in &outcome.entries {
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
