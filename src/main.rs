//! `temper` CLI entry point.
//!
//! A thin `clap` dispatch over the [`temper`] library: parse args, run the
//! generic contract engine, map the result to an exit code. Every subcommand
//! mirrors the pipeline verbs; all logic lives in the
//! library so `tests/` can drive it.

#![deny(rustdoc::broken_intra_doc_links)]

use std::collections::BTreeMap;
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
use temper::kind::{CustomKind, Format};
use temper::read;
use temper::reporter;
use temper::schema;
use temper::tap;

/// The SDK surface workspace under the cwd — the emit `--into` default and the
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

/// A typed maintenance surface for the Claude Code harness.
#[derive(Parser)]
#[command(name = "temper", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Lint a harness against the active contract. The path argument resolves the same
    /// way for every reporter — a harness root (gating its `.temper/` workspace when
    /// present, else the raw root off disk), or a workspace directory carrying
    /// `lock.toml`, gated against the harness root enclosing it. Every spelling of one
    /// harness resolves to the same verdict, and none reads the lock from a different
    /// place than the session-start reporter does. Session-start is a **reporter** of
    /// this gate, never a verb: it is advisory (always exits zero), so a Claude Code
    /// `SessionStart` hook runs `temper check . --reporter session-start`.
    Check {
        /// The harness root to lint (defaults to the cwd) — or the `.temper/` workspace
        /// inside one, which gates against the harness root enclosing it.
        root: Option<PathBuf>,
        /// An explicit spelling of the same harness root, kept as a usage-conflict
        /// guard: passing it together with the positional root is a usage error, not a
        /// silent precedence pick.
        #[arg(long, conflicts_with = "root")]
        harness: Option<PathBuf>,
        /// Also fail the run on `advisory` (warn-severity) violations, not just
        /// `required` ones — the strict CI policy.
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
        /// The surface workspace to project (defaults to `./.temper`). The lock
        /// under it carries the emit fingerprints freshness stands on.
        #[arg(long, default_value = DEFAULT_WORKSPACE.as_str())]
        into: PathBuf,
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
        /// an `--into` re-root that strands the whole projection tree refuses rather
        /// than mass-deleting silently.
        #[arg(long)]
        teardown: bool,
    },
    /// The `PreToolUse` guard: read Claude Code's `PreToolUse` payload from stdin
    /// and, when the write targets a `.claude/` projection, inform-and-route under
    /// the declared enforcement mode: `note` allows and defers out-of-band, `warn`
    /// allows and surfaces in-band (exit 0), `block` denies (exit 2). The mode is
    /// read live from the harness's lock —
    /// temper never escalates on its own determination, and an unrepresented
    /// harness (no lock) reads the default `warn`. Wired at the write boundary by
    /// `temper install`.
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
    /// GitHub Actions `::error`/`::warning::` workflow-command annotations, inline
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
            // root or its `--harness` synonym) goes to `harness_diagnostics`, which
            // resolves a harness root and a workspace directory alike — and resolves
            // either whole. A terminal `check <path>` can never read the lock from a
            // different place than it discovers the corpus from, nor from a different
            // place than the session-start reporter does.
            let harness_path = harness.or(root).unwrap_or_else(|| PathBuf::from("."));
            let (diagnostics, announced) = harness_diagnostics(&harness_path, &layers)?;

            match reporter {
                Reporter::Terminal => print!("{}", check::render(&diagnostics, &announced)),
                Reporter::Github => print!("{}", reporter::github(&diagnostics, &announced)),
                Reporter::Sarif => println!("{}", reporter::sarif(&diagnostics, &announced)),
                Reporter::SessionStart => {
                    println!("{}", reporter::session_start(&diagnostics, &announced));
                }
            }

            // `--deny-advisories` promotes `advisory` (warn) violations to blocking on top
            // of the always-blocking `required` ones. The session-start reporter is
            // advisory, so it never gates.
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
            let declarations = drift::read_declarations(Path::new(DEFAULT_WORKSPACE.as_str()))?;
            let builtin_defs = builtin_kind::definitions();
            let domain = yaml_frontmatter_kind_domain(&declarations, &builtin_defs)?;

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
                    let contract = kind_contract(&declarations, &builtin_defs, name)?;
                    schema::emit(&contract)
                }
                None => {
                    let mut map = serde_json::Map::new();
                    for name in &domain {
                        let contract = kind_contract(&declarations, &builtin_defs, name)?;
                        map.insert(name.clone(), schema::emit(&contract));
                    }
                    serde_json::Value::Object(map)
                }
            };

            println!("{}", serde_json::to_string_pretty(&json).into_diagnostic()?);
            Ok(ExitCode::SUCCESS)
        }
        Command::Emit {
            into,
            frozen,
            dry_run,
            teardown,
        } => {
            // The seam:
            // `node` runs the SDK program at `<into>/harness.ts`, and the engine becomes the
            // sole compiler of every projection and the whole lock from its JSON payload — no
            // harness root is re-supplied here, the payload IS the source.
            let report = drift::emit_program(
                &into,
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
            // The guard at Claude Code's write boundary: read the `PreToolUse` payload
            // from stdin, and — when it names one of the lock's emit-owned projections —
            // act at the author's declared enforcement mode, three values split by where the
            // finding goes: `note` allows and defers out-of-band (exit 0, no in-band
            // message — the next report, never the session); `warn` allows and surfaces
            // in-band (exit 0); `block` denies (exit 2). temper never escalates past the
            // mode the lock declares — the lock is what names a path a projection, so it
            // is also the sole source for how firmly that projection is enforced.
            // An unrepresented
            // harness (no lock) reads the default `warn`, matching
            // `compose::EnforcementMode`'s own default, and falls back to binding any
            // `.claude/` write since there is no declared set to consult.
            let workspace_dir = path.join(temper::WORKSPACE_DIR);
            let declarations = drift::read_declarations(&workspace_dir)?;
            let mode = compose::mode_from_declarations(&declarations)?;
            let lock_present = workspace_dir.join(temper::LOCK_FILENAME).is_file();
            let targets = drift::emit_owned_targets(&workspace_dir);
            let manifests = guarded_manifests(&declarations)?;
            let mut payload = String::new();
            io::Read::read_to_string(&mut io::stdin(), &mut payload).into_diagnostic()?;

            // A represented manifest is co-owned — a write touching only opaque residue is
            // legitimate — so its binding is a contract check of the pending members, not the
            // blanket projection-drift the `.claude/` binding runs. It is consulted first:
            // when the write targets a manifest, its verdict is authoritative; otherwise the
            // projection binding decides. Both act at the one enforcement mode the lock declares.
            if let Some(findings) = install::manifest_write_findings(&payload, &manifests) {
                if findings.is_empty() {
                    return Ok(ExitCode::SUCCESS);
                }
                let report = install::render_manifest_findings(&findings);
                return Ok(match mode {
                    compose::EnforcementMode::Note => ExitCode::SUCCESS,
                    compose::EnforcementMode::Warn => {
                        eprintln!("{report}");
                        ExitCode::SUCCESS
                    }
                    compose::EnforcementMode::Block => {
                        eprintln!("{report}");
                        ExitCode::from(2)
                    }
                });
            }

            Ok(
                match install::guard(&payload, mode, lock_present.then_some(targets.as_slice())) {
                    install::GuardVerdict::Allow | install::GuardVerdict::Note => ExitCode::SUCCESS,
                    install::GuardVerdict::Warn => {
                        eprintln!("{}", install::GUARD_MESSAGE);
                        ExitCode::SUCCESS
                    }
                    install::GuardVerdict::Block => {
                        eprintln!("{}", install::GUARD_MESSAGE);
                        ExitCode::from(2)
                    }
                },
            )
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

            let discovery = install::discover(&path)?;
            print!("{}", install::render_discovery(&discovery, lock.as_deref()));

            // A lock on disk has already answered the one question, so neither the
            // prompt nor `ask_represent`'s conservative unattended default applies:
            // converge on the lock. `--no-represent` there asserts the false half of a
            // settled fork — refuse rather than place less than the lock justifies.
            let represent = match (&lock, yes, no_represent) {
                (Some(lock), _, true) => {
                    return Err(miette::miette!(
                        "`--no-represent` contradicts `{}`: this project is already represented, and a represented harness's placements follow its lock. Re-run without the flag.",
                        lock.display()
                    ));
                }
                (Some(_), _, false) => install::Represent::Yes,
                (None, true, _) => install::Represent::Yes,
                (None, _, true) => install::Represent::No,
                (None, false, false) => ask_represent()?,
            };

            let outcome = install::run(&path, &discovery, represent, dry_run)?;
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
            print!("{}", read::explain_target(&target)?);
            Ok(ExitCode::SUCCESS)
        }
    }
}

