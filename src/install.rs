//! `temper install` — the one on-ramp.
//!
//! `install` opens with a discovery report ([`discover`]/[`render_discovery`]) —
//! findings first, ceremony after — then asks exactly one question via [`Represent`]:
//! represent this project as a temper program?
//!
//! - **No** wires the `SessionStart` reporter alone ([`place_settings_only`]) and
//!   stops — the stranger gate at session start, Node-free forever.
//! - **Yes** requires Node (checked up front, refused loud with instructions when
//!   absent), ensures the `@dtmd/temper` dependency is declared in
//!   `.temper/package.json` ([`ensure_package_json`]) and spawns `npm install`
//!   to fetch it when not already resolved via an ancestor `node_modules`
//!   ([`spawn_npm_install`]) — before a single file of the lift is written, so
//!   a spawn failure never leaves a half-scaffolded `.temper/` behind it — then
//!   scaffolds the SDK
//!   program once if none exists yet — the lift ([`scaffold`]): a whole
//!   conversion (0016), never an intermediate state — every present frontmatter
//!   field hoists into a typed property and prose moves module-side (inline for
//!   a short body, a module-adjacent file for a document), and a host manifest's
//!   **registration** entries convert beside the container member holding them,
//!   one module per entry, so no segment reaches the first emit unowned — plus a
//!   `harness.ts`
//!   skeleton, and temper's own gate as three `hook` members ([`GATE_HOOKS`]) —
//!   runs the first `emit` (the adoption moment,
//!   [`drift::emit_program`]), which regenerates every composed kind's artifact as a
//!   canonical projection — a layout kind's document stays a source at either depth,
//!   never regenerated — and places the managed-by note /
//!   schema modeline at every path the fresh lock declares **emit-owned**
//!   ([`drift::emit_owned_targets`], [`evaluate_placements`]) — the first emit's
//!   diff is the one reviewable adoption diff, never an own-path passthrough.
//!
//! **One writer per file.** On the yes-path every manifest the represented program
//! registers inside — `.claude/settings.json`, `.mcp.json`, whichever the declared
//! channels name — is a projection the program owns whole, so the gate hooks reach
//! `settings.json` the way every other member reaches its
//! artifact — through `emit`. Splicing them in beside `emit` reported `applied` and wired
//! nothing: the re-stamp emit below re-rendered the file from the program and erased the
//! splice in the same run. [`place_settings_only`] keeps the merge for the **no**-path,
//! where the file is the human's and no program renders it.
//!
//! [`gate_installed`] is the read-only shadow `check` folds in: the same placement
//! evaluation, dry-run, collapsed to one advisory [`Diagnostic`]. It never scaffolds,
//! installs a dependency, or emits — `install` alone adopts.
//!
//! **Fail-loud**: a placement, a scaffold write, or a dependency/emit step that
//! cannot complete is a hard [`InstallError`] / propagated [`miette::Report`], never
//! a silent skip.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::LazyLock;

use serde_json::{Value as JsonValue, json};
use toml_edit::DocumentMut;

use crate::builtin_kind;
use crate::check::{Diagnostic, Severity};
use crate::compose::EnforcementMode;
use crate::contract::Contract;
use crate::drift::{self, ApplyOutcome, EmitReport};
use crate::engine;
use crate::frontmatter;
use crate::import;
use crate::json_manifest;
use crate::json_splice;
use crate::kind::{
    self, CollectionAddress, CollectionKeyPath, CustomKind, HOOK_HANDLER_KEY, HOOK_MATCHER_KEY,
};
use crate::placement::{MODELINE_MARKER, NOTE_COMMENT, NOTE_MARKER};
use crate::toml_document;

thread_local! {
    /// Per-thread count of the guard shell edge's locus walks: every [`WalkDir`] this
    /// module builds bumps it once ([`locus_member_sites`]). The walk is single-threaded
    /// on its caller's thread, so this counts one call's walks in isolation — a
    /// concurrent run on another test thread cannot perturb it.
    ///
    /// [`WalkDir`]: walkdir::WalkDir
    static LOCUS_MEMBER_SITE_WALKS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// This thread's cumulative count of guard locus-root walks. Read before and after a
/// [`shell_edge_findings`] call and compare the delta to the locus roots the caller
/// declared, pinning that the shell edge's per-tool-call cost is one bounded walk per
/// root — never a re-walk per candidate file or glob, and never a widening to one
/// whole-tree walk.
#[must_use]
pub fn locus_member_site_walk_count() -> usize {
    LOCUS_MEMBER_SITE_WALKS.with(std::cell::Cell::get)
}

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
///
/// Guarded by a PATH-resolvability check: if `temper` is not found on PATH,
/// exits non-zero with a clear error message naming the missing binary, per the
/// fail-loud invariant.
pub const SESSION_START_COMMAND: &str = "command -v temper >/dev/null 2>&1 || { echo \"temper: command not found\" >&2; exit 127; } && temper check . --reporter session-start";

/// The placement kind and its display labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placement {
    /// The `SessionStart` hook — the reporting harness bootstrapped at session start.
    SessionStart,
    /// A schema modeline in a frontmatter artifact.
    Modeline,
    /// The `PreToolUse` enforcement-mode guard hook — the guard's pending-write edge.
    GuardHook,
    /// The `PostToolUse` guard hook — the same guard's shell edge, judging the tree a
    /// Bash call left.
    PostToolUseHook,
    /// A managed-by note in a frontmatter artifact.
    Note,
}

impl Placement {
    /// The display label for this placement, as reported and carried in diagnostics.
    pub fn label(&self) -> &'static str {
        match self {
            Placement::SessionStart => "session-start hook",
            Placement::Modeline => "schema modeline",
            Placement::GuardHook => "guard hook",
            Placement::PostToolUseHook => "post-tool-use hook",
            Placement::Note => "managed-by note",
        }
    }
}

impl std::fmt::Display for Placement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

/// The rule id the self-verify diagnostics ([`gate_installed`]) carry.
const GATE_RULE: &str = "install.gate-installed";

/// The tool-name matcher the guard hook binds — Claude Code's own write boundary.
/// The guard binds *this provider's* writes only (Write, Edit, MultiEdit); other
/// consumers of a shared file and direct Bash/PowerShell writes are not instrumented by it.
/// (`code.claude.com/docs/en/hooks`, retrieved 2026-07-24).
const GUARD_MATCHER: &str = "Write|Edit|MultiEdit";

/// The exec-form command **both** guard hook rows run: the `temper` binary's own
/// `guard` subcommand, reading the payload from stdin and deciding at the harness's
/// declared enforcement mode. The `.` roots the
/// lock the enforcement mode is read from — the project Claude Code runs the hook in.
///
/// One command at two edges, because the payload already says which edge fired it
/// ([`guard_edge`]): before a file-writing tool it judges the pending write the payload
/// names, after a shell tool the tree the call left. The alternative — a second command
/// per edge — makes the wiring, not the payload, the thing that decides, and a hook
/// stamping the wrong event's name in its output is rejected whole
/// ([`crate::reporter::HookEvent`]).
///
/// Guarded by a PATH-resolvability check: if `temper` is not found on PATH,
/// exits non-zero with a clear error message naming the missing binary, per the
/// fail-loud invariant.
///
/// Public so the guard-hook acceptance can drive the exact wired command.
pub const GUARD_COMMAND: &str = "command -v temper >/dev/null 2>&1 || { echo \"temper: command not found\" >&2; exit 127; } && temper guard .";

/// temper's gate commands paired with the TS binding each is exported under across the
/// SDK seam. The engine is the one home for the *bytes* — above — and this table is the
/// one home for the *names*: the seam gate (`tests/it/seam_bindings_current.rs`)
/// generates `sdk/src/generated/gate-commands.ts` from these pairs, the SDK re-exports
/// them from `@dtmd/temper/claude-code`, and a scaffolded gate-hook module imports the
/// binding instead of copying the bytes. So a command respelled here cannot leave a
/// represented harness running yesterday's spelling, and it cannot go stale on the SDK
/// side without the seam gate failing loud (0073).
///
/// Sorted by binding name: the generated module's export order, the barrel's re-export
/// line and a scaffolded module's import list are all read off this order.
///
/// Public so the seam gate writes that module from the same pairs the scaffold imports —
/// a second table beside this one is exactly the drift the generation exists to end.
pub const GATE_COMMAND_BINDINGS: [(&str, &str); 2] = [
    ("GUARD_COMMAND", GUARD_COMMAND),
    ("SESSION_START_COMMAND", SESSION_START_COMMAND),
];

/// The binding naming `value` where `value` is one of temper's own gate commands — the
/// spelling a scaffolded module imports in place of those bytes.
fn gate_command_binding(value: &str) -> Option<&'static str> {
    GATE_COMMAND_BINDINGS
        .iter()
        .find(|(_, command)| *command == value)
        .map(|(binding, _)| *binding)
}

/// The tool-name matcher the guard's `PostToolUse` row binds — direct Bash tool
/// invocations. A shell tool's writes name no path in the payload, so the `PreToolUse`
/// guard cannot see them; this row runs the same guard after the call and judges the
/// tree it left ([`shell_edge_findings`]).
/// (`code.claude.com/docs/en/hooks`, retrieved 2026-09-03).
const BASH_MATCHER: &str = "Bash";

/// How many hook groups temper's gate rides — the width of [`GATE_HOOKS`] and of every
/// per-hook reading taken beside it.
const GATE_HOOK_COUNT: usize = 3;

/// One of the three hook groups temper's own gate rides, as the `hook` **member** the
/// lift mints it as: the lifecycle event it keys under, the tool-name matcher it binds
/// (`None` where the event carries no tool), and the exec-form command it runs.
struct GateHook {
    /// The placement row [`run`] and [`gate_installed`] report this hook under.
    placement: Placement,
    /// The `hooks.<Event>` key — the member's `name` property and its module stem, while
    /// the member's own identity joins this with [`matcher`](Self::matcher).
    event: &'static str,
    /// The tool-name filter, absent on an event that fires unconditionally.
    matcher: Option<&'static str>,
    /// The exec-form command, this module's own constant.
    command: &'static str,
}

/// temper's whole gate, in the order the placement rows report it. On the represented
/// path these are program members: [`scaffold`] mints one module apiece, `emit` projects
/// them into `.claude/settings.json`'s `hooks` collection, and nothing else writes that
/// file — the settings document is a projection the program owns whole
/// ([`builtin_kind`]'s `settings` kind), so a second writer splicing the same bytes would
/// be erased by the next emit. [`place_settings_only`] is the unrepresented path's own
/// writer and places [`Placement::SessionStart`] alone: with no program there is no
/// projection to guard and nothing for `emit` to re-render.
const GATE_HOOKS: [GateHook; GATE_HOOK_COUNT] = [
    GateHook {
        placement: Placement::SessionStart,
        event: "SessionStart",
        matcher: None,
        command: SESSION_START_COMMAND,
    },
    GateHook {
        placement: Placement::GuardHook,
        event: "PreToolUse",
        matcher: Some(GUARD_MATCHER),
        command: GUARD_COMMAND,
    },
    GateHook {
        placement: Placement::PostToolUseHook,
        event: "PostToolUse",
        matcher: Some(BASH_MATCHER),
        command: GUARD_COMMAND,
    },
];

/// The binding limit every pending-write guard message closes on: the guard rides
/// Claude Code's `PreToolUse` row, so it sees exactly the tools [`GUARD_MATCHER`] binds
/// and nothing else. The tool list is that matcher's own alternation rendered for prose
/// (`|` as `/`), so widening the row widens the sentence with it. The post edge carries
/// no limit at all ([`GUARD_SHELL_EDGE_MESSAGE`]) — the write has landed, and there is
/// nothing left to refuse.
///
/// Public so the block-mode guard surface pin can assert the tail reaches the user
/// against the writer's own bytes rather than a hand copy of them.
pub fn binding_limit() -> &'static str {
    static LIMIT: LazyLock<String> = LazyLock::new(|| {
        format!(
            "This guard binds only Claude Code tool-mediated writes ({}); direct Bash/PowerShell writes are not bound by it.",
            GUARD_MATCHER.replace('|', "/")
        )
    });
    &LIMIT
}

/// The message `temper guard` prints on a projection hit — the `guard` subcommand (`main`)
/// prints it whether it warns or blocks, under the `warn` or `block` enforcement mode.
fn guard_message() -> &'static str {
    static MESSAGE: LazyLock<String> = LazyLock::new(|| {
        format!(
            "temper-managed projection: .claude/ is projected from the .temper/ surface — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit. {}",
            binding_limit()
        )
    });
    &MESSAGE
}

/// The message `temper guard` prints when a pending write lands inside a represented
/// committed kind's governed locus and names no member the lock declares there. It is
/// the boundary half of `check`'s root `locus-declared` clause
/// ([`drift::undeclared_locus_members_from_doc`]) and names the same remedy in the same
/// words — one fact must not be spoken two ways at two placements. Unlike
/// [`guard_message`], the file is not a projection the author edited: it is a document
/// the program declares nothing about, so the finding names the governing kind rather
/// than an owning module.
///
/// `custom` is the governing kind's own ([`GuardedLocus::custom`]), and it decides the
/// loader claim exactly as it does in `check`'s finding: Claude Code loads a built-in
/// kind's locus, and who reads a locus the author's own kind declares is not temper's to
/// assert.
fn undeclared_locus_message(kind: &str, custom: bool) -> String {
    let loader_claim = if custom {
        ""
    } else {
        ", yet Claude Code loads it"
    };
    format!(
        "temper-governed locus: this write lands a document at the `{kind}` kind's governed locus that the lock declares no member for — `emit` will never maintain it and `check` reports it undeclared{loader_claim}; declare the member in the program and re-emit. {}",
        binding_limit()
    )
}

/// The header `temper guard` prints when a pending write to a represented manifest carries a
/// member that violates its contract — the per-member contract findings ([`GuardedManifest`])
/// follow it, one per line. Unlike a `.claude/` projection ([`guard_message`]), a manifest **no
/// container member projects** is co-owned: a write touching only opaque residue conforms and
/// passes, so the finding names the contract broken, not the file edited. A manifest a container
/// member owns whole is not co-owned at all and never reaches this header
/// ([`GuardedManifest::container`]) — it is a projection, and speaks the projection wording.
fn guard_manifest_message() -> &'static str {
    static MESSAGE: LazyLock<String> = LazyLock::new(|| {
        format!(
            "temper-governed manifest: a member of this write violates its contract — fix the member to conform, or challenge the contract. {}",
            binding_limit()
        )
    });
    &MESSAGE
}

/// The header `temper guard` prints when a pending `Edit`/`MultiEdit` to a represented
/// manifest cannot be reconstructed into the whole manifest it would land, so no member was
/// checked at all. A co-owned manifest never earns the blanket projection wording
/// ([`guard_message`]) — the way through is a write the guard can read, not an untouched file.
fn guard_manifest_edit_message() -> &'static str {
    static MESSAGE: LazyLock<String> = LazyLock::new(|| {
        format!(
            "temper-governed manifest: this edit cannot be reconstructed into the manifest it would land, so its governed members went unchecked — re-issue the change as a whole-file Write. {}",
            binding_limit()
        )
    });
    &MESSAGE
}

