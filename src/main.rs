//! `temper` CLI entry point.
//!
//! A thin `clap` dispatch over the [`temper`] library: parse args, run the
//! generic contract engine, map the result to an exit code. Every subcommand
//! mirrors the pipeline verbs; all logic lives in the
//! library so `tests/` can drive it.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::LazyLock;

use clap::{Parser, Subcommand, ValueEnum};
use miette::IntoDiagnostic;
use temper::builtin;
use temper::builtin_kind;
use temper::bundle;
use temper::check::{self, Severity};
use temper::compose;
use temper::contract;
use temper::contract::Contract;
use temper::coverage;
use temper::coverage_note;
use temper::dial;
use temper::document;
use temper::drift;
use temper::engine;
use temper::extract;
use temper::frontmatter;
use temper::graph;
use temper::import;
use temper::install;
use temper::json_manifest;
use temper::kind::{self, CollectionAddress, CustomKind, Unit};
use temper::read;
use temper::reporter;
use temper::roster;
use temper::schema;
use temper::tap;
use temper::toml_document;
use toml_edit::DocumentMut;

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
    let LockFamily { declarations, .. } =
        assemble_lock_family(&discovery, &drift::read_declarations(&workspace)?, &[])?;

    // Every embedded built-in kind's discovered features — the same generic loop
    // `gate`'s two-greens runs, not a hardcoded skill/rule pair
    // (MEMORY-ENTERS-REQUIREMENT-CORPUS), so a memory member's declared `satisfies`
    // reaches `explain` exactly as it reaches the gate's roster/graph/coverage tiers.
    let builtin_defs = builtin_kind::definitions();
    let builtin_features = builtin_features_by_kind(&builtin_defs, &discovery, &declarations)?;

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
    let mut custom_kinds: Vec<CustomKindEntry> = Vec::new();
    let mut custom_members: Vec<read::CustomMember> = Vec::new();
    let (custom_rows, _collisions) = partition_kind_rows(&declarations, &builtin_defs)?;
    for row in custom_rows {
        let custom_kind = CustomKind::from_kind_fact_row(row)?;
        contracts.insert(
            row.name.clone(),
            compose::default_contract_from_rows(&declarations.clauses, &row.name)?,
        );
        let units = resolve_kind_units(&custom_kind, &discovery, &declarations)?;
        let features: Vec<extract::Features> = units
            .iter()
            .map(|unit| builtin_kind::features(&custom_kind, unit, &declarations.nested_members))
            .collect();
        for unit in &units {
            custom_members.push(read::CustomMember {
                kind: custom_kind.name.clone(),
                id: unit.id.clone(),
                satisfies: unit.satisfies_clauses.clone(),
            });
        }
        custom_kinds.push((custom_kind, features));
    }
    let embedded_features = embedded_features_by_kind(&declarations);
    let by_kind = assemble_by_kind(&builtin_features, &custom_kinds, &embedded_features);

    // The one requirement namespace: the assembly's declared `[requirement.*]`
    // roster — a custom-kind member has no channel of its own to publish one (the
    // pre-0016 own-path surface that once carried it is retired).
    let roster: BTreeMap<String, compose::Requirement> = declarations
        .requirements
        .iter()
        .map(|row| Ok((row.name.clone(), requirement_from_row(row)?)))
        .collect::<Result<_, compose::ClauseRowError>>()?;
    let assembly_edges = edges_from_declarations(&declarations)?;
    // Authored mentions (route-resolved at check) and layout prose imports (path-resolved
    // at emit) both lift off the lock; `why` route-resolves them against this same corpus,
    // narrating a dangling mention as the gate's route finding rather than a resolved edge,
    // so a read cannot disagree with the gate (READ-EDGE-UNIFY).
    let mut mention_edges = mention_edges_from_declarations(&declarations);
    mention_edges.extend(import_edges_from_doc(&lock_doc)?);

    // The world's inbound registration channel set into each built-in kind — the same
    // derivation the gate's `reachable` runs, keyed by bare kind name to join `by_kind`.
    let mut registrations: BTreeMap<&str, Vec<kind::Registration>> = BTreeMap::new();
    for def in builtin_defs.values() {
        if !def.registration.is_empty() {
            registrations.insert(def.name.as_str(), def.registration.clone());
        }
    }

    let repo_files = repo_file_set(Path::new("."));
    let directive_members = collect_directive_members(&discovery, &declarations)?;
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
        let kind = overlay_builtin_kind(kind, declarations)?;
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

    let (custom_rows, _collisions) = partition_kind_rows(declarations, &builtin_defs)?;
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
    let LockFamily {
        declarations,
        joined_clauses,
        joined_locks,
        local_members,
        dial,
    } = assemble_lock_family(&discovery, &committed, layers)?;
    // Every address the dial reached, accumulated across the contracts and selections
    // below: an entry that reached none is the one thing a dial can be wrong about that
    // its own schema cannot catch.
    let mut dialed: BTreeSet<String> = BTreeSet::new();
    let assembly_requirements: BTreeMap<String, compose::Requirement> = declarations
        .requirements
        .iter()
        .map(|row| Ok((row.name.clone(), requirement_from_row(row)?)))
        .collect::<Result<_, compose::ClauseRowError>>()?;
    let assembly_edges = edges_from_declarations(&declarations)?;
    // The lifted reference edges the graph predicates and read verbs fold in alongside the
    // declared-field arcs: authored mentions (route-resolved at check — `route_mentions`
    // owns a deferred mention's dangling verdict) and layout prose imports (path-resolved
    // at emit), each lifted off the lock's own declaration family.
    let mut mention_edges = mention_edges_from_declarations(&declarations);
    mention_edges.extend(import_edges_from_doc(&lock_doc)?);

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
    // Every clause's address is unique across the lock, decided before a single contract
    // is lifted: a clause no finding can name unambiguously cannot be judged usefully.
    diagnostics.extend(clause_collision_diagnostics(&declarations, &joined_clauses));
    let builtin_defs = builtin_kind::definitions();
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

        let features = kind_features(kind, &discovery, &declarations)?;

        diagnostics.extend(engine::admissibility(&contract, &engine::Locus::Document));
        diagnostics.extend(engine::validate(&contract, &features));
        member_counts.insert(kind.name.clone(), features.len());
        contracts.insert(kind.name.clone(), contract);
        builtin_features.insert(kind.name.clone(), features);
    }

    // Every lock-declared kind that is not one of the embedded built-ins:
    // a
    // built-in's own row is only the overlay `overlay_builtin_kind` already
    // consumes, never a second kind definition. A custom kind carries no embedded
    // default — its whole default contract is the committed lock's own clause rows
    // naming it ([`compose::default_contract_from_rows`]) — but is otherwise
    // dispatched through the identical two-greens the built-in loop above runs.
    let mut custom_kinds: Vec<CustomKindEntry> = Vec::new();
    let (custom_rows, collisions) = partition_kind_rows(&declarations, &builtin_defs)?;
    // The one site among the three dispatchers that can surface a diagnostic.
    diagnostics.extend(collisions.iter().map(|row| kind_collision_diagnostic(row)));
    // Two distinct kinds resolving to one `governs` locus would double-route every
    // matching document into both member sets — a document's kind is its position
    // alone, never its content — so a shared locus refuses loud here.
    diagnostics.extend(governs_collision_diagnostics(
        &builtin_defs,
        &custom_rows,
        &declarations,
    )?);
    // A declared commitment class the locus cannot carry is decided here, beside the
    // locus's other coherence check, before any member is read under a kind whose own
    // declaration does not hold together.
    diagnostics.extend(local_locus_admissibility(
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
        let features = kind_features(&custom_kind, &discovery, &declarations)?;

        diagnostics.extend(engine::admissibility(&contract, &engine::Locus::Document));
        diagnostics.extend(engine::validate(&contract, &features));
        member_counts.insert(row.name.clone(), features.len());
        contracts.insert(row.name.clone(), contract);
        custom_kinds.push((custom_kind, features));
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
            &collect_directive_members(&discovery, &declarations)?,
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
    diagnostics.extend(nested_member_admissibility(&declarations));

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
    diagnostics.extend(joined_kind_admissibility(&joined_clauses, &contracts)?);

    let by_kind = assemble_by_kind(&builtin_features, &custom_kinds, &embedded_features);

    // A bare `satisfies` label an older engine wrote qualifies against this corpus, but a
    // name two kinds share is a malformed lock refused loud rather than cross-attributed.
    diagnostics.extend(satisfies_label_admissibility(&declarations, &by_kind));

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

    // `acyclic`: the resolved graph must contain no
    // cycle — a circular import loads nothing, so every finding is a true
    // positive. Always-on over the whole edge set, like route resolution above.
    diagnostics.extend(graph::acyclic(&edges, &by_kind));

    // `degree`: the one set predicate whose judge needs the graph — a clause bounds
    // every selected member's in/out edge count, so it takes the same selections
    // `engine::judge` reads *and* the edges, reusing the arc resolution
    // `acyclic`/`check` assemble, plus the already-resolved mention edges —
    // obligation-free by default, counted only when a `degree` clause opts in.
    diagnostics.extend(graph::degree(&selections, &edges, &mention_edges, &by_kind));

    // `mention-reachable`: the second selection predicate whose judge needs the graph —
    // each selected member's references must be able to fire where their target can be
    // invoked, which reads the *target* member's gate field. It ranges over the same
    // unified edge set `degree` does — the resolved field edges *and* the mention/import
    // family — so a rendering claim carried on a field edge is judged, never dropped.
    // Opt-in like `degree`: a selection declaring no such clause does no work.
    diagnostics.extend(graph::mention_reachable(
        &selections,
        &edges,
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
    // or locked custom, the note reads the lock for the latter — governs, so the
    // gate's silence about an unmodeled surface never reads as "checked". Warn-only —
    // it leaves the run's exit code and the session-start verdict unchanged.
    diagnostics.extend(coverage_note::check(
        harness_root,
        &builtin_kind::definitions(),
        &member_counts,
    )?);

    // The freshness fact: a committed projection
    // whose bytes no longer match the lock's emit fingerprint is `config.stale`. Read
    // off the surface `workspace`'s lock (where the members were imported and the
    // fingerprints recorded), advisory so a hand-edited or un-re-emitted projection is
    // surfaced without failing the run.
    diagnostics.extend(drift::config_stale(workspace));

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

/// This kind's effective declaration: the committed lock's own kind-fact row when the
/// lock declares one that [`row_relocates_builtin`] — matched by bare name, the kind's
/// whole identity — overlaid onto `kind`'s embedded declaration, or `kind` unchanged
/// when it doesn't: the **built-in lock**, the same declaration shape the engine
/// carries compiled-in for an unadopted harness. Three facts overlay from the one
/// matched row: the `governs` locus always (a relocation may declare no other diverging
/// fact at all), `templates` only when the row declares at least one, and `content` only
/// when the row declares a layout — an empty row column defers to `kind`'s own (always
/// empty templates and a `File` body for a built-in), never blanking a nonexistent
/// override.
fn overlay_builtin_kind(
    kind: &CustomKind,
    declarations: &drift::Declarations,
) -> Result<CustomKind, drift::LockRowError> {
    let mut matched = None;
    for row in &declarations.kinds {
        if row.name == kind.name && row_relocates_builtin(row, kind)? {
            matched = Some(row);
            break;
        }
    }
    let Some(row) = matched else {
        return Ok(kind.clone());
    };
    let mut overlaid = kind.clone();
    // The two governs columns are one spelling, and a row declaring neither defers to the
    // built-in's own locus rather than blanking it — the posture `overlay_content` takes.
    if let Some((root, glob)) = row.governs_root.clone().zip(row.governs_glob.clone()) {
        overlaid.governs = Some(kind::Governs { root, glob });
    }
    if !row.templates.is_empty() {
        overlaid = overlaid.overlay_templates(&row.templates);
    }
    overlaid = overlaid.overlay_content(row.content.as_ref())?;
    Ok(overlaid)
}

/// Read one layout-content document at `file` into a [`Unit`], off the kind's declared
/// `layout`: the whole file is the body's heading tree, the field sections fill the
/// unit's fields (each slot's verbatim span, so a clause ranges over it as a field). A
/// declared-relationship edge slot is the exception: its entries are addresses, folded
/// onto the unit as a list field the reference graph resolves live off the host's
/// features — like a file member's frontmatter reference list — while `satisfies` reaches
/// the unit off the lock's own family, keyed by member id, not off the document here. The
/// id folds the file's placement under `base` the same way a file-content member's does.
/// A document that does not fit the layout — a section missing, structure
/// no primitive admits — refuses loud through [`kind::LayoutError`], naming the file and
/// heading.
///
/// # Errors
///
/// Returns an error if the document is unreadable or does not fit its declared layout.
fn layout_unit(
    layout: &kind::Layout,
    file: &Path,
    base: &Path,
    edge_fields: &BTreeSet<String>,
) -> miette::Result<Unit> {
    let raw = std::fs::read_to_string(file).into_diagnostic()?;
    let reading = layout.read(&raw, file, edge_fields)?;
    let id = frontmatter::fold_file_id(base, file)?;
    let mut frontmatter: BTreeMap<String, serde_json::Value> = reading
        .fields
        .into_iter()
        .map(|(slot, span)| (slot, serde_json::Value::String(span)))
        .collect();
    // A relationship edge slot's entries fold on as a list field the reference graph reads
    // off the host's features. `satisfies` is excepted: it reaches the unit off the lock's
    // own `satisfies` family below, never off this document read.
    for (slot, entries) in reading.edges {
        if slot == kind::SATISFIES_EDGE_FIELD {
            continue;
        }
        frontmatter.insert(
            slot,
            serde_json::Value::Array(entries.into_iter().map(serde_json::Value::String).collect()),
        );
    }
    Ok(Unit {
        id,
        frontmatter,
        body: raw,
        source_path: file.to_path_buf(),
        satisfies: Vec::new(),
        satisfies_clauses: Vec::new(),
    })
}

/// A kind's members, resolved live off disk — the one corpus both `gate` and `explain`
/// range over. Every
/// member is discovered by walking this kind's [`overlay_builtin_kind`]-overlaid
/// `governs` locus, read straight off harness disk so the corpus can never drift from a
/// stale copy; its `satisfies` fill edges come from the run's assembled
/// [`SatisfiesRow`](drift::SatisfiesRow) family, keyed by member id — a committed
/// member's row off the lock, a local member's derived at
/// [`assemble_lock_family`], so this read never re-decides which source it has. Its
/// rationale-carrying `satisfies_clauses` mirrors it: a lock-declared row narrates as a
/// rationale-less [`document::Satisfies`](document::Satisfies) — the lock row carries
/// no rationale text — so `explain` can never disagree with the gate about which
/// requirements a member fills.
///
/// # Errors
///
/// Returns an error if a source file is unreadable or malformed, or a governed
/// directory cannot be enumerated.
fn resolve_kind_units(
    kind: &CustomKind,
    disc: &import::Discovery,
    declarations: &drift::Declarations,
) -> miette::Result<Vec<Unit>> {
    let overlaid = overlay_builtin_kind(kind, declarations)?;
    let governs = overlaid.governs.clone();
    // The kind's edge-field slots: a layout field section on one of these reads as
    // addresses, not a verbatim span, so it never lands as a frontmatter field. A custom
    // kind's relationships live only in the lock's assembly facts (its kind-fact row
    // carries none), so the embedded set is unioned with the lock-declared one — the same
    // union emit reads by. `satisfies` reaches the unit off the lock's own family below,
    // exactly as a file-content member's do.
    let mut edge_fields = kind.edge_field_slots();
    edge_fields.extend(drift::layout_edge_fields(
        &declarations.assembly,
        &kind.name,
    )?);

    // A **manifest kind** — a fields-only kind carrying a collection address — reads its
    // members out of a host JSON manifest, never a file tree: the runtime gate-path
    // dispatch the manifest adapter delivered a library face for (MANIFEST-ADAPTER-READ)
    // lands here at its first manifest kind. Every other kind walks its `governs` locus and
    // reads each file through the one adapter dispatch its declared format decides
    // (`read_file_unit`).
    let mut units = match (&overlaid.content, &overlaid.collection_address, &governs) {
        (kind::Content::Fields, Some(address), _) => manifest_units(disc, &overlaid, address)?,
        // A nested file kind governs no glob to walk: its members sit under each *host*
        // member's unit at the host kind's template pattern, so they are found through the
        // declared set the host lives in, and each child's id folds against its own host's
        // unit rather than one shared scan root.
        (_, _, None) => {
            let kinds = declared_kinds(declarations)?;
            let mut child_units = Vec::new();
            for found in import::discover_nested_file(
                disc,
                &overlaid,
                &kinds,
                import::LocalOverride::Honored,
            ) {
                child_units.push(read_file_unit(
                    &overlaid,
                    &found.file,
                    &found.host_unit,
                    &edge_fields,
                )?);
            }
            child_units
        }
        (_, _, Some(governs)) => {
            let base = disc.harness().join(&governs.root);
            let mut file_units = Vec::new();
            for file in
                import::discover_kind_files(disc, kind, governs, import::LocalOverride::Honored)
            {
                file_units.push(read_file_unit(&overlaid, &file, &base, &edge_fields)?);
            }
            file_units
        }
    };

    // `satisfies` fill edges reach every member — file or manifest — off the lock's own
    // family, joined on the member's `kind:name` address so a registration member joins
    // the requirement corpus exactly as a file member does. A canonical row carries that
    // qualified label; a bare label an older engine wrote still binds against this kind's
    // own id (a bare label two kinds share is refused at admissibility, never
    // cross-attributed here).
    for unit in &mut units {
        let address = extract::host_address(&kind.name, &unit.id);
        for row in &declarations.satisfies {
            if row.member != address && row.member != unit.id {
                continue;
            }
            if !unit.satisfies.contains(&row.requirement) {
                unit.satisfies.push(row.requirement.clone());
            }
            if !unit
                .satisfies_clauses
                .iter()
                .any(|clause| clause.requirement == row.requirement)
            {
                unit.satisfies_clauses
                    .push(document::Satisfies::new(row.requirement.clone()));
            }
        }
    }

    units.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(units)
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
fn read_file_unit(
    kind: &CustomKind,
    file: &Path,
    base: &Path,
    edge_fields: &BTreeSet<String>,
) -> miette::Result<Unit> {
    match (&kind.content, &kind.format) {
        (kind::Content::Layout(layout), _) => layout_unit(layout, file, base, edge_fields),
        (kind::Content::File | kind::Content::Fields, Some(kind::Format::JsonDocument)) => {
            Ok(json_manifest::DocumentMember::read(kind, file)?.to_unit())
        }
        (kind::Content::File | kind::Content::Fields, Some(kind::Format::TomlDocument)) => {
            Ok(toml_document::read(kind, file)?.to_unit())
        }
        (
            kind::Content::File | kind::Content::Fields,
            Some(kind::Format::YamlFrontmatter) | None,
        ) => {
            let source = frontmatter::Member::from_source_rooted(kind, file, base)?;
            Ok(Unit {
                id: source.id.clone(),
                frontmatter: source.fields.iter().cloned().collect(),
                body: source.body.clone(),
                source_path: source.provenance.source_path.clone(),
                satisfies: Vec::new(),
                satisfies_clauses: Vec::new(),
            })
        }
    }
}

/// Every kind this harness declares, keyed by bare name: each embedded built-in under its
/// lock overlay, plus each lock-declared custom kind — the same universe `gate`'s own
/// dispatch ranges over. A nested file kind's host is found here, so the set is what
/// carries the two halves of its locus that the child kind itself cannot.
///
/// # Errors
///
/// Returns an error if the embedded kind set fails to load or a lock row falls outside a
/// closed vocabulary.
fn declared_kinds(
    declarations: &drift::Declarations,
) -> miette::Result<BTreeMap<String, CustomKind>> {
    let builtin_defs = builtin_kind::definitions();
    let mut kinds = BTreeMap::new();
    for kind in builtin_defs.values() {
        kinds.insert(kind.name.clone(), overlay_builtin_kind(kind, declarations)?);
    }
    let (custom_rows, _collisions) = partition_kind_rows(declarations, &builtin_defs)?;
    for row in custom_rows {
        kinds.insert(row.name.clone(), CustomKind::from_kind_fact_row(row)?);
    }
    Ok(kinds)
}

/// A manifest `kind`'s registration members as raw [`Unit`]s — every `hooks.<Event>` (or
/// `mcpServers.*`) entry the host manifest carries at the kind's declared collection
/// `address`, read through the JSON manifest adapter ([`json_manifest::Manifest::read_kind`]).
/// A member's id is its collection key, and that key surfaces under the address's key
/// field when it names one (`hooks.<Event>` → `event`), so a clause can range over the
/// lifecycle event a hook keys at. `satisfies` is left empty here — the caller folds it in
/// off the lock, exactly as for a file member.
///
/// The member's own object fields fold into the unit's frontmatter — an mcp-server's
/// `command`/`type`/`url` become checkable fields the same way a frontmatter member's do,
/// projected at the shared read-time fold. A hook's `hooks.<Event>` value is an array, so
/// its member carries no object fields and only the event key surfaces; an mcp-server's
/// entry is an object, so its fields fold in and `mcpServers.*` names no key field (a
/// server's key is its identity, read off its own fields).
///
/// # Errors
///
/// Returns an error if the manifest cannot be discovered or read.
fn manifest_units(
    disc: &import::Discovery,
    kind: &CustomKind,
    address: &CollectionAddress,
) -> miette::Result<Vec<Unit>> {
    let mut units = Vec::new();
    for manifest in json_manifest::Manifest::read_kind(disc, kind)? {
        let source_path = manifest.provenance.source_path.clone();
        for member in &manifest.members {
            units.push(member.to_unit(address, &source_path));
        }
    }
    Ok(units)
}

/// A kind's members' extracted [`Features`](extract::Features) — [`resolve_kind_units`]
/// run through the [`overlay_builtin_kind`]-overlaid kind's own composed extraction,
/// each member's nested-member facts resolved off the run's assembled `nested_members`
/// rows by address ([`builtin_kind::features`]), never by re-parsing its rendered body.
///
/// # Errors
///
/// As [`resolve_kind_units`].
fn kind_features(
    kind: &CustomKind,
    disc: &import::Discovery,
    declarations: &drift::Declarations,
) -> miette::Result<Vec<extract::Features>> {
    let kind = overlay_builtin_kind(kind, declarations)?;
    let units = resolve_kind_units(&kind, disc, declarations)?;
    Ok(units
        .iter()
        .map(|unit| builtin_kind::features(&kind, unit, &declarations.nested_members))
        .collect())
}

/// The run's whole declaration family, assembled once for every consumer below.
struct LockFamily {
    /// The committed lock's rows, joined with every local-locus kind's read-time derived
    /// ones — the corpus's own declarations, the set that decides which kinds and members
    /// exist here.
    declarations: drift::Declarations,
    /// The clause rows of the locks this invocation joins, each already addressed under
    /// the layer that carried it ([`qualify_layer_label`]). Kept beside the corpus's own
    /// rows rather than merged into them: these declare nothing about what this harness
    /// *is* — they only add checks over what it already declares — and a consumer that
    /// reads them as the corpus's own would let a layer redefine the corpus.
    joined_clauses: Vec<drift::ClauseRow>,
    /// Every lock this invocation joined, as `--layer` spelled it — the locks the rows
    /// above came off, kept beside them because a lock that carried no clause joined the
    /// run just the same and the announcement names the lock, never its contents.
    joined_locks: Vec<String>,
    /// Every local member the assembly read, by `<kind>:<id>` address, retained here
    /// beside the rows its read produced: the documents are uncommitted, so no consumer
    /// below can find them again short of re-walking the kind's glob — a second read that
    /// could disagree with this one about which members exist.
    local_members: Vec<String>,
    /// This machine's own dial: the severities it re-reads the clauses above at. Read
    /// with the rest of the family for the same one-read reason, and kept apart from both
    /// row sets for the joined clauses': a dial declares nothing about what this harness
    /// is, and adds no check to it either — it only re-weighs the checks the rows above
    /// already carry.
    dial: dial::Dial,
}

/// The run's whole declaration family: the committed lock, every local-locus kind's
/// read-time derived rows ([`local_document_rows`]), and the clause rows of the locks
/// `layers` names.
///
/// A local kind is committed but its members' documents are not, so the lock carries no
/// row of theirs. Deriving them *here* — once, before any consumer reads — is what lets
/// every consumer below read one family: a clause bound to an embedded kind selects a
/// local host's members exactly as it selects a committed host's, and a local member's
/// fills reach the roster on the same read. A consumer re-deciding which of two sources it
/// reads is the shape this replaces; the derivation runs against the committed family,
/// which is what decides the kinds and loci the members are discovered under.
///
/// A joined lock and this machine's dial are read here for the same reason: one read,
/// before any consumer, so no call site below re-opens a layered input and re-decides
/// what it says.
///
/// # Errors
///
/// As [`resolve_kind_units`], [`local_document_rows`] and [`read_layer_clauses`].
fn assemble_lock_family(
    disc: &import::Discovery,
    committed: &drift::Declarations,
    layers: &[PathBuf],
) -> miette::Result<LockFamily> {
    let mut assembled = committed.clone();
    // Sorted, so the announced set reads the same whatever order a kind's locus walk
    // happened to yield.
    let mut local_members: BTreeSet<String> = BTreeSet::new();
    for kind in declared_kinds(committed)?.values() {
        if kind.commitment != Some(kind::Commitment::Local) {
            continue;
        }
        let units = resolve_kind_units(kind, disc, committed)?;
        let rows = local_document_rows(kind, &units, committed)?;
        local_members.extend(
            units
                .iter()
                .map(|unit| extract::host_address(&kind.name, &unit.id)),
        );
        assembled.nested_members.extend(rows.nested);
        assembled.satisfies.extend(rows.satisfies);
    }
    let joined = read_layer_clauses(layers)?;
    Ok(LockFamily {
        dial: read_dial(disc, committed)?,
        declarations: assembled,
        joined_clauses: joined.clauses,
        joined_locks: joined.locks,
        local_members: local_members.into_iter().collect(),
    })
}

/// This machine's [`dial::Dial`], read off the shipped `dial` kind's own members.
///
/// The kind is embedded rather than lock-declared, so it is the definition that is
/// reached for here rather than the loop above's declared set — a harness gets its dial
/// from adopting temper at all, never from declaring one. The read is the same
/// [`kind_features`] the gate's own dispatcher runs over the kind, which is what keeps
/// the entries this returns and the document the contract judges from ever being two
/// different reads of one file.
///
/// # Errors
///
/// As [`kind_features`] — a malformed dial document fails the run rather than reading as
/// an empty dial, since a dial silently applying nothing is the fail-open case.
fn read_dial(
    disc: &import::Discovery,
    declarations: &drift::Declarations,
) -> miette::Result<dial::Dial> {
    let Some(kind) = builtin_kind::definition(dial::KIND) else {
        return Ok(dial::Dial::default());
    };
    Ok(dial::Dial::from_features(&kind_features(
        &kind,
        disc,
        declarations,
    )?))
}

/// The separator between a joined clause's own compiled address and the layer that
/// carried it: `<label>@<layer>`.
///
/// A compiled address is dot-joined ([`contract::clause_label`]), so `@` appears in no
/// label emit can write — which is what makes a joined address unable to collide with a
/// host's, whatever the two locks happen to declare. Legible in both directions: the
/// author who reads a finding reads which `--layer` argument produced it, and the name is
/// spelled straight back out of the finding to reach the clause.
const LAYER_QUALIFIER: char = '@';

/// The lock file a `--layer` argument names: the path itself, or the lock inside it when
/// the argument names a directory.
fn layer_lock_path(layer: &Path) -> PathBuf {
    if layer.is_dir() {
        layer.join(temper::LOCK_FILENAME)
    } else {
        layer.to_path_buf()
    }
}

/// The locks an invocation joined, and the clause rows they carried.
struct JoinedLayers {
    /// Each joined lock, as the invocation spelled it — the same spelling every clause
    /// below is addressed under, and deduped on the same identity, so a lock named twice
    /// is one layer here too.
    locks: Vec<String>,
    /// Every joined clause row, addressed under the layer that carried it.
    clauses: Vec<drift::ClauseRow>,
}

/// The clause rows of every lock `layers` names, each addressed under the layer that
/// carried it, with the locks themselves.
///
/// Only the clause family joins. A layer hardens the gate over *this* corpus; the rows
/// that say what this corpus is — its kinds, its members' fills, its requirements — stay
/// the committed lock's alone, because joining those would let a layer relocate a kind's
/// locus or forge a fill and so *soften* the very gate it claims to tighten. Clauses are
/// the family a join can only add to.
///
/// # Errors
///
/// A named layer that cannot be read, or whose lock is malformed, is an error, not an
/// empty set: an absent lock the invocation named is a layer that did not gate, and a
/// layer silently gating nothing is the one outcome fail-closed forbids. (The *host*
/// lock's absence is legitimate — an unadopted harness has none — which is why that read
/// tolerates it and this one cannot.)
fn read_layer_clauses(layers: &[PathBuf]) -> miette::Result<JoinedLayers> {
    let mut joined = JoinedLayers {
        locks: Vec::new(),
        clauses: Vec::new(),
    };
    let mut seen: BTreeSet<PathBuf> = BTreeSet::new();
    for layer in layers {
        let path = layer_lock_path(layer);
        // One lock joined twice is one layer: naming it under two spellings is a benign
        // repetition, and joining it again would collide its every address with its own
        // first copy and refuse the run as a malformed layer.
        let identity = fs::canonicalize(&path).unwrap_or_else(|_| path.clone());
        if !seen.insert(identity) {
            continue;
        }
        let text = fs::read_to_string(&path).map_err(|source| {
            miette::miette!(
                "failed to read joined layer lock {}: {source}",
                path.display()
            )
        })?;
        let spelling = layer.display().to_string();
        for row in drift::parse_declarations(&path, &text)?.clauses {
            joined.clauses.push(qualify_layer_label(row, &spelling));
        }
        joined.locks.push(spelling);
    }
    Ok(joined)
}

/// One joined clause row, re-addressed under the layer that carried it.
///
/// A row carrying no address is left as it is: every emitted row is stamped with one, so
/// a row without one is a lock emit did not write, and the contract lift is the one home
/// that refuses it ([`compose::clause_from_row`]) — re-deciding that here would be a
/// second verdict on the same fact.
fn qualify_layer_label(mut row: drift::ClauseRow, layer: &str) -> drift::ClauseRow {
    if let Some(label) = row.label.take() {
        row.label = Some(format!("{label}{LAYER_QUALIFIER}{layer}"));
    }
    row
}

/// The admissibility findings of every joined clause naming a kind absent from
/// `contracts` — the kinds this corpus declares none of, whose clauses no dispatcher
/// above ever lifted.
///
/// # Errors
///
/// As [`compose::default_contract_from_rows`].
fn joined_kind_admissibility(
    joined: &[drift::ClauseRow],
    contracts: &BTreeMap<String, Contract>,
) -> Result<Vec<check::Diagnostic>, compose::ClauseRowError> {
    let undeclared: BTreeSet<&str> = joined
        .iter()
        .filter_map(|row| row.kind.as_deref())
        .filter(|kind| !contracts.contains_key(*kind))
        .collect();
    let mut diagnostics = Vec::new();
    for kind in undeclared {
        let contract = compose::default_contract_from_rows(joined, kind)?;
        diagnostics.extend(engine::admissibility(&contract, &engine::Locus::Document));
    }
    Ok(diagnostics)
}

/// A local-locus kind's members' declaration rows, derived off their own documents —
/// what the lock would carry for a committed kind, and never does for this one.
///
/// The rows go through the same reader `emit` lowers a committed layout host's source
/// with ([`drift::read_layout_document`]), so a local member's rows are the rows its
/// document declares, not a second interpretation of it.
///
/// The dispatch mirrors [`read_file_unit`]'s arm for arm, over the same declared format:
/// a layout's regions declare a member collection and an edge slot, so those two families
/// are read off the document here; the frontmatter, json-document and toml-document read
/// faces declare neither, so a member of theirs has no row in either family and its fields
/// and prose reach the corpus through `read_file_unit` alone. Matching both facts
/// exhaustively is the point — a read face that *does* declare rows must answer here, and a
/// catch-all would instead let it gate against silence.
///
/// # Errors
///
/// Returns an error when a member's document cannot be read or does not fit the kind's
/// declared layout.
fn local_document_rows(
    kind: &CustomKind,
    units: &[Unit],
    declarations: &drift::Declarations,
) -> miette::Result<drift::LayoutDocumentRows> {
    let layout = match (&kind.content, &kind.format) {
        (kind::Content::Layout(layout), _) => layout,
        (
            kind::Content::File | kind::Content::Fields,
            Some(
                kind::Format::YamlFrontmatter
                | kind::Format::JsonDocument
                | kind::Format::TomlDocument,
            )
            | None,
        ) => return Ok(drift::LayoutDocumentRows::default()),
    };
    let mut edge_fields = kind.edge_field_slots();
    edge_fields.extend(drift::layout_edge_fields(
        &declarations.assembly,
        &kind.name,
    )?);

    let mut rows = drift::LayoutDocumentRows::default();
    for unit in units {
        let document = drift::read_layout_document(
            layout,
            &kind.name,
            &unit.id,
            &unit.source_path,
            &edge_fields,
        )?;
        rows.nested.extend(document.nested);
        rows.satisfies.extend(document.satisfies);
    }
    Ok(rows)
}

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

/// A registered custom kind as the corpus construction carries it: its loaded
/// [`CustomKind`] definition (identity travels on `.name` — no separate borrowed name
/// column) and its computed member [`Features`](extract::Features). Named so the
/// shared corpus helpers keep a legible signature (`clippy::type_complexity`).
type CustomKindEntry = (CustomKind, Vec<extract::Features>);

/// Resolve every embedded built-in kind's discovered [`Features`](extract::Features)
/// off `harness_root`, keyed by bare kind name — the one loop over
/// [`builtin_kind::definitions`] that `gate` and `explain` both build their by-kind
/// corpus from, so a memory member's (or any other built-in's) features reach both
/// callers identically instead of each hand-picking a `skill`/`rule` pair.
///
/// # Errors
///
/// As [`kind_features`].
fn builtin_features_by_kind(
    builtin_defs: &BTreeMap<String, CustomKind>,
    disc: &import::Discovery,
    declarations: &drift::Declarations,
) -> miette::Result<BTreeMap<String, Vec<extract::Features>>> {
    let mut by_kind = BTreeMap::new();
    for kind in builtin_defs.values() {
        by_kind.insert(kind.name.clone(), kind_features(kind, disc, declarations)?);
    }
    Ok(by_kind)
}

/// Assemble the by-kind [`Features`](extract::Features) corpus every set-scope and
/// graph predicate ranges over: every built-in kind ([`builtin_features_by_kind`])
/// plus each lock-declared custom kind's features, keyed by kind name. Borrows every
/// slice, so the caller holds the owned feature vecs for the map's lifetime.
fn assemble_by_kind<'a>(
    builtin_features: &'a BTreeMap<String, Vec<extract::Features>>,
    custom_kinds: &'a [CustomKindEntry],
    embedded_features: &'a BTreeMap<String, Vec<extract::Features>>,
) -> BTreeMap<&'a str, &'a [extract::Features]> {
    let mut by_kind: BTreeMap<&str, &[extract::Features]> = builtin_features
        .iter()
        .map(|(name, features)| (name.as_str(), features.as_slice()))
        .collect();
    for (kind, features) in custom_kinds {
        by_kind.insert(kind.name.as_str(), features.as_slice());
    }
    // An embedded kind carries no kind-fact row of its own — it reaches the lock solely
    // through its host's `templates` column ([`embedded_features_by_kind`]) — so it is in
    // neither the built-in nor the custom set. Keyed here, it joins the target set and an
    // edge targeting it resolves against its nested members instead of reading as
    // unmodeled. Its name never collides with an `at`-locus kind's (a kind holds one
    // locus), so it lands last without contention.
    for (kind, features) in embedded_features {
        by_kind.insert(kind.as_str(), features.as_slice());
    }
    by_kind
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
        declared_embedded_kinds(declarations)
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

/// The embedded kinds the lock declares: every child kind a host names, whether through
/// its `templates` column — a *path-less* entry, the embedded layer; a `path` templates a
/// file child, which owns its own unit — or a layout member collection's `member_kind`. The set a
/// `nested_member` row's kind must belong to — a row of any other kind is an orphan no
/// host templates ([`nested_member_admissibility`]) — and the keys
/// [`embedded_features_by_kind`] seeds its corpus with.
fn declared_embedded_kinds(declarations: &drift::Declarations) -> BTreeSet<String> {
    let mut kinds = BTreeSet::new();
    for row in &declarations.kinds {
        for template in &row.templates {
            // A template carrying a `path` templates a *file* child — the child owns a
            // unit at that pattern rather than a fence in the host's body — so its child
            // kind is no part of the embedded set.
            if template.path.is_none() {
                kinds.insert(template.kind.clone());
            }
        }
        if let Some(content) = &row.content {
            for region in &content.regions {
                if let Some(member_kind) = &region.member_kind {
                    kinds.insert(member_kind.clone());
                }
            }
        }
    }
    kinds
}

/// The diagnostic `rule` id an orphaned `nested_member` row reports under — a committed
/// lock's cross-family incoherence, decided before the by-kind corpus is trusted to model
/// the harness.
const NESTED_MEMBER_ADMISSIBILITY_RULE: &str = "nested-member.admissibility";

/// The diagnostic `rule` id a bare `satisfies` label two kinds both claim reports under.
const SATISFIES_LABEL_ADMISSIBILITY_RULE: &str = "satisfies.admissibility";

/// Reject a bare `satisfies` label a same-named member of two kinds both carry. A
/// canonical row addresses its filler by `kind:name`, so a bare label is one an older
/// engine wrote; it qualifies against the live corpus where exactly one kind bears the
/// name, but a name two kinds share is the ambiguous lock the closed identity forbids —
/// refused loud here rather than cross-attributed to both members' fill sets. A
/// qualified label already names its kind and can never be ambiguous.
fn satisfies_label_admissibility(
    declarations: &drift::Declarations,
    by_kind: &BTreeMap<&str, &[extract::Features]>,
) -> Vec<check::Diagnostic> {
    let bare: BTreeSet<&str> = declarations
        .satisfies
        .iter()
        .map(|row| row.member.as_str())
        .filter(|member| !member.contains(':'))
        .collect();
    bare.into_iter()
        .filter_map(|member| {
            let kinds: Vec<&str> = by_kind
                .iter()
                .filter(|(_, members)| members.iter().any(|features| features.id == member))
                .map(|(kind, _)| *kind)
                .collect();
            (kinds.len() > 1).then(|| {
                check::Diagnostic::error(
                    SATISFIES_LABEL_ADMISSIBILITY_RULE,
                    member,
                    format!(
                        "satisfies row `{}` is a bare member label the `{}` kinds each carry — an \
                         ambiguous lock row no single member can own",
                        member,
                        kinds.join("`, `"),
                    ),
                )
            })
        })
        .collect()
}

/// Validate the lock's `nested_member` rows against the declared nesting: every row's kind
/// must be an embedded kind some host declares — a `templates` column entry or a layout
/// member collection's `member_kind`. A row of a kind no host templates is an orphan the
/// by-kind corpus ([`embedded_features_by_kind`]) would unmodel while the host-address read
/// ([`drift::nested_members_from_rows`]) still carries it — the two disagreeing over one
/// committed lock. Reject it here, naming the kind and its host, the same malformed-lock
/// class as two rows wearing one label.
fn nested_member_admissibility(declarations: &drift::Declarations) -> Vec<check::Diagnostic> {
    let declared = declared_embedded_kinds(declarations);
    declarations
        .nested_members
        .iter()
        .filter(|row| !declared.contains(&row.kind))
        .map(|row| {
            check::Diagnostic::error(
                NESTED_MEMBER_ADMISSIBILITY_RULE,
                &row.host,
                format!(
                    "nested member `{}` under `{}` is of kind `{}`, which no host declares as a \
                     nested template — an orphaned lock row no kind's `templates` or member \
                     collection admits",
                    row.key, row.host, row.kind
                ),
            )
        })
        .collect()
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

/// Pair each member with the provenance `source_path` the directive classing joins on:
/// the decidable [`Features`](extract::Features)
/// view drops the full path, so it is read off the units the features were extracted
/// from. Every member is carried — a directive may point at a member that imports
/// nothing — with its `directives` occurrences (empty for a kind composing no
/// `directives` primitive).
///
/// Ranges over **every** embedded built-in kind's members via
/// [`builtin_kind::definitions`] — not a hardcoded skill/rule pair — so a discovered
/// `CLAUDE.md` memory member's `at-import` targets reach [`graph::classify_directives`]
/// and an unbacked `@path` draws its finding (DIRECTIVE-MEMBERS-ALL-KINDS, the same
/// generalization CHECK-MEMBERS-ALL-KINDS made for clause dispatch), **and** every
/// lock-declared custom kind's members, the same synthesis `gate`'s own dispatch runs
/// (CHECK-LOCK-KIND-ROWS). Each kind's members are resolved through
/// [`resolve_kind_units`] — the same live, governs-driven read the gate's own dispatch
/// uses — and keyed by the bare `kind.name`, the keying `by_kind`/`classify_directives`
/// join on.
///
/// # Errors
///
/// As [`resolve_kind_units`].
fn collect_directive_members(
    disc: &import::Discovery,
    declarations: &drift::Declarations,
) -> miette::Result<Vec<graph::DirectiveMember>> {
    let mut members = Vec::new();
    let builtin_defs = builtin_kind::definitions();
    for kind in builtin_defs.values() {
        for unit in resolve_kind_units(kind, disc, declarations)? {
            let feature = builtin_kind::features(kind, &unit, &declarations.nested_members);
            members.push(graph::DirectiveMember {
                kind: kind.name.clone(),
                id: feature.id.clone(),
                source_path: unit.source_path.clone(),
                directives: feature.directives.clone(),
            });
        }
    }
    let (custom_rows, _collisions) = partition_kind_rows(declarations, &builtin_defs)?;
    for row in custom_rows {
        let custom_kind = CustomKind::from_kind_fact_row(row)?;
        for unit in resolve_kind_units(&custom_kind, disc, declarations)? {
            let feature = builtin_kind::features(&custom_kind, &unit, &declarations.nested_members);
            members.push(graph::DirectiveMember {
                kind: custom_kind.name.clone(),
                id: feature.id.clone(),
                source_path: unit.source_path.clone(),
                directives: feature.directives.clone(),
            });
        }
    }
    Ok(members)
}

/// The diagnostic `rule` id a lock-declared kind row reports under when its bare name
/// matches an embedded built-in's but its declared shape does not: it can be admitted
/// neither as that built-in's relocated `governs` (the one legitimate reason a row
/// reuses a built-in's name — [`row_relocates_builtin`]) nor as a distinct custom kind
/// (the name is already claimed). Shares the roster's admissibility tag — a colliding
/// bare name is inadmissible, decided before anything judges the kind.
const KIND_COLLISION_RULE: &str = "kind.admissibility";

/// Whether a lock-declared kind-fact `row` sharing `builtin`'s bare name is admissible
/// as a **relocation** of it — the tested, legitimate mechanism
/// (`overlay_builtin_kind`) that points a built-in's `governs` locus somewhere other
/// than its embedded default, by declaring a row under the built-in's own name. A row only
/// relocates: every fact besides `governs` either agrees with the built-in's own or is
/// left undeclared (deferring to it) — `format`, `unit_shape`, `registration`. A row
/// that declares any of those *differently* is not reconfiguring the built-in, it is a
/// distinct kind's shape wearing the built-in's name — a namespace collision, never
/// silently subsumed into the built-in's walk. `templates` is excluded from this set: a
/// built-in's own `templates` is always empty (nothing populates it outside
/// `from_kind_fact_row`), so a declared, non-empty `templates` legitimately extends the
/// built-in's host with a child template rather than colliding with it.
fn row_relocates_builtin(
    row: &drift::KindFactRow,
    builtin: &CustomKind,
) -> Result<bool, drift::LockRowError> {
    let declared = CustomKind::from_kind_fact_row(row)?;
    Ok(
        (declared.format.is_none() || declared.format == builtin.format)
            && (declared.unit_shape.is_none() || declared.unit_shape == builtin.unit_shape)
            && (declared.registration.is_empty() || declared.registration == builtin.registration),
    )
}

/// Partition the lock's declared kind rows for the three sites (`explain`, `gate`,
/// `collect_directive_members`) that dispatch every non-built-in kind through the
/// generic custom-kind path: a row naming no built-in is genuinely custom (returned in
/// declaration order); a row naming a built-in it does not [`row_relocates_builtin`]
/// with is a collision (KIND-NAME-COLLISION-ADMISSIBILITY), reported separately so the
/// one caller that can diagnose (`gate`) does, and every caller skips it identically —
/// consolidating what were three duplicated `if builtin_defs.contains_key(...) {
/// continue }` sites. A row that *does* relocate a built-in is neither: it is silently
/// consumed by that built-in's own [`resolve_kind_units`] call, exactly as before.
#[allow(clippy::type_complexity)]
fn partition_kind_rows<'a>(
    declarations: &'a drift::Declarations,
    builtin_defs: &BTreeMap<String, CustomKind>,
) -> Result<(Vec<&'a drift::KindFactRow>, Vec<&'a drift::KindFactRow>), drift::LockRowError> {
    let mut custom = Vec::new();
    let mut collisions = Vec::new();
    for row in &declarations.kinds {
        match builtin_defs.get(&row.name) {
            None => custom.push(row),
            Some(builtin) if !row_relocates_builtin(row, builtin)? => collisions.push(row),
            Some(_) => {}
        }
    }
    Ok((custom, collisions))
}

/// A [`KIND_COLLISION_RULE`] finding for a colliding row from [`partition_kind_rows`] —
/// refusing rather than the silent skip that dropped the row's members from every
/// corpus with no diagnostic (KIND-NAME-COLLISION-ADMISSIBILITY).
fn kind_collision_diagnostic(row: &drift::KindFactRow) -> check::Diagnostic {
    check::Diagnostic::error(
        KIND_COLLISION_RULE,
        &row.name,
        format!(
            "kind `{}` collides with an embedded built-in of the same name: its declared \
             shape (`format`/`unit_shape`/`registration`) does not match the built-in's, so \
             it is neither an admissible relocation of the built-in's `governs` locus nor a \
             distinct custom kind of its own — rename it, or drop the diverging facts to \
             relocate the built-in instead",
            row.name
        ),
    )
}

/// The diagnostic `rule` id two clause rows sharing one address report under. Sibling of
/// [`KIND_COLLISION_RULE`] and [`GOVERNS_COLLISION_RULE`], one namespace down: those guard
/// the kind's bare name and its locus, this one guards the clause's own address.
const CLAUSE_COLLISION_RULE: &str = "clause.label-collision";

/// A [`CLAUSE_COLLISION_RULE`] finding per address worn by more than one clause row —
/// the whole declaration set at once, kinds' own clauses and requirements' nested ones
/// alike, since a requirement's address shares the namespace.
///
/// A clause's label is its identity: the name every one of its findings prints and the
/// only name a dial can reach it by. Two rows under one label leave both unaddressable —
/// a dial entry would silently hit whichever the reader resolved first, and a finding
/// would name a clause the author cannot tell apart from its twin. That is a malformed
/// lock, refused before it judges anything, never a collision resolved with a counter
/// that would renumber a clause's siblings every time one is inserted above it.
///
/// `joined` faces the same rule: a layer's rows share the address space they are judged
/// in, so two of them under one label leave both unaddressable exactly as the host's own
/// twins would. A joined row can never collide with a *host* row — its address carries the
/// layer that produced it ([`LAYER_QUALIFIER`]) — so what fires here is a malformed layer
/// or a malformed corpus, never the join itself.
fn clause_collision_diagnostics(
    declarations: &drift::Declarations,
    joined: &[drift::ClauseRow],
) -> Vec<check::Diagnostic> {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    let rows = declarations
        .clauses
        .iter()
        .chain(declarations.requirements.iter().flat_map(|r| &r.clauses))
        .chain(joined);
    for row in rows {
        if let Some(label) = &row.label {
            *counts.entry(label.as_str()).or_default() += 1;
        }
    }
    counts
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .map(|(label, count)| {
            check::Diagnostic::error(
                CLAUSE_COLLISION_RULE,
                label,
                format!(
                    "{count} clause rows share the address `{label}`, so neither can be \
                     named: a clause's label is its identity — the id its findings print \
                     and the only name a dial reaches it by — and one address can address \
                     one clause"
                ),
            )
        })
        .collect()
}

/// The diagnostic `rule` id a kind declaring an inadmissible commitment class reports
/// under. Sibling of [`GOVERNS_COLLISION_RULE`]: both guard the locus's own coherence
/// before the corpus is trusted to model the harness — that one across kinds, this one
/// within a kind.
const LOCAL_LOCUS_RULE: &str = "kind.local-locus";

/// A [`LOCAL_LOCUS_RULE`] finding per kind whose declared `local` commitment class is
/// inadmissible — the locus fence ([`CustomKind::local_locus_fault`]), raised over every
/// kind in play before their members are read.
fn local_locus_admissibility(
    builtin_defs: &BTreeMap<String, CustomKind>,
    custom_rows: &[&drift::KindFactRow],
    declarations: &drift::Declarations,
) -> Result<Vec<check::Diagnostic>, drift::LockRowError> {
    let mut kinds = Vec::new();
    for kind in builtin_defs.values() {
        kinds.push(overlay_builtin_kind(kind, declarations)?);
    }
    for row in custom_rows {
        kinds.push(CustomKind::from_kind_fact_row(row)?);
    }
    Ok(kinds
        .iter()
        .filter_map(|kind| {
            let fault = kind.local_locus_fault()?;
            Some(check::Diagnostic::error(
                LOCAL_LOCUS_RULE,
                &kind.name,
                format!(
                    "kind `{}` declares the `local` commitment class, but {fault}",
                    kind.name
                ),
            ))
        })
        .collect())
}

/// The diagnostic `rule` id for two distinct kinds resolving to the same `governs`
/// (root+glob) locus. Sibling of [`KIND_COLLISION_RULE`], which guards the bare-name
/// namespace; this one guards the locus namespace — a document's kind is its position
/// alone, so two kinds selecting the same locus is a routing ambiguity, not two homes.
const GOVERNS_COLLISION_RULE: &str = "kind.governs-collision";

/// Governs-glob-collision findings over the **effective** kind set: the built-in
/// definitions, each overlaid with any [`row_relocates_builtin`] row that moves its
/// locus, plus the genuinely-custom rows. Two distinct kinds resolving to the same
/// `governs` (root+glob) would silently double-route every matching document into both
/// member sets — a document's kind is its position alone (`representation.md`) — so each
/// shared locus surfaces one error naming the kinds and the glob. A relocation is
/// resolved before comparison, so moving a built-in's locus to a fresh path is never a
/// self-collision; moving it *onto* a custom kind's locus is.
///
/// Manifest kinds (a `collection_address`) are excluded: they register members at an
/// address *inside* a host manifest, never claiming the whole document, so a manifest
/// kind legitimately shares its host file with the file-locus kind that owns it whole (a
/// `hook` and a `settings.json`-owning kind coexist). The mining the spec forbids is two
/// *document* kinds contending for one file; two manifest kinds contending for one
/// collection address is a distinct, out-of-scope question.
///
/// A nested file kind falls out for a stronger reason: it governs no glob at all — its
/// members' paths compose from their host's unit and the host template's pattern — so it
/// enters no bucket and can contend with nobody. That is the whole point of the locus.
fn governs_collision_diagnostics(
    builtin_defs: &BTreeMap<String, CustomKind>,
    custom_rows: &[&drift::KindFactRow],
    declarations: &drift::Declarations,
) -> Result<Vec<check::Diagnostic>, drift::LockRowError> {
    let mut by_governs: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    for kind in builtin_defs.values() {
        let overlaid = overlay_builtin_kind(kind, declarations)?;
        let Some(governs) = overlaid.governs else {
            continue;
        };
        if overlaid.collection_address.is_some() {
            continue;
        }
        by_governs
            .entry((governs.root, governs.glob))
            .or_default()
            .push(kind.name.clone());
    }
    for row in custom_rows {
        let Some(governs) = row.governs_root.clone().zip(row.governs_glob.clone()) else {
            continue;
        };
        if row.collection_address.is_some() {
            continue;
        }
        by_governs
            .entry(governs)
            .or_default()
            .push(row.name.clone());
    }
    Ok(by_governs
        .into_iter()
        .filter(|(_, names)| names.len() > 1)
        .map(|((root, glob), mut names)| {
            names.sort();
            governs_collision_diagnostic(&root, &glob, &names)
        })
        .collect())
}

/// A [`GOVERNS_COLLISION_RULE`] finding naming every kind that shares the
/// `root`/`glob` locus and the glob itself.
fn governs_collision_diagnostic(root: &str, glob: &str, names: &[String]) -> check::Diagnostic {
    let named = names
        .iter()
        .map(|name| format!("`{name}`"))
        .collect::<Vec<_>>()
        .join(", ");
    check::Diagnostic::error(
        GOVERNS_COLLISION_RULE,
        format!("{root}/{glob}"),
        format!(
            "kinds {named} share the `governs` glob `{root}/{glob}` — a document's kind is \
             its position alone, so two kinds selecting the same locus would route every \
             matching document into both member sets; give each kind a distinct `governs`",
        ),
    )
}

/// Lift the lock's [`drift::RequirementRow`] — the whole requirement shape `import`
/// wrote — into the [`compose::Requirement`] the roster/coverage/graph tiers
/// already take.
fn requirement_from_row(
    row: &drift::RequirementRow,
) -> Result<compose::Requirement, compose::ClauseRowError> {
    Ok(compose::Requirement {
        name: row.name.clone(),
        prose: row.prose.clone(),
        kind: row.kind.clone(),
        required: row.required,
        clauses: row
            .clauses
            .iter()
            .map(clause_from_row)
            .collect::<Result<Vec<_>, _>>()?,
        verifier: row.verifier.clone(),
    })
}

/// Lift one of a requirement row's nested [`drift::ClauseRow`]s into a
/// [`contract::Clause`] — the mirror of [`requirement_from_row`] for the set-/edge-scope
/// demand it carries, via the shared [`compose::clause_from_row`] lift. A
/// requirement-nested row's guidance/source isn't carried the same way as a
/// kind-level clause's, so both are overwritten to `None` on success rather than
/// passed through.
///
/// # Errors
///
/// Propagates the [`compose::ClauseRowError`] the shared lift raises for a row the
/// closed vocabulary cannot admit — rejected loud, never a silently dropped clause.
fn clause_from_row(row: &drift::ClauseRow) -> Result<contract::Clause, compose::ClauseRowError> {
    compose::clause_from_row(row).map(|clause| contract::Clause {
        guidance: None,
        source: None,
        ..clause
    })
}

/// The assembly's declared edges off the lock's `assembly` fact family — every
/// `fact = "edge"` row. A present edge row missing a required `field`/`from`/`to` column
/// is a load error naming the assembly family, never a silently absent edge — as is a
/// `to` that names no kind at all.
///
/// # Errors
///
/// Returns a [`drift::LockRowError`] when a present edge fact omits a required column or
/// declares an empty target set.
fn edges_from_declarations(
    declarations: &drift::Declarations,
) -> Result<Vec<compose::Edge>, drift::LockRowError> {
    declarations
        .assembly
        .iter()
        .filter(|fact| fact.fact == "edge")
        .map(|fact| {
            let to: Vec<String> = edge_column(fact.to.clone(), "to")?;
            // An edge declaring no target kind can never resolve — loading it would
            // silently narrow the gate to a route it can never judge.
            if to.is_empty() {
                return Err(drift::LockRowError::WrongType {
                    family: "assembly".to_string(),
                    column: "to".to_string(),
                    want: "non-empty set of target kinds".to_string(),
                });
            }
            Ok(compose::Edge {
                field: edge_column(fact.field.clone(), "field")?,
                from: edge_column(fact.from.clone(), "from")?,
                to,
            })
        })
        .collect()
}

/// One required column off a present `edge` assembly fact — an absent one is a load error
/// naming the assembly family, the same reject a malformed row takes at load.
fn edge_column<T>(value: Option<T>, column: &str) -> Result<T, drift::LockRowError> {
    value.ok_or_else(|| drift::LockRowError::MissingColumn {
        family: "assembly".to_string(),
        column: column.to_string(),
    })
}

/// The lock's already-resolved `mention` rows, lifted into [`graph::ResolvedEdge`]s —
/// the mention-family mirror of [`edges_from_declarations`]: no field lookup (a mention
/// is resolved once, at emit), just the address parse [`graph::resolved_mention_edges`]
/// runs.
fn mention_edges_from_declarations(declarations: &drift::Declarations) -> Vec<graph::ResolvedEdge> {
    let mentions: Vec<graph::MentionDeclaration> = declarations
        .mentions
        .iter()
        .map(|row| graph::MentionDeclaration {
            member: row.member.clone(),
            target: row.target.clone(),
        })
        .collect();
    graph::resolved_mention_edges(&mentions)
}

/// Extract import edges from an already-parsed lock document, avoiding redundant
/// reads/parses.
///
/// # Errors
///
/// Returns a [`drift::DriftError`] when a present source-dependency row is malformed.
fn import_edges_from_doc(doc: &DocumentMut) -> miette::Result<Vec<graph::ResolvedEdge>> {
    let layouts = drift::layout_imports_from_doc(doc)?;
    let includes = drift::includes_from_doc(doc)?;
    let imports: Vec<graph::ImportDeclaration> = layouts
        .into_iter()
        .chain(includes)
        .filter(|row| !row.target.is_empty())
        .map(|row| graph::ImportDeclaration {
            member: row.member,
            target: row.target,
        })
        .collect();
    Ok(graph::resolved_import_edges(&imports))
}

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
