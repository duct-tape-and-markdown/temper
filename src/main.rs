//! `temper` CLI entry point.
//!
//! A thin `clap` dispatch over the [`temper`] library: parse args, run the
//! generic contract engine, map the result to an exit code. Every subcommand
//! mirrors the pipeline verbs; all logic lives in the
//! library so `tests/` can drive it.

#![deny(rustdoc::broken_intra_doc_links)]

use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::LazyLock;

use clap::{Parser, Subcommand, ValueEnum};
use miette::IntoDiagnostic;
use temper::builtin_kind;
use temper::bundle;
use temper::check::{self, Severity};
use temper::compose;
use temper::drift;
use temper::gate;
use temper::install;
use temper::kind::Format;
use temper::read;
use temper::reporter;
use temper::schema;
use temper::tap;

/// The SDK surface workspace under the cwd — the `emit` workspace default and the
/// path `schema` / `explain` / `bundle` read the committed lock from.
///
/// Explicitly `./`-prefixed: the default is the one relative shape `emit` re-resolves
/// a `node` arg against a moved cwd from, so it is the spelling the path-doubling
/// regression is pinned at (`drift::run_sdk_program`). Downstream the prefix is
/// immaterial — `emit` lexically normalizes it away before deriving `harness_root`.
///
/// A `LazyLock` rather than a `const`: it is derived from [`temper::WORKSPACE_DIR`],
/// and `concat!` takes only literals — which would re-spell the name this const has
/// its one home for.
static DEFAULT_WORKSPACE: LazyLock<String> = LazyLock::new(temper::default_workspace);

/// The long version form: the release tag's string plus the commit `build.rs` stamped,
/// when the source tree it was built from carried git metadata.
///
/// Build provenance rides this form alone: the short form stays [`temper::VERSION`]
/// byte-for-byte, so every build of one tag — tarball or git checkout — agrees with it and
/// with every artifact that stamps it (`bundle`'s plugin manifest, the SARIF driver).
///
/// A `LazyLock` because the stamp is read at runtime from an optional env var, which no
/// `concat!` of literals can splice. Borrowing it out of a `static` is what makes the
/// `&'static str` clap wants.
static LONG_VERSION: LazyLock<String> =
    LazyLock::new(|| match option_env!("TEMPER_BUILD_COMMIT") {
        Some(commit) => format!("{} ({commit})", temper::VERSION),
        None => temper::VERSION.to_string(),
    });