/// The header `temper guard` prints when the manifest a pending write would land was
/// reconstructed in full and does not parse. No later placement catches this one: a harness
/// carrying an unparseable manifest cannot load, so the next `check` aborts before any
/// reporter runs — and for `.claude/settings.json` the `SessionStart` hook that would have
/// carried the verdict is declared in the file that no longer parses. The boundary is the
/// only placement left, so it speaks rather than deferring to CI.
fn guard_manifest_unparseable_message() -> &'static str {
    static MESSAGE: LazyLock<String> = LazyLock::new(|| {
        format!(
            "temper-governed manifest: this write would leave the manifest unparseable, so nothing it governs can be checked — and a harness that cannot load aborts the next temper check before any reporter runs, so no later placement catches it either. Fix the JSON before landing it. {}",
            binding_limit()
        )
    });
    &MESSAGE
}

/// The header `temper guard` prints at the **post** edge of a tool call over its
/// **projection** half, where a shell tool's writes name no path the guard could have bound
/// before the fact and the tree it left is the whole subject. The per-row drift findings
/// ([`shell_edge_findings`]) follow it, one per line, each already naming the projection
/// that moved and the member that owns it. It states the restore rather than a limit: the
/// write has landed, so there is nothing left to refuse but the call's result.
///
/// It states the **tree's condition**, never the call's authorship: the edge enumerates
/// the lock's declared set rather than reading a path off the payload, so a projection it
/// finds drifted may have been moved by any earlier call, or by no call at all.
const GUARD_SHELL_EDGE_MESSAGE: &str = "temper-managed projection drift: a committed projection is out of sync with the .temper/ surface the lock fingerprinted — edit the owning .temper/ module or document and re-run `temper emit` to restore it. A shell tool's writes name no path in the hook payload, so this edge judges the projection set the lock declares rather than one file, and cannot say which call moved it.";

/// The header `temper guard` prints at the **post** edge over its **locus** half: a document
/// the call left at a represented kind's governed locus that the lock declares no member
/// for. A separate preamble because [`GUARD_SHELL_EDGE_MESSAGE`] asserts *projection drift*
/// — a file the lock fingerprinted moved — and a stray at a governed locus is not that: the
/// lock never named it, so there is no fingerprint to restore and no owning module to edit.
/// Names the governing kind's locus rather than an owning member, the same split
/// [`undeclared_locus_message`] makes at the pending-write edge, and states the restore
/// rather than a limit for the same reason its drift sibling does.
///
/// Like its drift sibling it states the tree's condition and never the call's authorship.
/// It also carries no loader claim: it heads a set of strays from any number of kinds, and
/// whether Claude Code loads one turns on whose kind governs its locus — a per-kind fact
/// each finding under it already speaks ([`drift::undeclared_locus_members_from_doc`]).
const GUARD_SHELL_EDGE_LOCUS_MESSAGE: &str = "temper-governed locus: a document sits at a represented kind's governed locus that the lock declares no member for — `emit` will never maintain it and `check` reports it undeclared; declare the member in the program and re-emit, or remove the file. A shell tool's writes name no path in the hook payload, so this edge enumerates the loci the lock's kinds govern rather than one file, and cannot say which call left it.";

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
enum InstallError {
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

    /// A discovered member's id cannot name a module file inside its kind's directory
    /// under `.temper/` — a registration key carrying a path separator, say.
    #[error("the {kind} member {id:?} cannot name a module file in {dir}")]
    #[diagnostic(
        code(temper::install::member_file_name),
        help(
            "a member's module is one file in its kind's directory, named portably: no `< > : \" / \\ | ? *`, no control character, no trailing space or dot, and not a reserved device name (CON, PRN, AUX, NUL, COM1-9, LPT1-9); rename the entry so its key names one"
        )
    )]
    MemberFileName {
        /// The kind row label of the member whose module could not be placed.
        kind: String,
        /// The id that cannot name a file there.
        id: String,
        /// The module directory the file would have had to land in.
        dir: PathBuf,
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

/// One kind's discovered members, in the flavor that kind's own declaration decides. A
/// file-locus kind's members are the artifact files the walk found; a **registration**
/// kind owns no file at all — the walk finds its host manifest, and its members are the
/// entries at its declared [`CollectionAddress`] inside it (a `hooks.<Event>` matcher
/// group, an `enabledPlugins` enablement).
///
/// The two flavors never mix on one kind, so the report's per-kind count is one length
/// either way. Counting the manifest file instead reported `hook 1` for a
/// `settings.json` carrying no `hooks` key at all, and handed the lift a file with no
/// member of that kind in it to convert.
#[derive(Debug, Clone, PartialEq)]
pub enum KindMembers {
    /// A file-locus kind's members: one source file apiece.
    Files(Vec<PathBuf>),
    /// A registration kind's members: one collection entry apiece, read off its host
    /// manifests through the one manifest read face ([`json_manifest::Manifest`]).
    Registrations {
        /// The host manifests the walk found for this kind — the files its collection
        /// address is read at, carried whether or not one holds an entry, because the
        /// address claims its key in the manifest either way ([`scaffold`] keeps that
        /// key off the container member that owns the file).
        manifests: Vec<PathBuf>,
        /// The entries found at that address, across every one of them.
        entries: Vec<json_manifest::RegistrationMember>,
    },
}

impl KindMembers {
    /// How many members this kind contributes to the report's count.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Files(files) => files.len(),
            Self::Registrations { entries, .. } => entries.len(),
        }
    }

    /// Whether this kind contributed no member at all — the row [`render_discovery`]
    /// leaves unprinted.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The artifact files these members were read from: a file kind's own sources, and
    /// empty for a registration kind, whose entries surface inside a member of another
    /// kind's file rather than one of its own.
    #[must_use]
    pub fn files(&self) -> &[PathBuf] {
        match self {
            Self::Files(files) => files,
            Self::Registrations { .. } => &[],
        }
    }
}

/// The discovery walk's findings — "what the walk found (members by kind...)",
/// reported before
/// the one question and reused by the yes-path's scaffold so the lift lifts exactly
/// what was reported, never a re-walked, possibly-differing set.
#[derive(Debug, Clone, Default)]
pub struct DiscoveryReport {
    /// Discovered members, keyed by the kind's bare row label — every embedded built-in
    /// kind, each in the flavor its declaration decides ([`KindMembers`]).
    pub members: BTreeMap<String, KindMembers>,
}

impl DiscoveryReport {
    /// The total member count across every discovered kind.
    #[must_use]
    pub fn total(&self) -> usize {
        self.members.values().map(KindMembers::len).sum()
    }
}

/// Walk `root` for every embedded built-in kind's members — the discovery report's
/// data, computed once and shared by the printed report and the yes-path's scaffold.
/// The whole kind set travels with each kind: a nested file kind is discovered under its
/// host's units at the host template's pattern, so its own row of the report is a
/// function of another kind's declaration.
///
/// Adoption's walk withholds the local-locus override the read side honors
/// ([`import::LocalOverride`]): what this report finds becomes a committed member module
/// and its artifact a projection, and a local kind's per-machine document is neither —
/// its authored home never moves. So the ignore rules and the workspace skip stand here
/// whatever a kind declares.
///
/// A kind declaring a [`CollectionAddress`] is walked for its **host manifest** and then
/// read at that address: its members are the collection's entries, never the file
/// carrying them ([`KindMembers`]). The read is one per (kind, manifest) pair and so
/// bounded by the kind set — the walk is the tree-scale cost here, not this.
///
/// # Errors
/// Returns a [`miette::Report`] if a kind's discovery walk fails to read a directory, or
/// if a discovered host manifest cannot be read as JSON — a manifest install cannot parse
/// is one it could neither report nor lift, so it refuses loud rather than reporting zero.
pub fn discover(root: &Path) -> miette::Result<DiscoveryReport> {
    let mut members = BTreeMap::new();
    let kinds = builtin_kind::definitions();
    // One ignore-honoring walk per flavor, shared across every kind this report discovers
    // ([`import::Discovery`]).
    let disc = import::Discovery::new(root);
    for kind in kinds.values() {
        let files = import::discover_builtin(&disc, kind, &kinds, import::LocalOverride::Withheld);
        let found = match &kind.collection_address {
            Some(address) => KindMembers::Registrations {
                entries: files
                    .iter()
                    .map(|file| json_manifest::Manifest::read(file, &[address]))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .flat_map(|manifest| manifest.members)
                    .collect(),
                manifests: files,
            },
            None => KindMembers::Files(files),
        };
        members.insert(kind.name.clone(), found);
    }
    Ok(DiscoveryReport { members })
}

/// Render the discovery report for the terminal — findings first, ceremony after:
/// `mark`, the file [`represented_by`] found if any, naming what already answered the
/// represent question on disk — the question below the report is skipped, so the answer is
/// stated rather than left invisible — then member counts by kind, or a plain statement
/// that nothing was found.
#[must_use]
pub fn render_discovery(report: &DiscoveryReport, mark: Option<&Path>) -> String {
    let mut out = String::from("discovery:\n");
    if let Some(mark) = mark {
        out.push_str(&format!(
            "  already represented — {} answers the represent question\n",
            mark.display()
        ));
    }
    if report.total() == 0 {
        out.push_str("  no members found under this project's known kinds\n");
        return out;
    }
    for (kind, found) in &report.members {
        if found.is_empty() {
            continue;
        }
        out.push_str(&format!("  {kind:<20} {}\n", found.len()));
    }
    out
}

/// One placement's outcome from [`run`]/[`gate_installed`]: which placement, at
/// which path, and what the three-state merge decided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallEntry {
    /// The placement kind — the `SessionStart` hook, the guard, a modeline, or a note.
    pub placement: Placement,
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

    // What the settings document wired before this run touched anything — the only
    // reading of "before" the emit below cannot destroy, and the split between a gate
    // hook this run wired ([`ApplyOutcome::Applied`]) and one already in place.
    let gate_before = gate_hooks_state(&settings_path(root))?.map(|state| state.wired);

    // Assured before the lift writes a single member module: "no half-scaffolded
    // state" — a dependency spawn failure must never leave a partial `.temper/`
    // program behind it.
    if !dry_run {
        ensure_package_json(&temper_dir)?;
        if !dependency_resolves(&temper_dir) {
            spawn_npm_install(&temper_dir)?;
        }
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
        // Read *after* the emit above rewrote the lock: placements bind to the rows this
        // run just wrote, never to the previous run's.
        let lock = drift::read_lock_document(&temper_dir)?;
        evaluate_placements(root, dry_run, Some(gate_before), &lock)?
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

/// The authored program marking the harness at `root` **represented** —
/// `.temper/harness.ts` on disk — or `None` where there is none.
///
/// **The one reading every face takes**, so no two of them can disagree about which fork a
/// harness is on. The lock is not it: a harness carrying a program it has never emitted
/// carries no lock and is represented all the same, and reading the lock alone there lets
/// `--no-represent` be accepted and [`place_settings_only`] splice `.claude/settings.json`
/// — install re-adding a gate hook to a file the program owns whole, which 0073 forbids.
#[must_use]
pub fn represented_by(root: &Path) -> Option<PathBuf> {
    let entry = root.join(crate::WORKSPACE_DIR).join(HARNESS_ENTRY);
    entry.is_file().then_some(entry)
}

/// Report whether temper's own gate is installed and undrifted at `root` — the
/// `check` self-verify.
///
/// Never scaffolds, installs a dependency, or emits — only [`run`] adopts.
/// Evaluates the placements a *represented* project's current lock justifies
/// ([`represented_by`]), or — unrepresented — the session-start hook alone, both dry-run,
/// folded into **one advisory** [`Diagnostic`]. Always `warn`, never `error`; empty when
/// every placement is already in place.
///
/// `lock` is the caller's already-parsed `.temper/lock.toml`, **required rather than
/// optional**: this face runs inside `check`, which reads the lock once for every tier,
/// and an `Option` here would leave a second door onto the same file open for the next
/// in-engine caller to take by accident.
///
/// Each reported placement carries **its own remedy**, because they are not all install's
/// to converge. On a represented harness a gate hook is a member the author owns and
/// install never re-adds it (0073), so a missing or stale one names the module to author
/// and the `harness.ts` import line that reaches it ([`gate_hook_clause`]); the notes and
/// modelines really are install's write, so they share the one `temper install` clause.
#[must_use]
pub fn gate_installed(root: &Path, lock: &DocumentMut) -> Vec<Diagnostic> {
    let temper_dir = root.join(crate::WORKSPACE_DIR);
    if !temper_dir.is_dir() {
        return Vec::new();
    }
    let represented = represented_by(root).is_some();
    let Ok(entries) = (if represented {
        evaluate_placements(root, true, None, lock)
    } else {
        place_settings_only(root, true)
    }) else {
        return Vec::new();
    };

    // Only `Unchanged` is silence. A gate hook a member superseded is *stale*, not settled:
    // skipping it here is what let a member running a retired command spelling read green
    // forever while every tool call fired the wrong bytes (0073's Context). Modelines and
    // notes are one per modeled artifact, so each names its own file.
    let mut gate = Vec::new();
    let (mut modelines, mut notes) = (Vec::new(), Vec::new());
    for entry in &entries {
        if entry.outcome == ApplyOutcome::Unchanged {
            continue;
        }
        match entry.placement {
            Placement::Modeline => {
                modelines.push(format!("schema modeline {}", entry.path.display()));
            }
            Placement::Note => notes.push(format!("managed-by note {}", entry.path.display())),
            // The rest are the gate hooks, and [`GATE_HOOKS`] is the authority on which:
            // a placement with no row there has no member module to name a remedy in.
            placement => {
                if let Some(hook) = gate_hook_at(placement) {
                    gate.push(gate_hook_clause(hook, entry.outcome, represented));
                }
            }
        }
    }
    if gate.is_empty() && modelines.is_empty() && notes.is_empty() {
        return Vec::new();
    }

    let mut clauses = gate;
    let install_owned: Vec<String> = modelines.into_iter().chain(notes).collect();
    if !install_owned.is_empty() {
        clauses.push(format!(
            "run `temper install` to converge {}",
            install_owned.join(", ")
        ));
    }

    vec![Diagnostic::warn(
        GATE_RULE,
        root.to_string_lossy().into_owned(),
        format!(
            "temper's gate is not installed or has drifted — {}",
            clauses.join("; ")
        ),
    )]
}

/// The gate hook riding `placement`, when one does — how a reported [`InstallEntry`] gets
/// back to the member whose module names its remedy.
fn gate_hook_at(placement: Placement) -> Option<&'static GateHook> {
    GATE_HOOKS.iter().find(|hook| hook.placement == placement)
}