/// `schema`'s domain: every kind — embedded built-in or lock-declared — whose
/// projection format is [`Format::YamlFrontmatter`], the represented keystroke
/// placement (`specs/distribution.md`, "The placements and their enforcement
/// modes"). Sorted, so `--kind` errors and the no-`--kind` map are deterministic.
///
/// # Errors
///
/// Propagates the lock-row lift errors a malformed declared kind row raises.
fn yaml_frontmatter_kind_domain(
    declarations: &drift::Declarations,
    builtin_defs: &BTreeMap<String, CustomKind>,
) -> miette::Result<Vec<String>> {
    let mut domain: Vec<String> = builtin_defs
        .values()
        .filter(|def| def.format == Some(Format::YamlFrontmatter))
        .map(|def| def.name.clone())
        .collect();

    let (custom_rows, _collisions) = compose::partition_kind_rows(declarations, builtin_defs)?;
    for row in custom_rows {
        let custom_kind = CustomKind::from_kind_fact_row(row)?;
        if custom_kind.format == Some(Format::YamlFrontmatter) {
            domain.push(row.name.clone());
        }
    }

    domain.sort();
    Ok(domain)
}

/// A domain kind's active contract — lock clauses, else the embedded default for a
/// built-in, else the bare default for a declared custom kind — the same split
/// [`guarded_manifests`] and `read.rs::explain_target` run, so `schema` never
/// disagrees with `check`/`explain`/`guard` about a kind's contract.
///
/// # Errors
///
/// Propagates the clause-lift errors contract resolution raises.
fn kind_contract(
    declarations: &drift::Declarations,
    builtin_defs: &BTreeMap<String, CustomKind>,
    name: &str,
) -> miette::Result<temper::contract::Contract> {
    if builtin_defs.contains_key(name) {
        compose::builtin_contract(&declarations.clauses, &declarations.kinds, name)
    } else {
        Ok(compose::default_contract_from_rows(
            &declarations.clauses,
            &declarations.kinds,
            name,
        )?)
    }
}

