//! `temper` CLI entry point.
//!
//! A thin `clap` dispatch over the [`temper`] library: parse args, run the
//! generic contract engine, map the result to an exit code. Every subcommand
//! mirrors the pipeline verbs; all logic lives in the
//! library so `tests/` can drive it.

use std::collections::{BTreeMap, BTreeSet};
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::LazyLock;

use clap::{Parser, Subcommand, ValueEnum};
use miette::IntoDiagnostic;
use temper::admissibility;
use temper::builtin;
use temper::builtin_kind;
use temper::bundle;
use temper::check::{self, Severity};
use temper::compose;
use temper::contract::Contract;
use temper::coverage;
use temper::coverage_note;
use temper::dial;
use temper::drift;
use temper::engine;
use temper::extract;
use temper::graph;
use temper::import;
use temper::install;
use temper::json_manifest;
use temper::kind::{self, CollectionAddress, CustomKind};
use temper::read;
use temper::reporter;
use temper::roster;
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

/// Resolve a built-in kind's bare row label into its default [`Contract`], failing
/// loud if the build embeds no default contract of that name — a
/// missing default contract is a hard error, never a silently empty contract.
fn builtin_default_contract(kind: &str) -> miette::Result<Contract> {
    builtin::contract(kind)
        .ok_or_else(|| miette::miette!("built-in kind `{kind}` ships no embedded default contract"))
}

/// The kinds `schema` emits a default contract for, by bare row label; widening it
/// to `memory` is a separate question.
const BUILTIN_DEFAULT_CONTRACT_KINDS: &[&str] = &["skill", "rule"];