/// One gate hook that is not in place, named with the remedy for it.
///
/// On a represented harness the remedy is the **author's**, never the verb's: install
/// never re-adds a gate hook there (0073) — the author may have deleted one on purpose —
/// so `gate_installed`'s advisory is the whole intervention, and it has to say what to
/// write.
///
/// - **missing** ([`ApplyOutcome::Conflicted`]) — nothing is seated at the hook's
///   `(event, matcher)` address, so both halves are owed: the member module and the
///   `harness.ts` import line that reaches it. An unimported module is the unreached
///   member `reached-from` indicts, so naming only the module would author a second defect.
/// - **stale** ([`ApplyOutcome::SupersededByMember`]) — a member *is* seated there,
///   running something other than temper's command. The module exists and is imported, so
///   the remedy is the handler alone, pointed at the [`GATE_COMMAND_BINDINGS`] binding
///   rather than a literal that can go stale again.
///
/// Unrepresented, the session-start hook is install's own write
/// ([`place_settings_only`]), so the remedy there is the verb.
fn gate_hook_clause(hook: &GateHook, outcome: ApplyOutcome, represented: bool) -> String {
    let label = hook.placement.label();
    if !represented {
        return format!("{label} missing: run `temper install`");
    }
    let module = gate_hook_module(hook);
    if outcome == ApplyOutcome::SupersededByMember {
        let binding = gate_command_binding(hook.command)
            .expect("every gate hook's command is one of `GATE_COMMAND_BINDINGS`");
        return format!(
            "{label} stale: `{workspace}/{module}` runs something else, so point its handler at `{binding}` from `{SDK_PACKAGE}/claude-code`",
            workspace = crate::WORKSPACE_DIR
        );
    }
    format!(
        "{label} missing: author `{workspace}/{module}`, then add `import {{ {ident} }} from \"./{module}\";` to `{workspace}/{HARNESS_ENTRY}`",
        workspace = crate::WORKSPACE_DIR,
        ident = member_ident(GATE_HOOK_KIND, hook.event),
    )
}

/// Resolves the settings document every `install` placement writes under a project root.
fn settings_path(root: &Path) -> PathBuf {
    root.join(builtin_kind::CLAUDE_ROOT).join("settings.json")
}

/// Project only the `SessionStart` hook into `.claude/settings.json` — the no-path's
/// whole write, and the only writer that merges this file. No guard, no note, no
/// modeline: those bind only paths a lock declares emit-owned, and an unrepresented
/// project has no lock.
///
/// **Never reached on a represented harness.** There the settings document is a
/// projection the program owns whole and this splice would be install re-adding a gate
/// hook to it, which 0073 forbids outright. [`run`] executes the fork it is handed, so the
/// one decider is the dispatch that answers it (`src/main.rs`) — and it answers on
/// [`represented_by`], the same reading [`gate_installed`] takes, so neither face can send
/// a represented harness down this path.
fn place_settings_only(root: &Path, dry_run: bool) -> miette::Result<Vec<InstallEntry>> {
    let settings_path = settings_path(root);
    let existing = read_optional(&settings_path)?;
    let settings = project_settings(&settings_path, existing.as_deref())?;
    drift::place(&settings_path, &settings.desired, None, dry_run)?;
    Ok(vec![InstallEntry {
        placement: Placement::SessionStart,
        outcome: placement_outcome(settings.hook_present),
        path: settings_path,
    }])
}

/// What `.claude/settings.json` says about one of [`GATE_HOOKS`] right now. Both readings
/// come off the same document, and the split between them is the whole missing/stale
/// distinction.
#[derive(Debug, Clone, Copy)]
struct GateHookState {
    /// Some group under this hook's event runs its command — the gate is wired here.
    wired: bool,
    /// Some group is seated at this hook's `(event, matcher)` address, whatever it runs.
    seated: bool,
}

/// Every one of [`GATE_HOOKS`] read off `.claude/settings.json`, in `GATE_HOOKS` order —
/// straight off the file, which on the represented path is `emit`'s projection of the gate
/// hook members. An absent or empty document wires and seats none; an unparseable one is
/// an [`InstallError`], never a silent "none".
///
/// The projection is the *only* place the seated reading can come from: a lock row's
/// `fields` are seam-inbound and dropped, and 0074 keys a hook registration on the bare
/// event, so the matcher — the other half of a gate hook member's identity — survives
/// nowhere else.
fn gate_hooks_state(path: &Path) -> Result<[GateHookState; GATE_HOOK_COUNT], InstallError> {
    const ABSENT: GateHookState = GateHookState {
        wired: false,
        seated: false,
    };
    let Some(text) = read_optional(path)? else {
        return Ok([ABSENT; GATE_HOOK_COUNT]);
    };
    if text.trim().is_empty() {
        return Ok([ABSENT; GATE_HOOK_COUNT]);
    }
    let root: JsonValue = serde_json::from_str(&text).map_err(|source| InstallError::Settings {
        path: path.to_path_buf(),
        source,
    })?;
    let object = root
        .as_object()
        .ok_or_else(|| InstallError::SettingsShape {
            path: path.to_path_buf(),
        })?;
    Ok(GATE_HOOKS.map(|hook| GateHookState {
        wired: event_has_group(object, hook.event, |group| {
            group_has_command(group, hook.command)
        }),
        seated: event_has_group(object, hook.event, |group| {
            group_binds_matcher(group, hook.matcher)
        }),
    }))
}

/// One gate hook's reported outcome, read off the projection rather than off a write
/// `install` performed — the represented path places nothing in this file.
///
/// - wired now, not before → [`Applied`](ApplyOutcome::Applied): this run wired it.
/// - wired now and before → [`Unchanged`](ApplyOutcome::Unchanged).
/// - not wired, a member seated at its `(event, matcher)` address →
///   [`SupersededByMember`](ApplyOutcome::SupersededByMember): the member is *stale* —
///   it holds temper's gate address and runs something else, which is exactly the shape a
///   respelled command leaves behind.
/// - not wired, nothing seated there → [`Conflicted`](ApplyOutcome::Conflicted): the
///   member is *missing*. Surfaced rather than clobbered — `install` scaffolds the gate
///   hook members with the rest of the lift and never edits an authored `harness.ts`
///   afterwards (0073).
///
/// The seated half is read off the projection, never off the lock: see [`gate_hooks_state`].
fn gate_outcome(before: bool, now: GateHookState) -> ApplyOutcome {
    match (now.wired, before, now.seated) {
        (true, true, _) => ApplyOutcome::Unchanged,
        (true, false, _) => ApplyOutcome::Applied,
        (false, _, true) => ApplyOutcome::SupersededByMember,
        (false, _, false) => ApplyOutcome::Conflicted,
    }
}

