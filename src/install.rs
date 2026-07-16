//! `temper install` — the one on-ramp.
//!
//! `install` opens with a discovery report ([`discover`]/[`render_discovery`]) —
//! findings first, ceremony after — then asks exactly one question via [`Represent`]:
//! represent this project as a temper program?
//!
//! - **No** wires the `SessionStart` reporter alone ([`place_settings_only`]) and
//!   stops — the stranger gate at session start, Node-free forever.
//! - **Yes** requires Node (checked up front, refused loud with instructions when
//!   absent), ensures the `@dtmd/temper` dependency ([`ensure_dependency`]) —
//!   before a single file of the lift is written, so a spawn failure never
//!   leaves a half-scaffolded `.temper/` behind it — then scaffolds the SDK
//!   program once if none exists yet — the lift ([`scaffold`]): a whole
//!   conversion (0016), never an intermediate state — every present frontmatter
//!   field hoists into a typed property and prose moves module-side (inline for
//!   a short body, a module-adjacent file for a document) — plus a `harness.ts`
//!   skeleton — runs the first `emit` (the adoption moment,
//!   [`drift::emit_program`]), which regenerates every composed kind's artifact as a
//!   canonical projection — a layout kind's document stays a source at either depth,
//!   never regenerated — and places the guard hook / managed-by note /
//!   schema modeline at every path the fresh lock declares **emit-owned**
//!   ([`drift::emit_owned_targets`], [`evaluate_placements`]) — the first emit's
//!   diff is the one reviewable adoption diff, never an own-path passthrough.
//!
//! [`gate_installed`] is the read-only shadow `check` folds in: the same placement
//! evaluation, dry-run, collapsed to one advisory [`Diagnostic`]. It never scaffolds,
//! installs a dependency, or emits — `install` alone adopts.
//!
//! **Fail-loud**: a placement, a scaffold write, or a dependency/emit step that
//! cannot complete is a hard [`InstallError`] / propagated [`miette::Report`], never
//! a silent skip.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::LazyLock;

use regex::Regex;
use serde_json::{Value as JsonValue, json};

use crate::builtin_kind;
use crate::check::{Diagnostic, Severity};
use crate::compose::EnforcementMode;
use crate::contract::Contract;
use crate::drift::{self, ApplyOutcome, EmitReport};
use crate::engine;
use crate::frontmatter;
use crate::import;
use crate::json_manifest;
use crate::json_splice::{self, Edit};
use crate::kind::{self, CollectionAddress, CustomKind};

/// The SDK program's entry file — scaffolded once by the lift, run by every
/// subsequent `emit`.
const HARNESS_ENTRY: &str = "harness.ts";

/// The npm package name the yes-path ensures as a dependency.
const SDK_PACKAGE: &str = "@dtmd/temper";

/// `sdk/package.json`'s own text, embedded at compile time so the dependency
/// range `install` writes can never independently drift from the workspace
/// SDK's real released version.
const SDK_PACKAGE_JSON: &str = include_str!("../sdk/package.json");

/// The dependency range `install` writes when `.temper/package.json` does not
/// already declare [`SDK_PACKAGE`] — derived from [`SDK_PACKAGE_JSON`]'s own
/// `version` field, parsed once.
fn sdk_version_range() -> &'static str {
    static RANGE: LazyLock<String> = LazyLock::new(|| {
        let manifest: JsonValue = serde_json::from_str(SDK_PACKAGE_JSON)
            .expect("sdk/package.json is a committed, well-formed manifest");
        let version = manifest["version"]
            .as_str()
            .expect("sdk/package.json declares a string `version` field");
        format!("^{version}")
    });
    &RANGE
}

/// The exec-form command Claude Code runs at session start: the `temper` binary
/// checking the project root under the advisory session-start reporter. The `.` is
/// the **harness root** the hook runs in, so the reporter resolves `./.temper`'s
/// committed lock and gates the full declared model — never `.temper` itself, the
/// surface spelling that would walk members off the workspace dir and read every
/// requirement unfilled. Public so the session-start acceptance can drive the exact
/// wired command.
pub const SESSION_START_COMMAND: &str = "temper check . --reporter session-start";

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
/// is that other consumers of a shared
/// file are not instrumented by it.
const GUARD_MATCHER: &str = "Write|Edit|MultiEdit";

/// The exec-form command the `PreToolUse` guard hook runs: the `temper` binary's own
/// `guard` subcommand, reading the payload from stdin and deciding at the harness's
/// declared enforcement mode. The `.` roots the
/// lock the enforcement mode is read from — the project Claude Code runs the hook in.
const GUARD_COMMAND: &str = "temper guard .";

/// The stable token the guard command carries so a re-install *replaces* the existing
/// temper guard in place rather than appending a second one. The command is
/// mode-independent (the subcommand reads the enforcement mode live), so this is simply the
/// subcommand invocation.
const GUARD_MARKER: &str = "temper guard";

/// The message `temper guard` prints on a projection hit — stating the limit verbatim:
/// the guard binds only this provider's writes, so other tools writing a shared file are
/// not bound by it. Public so the `guard` subcommand ([`main`]) prints it whether it
/// warns or blocks, under the `warn` or `block` enforcement mode.
pub const GUARD_MESSAGE: &str = "temper-managed projection: .claude/ is projected from the .temper/ surface — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit. This guard binds only Claude Code writes; other tools writes are not bound by it.";

/// The header `temper guard` prints when a pending write to a represented manifest carries a
/// member that violates its contract — the per-member contract findings ([`GuardedManifest`])
/// follow it, one per line. Unlike a `.claude/` projection ([`GUARD_MESSAGE`]), a manifest is
/// co-owned: a write touching only opaque residue conforms and passes, so the finding names the
/// contract broken, not the file edited. States the same binding limit verbatim.
pub const GUARD_MANIFEST_MESSAGE: &str = "temper-governed manifest: a member of this write violates its contract — fix the member to conform, or challenge the contract. This guard binds only Claude Code writes; other tools writes are not bound by it.";

/// The extended-regex `temper guard` greps the `PreToolUse` payload for: a `file_path`
/// value under a `.claude/` locus, captured so the guard can test it for lock-declared
/// projection-set membership. Matching the field (not the whole payload) keeps a write
/// whose *content* merely mentions `.claude/` from tripping the guard. Kept deliberately
/// conservative — a false negative routes to CI (the backstop wall), a false positive
/// would block honest work.
const GUARD_PATH_MATCH: &str = r#""file_path"[[:space:]]*:[[:space:]]*"([^"]*\.claude/[^"]*)""#;