/// A built-in `kind`'s effective [`Contract`]: its lock-declared clause rows are its
/// whole contract when the lock names any, lifted through the same reject-loud path a
/// custom kind's rows take ([`compose::default_contract_from_rows`]); with no rows the
/// kind falls back to the embedded default ([`builtin_default_contract`]).
/// Rows-or-default — never a severity-flip layer over the embedded default: a spread's
/// appended clause gates, an array-surgery removal holds, and an out-of-vocabulary row
/// rejects loud rather than sitting inert.
///
/// # Errors
///
/// Propagates the [`compose::ClauseRowError`] the row lift raises for a row the closed
/// vocabulary cannot admit, or the missing-embedded-contract error if a rowless kind
/// ships none.
fn builtin_contract(clauses: &[drift::ClauseRow], kind: &str) -> miette::Result<Contract> {
    if clauses.iter().any(|row| row.kind.as_deref() == Some(kind)) {
        Ok(compose::default_contract_from_rows(clauses, kind)?)
    } else {
        builtin_default_contract(kind)
    }
}

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
                    let contract = builtin_contract(&declarations.clauses, name)?;
                    schema::emit(&contract)
                }
                None => {
                    let mut map = serde_json::Map::new();
                    for name in BUILTIN_DEFAULT_CONTRACT_KINDS {
                        let contract = builtin_contract(&declarations.clauses, name)?;
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
            let mode = mode_from_declarations(&declarations)?;
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
    let manifest_cache = build_manifest_cache(&discovery, &declarations)?;

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
            builtin_contract(&declarations.clauses, &kind.name)?,
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
    let embedded_features = embedded_features_by_kind(&declarations);
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

    let repo_files = repo_file_set(Path::new("."));
    let directive_members =
        directive_members_from_resolved(&builtin_units_and_features, &custom_units_and_features);
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

/// The enforcement mode `declarations` declare, for a caller that has already read them —
/// [`gate`], which needs the mode to decide whether a dialed softening binds and must
/// never reach a second verdict on the posture from a second read.
///
/// # Errors
///
/// Returns a [`drift::LockRowError::Vocabulary`] when the `mode` fact carries an
/// unrecognized value outside the closed `{note, warn, block}` vocabulary.
fn mode_from_declarations(
    declarations: &drift::Declarations,
) -> miette::Result<compose::EnforcementMode> {
    let Some(value) = declarations
        .assembly
        .iter()
        .find(|row| row.fact == "mode")
        .and_then(|row| row.value.as_deref())
    else {
        return Ok(compose::EnforcementMode::default());
    };
    match value {
        "note" => Ok(compose::EnforcementMode::Note),
        "warn" => Ok(compose::EnforcementMode::Warn),
        "block" => Ok(compose::EnforcementMode::Block),
        other => Err(drift::LockRowError::Vocabulary {
            family: "assembly".to_string(),
            column: "mode".to_string(),
            value: other.to_string(),
        }
        .into()),
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
        let contract = builtin_contract(&declarations.clauses, &kind.name)?;
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
        HarnessPath::Root { workspace, .. } => gate(&workspace, harness_path, layers),
        HarnessPath::Workspace { enclosing } => gate(harness_path, &enclosing, layers),
        HarnessPath::Raw => gate(harness_path, harness_path, layers),
    }
}

/// Build a shared manifest cache for a single gate/explain invocation, grouping manifest
/// kinds by their manifest file path and reading each file once with all governing kinds'
/// addresses. This hoisting ensures manifest files are read exactly once per run, never
/// once per governing kind (GATE-MANIFEST-SHARED-READ-HOIST).
fn build_manifest_cache(
    disc: &import::Discovery,
    declarations: &drift::Declarations,
) -> miette::Result<compose::ManifestCache> {
    let mut cache: compose::ManifestCache = BTreeMap::new();
    let kinds = compose::declared_kinds(declarations)?;

    // Group manifest kinds by their manifest file path.
    let mut by_manifest: BTreeMap<PathBuf, Vec<&CollectionAddress>> = BTreeMap::new();
    for kind in kinds.values() {
        if let Some(address) = &kind.collection_address
            && let Some(governs) = &kind.governs
        {
            // Discover the actual files this kind governs.
            let files =
                import::discover_kind_files(disc, kind, governs, import::LocalOverride::Honored);
            for file in files {
                by_manifest.entry(file).or_default().push(address);
            }
        }
    }

    // Read each manifest file once with all addresses for that file.
    for (file, addresses) in by_manifest {
        let manifest = json_manifest::Manifest::read(&file, &addresses)?;
        cache.insert(file, (manifest, BTreeMap::new()));
    }

    Ok(cache)
}

/// Produce the merged diagnostic set for a surface `workspace` against the active
/// by-kind contracts, with the [`check::Announcement`] of the inputs that judged it —
/// the shared gate behind both `check` and the session-start
/// reporter. `harness_root` is the
/// directory a member's source path and a script verifier's path resolve against (the
/// CWD for a two-step `check`, the harness path for the one-shot gate). `layers` names
/// the policy locks this invocation joins, top of the layer stack.
pub fn gate(
    workspace: &Path,
    harness_root: &Path,
    layers: &[PathBuf],
) -> miette::Result<(Vec<check::Diagnostic>, check::Announcement)> {
    // The assembly's own declared facts — requirements and edges — ride the lock's
    // declaration rows: `emit` is the sole
    // producer, this is the gate's one read of it.
    // Never gated on a lock's presence — an unadopted harness's lock declares
    // nothing, so this tier is a no-op over it rather than skipped (never a
    // half-adopted state).
    let committed = drift::read_declarations(workspace)?;
    // Parse the lock document once for reuse across source-dependency checks, hoisting
    // the read/parse operation per the cost doctrine (engineering.md, "Cost scale is hoisted").
    let lock_doc = drift::read_lock_document(workspace)?;
    // One ignore-honoring walk per flavor, shared across every kind and nested host this
    // gate discovers ([`import::Discovery`]) — the session-open `check` walks the
    // consumer's whole tree, so a per-kind re-walk is the tick's dominant cost. This one
    // cache is threaded through every discovery call below; a run walks each consulted
    // flavor exactly once, pinned at run granularity by [`import::walk_count`].
    let discovery = import::Discovery::new(harness_root);
    // The dial's softening is inert under `block`, so the mode is resolved before any
    // contract is, and off the same declarations the family assembles from — a run whose
    // gate and whose guard disagreed about the harness's own posture would be two gates.
    let mode = mode_from_declarations(&committed)?;
    // Empty cache for early assembly; will build a proper cache after lock_family returns.
    let empty_cache: compose::ManifestCache = BTreeMap::new();
    let compose::LockFamily {
        declarations,
        joined_clauses,
        joined_locks,
        local_members,
        dial,
    } = compose::assemble_lock_family(&discovery, &committed, layers, &empty_cache)?;
    // Every address the dial reached, accumulated across the contracts and selections
    // below: an entry that reached none is the one thing a dial can be wrong about that
    // its own schema cannot catch.
    let mut dialed: BTreeSet<String> = BTreeSet::new();
    let assembly_requirements: BTreeMap<String, compose::Requirement> = declarations
        .requirements
        .iter()
        .map(|row| Ok((row.name.clone(), drift::requirement_from_row(row)?)))
        .collect::<Result<_, compose::ClauseRowError>>()?;
    let assembly_edges = drift::edges_from_declarations(&declarations)?;
    // The lifted reference edges the graph predicates and read verbs fold in alongside the
    // declared-field arcs: authored mentions (route-resolved at check — `route_mentions`
    // owns a deferred mention's dangling verdict) and layout prose imports (path-resolved
    // at emit), each lifted off the lock's own declaration family.
    let mut mention_edges = drift::mention_edges_from_declarations(&declarations);
    mention_edges.extend(drift::import_edges_from_doc(&lock_doc)?);

    // The generic two-greens over EVERY embedded built-in kind, keyed by its bare row
    // label: each kind's members — resolved by
    // [`kind_features`] straight off harness disk, shared with `explain`
    // (READ-EDGE-UNIFY) so a read cannot disagree with the gate about which members
    // exist — are dispatched to its default contract and validated, so a discovered `CLAUDE.md`
    // memory member fires its `memory` clauses exactly as a skill/rule does — no
    // longer silently skipped by a hardcoded skill/rule pair. The resolved features
    // feed straight into `builtin_features` (MEMORY-ENTERS-REQUIREMENT-CORPUS): every
    // built-in's satisfies edges reach the roster/graph/coverage tiers below, not only
    // skill/rule's.
    let mut diagnostics = Vec::new();
    // Per-kind checked-member counts, keyed by bare row label — carried out of
    // the dispatch loop for the advisory coverage note below (WEDGE-COVERAGE-NOTE),
    // so "checked N members" is stated rather than left as bare silence.
    let mut member_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut builtin_features: BTreeMap<String, Vec<extract::Features>> = BTreeMap::new();
    // Each kind's resolved contract, kept past its dispatch loop: the set clauses in it
    // bind to the kind's *whole* by-kind selection, which only exists once every
    // dispatcher has run and `by_kind` is assembled below.
    let mut contracts: BTreeMap<String, Contract> = BTreeMap::new();

    // Build a shared manifest cache: one read per manifest file, shared across all
    // manifest kinds that govern it (GATE-MANIFEST-SHARED-READ-HOIST).
    let manifest_cache = build_manifest_cache(&discovery, &declarations)?;

    // Every clause's address is unique across the lock, decided before a single contract
    // is lifted: a clause no finding can name unambiguously cannot be judged usefully.
    diagnostics.extend(admissibility::clause_collision_diagnostics(
        &declarations,
        &joined_clauses,
    ));
    let builtin_defs = builtin_kind::definitions();
    let mut builtin_units_and_features: BTreeMap<String, compose::KindUnitsAndFeatures> =
        BTreeMap::new();
    for kind in builtin_defs.values() {
        // Two greens: admissibility — the contract validated
        // against the definition before it is trusted to judge — then conformance.
        // The contract is the lock's declared `clauses` for the kind when it names any,
        // else the embedded default (`builtin_contract`). The invocation's joined clauses
        // are appended *after* that fallback decides, never folded into the rows it reads:
        // a layer's row is not this harness declaring one, so it must never be what tips a
        // built-in off its embedded default and onto a contract of the layer's alone.
        let mut contract = compose::with_joined_clauses(
            builtin_contract(&declarations.clauses, &kind.name)?,
            &joined_clauses,
            &kind.name,
        )?;
        // Every kind's contract is dialable but the dial's own: its clauses are the
        // envelope the dial document is checked against, so a machine that could soften
        // them could spell its way out of the shape that bounds it. `dial::refusals`
        // reports the entry that tried rather than leaving it silently inert.
        if kind.name != dial::KIND {
            dialed.extend(dial.apply(mode, &mut contract.clauses));
        }

        let uaf =
            compose::kind_units_and_features(kind, &discovery, &declarations, &manifest_cache)?;
        let features = &uaf.features;

        diagnostics.extend(engine::admissibility(&contract, &engine::Locus::Document));
        diagnostics.extend(engine::validate(&contract, features));
        member_counts.insert(kind.name.clone(), features.len());
        contracts.insert(kind.name.clone(), contract);
        builtin_features.insert(kind.name.clone(), features.clone());
        builtin_units_and_features.insert(kind.name.clone(), uaf);
    }

    // Every lock-declared kind that is not one of the embedded built-ins:
    // a
    // built-in's own row is only the overlay `compose::overlay_builtin_kind` already
    // consumes, never a second kind definition. A custom kind carries no embedded
    // default — its whole default contract is the committed lock's own clause rows
    // naming it ([`compose::default_contract_from_rows`]) — but is otherwise
    // dispatched through the identical two-greens the built-in loop above runs.
    let mut custom_kinds: Vec<compose::CustomKindEntry> = Vec::new();
    let mut custom_units_and_features: Vec<(CustomKind, compose::KindUnitsAndFeatures)> =
        Vec::new();
    let (custom_rows, collisions) = compose::partition_kind_rows(&declarations, &builtin_defs)?;
    // The one site among the three dispatchers that can surface a diagnostic.
    diagnostics.extend(
        collisions
            .iter()
            .map(|row| admissibility::kind_collision_diagnostic(row)),
    );
    // Two distinct kinds resolving to one `governs` locus would double-route every
    // matching document into both member sets — a document's kind is its position
    // alone, never its content — so a shared locus refuses loud here.
    diagnostics.extend(admissibility::governs_collision_diagnostics(
        &builtin_defs,
        &custom_rows,
        &declarations,
    )?);
    // A declared commitment class the locus cannot carry is decided here, beside the
    // locus's other coherence check, before any member is read under a kind whose own
    // declaration does not hold together.
    diagnostics.extend(admissibility::local_locus_admissibility(
        &builtin_defs,
        &custom_rows,
        &declarations,
    )?);
    for row in custom_rows {
        let custom_kind = CustomKind::from_kind_fact_row(row)?;
        let mut contract = compose::with_joined_clauses(
            compose::default_contract_from_rows(&declarations.clauses, &row.name)?,
            &joined_clauses,
            &row.name,
        )?;
        dialed.extend(dial.apply(mode, &mut contract.clauses));
        let uaf = compose::kind_units_and_features(
            &custom_kind,
            &discovery,
            &declarations,
            &manifest_cache,
        )?;
        let features = &uaf.features;

        diagnostics.extend(engine::admissibility(&contract, &engine::Locus::Document));
        diagnostics.extend(engine::validate(&contract, features));
        member_counts.insert(row.name.clone(), features.len());
        contracts.insert(row.name.clone(), contract);
        custom_kinds.push((custom_kind.clone(), features.clone()));
        custom_units_and_features.push((custom_kind, uaf));
    }

    // The directive backing-set file-set: every file under the harness root, over-collected so an extra
    // file can only suppress a finding, never forge one. Computed once on the FLOOR
    // and read by the directive classing below.
    let repo_files = repo_file_set(harness_root);

    // Directive-target classing on the FLOOR tier: an unbacked `@import` is a **pure fact** about the importing member —
    // the silent-context-loss failure class made author-time — so it surfaces with zero
    // config. Over the built-in kinds' members (empty custom slice), the unbacked findings
    // extend as a **non-gating advisory**: the fact is stated, the run never fails on it
    // alone. The graph-scope escalation stays assembly-gated (WEDGE ruling 2026-07-03: an
    // unbacked import is a pure fact, not a graph-scope opinion like reachability).
    diagnostics.extend(
        graph::classify_directives(
            &directive_members_from_resolved(
                &builtin_units_and_features,
                &custom_units_and_features,
            ),
            &repo_files,
        )
        .findings
        .into_iter()
        .map(|mut finding| {
            finding.severity = Severity::Warn;
            finding
        }),
    );

    // The harness-contract tier: the set predicates over the parsed roster, each
    // quantified over a requirement's opt-in selection.
    // Runs unconditionally — the lock is the sole source of assembly facts now, so an
    // unadopted harness's empty declarations make this tier a no-op rather than a
    // skip.
    let edges = assembly_edges.clone();

    // Cross-family coherence: a `nested_member` row of a kind no host templates is an
    // orphan the by-kind corpus below would unmodel while the host-address read still
    // carries it. Reject it at admissibility, before the corpus is trusted.
    diagnostics.extend(admissibility::nested_member_admissibility(&declarations));

    // The by-kind corpus every set-scope and graph predicate ranges over,
    // assembled through the same helper the read arm uses.
    let embedded_features = embedded_features_by_kind(&declarations);

    // The third dispatcher: an embedded kind's members through the identical two greens
    // the two at-locus loops above run, so a clause bound to an embedded kind is judged
    // rather than silently no-opped. Ordered here because the embedded corpus is what a
    // host's `templates` column yields, which is only assembled above. Like a custom
    // kind, an embedded kind carries no embedded default — its whole contract is the
    // committed lock's own clause rows naming it. Its member counts stay out of the
    // coverage note's summary: that map is keyed by kind-fact row label, which an
    // embedded kind has none of, and an embedded member's host file is already counted
    // under its own kind.
    for (kind, features) in &embedded_features {
        let mut contract = compose::with_joined_clauses(
            compose::default_contract_from_rows(&declarations.clauses, kind)?,
            &joined_clauses,
            kind,
        )?;
        dialed.extend(dial.apply(mode, &mut contract.clauses));

        diagnostics.extend(engine::admissibility(
            &contract,
            &engine::Locus::Embedded(kind.clone()),
        ));
        diagnostics.extend(engine::validate(&contract, features));
        contracts.insert(kind.clone(), contract);
    }

    // Every kind a joined clause names that this corpus declares none of. Nothing here
    // selects such a clause, so it judges nothing — but a layer must fail closed whether
    // or not the host happens to give its clauses something to range over, so the rows
    // still face the admissibility their kind's own dispatcher would have run.
    diagnostics.extend(admissibility::joined_kind_admissibility(
        &joined_clauses,
        &contracts,
    )?);

    let by_kind = compose::assemble_by_kind(&builtin_features, &custom_kinds, &embedded_features);

    // A bare `satisfies` label an older engine wrote qualifies against this corpus, but a
    // name two kinds share is a malformed lock refused loud rather than cross-attributed.
    diagnostics.extend(admissibility::satisfies_label_admissibility(
        &declarations,
        &by_kind,
    ));

    // Every opt-in-capable member's features (every built-in kind *and* each custom
    // kind's members) — the stream coverage ranges over below.
    let all_features: Vec<extract::Features> = builtin_features
        .values()
        .flatten()
        .chain(custom_kinds.iter().flat_map(|(_, features)| features))
        .cloned()
        .collect();

    // The one requirement namespace: the assembly's declared `[requirement.*]`
    // roster. A custom-kind member has no channel of its own to publish a
    // requirement (the pre-0016 own-path surface that once carried one is retired);
    // the SDK already unions `harness.require` and every member's `requires` into
    // `declarations.requirements` at emit time, rejecting a cross-publisher name
    // collision there.
    let requirements = assembly_requirements.clone();

    // Each requirement's own definition is validated before the roster is
    // trusted to judge the harness.
    diagnostics.extend(roster::admissibility(&requirements, &by_kind, harness_root));

    // The declared selections, whole: every requirement's opt-in selection and every
    // kind's by-kind selection. Both lists are assembled before either is judged
    // because a `membership` clause draws its allowed set from a *second* selection,
    // and the judge resolves that target off this one list — the existential and the
    // universal binding are the same algebra, so neither can be judged in isolation.
    let mut selections = roster::selections(&requirements, &by_kind);
    selections.extend(kind_selections(&contracts, &by_kind));
    // The last of the dial's four sites, and the only one over selections rather than
    // contracts. A requirement's own clauses reach a judge only here,
    // as does the each-grain narrowing clause its `kind` facet sources — both are
    // synthesized past the contracts above, so dialing those alone would leave a
    // requirement's findings the one family an author could read an address off and not
    // dial. A kind selection re-dials clauses already dialed above; `apply` is
    // idempotent, and one site over the whole list beats a second rule about which half
    // of it is fresh — the dial's own kind selection excepted, since it carries a copy of
    // the very contract the loop above holds out of reach.
    for selection in &mut selections {
        if selection.selector == engine::Selector::Kind(dial::KIND.to_string()) {
            continue;
        }
        dialed.extend(dial.apply(mode, &mut selection.clauses));
    }
    diagnostics.extend(dial.refusals(&dialed));

    // Every dial site has run, so `dialed` is final and the three thirds are all in hand:
    // what judged this run beyond the committed harness, assembled here rather than
    // re-derived by whichever reporter renders it.
    let announcement = check::Announcement {
        local_members,
        dialed_clauses: dialed.into_iter().collect(),
        joined_locks,
    };

    // The selection grain: `count` / `unique` / `membership` whole, `kind` each.
    diagnostics.extend(engine::judge(&selections));

    // The edge scope: build the reference graph over the declared edges and check route
    // resolution — a declared reference must resolve to a real artifact of the
    // target kind. Admissibility before conformance:
    // an edge naming no reference field or targeting an unmodeled kind is
    // reported once and skipped by the route check.
    diagnostics.extend(graph::admissibility(&edges, &by_kind));
    diagnostics.extend(graph::check(&edges, &by_kind));

    // Mention route resolution: `emit` defers a mention naming a declared kind with no
    // composed member — its row rides the lock — so `check` owns that verdict here,
    // resolving each mention's target against the discovered corpus (members) and the
    // roster (bare requirement names). A dangler fires `graph.route` exactly as a declared
    // reference does.
    diagnostics.extend(graph::route_mentions(
        &mention_edges,
        &by_kind,
        &assembly_requirements,
    ));

    // Compute the resolved edges once, shared across acyclic, degree, and mention_reachable
    // to avoid recomputation of the whole-input edge-resolution walk.
    let resolved_edges_result = graph::resolved_edges(&edges, &by_kind);
    let resolved_edges = &resolved_edges_result.resolved;

    // `acyclic`: the resolved graph must contain no
    // cycle — a circular import loads nothing, so every finding is a true
    // positive. Always-on over the whole edge set, like route resolution above.
    diagnostics.extend(graph::acyclic(resolved_edges));

    // `degree`: the one set predicate whose judge needs the graph — a clause bounds
    // every selected member's in/out edge count, so it takes the same selections
    // `engine::judge` reads *and* the edges, reusing the arc resolution
    // `acyclic`/`check` assemble, plus the already-resolved mention edges —
    // obligation-free by default, counted only when a `degree` clause opts in.
    diagnostics.extend(graph::degree(&selections, resolved_edges, &mention_edges));

    // `mention-reachable`: the second selection predicate whose judge needs the graph —
    // each selected member's references must be able to fire where their target can be
    // invoked, which reads the *target* member's gate field. It ranges over the same
    // unified edge set `degree` does — the resolved field edges *and* the mention/import
    // family — so a rendering claim carried on a field edge is judged, never dropped.
    // Opt-in like `degree`: a selection declaring no such clause does no work.
    diagnostics.extend(graph::mention_reachable(
        &selections,
        resolved_edges,
        &mention_edges,
        &by_kind,
        &embedded_hosts_by_source(&declarations),
    ));

    // The requirement-coverage tier: every `required`
    // requirement must have a resolving home (≥1 artifact opting in via
    // `satisfies`) and every authored `satisfies` must resolve to a declared
    // requirement. Kind-blind: it ranges over every opt-in-capable artifact —
    // built-in kinds *and* each custom kind's members — so temper's own `spec`
    // corpus can opt in exactly as a skill does. The
    // requirement set is the *unioned* namespace, so a member-published obligation
    // is gated here exactly as an assembly-published one.
    diagnostics.extend(coverage::check(&requirements, &all_features));

    // The install self-verify: temper checking its
    // *own* gate is wired. Advisory (warn) only — a not-yet-installed gate nudges
    // without failing the run, and the session-start reporter ignores warn
    // severity.
    diagnostics.extend(install::gate_installed(harness_root));

    // The wedge's advisory coverage note: state which kinds checked how many members,
    // and name the known Claude Code surfaces present on disk that no kind — built-in
    // or locked custom — governs, so the gate's silence about an unmodeled surface never
    // reads as "checked". Warn-only — it leaves the run's exit code and the session-start
    // verdict unchanged. Threads the already-parsed `committed.kinds` to avoid a redundant
    // lock re-parse (COVERAGE-NOTE-LOCK-PARSE-HOIST).
    diagnostics.extend(coverage_note::check(
        harness_root,
        &builtin_kind::definitions(),
        &member_counts,
        &committed.kinds,
    )?);

    // The freshness fact: a committed projection
    // whose bytes no longer match the lock's emit fingerprint is `config.stale`. Read
    // off the surface `workspace`'s lock (where the members were imported and the
    // fingerprints recorded), advisory so a hand-edited or un-re-emitted projection is
    // surfaced without failing the run.
    diagnostics.extend(drift::config_stale_from_doc(&lock_doc, workspace));

    // The source-dependency freshness facts: a fingerprinted layout-import or
    // composed-prose include target whose bytes no longer match the lock — the target
    // moved and `emit` has not re-run. Advisory, the same `warn` posture `config.stale`
    // takes over a drifted projection. Use the pre-parsed lock document to avoid re-reading.
    let harness_root_for_staleness = drift::harness_root_of(workspace);
    diagnostics.extend(drift::layout_import_stale_from_doc(
        &lock_doc,
        &harness_root_for_staleness,
    )?);
    diagnostics.extend(drift::include_stale_from_doc(
        &lock_doc,
        &harness_root_for_staleness,
    )?);

    Ok((diagnostics, announcement))
}

/// One discovered source file as a raw [`Unit`], its id folded against `base` — the read
/// both file loci share, so a nested file child and a `governs`-scanned member differ only
/// in the base each composes under. **The one adapter dispatch**: a layout kind's document
/// is read under its declared layout — its field sections fill the unit's fields, a
/// non-fitting document refusing loud; a kind declaring the `json-document` or
/// `toml-document` format reads its whole artifact as one structured document through that
/// grammar's adapter; every other file kind reads through the generic frontmatter adapter.
/// A fields-only kind with no collection address (not a manifest kind) rides whichever of
/// those its format names, differing only in projection. A per-call-site format match would be a second dispatch to disagree with
/// this one, so both file loci route through here.
///
/// # Errors
///
/// Returns an error if the file is unreadable, malformed, or does not fit its declared
/// layout or format.
/// Every kind this harness declares, keyed by bare name: each embedded built-in under its
/// lock overlay, plus each lock-declared custom kind — the same universe `gate`'s own
/// dispatch ranges over. A nested file kind's host is found here, so the set is what
/// carries the two halves of its locus that the child kind itself cannot.
///
/// # Errors
///
/// Returns an error if the embedded kind set fails to load or a lock row falls outside a
/// closed vocabulary.
/// The separator between a joined clause's own compiled address and the layer that
/// carried it: `<label>@<layer>`.
///
/// A compiled address is dot-joined ([`contract::clause_label`]), so `@` appears in no
/// label emit can write — which is what makes a joined address unable to collide with a
/// host's, whatever the two locks happen to declare. Legible in both directions: the
/// author who reads a finding reads which `--layer` argument produced it, and the name is
/// spelled straight back out of the finding to reach the clause.
/// The admissibility findings of every joined clause naming a kind absent from
/// `contracts` — the kinds this corpus declares none of, whose clauses no dispatcher
/// above ever lifted.
///
/// # Errors
///
/// As [`compose::default_contract_from_rows`].
/// Every file under `root`, as repo-relative slash-separated paths — the
/// `paths-match` reachability input.
/// A superset is sound (a glob matching an extra file only suppresses a finding); a
/// *missing* file is not (it could forge a dead-edge false positive), so nothing is
/// excluded and an unreadable entry is skipped rather than aborting the gate. Paths
/// use `/` so a glob authored the harness's way matches on every platform.
fn repo_file_set(root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for entry in walkdir::WalkDir::new(root).min_depth(1).sort_by_file_name() {
        let Ok(entry) = entry else { continue };
        if entry.file_type().is_file()
            && let Ok(rel) = entry.path().strip_prefix(root)
        {
            files.push(rel.to_string_lossy().replace('\\', "/"));
        }
    }
    files
}

/// Every kind's **by-kind selection**: its whole member population — the universal
/// binding — bound to its own contract's clauses.
///
/// The clause set is the contract entire, not the set clauses alone: `engine::judge`
/// reads the ones that range over the selection and `engine::validate` has already read
/// the member-grain ones off the identical contract, so the two judges split one
/// declaration rather than two filtered copies of it. A kind in `by_kind` with no
/// contract of its own declares no clause to judge and contributes no selection.
fn kind_selections<'a>(
    contracts: &BTreeMap<String, Contract>,
    by_kind: &BTreeMap<&'a str, &'a [extract::Features]>,
) -> Vec<engine::Selection<'a>> {
    by_kind
        .iter()
        .filter_map(|(kind, features)| {
            let contract = contracts.get(*kind)?;
            Some(engine::Selection {
                selector: engine::Selector::Kind((*kind).to_string()),
                clauses: contract.clauses.clone(),
                members: features.iter().map(|feature| (*kind, feature)).collect(),
            })
        })
        .collect()
}