/// A typed maintenance surface for the Claude Code harness.
//
// `version` feeds the short form (`-V`) and `long_version` the long one (`--version`);
// each falls back to the other when only one is set — clap's own documented split
// (`Command::version` / `Command::long_version`,
// <https://docs.rs/clap/4.6.1/clap/builder/struct.Command.html#method.long_version>,
// retrieved 2026-09-29; verified against the vendored clap_builder 4.6.0 source).
// `version` is spelled to `temper::VERSION` rather than left to the derive's
// `CARGO_PKG_VERSION` magic so the one const naming the release tag's string is visibly
// the thing the short form carries.
#[derive(Parser)]
#[command(
    name = "temper",
    version = temper::VERSION,
    long_version = LONG_VERSION.as_str(),
    about,
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Gate a harness against the active contract. The path argument resolves the same
    /// way for every reporter — a harness root (gating its `.temper/` workspace when
    /// present, else the raw root off disk), or a workspace directory carrying
    /// `lock.toml`, gated against the harness root enclosing it. Every spelling of one
    /// harness resolves to the same verdict, and none reads the lock from a different
    /// place than the session-start reporter does. Session-start is a **reporter** of
    /// this gate, never a verb: it is advisory (always exits zero), so a Claude Code
    /// `SessionStart` hook runs `temper check . --reporter session-start`.
    Check {
        /// The harness root to gate (defaults to the cwd) — or the `.temper/` workspace
        /// inside one, which gates against the harness root enclosing it.
        root: Option<PathBuf>,
        /// An explicit spelling of the same harness root, kept as a usage-conflict
        /// guard: passing it together with the positional root is a usage error, not a
        /// silent precedence pick.
        #[arg(long, conflicts_with = "root")]
        harness: Option<PathBuf>,
        /// Also fail the run on `advisory` violations, not just `required` ones — the
        /// strict CI policy. Disclosure notes are not violations and never block.
        #[arg(long)]
        deny_advisories: bool,
        /// A policy layer to join, named as the lock that carries it (the lock file, or a
        /// directory holding one) — repeatable, joined in the order given. The layer's
        /// clause rows range over this corpus's own selections, keyed by kind name. A
        /// layer can only harden: its clauses are added to the contracts this harness
        /// already declares, never substituted for them.
        #[arg(long = "layer", value_name = "PATH")]
        layers: Vec<PathBuf>,
        /// The machine format for the diagnostic set. Presentation only — the exit-code verdict is identical
        /// whichever is chosen (session-start excepted: it is always advisory).
        #[arg(long, value_enum, default_value_t = Reporter::Terminal)]
        reporter: Reporter,
    },
    /// Emit the active per-kind contract as an editor JSON Schema (the keystroke
    /// gate).
    Schema {
        /// Emit only this artifact kind's schema (see the unknown-kind error for
        /// the live domain); omitted ⇒ a JSON object mapping each modeled kind to
        /// its schema.
        #[arg(long)]
        kind: Option<String>,
    },
    /// Compile the authoring face: re-emit each projection **whole** from the
    /// surface, byte-deterministically and double-emit verified.
    /// Each artifact is regenerated full-file —
    /// byte-stable and idempotent — and written back to the source path its provenance
    /// names; a direct edit to the projection is drift routed to the authored source,
    /// never something emit merges around.
    Emit {
        /// The surface workspace to project (defaults to `./.temper`), taken
        /// positionally as every verb takes its workspace. The lock under it carries
        /// the emit fingerprints freshness stands on.
        #[arg(default_value = DEFAULT_WORKSPACE.as_str())]
        workspace: PathBuf,
        /// Refuse network access — the CI posture.
        /// `emit` performs no network I/O today, so this is accepted for CI parity.
        #[arg(long)]
        frozen: bool,
        /// Compute and report every projection without writing a single byte — not
        /// the re-emitted sources, not the updated lock.
        #[arg(long)]
        dry_run: bool,
        /// Spell a full teardown: let a reap wave that would delete every live
        /// projection through instead of refusing at the cliff. Off by default, so
        /// a workspace re-root that strands the whole projection tree refuses rather
        /// than mass-deleting silently.
        #[arg(long)]
        teardown: bool,
    },
    /// The guard at both edges of a tool call: read Claude Code's hook payload from
    /// stdin and inform-and-route under the declared enforcement mode — `note` allows
    /// and defers out-of-band, `warn` allows and surfaces in-band (exit 0), `block`
    /// refuses. Which edge fired decides the subject: a `PreToolUse` payload names a
    /// pending write, bound when it targets a `.claude/` projection or a governed locus
    /// the lock declares no member at (`block` denies it, exit 2); a `PostToolUse`
    /// payload follows a shell tool whose writes it names no path for, so the guard
    /// judges the tree the call left for projection drift and `block` refuses the call's
    /// result in-band, naming the restore — a write already made cannot be denied. The
    /// mode is read live from the harness's lock —
    /// temper never escalates on its own determination, and an unrepresented
    /// harness (no lock) reads the default `warn`. Both edges are wired by
    /// `temper install`. A pass prints nothing.
    Guard {
        /// The harness root whose `.temper/lock.toml` declares the posture (defaults
        /// to the current directory, the project Claude Code runs the hook from).
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// The telemetry tap: read a Claude Code hook payload from stdin and append
    /// one machine-written record — the event's identity and its minimal
    /// discriminant (the member or path it names, the load reason, the session
    /// id), never captured prose — to the per-machine, uncommitted event log
    /// under the harness's workspace. Advisory recording, never a gate: it always
    /// exits zero, and a payload naming no recognized event records nothing.
    Tap {
        /// The harness root whose `.temper/` workspace carries the event log
        /// (defaults to the current directory, the project Claude Code runs the
        /// hook from).
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// `temper install` — the one on-ramp:
    /// a discovery report, then one question — represent this
    /// project as a temper program? `--no-represent` wires the `SessionStart`
    /// reporter alone; `--yes` (or an interactive `y`) scaffolds the SDK program
    /// (the lift), ensures the `@dtmd/temper` dependency, runs the first `emit`, and
    /// places the guard/managed-by note/schema modeline at the fresh lock's
    /// emit-owned targets.
    Install {
        /// The project root to represent (defaults to the current directory, beside
        /// the `.claude/` the placements land in).
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Answer the one question "yes" without prompting — agents and CI.
        #[arg(long, conflicts_with = "no_represent")]
        yes: bool,
        /// Answer the one question "no" without prompting — agents and CI.
        #[arg(long)]
        no_represent: bool,
        /// Compute and report every placement without writing a single byte.
        #[arg(long)]
        dry_run: bool,
    },
    /// Compose the surface into a publishable Claude Code plugin +
    /// `marketplace.json`: the operate-the-gate skill,
    /// the `SessionStart` hook, and the shipped built-in kinds embedded.
    /// Deterministic and byte-faithful where it carries prose, so re-running
    /// reproduces an identical tree.
    Bundle {
        /// The surface workspace to compose from (defaults to `./.temper`).
        #[arg(default_value = DEFAULT_WORKSPACE.as_str())]
        path: PathBuf,
        /// Where to write the plugin tree (defaults to `./plugin`).
        #[arg(long, default_value = "./plugin")]
        out: PathBuf,
    },
    /// The one read verb: resolve
    /// `<target>` across the member / requirement / kind / leaf-address
    /// namespaces and narrate whichever the graph `check` already computes answers it —
    /// a member's forward walk, blast radius, and neighborhood; a requirement's
    /// satisfier set, coverage, and blast radius; a kind's declared authoring
    /// guidance/cite, narrated before any member of it exists; or a leaf's citations
    /// (distinct from its fallout) and neighborhood. A bare name resolves a member or
    /// requirement first (ambiguous, and erroring with each match's qualified spelling,
    /// when it names both); absent either, it falls back to a kind name — a qualified
    /// prefix (`member:`/`requirement:`/`kind:`/`address:`) always wins outright. A
    /// read, never a gate: exits zero on every input.
    Explain {
        /// A member id, a requirement name, a kind name, a leaf address
        /// (`<member>/<kind>/<key>/<child-path>`), or one qualified as
        /// `member:<name>` / `requirement:<name>` / `kind:<name>` /
        /// `address:<leaf-address>`.
        target: String,
    },
}

/// The machine format `check` renders its diagnostic set in.
/// Every variant reshapes *presentation
/// only*; none re-judges the harness, so the exit-code verdict is identical.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum Reporter {
    /// The default: miette's graphical terminal render ([`check::render`]).
    Terminal,
    /// GitHub Actions `::error`/`::warning`/`::notice::` workflow-command annotations, inline
    /// on the PR ([`reporter::github`]).
    Github,
    /// A SARIF 2.1.0 log for code-scanning ingestion ([`reporter::sarif`]).
    Sarif,
    /// The `claude-session-start` payload ([`reporter::session_start`]) — the advisory
    /// session-start reporter. It reads the path as a
    /// harness root and always exits zero, so a failing contract routes through the
    /// human via the notify-and-approve verdict rather than blocking the session.
    SessionStart,
}

