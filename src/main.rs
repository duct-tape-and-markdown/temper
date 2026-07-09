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
use temper::document;
use temper::drift;
use temper::engine;
use temper::extract;
use temper::frontmatter;
use temper::graph;
use temper::import;
use temper::install;
use temper::kind::{self, CustomKind, Unit};
use temper::read;
use temper::reporter;
use temper::roster;
use temper::schema;

/// The surface workspace default for `--into` / the `check` argument:
/// a `.temper` directory under the cwd.
const DEFAULT_WORKSPACE: &str = "./.temper";

/// The surface workspace directory beside a harness root.
/// Session-start's surface-present branch gates this directly; its surfaceless
/// branch gates the harness root directly instead.
const TEMPER_DIR: &str = ".temper";

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

/// The built-in default contracts keyed by their bare row label.
fn builtin_default_contracts() -> miette::Result<Vec<(String, Contract)>> {
    let mut contracts = Vec::with_capacity(BUILTIN_DEFAULT_CONTRACT_KINDS.len());
    for kind in BUILTIN_DEFAULT_CONTRACT_KINDS {
        contracts.push(((*kind).to_string(), builtin_default_contract(kind)?));
    }
    Ok(contracts)
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
    /// Lint the config surface against the active contract. Session-start is a
    /// **reporter** of this gate, never a verb: `--reporter session-start` reads the path as a harness root and is
    /// advisory (always exits zero), so a Claude Code `SessionStart` hook runs
    /// `temper check . --reporter session-start`.
    Check {
        /// The surface workspace to lint (defaults to `./.temper`); with `--reporter
        /// session-start` it is read as a *harness root* instead (defaults to `.`).
        workspace: Option<PathBuf>,
        /// One-shot mode: lint a raw harness directly — its own `.temper/` surface
        /// gates on its lock when one is already present, else the harness root is
        /// imported internally into a throwaway surface; either way the identical
        /// by-kind gate runs and no workspace is written. Conflicts with `workspace`.
        #[arg(long, conflicts_with = "workspace")]
        harness: Option<PathBuf>,
        /// Also fail the run on `advisory` (warn-severity) violations, not just
        /// `required` ones — the strict CI policy.
        #[arg(long)]
        deny_advisories: bool,
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
        #[arg(long, default_value = DEFAULT_WORKSPACE)]
        into: PathBuf,
        /// Refuse network access — the CI posture.
        /// `emit` performs no network I/O today, so this is accepted for CI parity.
        #[arg(long)]
        frozen: bool,
        /// Compute and report every projection without writing a single byte — not
        /// the re-emitted sources, not the updated lock.
        #[arg(long)]
        dry_run: bool,
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
        #[arg(default_value = DEFAULT_WORKSPACE)]
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
            workspace,
            harness,
            deny_advisories,
            reporter,
        } => {
            // Session-start is a reporter, not a verb: it reads the path as a *harness root* (surface-present or
            // gated directly off disk), emits the payload, and is advisory — always exits
            // zero, so a failing contract routes through the human, never blocks the
            // session.
            let diagnostics = if reporter == Reporter::SessionStart {
                let harness_path = harness.or(workspace).unwrap_or_else(|| PathBuf::from("."));
                harness_diagnostics(&harness_path)?
            } else {
                // Two ways into the same gate. `--harness` is the one-shot wedge: gate the
                // harness root directly. Without it, the two-step path gates an
                // already-imported surface over its harness root (the cwd). Same
                // diagnostic shape ⇒ shared render.
                match harness {
                    Some(harness) => harness_diagnostics(&harness)?,
                    None => {
                        let workspace =
                            workspace.unwrap_or_else(|| PathBuf::from(DEFAULT_WORKSPACE));
                        // Two-step: the surface *is* the imported members, rooted at the cwd.
                        gate(&workspace, Path::new("."))?
                    }
                }
            };

            match reporter {
                Reporter::Terminal => print!("{}", check::render(&diagnostics)),
                Reporter::Github => print!("{}", reporter::github(&diagnostics)),
                Reporter::Sarif => println!("{}", reporter::sarif(&diagnostics)),
                Reporter::SessionStart => {
                    println!("{}", reporter::session_start(&diagnostics));
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
            // emit the *active* contract per kind — the same default contract ⊕
            // lock-declared clause overrides `check` gates against — as an editor
            // JSON Schema.
            let declarations = drift::read_declarations(Path::new(DEFAULT_WORKSPACE))?;

            // Keyed by each kind's bare row label.
            let default_contracts = builtin_default_contracts()?;

            let json = match kind.as_deref() {
                // An unknown kind is a hard error, never a silent empty schema.
                Some(requested) => {
                    let default_contract = default_contracts
                        .into_iter()
                        .find(|(name, _)| name == requested);
                    let (name, default_contract) = default_contract.ok_or_else(|| {
                        miette::miette!("unknown kind `{requested}` (temper models: skill, rule)")
                    })?;
                    let contract =
                        compose::effective(&declarations.clauses, &name, default_contract);
                    schema::emit(&contract)
                }
                None => {
                    let mut map = serde_json::Map::new();
                    for (name, default_contract) in default_contracts {
                        let contract =
                            compose::effective(&declarations.clauses, &name, default_contract);
                        map.insert(name, schema::emit(&contract));
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
        } => {
            // The seam:
            // `node` runs the SDK program at `<into>/harness.ts`, and the engine becomes the
            // sole compiler of every projection and the whole lock from its JSON payload — no
            // harness root is re-supplied here, the payload IS the source.
            let report = drift::emit_program(&into, drift::EmitOptions { dry_run, frozen })?;
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
            let workspace_dir = path.join(TEMPER_DIR);
            let mode = mode_from_lock(&workspace_dir);
            let lock_present = workspace_dir.join("lock.toml").is_file();
            let targets = drift::emit_owned_targets(&workspace_dir);
            let mut payload = String::new();
            io::Read::read_to_string(&mut io::stdin(), &mut payload).into_diagnostic()?;
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
        Command::Install {
            path,
            yes,
            no_represent,
            dry_run,
        } => {
            let discovery = install::discover(&path)?;
            print!("{}", install::render_discovery(&discovery));

            let represent = if yes {
                install::Represent::Yes
            } else if no_represent {
                install::Represent::No
            } else {
                ask_represent()?
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
    let workspace = PathBuf::from(DEFAULT_WORKSPACE);
    let harness_root = Path::new(".");

    // The assembly's own declared facts, read first: the corpus below walks each
    // kind's governs locus off *this*.
    let declarations = drift::read_declarations(&workspace)?;

    let skill_kind = builtin_kind::definition("skill")?
        .ok_or_else(|| miette::miette!("built-in kind `skill` is not embedded in this binary"))?;
    let rule_kind = builtin_kind::definition("rule")?
        .ok_or_else(|| miette::miette!("built-in kind `rule` is not embedded in this binary"))?;
    let skill_features = kind_features(&skill_kind, harness_root, &declarations)?;
    let rule_features = kind_features(&rule_kind, harness_root, &declarations)?;

    // Every lock-declared kind that is not a built-in — the same synthesis `gate` runs
    // (READ-EDGE-UNIFY), so a read cannot disagree with the gate about which kinds and
    // members exist.
    let builtin_defs = builtin_kind::definitions()?;
    let mut custom_kinds: Vec<CustomKindEntry> = Vec::new();
    let mut custom_members: Vec<read::CustomMember> = Vec::new();
    let (custom_rows, _collisions) = partition_kind_rows(&declarations, &builtin_defs);
    for row in custom_rows {
        let custom_kind = CustomKind::from_kind_fact_row(row);
        let units = resolve_kind_units(&custom_kind, harness_root, &declarations)?;
        let features: Vec<extract::Features> = units
            .iter()
            .map(|unit| builtin_kind::features(&custom_kind, unit))
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
    let by_kind = assemble_by_kind(&skill_features, &rule_features, &custom_kinds);

    let assembly_requirements: BTreeMap<String, compose::Requirement> = declarations
        .requirements
        .iter()
        .map(|row| (row.name.clone(), requirement_from_row(row)))
        .collect();
    let assembly_edges = edges_from_declarations(&declarations);
    let mention_edges = mention_edges_from_declarations(&declarations);

    // The one requirement namespace: the assembly's declared `[requirement.*]`
    // roster — a custom-kind member has no channel of its own to publish one (the
    // pre-0016 own-path surface that once carried it is retired).
    let roster = assembly_requirements.clone();

    // The world's inbound registration channel set into each built-in kind — the same
    // derivation the gate's `reachable` runs, keyed by bare kind name to join `by_kind`.
    let mut registrations: BTreeMap<&str, Vec<kind::Registration>> = BTreeMap::new();
    for def in builtin_defs.values() {
        if !def.registration.is_empty() {
            registrations.insert(def.name.as_str(), def.registration.clone());
        }
    }

    let repo_files = repo_file_set(Path::new("."));
    let directive_members = collect_directive_members(harness_root, &declarations)?;
    let directive_edges = graph::classify_directives(&directive_members, &repo_files).edges;

    // Citations — the declared one-way edges naming a leaf; the floor carries no
    // producer yet, so the set is empty.
    let citations: Vec<read::Citation> = Vec::new();

    Ok(read::explain(
        &custom_members,
        &assembly_requirements,
        &roster,
        &by_kind,
        &assembly_edges,
        &mention_edges,
        &registrations,
        &repo_files,
        &directive_edges,
        &citations,
        target,
    ))
}

/// Read the `guard`'s enforcement mode live off a harness's lock:
/// the root member's `mode`
/// fact in `<workspace_dir>/lock.toml`'s assembly declaration rows. An unrepresented
/// harness (no lock, or one predating the field) reads
/// [`compose::EnforcementMode::default`] — `warn` — matching the lock-less
/// "nothing to bind" posture everywhere else in this module.
fn mode_from_lock(workspace_dir: &Path) -> compose::EnforcementMode {
    drift::read_declarations(workspace_dir)
        .unwrap_or_default()
        .assembly
        .iter()
        .find(|row| row.fact == "mode")
        .and_then(|row| row.value.as_deref())
        .and_then(|value| match value {
            "note" => Some(compose::EnforcementMode::Note),
            "warn" => Some(compose::EnforcementMode::Warn),
            "block" => Some(compose::EnforcementMode::Block),
            _ => None,
        })
        .unwrap_or_default()
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

/// The one-shot gate over a harness root — shared by `check --harness` and the
/// session-start reporter: surface-present ⇒ gate the authored `.temper/` itself, so
/// its lock's declared requirement/satisfies/clause rows are read; surfaceless ⇒ gate
/// the harness root directly — the discovery walk finds its members straight off
/// disk, against the kind's embedded `governs` (the built-in lock), with an empty
/// declaration set (never adopted, nothing to read).
///
/// The surface-present branch never re-imports: a fresh import discards recognition (the
/// authored `satisfies` links), so every filled requirement would read unfilled — the
/// false positive on clean input the surface-present clause forbids. It also supplies
/// `gate` with the one place a harness's declared requirement/satisfies/clause rows
/// live: reading `harness_path` alone finds no lock at all, so a harness gated only off
/// its raw root would evaluate against an empty declaration set.
fn harness_diagnostics(harness_path: &Path) -> miette::Result<Vec<check::Diagnostic>> {
    let authored = harness_path.join(TEMPER_DIR);
    if authored.is_dir() {
        gate(&authored, harness_path)
    } else {
        gate(harness_path, harness_path)
    }
}

/// Produce the merged diagnostic set for a surface `workspace` against the active
/// by-kind contracts — the shared gate behind both `check` and the session-start
/// reporter. `harness_root` is the
/// directory a member's source path and a `verified_by` path resolve against (the
/// CWD for a two-step `check`, the harness path for the one-shot gate).
fn gate(workspace: &Path, harness_root: &Path) -> miette::Result<Vec<check::Diagnostic>> {
    // The assembly's own declared facts — requirements and edges — ride the lock's
    // declaration rows: `emit` is the sole
    // producer, this is the gate's one read of it.
    // Never gated on a lock's presence — an unadopted harness's lock declares
    // nothing, so this tier is a no-op over it rather than skipped (never a
    // half-adopted state).
    let declarations = drift::read_declarations(workspace)?;
    let assembly_requirements: BTreeMap<String, compose::Requirement> = declarations
        .requirements
        .iter()
        .map(|row| (row.name.clone(), requirement_from_row(row)))
        .collect();
    let assembly_edges = edges_from_declarations(&declarations);
    let mention_edges = mention_edges_from_declarations(&declarations);

    // The generic two-greens over EVERY embedded built-in kind, keyed by its bare row
    // label: each kind's members — resolved by
    // [`kind_features`] straight off harness disk, shared with `explain`
    // (READ-EDGE-UNIFY) so a read cannot disagree with the gate about which members
    // exist — are dispatched to its default contract and validated, so a discovered `CLAUDE.md`
    // memory member fires its `memory` clauses exactly as a skill/rule does — no
    // longer silently skipped by a hardcoded skill/rule pair.
    // SCOPE: only this validation path generalizes — the roster/graph tier below
    // stays skill/rule/custom (no memory member publishes a requirement today;
    // folding more built-ins into the requirement corpus is a separate scope
    // question), so `skill`/`rule` are captured out of the dispatch below into
    // `skill_features`/`rule_features`.
    let mut diagnostics = Vec::new();
    // Per-kind checked-member counts, keyed by bare row label — carried out of
    // the dispatch loop for the advisory coverage note below (WEDGE-COVERAGE-NOTE),
    // so "checked N members" is stated rather than left as bare silence.
    let mut member_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut skill_features: Vec<extract::Features> = Vec::new();
    let mut rule_features: Vec<extract::Features> = Vec::new();
    let builtin_defs = builtin_kind::definitions()?;
    for kind in builtin_defs.values() {
        // Two greens: admissibility — the contract validated
        // against the definition before it is trusted to judge — then conformance.
        // The per-kind clause overrides source from the lock's declared `clauses`.
        let contract = compose::effective(
            &declarations.clauses,
            &kind.name,
            builtin_default_contract(&kind.name)?,
        );

        let features = kind_features(kind, harness_root, &declarations)?;

        diagnostics.extend(engine::admissibility(&contract));
        diagnostics.extend(engine::validate(&contract, &features));
        member_counts.insert(kind.name.clone(), features.len());
        match kind.name.as_str() {
            "skill" => skill_features = features,
            "rule" => rule_features = features,
            _ => {}
        }
    }

    // Every lock-declared kind that is not one of the embedded built-ins:
    // a
    // built-in's own row is only the governs-override `effective_governs` already
    // consumes, never a second kind definition. A custom kind carries no embedded
    // default — its whole default contract is the committed lock's own clause rows
    // naming it ([`compose::default_contract_from_rows`]) — but is otherwise
    // dispatched through the identical two-greens the built-in loop above runs.
    let mut custom_kinds: Vec<CustomKindEntry> = Vec::new();
    let (custom_rows, collisions) = partition_kind_rows(&declarations, &builtin_defs);
    // The one site among the three dispatchers that can surface a diagnostic.
    diagnostics.extend(collisions.iter().map(|row| kind_collision_diagnostic(row)));
    for row in custom_rows {
        let custom_kind = CustomKind::from_kind_fact_row(row);
        let contract = compose::default_contract_from_rows(&declarations.clauses, &row.name);
        let features = kind_features(&custom_kind, harness_root, &declarations)?;

        diagnostics.extend(engine::admissibility(&contract));
        diagnostics.extend(engine::validate(&contract, &features));
        member_counts.insert(row.name.clone(), features.len());
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
            &collect_directive_members(harness_root, &declarations)?,
            &repo_files,
        )
        .findings
        .into_iter()
        .map(|mut finding| {
            finding.severity = Severity::Warn;
            finding
        }),
    );

    // The harness-contract tier: set-scope predicates over the parsed roster, each
    // quantified over a requirement's satisfier set.
    // Runs unconditionally — the lock is the sole source of assembly facts now, so an
    // unadopted harness's empty declarations make this tier a no-op rather than a
    // skip.
    let edges = assembly_edges.clone();

    // The by-kind corpus every set-scope and graph predicate ranges over,
    // assembled through the same helper the read arm uses.
    let by_kind = assemble_by_kind(&skill_features, &rule_features, &custom_kinds);

    // Every opt-in-capable member's features (built-in kinds *and* each custom
    // kind's members) — the stream coverage ranges over below.
    let all_features: Vec<extract::Features> = skill_features
        .iter()
        .chain(rule_features.iter())
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

    // The set-scope predicates: each requirement's `count` / `unique` /
    // `membership` gate over its satisfier set.
    diagnostics.extend(roster::check(&requirements, &by_kind));

    // The edge scope: build the reference graph over the declared edges and check route
    // resolution — a declared reference must resolve to a real artifact of the
    // target kind. Admissibility before conformance:
    // an edge naming no reference field or targeting an unmodeled kind is
    // reported once and skipped by the route check.
    diagnostics.extend(graph::admissibility(&edges, &by_kind));
    diagnostics.extend(graph::check(&edges, &by_kind));

    // `acyclic`: the resolved graph must contain no
    // cycle — a circular import loads nothing, so every finding is a true
    // positive. Always-on over the whole edge set, like route resolution above.
    diagnostics.extend(graph::acyclic(&edges, &by_kind));

    // `degree`: a requirement declares an in/out
    // edge-count bound every satisfier's degree must fall inside, so it takes
    // the requirements *and* the edges, reusing the arc resolution
    // `acyclic`/`check` assemble, plus the already-resolved mention edges — obligation-free
    // by default, counted only when a `degree` clause opts in. Opt-in per requirement.
    diagnostics.extend(graph::degree(
        &assembly_requirements,
        &edges,
        &mention_edges,
        &by_kind,
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
        &builtin_kind::definitions()?,
        &member_counts,
    ));

    // The fail-loud coherence tripwire: the harness was adopted (its own `.temper/lock.toml`
    // declares requirements) but the gate resolved none of them and the workspace it
    // was actually pointed at carries no declaration rows either — the harness-root
    // `temper check .` case the wave-end confirmation caught (checked 0 members, exit
    // 0). Read independently off `harness_root` — never `workspace`, which is the very
    // thing that can be mis-rooted — so a correctly-rooted check (≥1 resolved member)
    // and a genuinely empty (never-adopted) harness both stay silent.
    let declared = !drift::read_declarations(&harness_root.join(TEMPER_DIR))?
        .requirements
        .is_empty();
    let resolved_members: usize = member_counts.values().sum();
    let declarations_empty = declarations.kinds.is_empty()
        && declarations.clauses.is_empty()
        && declarations.requirements.is_empty()
        && declarations.assembly.is_empty()
        && declarations.satisfies.is_empty();
    diagnostics.extend(check::empty_assembly_incoherence(
        harness_root,
        declared,
        resolved_members,
        declarations_empty,
    ));

    // The freshness fact: a committed projection
    // whose bytes no longer match the lock's emit fingerprint is `config.stale`. Read
    // off the surface `workspace`'s lock (where the members were imported and the
    // fingerprints recorded), advisory so a hand-edited or un-re-emitted projection is
    // surfaced without failing the run.
    diagnostics.extend(drift::config_stale(workspace));

    Ok(diagnostics)
}

/// This kind's effective `governs` locus: the
/// committed lock's own kind-fact row when the lock
/// declares one for it — matched by bare name, the kind's whole identity
/// — or the kind's own embedded `governs` when it doesn't: the
/// **built-in lock**, the same declaration shape the engine carries compiled-in for an
/// unadopted harness.
fn effective_governs(kind: &CustomKind, declarations: &drift::Declarations) -> kind::Governs {
    declarations
        .kinds
        .iter()
        .find(|row| row.name == kind.name && row_relocates_builtin(row, kind))
        .map(|row| kind::Governs {
            root: row.governs_root.clone(),
            glob: row.governs_glob.clone(),
        })
        .unwrap_or_else(|| kind.governs.clone())
}

/// A kind's members, resolved live off disk — the one corpus both `gate` and `explain`
/// range over. Every
/// member is discovered by walking this kind's [`effective_governs`]
/// locus, read straight off harness disk so the corpus can never drift from a stale
/// copy; its `satisfies` fill edges come from the lock's own
/// [`SatisfiesRow`](drift::SatisfiesRow) family, keyed by member id — the real
/// SDK-emit shape and the only source a converted harness ever populates. Its
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
    harness_root: &Path,
    declarations: &drift::Declarations,
) -> miette::Result<Vec<Unit>> {
    let governs = effective_governs(kind, declarations);
    let mut units = Vec::new();
    for file in import::discover_kind_files(harness_root, kind, &governs)? {
        let source = frontmatter::Member::from_source_rooted(
            kind,
            &file,
            &harness_root.join(&governs.root),
        )?;
        let mut unit = Unit {
            id: source.id.clone(),
            frontmatter: source.fields.iter().cloned().collect(),
            body: source.body.clone(),
            source_path: source.provenance.source_path.clone(),
            satisfies: Vec::new(),
            satisfies_clauses: Vec::new(),
        };
        for row in &declarations.satisfies {
            if row.member == unit.id && !unit.satisfies.contains(&row.requirement) {
                unit.satisfies.push(row.requirement.clone());
            }
            if row.member == unit.id
                && !unit
                    .satisfies_clauses
                    .iter()
                    .any(|clause| clause.requirement == row.requirement)
            {
                unit.satisfies_clauses
                    .push(document::Satisfies::new(row.requirement.clone()));
            }
        }
        units.push(unit);
    }

    units.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(units)
}

/// A kind's members' extracted [`Features`](extract::Features) — [`resolve_kind_units`]
/// run through the kind's own composed extraction.
///
/// # Errors
///
/// As [`resolve_kind_units`].
fn kind_features(
    kind: &CustomKind,
    harness_root: &Path,
    declarations: &drift::Declarations,
) -> miette::Result<Vec<extract::Features>> {
    Ok(resolve_kind_units(kind, harness_root, declarations)?
        .iter()
        .map(|unit| builtin_kind::features(kind, unit))
        .collect())
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

/// Assemble the by-kind [`Features`](extract::Features) corpus every set-scope and
/// graph predicate ranges over: the built-in kinds plus each lock-declared custom
/// kind's features, keyed by kind name. Borrows every slice, so the caller holds the
/// owned feature vecs for the map's lifetime.
fn assemble_by_kind<'a>(
    skill_features: &'a [extract::Features],
    rule_features: &'a [extract::Features],
    custom_kinds: &'a [CustomKindEntry],
) -> BTreeMap<&'a str, &'a [extract::Features]> {
    let mut by_kind: BTreeMap<&str, &[extract::Features]> =
        BTreeMap::from([("skill", skill_features), ("rule", rule_features)]);
    for (kind, features) in custom_kinds {
        by_kind.insert(kind.name.as_str(), features.as_slice());
    }
    by_kind
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
    harness_root: &Path,
    declarations: &drift::Declarations,
) -> miette::Result<Vec<graph::DirectiveMember>> {
    let mut members = Vec::new();
    let builtin_defs = builtin_kind::definitions()?;
    for kind in builtin_defs.values() {
        for unit in resolve_kind_units(kind, harness_root, declarations)? {
            let feature = builtin_kind::features(kind, &unit);
            members.push(graph::DirectiveMember {
                kind: kind.name.clone(),
                id: feature.id.clone(),
                source_path: unit.source_path.clone(),
                directives: feature.directives.clone(),
            });
        }
    }
    let (custom_rows, _collisions) = partition_kind_rows(declarations, &builtin_defs);
    for row in custom_rows {
        let custom_kind = CustomKind::from_kind_fact_row(row);
        for unit in resolve_kind_units(&custom_kind, harness_root, declarations)? {
            let feature = builtin_kind::features(&custom_kind, &unit);
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
/// (`effective_governs`) that points a built-in's `governs` locus somewhere other than
/// its embedded default, by declaring a row under the built-in's own name. A row only
/// relocates: every fact besides `governs` either agrees with the built-in's own or is
/// left undeclared (deferring to it) — `format`, `unit_shape`, `registration`. A row
/// that declares any of those *differently* is not reconfiguring the built-in, it is a
/// distinct kind's shape wearing the built-in's name — a namespace collision, never
/// silently subsumed into the built-in's walk. `templates` is excluded from this set: a
/// built-in's own `templates` is always empty (nothing populates it outside
/// `from_kind_fact_row`), so a declared, non-empty `templates` legitimately extends the
/// built-in's host with a child template rather than colliding with it.
fn row_relocates_builtin(row: &drift::KindFactRow, builtin: &CustomKind) -> bool {
    let declared = CustomKind::from_kind_fact_row(row);
    (declared.format.is_none() || declared.format == builtin.format)
        && (declared.unit_shape.is_none() || declared.unit_shape == builtin.unit_shape)
        && (declared.registration.is_empty() || declared.registration == builtin.registration)
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
fn partition_kind_rows<'a>(
    declarations: &'a drift::Declarations,
    builtin_defs: &BTreeMap<String, CustomKind>,
) -> (Vec<&'a drift::KindFactRow>, Vec<&'a drift::KindFactRow>) {
    let mut custom = Vec::new();
    let mut collisions = Vec::new();
    for row in &declarations.kinds {
        match builtin_defs.get(&row.name) {
            None => custom.push(row),
            Some(builtin) if !row_relocates_builtin(row, builtin) => collisions.push(row),
            Some(_) => {}
        }
    }
    (custom, collisions)
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

/// Lift the lock's [`drift::RequirementRow`] — the whole requirement shape `import`
/// wrote — into the [`compose::Requirement`]
/// the roster/coverage/graph tiers already take, the mirror of `import`'s own
/// `requirement_row`. The row carries no `means` (`import` never emits it
/// — "`temper` never interprets `means`" — no gate reads it), so it defaults here too.
fn requirement_from_row(row: &drift::RequirementRow) -> compose::Requirement {
    compose::Requirement {
        name: row.name.clone(),
        means: None,
        kind: row.kind.clone(),
        required: row.required,
        clauses: row.clauses.iter().filter_map(clause_from_row).collect(),
        verified_by: row.verified_by.clone(),
    }
}

/// Lift one of a requirement row's nested [`drift::ClauseRow`]s into a
/// [`contract::Clause`] — the mirror of [`requirement_from_row`] for the set-/edge-scope
/// demand it carries, via the shared [`compose::clause_from_row`] lift. A
/// requirement-nested row's guidance/source isn't carried the same way as a
/// kind-level clause's, so both are overwritten to `None` on the `Some` case
/// rather than passed through. A row naming an unrecognized predicate, or missing
/// the argument its predicate requires, degrades to absent — the same tolerant
/// read the rest of the lock takes over hand-editable state
/// (`crate::drift::read_declarations`).
fn clause_from_row(row: &drift::ClauseRow) -> Option<contract::Clause> {
    compose::clause_from_row(row).map(|clause| contract::Clause {
        guidance: None,
        source: None,
        ..clause
    })
}

/// The assembly's declared edges off the lock's `assembly` fact family — every
/// `fact = "edge"` row, tolerantly (a row missing a column is skipped, not errored;
/// `drift::read_declarations`'s tolerance).
fn edges_from_declarations(declarations: &drift::Declarations) -> Vec<compose::Edge> {
    declarations
        .assembly
        .iter()
        .filter(|fact| fact.fact == "edge")
        .filter_map(|fact| {
            Some(compose::Edge {
                field: fact.field.clone()?,
                from: fact.from.clone()?,
                to: fact.to.clone()?,
            })
        })
        .collect()
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
}