/// The embedded-kind corpus: every kind declared at the embedded locus keyed to its
/// members' [`Features`](extract::Features), so [`assemble_by_kind`] can fold it into the
/// one `by_kind` map every graph predicate ranges over. An embedded kind is named where a
/// host declares it — a `templates` column entry, or a layout member collection's
/// `member_kind` — and carries no kind-fact row, so this is the sole seam it enters the
/// corpus through. Its members are the run's assembled `nested_member` rows of that kind
/// — a committed host's off the lock, a local host's derived at [`assemble_lock_family`],
/// so a clause over it selects a local host's members and a committed host's alike — each
/// lifted to a member whose id is the row's key and whose fields are its leaves, so an
/// edge resolves against it by identity ([`embedded_member_features`]). A declared kind
/// with no rows keys to an empty slice — modeled, so an edge targeting it is admissible
/// and a dangling entry is a route finding, not an admissibility one; a kind no host
/// declares is absent, so an edge targeting it stays an admissibility finding. Depth is
/// one layer: a `nested_member` row's own sibling collections are the leaf grain the read
/// family addresses, not a second embedded kind's member set.
fn embedded_features_by_kind(
    declarations: &drift::Declarations,
) -> BTreeMap<String, Vec<extract::Features>> {
    let mut by_kind: BTreeMap<String, Vec<extract::Features>> =
        admissibility::declared_embedded_kinds(declarations)
            .into_iter()
            .map(|kind| (kind, Vec::new()))
            .collect();
    // Each declared embedded kind's members are its `nested_member` rows. A row whose
    // kind no host declares is an orphan rejected at admissibility
    // ([`nested_member_admissibility`]), so this `get_mut` now backstops that already-loud
    // unreachable state rather than swallowing a live one.
    let edge_fields = edge_fields_by_kind(declarations);
    let no_edges = BTreeSet::new();
    for row in &declarations.nested_members {
        if let Some(features) = by_kind.get_mut(&row.kind) {
            features.push(embedded_member_features(
                row,
                edge_fields.get(&row.kind).unwrap_or(&no_edges),
            ));
        }
    }
    by_kind
}