fn main() -> miette::Result<ExitCode> {
    match Cli::parse().command {
        Command::Check {
            root,
            harness,
            deny_advisories,
            layers,
            reporter,
        } => {
            // The one resolution for every reporter: the path argument (the positional
            // root or its `--harness` synonym) goes to `gate_verdict`, which
            // resolves a harness root and a workspace directory alike — and resolves
            // either whole. A terminal `check <path>` can never read the lock from a
            // different place than it discovers the corpus from, nor from a different
            // place than the session-start reporter does.
            let harness_path = harness.or(root).unwrap_or_else(|| PathBuf::from("."));
            let (diagnostics, announced) = gate_verdict(&harness_path, &layers);

            match reporter {
                Reporter::Terminal => print!("{}", check::render(&diagnostics, &announced)),
                Reporter::Github => print!("{}", reporter::github(&diagnostics, &announced)),
                Reporter::Sarif => println!("{}", reporter::sarif(&diagnostics, &announced)),
                Reporter::SessionStart => {
                    println!("{}", reporter::session_start(&diagnostics, &announced));
                }
            }

            // `--deny-advisories` promotes `advisory` (warn) violations to blocking on top
            // of the always-blocking `required` ones. `Warn` exactly, never any
            // non-error: a `Note` is disclosure with no clause behind it to escalate, so
            // a clean harness's own `coverage.checked` summary can never trip the flag.
            // The session-start reporter is advisory, so it never gates.
            let advisory_blocks = deny_advisories
                && diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.severity == Severity::Warn);
            Ok(
                if reporter != Reporter::SessionStart
                    && (check::any_error(&diagnostics) || advisory_blocks)
                {
                    ExitCode::FAILURE
                } else {
                    ExitCode::SUCCESS
                },
            )
        }
        Command::Schema { kind } => {
            // The keystroke placement of the gate:
            // emit the *active* contract per kind — the same rows-or-default contract
            // `check` gates against — as an editor JSON Schema, over the widened
            // domain (every YAML-frontmatter kind in play, builtin or lock-declared —
            // not the skill/rule fossil).
            let declarations =
                compose::read_committed_declarations(Path::new(DEFAULT_WORKSPACE.as_str()))?;
            let builtin_defs = builtin_kind::definitions();
            let kinds = compose::effective_kinds(&declarations)?;
            let mut domain: Vec<String> = kinds
                .values()
                .filter(|kind| kind.format == Some(Format::YamlFrontmatter))
                .map(|kind| kind.name.clone())
                .collect();
            domain.sort();

            let json = match kind.as_deref() {
                // An unknown kind is a hard error, never a silent empty schema.
                Some(requested) => {
                    let name = domain
                        .iter()
                        .find(|name| name.as_str() == requested)
                        .ok_or_else(|| {
                            miette::miette!(
                                "unknown kind `{requested}` (temper models: {})",
                                domain.join(", ")
                            )
                        })?;
                    let contract = compose::kind_contract_with_bases(
                        &declarations,
                        &kinds,
                        &builtin_defs,
                        name,
                    )?;
                    schema::emit(&contract)
                }
                None => {
                    let mut map = serde_json::Map::new();
                    for name in &domain {
                        let contract = compose::kind_contract_with_bases(
                            &declarations,
                            &kinds,
                            &builtin_defs,
                            name,
                        )?;
                        map.insert(name.clone(), schema::emit(&contract));
                    }
                    serde_json::Value::Object(map)
                }
            };

            println!("{}", serde_json::to_string_pretty(&json).into_diagnostic()?);
            Ok(ExitCode::SUCCESS)
        }
        Command::Emit {
            workspace,
            frozen,
            dry_run,
            teardown,
        } => {
            // The seam:
            // `node` runs the SDK program at `<workspace>/harness.ts`, and the engine becomes the
            // sole compiler of every projection and the whole lock from its JSON payload — no
            // harness root is re-supplied here, the payload IS the source.
            let report = drift::emit_program(
                &workspace,
                drift::EmitOptions {
                    dry_run,
                    frozen,
                    teardown,
                },
            )?;
            if dry_run {
                println!("dry run — no files written");
            }
            print!("{}", drift::render_emit(&report));
            Ok(ExitCode::SUCCESS)
        }
        Command::Guard { path } => {
            // The guard at both edges of a Claude Code tool call: read the hook payload
            // from stdin and act at the author's declared enforcement mode, three values
            // split by where the finding goes: `note` allows and defers out-of-band (exit
            // 0, no in-band message — the next report, never the session); `warn` allows
            // and surfaces in-band via `additionalContext` (exit 0), the only channel a
            // zero-exit hook reaches the model through; `block` refuses. temper never
            // escalates past the mode the lock declares — the lock is what names a path a
            // projection, so it is also the sole source for how firmly that projection is
            // enforced. An unrepresented
            // harness (no lock) reads the default `warn`, matching
            // `compose::EnforcementMode`'s own default, and falls back to binding any
            // `.claude/` write since there is no declared set to consult.
            let mut stdin_payload = String::new();
            io::Read::read_to_string(&mut io::stdin(), &mut stdin_payload).into_diagnostic()?;

            // One decode, before any judge reads a field: every binding below reads the
            // payload structurally off this value, so no two of them can disagree about
            // what one field says. A payload that is not JSON refuses loud rather than
            // falling through to an allow (intent.md, invariant 6) — the guard reads a
            // machine-written envelope, so arbitrary text on stdin means the hook row is
            // mis-wired, and a silent zero would hide the one condition that disables the
            // boundary wholesale.
            let payload: serde_json::Value =
                serde_json::from_str(&stdin_payload).map_err(|err| {
                    miette::miette!(
                        "`temper guard` read no hook payload on stdin ({err}) — this verb \
                         runs from a Claude Code `PreToolUse`/`PostToolUse` hook row and \
                         judges the JSON envelope that fires it. Re-run `temper install` \
                         to re-wire the gate hooks, or invoke the verb the way the row \
                         does."
                    )
                })?;

            // The shell edge cannot deny a write already made, so `block` refuses the
            // call's *result* in-band and names the restore: that arm exits zero at every
            // mode, the refusal being the payload rather than the exit code.
            let (edge, decision) = install::run_guard(&path, &payload)?;
            if edge == install::GuardEdge::PostToolUse {
                match decision.verdict {
                    install::GuardVerdict::Allow | install::GuardVerdict::Note => {}
                    install::GuardVerdict::Warn => println!(
                        "{}",
                        reporter::tool_use(reporter::HookEvent::PostToolUse, &decision.message)
                    ),
                    install::GuardVerdict::Block => {
                        println!("{}", reporter::post_tool_use_block(&decision.message));
                    }
                }
                return Ok(ExitCode::SUCCESS);
            }
            Ok(match decision.verdict {
                install::GuardVerdict::Allow | install::GuardVerdict::Note => ExitCode::SUCCESS,
                install::GuardVerdict::Warn => {
                    println!(
                        "{}",
                        reporter::tool_use(reporter::HookEvent::PreToolUse, &decision.message)
                    );
                    ExitCode::SUCCESS
                }
                install::GuardVerdict::Block => {
                    eprintln!("{}", decision.message);
                    ExitCode::from(2)
                }
            })
        }
        Command::Tap { path } => {
            // Advisory recording at Claude Code's lifecycle boundary: read the hook
            // payload from stdin, extract the event's identity + minimal discriminant,
            // and append one record to the per-machine log. A payload naming no
            // recognized event records nothing, and a failed append never gates — the
            // tap always exits zero.
            let workspace_dir = path.join(temper::WORKSPACE_DIR);
            let mut payload = String::new();
            io::Read::read_to_string(&mut io::stdin(), &mut payload).into_diagnostic()?;
            if let Some(record) = tap::record_from_payload(&payload)
                && let Err(err) = tap::append(&workspace_dir, &record)
            {
                eprintln!("temper tap: {err}");
            }
            Ok(ExitCode::SUCCESS)
        }
        Command::Install {
            path,
            yes,
            no_represent,
            dry_run,
        } => {
            // The one resolution, shared with `check`: install's path argument is read
            // against the locks already on disk before a question is asked about it.
            let resolved = resolve_harness_path(&path)?;

            // A path that IS a workspace names no harness root. Rooting install there
            // would discover the workspace's own files and scaffold a
            // `<path>/.claude/settings.json` inside the workspace — so refuse, naming
            // the root the argument meant.
            if let HarnessPath::Workspace { enclosing } = &resolved {
                return Err(miette::miette!(
                    "`{}` is a temper workspace, not a harness root — `install` targets the root a workspace governs; pass `{}`",
                    path.display(),
                    enclosing.display()
                ));
            }
            let lock = match &resolved {
                HarnessPath::Root { lock, .. } => lock.clone(),
                HarnessPath::Workspace { .. } | HarnessPath::Raw => None,
            };
            let (workspace, root) = resolved.gate_pair(&path);

            // Which mark already settled the represent fork on disk. The authored program
            // comes first, and it is the reading `check`'s self-verify shares
            // (`install::represented_by`): a harness carrying a program it has never
            // emitted carries no lock, and answering No there would send `install` down the
            // unrepresented writer to splice `.claude/settings.json` — re-adding a gate hook
            // to a file the program owns whole, which 0073 forbids. The lock settles it too,
            // for a harness whose program was emitted and then deleted from under it.
            let settled = install::represented_by(&path).or(lock);

            // Discovery, the gate's verdict and the placements run as one library call
            // over one set of owners; the closure is the one question between them, asked
            // once the report and the contract's verdict are on screen.
            let outcome = install::execute(
                &root,
                &workspace,
                dry_run,
                |discovery, findings, announced| {
                    print!(
                        "{}",
                        install::render_discovery(
                            discovery,
                            settled.as_deref(),
                            findings,
                            announced
                        )
                    );

                    // A settled fork skips both the prompt and `ask_represent`'s conservative
                    // unattended default: converge on what is already on disk. `--no-represent`
                    // there asserts the false half of a settled fork — refuse rather than place
                    // less than the program justifies.
                    match (&settled, yes, no_represent) {
                        (Some(mark), _, true) => Err(miette::miette!(
                            "`--no-represent` contradicts `{}`: this project is already represented, and a represented harness's placements follow its own program. Re-run without the flag.",
                            mark.display()
                        )),
                        (Some(_), _, false) | (None, true, _) => Ok(install::Represent::Yes),
                        (None, _, true) => Ok(install::Represent::No),
                        (None, false, false) => ask_represent(),
                    }
                },
            )?;
            if dry_run {
                println!("dry run — no files written");
            }
            print!("{}", install::render(&outcome));
            Ok(ExitCode::SUCCESS)
        }
        Command::Bundle { path, out } => {
            let report = bundle::run(&path, &out)?;
            print!("{}", bundle::render(&report));
            Ok(ExitCode::SUCCESS)
        }
        Command::Explain { target } => {
            print!("{}", read::explain_target(Path::new("."), &target)?);
            Ok(ExitCode::SUCCESS)
        }
    }
}