/// The managed-by note's stable marker — the comment prefix that *locates* an already
/// placed note (so a second `install` never duplicates it); whether that note is then
/// left verbatim or re-placed keys on the line's bytes vs [`NOTE_COMMENT`], not this
/// prefix (`project_note`, content-drift-aware).
const NOTE_MARKER: &str = "# temper: managed projection";

/// The banner form's stable marker — the block-level HTML comment prefix that *locates*
/// an already placed banner on a frontmatterless projection, the [`NOTE_MARKER`]
/// counterpart for a body that carries no frontmatter to hold the `#` note
/// (`project_banner`, content-drift-aware).
const BANNER_MARKER: &str = "<!-- temper: managed projection";

/// The schema modeline's stable marker — the frontmatter comment prefix `install` keys
/// its idempotence on and `emit` keys its preservation on, so both projectors agree on
/// which line is the modeline.
const MODELINE_MARKER: &str = "# yaml-language-server:";

/// The managed-by note itself: a frontmatter comment stating the file is generated and
/// pointing at the surface. Cost-free metadata YAML frontmatter tolerates — never
/// stamped by `emit`.
const NOTE_COMMENT: &str = "# temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file.";

/// The managed-by note's block-level HTML-comment form, for a frontmatterless
/// markdown projection (a memory `CLAUDE.md`, any frontmatterless kind) with no
/// frontmatter to carry the `#` [`NOTE_COMMENT`]. Claude Code strips a block-level
/// HTML comment before injection, so the banner is human-visible and model-invisible —
/// a courtesy marker, the drift hash still catching a hand-edit either way. States the
/// same message as [`NOTE_COMMENT`], verbatim.
const NOTE_BANNER: &str = "<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->";

/// The one question `install` asks, exactly once, after the discovery report:
/// there is one
/// genuine fork in the world — a harness is represented or it is not. Asked only where
/// that fork is still live: a root whose workspace already carries a lock has answered
/// it on disk, and install converges on that lock rather than re-asking a settled
/// question.
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

    /// Node.js was not found on `PATH` — the represented (yes) path requires it.
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

    /// `npm install` could not be spawned in the `.temper/` workspace.
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

/// The one question's answer: there is one
/// genuine fork in the world, so exactly one boolean fork here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Represent {
    /// Represent this project as a temper program — the lift + first emit path.
    Yes,
    /// Do not — wire the session-start reporter alone.
    No,
}

/// The discovery walk's findings — "what the walk found (members by kind...)",
/// reported before
/// the one question and reused by the yes-path's scaffold so the lift lifts exactly
/// what was reported, never a re-walked, possibly-differing set.
#[derive(Debug, Clone, Default)]
pub struct DiscoveryReport {
    /// Discovered member source files, keyed by the kind's bare row label
    /// — every embedded built-in kind.
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
/// The whole kind set travels with each kind: a nested file kind is discovered under its
/// host's units at the host template's pattern, so its own row of the report is a
/// function of another kind's declaration.
///
/// # Errors
/// Returns a [`miette::Report`] if the embedded kind set fails to load or a kind's
/// discovery walk fails to read a directory.
pub fn discover(root: &Path) -> miette::Result<DiscoveryReport> {
    let mut members = BTreeMap::new();
    let kinds = builtin_kind::definitions()?;
    for kind in kinds.values() {
        let files = import::discover_builtin(root, kind, &kinds)?;
        members.insert(kind.name.clone(), files);
    }
    Ok(DiscoveryReport { members })
}

/// Render the discovery report for the terminal — findings first, ceremony after:
/// `lock`, when the caller's path resolution found one, naming the root that already
/// answered the represent question on disk — the question below the report is skipped,
/// so the answer is stated rather than left invisible — then member counts by kind, or
/// a plain statement that nothing was found.
#[must_use]
pub fn render_discovery(report: &DiscoveryReport, lock: Option<&Path>) -> String {
    let mut out = String::from("discovery:\n");
    if let Some(lock) = lock {
        out.push_str(&format!(
            "  already represented — {} answers the represent question\n",
            lock.display()
        ));
    }
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
/// emit-owned targets.
fn run_represented(
    root: &Path,
    discovery: &DiscoveryReport,
    dry_run: bool,
) -> miette::Result<InstallOutcome> {
    ensure_node_available()?;

    let temper_dir = root.join(crate::WORKSPACE_DIR);
    let harness_entry = temper_dir.join(HARNESS_ENTRY);
    let already_scaffolded = harness_entry.is_file();

    // Assured before the lift writes a single member module: "no half-scaffolded
    // state" — a dependency spawn failure must never leave a partial `.temper/`
    // program behind it.
    if !dry_run && !dependency_resolves(&temper_dir) {
        ensure_dependency(&temper_dir)?;
    }

    let scaffolded = if already_scaffolded {
        0
    } else {
        scaffold(&temper_dir, discovery, dry_run)?
    };

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
                teardown: false,
            },
        )?)
    };

    let entries = if emit.is_some() {
        evaluate_placements(root, &temper_dir, dry_run)?
    } else {
        Vec::new()
    };

    // `evaluate_placements` just mutated emit-owned targets' bytes (the managed-by
    // note, the schema modeline) *after* the emit above already stamped the lock's
    // fingerprints from the pre-placement bytes — so without this, the lock is
    // stale against its own first-install output until a second run's `emit_one`
    // folds the placement lines back in. Re-running `emit_program` here (real
    // writes only — a `--dry-run` preview placed nothing to re-stamp) reads those
    // placement-inclusive bytes straight back and rewrites the lock to match, so
    // one yes-path run settles.
    if !dry_run && emit.is_some() {
        drift::emit_program(
            &temper_dir,
            drift::EmitOptions {
                dry_run: false,
                frozen: false,
                teardown: false,
            },
        )?;
    }

    Ok(InstallOutcome {
        represented: true,
        scaffolded,
        emit,
        entries,
    })
}