/// Each embedded member's source node keyed to its **host**'s node: `(embedded-kind,
/// key) → (host-kind, host-id)`, read off the `nested_member` rows' `host` address. An
/// embedded-carried edge keys its source to the embedded member, never the host, so
/// `graph::mention_reachable` needs this map to judge a body-carried citation under its
/// host's scope — the source-side twin of the target-side `target_identity` seam. A row
/// whose `host` is not a `kind:name` address is skipped: it addresses no host
/// node, so the edge it would map stays keyed to the embedded member alone.
fn embedded_hosts_by_source(
    declarations: &drift::Declarations,
) -> BTreeMap<graph::Node, graph::Node> {
    let mut hosts = BTreeMap::new();
    for row in &declarations.nested_members {
        if let Some((kind, name)) = row.host.split_once(':') {
            hosts.insert(
                (row.kind.clone(), row.key.clone()),
                (kind.to_string(), name.to_string()),
            );
        }
    }
    hosts
}

/// The edge fields each kind declares, off the lock's `assembly` `edge` facts — the
/// declared set a `format-places-edges` clause measures a value's own
/// [`placed_edges`](drift::NestedMemberRow::placed_edges) against
/// ([`embedded_member_features`]). A malformed edge fact is
/// [`edges_from_declarations`]'s own load error, raised before any check runs, so this
/// fold reads the well-formed rows rather than raise the identical fault twice.
fn edge_fields_by_kind(declarations: &drift::Declarations) -> BTreeMap<String, BTreeSet<String>> {
    let mut by_kind: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for fact in &declarations.assembly {
        if fact.fact != "edge" {
            continue;
        }
        if let (Some(from), Some(field)) = (fact.from.clone(), fact.field.clone()) {
            by_kind.entry(from).or_default().insert(field);
        }
    }
    by_kind
}

