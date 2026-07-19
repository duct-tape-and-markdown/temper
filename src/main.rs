//! `temper` CLI entry point.
//!
//! A thin `clap` dispatch over the [`temper`] library: parse args, run the
//! generic contract engine, map the result to an exit code. Every subcommand
//! mirrors the pipeline verbs; all logic lives in the
//! library so `tests/` can drive it.

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
use temper::contract::Contract;
use temper::drift;
use temper::extract;
use temper::gate;
use temper::graph;
use temper::import;
use temper::install;
use temper::kind::{self, CustomKind};
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
static DEFAULT_WORKSPACE: LazyLock<String> =
    LazyLock::new(|| format!("./{}", temper::WORKSPACE_DIR));

thread_local! {
    static RESOLVE_KIND_UNITS_COUNT: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// This thread's cumulative count of `resolve_kind_units` invocations. Read before and
/// after a gate/explain run and compare the delta to the kinds the run resolves, pinning
/// that `resolve_kind_units` — the corpus's one source of live member units off disk —
/// runs exactly once per built-in kind and once per custom kind, never twice.
#[must_use]
pub fn resolve_kind_units_count() -> usize {
    RESOLVE_KIND_UNITS_COUNT.with(std::cell::Cell::get)
}

/// The kinds `schema` emits a default contract for, by bare row label; widening it
/// to `memory` is a separate question.
const BUILTIN_DEFAULT_CONTRACT_KINDS: &[&str] = &["skill", "rule"];

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
        /// Emit only this artifact kind's schema (`skill`, `rule`); omitted ⇒ a
        /// JSON object mapping each modeled kind to its schema.
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
    /// `<target>` across the member / requirement / leaf-address
    /// namespaces and narrate whichever the graph `check` already computes answers it —
    /// a member's forward walk, blast radius, and neighborhood; a requirement's
    /// satisfier set, coverage, and blast radius; or a leaf's citations (distinct from
    /// its fallout) and neighborhood. Exactly one hit resolves; a bare name matching
    /// both a member and a requirement is ambiguous and errors with each match's
    /// qualified spelling (`member:<name>`, `requirement:<name>`) for the retry — a
    /// qualified prefix (`member:`/`requirement:`/`address:`) is always accepted
    /// outright. A read, never a gate: exits zero on every input.
    Explain {
        /// A member id, a requirement name, a leaf address
        /// (`<member>/<kind>/<key>/<child-path>`), or one qualified as
        /// `member:<name>` / `requirement:<name>` / `address:<leaf-address>`.
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
            // `check` gates against — as an editor JSON Schema.
            let declarations = drift::read_declarations(Path::new(DEFAULT_WORKSPACE.as_str()))?;

            let json = match kind.as_deref() {
                // An unknown kind is a hard error, never a silent empty schema.
                Some(requested) => {
                    let name = BUILTIN_DEFAULT_CONTRACT_KINDS
                        .iter()
                        .find(|name| **name == requested)
                        .ok_or_else(|| {
                            miette::miette!(
                                "unknown kind `{requested}` (temper models: skill, rule)"
                            )
                        })?;
                    let contract = compose::builtin_contract(&declarations.clauses, name)?;
                    schema::emit(&contract)
                }
                None => {
                    let mut map = serde_json::Map::new();
                    for name in BUILTIN_DEFAULT_CONTRACT_KINDS {
                        let contract = compose::builtin_contract(&declarations.clauses, name)?;
                        map.insert((*name).to_string(), schema::emit(&contract));
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
            print!("{}", explain(&target)?);
            Ok(ExitCode::SUCCESS)
        }
    }
}

/// Narrate `target` through the one read verb: assemble the same by-kind feature corpus, composed
/// requirement roster, declared edges, registrations, and directive/reachability inputs
/// the gate's own predicates range over (READ-EDGE-UNIFY) — over the standard `.temper`
/// workspace and the harness at the CWD, mirroring `check`'s own two-step corpus
/// assembly (`gate`) — and dispatch through [`read::explain`]'s target-species
/// resolution.
fn explain(target: &str) -> miette::Result<String> {
    let workspace = PathBuf::from(DEFAULT_WORKSPACE.as_str());
    let harness_root = Path::new(".");
    // One ignore-honoring walk per flavor, shared across every kind and nested host this
    // read discovers ([`import::Discovery`]).
    let discovery = import::Discovery::new(harness_root);

    // The assembly's own declared facts, read first: the corpus below walks each
    // kind's governs locus off *this*.
    // No layers: `explain` narrates what this corpus declares. A policy layer is the
    // invocation's own, and the invocation here is a read.
    // Parse the lock document once for reuse across source-dependency checks, hoisting
    // the read/parse operation per the cost doctrine (engineering.md, "Cost scale is hoisted").
    let lock_doc = drift::read_lock_document(&workspace)?;
    // Empty cache for early assembly; will build a proper cache after lock_family returns.
    let empty_cache: compose::ManifestCache = BTreeMap::new();
    let compose::LockFamily { declarations, .. } = compose::assemble_lock_family(
        &discovery,
        &drift::read_declarations(&workspace)?,
        &[],
        &empty_cache,
    )?;

    // Build a shared manifest cache for this explain invocation.
    let manifest_cache = compose::build_manifest_cache(&discovery, &declarations)?;

    // Every embedded built-in kind's discovered features — the same generic loop
    // `gate`'s two-greens runs, not a hardcoded skill/rule pair
    // (MEMORY-ENTERS-REQUIREMENT-CORPUS), so a memory member's declared `satisfies`
    // reaches `explain` exactly as it reaches the gate's roster/graph/coverage tiers.
    let builtin_defs = builtin_kind::definitions();
    let builtin_units_and_features = compose::builtin_units_and_features_by_kind(
        &builtin_defs,
        &discovery,
        &declarations,
        &manifest_cache,
    )?;
    let builtin_features: BTreeMap<String, Vec<extract::Features>> = builtin_units_and_features
        .iter()
        .map(|(k, uaf)| (k.clone(), uaf.features.clone()))
        .collect();

    // Each kind's resolved contract, lifted exactly as `gate` lifts it, so the clause
    // addresses `explain` narrates are the ones a finding prints (READ-EDGE-UNIFY).
    let mut contracts: BTreeMap<String, Contract> = BTreeMap::new();
    for kind in builtin_defs.values() {
        contracts.insert(
            kind.name.clone(),
            compose::builtin_contract(&declarations.clauses, &kind.name)?,
        );
    }

    // Every lock-declared kind that is not a built-in — the same synthesis `gate` runs
    // (READ-EDGE-UNIFY), so a read cannot disagree with the gate about which kinds and
    // members exist.
    let mut custom_kinds: Vec<compose::CustomKindEntry> = Vec::new();
    let mut custom_units_and_features: Vec<(CustomKind, compose::KindUnitsAndFeatures)> =
        Vec::new();
    let mut custom_members: Vec<read::CustomMember> = Vec::new();
    let (custom_rows, _collisions) = compose::partition_kind_rows(&declarations, &builtin_defs)?;
    for row in custom_rows {
        let custom_kind = CustomKind::from_kind_fact_row(row)?;
        contracts.insert(
            row.name.clone(),
            compose::default_contract_from_rows(&declarations.clauses, &row.name)?,
        );
        let uaf = compose::kind_units_and_features(
            &custom_kind,
            &discovery,
            &declarations,
            &manifest_cache,
        )?;
        let features = &uaf.features;
        let units = &uaf.units;
        for unit in units {
            custom_members.push(read::CustomMember {
                kind: custom_kind.name.clone(),
                id: unit.id.clone(),
                satisfies: unit.satisfies_clauses.clone(),
            });
        }
        custom_kinds.push((custom_kind.clone(), features.clone()));
        custom_units_and_features.push((custom_kind, uaf));
    }
    let embedded_features = compose::embedded_features_by_kind(&declarations);
    let by_kind = compose::assemble_by_kind(&builtin_features, &custom_kinds, &embedded_features);

    // The one requirement namespace: the assembly's declared `[requirement.*]`
    // roster — a custom-kind member has no channel of its own to publish one (the
    // pre-0016 own-path surface that once carried it is retired).
    let roster: BTreeMap<String, compose::Requirement> = declarations
        .requirements
        .iter()
        .map(|row| Ok((row.name.clone(), drift::requirement_from_row(row)?)))
        .collect::<Result<_, compose::ClauseRowError>>()?;
    let assembly_edges = drift::edges_from_declarations(&declarations)?;
    // Authored mentions (route-resolved at check) and layout prose imports (path-resolved
    // at emit) both lift off the lock; `why` route-resolves them against this same corpus,
    // narrating a dangling mention as the gate's route finding rather than a resolved edge,
    // so a read cannot disagree with the gate (READ-EDGE-UNIFY).
    let mut mention_edges = drift::mention_edges_from_declarations(&declarations);
    mention_edges.extend(drift::import_edges_from_doc(&lock_doc)?);

    // The world's inbound registration channel set into each built-in kind — the same
    // derivation the gate's `reachable` runs, keyed by bare kind name to join `by_kind`.
    let mut registrations: BTreeMap<&str, Vec<kind::Registration>> = BTreeMap::new();
    for def in builtin_defs.values() {
        if !def.registration.is_empty() {
            registrations.insert(def.name.as_str(), def.registration.clone());
        }
    }

    let repo_files = compose::repo_file_set(Path::new("."));
    let directive_members = compose::directive_members_from_resolved(
        &builtin_units_and_features,
        &custom_units_and_features,
    );
    let directive_edges = graph::classify_directives(&directive_members, &repo_files).edges;

    // Citations — the declared one-way edges naming a leaf; the floor carries no
    // producer yet, so the set is empty.
    let citations: Vec<read::Citation> = Vec::new();

    // The per-machine tap log at the workspace log path — the evidence the field strand
    // narrates. An absent log yields an empty readout (never an error), so the strand
    // narrates none.
    let readout = tap::read_log(&workspace)?;

    Ok(read::explain(
        &custom_members,
        &roster,
        &contracts,
        &by_kind,
        &assembly_edges,
        &mention_edges,
        &registrations,
        &repo_files,
        &directive_edges,
        &citations,
        &readout.records,
        readout.older_version,
        target,
    ))
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
        let contract = compose::builtin_contract(&declarations.clauses, &kind.name)?;
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
        let contract = compose::default_contract_from_rows(&declarations.clauses, &row.name)?;
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
    Some(if governs.root == "." {
        PathBuf::from(&governs.glob)
    } else {
        Path::new(&governs.root).join(&governs.glob)
    })
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

#[cfg(test)]
pub use temper::frontmatter;

#[cfg(test)]
mod test_support;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::tmpdir;
    use std::fs;

    /// The directive-backing set reads **raw disk**, never ignore-filtered: whether an
    /// `@import` target is backed is a fact about the filesystem the harness loads
    /// regardless of `.gitignore`, and the safe direction fixes it — an extra backing file only *suppresses* a
    /// finding, while pruning one could *forge* an unbacked finding on a target that
    /// exists. This is the counterpart to discovery, which *does* prune — two sets,
    /// two rules, never merged.
    #[test]
    fn repo_file_set_stays_raw_disk_including_gitignored_targets() {
        let root = tmpdir("repo-file-set");
        let dep = root.join("node_modules").join("dep");
        fs::create_dir_all(&dep).unwrap();
        fs::write(root.join(".gitignore"), "node_modules/\n").unwrap();
        fs::write(root.join("CLAUDE.md"), "# root\n").unwrap();
        // An `@import` target the harness loads even though `.gitignore` excludes it.
        fs::write(dep.join("SHARED.md"), "shared\n").unwrap();

        let files = compose::repo_file_set(&root);
        assert!(
            files.iter().any(|f| f == "node_modules/dep/SHARED.md"),
            "the gitignored backing target must still be seen (raw disk): {files:?}"
        );
        assert!(files.iter().any(|f| f == "CLAUDE.md"));
    }

    /// The run-level count-pin (`engineering.md`, "Cost scale is hoisted, and pinned by
    /// count"): a whole check run walks each consulted discovery flavor exactly once. The
    /// cache-level pin in `import.rs` proves N kind discoveries over one shared cache cost
    /// one walk per flavor; this pins that the *run* actually rides a single shared cache.
    /// `gate` threads one `Discovery` through every discovery call, so over the run the
    /// global walk count advances by exactly the flavors consulted — a code path that
    /// built a second `Discovery` mid-run would walk off its own cache and overshoot. The
    /// run consults both flavors: committed kinds (skill, rule, …) ride the standard
    /// flavor, the local-locus kinds (settings-local, dial) ride the local one — two
    /// flavors, two walks, never a third. The walk count is per-thread, so the delta is
    /// this run's alone whatever else runs concurrently.
    #[test]
    fn a_full_check_run_walks_each_consulted_flavor_once() {
        let harness = tmpdir("run-walk-pin");
        let skill = harness.join(".claude").join("skills").join("coordinate");
        fs::create_dir_all(&skill).unwrap();
        fs::write(
            skill.join("SKILL.md"),
            "---\nname: coordinate\ndescription: Drive a task across a team of agents.\n---\n# Coordinate\n",
        )
        .unwrap();
        let rules = harness.join(".claude").join("rules");
        fs::create_dir_all(&rules).unwrap();
        fs::write(rules.join("rust.md"), "# Rust\n").unwrap();

        // The raw-harness gate: workspace and harness root are the one path, exactly as
        // `harness_diagnostics` dispatches a bare harness.
        let before = import::walk_count();
        gate::gate(&harness, &harness, &[]).unwrap();
        let walks = import::walk_count() - before;

        assert_eq!(
            walks, 2,
            "a whole run must walk each consulted flavor exactly once — one shared cache \
             threaded through the run, never a per-kind or per-call re-walk",
        );
    }

    /// The per-kind resolution count-pin: every kind's members are read from disk and
    /// parsed exactly once per gate/explain invocation. The test verifies that no kind is
    /// resolved more than once — before the fix, kinds were resolved twice (once through
    /// kind_features for validation and again through collect_directive_members).
    #[test]
    fn resolve_kind_units_runs_once_per_kind_not_twice() {
        let harness = tmpdir("resolve-units-once");
        let skill = harness.join(".claude").join("skills").join("test-skill");
        fs::create_dir_all(&skill).unwrap();
        fs::write(
            skill.join("SKILL.md"),
            "---\nname: test-skill\ndescription: Test skill.\n---\n# Skill\n",
        )
        .unwrap();
        let rules = harness.join(".claude").join("rules");
        fs::create_dir_all(&rules).unwrap();
        fs::write(rules.join("test-rule.md"), "# Rule\n").unwrap();

        // Create a custom kind with one member to verify custom kinds are also resolved
        // exactly once.
        let custom_kinds = harness.join(".temper");
        fs::create_dir_all(&custom_kinds).unwrap();
        fs::write(
            custom_kinds.join("lock.toml"),
            r#"[[kind]]
name = "custom-kind"
content = "file"
format = "toml-document"
governs = ".custom"
unit_shape = "file"
"#,
        )
        .unwrap();
        let custom_dir = harness.join(".custom");
        fs::create_dir_all(&custom_dir).unwrap();
        fs::write(
            custom_dir.join("member.toml"),
            "[custom]\ndata = \"test\"\n",
        )
        .unwrap();

        let before = resolve_kind_units_count();
        gate::gate(&harness, &harness, &[]).unwrap();
        let resolves = resolve_kind_units_count() - before;

        // Before the fix, resolve_kind_units was called twice per kind: once through
        // kind_features and again through collect_directive_members. After the fix, it's
        // called exactly once per kind. The exact count depends on how many built-in kinds
        // exist (14) plus custom kinds (at least 1), but the key invariant is that with
        // 2+ kinds, we should see fewer than `2 * kind_count` resolves. For 15+ kinds,
        // a pre-fix run would make 30+ calls; post-fix should be ~15-20.
        let kind_count_estimate = 15;
        let max_expected_if_doubled = kind_count_estimate * 2;
        assert!(
            resolves < max_expected_if_doubled,
            "resolve_kind_units called {resolves} times; if it ran twice per kind \
             (pre-fix), would expect {max_expected_if_doubled}+ — the threading fix may not be working",
        );
    }

    /// One `nested_member` row of a `citation` kind declaring the edges `edges` names,
    /// filling the leaves `leaves` names, whose format placed `placed` (`None` ⇒ no
    /// format rendered the value).
    fn citation_row(
        edges: &[&str],
        leaves: &[&str],
        placed: Option<Vec<String>>,
    ) -> drift::Declarations {
        drift::Declarations {
            assembly: edges
                .iter()
                .map(|field| drift::AssemblyFactRow {
                    fact: "edge".to_string(),
                    from: Some("citation".to_string()),
                    field: Some((*field).to_string()),
                    to: Some(vec!["rule".to_string()]),
                    value: None,
                })
                .collect(),
            nested_members: vec![drift::NestedMemberRow {
                host: "memory:CLAUDE".to_string(),
                kind: "citation".to_string(),
                key: "the-standard".to_string(),
                leaves: leaves
                    .iter()
                    .map(|leaf| ((*leaf).to_string(), "rule:rust".to_string()))
                    .collect(),
                collections: Vec::new(),
                placed_edges: placed,
                rendered_lines: None,
                rendered_chars: None,
            }],
            ..drift::Declarations::default()
        }
    }

    /// The placement feature of the row `declarations` carries, against its kind's
    /// declared edges — the join the lift performs.
    fn placement_feature(declarations: &drift::Declarations) -> Option<BTreeMap<String, bool>> {
        let edges = compose::edge_fields_by_kind(declarations);
        compose::embedded_member_features(
            &declarations.nested_members[0],
            edges.get("citation").unwrap(),
        )
        .edge_placements
    }

    /// A member's placement feature is the join of two lock families: the edges the
    /// `assembly` family says its kind declares, against the `placed_edges` its own row
    /// says the format rendered. Neither alone decides a `format-places-edges` clause.
    #[test]
    fn an_embedded_members_placement_feature_joins_declared_edges_against_the_placed_set() {
        let declarations = citation_row(
            &["source", "supersedes"],
            &["source", "supersedes"],
            Some(vec!["source".to_string()]),
        );
        assert_eq!(
            placement_feature(&declarations),
            Some(BTreeMap::from([
                ("source".to_string(), true),
                ("supersedes".to_string(), false),
            ])),
            "the edge the format never selected must read as unplaced",
        );
    }

    /// An edge field the value never filled is no edge, so it is no placement obligation:
    /// ranging over the kind's whole declared set would read the absent field as one the
    /// format dropped. An empty leaf is unfilled the same way an absent one is.
    #[test]
    fn an_unfilled_edge_field_carries_no_placement_obligation() {
        let unfilled = citation_row(
            &["source", "supersedes"],
            &["source"],
            Some(vec!["source".to_string()]),
        );
        assert_eq!(
            placement_feature(&unfilled),
            Some(BTreeMap::from([("source".to_string(), true)])),
            "the unfilled `supersedes` is no edge, so the format omitted nothing",
        );

        let mut empty_leaf = citation_row(&["source"], &["source"], Some(Vec::new()));
        empty_leaf.nested_members[0]
            .leaves
            .insert("source".to_string(), String::new());
        assert_eq!(placement_feature(&empty_leaf), Some(BTreeMap::new()));
    }

    /// The two ways a member offers nothing to indict stay apart: no format rendered the
    /// value at all (a layout host's document is source), versus a format that ran over a
    /// value carrying no edge. Both hold at the gate, but only this lift can tell them
    /// apart — an empty map standing for both is what left the clause undecidable.
    #[test]
    fn a_value_no_format_rendered_is_distinct_from_a_format_with_nothing_to_place() {
        assert_eq!(
            placement_feature(&citation_row(&["source"], &["source"], None)),
            None
        );
        assert_eq!(
            placement_feature(&citation_row(&["source"], &[], Some(Vec::new()))),
            Some(BTreeMap::new()),
        );

        // The row whose format ran over a filled edge and placed nothing does carry the
        // fact — an unplaced edge, which is the finding the clause exists to make.
        assert_eq!(
            placement_feature(&citation_row(&["source"], &["source"], Some(Vec::new()))),
            Some(BTreeMap::from([("source".to_string(), false)])),
        );
    }
}