/// Ask `install`'s one question interactively: read a line from stdin, `y`/`yes` (case-insensitive)
/// answering [`install::Represent::Yes`], anything else (including a bare newline)
/// answering [`install::Represent::No`] — the conservative default for an
/// unattended terminal, mirroring the printed prompt's `[y/N]`.
fn ask_represent() -> miette::Result<install::Represent> {
    print!("{} ", install::REPRESENT_QUESTION);
    io::Write::flush(&mut io::stdout()).into_diagnostic()?;
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).into_diagnostic()?;
    Ok(match answer.trim().to_lowercase().as_str() {
        "y" | "yes" => install::Represent::Yes,
        _ => install::Represent::No,
    })
}

/// Where a verb's path argument lands among the locks on disk — the one resolution
/// `check`'s gate and `install`'s represent decision both read a path through, so no
/// verb can disagree with another about which harness a spelling names.
enum HarnessPath {
    /// `<path>/.temper` is a dir ⇒ `<path>` is a harness root carrying a surface
    /// workspace.
    Root {
        /// The authored workspace beside the root — the lock's home.
        workspace: PathBuf,
        /// The workspace's `lock.toml` when it is on disk — the record of an emit, and so
        /// one of the two marks that settle the represent fork ahead of the question.
        lock: Option<PathBuf>,
    },
    /// `<path>/lock.toml` is a file ⇒ `<path>` *is* the workspace, addressed directly,
    /// and `enclosing` is the harness root it governs.
    Workspace {
        /// The harness root enclosing the workspace — the argument a root-taking verb
        /// meant.
        enclosing: PathBuf,
    },
    /// Neither ⇒ an unrepresented raw root: its members live off disk against each
    /// kind's embedded `governs` (the built-in lock), and nothing was ever adopted.
    Raw,
}