/// Report whether temper's own gate is installed and undrifted at `root` — the
/// `check` self-verify.
///
/// Never scaffolds, installs a dependency, or emits — only [`run`] adopts.
/// Evaluates the placements a *represented* project's current lock justifies, or —
/// on an unrepresented project (no `.temper/harness.ts`) — the session-start hook
/// alone, both dry-run, folded into **one advisory** [`Diagnostic`] carrying the
/// missing/drifted counts. Always `warn`, never `error`; empty when every placement
/// is already in place.
#[must_use]
pub fn gate_installed(root: &Path) -> Vec<Diagnostic> {
    let temper_dir = root.join(crate::WORKSPACE_DIR);
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
        parts.push(format!(
            "{modelines} schema modeline{}",
            crate::display::plural(modelines as usize)
        ));
    }
    if notes > 0 {
        parts.push(format!(
            "{notes} managed-by note{}",
            crate::display::plural(notes as usize)
        ));
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

/// Resolves the settings document every `install` placement writes under a project root.
fn settings_path(root: &Path) -> PathBuf {
    root.join(".claude").join("settings.json")
}

/// Project only the `SessionStart` hook into `.claude/settings.json` — the no-path's
/// whole write. No guard, no note, no modeline: those bind
/// only paths a lock declares emit-owned, and an unrepresented project has no lock.
fn place_settings_only(root: &Path, dry_run: bool) -> miette::Result<Vec<InstallEntry>> {
    let settings_path = settings_path(root);
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
/// rather than a raw discovery walk.
fn evaluate_placements(
    root: &Path,
    temper_dir: &Path,
    dry_run: bool,
) -> miette::Result<Vec<InstallEntry>> {
    let targets = drift::emit_owned_targets(temper_dir);

    let mut entries = Vec::new();
    let settings_path = settings_path(root);
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
        let source = fs::read_to_string(&target.path).map_err(|source| InstallError::Read {
            path: target.path.clone(),
            source,
        })?;
        let mut current = source;

        // The managed-by note in whichever form the projection can carry: a frontmatter
        // `#` comment when there is frontmatter, else a block-level HTML-comment banner
        // for a frontmatterless markdown body (a memory `CLAUDE.md`, any frontmatterless
        // kind). Content drives the choice — `project_note` declines a frontmatterless
        // source, and the banner is confined to markdown, where an HTML comment is inert.
        let noted = project_note(&current).or_else(|| {
            is_markdown_path(&target.path)
                .then(|| project_banner(&current))
                .flatten()
        });
        if let Some(desired) = noted {
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
/// target, generated by `temper schema`. A
/// modeline pointing at nothing is worse than no modeline.
fn schema_artifact_exists(root: &Path, kind: &str) -> bool {
    root.join(crate::WORKSPACE_DIR)
        .join("schema")
        .join(format!("{kind}.json"))
        .is_file()
}

/// The verdict `temper guard` reaches over a `PreToolUse` payload at the root
/// member's declared enforcement mode: whether Claude Code's pending write is allowed (silently, in-band, or
/// deferred out-of-band) or blocked. temper never escalates past the mode the
/// harness declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardVerdict {
    /// The write does not target a `.claude/` projection — allow it silently.
    Allow,
    /// A projection edit under the `note` mode — allow the call, exit 0, with no
    /// in-band context injection: the finding is out-of-band only, riding the next
    /// `check`/report, never the live session.
    Note,
    /// A projection edit under the `warn` mode — allow it and surface the finding
    /// in-band, exit 0.
    Warn,
    /// A projection edit under the `block` mode — deny the write (exit 2).
    Block,
}

/// Decide `temper guard`'s verdict over a raw `PreToolUse` `payload` at `mode`'s
/// enforcement mode, bound to `targets` — the lock's emit-owned projection set
/// ([`drift::emit_owned_targets`]). `targets` is `None` for a harness with no
/// `lock.toml` at all (never emitted, or the file removed out from under an
/// already-installed hook): with no declared set to consult, the guard falls
/// back to binding any `.claude/` `file_path`, matching the pre-lock behavior —
/// absent evidence must never silently suppress the guard.
///
/// A `file_path` naming no `.claude/` locus, or (with `targets` present) naming no
/// declared projection, is [`GuardVerdict::Allow`]. Otherwise the finding maps
/// onto the mode vocabulary, split by where it goes: `note` defers it out-of-band
/// ([`GuardVerdict::Note`]), `warn` surfaces it in-band ([`GuardVerdict::Warn`]),
/// `block` denies the call ([`GuardVerdict::Block`]).
#[must_use]
pub fn guard(
    payload: &str,
    mode: EnforcementMode,
    targets: Option<&[drift::EmitOwnedEntry]>,
) -> GuardVerdict {
    let Some(file_path) = claude_file_path(payload) else {
        return GuardVerdict::Allow;
    };
    if let Some(targets) = targets
        && !matches_projection(&file_path, targets)
    {
        return GuardVerdict::Allow;
    }
    match mode {
        EnforcementMode::Note => GuardVerdict::Note,
        EnforcementMode::Warn => GuardVerdict::Warn,
        EnforcementMode::Block => GuardVerdict::Block,
    }
}

/// The `.claude/`-rooted `file_path` a `PreToolUse` `payload` names, when present
/// ([`GUARD_PATH_MATCH`]'s captured value).
fn claude_file_path(payload: &str) -> Option<String> {
    // A compile-time-constant pattern: the only failure is a malformed literal, a build
    // invariant, so `expect` here can never fire on a real path.
    Regex::new(GUARD_PATH_MATCH)
        .expect("GUARD_PATH_MATCH is a valid regex")
        .captures(payload)
        .and_then(|captures| captures.get(1))
        .map(|value| value.as_str().to_string())
}

/// Whether `file_path` names one of `targets` — a straight suffix compare against each
/// row's `/`-normalized `source_path` (`PATH-SEP-NORMALIZE`), tolerant of `file_path`
/// arriving absolute (Claude Code's own convention) against a workspace-relative lock row.
fn matches_projection(file_path: &str, targets: &[drift::EmitOwnedEntry]) -> bool {
    let file_path = file_path.replace('\\', "/");
    targets.iter().any(|target| {
        let source = target.path.to_string_lossy().replace('\\', "/");
        file_path.ends_with(source.as_str())
    })
}

/// One represented manifest the `PreToolUse` guard checks a pending write against — its
/// harness-relative path, the manifest kind whose members surface inside it, that kind's effective
/// contract, and the collection address the members key at. Assembled by the caller off the
/// lock's kinds/clauses exactly as the gate resolves them, so the guard and the gate cannot
/// disagree about a member's contract.
pub struct GuardedManifest {
    /// The manifest's harness-relative path — the kind's `governs` locus, the suffix a
    /// pending write's `file_path` is matched against.
    pub path: PathBuf,
    /// The manifest kind whose registration members surface inside `path`.
    pub kind: CustomKind,
    /// The kind's effective contract — its lock-declared clauses, else the embedded default.
    pub contract: Contract,
    /// The collection address the members key at (`mcpServers.*`, `hooks.<Event>`).
    pub address: CollectionAddress,
}

/// Check a pending `PreToolUse` write against every represented manifest's contract —
/// entry 4 of the manifest write side, extending the `.claude/`-projection binding
/// ([`guard`]) to the manifest members the write face now governs.
///
/// Returns `None` when the write targets no represented manifest (a non-manifest path, a
/// payload with no whole-file `content`, or content that will not parse as this manifest) —
/// the caller falls back to the projection-drift binding. Returns `Some(findings)` when the
/// write does target one: `findings` is empty for a conforming manifest (a co-owned manifest
/// write touching only opaque residue, or members that all pass), or the error-severity
/// contract violations its members trip, to be surfaced at the author's declared enforcement
/// mode. Only a whole-file `content` (a `Write`) is validated — a partial `Edit`/`MultiEdit`
/// payload carries no full manifest to check, so it reads as no-manifest and CI backstops it.
#[must_use]
pub fn manifest_write_findings(
    payload: &str,
    manifests: &[GuardedManifest],
) -> Option<Vec<Diagnostic>> {
    let value: JsonValue = serde_json::from_str(payload).ok()?;
    let input = value.get("tool_input")?;
    let file_path = input.get("file_path").and_then(JsonValue::as_str)?;
    let content = input.get("content").and_then(JsonValue::as_str)?;
    let file_path = file_path.replace('\\', "/");

    let mut matched = false;
    let mut findings = Vec::new();
    for manifest in manifests {
        let target = manifest.path.to_string_lossy().replace('\\', "/");
        if !file_path.ends_with(target.as_str()) {
            continue;
        }
        matched = true;
        // A pending write that will not even parse as a manifest is left to CI (the write
        // would trip `check`'s own loud malformed read); the guard is conservative and only
        // ever fails to forge a finding, never suppresses honest work over a parse hiccup.
        let Ok(parsed) =
            json_manifest::Manifest::parse(&manifest.path, content, &[&manifest.address])
        else {
            continue;
        };
        let features: Vec<_> = parsed
            .members
            .iter()
            .map(|member| {
                builtin_kind::features(
                    &manifest.kind,
                    &member.to_unit(&manifest.address, &parsed.provenance.source_path),
                    &[],
                )
            })
            .collect();
        findings.extend(
            engine::validate(&manifest.contract, &features)
                .into_iter()
                .filter(|finding| finding.severity == Severity::Error),
        );
    }
    matched.then_some(findings)
}

/// Render a represented manifest's contract violations for the guard's in-band surface: the
/// [`GUARD_MANIFEST_MESSAGE`] header, then one `<rule>: <finding>` line per violation.
#[must_use]
pub fn render_manifest_findings(findings: &[Diagnostic]) -> String {
    let mut out = String::from(GUARD_MANIFEST_MESSAGE);
    for finding in findings {
        out.push_str(&format!("\n  {}: {}", finding.rule, finding.message));
    }
    out
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
/// constituency, never before") — or a fresh document when the file is absent or
/// empty. Idempotent: an already-present temper hook at its desired shape is left
/// alone, so re-merging reproduces the bytes.
///
/// Format-preserving: an existing document is never re-serialized. Only the two
/// hook groups' own bytes change — every other key, its order, and the file's
/// formatting survive (decision 0008, the JSON peer of the `toml_edit` keystone).
fn project_settings(
    path: &Path,
    existing: Option<&str>,
    include_guard: bool,
) -> Result<SettingsProjection, InstallError> {
    match existing {
        Some(text) if !text.trim().is_empty() => merge_settings(path, text, include_guard),
        _ => fresh_settings(path, include_guard),
    }
}

/// A fresh canonical `.claude/settings.json` — there is no existing document to
/// preserve, so a plain pretty re-serialize is exactly the right shape.
fn fresh_settings(path: &Path, include_guard: bool) -> Result<SettingsProjection, InstallError> {
    let mut hooks = serde_json::Map::new();
    hooks.insert("SessionStart".to_string(), json!([session_start_group()]));
    if include_guard {
        hooks.insert(
            "PreToolUse".to_string(),
            json!([guard_group(GUARD_COMMAND)]),
        );
    }
    let root = json!({ "hooks": hooks });
    let desired = format!(
        "{}\n",
        serde_json::to_string_pretty(&root).map_err(|source| InstallError::Settings {
            path: path.to_path_buf(),
            source,
        })?
    );
    Ok(SettingsProjection {
        desired,
        hook_present: false,
        guard_present: false,
    })
}

/// Splice the temper hook groups into an existing, non-empty `.claude/settings.json`
/// document without re-serializing it. Already-present, already-correct hooks are
/// left untouched, so a no-op merge returns `text` byte-identical.
fn merge_settings(
    path: &Path,
    text: &str,
    include_guard: bool,
) -> Result<SettingsProjection, InstallError> {
    let root: JsonValue = serde_json::from_str(text).map_err(|source| InstallError::Settings {
        path: path.to_path_buf(),
        source,
    })?;
    let object = root
        .as_object()
        .ok_or_else(|| InstallError::SettingsShape {
            path: path.to_path_buf(),
        })?;

    let hook_present = session_start_present(object);
    let guard_present = include_guard && guard_present(object, GUARD_COMMAND);
    let guard_marker_present = include_guard && guard_marker_present(object);

    if hook_present && guard_present == include_guard {
        return Ok(SettingsProjection {
            desired: text.to_string(),
            hook_present,
            guard_present,
        });
    }

    let root_start =
        text.find(|c: char| !c.is_whitespace())
            .ok_or_else(|| InstallError::SettingsShape {
                path: path.to_path_buf(),
            })?;
    let root_shape = json_splice::object_shape(text, root_start);

    let mut edits = Vec::new();
    match root_shape.members.iter().find(|m| m.key == "hooks") {
        Some(hooks_member) => {
            let hooks_shape = json_splice::object_shape(text, hooks_member.value_span.0);
            splice_hooks(
                text,
                &hooks_shape,
                hook_present,
                include_guard,
                guard_present,
                guard_marker_present,
                &mut edits,
            );
        }
        None => {
            let mut hooks = serde_json::Map::new();
            hooks.insert("SessionStart".to_string(), json!([session_start_group()]));
            if include_guard {
                hooks.insert(
                    "PreToolUse".to_string(),
                    json!([guard_group(GUARD_COMMAND)]),
                );
            }
            edits.push(json_splice::insert_member(
                &root_shape,
                "hooks",
                &json!(hooks),
                1,
            ));
        }
    }

    let desired = json_splice::apply_edits(text, edits);
    Ok(SettingsProjection {
        desired,
        hook_present,
        guard_present,
    })
}

/// Add the edits needed to bring an existing `hooks` object up to date: append the
/// `SessionStart` group when absent (never modifying an existing one — a second
/// `install` only ever adds its own group, never touches a human's), and either
/// insert, append, or in-place update the `PreToolUse` guard group depending on
/// what's already there ("the guard arrives with its constituency, never before").
fn splice_hooks(
    text: &str,
    hooks_shape: &json_splice::ObjectShape,
    hook_present: bool,
    include_guard: bool,
    guard_present: bool,
    guard_marker_present: bool,
    edits: &mut Vec<Edit>,
) {
    if !hook_present {
        match hooks_shape.members.iter().find(|m| m.key == "SessionStart") {
            Some(member) => {
                let array = json_splice::array_shape(text, member.value_span.0);
                edits.push(json_splice::append_element(
                    &array,
                    &session_start_group(),
                    3,
                ));
            }
            None => {
                edits.push(json_splice::insert_member(
                    hooks_shape,
                    "SessionStart",
                    &json!([session_start_group()]),
                    2,
                ));
            }
        }
    }

    if include_guard && !guard_present {
        match hooks_shape.members.iter().find(|m| m.key == "PreToolUse") {
            Some(member) => {
                let array = json_splice::array_shape(text, member.value_span.0);
                if guard_marker_present {
                    edits.extend(splice_guard_command(text, &array, GUARD_COMMAND));
                } else {
                    edits.push(json_splice::append_element(
                        &array,
                        &guard_group(GUARD_COMMAND),
                        3,
                    ));
                }
            }
            None => {
                edits.push(json_splice::insert_member(
                    hooks_shape,
                    "PreToolUse",
                    &json!([guard_group(GUARD_COMMAND)]),
                    2,
                ));
            }
        }
    }
}

/// The edits that rewrite just the `command` string of every hook entry, in every
/// `PreToolUse` group of `array`, whose command already carries [`GUARD_MARKER`] —
/// the surgical form of replacing a stale temper guard with `new_command` in place,
/// touching nothing else in the group (its `matcher`, sibling groups, or the rest
/// of the document).
fn splice_guard_command(
    text: &str,
    array: &json_splice::ArrayShape,
    new_command: &str,
) -> Vec<Edit> {
    let mut edits = Vec::new();
    for &group_span in &array.elements {
        let Ok(group_value) = serde_json::from_str::<JsonValue>(&text[group_span.0..group_span.1])
        else {
            continue;
        };
        if !group_has_command(&group_value, |command| command.contains(GUARD_MARKER)) {
            continue;
        }
        let group_shape = json_splice::object_shape(text, group_span.0);
        let Some(hooks_member) = group_shape.members.iter().find(|m| m.key == "hooks") else {
            continue;
        };
        let inner = json_splice::array_shape(text, hooks_member.value_span.0);
        for &hook_span in &inner.elements {
            let Ok(hook_value) = serde_json::from_str::<JsonValue>(&text[hook_span.0..hook_span.1])
            else {
                continue;
            };
            let is_guard = hook_value
                .get("command")
                .and_then(JsonValue::as_str)
                .is_some_and(|command| command.contains(GUARD_MARKER));
            if !is_guard {
                continue;
            }
            let hook_shape = json_splice::object_shape(text, hook_span.0);
            if let Some(command_member) = hook_shape.members.iter().find(|m| m.key == "command") {
                edits.push(Edit {
                    span: command_member.value_span,
                    replacement: serde_json::to_string(new_command)
                        .expect("a plain command string serializes infallibly"),
                });
            }
        }
    }
    edits
}

/// The `SessionStart` hook group temper installs: the exec-form command alone.
fn session_start_group() -> JsonValue {
    json!({ "hooks": [ { "type": "command", "command": SESSION_START_COMMAND } ] })
}

/// The `PreToolUse` guard group temper installs, running `command` at [`GUARD_MATCHER`].
fn guard_group(command: &str) -> JsonValue {
    json!({
        "matcher": GUARD_MATCHER,
        "hooks": [ { "type": "command", "command": command } ]
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
        .is_some_and(|groups| {
            groups
                .iter()
                .any(|group| group_has_command(group, |command| command == SESSION_START_COMMAND))
        })
}

/// Whether a `PreToolUse` group carrying *this exact* guard command is already present.
/// A differing command reads `false`, so the guard reports as (re)applied and
/// [`splice_guard_command`] rewrites it.
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

/// Whether a `PreToolUse` group carrying *any* temper guard command (identified by
/// [`GUARD_MARKER`]) is already present, regardless of its exact command — the
/// "update in place vs. append fresh" fork [`splice_hooks`] reads.
fn guard_marker_present(object: &serde_json::Map<String, JsonValue>) -> bool {
    object
        .get("hooks")
        .and_then(|hooks| hooks.get("PreToolUse"))
        .and_then(JsonValue::as_array)
        .is_some_and(|groups| {
            groups
                .iter()
                .any(|group| group_has_command(group, |command| command.contains(GUARD_MARKER)))
        })
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
// ---------------------------------------------------------------------------

/// The scaffold subdirectory a bare kind's member modules live under.
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

/// The SDK's own inline-prose threshold (`sdk/src/prose.ts`, "the three-line
/// rule"): a body at or under this many lines lives inline as a `text` template
/// literal; a longer one is a document, written to a module-adjacent file.
const INLINE_PROSE_LINE_LIMIT: usize = 3;

/// Scaffold the SDK program from `discovery`'s findings — the lift's whole
/// output, a **whole conversion** (0016), never an intermediate state: a member
/// module per discovered artifact hoisting every present frontmatter field into
/// a typed property ([`member_module_source`]) and moving its prose
/// module-side, plus a `harness.ts` skeleton importing them all. Writes nothing
/// under `dry_run`, returning only the count a real run would scaffold.
///
/// # Errors
/// Returns a [`miette::Report`] if a member's source cannot be parsed or a
/// scaffold file cannot be written.
fn scaffold(
    temper_dir: &Path,
    discovery: &DiscoveryReport,
    dry_run: bool,
) -> miette::Result<usize> {
    let kinds = builtin_kind::definitions()?;

    let mut lifted: Vec<(String, frontmatter::Member)> = Vec::new();
    for (name, files) in &discovery.members {
        let Some(kind) = kinds.get(name) else {
            continue;
        };
        // A layout kind's document is a source, not a projection — its authored home
        // never moves, so the lift never converts it into a member module.
        if kind.content != kind::Content::File {
            continue;
        }
        for file in files {
            lifted.push((name.clone(), frontmatter::Member::from_source(kind, file)?));
        }
    }
    lifted.sort_by(|(a_kind, a), (b_kind, b)| (a_kind, &a.id).cmp(&(b_kind, &b.id)));

    if dry_run {
        return Ok(lifted.len());
    }

    let mut scaffolded = Vec::with_capacity(lifted.len());
    for (kind, member) in &lifted {
        let ident = member_ident(kind, &member.id);
        let dir = temper_dir.join(member_dir(kind));
        write_scaffold_file(
            &dir.join(format!("{}.ts", member.id)),
            &member_module_source(kind, &member.id, &ident, &member.fields, &member.body),
        )?;
        if !fits_inline(&member.body) {
            write_scaffold_file(&dir.join(format!("{}.md", member.id)), &member.body)?;
        }
        scaffolded.push(ScaffoldedMember {
            ident,
            import_path: format!("./{}/{}.ts", member_dir(kind), member.id),
        });
    }

    write_scaffold_file(
        &temper_dir.join(HARNESS_ENTRY),
        &harness_entry_source(&scaffolded),
    )?;

    Ok(lifted.len())
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

/// Whether `body` lives inline as a `` text`…` `` template literal rather than a
/// module-adjacent file: at or under [`INLINE_PROSE_LINE_LIMIT`] lines, and
/// carrying at least one line already flush left. The template's `dedent`
/// (`sdk/src/prose.ts`) strips the *minimum* indentation across its non-blank
/// lines; a body with no flush-left line would have real leading whitespace
/// stripped by that pass, so such a body stays a document instead.
fn fits_inline(body: &str) -> bool {
    if body.is_empty() {
        return true;
    }
    body.lines().count() <= INLINE_PROSE_LINE_LIMIT
        && body
            .lines()
            .any(|line| line.chars().next().is_some_and(|c| !c.is_whitespace()))
}

/// One lifted member's module source (0016, whole conversion): every present
/// frontmatter field but `name` (already the object literal's identity
/// property) hoists into its own typed TS property via [`json_to_ts_literal`],
/// in the same order [`frontmatter::Member::fields`] carries them; `body`
/// moves module-side — inline as a `` text`…` `` literal ([`fits_inline`]) or,
/// for a document, a `file()` reference to the module-adjacent `<name>.md`
/// [`scaffold`] writes beside this module. Replaces the retired own-path lift:
/// the projected artifact is never this module's own `file()` source.
fn member_module_source(
    kind: &str,
    name: &str,
    ident: &str,
    fields: &[(String, JsonValue)],
    body: &str,
) -> String {
    let mut fields_src = String::new();
    for (key, value) in fields {
        if key == "name" {
            continue;
        }
        fields_src.push_str(&format!(
            "  {}: {},\n",
            ts_property_key(key),
            json_to_ts_literal(value)
        ));
    }

    let prose_src = if fits_inline(body) {
        format!("  prose: {},\n", inline_prose_literal(body))
    } else {
        format!("  prose: file(import.meta.url, \"./{name}.md\"),\n")
    };

    format!(
        "import {{ file, text, {kind} }} from \"@dtmd/temper/claude-code\";\n\nexport const {ident} = {kind}({{\n  name: {name:?},\n{fields_src}{prose_src}}});\n"
    )
}

/// A frontmatter field key rendered as a TS object-literal property key: a bare
/// identifier when the key already is one (`description`), else a quoted string
/// literal (`"disable-model-invocation"`) — mirroring how the SDK's own kind
/// interfaces spell a hyphenated field (`sdk/src/builtins.ts`).
fn ts_property_key(key: &str) -> String {
    let is_identifier = key.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_' || c == '$')
        && key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$');
    if is_identifier {
        key.to_string()
    } else {
        format!("{key:?}")
    }
}

/// Render a JSON frontmatter value as a TS literal. JSON's grammar is a
/// syntactic subset of TS/JS object- and array-literal syntax, so serializing
/// the value as JSON already renders it as a TS literal — one renderer generic
/// over every JSON shape a scaffolded field carries (string, number, bool,
/// array, object), replacing the description-only special case the lift used
/// to carry.
fn json_to_ts_literal(value: &JsonValue) -> String {
    serde_json::to_string(value).expect("a JSON value serializes infallibly")
}

/// Render `body` as a `` text`…` `` tagged-template literal, byte-faithful
/// through the SDK's `dedent` (`sdk/src/prose.ts`): every line lands flush left
/// in the generated source (never re-indented to match the surrounding object
/// literal), so `dedent`'s strip — computed off [`fits_inline`]'s guaranteed
/// flush-left line — is a no-op and `body` round-trips exactly, trailing
/// newline included or not.
fn inline_prose_literal(body: &str) -> String {
    if body.is_empty() {
        return "text``".to_string();
    }
    let trailing_newline = body.ends_with('\n');
    let content = body.strip_suffix('\n').unwrap_or(body);
    let mut out = String::from("text`\n");
    for line in content.split('\n') {
        out.push_str(&escape_template_literal(line));
        out.push('\n');
    }
    if !trailing_newline {
        out.pop();
    }
    out.push('`');
    out
}

/// Escape a template-literal line so its authored text survives verbatim as TS
/// source: a backslash, a backtick, or a `${` would otherwise end the literal
/// early or open an interpolation.
fn escape_template_literal(line: &str) -> String {
    line.replace('\\', "\\\\")
        .replace('`', "\\`")
        .replace("${", "\\${")
}

/// The `harness.ts` skeleton: import every scaffolded member, compose them into
/// `harness({ members: [...] })`, and print `emit`'s seam to stdout — the whole
/// program `emit_program` then runs.
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

/// The npm executable name to spawn: `Command`'s child-process launch does not
/// consult `PATHEXT`, so it resolves only an exact-match filename. Windows ships
/// npm as `npm.cmd` (no bare `npm.exe`), so a `windows` spawn must name the shim
/// explicitly; everywhere else `npm` on `PATH` is the real executable.
pub fn npm_program() -> &'static str {
    if cfg!(windows) { "npm.cmd" } else { "npm" }
}

/// Ensure the `@dtmd/temper` dependency: declare it in `.temper/package.json`
/// (creating a minimal manifest when absent) and `npm install` it. Idempotent by
/// construction — [`run_represented`] only calls this when [`dependency_resolves`]
/// already reads `false`.
fn ensure_dependency(temper_dir: &Path) -> Result<(), InstallError> {
    ensure_package_json(temper_dir)?;
    let output = Command::new(npm_program())
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
        .or_insert_with(|| json!(sdk_version_range()));

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
    let inner = frontmatter::closing_delimiter(rest).map(|(matter, _)| matter)?;
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
/// comment, or `None` when it has no frontmatter to carry it — a memory `CLAUDE.md`
/// and every frontmatterless kind, which [`project_banner`] serves with the
/// block-level HTML-comment form instead. Applied *before* the modeline so the
/// modeline stays the leading line.
///
/// **Content-drift-aware**: idempotence keys on the note's *bytes*, not the bare [`NOTE_MARKER`]
/// prefix. A marked line whose body still matches [`NOTE_COMMENT`] is returned
/// verbatim (no churn); a marked line carrying a retired wording — the reword that
/// [`NOTE_COMMENT`] shipped — is *re-placed*, splicing the current [`NOTE_COMMENT`]
/// over the stale line so a changed placement re-places instead of reporting
/// `Unchanged`. Presence-only keying let a stale note pass `gate_installed` forever.
///
/// Byte-faithful (`.claude/rules/rust.md`, round-trip discipline): the note line is
/// the only rewritten bytes. The note rides `install`, never `emit` — a YAML comment
/// is not authored surface content, so the content-faithful projector
/// never re-emits it.
fn project_note(source: &str) -> Option<String> {
    let rest = source.strip_prefix("---\n")?;
    let inner = frontmatter::closing_delimiter(rest).map(|(matter, _)| matter)?;
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

/// Project a frontmatterless markdown `source` with the managed-by banner prepended as
/// a block-level HTML comment at the head of the body, or `None` when `source` carries
/// frontmatter (the `#` [`project_note`] form owns that case). One blank line separates
/// the banner from the body, so it renders as its own block.
///
/// **Content-drift-aware**, exactly as [`project_note`]: idempotence keys on the
/// banner's *bytes*, not the bare [`BANNER_MARKER`] prefix. A leading banner whose line
/// matches [`NOTE_BANNER`] is returned verbatim (no churn); one carrying a retired
/// wording is *re-placed*; an absent one is prepended. Byte-faithful — the banner line
/// (and its separating newline) are the only inserted bytes.
fn project_banner(source: &str) -> Option<String> {
    // A frontmatter source is the note's, not the banner's — never shove a comment
    // ahead of a leading `---`, which would break the frontmatter block.
    if source.starts_with("---\n") {
        return None;
    }
    let first = source.lines().next().unwrap_or_default();
    if first.trim_start().starts_with(BANNER_MARKER) {
        if first == NOTE_BANNER {
            return Some(source.to_string());
        }
        return Some(source.replacen(first, NOTE_BANNER, 1));
    }
    Some(format!("{NOTE_BANNER}\n\n{source}"))
}

/// Whether `path` names a markdown file — the one body shape the block-level
/// HTML-comment banner is safe in (an HTML comment is inert markdown, malformed inside
/// a JSON manifest). Gates [`project_banner`] so only a markdown-bodied frontmatterless
/// projection grows one.
fn is_markdown_path(path: &Path) -> bool {
    path.extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
}

/// The install-placed managed-metadata lines present in `source`, in on-disk order —
/// the schema modeline and the managed-by note (as a frontmatter `#` comment), or the
/// block-level HTML-comment banner heading a frontmatterless markdown body. `emit`
/// round-trips these through its whole-file re-emit so its content-faithful projection
/// carries install's metadata instead of dropping it: install owns *placing and
/// auditing* them, emit only *preserves* what is already there. Empty when `source`
/// carries none.
pub(crate) fn placement_lines(source: &str) -> Vec<String> {
    if let Some(rest) = source.strip_prefix("---\n")
        && let Some((inner, _)) = frontmatter::closing_delimiter(rest)
    {
        return inner
            .lines()
            .filter(|line| is_placement_comment(line))
            .map(str::to_string)
            .collect();
    }
    // Frontmatterless: install's banner rides the head of the body, not a frontmatter
    // block. Return it so emit re-places it exactly as it re-places the `#` note.
    source
        .lines()
        .next()
        .filter(|line| line.trim_start().starts_with(BANNER_MARKER))
        .map(|line| vec![line.to_string()])
        .unwrap_or_default()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::tmpdir;

    #[test]
    fn member_module_source_hoists_every_present_field_bar_name() {
        // A document body (over the inline threshold) so the field-hoisting
        // shape is exercised independent of the prose split — `file()`
        // referencing the module-adjacent doc `scaffold` writes beside it.
        let fields = vec![
            ("name".to_string(), serde_json::json!("coordinate")),
            (
                "description".to_string(),
                serde_json::json!("Use when coordinating agents across axes."),
            ),
            ("license".to_string(), serde_json::json!("MIT")),
            (
                "disable-model-invocation".to_string(),
                serde_json::json!(true),
            ),
        ];
        let body =
            "# Coordinate\n\nDrive the team through the playbook.\n\nMore than three lines.\n";
        let source = member_module_source("skill", "coordinate", "skill_coordinate", &fields, body);
        assert_eq!(
            source,
            "import { file, text, skill } from \"@dtmd/temper/claude-code\";\n\n\
             export const skill_coordinate = skill({\n  \
             name: \"coordinate\",\n  \
             description: \"Use when coordinating agents across axes.\",\n  \
             license: \"MIT\",\n  \
             \"disable-model-invocation\": true,\n  \
             prose: file(import.meta.url, \"./coordinate.md\"),\n});\n"
        );
    }

    #[test]
    fn member_module_source_carries_a_hoisted_array_field_and_no_description() {
        let fields = vec![("paths".to_string(), serde_json::json!(["src/**/*.rs"]))];
        let source = member_module_source("rule", "rust", "rule_rust", &fields, "# Rust\n");
        assert_eq!(
            source,
            "import { file, text, rule } from \"@dtmd/temper/claude-code\";\n\n\
             export const rule_rust = rule({\n  \
             name: \"rust\",\n  \
             paths: [\"src/**/*.rs\"],\n  \
             prose: text`\n# Rust\n`,\n});\n"
        );
    }

    #[test]
    fn member_module_source_skips_the_name_field_to_avoid_a_duplicate_key() {
        // `name` rides the object literal's top-level identity property
        // ([`member_module_source`]); a kind whose frontmatter also declares a
        // `name` field (an agent's identity source) must not get a second one.
        let fields = vec![
            ("name".to_string(), serde_json::json!("reviewer")),
            (
                "description".to_string(),
                serde_json::json!("Reviews pull requests."),
            ),
        ];
        let source = member_module_source("agent", "reviewer", "agent_reviewer", &fields, "");
        assert_eq!(source.matches("name:").count(), 1);
    }

    #[test]
    fn fits_inline_holds_for_an_empty_or_short_flush_left_body() {
        assert!(fits_inline(""));
        assert!(fits_inline("Pushback is the point.\n"));
        assert!(fits_inline("# Collaboration\n\nPushback is the point.\n"));
    }

    #[test]
    fn fits_inline_fails_past_the_line_limit_or_with_no_flush_left_line() {
        assert!(!fits_inline("one\ntwo\nthree\nfour\n"));
        // Every non-blank line indented — dedent would strip real content.
        assert!(!fits_inline("    indented one\n    indented two\n"));
    }

    #[test]
    fn inline_prose_literal_preserves_a_missing_trailing_newline() {
        assert_eq!(
            inline_prose_literal("Last line, no newline."),
            "text`\nLast line, no newline.`"
        );
    }

    #[test]
    fn inline_prose_literal_escapes_backticks_and_interpolation_markers() {
        assert_eq!(
            inline_prose_literal("a `code` span and a ${literal}\n"),
            "text`\na \\`code\\` span and a \\${literal}\n`"
        );
    }

    #[test]
    fn ts_property_key_quotes_a_hyphenated_key_and_bares_a_plain_one() {
        assert_eq!(ts_property_key("description"), "description");
        assert_eq!(
            ts_property_key("disable-model-invocation"),
            "\"disable-model-invocation\""
        );
    }

    #[test]
    fn json_to_ts_literal_renders_every_json_shape_as_valid_ts() {
        assert_eq!(json_to_ts_literal(&serde_json::json!("x")), "\"x\"");
        assert_eq!(json_to_ts_literal(&serde_json::json!(true)), "true");
        assert_eq!(json_to_ts_literal(&serde_json::json!(7)), "7");
        assert_eq!(
            json_to_ts_literal(&serde_json::json!(["a", "b"])),
            "[\"a\",\"b\"]"
        );
    }

    #[test]
    fn ensure_package_json_pins_a_range_past_the_file_export() {
        let dir = tmpdir("package-json-seam");
        ensure_package_json(&dir).unwrap();
        let written: JsonValue =
            serde_json::from_str(&fs::read_to_string(dir.join("package.json")).unwrap()).unwrap();
        assert_eq!(written["dependencies"][SDK_PACKAGE], sdk_version_range());
    }

    #[test]
    fn project_banner_prepends_the_html_comment_and_a_re_run_converges() {
        let body = "# Project\n\nProject-wide memory for the agents.\n";
        let placed = project_banner(body).expect("a frontmatterless markdown body takes a banner");
        assert!(placed.starts_with(&format!("{NOTE_BANNER}\n\n")));
        assert!(placed.ends_with(body));
        // Content-keyed idempotence: a second pass is byte-identical, never a duplicate.
        assert_eq!(project_banner(&placed).as_deref(), Some(placed.as_str()));
        assert_eq!(placed.matches(BANNER_MARKER).count(), 1);
    }

    #[test]
    fn project_banner_declines_a_frontmatter_source() {
        // A frontmatter body is the `#` note's; the banner never fronts a `---` block.
        assert_eq!(project_banner("---\nname: x\n---\n# Body\n"), None);
    }

    #[test]
    fn project_banner_re_places_a_stale_wording() {
        let stale = "<!-- temper: managed projection — old wording. -->\n\n# Project\n";
        let placed = project_banner(stale).expect("a marked-but-stale banner re-places");
        assert!(placed.starts_with(NOTE_BANNER));
        assert!(placed.ends_with("\n\n# Project\n"));
        assert_eq!(placed.matches(BANNER_MARKER).count(), 1);
    }

    #[test]
    fn is_markdown_path_gates_the_banner_to_markdown() {
        assert!(is_markdown_path(Path::new("CLAUDE.md")));
        assert!(is_markdown_path(Path::new(
            ".claude/rules/collaboration.md"
        )));
        assert!(!is_markdown_path(Path::new(".mcp.json")));
    }

    #[test]
    fn placement_lines_round_trips_the_body_banner_of_a_frontmatterless_source() {
        let source = format!("{NOTE_BANNER}\n\n# Project\n\nMemory body.\n");
        assert_eq!(placement_lines(&source), vec![NOTE_BANNER.to_string()]);
        // A bare frontmatterless body carries no placement.
        assert!(placement_lines("# Project\n\nMemory body.\n").is_empty());
    }
}