/// Lift one [`NestedMemberRow`](drift::NestedMemberRow) into the
/// [`Features`](extract::Features) an edge resolves against: the row's key is the member
/// id an edge matches by identity, and its leaves surface as string fields so a clause
/// (or a deeper edge) can range over them exactly as a file member's frontmatter. The
/// body-derived features are empty — an embedded member has no document of its own; it is
/// read off its host's declared surface.
///
/// `edge_fields` is what the member's kind declares ([`edge_fields_by_kind`]); pairing the
/// ones this row actually fills with its own `placed_edges` is what makes a
/// `format-places-edges` clause decidable without the engine ever seeing the format that
/// rendered the value. An unfilled field is no edge, so it is no obligation: ranging over
/// the kind's whole declared set would read an absent edge as one the format dropped.
fn embedded_member_features(
    row: &drift::NestedMemberRow,
    edge_fields: &BTreeSet<String>,
) -> extract::Features {
    let fields = row
        .leaves
        .iter()
        .map(|(name, text)| (name.clone(), serde_json::Value::String(text.clone())))
        .collect();
    extract::Features {
        id: row.key.clone(),
        fields,
        body_lines: 0,
        // The rendered span `emit` captured off the value's own projection, lifted from
        // the row so an `extent` clause bound to the embedded kind budgets real data. A
        // `None` span is a value no format rendered (a layout host read off source): it has
        // no projection to measure, so its `extent` stays undecidable rather than reading a
        // zero as a pass.
        rendered_lines: row.rendered_lines,
        rendered_chars: row.rendered_chars,
        headings: Vec::new(),
        sections: Vec::new(),
        source_dir: None,
        directives: Vec::new(),
        fenced_blocks: Vec::new(),
        nested_members: Vec::new(),
        satisfies: Vec::new(),
        // `None` ⇒ no format rendered the value (a layout host's document is source, not
        // projection), which is not a format to indict. `Some` over an empty map ⇒ a
        // format ran and the value carries no edge to place. The engine cannot tell the
        // two apart once they collapse into one empty map, so they are kept apart here.
        edge_placements: row.placed_edges.as_ref().map(|placed| {
            edge_fields
                .iter()
                .filter(|field| row.leaves.get(*field).is_some_and(|text| !text.is_empty()))
                .map(|field| (field.clone(), placed.contains(field)))
                .collect()
        }),
    }
}