/// Every represented manifest the `PreToolUse` guard checks a pending write against — one
/// [`install::GuardedManifest`] per manifest kind, whether an embedded built-in
/// ([`builtin_kind::definitions`]) or a lock-declared custom kind. A manifest kind is one
/// carrying a `collection_address`; its contract is resolved exactly as [`gate`] resolves it
/// (lock clauses, else the embedded default), so the guard and the gate judge a member's
/// contract identically.
///
/// # Errors
///
/// Propagates the clause-lift errors [`gate`]'s own contract resolution raises.
fn guarded_manifests(
    declarations: &drift::Declarations,
) -> miette::Result<Vec<install::GuardedManifest>> {
    let builtin_defs = builtin_kind::definitions();

    let mut manifests = Vec::new();
    for kind in builtin_defs.values() {
        let kind = compose::overlay_builtin_kind(kind, declarations)?;
        let (Some(address), Some(path)) = (kind.collection_address.clone(), manifest_path(&kind))
        else {
            continue;
        };
        let contract =
            compose::builtin_contract(&declarations.clauses, &declarations.kinds, &kind.name)?;
        manifests.push(install::GuardedManifest {
            path,
            kind,
            contract,
            address,
        });
    }

    let (custom_rows, _collisions) = compose::partition_kind_rows(declarations, &builtin_defs)?;
    for row in custom_rows {
        let kind = CustomKind::from_kind_fact_row(row)?;
        let (Some(address), Some(path)) = (kind.collection_address.clone(), manifest_path(&kind))
        else {
            continue;
        };
        let contract = compose::default_contract_from_rows(
            &declarations.clauses,
            &declarations.kinds,
            &row.name,
        )?;
        manifests.push(install::GuardedManifest {
            path,
            kind,
            contract,
            address,
        });
    }
    Ok(manifests)
}

/// The harness-relative path a manifest `kind` governs — its `governs` locus, the suffix the
/// guard matches a pending write's `file_path` against (tolerant of the file_path arriving
/// absolute, the same suffix compare the projection binding runs). [`None`] for a kind
/// governing no locus, which has no host file for the guard to watch.
fn manifest_path(kind: &CustomKind) -> Option<PathBuf> {
    let governs = kind.governs.as_ref()?;
    Some(drift::join_locus(&governs.root, &governs.glob))
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
        /// The workspace's `lock.toml` when it is on disk: the root is represented, and
        /// the represent fork is answered here rather than by a question.
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

/// The one-shot gate over a path argument — shared by `check` and the session-start
/// reporter, over [`resolve_harness_path`]'s three answers.
///
/// A [`HarnessPath::Workspace`] gates against its enclosing root, never against itself:
/// rooting it at itself would read the lock from `<path>` while walking `<path>` for a
/// corpus that lives beside it — declared requirements arriving with nothing behind
/// them, so every one of them false-fires `requirement.unfilled`.
///
/// # Errors
///
/// As [`resolve_harness_path`].
///
/// The adopted branch never re-imports: a fresh import discards recognition (the
/// authored `satisfies` links), so every filled requirement would read unfilled — the
/// false positive on clean input the surface-present clause forbids.
fn harness_diagnostics(
    harness_path: &Path,
    layers: &[PathBuf],
) -> miette::Result<(Vec<check::Diagnostic>, check::Announcement)> {
    match resolve_harness_path(harness_path)? {
        HarnessPath::Root { workspace, .. } => gate::gate(&workspace, harness_path, layers),
        HarnessPath::Workspace { enclosing } => gate::gate(harness_path, &enclosing, layers),
        HarnessPath::Raw => gate::gate(harness_path, harness_path, layers),
    }
}