impl HarnessPath {
    /// The `(workspace, root)` pair this answer gates through, given the path argument it
    /// was resolved from — the one home for "which workspace does this spelling read its
    /// lock from, and which tree does it walk". Two callers read it: the gate itself
    /// ([`gate_verdict`]) and `install`'s gate-and-place run. A second copy of the match
    /// is exactly how the two could come to disagree about which lock a spelling names.
    fn gate_pair(self, harness_path: &Path) -> (PathBuf, PathBuf) {
        match self {
            Self::Root { workspace, .. } => (workspace, harness_path.to_path_buf()),
            Self::Workspace { enclosing } => (harness_path.to_path_buf(), enclosing),
            Self::Raw => (harness_path.to_path_buf(), harness_path.to_path_buf()),
        }
    }
}

/// Resolve a path argument to the harness it names, and resolve it *whole*: a
/// workspace and the harness root its corpus is discovered from always name the same
/// harness.
///
/// # Errors
///
/// A workspace at the filesystem root has no enclosing harness root — unresolvable, so
/// it fails loud rather than resolving somewhere else.
fn resolve_harness_path(path: &Path) -> miette::Result<HarnessPath> {
    let workspace = path.join(temper::WORKSPACE_DIR);
    if workspace.is_dir() {
        let lock = workspace.join(temper::LOCK_FILENAME);
        let lock = lock.is_file().then_some(lock);
        return Ok(HarnessPath::Root { workspace, lock });
    }
    if path.join(temper::LOCK_FILENAME).is_file() {
        let enclosing = path.parent().ok_or_else(|| {
            miette::miette!(
                "workspace `{}` has no enclosing harness root to discover a corpus from",
                path.display()
            )
        })?;
        // A bare relative workspace (`check .temper`) parents to the empty path, which
        // names the CWD it was resolved against.
        let enclosing = if enclosing.as_os_str().is_empty() {
            Path::new(".")
        } else {
            enclosing
        };
        return Ok(HarnessPath::Workspace {
            enclosing: enclosing.to_path_buf(),
        });
    }
    Ok(HarnessPath::Raw)
}