/// Report the three gate hooks and place each emit-owned target's managed-by note +
/// schema modeline — the represented project's whole placement set, grounded in the
/// caller's parsed `lock` ([`drift::emit_owned_targets_from_doc`]) rather than a raw
/// discovery walk.
///
/// The gate hooks are **read, never written**: on this path the manifest they key inside
/// is a represented manifest like any other the lock's registrations name, a projection
/// the program owns whole, and [`GATE_HOOKS`] reach it as the `hook` members
/// [`scaffold`] minted, so `emit` is its one writer. `gate_before` is what the file wired
/// before this run's emit, which is what splits [`ApplyOutcome::Applied`] from
/// [`ApplyOutcome::Unchanged`]; `None` — [`gate_installed`]'s read-only shadow, which
/// runs no emit at all — takes the current state as the before state, so a wired gate
/// reads `Unchanged`.
fn evaluate_placements(
    root: &Path,
    dry_run: bool,
    gate_before: Option<[bool; GATE_HOOK_COUNT]>,
    lock: &DocumentMut,
) -> miette::Result<Vec<InstallEntry>> {
    let targets = drift::emit_owned_targets_from_doc(lock);

    let mut entries = Vec::new();
    let settings_path = settings_path(root);
    let now = gate_hooks_state(&settings_path)?;
    let before = gate_before.unwrap_or_else(|| now.map(|state| state.wired));
    for (index, hook) in GATE_HOOKS.iter().enumerate() {
        entries.push(InstallEntry {
            placement: hook.placement,
            outcome: gate_outcome(before[index], now[index]),
            path: settings_path.clone(),
        });
    }

    // The note is converged first so the modeline stays the leading frontmatter line.
    for target in targets {
        // A lock row names its path against the harness root, so it is joined onto the
        // root this install was aimed at — never resolved against the ambient cwd.
        let path = root.join(&target.path);
        let source = fs::read_to_string(&path).map_err(|source| InstallError::Read {
            path: path.clone(),
            source,
        })?;
        let mut current = source;

        // Converge a stale note wording where the frontmatter `#` note already exists.
        // Never creates one: emit places both marker forms — the `#` note on a
        // frontmatter projection, the banner on a frontmatterless markdown one — so
        // install's whole job here is re-wording a retired placement.
        if let Some(desired) = converge_note_wording(&current) {
            let outcome = drift::place(&path, &desired, None, dry_run)?;
            entries.push(InstallEntry {
                placement: Placement::Note,
                outcome,
                path: path.clone(),
            });
            current = desired;
        }

        // Converge a stale banner wording if the banner already exists on a
        // frontmatterless markdown projection. Never creates a new banner — that is
        // emit's responsibility.
        if crate::placement::is_markdown_path(&path)
            && let Some(desired) = converge_banner_wording(&current)
        {
            let outcome = drift::place(&path, &desired, None, dry_run)?;
            entries.push(InstallEntry {
                placement: Placement::Note,
                outcome,
                path: path.clone(),
            });
            current = desired;
        }

        // Never point an editor at a `$schema` reference with nothing on the other
        // end: the modeline lands only once its schema artifact actually exists.
        if schema_artifact_exists(root, &target.kind)
            && let Some(desired) =
                project_modeline(&current, &schema_ref(root, &path, &target.kind))
        {
            entries.push(InstallEntry {
                placement: Placement::Modeline,
                outcome: drift::place(&path, &desired, None, dry_run)?,
                path,
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

/// Which edge of a tool call fired `temper guard`, and so which judge answers: the
/// pending write the payload names, or the tree the call already left.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardEdge {
    /// Before a file-writing tool — [`guard`] binds the `file_path` the payload names.
    PreToolUse,
    /// After a shell tool — [`shell_edge_findings`] judges the tree on disk, because
    /// the call's writes name no path the payload carries.
    PostToolUse,
}

/// The edge `payload` fired at, read off the `hook_event_name` member of the decoded
/// hook payload — the field naming which edge of the tool call this is, carried by every
/// event's envelope (code.claude.com/docs/en/hooks, retrieved 2026-07-17, the same
/// external fact `builtin_kind`'s tap classifier reads it as).
///
/// Only `PostToolUse` answers [`GuardEdge::PostToolUse`]. A payload naming no event, or
/// naming one the gate wires no row for, keeps the pending-write edge: that judge binds
/// nothing a payload does not name a `file_path` in, so reading it at the wrong edge says
/// nothing, where reading a *pre*-edge write as the post edge would drop the one binding
/// that can still deny it.
#[must_use]
pub fn guard_edge(payload: &JsonValue) -> GuardEdge {
    match payload.get("hook_event_name").and_then(JsonValue::as_str) {
        Some("PostToolUse") => GuardEdge::PostToolUse,
        _ => GuardEdge::PreToolUse,
    }
}

/// The finding `temper guard` surfaces at the **post** edge over the harness whose
/// surface workspace is `workspace_dir` — two halves of the tree the call left, each the
/// judge `check` already runs over that same fact:
///
/// - the **projection** half of the root member's `fresh` clause
///   ([`drift::config_stale_from_doc`]), re-hashing each lock row's `source_path` against
///   the `emit_hash` the lock recorded;
/// - the **locus** half of its `locus-declared` clause
///   ([`drift::undeclared_locus_members_from_doc`]), asking the lock's provenance rows
///   about every document sitting at a governed locus `loci` names.
///
/// [`None`] where both halves are silent — a pass prints nothing — and for each half
/// whose clause the root contract does not bind at all. One judge in one vocabulary per
/// half, so the boundary and the gate cannot disagree about either fact. Clauses are
/// located the way [`crate::compose::root_contract`]'s rows-or-default rule locates every
/// root clause, off the `declarations` already in hand — no walk — and the dial is not
/// consulted because it moves a clause's severity only, never its binding, and severity is
/// not what decides here: the *enforcement mode* does, exactly as it does for the
/// pending-write edge.
///
/// **One lock read.** Both judges take the parsed document, read once here through
/// [`drift::read_lock_document`], so a post-edge call parses one lock however many halves
/// speak.
///
/// The locus half is gated on the lock's **presence**, exactly as the pending-write edge
/// gates its own locus binding on having a declared target set: on an unrepresented
/// harness every document at every locus is undeclared, and naming them all would be
/// noise rather than a finding.
///
/// # Errors
///
/// Propagates the [`crate::compose::ClauseRowError`] a clause row outside the closed
/// vocabulary raises, and the lock read/parse error a corrupt lock raises — refused loud
/// here as everywhere.
pub fn shell_edge_findings(
    workspace_dir: &Path,
    declarations: &drift::Declarations,
    loci: &[GuardedLocus],
) -> miette::Result<Option<String>> {
    let root = crate::compose::root_contract(&declarations.clauses, &declarations.assembly)?;
    let clause = |predicate| root.clauses.iter().find(|c| c.predicate == predicate);
    let fresh = clause(crate::contract::Predicate::Fresh);
    let locus_declared = clause(crate::contract::Predicate::LocusDeclared);
    if fresh.is_none() && locus_declared.is_none() {
        return Ok(None);
    }

    let doc = drift::read_lock_document(workspace_dir)?;
    let drifted = fresh
        .map(|clause| drift::config_stale_from_doc(&doc, workspace_dir, clause))
        .unwrap_or_default();
    let lock_present = workspace_dir.join(crate::LOCK_FILENAME).is_file();
    let strays = locus_declared
        .filter(|_| lock_present)
        .map(|clause| {
            let sites = locus_member_sites(&drift::harness_root_of(workspace_dir), loci);
            drift::undeclared_locus_members_from_doc(&doc, &sites, clause).findings
        })
        .unwrap_or_default();

    Ok(render_shell_edge_report(&drifted, &strays))
}

/// Every document on disk at a locus `loci` names, as the site shape the locus judge asks
/// the lock's provenance rows about. The post edge's answer to having no path in its
/// payload: the pending-write edge tests one `file_path` against the patterns
/// ([`matches_governed_locus`]), so here each locus root is enumerated instead and every
/// file under it matched against the same [`crate::glob::compile_glob`] pattern — the post
/// edge binds exactly the paths the pre edge would have tested, and no others.
///
/// The walk's bound is pinned by [`locus_member_site_walk_count`]. That bound is why
/// `import`'s own locus scan is the wrong reuse — it reads an index built from a
/// whole-tree ignore-honoring walk — and why the caller's locus set carries no `.`-rooted
/// locus: the guard has no ignore reader, so a root-rooted glob would judge vendored
/// documents discovery prunes.
///
/// Sorted by path within each locus ([`walkdir::WalkDir::sort_by_file_name`]), so the
/// findings a call surfaces read in one order on every platform.
fn locus_member_sites(harness_root: &Path, loci: &[GuardedLocus]) -> Vec<drift::LocusMemberSite> {
    let mut sites = Vec::new();
    for locus in loci {
        let Some(matcher) = crate::glob::compile_glob(&locus.pattern) else {
            continue;
        };
        LOCUS_MEMBER_SITE_WALKS.with(|c| c.set(c.get() + 1));
        let walk = walkdir::WalkDir::new(harness_root.join(&locus.root))
            .min_depth(1)
            .sort_by_file_name();
        for entry in walk {
            // A locus directory that does not exist, or an entry that cannot be read, is
            // no site: absent evidence must never forge a finding.
            let Ok(entry) = entry else { continue };
            if !entry.file_type().is_file() {
                continue;
            }
            let Some(source_path) = harness_relative(&entry.path().to_string_lossy(), harness_root)
            else {
                continue;
            };
            if !matcher.is_match(&source_path) {
                continue;
            }
            sites.push(drift::LocusMemberSite {
                kind: locus.kind.clone(),
                source_path,
                custom: locus.custom,
            });
        }
    }
    sites
}

/// Render the post edge's two halves for the guard's in-band surface — each non-empty half
/// under its own preamble, blank-line separated, [`None`] when both are silent. The halves
/// are separate sections because the preamble is a *claim*: drift asserts a fingerprinted
/// projection moved, and a stray at a governed locus is not that — the lock never named it,
/// so there is no fingerprint to restore.
fn render_shell_edge_report(drifted: &[Diagnostic], strays: &[Diagnostic]) -> Option<String> {
    let sections: Vec<String> = [
        (GUARD_SHELL_EDGE_MESSAGE, drifted),
        (GUARD_SHELL_EDGE_LOCUS_MESSAGE, strays),
    ]
    .into_iter()
    .filter(|(_, findings)| !findings.is_empty())
    .map(|(preamble, findings)| render_guard_findings(preamble, findings))
    .collect();
    if sections.is_empty() {
        None
    } else {
        Some(sections.join("\n\n"))
    }
}

/// Render one stretch of the guard's in-band surface: its `preamble`, then one
/// `<rule>: <finding>` line per finding. The one encoder both edges speak through — a post
/// edge's half ([`render_shell_edge_report`]) and a manifest's findings under the header
/// they earn ([`render_manifest_findings`]) — so the guard's one surface reads one way
/// whichever edge speaks. The per-finding lines are the judge's own words, the same ones
/// `check` prints.
fn render_guard_findings(preamble: &str, findings: &[Diagnostic]) -> String {
    let mut out = String::from(preamble);
    for finding in findings {
        out.push_str(&format!("\n  {}: {}", finding.rule, finding.message));
    }
    out
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

/// One governed locus the `PreToolUse` guard binds a pending write against — a
/// represented **committed** kind's `governs` locus, spelled as the one `/`-separated
/// pattern `glob::compile_glob` compiles (`governs.root` joined to
/// `governs.glob` through [`drift::join_locus`], the same join `manifest_path` makes).
/// Assembled by the caller off the embedded kind set overlaid with the lock's
/// relocations, so the loci the guard binds are the loci discovery walks.
///
/// Not [`CustomKind::owns_source`]: that matches a glob's *leaf* against a bare
/// filename (`rule`'s `*.md` matches any markdown anywhere), which dispatches a
/// discovered file to its kind but cannot decide whether a path is *in* a locus.
pub struct GuardedLocus {
    /// The governing kind's bare name, as the finding reports it.
    pub kind: String,
    /// The kind's `governs` root alone — the directory the **post** edge enumerates from,
    /// where no payload field names a path to test ([`locus_member_sites`]). Carried beside
    /// `pattern` rather than re-derived off it: the literal prefix of a compiled glob is
    /// `globset`'s to know, not a substring this module should guess at.
    pub root: String,
    /// The locus pattern — the kind's `governs` root joined to its glob, `/`-separated.
    pub pattern: String,
    /// Whether the governing kind is a lock-declared *custom* kind — the one fact the
    /// locus judge's `local` remedy turns on ([`drift::LocusMemberSite::custom`]), carried
    /// so the post edge's findings are word-for-word the ones `check` prints.
    pub custom: bool,
}

/// `temper guard`'s decision over one pending write: the verdict it reached and the
/// finding that goes with it. The message is empty for [`GuardVerdict::Allow`] and for
/// [`GuardVerdict::Note`], which surfaces nothing in-band — it rides the next report.
/// Carried together because the two bindings speak different findings at the same
/// mode: a declared projection is drift ([`guard_message`]), a write inside a governed
/// locus is an undeclared member (`undeclared_locus_message`).
pub struct GuardDecision {
    /// What the guard decided — allow silently, defer out-of-band, surface in-band, deny.
    pub verdict: GuardVerdict,
    /// The finding text the surfacing verdicts print, empty when nothing surfaces.
    pub message: String,
}

/// Decide `temper guard`'s verdict over a decoded `PreToolUse` `payload` at `mode`'s
/// enforcement mode, bound to `targets` — the lock's emit-owned projection set
/// ([`drift::emit_owned_targets`]) — and to `loci`, the governed loci of the
/// represented committed kinds ([`GuardedLocus`]). `targets` is `None` for a harness
/// with no `lock.toml` at all (never emitted, or the file removed out from under an
/// already-installed hook): with no declared set to consult, the guard falls
/// back to binding any `.claude/` `file_path`, matching the pre-lock behavior —
/// absent evidence must never silently suppress the guard, and `loci` is not consulted
/// there because an unrepresented harness declares no member anywhere.
///
/// **Binding scope**: every surfacing verdict closes on [`binding_limit`], which names
/// the tools this guard's row binds and the writes that bypass it.
///
/// Two bindings under a lock, in order: the `file_path` names a declared projection (a
/// direct edit to an emit-owned file — drift), else it falls inside a governed locus
/// the lock declares no member at (a document `emit` will never maintain and Claude
/// Code loads anyway). Representation must not *loosen* the boundary: before the second
/// binding, creating `.claude/rules/stray.md` in a represented harness passed while the
/// no-lock fallback bound the very same write. A `file_path` matching neither (or, with
/// `targets` absent, naming no `.claude/` locus) is [`GuardVerdict::Allow`]. Otherwise
/// the finding maps onto the mode vocabulary, split by where it goes: `note` defers
/// it out-of-band ([`GuardVerdict::Note`]), `warn` surfaces it in-band
/// ([`GuardVerdict::Warn`]), `block` denies the call ([`GuardVerdict::Block`]).
#[must_use]
pub fn guard(
    payload: &JsonValue,
    mode: EnforcementMode,
    root: &Path,
    targets: Option<&[drift::EmitOwnedEntry]>,
    loci: &[GuardedLocus],
) -> GuardDecision {
    let allow = || GuardDecision {
        verdict: GuardVerdict::Allow,
        message: String::new(),
    };
    let Some(file_path) = extract_file_path(payload) else {
        return allow();
    };

    // When targets are declared, check the file_path against them, then against the
    // governed loci that declare no member for it.
    let message = if let Some(targets) = targets {
        if let Some(owner) = matched_projection(file_path, root, targets) {
            format!("{}{}", guard_message(), projection_owner_line(owner))
        } else if let Some(locus) = matches_governed_locus(file_path, root, loci) {
            undeclared_locus_message(&locus.kind, locus.custom)
        } else {
            return allow();
        }
    } else {
        // Fallback: with no lock, bind only `.claude/` paths (the documented no-lock
        // fallback behavior — absent evidence must never suppress the guard).
        if !is_claude_path(file_path) {
            return allow();
        }
        guard_message().to_string()
    };

    let verdict = match mode {
        EnforcementMode::Note => GuardVerdict::Note,
        EnforcementMode::Warn => GuardVerdict::Warn,
        EnforcementMode::Block => GuardVerdict::Block,
    };
    GuardDecision {
        // `note` records the finding out-of-band only — never into the live session — so
        // the message it would have spoken is dropped here rather than at the caller.
        message: if verdict == GuardVerdict::Note {
            String::new()
        } else {
            message
        },
        verdict,
    }
}

/// The `tool_input` object a hook payload carries the firing tool's own arguments in —
/// one home for the field name, read by both judges a pending write faces
/// ([`extract_file_path`] and [`manifest_write_findings`]), so neither can reach a
/// different reading of it (code.claude.com/docs/en/hooks, "PreToolUse input", retrieved
/// 2026-09-25).
fn tool_input(payload: &JsonValue) -> Option<&JsonValue> {
    payload.get("tool_input")
}

/// The `file_path` value a `PreToolUse` `payload` names, when present — the `file_path`
/// member of the decoded payload's [`tool_input`] (an external fact:
/// code.claude.com/docs/en/tools-reference, retrieved 2026-09-07).
///
/// Read structurally off the one decode, never scraped out of the payload's text: the
/// field read is scoped to the write's own arguments, so a write whose *content* merely
/// mentions `file_path` names nothing here — the conservatism a field-scoped grep was
/// reaching for, held strictly, and with a JSON-escaped path value arriving whole rather
/// than truncated at its first escape.
///
/// Extracts any path regardless of locus; the caller determines whether to bind it
/// (against targets or the `.claude/` fallback).
fn extract_file_path(payload: &JsonValue) -> Option<&str> {
    tool_input(payload)?
        .get("file_path")
        .and_then(JsonValue::as_str)
}

/// Whether `file_path` names a `.claude/`-rooted artifact — the fallback binding
/// when no lock-declared targets exist to consult.
fn is_claude_path(file_path: &str) -> bool {
    file_path.contains(&format!("{}/", builtin_kind::CLAUDE_ROOT))
}

/// Whether `file_path` names any candidate — an equality compare of `/`-normalized paths
/// (`PATH-SEP-NORMALIZE`), both sides spelled harness-relative. A `file_path` arriving
/// absolute (Claude Code's own convention) is relativized against `root` first; one landing
/// outside the root matches nothing, since no candidate can name it.
fn path_matches<'a>(
    file_path: &str,
    root: &Path,
    mut candidates: impl Iterator<Item = &'a Path>,
) -> bool {
    let Some(file_path_normalized) = harness_relative(file_path, root) else {
        return false;
    };
    candidates
        .any(|candidate| candidate.to_string_lossy().replace('\\', "/") == file_path_normalized)
}

/// `file_path` spelled the one way every path compare in this module spells both its
/// sides: relativized against `root` (Claude Code names an absolute path; the lock and a
/// locus pattern are harness-relative), `/`-separated (`PATH-SEP-NORMALIZE`), and with a
/// leading `./` erased — `./x` and `x` name one file, and the lock only ever spells the
/// latter. [`None`] for a path outside `root`, which no candidate and no locus can name.
fn harness_relative(file_path: &str, root: &Path) -> Option<String> {
    let relative = crate::path::relativize_against_root(file_path, root)?;
    Some(
        crate::path::normalize_path(Path::new(&relative))
            .to_string_lossy()
            .replace('\\', "/"),
    )
}

/// The first of `targets` `file_path` names, or `None` for a path no target names — an
/// equality compare against each row's `/`-normalized `source_path` (`PATH-SEP-NORMALIZE`),
/// tolerant of `file_path` arriving absolute (Claude Code's own convention) against a
/// workspace-relative lock row. The matched row, not a bare yes/no, because the refusal owes
/// the author the member that owns the bytes (`projection_owner_line`).
fn matched_projection<'a>(
    file_path: &str,
    root: &Path,
    targets: &'a [drift::EmitOwnedEntry],
) -> Option<&'a drift::EmitOwnedEntry> {
    targets
        .iter()
        .find(|target| path_matches(file_path, root, std::iter::once(target.path.as_path())))
}

/// The line a projection refusal appends naming the member that owns the bytes — a drift
/// finding names the member that owns the bytes, the side that moved, and the remedy
/// (`model/pipeline.md`, "Drift"), and [`guard_message`] alone names only the side and the
/// remedy. Rendered as one indented line under the header, the shape
/// [`render_guard_findings`] already gives a finding, so the guard's one surface reads one
/// way. The no-lock fallback appends nothing: with no declared set there is
/// no member to name.
fn projection_owner_line(owner: &drift::EmitOwnedEntry) -> String {
    format!(
        "\n  owner: the `{}` member `{}` owns `{}` — edit that member's source and re-emit",
        owner.kind,
        owner.name,
        owner.path.to_string_lossy().replace('\\', "/")
    )
}

/// The first of `loci` whose pattern `file_path` falls inside, or `None` for a path in
/// no governed locus. Spelled harness-relative on both sides, exactly as
/// [`path_matches`] spells its equality compare: a `file_path` arriving absolute
/// (Claude Code's own convention) is relativized against `root` first, and one landing
/// outside the root is in no locus, since every locus is rooted in the harness.
///
/// The match itself is [`crate::glob::compile_glob`] — `literal_separator(true)`, so
/// `*`/`?` stay inside one segment and `**` crosses them. That is exactly
/// `import::scan_locus`'s per-segment walk semantics reached in one compare, so the
/// guard binds the paths discovery would have walked and no others.
fn matches_governed_locus<'a>(
    file_path: &str,
    root: &Path,
    loci: &'a [GuardedLocus],
) -> Option<&'a GuardedLocus> {
    let normalized = harness_relative(file_path, root)?;
    loci.iter().find(|locus| {
        crate::glob::compile_glob(&locus.pattern)
            .is_some_and(|matcher| matcher.is_match(&normalized))
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
    /// The collection address the members key at (`mcpServers.*`, `hooks.<Event>`,
    /// `enabledPlugins.*`).
    pub address: CollectionAddress,
    /// The member keys the lock declares at this collection address — expected to be
    /// present in a write; a pending write omitting any of these is flagged at the
    /// declared enforcement mode.
    pub expected_keys: Vec<String>,
    /// The **container kind** whose member projects this manifest whole, when the lock
    /// declares one — the same structural fact `emit` reads to decide whether a rollup row
    /// lands for the manifest's path (`drift`'s member loop). A container member makes every
    /// byte of the file emit's, so the part-authored file the co-owned reading assumes cannot
    /// arise: a pending hand write earns the projection refusal, not a contract check
    /// ([`manifest_write_findings`]). `None` for a manifest no member projects — `.mcp.json`
    /// composed from registrations alone — which stays co-owned.
    pub container: Option<String>,
}

/// One string replacement a pending `Edit`/`MultiEdit` payload asks for: the exact text to
/// find, its replacement, and whether every occurrence is replaced rather than the single
/// unambiguous one (`code.claude.com/docs/en/tools-reference`, retrieved 2026-09-07).
struct PendingEdit {
    /// The exact text the edit finds — matched byte-for-byte, never trimmed or re-indented.
    old: String,
    /// The text it is replaced with.
    new: String,
    /// Replace every occurrence rather than requiring exactly one.
    replace_all: bool,
}

/// The full manifest text a pending write would land, as far as the guard can reconstruct it.
enum PendingManifest {
    /// The whole file the write would leave on disk — a `Write`'s `content`, or the on-disk
    /// file with the payload's edits applied.
    Text(String),
    /// The payload carries no write shape this guard understands (neither whole-file
    /// `content` nor any edit strings), so there is nothing to check.
    Unrecognized,
    /// The payload is an edit the guard cannot honestly apply — the file is unreadable, or
    /// its text does not carry the edit's `old_string` exactly once.
    Unreconstructable,
}

/// The rule id the [`PendingManifest::Unreconstructable`] denial carries — distinct from a
/// member's contract violation, since nothing was checked: the guard could not build the
/// manifest the write would land.
const GUARD_MANIFEST_EDIT_RULE: &str = "guard.manifest-edit-unreconstructable";

/// The rule id the unparseable-manifest denial carries. Distinct from
/// [`GUARD_MANIFEST_EDIT_RULE`], which covers a payload the guard could not reconstruct at
/// all: here the bytes *were* reconstructed and are decidably not a manifest.
const GUARD_MANIFEST_UNPARSEABLE_RULE: &str = "guard.manifest-unparseable";

/// The full manifest text `input`'s pending write would leave on disk. A `Write` carries it
/// outright as `content`; an `Edit`/`MultiEdit` carries only replacement strings
/// (`old_string`/`new_string`, or an `edits` array of them), so the on-disk file at
/// `file_path` is read and the edits applied in sequence — the same whole manifest either
/// shape would produce, so one rule judges both.
///
/// Field names per `code.claude.com/docs/en/tools-reference` (retrieved 2026-09-07); that
/// reference documents `Write` and `Edit` only — `MultiEdit`'s `edits` array of the same
/// per-edit fields is UNVERIFIED, kept because the guard's own hook matcher still binds the
/// tool. A misread there costs a denial the author resolves with `Write`, never a silent pass.
fn pending_manifest(input: &JsonValue, file_path: &str, root: &Path) -> PendingManifest {
    if let Some(content) = input.get("content").and_then(JsonValue::as_str) {
        return PendingManifest::Text(content.to_string());
    }

    let edits = match input.get("edits") {
        Some(value) => {
            let Some(array) = value.as_array() else {
                return PendingManifest::Unreconstructable;
            };
            let mut edits = Vec::with_capacity(array.len());
            for entry in array {
                let Some(edit) = read_edit(entry) else {
                    return PendingManifest::Unreconstructable;
                };
                edits.push(edit);
            }
            edits
        }
        None => match read_edit(input) {
            Some(edit) => vec![edit],
            // A payload carrying one edit string but not its partner is an edit that cannot
            // be applied, not a shape the guard fails to recognize — only a payload naming
            // neither falls through to the caller's projection binding.
            None if input.get("old_string").is_some() || input.get("new_string").is_some() => {
                return PendingManifest::Unreconstructable;
            }
            None => return PendingManifest::Unrecognized,
        },
    };

    let Some(relative) = crate::path::relativize_against_root(file_path, root) else {
        return PendingManifest::Unreconstructable;
    };
    let Ok(mut text) = fs::read_to_string(root.join(relative)) else {
        return PendingManifest::Unreconstructable;
    };
    for edit in &edits {
        let Some(edited) = apply_edit(&text, edit) else {
            return PendingManifest::Unreconstructable;
        };
        text = edited;
    }
    PendingManifest::Text(text)
}

/// One [`PendingEdit`] off an object carrying the edit fields — the whole `tool_input` of an
/// `Edit`, or one entry of a `MultiEdit`'s `edits`. [`None`] when the object carries no edit
/// strings at all.
fn read_edit(value: &JsonValue) -> Option<PendingEdit> {
    Some(PendingEdit {
        old: value
            .get("old_string")
            .and_then(JsonValue::as_str)?
            .to_string(),
        new: value
            .get("new_string")
            .and_then(JsonValue::as_str)?
            .to_string(),
        replace_all: value
            .get("replace_all")
            .and_then(JsonValue::as_bool)
            .unwrap_or(false),
    })
}

/// `text` with `edit` applied, or [`None`] when the edit cannot be honestly applied: an empty
/// or absent `old`, or — without `replace_all` — one occurring more than once, the ambiguity
/// Claude Code itself refuses to resolve. Reconstructing a manifest from a guess is worse
/// than declining to check it.
fn apply_edit(text: &str, edit: &PendingEdit) -> Option<String> {
    if edit.old.is_empty() {
        return None;
    }
    if edit.replace_all {
        return text
            .contains(&edit.old)
            .then(|| text.replace(&edit.old, &edit.new));
    }
    let mut hits = text.match_indices(&edit.old);
    let (at, _) = hits.next()?;
    if hits.next().is_some() {
        return None;
    }
    let mut out = String::with_capacity(text.len() + edit.new.len());
    out.push_str(&text[..at]);
    out.push_str(&edit.new);
    out.push_str(&text[at + edit.old.len()..]);
    Some(out)
}

/// The finding a pending edit to `manifest` earns when the guard cannot reconstruct the
/// manifest it would land — naming the members left unchecked and the write shape that can
/// be checked, never the blanket projection wording a co-owned manifest has no business
/// hearing.
fn unreconstructable_edit_finding(manifest: &GuardedManifest) -> Diagnostic {
    let governed = if manifest.expected_keys.is_empty() {
        format!(
            "the members it governs at `{}`",
            manifest.address.key_path.wire_label()
        )
    } else {
        format!(
            "its lock-declared members `{}` at `{}`",
            manifest.expected_keys.join("`, `"),
            manifest.address.key_path.wire_label()
        )
    };
    let path = manifest.path.to_string_lossy().replace('\\', "/");
    Diagnostic::error(
        GUARD_MANIFEST_EDIT_RULE,
        path.clone(),
        format!(
            "cannot reconstruct `{path}` from this edit — the file is unreadable, or its text does not carry the edited string exactly once — so {} went unchecked; re-issue the change as a whole-file `Write`.",
            governed
        ),
    )
}

/// The finding a pending write earns when the manifest it would land was reconstructed in
/// full and does not parse — naming the manifest path and the parse fault, so the author
/// reads what is wrong with the bytes rather than which member went unchecked (none did:
/// there are no members to read out of a document that is not a manifest).
fn unparseable_manifest_finding(
    manifest: &GuardedManifest,
    error: &json_manifest::JsonManifestError,
) -> Diagnostic {
    // `Manifest::parse` raises `Malformed` and nothing else — its `detail` is the parse
    // fault alone, without the path the message already names. Any other variant would
    // arrive from a future parse failure, so it renders whole rather than being dropped.
    let detail = match error {
        json_manifest::JsonManifestError::Malformed { detail, .. } => detail.clone(),
        other => other.to_string(),
    };
    let path = manifest.path.to_string_lossy().replace('\\', "/");
    Diagnostic::error(
        GUARD_MANIFEST_UNPARSEABLE_RULE,
        path.clone(),
        format!("`{path}` would not parse as a manifest after this write: {detail}"),
    )
}

/// Check a pending `PreToolUse` write against every represented manifest's contract —
/// entry 4 of the manifest write side, extending the `.claude/`-projection binding
/// ([`guard`]) to the manifest members the write face now governs.
///
/// Returns `None` when the write targets no represented manifest — a non-manifest path, a
/// payload carrying no write shape at all, or a manifest a container member projects whole
/// ([`GuardedManifest::container`]) — and the caller falls back to the projection-drift
/// binding. That last case is the one the co-ownership reading does not reach: with a container
/// member the file is a whole projection, so it carries no residue a hand write could
/// legitimately touch, and passing such a write here would contradict the
/// projection-drift verdict `check`'s root `fresh` clause gives the very same bytes.
/// Returns `Some(findings)` when the write does
/// target a co-owned manifest: `findings` is empty for a conforming one (a write touching only
/// opaque residue, or members that all pass), or the error-severity findings its members trip,
/// to be surfaced at the
/// author's declared enforcement mode. `Write` and `Edit`/`MultiEdit` are judged by one rule —
/// the manifest the write would land ([`pending_manifest`]) — and an edit the guard cannot
/// reconstruct earns its own finding rather than falling through to the projection binding,
/// whose wording denies a co-owned manifest wholesale. A write whose reconstructed bytes are
/// not a manifest at all earns a third finding ([`GUARD_MANIFEST_UNPARSEABLE_RULE`]): that
/// one cannot be deferred to a later placement, since a harness carrying an unparseable
/// manifest never loads.
#[must_use]
pub fn manifest_write_findings(
    payload: &JsonValue,
    root: &Path,
    manifests: &[GuardedManifest],
) -> Option<Vec<Diagnostic>> {
    let input = tool_input(payload)?;
    let file_path = extract_file_path(payload)?;

    let matched: Vec<&GuardedManifest> = manifests
        .iter()
        .filter(|manifest| path_matches(file_path, root, std::iter::once(manifest.path.as_path())))
        .collect();
    if matched.is_empty() {
        return None;
    }

    // A matched manifest a container member projects whole is not co-owned: every byte is
    // emit's, so invariant 7's part-authored file cannot arise and there is no residue a hand
    // write may touch. Hand it back to the projection binding rather than passing it — the
    // same discriminator `emit` uses to decide whether a rollup row lands for this path.
    // Decided before the content is resolved, so the fall-through never costs a disk read.
    if matched.iter().any(|manifest| manifest.container.is_some()) {
        return None;
    }

    // Resolved once the path is known to name a manifest, so no unrelated write ever costs
    // a disk read.
    let content = match pending_manifest(input, file_path, root) {
        PendingManifest::Text(text) => text,
        PendingManifest::Unrecognized => return None,
        PendingManifest::Unreconstructable => {
            return Some(
                matched
                    .into_iter()
                    .map(unreconstructable_edit_finding)
                    .collect(),
            );
        }
    };

    let mut findings = Vec::new();
    // One finding per manifest path, not per collection address: the three addresses that
    // share `.claude/settings.json` all fail the same parse and would otherwise say so
    // three times.
    let mut reported_unparseable: std::collections::BTreeSet<&Path> =
        std::collections::BTreeSet::new();
    for manifest in matched {
        // A pending write whose reconstructed bytes are not a manifest is refused here
        // rather than deferred: the deferral assumed a later placement would catch it, and
        // for this file class none can — a harness carrying an unparseable manifest cannot
        // load, so `check` aborts before any reporter runs and the `SessionStart` hook that
        // would have carried the verdict is itself declared in the unparseable file.
        let parsed =
            match json_manifest::Manifest::parse(&manifest.path, &content, &[&manifest.address]) {
                Ok(parsed) => parsed,
                Err(error) => {
                    if reported_unparseable.insert(manifest.path.as_path()) {
                        findings.push(unparseable_manifest_finding(manifest, &error));
                    }
                    continue;
                }
            };

        // Check that all lock-declared members are present in the pending write.
        let present_keys: std::collections::BTreeSet<_> =
            parsed.members.iter().map(|m| &m.key).collect();
        for expected_key in &manifest.expected_keys {
            if !present_keys.contains(expected_key) {
                findings.push(Diagnostic::error(
                    "guard.manifest-dropped-member",
                    expected_key,
                    format!(
                        "lock declares member `{}` at `{}` but write omits it",
                        expected_key,
                        manifest.address.key_path.wire_label()
                    ),
                ));
            }
        }

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
    Some(findings)
}

/// Render a represented manifest's findings for the guard's in-band surface: the header its
/// findings earn — [`guard_manifest_unparseable_message`] for a write that would leave the
/// manifest unparseable, [`guard_manifest_edit_message`] for an edit that could not be
/// reconstructed and so checked nothing, [`guard_manifest_message`] for a member that broke
/// its contract — then [`render_guard_findings`] under it. The two "nothing was checked"
/// headers outrank the contract wording, which would misname the fault; picking between
/// them is the whole job this half has that the post edge's does not.
#[must_use]
pub fn render_manifest_findings(findings: &[Diagnostic]) -> String {
    let carries = |rule: &str| findings.iter().any(|finding| finding.rule == rule);
    let preamble = if carries(GUARD_MANIFEST_UNPARSEABLE_RULE) {
        guard_manifest_unparseable_message()
    } else if carries(GUARD_MANIFEST_EDIT_RULE) {
        guard_manifest_edit_message()
    } else {
        guard_manifest_message()
    };
    render_guard_findings(preamble, findings)
}

/// Map "was this placement already in its desired state" onto the settings outcomes for
/// the unrepresented path's merge ([`place_settings_only`]). That file carries no
/// baseline fingerprint (idempotent placement), so its placement is only ever
/// [`Applied`](ApplyOutcome::Applied) (absent/drifted) or
/// [`Unchanged`](ApplyOutcome::Unchanged) — never `Conflicted`. The represented path
/// reads its gate off the projection instead ([`gate_outcome`]).
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

/// The desired `.claude/settings.json` plus whether temper's `SessionStart` hook was
/// already in its desired state before the merge — the **unrepresented** path's whole
/// projection ([`place_settings_only`]). A represented harness merges nothing here: its
/// three gate hooks are program members ([`GATE_HOOKS`]) and `emit` is the file's one
/// writer.
struct SettingsProjection {
    /// The re-emitted settings JSON (canonical pretty, trailing newline).
    desired: String,
    /// Whether the `SessionStart` hook was already present.
    hook_present: bool,
}

/// Project the desired `.claude/settings.json` — the existing settings with the
/// `SessionStart` hook merged in, or a fresh document when the file is absent or empty.
/// Idempotent: an already-present temper hook at its desired shape is left alone, so
/// re-merging reproduces the bytes.
///
/// Format-preserving: an existing document is never re-serialized. Only the hook group's
/// own bytes change — every other key, its order, and the file's formatting survive
/// (decision 0008, the JSON peer of the `toml_edit` keystone). That guarantee is why
/// this merge is the *unrepresented* path's alone: there the file is the human's and
/// temper is a guest in it, where a represented harness projects it whole from the
/// program.
fn project_settings(
    path: &Path,
    existing: Option<&str>,
) -> Result<SettingsProjection, InstallError> {
    match existing {
        Some(text) if !text.trim().is_empty() => merge_settings(path, text),
        _ => fresh_settings(path),
    }
}

/// A fresh canonical `.claude/settings.json` — there is no existing document to
/// preserve, so a plain pretty re-serialize is exactly the right shape.
fn fresh_settings(path: &Path) -> Result<SettingsProjection, InstallError> {
    let hooks_key = CollectionKeyPath::HooksEvent.collection_key();
    let root = json!({ hooks_key: { "SessionStart": [session_start_group()] } });
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
    })
}

/// Splice the `SessionStart` group into an existing, non-empty `.claude/settings.json`
/// document without re-serializing it. An already-present, already-correct hook is left
/// untouched (so a no-op merge returns `text` byte-identical), and a human's own
/// `SessionStart` groups are never modified — temper only ever adds its own.
fn merge_settings(path: &Path, text: &str) -> Result<SettingsProjection, InstallError> {
    let root: JsonValue = serde_json::from_str(text).map_err(|source| InstallError::Settings {
        path: path.to_path_buf(),
        source,
    })?;
    let object = root
        .as_object()
        .ok_or_else(|| InstallError::SettingsShape {
            path: path.to_path_buf(),
        })?;

    let hook_present = event_has_group(object, "SessionStart", |group| {
        group_has_command(group, SESSION_START_COMMAND)
    });
    if hook_present {
        return Ok(SettingsProjection {
            desired: text.to_string(),
            hook_present,
        });
    }

    let root_start =
        text.find(|c: char| !c.is_whitespace())
            .ok_or_else(|| InstallError::SettingsShape {
                path: path.to_path_buf(),
            })?;
    let root_shape = json_splice::object_shape(text, root_start);

    let hooks_key = CollectionKeyPath::HooksEvent.collection_key();
    let edit = match root_shape.members.iter().find(|m| m.key == hooks_key) {
        Some(hooks_member) => {
            let hooks_shape = json_splice::object_shape(text, hooks_member.value_span.0);
            match hooks_shape.members.iter().find(|m| m.key == "SessionStart") {
                Some(member) => {
                    let array = json_splice::array_shape(text, member.value_span.0);
                    json_splice::append_element(&array, &session_start_group(), 3)
                }
                None => json_splice::insert_member(
                    &hooks_shape,
                    "SessionStart",
                    &json!([session_start_group()]),
                    2,
                ),
            }
        }
        None => json_splice::insert_member(
            &root_shape,
            hooks_key,
            &json!({ "SessionStart": [session_start_group()] }),
            1,
        ),
    };

    Ok(SettingsProjection {
        desired: json_splice::apply_edits(text, vec![edit]),
        hook_present,
    })
}

/// The `SessionStart` hook group temper installs: the exec-form command alone.
/// Shape: `{hooks: [{type, command}]}` (`code.claude.com/docs/en/hooks`, retrieved 2026-07-24);
/// `matcher` is optional for `SessionStart` (fires on every session start).
pub(crate) fn session_start_group() -> JsonValue {
    json!({ HOOK_HANDLER_KEY: [ { "type": "command", "command": SESSION_START_COMMAND } ] })
}

/// Whether some group registered under `event` satisfies `test` — the one walk over a
/// lifecycle event's matcher groups, and so the one home for the `hooks.<Event>` shape
/// (`code.claude.com/docs/en/hooks`, retrieved 2026-07-24).
///
/// Two questions ride it, and the gate needs both:
///
/// - [`group_has_command`] — *is temper's gate wired here*. The unrepresented merge asks
///   it of the human's document (its idempotence check), and [`gate_hooks_state`] asks it
///   of the projected one about each of [`GATE_HOOKS`].
/// - [`group_binds_matcher`] — *is a member seated at this gate hook's address at all*,
///   which is the missing/stale split ([`gate_outcome`]).
fn event_has_group(
    object: &serde_json::Map<String, JsonValue>,
    event: &str,
    test: impl Fn(&JsonValue) -> bool,
) -> bool {
    object
        .get(CollectionKeyPath::HooksEvent.collection_key())
        .and_then(|hooks| hooks.get(event))
        .and_then(JsonValue::as_array)
        .is_some_and(|groups| groups.iter().any(test))
}

/// Whether a hook group binds exactly `matcher` — `None` matching a group carrying no
/// `matcher` key, the shape a group on an unconditional event takes
/// ([`json_manifest::hook_matcher_group`] writes the field only where the member carries
/// it). A group's identity is the `(event, matcher)` pair (0074), so with the event already
/// fixed by [`event_has_group`] this is the rest of a gate hook's address.
fn group_binds_matcher(group: &JsonValue, matcher: Option<&str>) -> bool {
    group.is_object() && group.get(HOOK_MATCHER_KEY).and_then(JsonValue::as_str) == matcher
}

/// Whether a hook group carries `command` verbatim on one of its handlers. A differing
/// command reads `false`, since a hook that runs something else is not temper's gate.
fn group_has_command(group: &JsonValue, command: &str) -> bool {
    group
        .get(HOOK_HANDLER_KEY)
        .and_then(JsonValue::as_array)
        .is_some_and(|hooks| {
            hooks.iter().any(|hook| {
                hook.get("command")
                    .and_then(JsonValue::as_str)
                    .is_some_and(|found| found == command)
            })
        })
}

// ---------------------------------------------------------------------------
// the lift — scaffold the SDK program from the discovery report
// ---------------------------------------------------------------------------

/// The scaffold subdirectory a bare kind's member modules live under. Every
/// kind gets its own bucket — a shared directory would let two kinds' same-named
/// members overwrite each other on `write_creating_parents` — so an
/// unrecognized kind falls through to its own bare name rather than a shared
/// catch-all.
fn member_dir(kind: &str) -> String {
    match kind {
        "skill" => "skills".to_string(),
        "rule" => "rules".to_string(),
        "memory" => "memory".to_string(),
        "hook" => "hooks".to_string(),
        other => other.to_string(),
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

/// One discovered member read for the lift, normalized across the read adapters so the
/// scaffold writes one module shape whatever grammar the source was authored in — the
/// same one adapter dispatch the check side's file read takes, narrowed to the file
/// formats the lift converts, plus the manifest face a registration member is read
/// through ([`lifted_registration`]).
struct LiftedMember {
    /// The member id — its `name` property, and the module's file stem where that stem
    /// is free ([`module_stems`]).
    ///
    /// For a registration member this is the **collection key**, never the member's own
    /// name: a hook's name joins its matcher, and the SDK's `hook()` composes that name
    /// from the `name` property and the `matcher` field the module already spells. Both
    /// facts this id serves want the key. The `name` property wants it because the SDK
    /// composes the rest; the file stem wants it because `:` and `|` are reserved
    /// characters in a Windows file name
    /// (learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file, retrieved
    /// 2026-09-29) and the scaffold is committed — so a stem off the joined name would
    /// have every adopter's emit refused at [`is_portable_file_stem`], over a name the
    /// SDK composes anyway.
    id: String,
    /// The fields to hoist into typed properties, in projection order.
    fields: Vec<(String, JsonValue)>,
    /// The prose body, or [`None`] for a whole-document format ([`kind::Format::JsonDocument`],
    /// [`kind::Format::TomlDocument`]): such a member is its fields, with no body slot to
    /// move module-side.
    body: Option<String>,
}

/// Read one discovered artifact under the format its kind declares — a `json-document` or
/// `toml-document` kind's whole artifact through that grammar's adapter, every other file
/// kind through the frontmatter adapter. Reading a JSON document as frontmatter instead
/// would find no fields and hand the whole document back as a prose body, which no emit
/// has a home for.
///
/// # Errors
/// Returns a [`miette::Report`] if the source cannot be read or does not parse under its
/// declared format.
fn read_lifted_member(kind: &CustomKind, file: &Path) -> miette::Result<LiftedMember> {
    match kind.format {
        Some(kind::Format::JsonDocument) => {
            let document = json_manifest::DocumentMember::read(kind, file)?;
            Ok(LiftedMember {
                id: document.id,
                fields: document.fields.into_iter().collect(),
                body: None,
            })
        }
        Some(kind::Format::TomlDocument) => {
            let document = toml_document::read(kind, file)?;
            Ok(LiftedMember {
                id: document.id,
                fields: document.fields.into_iter().collect(),
                body: None,
            })
        }
        Some(kind::Format::YamlFrontmatter) | None => {
            let member = frontmatter::Member::from_source(kind, file)?;
            Ok(LiftedMember {
                id: member.id,
                fields: member.fields,
                body: Some(member.body),
            })
        }
    }
}

/// One registration entry read for the lift: its collection key is the member's id, its
/// own fields are the whole member, and it carries no body — a fields-shape kind has no
/// prose slot to move module-side ([`kind::Content::Fields`]).
///
/// What the lift writes is a module its author then owns, so the fields are spelled on the
/// **authoring surface** — a module carrying a field the shipped kind does not type is one
/// an adopter's own `tsc` refuses. For a group-array entry that surface types the group's
/// own field beside the `hooks` array it carries its handlers in (`Hook`,
/// `sdk/src/builtins.ts`), which is exactly what the read gives back — the group-array read
/// keeps that array as one of the group's own fields — so the fields pass through as read,
/// at every handler count, and the group re-nests to the bytes it was read from
/// ([`json_manifest::hook_matcher_group`]).
///
/// The group's handlers are not lifted as members of their own: a handler is an embedded
/// member with no module of its own to be written to, composed inside its host group's
/// `hooks` array and keyed by its position there (0075).
fn lifted_registration(entry: &json_manifest::RegistrationMember) -> LiftedMember {
    LiftedMember {
        id: entry.key.clone(),
        fields: entry.fields.clone().into_iter().collect(),
        body: None,
    }
}

/// Every collection key a registration kind claims, keyed by the host manifest it claims
/// it in. A claimed key is that kind's members' to carry, so the **container** member
/// owning the file must not lift it as one of its own fields: the check side already
/// keeps a claimed key out of a manifest's opaque residue
/// ([`json_manifest::Manifest::parse`]), and a container that lifted one anyway would
/// have emit write that segment twice — once from the registration members and once from
/// the container's field — landing a manifest with the key repeated.
///
/// The claim follows the **address**, not the entries found at it: an empty `hooks: {}`
/// is still the `hook` kind's collection, and reading it as container residue would put
/// the two writers back in contention the moment an entry appears.
fn claimed_collection_keys(discovery: &DiscoveryReport) -> BTreeMap<&Path, Vec<&str>> {
    let mut claimed: BTreeMap<&Path, Vec<&str>> = BTreeMap::new();
    let kinds = builtin_kind::definitions();
    for (name, found) in &discovery.members {
        let KindMembers::Registrations { manifests, .. } = found else {
            continue;
        };
        let Some(collection) = kinds
            .get(name)
            .and_then(|kind| kind.collection_address.as_ref())
            .map(|address| address.key_path.collection_key())
        else {
            continue;
        };
        for manifest in manifests {
            claimed
                .entry(manifest.as_path())
                .or_default()
                .push(collection);
        }
    }
    claimed
}

/// The characters Windows reserves in a file name, the strictest of the platforms a
/// committed scaffold has to check out on
/// (learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file, retrieved 2026-09-29).
/// Both path separators are in the set, so this subsumes the readback judgment the host
/// platform used to make.
const RESERVED_FILE_NAME_CHARS: &[char] = &['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

/// The extensionless device names Windows reserves, from the same page. `COM1`-`COM9` and
/// `LPT1`-`LPT9` are judged by shape ([`is_reserved_device_name`]) rather than listed.
const RESERVED_DEVICE_NAMES: &[&str] = &["CON", "PRN", "AUX", "NUL"];

/// Whether `stem` can name a file on every platform temper ships a binary for
/// (`specs/distribution.md`), judged against the Windows reserved set — the strictest of
/// them, and a cited external fact rather than a list invented here.
///
/// The check is total on whatever host runs it, on purpose: the lift's output is
/// committed source, so a member module a unix author scaffolds and a Windows teammate
/// cannot check out is the same defect as one unwritable here, and [`run_represented`]'s
/// "no half-scaffolded state" only holds if the refusal lands before the first write.
fn is_portable_file_stem(stem: &str) -> bool {
    if stem.is_empty() {
        return false;
    }
    if stem
        .chars()
        .any(|ch| RESERVED_FILE_NAME_CHARS.contains(&ch) || ch.is_control())
    {
        return false;
    }
    // A trailing space or dot is silently stripped rather than refused, so the name the
    // author wrote and the file that appears are two different names.
    if stem.ends_with(' ') || stem.ends_with('.') {
        return false;
    }
    // A device name is reserved through any extension — `NUL.txt` is `NUL` — and the
    // stem reaches disk with `.ts` appended, so the head before the first dot is judged.
    !is_reserved_device_name(stem.split('.').next().unwrap_or(stem))
}

/// Whether `head` is one of Windows' reserved device names, case-insensitively.
fn is_reserved_device_name(head: &str) -> bool {
    let upper = head.to_ascii_uppercase();
    if RESERVED_DEVICE_NAMES.contains(&upper.as_str()) {
        return true;
    }
    ["COM", "LPT"].iter().any(|port| {
        upper
            .strip_prefix(port)
            .is_some_and(|slot| matches!(slot.as_bytes(), [b'1'..=b'9']))
    })
}

/// Allocate one module file stem per lifted member, in the order they will be written.
///
/// A member's stem is its own id. That is the whole story for a file kind, whose ids come
/// off disk — but two members can share one id: Claude Code nests several matcher groups
/// under one lifecycle event and each is its own member, and every `CLAUDE.md` on the
/// tree folds to the same `memory` id. This allocates a distinct **file name** — the id,
/// then the id with the next free ordinal. Deriving the stem from the member's own name
/// instead is ruled out by [`LiftedMember::id`]: the joined name is not a portable file
/// name. The module's `name` property stays the id either way.
///
/// [`GATE_HOOKS`]' stems are reserved before any of them, so temper's own gate modules
/// keep the fixed `hooks/<Event>.ts` names whatever the adopted project's `settings.json`
/// carries, and [`scaffold`]'s two writers never land on one path.
///
/// # Errors
/// Returns [`InstallError::MemberFileName`] for an id that is not a portable file name
/// ([`is_portable_file_stem`]). A registration key is read off a manifest — the one id the
/// lift does not get from the filesystem — so a key carrying a path separator would seat
/// the module somewhere other than that directory, or outside `.temper/` entirely.
fn module_stems(
    temper_dir: &Path,
    lifted: &[(String, LiftedMember)],
) -> Result<Vec<String>, InstallError> {
    let mut taken: std::collections::BTreeSet<(String, String)> = GATE_HOOKS
        .iter()
        .map(|hook| (member_dir(GATE_HOOK_KIND), hook.event.to_string()))
        .collect();
    let mut stems = Vec::with_capacity(lifted.len());
    for (kind, member) in lifted {
        let module_dir = member_dir(kind);
        let dir = temper_dir.join(&module_dir);
        // The judgment is every platform's, not the host's: the scaffold is committed
        // source, so a module this machine can write but a teammate's Windows checkout
        // cannot is the same defect as one unwritable here. An ordinal suffix appends
        // `-<digit>` — no reserved character, no trailing dot or space, and no device
        // name — so clearing the id clears every stem derived from it.
        if !is_portable_file_stem(&member.id) {
            return Err(InstallError::MemberFileName {
                kind: kind.clone(),
                id: member.id.clone(),
                dir,
            });
        }
        let mut stem = member.id.clone();
        let mut ordinal = 1usize;
        while !taken.insert((module_dir.clone(), stem.clone())) {
            ordinal += 1;
            stem = format!("{}-{ordinal}", member.id);
        }
        stems.push(stem);
    }
    Ok(stems)
}

/// Scaffold the SDK program from `discovery`'s findings — the lift's whole
/// output, a **whole conversion** (0016), never an intermediate state: a member
/// module per discovered member hoisting every present field into a typed
/// property ([`member_module_source`]) and moving its prose module-side, plus
/// a `harness.ts` skeleton importing them all. Writes nothing
/// under `dry_run`, returning only the count a real run would scaffold.
///
/// The conversion covers a manifest's **registration** segments as well as the container
/// member holding them: a `settings.json` hook group and an `enabledPlugins` entry each
/// reach their own module, so the container's first emit re-renders every segment it
/// arrived with rather than shedding the ones no member declares.
///
/// # Errors
/// Returns a [`miette::Report`] if a member's source cannot be parsed, an id cannot name
/// a module file ([`module_stems`]), or a scaffold file cannot be written.
fn scaffold(
    temper_dir: &Path,
    discovery: &DiscoveryReport,
    dry_run: bool,
) -> miette::Result<usize> {
    let kinds = builtin_kind::definitions();
    let claimed = claimed_collection_keys(discovery);

    let mut lifted: Vec<(String, LiftedMember)> = Vec::new();
    for (name, found) in &discovery.members {
        let Some(kind) = kinds.get(name) else {
            continue;
        };
        // A local-locus kind's document is per-machine and uncommitted: read in place at
        // check, never an `emit` input or target. The lift converts an artifact into a
        // committed member module whose artifact is a projection, and a local document is
        // neither — so it is counted in the report and converted into nothing.
        if kind.commitment == Some(kind::Commitment::Local) {
            continue;
        }
        match found {
            KindMembers::Files(files) => {
                // A layout kind's document is a source, not a projection — its authored
                // home never moves, so the lift never converts it into a member module.
                if kind.content != kind::Content::File {
                    continue;
                }
                for file in files {
                    let mut member = read_lifted_member(kind, file)?;
                    if let Some(claimed) = claimed.get(file.as_path()) {
                        member
                            .fields
                            .retain(|(key, _)| !claimed.contains(&key.as_str()));
                    }
                    lifted.push((name.clone(), member));
                }
            }
            KindMembers::Registrations { entries, .. } => {
                for entry in entries {
                    lifted.push((name.clone(), lifted_registration(entry)));
                }
            }
        }
    }
    lifted.sort_by(|(a_kind, a), (b_kind, b)| (a_kind, &a.id).cmp(&(b_kind, &b.id)));

    // Allocated before the preview returns, so an id that cannot name a module file
    // refuses identically whether or not this run writes.
    let stems = module_stems(temper_dir, &lifted)?;

    if dry_run {
        return Ok(lifted.len());
    }

    let mut scaffolded = Vec::with_capacity(lifted.len());
    for ((kind, member), stem) in lifted.iter().zip(&stems) {
        let ident = member_ident(kind, stem);
        let dir = temper_dir.join(member_dir(kind));
        write_scaffold_file(
            &dir.join(format!("{stem}.ts")),
            &member_module_source(
                kind,
                &member.id,
                stem,
                &ident,
                &member.fields,
                member.body.as_deref(),
            ),
        )?;
        if let Some(body) = member.body.as_deref().filter(|body| !fits_inline(body)) {
            write_scaffold_file(&dir.join(format!("{stem}.md")), body)?;
        }
        scaffolded.push(ScaffoldedMember {
            ident,
            import_path: format!("./{}/{stem}.ts", member_dir(kind)),
        });
    }

    // temper's own gate rides the program like any other member: one `hook` module per
    // group in [`GATE_HOOKS`], composed into `harness.ts` below, so `emit` projects all
    // three into `.claude/settings.json`'s `hooks` collection and nothing splices that
    // file behind emit's back. The commands stay this module's constants and reach the
    // module as the SDK binding generated from them ([`GATE_COMMAND_BINDINGS`] →
    // `sdk/src/generated/gate-commands.ts`), never a literal: a respelling here follows
    // into every represented harness on its next `npm install`, and the seam gate
    // (`tests/it/seam_bindings_current.rs`) holds the SDK copy byte-equal to these bytes.
    //
    // They are not counted in the lift's total: the lift converts *discovered artifacts*,
    // and these are members temper authors.
    for hook in &GATE_HOOKS {
        let ident = member_ident(GATE_HOOK_KIND, hook.event);
        let module = gate_hook_module(hook);
        write_scaffold_file(
            &temper_dir.join(&module),
            &member_module_source(
                GATE_HOOK_KIND,
                hook.event,
                hook.event,
                &ident,
                &gate_hook_fields(hook),
                None,
            ),
        )?;
        scaffolded.push(ScaffoldedMember {
            ident,
            import_path: format!("./{module}"),
        });
    }

    write_scaffold_file(
        &temper_dir.join(HARNESS_ENTRY),
        &harness_entry_source(&scaffolded),
    )?;

    Ok(lifted.len())
}

/// The kind row label temper's own gate hooks scaffold under — the same `hook` kind any
/// authored `hooks.<Event>` member takes.
const GATE_HOOK_KIND: &str = "hook";

/// Where one gate hook's member module sits under the workspace dir — also, prefixed
/// `./`, the `harness.ts` import specifier that reaches it. One home for both, so the
/// remedy [`gate_hook_clause`] tells an author to write names the module [`scaffold`]
/// actually writes.
fn gate_hook_module(hook: &GateHook) -> String {
    format!("{}/{}.ts", member_dir(GATE_HOOK_KIND), hook.event)
}

/// The typed fields one gate hook's member module carries: its matcher where the event
/// binds one, then the single `command` handler it fires, inside the `hooks` array a
/// matcher group carries its handlers in (`code.claude.com/docs/en/hooks`, retrieved
/// 2026-09-29). The grain is the group's, matching the authoring surface the module is
/// written against (`Hook`, `sdk/src/builtins.ts`): a handler's own keys never sit flat
/// beside the group's. `name` is the event, already the module's identity property, so it
/// is not repeated here.
///
/// The command travels as its own bytes — [`json_to_ts_literal`] is what renders them as
/// the imported [`GATE_COMMAND_BINDINGS`] binding rather than a literal, wherever in the
/// value they sit. The matchers have no such binding and stay engine-only literals: they
/// are wiring facts about Claude Code's tool surface, not bytes a member runs.
fn gate_hook_fields(hook: &GateHook) -> Vec<(String, JsonValue)> {
    let mut fields = Vec::new();
    if let Some(matcher) = hook.matcher {
        fields.push((HOOK_MATCHER_KEY.to_string(), json!(matcher)));
    }
    fields.push((
        HOOK_HANDLER_KEY.to_string(),
        json!([{ "type": "command", "command": hook.command }]),
    ));
    fields
}

/// A member module's TS identifier: kind-prefixed so a skill and a rule sharing a
/// name never collide, non-alphanumeric bytes folded to `_` — in the kind prefix as
/// well as the name, since a hyphenated kind label (`settings-local`) is no more a legal
/// identifier than a hyphenated member id is.
fn member_ident(kind: &str, name: &str) -> String {
    let mut ident = fold_ident(kind);
    ident.push('_');
    ident.push_str(&fold_ident(name));
    ident
}

/// `label` with every non-alphanumeric byte folded to `_` — the one fold both halves of
/// a [`member_ident`] take.
fn fold_ident(label: &str) -> String {
    label
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect()
}

/// The SDK binding a kind's constructor is exported under: its hyphenated row label
/// camelCased, the spelling `sdk/src/claude-code.ts` exports (`settings-local` →
/// `settingsLocal`, `plugin-manifest` → `pluginManifest`). The row label is the engine's
/// vocabulary and the camelCase binding is the SDK's; interpolating the former into the
/// generated module would render `import { settings-local }`, which is not TS.
fn sdk_constructor(kind: &str) -> String {
    let mut out = String::new();
    for (index, segment) in kind.split('-').enumerate() {
        let mut chars = segment.chars();
        match chars.next() {
            Some(first) if index > 0 => {
                out.extend(first.to_uppercase());
                out.push_str(chars.as_str());
            }
            _ => out.push_str(segment),
        }
    }
    out
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
/// field but `name` (already the object literal's identity property) hoists
/// into its own typed TS property via [`json_to_ts_literal`], in the order
/// [`read_lifted_member`] carries them; `body`
/// moves module-side — inline as a `` text`…` `` literal ([`fits_inline`]) or,
/// for a document, a `file()` reference to the module-adjacent `<stem>.md`
/// [`scaffold`] writes beside this module. `name` is the member's identity and
/// `stem` its module file name, which part company only where two members share
/// one id ([`module_stems`]). Replaces the retired own-path lift:
/// the projected artifact is never this module's own `file()` source.
///
/// A `body` of [`None`] is a whole-document format's member ([`read_lifted_member`]): its
/// fields are the whole member, so the module carries no `prose:` property and imports
/// neither prose constructor. The kind reaches the module as the SDK binding its
/// constructor is exported under ([`sdk_constructor`]), never the row label — and so does
/// a field whose value is one of temper's own gate commands: the binding the renderer
/// spelled joins the import list beside the constructor, so the module names the command
/// and never copies it.
fn member_module_source(
    kind: &str,
    name: &str,
    stem: &str,
    ident: &str,
    fields: &[(String, JsonValue)],
    body: Option<&str>,
) -> String {
    let constructor = sdk_constructor(kind);
    let mut bindings = BTreeSet::new();
    let mut fields_src = String::new();
    for (key, value) in fields {
        if key == "name" {
            continue;
        }
        fields_src.push_str(&format!(
            "  {}: {},\n",
            ts_property_key(key),
            json_to_ts_literal(value, &mut bindings)
        ));
    }

    let (mut imports, prose_src) = match body {
        None => (constructor.clone(), String::new()),
        Some(body) if fits_inline(body) => (
            format!("file, text, {constructor}"),
            format!("  prose: {},\n", inline_prose_literal(body)),
        ),
        Some(_) => (
            format!("file, text, {constructor}"),
            format!("  prose: file(import.meta.url, \"./{stem}.md\"),\n"),
        ),
    };
    for binding in &bindings {
        imports.push_str(", ");
        imports.push_str(binding);
    }

    format!(
        "import {{ {imports} }} from \"@dtmd/temper/claude-code\";\n\nexport const {ident} = {constructor}({{\n  name: {name:?},\n{fields_src}{prose_src}}});\n"
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

/// Render a JSON frontmatter value as a TS literal, collecting into `bindings` every
/// gate-command binding the value names. JSON's grammar is a syntactic subset of TS/JS
/// object- and array-literal syntax, so a JSON leaf already reads as a TS literal — one
/// renderer generic over every JSON shape a scaffolded field carries (string, number,
/// bool, array, object), replacing the description-only special case the lift used to
/// carry.
///
/// Containers are walked rather than serialized whole for one reason: a string whose
/// bytes *are* one of temper's own gate commands renders as the binding that names them
/// ([`GATE_COMMAND_BINDINGS`]) instead of a second copy, and the command a gate hook
/// fires sits inside its handler array, not flat on the member. The rule is a property of
/// the bytes, so it holds at any depth and for any field — never a carve-out on the
/// gate-hook path. Escaping stays `serde_json`'s ([`json_leaf`]); only the container
/// punctuation is this function's, and it is JSON's compact form byte for byte.
fn json_to_ts_literal(value: &JsonValue, bindings: &mut BTreeSet<&'static str>) -> String {
    match value {
        JsonValue::String(text) => match gate_command_binding(text) {
            Some(binding) => {
                bindings.insert(binding);
                binding.to_owned()
            }
            None => json_leaf(value),
        },
        JsonValue::Array(items) => {
            let rendered: Vec<String> = items
                .iter()
                .map(|item| json_to_ts_literal(item, bindings))
                .collect();
            format!("[{}]", rendered.join(","))
        }
        JsonValue::Object(entries) => {
            let rendered: Vec<String> = entries
                .iter()
                .map(|(key, value)| {
                    format!(
                        "{}:{}",
                        json_leaf(&JsonValue::String(key.clone())),
                        json_to_ts_literal(value, bindings)
                    )
                })
                .collect();
            format!("{{{}}}", rendered.join(","))
        }
        _ => json_leaf(value),
    }
}

/// One JSON value serialized — the encoder [`json_to_ts_literal`] delegates every
/// non-container value and every object key to, so no escaping is ever hand-rolled here.
fn json_leaf(value: &JsonValue) -> String {
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
    crate::fs_util::write_creating_parents(path, contents.as_bytes()).map_err(|source| {
        InstallError::Write {
            path: path.to_path_buf(),
            source,
        }
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

/// Spawn `npm install` to fetch the `@dtmd/temper` dependency into
/// `.temper/node_modules`, assuming `.temper/package.json` already declares it.
/// Skipped when the dependency resolves via an ancestor `node_modules`.
fn spawn_npm_install(temper_dir: &Path) -> Result<(), InstallError> {
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
    let (rest, matter) = frontmatter::frontmatter_matter(source)?;
    if matter
        .lines()
        .any(|line| line.trim_start().starts_with(MODELINE_MARKER))
    {
        return Some(source.to_string());
    }
    let modeline = format!("{MODELINE_MARKER} $schema={schema_ref}");
    Some(format!("---\n{modeline}\n{rest}"))
}

/// Converge the managed-by note's wording if a frontmatter `source` already carries a
/// marked note, or `None` when there is nothing to converge — a source with no
/// frontmatter block, or one whose frontmatter carries no note. Never creates a note:
/// `emit` places it on every projection that renders frontmatter, and a file emit owns
/// in full is never part-installed.
///
/// **Content-drift-aware**, exactly like [`converge_banner_wording`]: idempotence keys on
/// the note's *bytes*, not the bare [`NOTE_MARKER`] prefix. A marked line whose body
/// still matches [`NOTE_COMMENT`] is returned verbatim (no churn); a marked line carrying
/// a retired wording is *re-placed*, splicing the current [`NOTE_COMMENT`] over the stale
/// line so a changed placement re-places instead of reporting `Unchanged`. Presence-only
/// keying let a stale note pass `gate_installed` forever.
///
/// Byte-faithful (round-trip discipline): the note line is the only rewritten bytes.
fn converge_note_wording(source: &str) -> Option<String> {
    let (_, matter) = frontmatter::frontmatter_matter(source)?;
    let existing = matter
        .lines()
        .find(|line| line.trim_start().starts_with(NOTE_MARKER))?;
    if existing == NOTE_COMMENT {
        return Some(source.to_string());
    }
    // Stale wording: splice the current note over the marked line, leaving every
    // other byte — the modeline, the other fields, the body — untouched. The
    // marker is distinctive, so the first occurrence is this note line.
    Some(source.replacen(existing, NOTE_COMMENT, 1))
}

/// Converge the banner wording if a frontmatterless markdown `source` already carries
/// a banner with stale wording, or return `None` if there is no banner to converge.
/// Never creates a new banner — that is emit's responsibility.
///
/// **Content-drift-aware**, exactly like the old `project_banner`: idempotence keys on
/// the banner's *bytes*, not the bare [`crate::placement::BANNER_MARKER`] prefix. A leading
/// banner whose line matches [`crate::placement::BANNER`] is returned verbatim (no churn); one
/// carrying a retired wording is *re-placed*; an absent one returns `None`.
fn converge_banner_wording(source: &str) -> Option<String> {
    // A frontmatter source is the note's, not the banner's — never shove a comment
    // ahead of a leading `---`, which would break the frontmatter block.
    if frontmatter::frontmatter_matter(source).is_some() {
        return None;
    }
    let first = source.lines().next().unwrap_or_default();
    if first
        .trim_start()
        .starts_with(crate::placement::BANNER_MARKER)
    {
        if first == crate::placement::BANNER {
            return Some(source.to_string());
        }
        // Stale wording: splice the current banner over the marked line.
        return Some(source.replacen(first, crate::placement::BANNER, 1));
    }
    // No banner found — nothing to converge.
    None
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

    let (mut applied, mut unchanged, mut conflicted, mut superseded) = (0u32, 0u32, 0u32, 0u32);
    for entry in &outcome.entries {
        match entry.outcome {
            ApplyOutcome::Applied => applied += 1,
            ApplyOutcome::Unchanged => unchanged += 1,
            ApplyOutcome::Conflicted => conflicted += 1,
            ApplyOutcome::SupersededByMember => superseded += 1,
        }
        out.push_str(&format!(
            "{:<10}  {:<18}  {}\n",
            entry.outcome.label(),
            entry.placement,
            entry.path.display()
        ));
    }
    out.push_str(&format!(
        "\n{applied} applied, {unchanged} unchanged, {conflicted} conflicted, {superseded} superseded-by-member\n"
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::tmpdir;

    /// Every message the guard speaks at the pending-write edge closes on the one
    /// binding limit, and that limit's tool list is [`GUARD_MATCHER`]'s own alternation
    /// — a message that re-spells the list instead of composing the tail fails here the
    /// moment the matcher widens.
    #[test]
    fn every_pending_write_guard_message_closes_on_the_matchers_binding_limit() {
        let tools = GUARD_MATCHER.replace('|', "/");
        assert!(!tools.is_empty(), "the guard's PreToolUse row binds tools");
        let tail = binding_limit();
        let (head, _) = tail
            .split_once(&format!("({tools})"))
            .unwrap_or_else(|| panic!("the limit names the matcher's tools, got: {tail}"));

        // Both spellings of the locus message: the loader claim moves with the kind's
        // `custom`, and the limit is owed by either tail.
        let undeclared = undeclared_locus_message("skill", false);
        let undeclared_custom = undeclared_locus_message("widget", true);
        let messages: [&str; 6] = [
            guard_message(),
            &undeclared,
            &undeclared_custom,
            guard_manifest_message(),
            guard_manifest_edit_message(),
            guard_manifest_unparseable_message(),
        ];
        assert_eq!(
            messages.len(),
            6,
            "every pending-write guard message is judged here"
        );
        for message in messages {
            assert!(
                message.ends_with(tail),
                "a pending-write guard message must close on the one limit, got: {message}"
            );
            assert_eq!(
                message.matches(head).count(),
                1,
                "the limit is spoken once, not re-spelled, in: {message}"
            );
        }
    }

    /// The guard's in-band surface encodes a finding one way whichever edge speaks: the
    /// post edge's half and a manifest's findings both run through [`render_guard_findings`],
    /// so a respelled line cannot move one edge and leave the other behind. The manifest
    /// half keeps only the job the post edge's does not have — picking the header its
    /// findings earn, unparseable outranking unreconstructable-edit outranking contract.
    #[test]
    fn both_guard_edges_encode_one_finding_line_under_their_own_preamble() {
        let contract = Diagnostic::error("skill.max_len.description", "review", "1 over 200");
        let line = "\n  skill.max_len.description: 1 over 200";
        let findings = std::slice::from_ref(&contract);
        assert!(!findings.is_empty(), "the encoder is judged over a finding");

        for preamble in [GUARD_SHELL_EDGE_MESSAGE, GUARD_SHELL_EDGE_LOCUS_MESSAGE] {
            assert_eq!(
                render_guard_findings(preamble, findings),
                format!("{preamble}{line}"),
            );
        }
        assert_eq!(
            render_manifest_findings(findings),
            format!("{}{line}", guard_manifest_message()),
        );

        let edit = Diagnostic::error(GUARD_MANIFEST_EDIT_RULE, "manifest", "unreconstructable");
        let unparseable =
            Diagnostic::error(GUARD_MANIFEST_UNPARSEABLE_RULE, "manifest", "not JSON");
        for (set, header) in [
            (vec![contract.clone()], guard_manifest_message()),
            (
                vec![contract.clone(), edit.clone()],
                guard_manifest_edit_message(),
            ),
            (
                vec![contract.clone(), edit.clone(), unparseable.clone()],
                guard_manifest_unparseable_message(),
            ),
            (
                vec![unparseable.clone(), edit.clone(), contract.clone()],
                guard_manifest_unparseable_message(),
            ),
        ] {
            assert!(!set.is_empty(), "the header is judged over findings");
            let rendered = render_manifest_findings(&set);
            assert!(
                rendered.starts_with(header),
                "the header its findings earn, got: {rendered}"
            );
            for finding in &set {
                let encoded = format!("\n  {}: {}", finding.rule, finding.message);
                assert!(
                    rendered.contains(&encoded),
                    "every finding rides under the header, missing: {encoded}"
                );
            }
        }
    }

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
        let source = member_module_source(
            "skill",
            "coordinate",
            "coordinate",
            "skill_coordinate",
            &fields,
            Some(body),
        );
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
        let source = member_module_source(
            "rule",
            "rust",
            "rust",
            "rule_rust",
            &fields,
            Some("# Rust\n"),
        );
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
        let source = member_module_source(
            "agent",
            "reviewer",
            "reviewer",
            "agent_reviewer",
            &fields,
            Some(""),
        );
        assert_eq!(source.matches("name:").count(), 1);
    }

    #[test]
    fn member_module_source_renders_a_whole_document_member_as_fields_alone() {
        // A `json-document` kind's artifact is its fields — there is no body slot, so the
        // module carries no `prose:` property and imports neither prose constructor.
        let fields = vec![
            ("model".to_string(), serde_json::json!("opus")),
            (
                "permissions".to_string(),
                serde_json::json!({ "allow": ["Bash(cargo test:*)"] }),
            ),
        ];
        let source = member_module_source(
            "settings",
            "settings",
            "settings",
            "settings_settings",
            &fields,
            None,
        );
        assert_eq!(
            source,
            "import { settings } from \"@dtmd/temper/claude-code\";\n\n\
             export const settings_settings = settings({\n  \
             name: \"settings\",\n  \
             model: \"opus\",\n  \
             permissions: {\"allow\":[\"Bash(cargo test:*)\"]},\n});\n"
        );
    }

    #[test]
    fn member_module_source_imports_a_hyphenated_kinds_camel_case_constructor() {
        let fields = vec![("version".to_string(), serde_json::json!("1.2.3"))];
        let source = member_module_source(
            "plugin-manifest",
            "demo-pack",
            "demo-pack",
            "plugin_manifest_demo_pack",
            &fields,
            None,
        );
        assert_eq!(
            source,
            "import { pluginManifest } from \"@dtmd/temper/claude-code\";\n\n\
             export const plugin_manifest_demo_pack = pluginManifest({\n  \
             name: \"demo-pack\",\n  \
             version: \"1.2.3\",\n});\n"
        );
    }

    #[test]
    fn member_ident_folds_the_kind_prefix_as_well_as_the_name() {
        assert_eq!(member_ident("skill", "coordinate"), "skill_coordinate");
        assert_eq!(
            member_ident("settings-local", "settings.local"),
            "settings_local_settings_local"
        );
    }

    #[test]
    fn sdk_constructor_camel_cases_every_hyphenated_segment() {
        assert_eq!(sdk_constructor("settings"), "settings");
        assert_eq!(sdk_constructor("settings-local"), "settingsLocal");
        assert_eq!(sdk_constructor("known-marketplace"), "knownMarketplace");
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
        let mut bindings = BTreeSet::new();
        let render = |value: JsonValue, bindings: &mut BTreeSet<&'static str>| {
            json_to_ts_literal(&value, bindings)
        };
        assert_eq!(render(json!("x"), &mut bindings), "\"x\"");
        assert_eq!(render(json!(true), &mut bindings), "true");
        assert_eq!(render(json!(7), &mut bindings), "7");
        assert_eq!(render(json!(null), &mut bindings), "null");
        assert_eq!(render(json!(["a", "b"]), &mut bindings), "[\"a\",\"b\"]");
        // The walked container punctuation is JSON's compact form byte for byte, escaping
        // included — a nested object and a quote-bearing key render as `serde_json` would.
        assert_eq!(
            render(json!({ "a\"b": { "c": [1, false] } }), &mut bindings),
            "{\"a\\\"b\":{\"c\":[1,false]}}"
        );
        assert!(
            bindings.is_empty(),
            "no ordinary field names a gate command, got: {bindings:?}"
        );
    }

    #[test]
    fn json_to_ts_literal_spells_a_gate_command_as_its_binding_at_any_depth() {
        // The rule is a property of the bytes, not of the field: the command a gate hook
        // fires sits inside its handler array, so a leaf-level substitution is the only
        // one that reaches it.
        let mut bindings = BTreeSet::new();
        assert_eq!(
            json_to_ts_literal(
                &json!([{ "type": "command", "command": GUARD_COMMAND }]),
                &mut bindings
            ),
            "[{\"command\":GUARD_COMMAND,\"type\":\"command\"}]"
        );
        assert_eq!(
            json_to_ts_literal(&json!(SESSION_START_COMMAND), &mut bindings),
            "SESSION_START_COMMAND"
        );
        assert_eq!(
            bindings.iter().copied().collect::<Vec<_>>(),
            ["GUARD_COMMAND", "SESSION_START_COMMAND"],
            "every binding the render named joins the import list, in table order"
        );
    }

    #[test]
    fn every_gate_command_binding_names_a_distinct_command() {
        // The table is the one home for the names, so a duplicated binding or a
        // duplicated command would make [`gate_command_binding`] pick by position.
        let names: BTreeSet<&str> = GATE_COMMAND_BINDINGS.iter().map(|(n, _)| *n).collect();
        let commands: BTreeSet<&str> = GATE_COMMAND_BINDINGS.iter().map(|(_, c)| *c).collect();
        assert_eq!(names.len(), GATE_COMMAND_BINDINGS.len());
        assert_eq!(commands.len(), GATE_COMMAND_BINDINGS.len());
        for (name, command) in GATE_COMMAND_BINDINGS {
            assert_eq!(gate_command_binding(command), Some(name));
        }
        // Sorted by binding name — the order the generated module and every import list
        // read off it.
        assert!(
            GATE_COMMAND_BINDINGS.is_sorted_by_key(|(name, _)| *name),
            "got: {GATE_COMMAND_BINDINGS:?}"
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
    fn converge_banner_wording_returns_none_for_a_fresh_body() {
        let body = "# Project\n\nProject-wide memory for the agents.\n";
        // No banner present — converge_banner_wording returns None. Emit places new banners.
        assert_eq!(converge_banner_wording(body), None);
    }

    #[test]
    fn converge_banner_wording_declines_a_frontmatter_source() {
        // A frontmatter body is the `#` note's; the banner never fronts a `---` block.
        assert_eq!(converge_banner_wording("---\nname: x\n---\n# Body\n"), None);
    }

    #[test]
    fn converge_banner_wording_re_places_a_stale_wording() {
        let stale = "<!-- temper: managed projection — old wording. -->\n\n# Project\n";
        let placed = converge_banner_wording(stale).expect("a marked-but-stale banner re-places");
        assert!(placed.starts_with(crate::placement::BANNER));
        assert!(placed.ends_with("\n\n# Project\n"));
        assert_eq!(placed.matches(crate::placement::BANNER_MARKER).count(), 1);
    }

    #[test]
    fn converge_banner_wording_declines_a_crlf_opened_frontmatter_source() {
        assert_eq!(
            converge_banner_wording("---\r\nname: x\r\n---\r\n# Body\n"),
            None
        );
    }

    #[test]
    fn converge_banner_wording_returns_none_for_unterminated_frontmatter_block() {
        let unterminated = "---\n# Body without closing delimiter\n";
        // This is not actual frontmatter (no closing delimiter), so banner is NOT present.
        // converge_banner_wording returns None — emit will place the banner.
        assert_eq!(converge_banner_wording(unterminated), None);
    }
    /// The stem legality check is the committed scaffold's, not the host's: every shape
    /// Windows reserves is refused here, on linux, because a member module a unix author
    /// scaffolds is a file a Windows teammate has to check out
    /// (learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file, retrieved
    /// 2026-09-29). The accepted column is the vacuity guard — a check that refused
    /// everything would pass the refusal column alone.
    #[test]
    fn a_module_stem_is_judged_against_every_platforms_reserved_set() {
        for reserved in [
            // Each reserved character, in a key a manifest could plausibly carry.
            "PostToolUse:Bash",
            "Edit|Write",
            "plugin<acme",
            "plugin>acme",
            "quoted\"key",
            "path/sep",
            "path\\sep",
            "glob?key",
            "glob*key",
            // A control character, including the one a JSON string can carry escaped.
            "line\nbreak",
            "nul\u{0}byte",
            // A trailing space or dot is stripped rather than refused, so the authored
            // name and the file that appears diverge.
            "trailing ",
            "trailing.",
            ".",
            "..",
            // The device names, through any extension and in any case.
            "CON",
            "nul",
            "Aux",
            "PRN",
            "COM1",
            "com9",
            "LPT1",
            "lpt9",
            "NUL.md",
            // Nothing at all names no file.
            "",
        ] {
            assert!(
                !is_portable_file_stem(reserved),
                "{reserved:?} is not a portable file name"
            );
        }

        for ordinary in [
            "SessionStart",
            "formatter@acme",
            "acme",
            "rust",
            "CLAUDE.md",
            "settings",
            "COM",
            "COM10",
            "CONSOLE",
            "con-fig",
            "my.module",
            "a space inside",
        ] {
            assert!(
                is_portable_file_stem(ordinary),
                "{ordinary:?} names a file everywhere"
            );
        }
    }

    /// The stem allocator's ordinal suffix appends `-<digit>` — no reserved character, no
    /// trailing dot or space, and no device name — so an id that clears the check clears
    /// every stem derived from it, which is what lets [`module_stems`] judge the id once.
    #[test]
    fn an_ordinal_suffix_never_makes_a_cleared_id_unportable() {
        for id in ["SessionStart", "memory", "COM", "LPT", "a.b"] {
            assert!(is_portable_file_stem(id), "{id:?} is the cleared id");
            for ordinal in 2..=10usize {
                let stem = format!("{id}-{ordinal}");
                assert!(
                    is_portable_file_stem(&stem),
                    "{stem:?} is derived from a cleared id"
                );
            }
        }
    }
}