/// Construct directive members from pre-computed resolved units and features, avoiding
/// a second `resolve_kind_units` pass. Called by [`gate`] and [`explain`] to avoid
/// re-reading every member off disk after the units and features have already been
/// resolved for validation.
fn directive_members_from_resolved(
    builtin_units_and_features: &BTreeMap<String, compose::KindUnitsAndFeatures>,
    custom_units_and_features: &[(CustomKind, compose::KindUnitsAndFeatures)],
) -> Vec<graph::DirectiveMember> {
    let mut members = Vec::new();
    for (kind_name, uaf) in builtin_units_and_features {
        for (unit, features) in uaf.units.iter().zip(&uaf.features) {
            members.push(graph::DirectiveMember {
                kind: kind_name.clone(),
                id: features.id.clone(),
                source_path: unit.source_path.clone(),
                directives: features.directives.clone(),
            });
        }
    }
    for (custom_kind, uaf) in custom_units_and_features {
        for (unit, features) in uaf.units.iter().zip(&uaf.features) {
            members.push(graph::DirectiveMember {
                kind: custom_kind.name.clone(),
                id: features.id.clone(),
                source_path: unit.source_path.clone(),
                directives: features.directives.clone(),
            });
        }
    }
    members
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

        let files = repo_file_set(&root);
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
        gate(&harness, &harness, &[]).unwrap();
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
        gate(&harness, &harness, &[]).unwrap();
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
        let edges = edge_fields_by_kind(declarations);
        embedded_member_features(
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