/// The gate bare `check` runs over a path argument, a load fault lowered to its one
/// `error` diagnostic — the same [`gate::verdict_in`] `install`'s discovery report
/// reads, so the two can never judge a harness by different contracts.
///
/// A [`HarnessPath::Workspace`] gates against its enclosing root, never against itself:
/// rooting it at itself would read the lock from `<path>` while walking `<path>` for a
/// corpus that lives beside it — declared requirements arriving with nothing behind
/// them, so every one of them false-fires `requirement.count`. The adopted branch never
/// re-imports: a fresh import discards recognition (the authored `satisfies` links), so
/// every filled requirement would read unfilled.
fn gate_verdict(
    harness_path: &Path,
    layers: &[PathBuf],
) -> (Vec<check::Diagnostic>, check::Announcement) {
    // A harness that cannot be *loaded* is still a verdict, and it reaches the
    // selected reporter like any other: propagating the report would abort before
    // `check`'s reporter match, so `--reporter session-start` would print an empty
    // stdout — the shape a clean pass is read from — while the failure went to
    // stderr, where the hook does not look. A path that will not resolve lowers
    // here; a gate that will not load lowers in `gate::verdict_in`. Either way
    // `check::load_fault` makes it one `error` diagnostic, so `check`'s exit rule is
    // unchanged: the hard placements still exit non-zero.
    match resolve_harness_path(harness_path) {
        Ok(resolved) => {
            let (workspace, root) = resolved.gate_pair(harness_path);
            gate::verdict_in(
                &temper::import::Discovery::new(&root),
                &workspace,
                layers,
                harness_path,
            )
        }
        Err(report) => (
            vec![check::load_fault(
                &report,
                harness_path.display().to_string(),
                None,
            )],
            check::Announcement::default(),
        ),
    }
}
