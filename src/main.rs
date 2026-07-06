//! `temper` CLI entry point.
//!
//! A thin `clap` dispatch over the [`temper`] library: parse args, run the
//! generic contract engine, map the result to an exit code. Every subcommand
//! mirrors the CLI surface of `specs/architecture/20-surface.md`; all logic lives in the
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
use temper::check::{self, Severity, Workspace};
use temper::compose;
use temper::contract::{self, Contract};
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

/// The surface workspace default for `--into` / the `check` argument
/// (`specs/architecture/20-surface.md`): a `.temper` directory under the cwd.
const DEFAULT_WORKSPACE: &str = "./.temper";

/// The surface workspace directory beside a harness's `temper.toml`
/// (`specs/architecture/40-composition.md`). Session-start's surface-present branch gates
/// this directly; its surfaceless branch imports into a throwaway scratch surface
/// instead.
const TEMPER_DIR: &str = ".temper";

/// The optional author-declared contract layer, discovered at the project root
/// beside the harness it governs (`specs/architecture/40-composition.md`). Absent ⇒ the
/// by-kind floor runs unchanged.
const TEMPER_TOML: &str = "temper.toml";

/// The gitignored personal override layer discovered beside [`TEMPER_TOML`]
/// (`specs/architecture/40-composition.md`): `temper.toml` is committed project policy,
/// `temper-local.toml` a developer's personal clause/severity override that
/// layers over it. Absent ⇒ the committed layer (or bare floor) runs unchanged.
const TEMPER_LOCAL_TOML: &str = "temper-local.toml";

/// The diagnostic `rule` id a cross-publisher requirement-name collision reports
/// under (`specs/architecture/10-contracts.md`, "Decision: a requirement's publisher is any
/// authored surface document"): one namespace, so two surfaces publishing one name
/// is an admissibility finding, never a shadow. Shares the roster's admissibility
/// tag — a malformed namespace is inadmissible, decided before it judges anything.
const REQUIREMENT_COLLISION_RULE: &str = "requirement.admissibility";

/// Resolve a built-in package by name into its floor [`Contract`], failing loud
/// if the build embedded no package of that name (`specs/architecture/10-contracts.md`) — a
/// missing floor is a hard error, never a silently empty contract.
fn builtin_floor(name: &str) -> miette::Result<Contract> {
    builtin::contract(name)?
        .ok_or_else(|| miette::miette!("built-in package `{name}` is not embedded in this binary"))
}

/// temper's own **published** floor bindings (`specs/architecture/15-kinds.md`, "a published package
/// binds a qualified kind name"): each embedded built-in kind, by the bare name the
/// author writes, paired with the package name its floor loads from. The bare name
/// resolves to its qualified identity through the embedded set ([`builtin_floors`]).
const BUILTIN_FLOOR_BINDINGS: &[(&str, &str)] = &[
    ("skill", builtin::SKILL_PACKAGE),
    ("rule", builtin::RULE_PACKAGE),
];

/// The built-in floors keyed by their **qualified** kind identity (`claude-code.skill`),
/// resolved through the embedded set's provider axis (`specs/architecture/15-kinds.md`, "Decision:
/// kind identity carries a provider axis"). temper ships each package bound to the
/// qualified kind name a consumer's assembly can never mistake for another provider's;
/// a two-provider collision under one bare name would surface as a load error here.
fn builtin_floors() -> miette::Result<Vec<(String, Contract)>> {
    let mut floors = Vec::with_capacity(BUILTIN_FLOOR_BINDINGS.len());
    for (name, package) in BUILTIN_FLOOR_BINDINGS {
        let id = builtin_kind::qualified(name)?.ok_or_else(|| {
            miette::miette!("built-in kind `{name}` is not embedded in this binary")
        })?;
        floors.push((id, builtin_floor(package)?));
    }
    Ok(floors)
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
    /// **reporter** of this gate, never a verb (`specs/architecture/20-surface.md`, "CLI
    /// surface"): `--reporter session-start` reads the path as a harness root and is
    /// advisory (always exits zero), so a Claude Code `SessionStart` hook runs
    /// `temper check . --reporter session-start`.
    Check {
        /// The surface workspace to lint (defaults to `./.temper`); with `--reporter
        /// session-start` it is read as a *harness root* instead (defaults to `.`).
        workspace: Option<PathBuf>,
        /// One-shot mode: lint a raw harness directly — import it internally into
        /// a throwaway surface, run the identical by-kind gate, and write no
        /// workspace (`specs/architecture/20-surface.md`). Conflicts with `workspace`.
        #[arg(long, conflicts_with = "workspace")]
        harness: Option<PathBuf>,
        /// Also fail the run on `advisory` (warn-severity) violations, not just
        /// `required` ones — the strict CI policy (`specs/architecture/10-contracts.md`).
        #[arg(long)]
        deny_advisories: bool,
        /// The machine format for the diagnostic set (`specs/architecture/50-distribution.md`,
        /// reporters). Presentation only — the exit-code verdict is identical
        /// whichever is chosen (session-start excepted: it is always advisory).
        #[arg(long, value_enum, default_value_t = Reporter::Terminal)]
        reporter: Reporter,
    },
    /// Emit the active per-kind contract as an editor JSON Schema (the keystroke
    /// gate — `specs/architecture/50-distribution.md`, "The gate at keystroke").
    Schema {
        /// Emit only this artifact kind's schema (`skill`, `rule`); omitted ⇒ a
        /// JSON object mapping each modeled kind to its schema.
        #[arg(long)]
        kind: Option<String>,
    },
    /// Compile the authoring face: re-emit each projection **whole** from the
    /// surface, byte-deterministically and double-emit verified
    /// (`specs/architecture/20-surface.md`, law 5). Each artifact is regenerated full-file —
    /// byte-stable and idempotent — and written back to the source path its provenance
    /// names; a direct edit to the projection is drift routed to the authored source,
    /// never something emit merges around.
    Emit {
        /// The surface workspace to project (defaults to `./.temper`). The lock
        /// under it carries the emit fingerprints freshness stands on.
        #[arg(long, default_value = DEFAULT_WORKSPACE)]
        into: PathBuf,
        /// Refuse network access — the CI posture (`specs/architecture/20-surface.md`).
        /// `emit` performs no network I/O today, so this is accepted for CI parity.
        #[arg(long)]
        frozen: bool,
        /// Compute and report every projection without writing a single byte — not
        /// the re-emitted sources, not the updated lock.
        #[arg(long)]
        dry_run: bool,
    },
    /// The `PreToolUse` surface-authority guard (`specs/architecture/20-surface.md`, "surface
    /// authority is a declared posture"): read Claude Code's `PreToolUse` payload from
    /// stdin and, when the write targets a `.claude/` projection, inform-and-route under
    /// the `shared` posture (advisory, exit 0) or block under `surface` (exit 2). The
    /// posture is read live from the harness's lock (`.temper/lock.toml`'s `authority`
    /// declaration row) — temper never escalates on its own determination, and an
    /// unrepresented harness (no lock) reads the default `shared`. Wired at the write
    /// boundary by `temper install`.
    Guard {
        /// The harness root whose `.temper/lock.toml` declares the posture (defaults
        /// to the current directory, the project Claude Code runs the hook from).
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// `temper install` — the one on-ramp (`specs/architecture/20-surface.md`, "install —
    /// the front door"): a discovery report, then one question — represent this
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
    /// `marketplace.json` (`specs/architecture/50-distribution.md`): the operate-the-gate skill,
    /// the `SessionStart` hook, and the shipped built-in packages embedded.
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
    /// The one read verb (`specs/architecture/20-surface.md`, "Decision: one read verb —
    /// `explain`"): resolve `<target>` across the member / requirement / leaf-address
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
        /// (`<member>/<genre>/<key>/<field-path>`), or one qualified as
        /// `member:<name>` / `requirement:<name>` / `address:<leaf-address>`.
        target: String,
    },
}

/// The machine format `check` renders its diagnostic set in
/// (`specs/architecture/50-distribution.md`, reporters). Every variant reshapes *presentation
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
    /// session-start reporter (`specs/architecture/50-distribution.md`). It reads the path as a
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
            // Session-start is a reporter, not a verb (`specs/architecture/20-surface.md`,
            // "CLI surface"): it reads the path as a *harness root* (surface-present or
            // gated directly off disk), emits the payload, and is advisory — always exits
            // zero, so a failing contract routes through the human, never blocks the
            // session.
            let diagnostics = if reporter == Reporter::SessionStart {
                let harness_path = harness.or(workspace).unwrap_or_else(|| PathBuf::from("."));
                session_start_diagnostics(&harness_path)?
            } else {
                // Two ways into the same gate. `--harness` is the one-shot wedge: gate the
                // harness root directly against its own `temper.toml` — the discovery walk
                // finds members straight off disk, no import step. Without it, the
                // two-step path gates an already-imported surface. Same diagnostic shape ⇒
                // shared render.
                match harness {
                    Some(harness) => gate(&harness, &harness.join(TEMPER_TOML))?,
                    None => {
                        let workspace =
                            workspace.unwrap_or_else(|| PathBuf::from(DEFAULT_WORKSPACE));
                        // Two-step: the surface *is* the imported members.
                        gate(&workspace, Path::new(TEMPER_TOML))?
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
            // advisory by law 1 (`specs/architecture/20-surface.md`), so it never gates.
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
            // The keystroke placement of the gate (`specs/architecture/50-distribution.md`):
            // emit the *active* contract per kind — the same floor ⊕ `temper.toml`
            // layer `check` gates against — as an editor JSON Schema.
            let layer = load_layer(Path::new(TEMPER_TOML))?;

            // Keyed by each kind's **qualified** identity (`claude-code.skill`), the
            // published-binding form (`specs/architecture/15-kinds.md`).
            let floors = builtin_floors()?;

            let json = match kind.as_deref() {
                // An unknown kind is a hard error, never a silent empty schema. A request
                // resolves either bare (`skill`) or fully qualified (`claude-code.skill`):
                // the qualified identity's bare component is its last dotted segment.
                Some(requested) => {
                    let floor = floors.into_iter().find(|(name, _)| {
                        name == requested || name.rsplit('.').next() == Some(requested)
                    });
                    let (name, floor) = floor.ok_or_else(|| {
                        miette::miette!("unknown kind `{requested}` (temper models: skill, rule)")
                    })?;
                    let contract = compose::effective(layer.as_ref(), &name, floor)?;
                    schema::emit(&contract)
                }
                None => {
                    let mut map = serde_json::Map::new();
                    for (name, floor) in floors {
                        let contract = compose::effective(layer.as_ref(), &name, floor)?;
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
            // The seam (`specs/architecture/20-surface.md`, "The seam — one implementation"):
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
            // The surface-authority guard at Claude Code's write boundary
            // (`specs/architecture/20-surface.md`): read the `PreToolUse` payload from stdin,
            // and — when it targets a `.claude/` projection — act at the author's declared
            // posture. `shared` informs and routes (exit 0); `surface` blocks (exit 2).
            // temper never escalates past the posture the lock declares — the lock is
            // what names a path a projection, so it is also the sole authority for how
            // firmly that projection is enforced (`20-surface.md`, "surface authority is
            // a declared posture"). An unrepresented harness (no lock) reads the default
            // `shared`, matching `compose::Authority`'s own default.
            let authority = authority_from_lock(&path.join(TEMPER_DIR));
            let mut payload = String::new();
            io::Read::read_to_string(&mut io::stdin(), &mut payload).into_diagnostic()?;
            Ok(match install::guard(&payload, authority) {
                install::GuardVerdict::Allow => ExitCode::SUCCESS,
                install::GuardVerdict::Warn => {
                    eprintln!("{}", install::GUARD_MESSAGE);
                    ExitCode::SUCCESS
                }
                install::GuardVerdict::Block => {
                    eprintln!("{}", install::GUARD_MESSAGE);
                    ExitCode::from(2)
                }
            })
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

/// Narrate `target` through the one read verb (`specs/architecture/20-surface.md`, "Decision:
/// one read verb — `explain`"): assemble the same by-kind feature corpus, composed
/// requirement roster, declared edges, activations, and directive/reachability inputs
/// the gate's own predicates range over (READ-EDGE-UNIFY) — over the standard `.temper`
/// workspace and the harness at the CWD, mirroring `check`'s own two-step corpus
/// assembly (`gate`) — and dispatch through [`read::explain`]'s target-species
/// resolution. Custom kinds retire with the `KIND.md` file format
/// (`specs/architecture/15-kinds.md`), so no custom-kind members or edges are threaded in
/// yet; the plumbing is ready for the SDK path that replaces it.
fn explain(target: &str) -> miette::Result<String> {
    let workspace = PathBuf::from(DEFAULT_WORKSPACE);
    let layer = load_layer(Path::new(TEMPER_TOML))?;
    validate_inplace_kinds(layer.as_ref())?;
    let harness_root = Path::new(".");

    // The assembly's own declared facts, read first: the corpus below walks each
    // kind's governs locus off *this* (`specs/architecture/20-surface.md`, "The lock and
    // drift" — mirrors `gate`'s own read, READ-EDGE-UNIFY).
    let declarations = drift::read_declarations(&workspace)?;

    let skill_kind = builtin_kind::definition("skill")?
        .ok_or_else(|| miette::miette!("built-in kind `skill` is not embedded in this binary"))?;
    let rule_kind = builtin_kind::definition("rule")?
        .ok_or_else(|| miette::miette!("built-in kind `rule` is not embedded in this binary"))?;
    let skill_features = kind_features(
        &skill_kind,
        harness_root,
        &workspace,
        layer.as_ref(),
        &declarations,
    )?;
    let rule_features = kind_features(
        &rule_kind,
        harness_root,
        &workspace,
        layer.as_ref(),
        &declarations,
    )?;
    let custom_kinds: Vec<CustomKindEntry> = Vec::new();
    let by_kind = assemble_by_kind(&skill_features, &rule_features, &custom_kinds);

    // `why`/`requirements` read a member's existence and its authored `satisfies`
    // rationale off a `Workspace` + `custom` members (`crate::read`'s own listing), not
    // off `by_kind`. In-place carriage's `temper.toml` `[[member]]` tables carry no
    // rationale (`compose::InPlaceMember::satisfies` is a bare name list), so an
    // in-place skill/rule is synthesized as a rationale-less `CustomMember` straight off
    // the same `skill_features`/`rule_features` `by_kind` already holds. Absent any
    // in-place member, the real `Workspace` carries the surface tree's members with
    // their authored rationale intact, and no synthesis is needed.
    let ws = Workspace::load(&workspace)?;
    let any_inplace = layer
        .as_ref()
        .is_some_and(|layer| !layer.inplace_members().is_empty());
    let custom_members: Vec<read::CustomMember> = if any_inplace {
        skill_features
            .iter()
            .map(|features| ("skill", features))
            .chain(rule_features.iter().map(|features| ("rule", features)))
            .map(|(kind, features)| read::CustomMember {
                kind: kind.to_string(),
                id: features.id.clone(),
                satisfies: features
                    .satisfies
                    .iter()
                    .cloned()
                    .map(document::Satisfies::new)
                    .collect(),
            })
            .collect()
    } else {
        Vec::new()
    };

    let all_features: Vec<extract::Features> = skill_features
        .iter()
        .chain(rule_features.iter())
        .cloned()
        .collect();

    let assembly_requirements: BTreeMap<String, compose::Requirement> = declarations
        .requirements
        .iter()
        .map(|row| (row.name.clone(), requirement_from_row(row)))
        .collect();
    let assembly_edges = edges_from_declarations(&declarations);

    let (roster, _collisions) = union_published_requirements(&assembly_requirements, &all_features);

    // The world's inbound activation edge into each built-in kind — the same derivation
    // the gate's `reachable` runs, keyed by bare kind name to join `by_kind`.
    let builtin_defs = builtin_kind::definitions()?;
    let mut activations: BTreeMap<&str, kind::Activation> = BTreeMap::new();
    for def in builtin_defs.values() {
        if let Some(activation) = &def.activation {
            activations.insert(def.name.as_str(), activation.clone());
        }
    }

    let repo_files = repo_file_set(Path::new("."));
    let directive_members =
        collect_directive_members(harness_root, &workspace, layer.as_ref(), &declarations)?;
    let directive_edges = graph::classify_directives(&directive_members, &repo_files).edges;

    // Citations — the declared one-way edges naming a leaf; the floor carries no
    // producer yet (`specs/architecture/20-surface.md`, "Genre values"), so the set is empty
    // until an altitude serializes mentions.
    let citations: Vec<read::Citation> = Vec::new();

    Ok(read::explain(
        &ws,
        layer.as_ref(),
        &custom_members,
        &assembly_requirements,
        &roster,
        &by_kind,
        &assembly_edges,
        &activations,
        &repo_files,
        &directive_edges,
        &citations,
        target,
    ))
}

/// Read the `guard`'s posture live off a harness's lock (`specs/architecture/20-surface.md`,
/// "surface authority is a declared posture"): the `authority` fact in
/// `<workspace_dir>/lock.toml`'s assembly declaration rows. An unrepresented harness
/// (no lock, or one predating the fact) reads [`compose::Authority::default`] —
/// `shared` — matching the lock-less "nothing to bind" posture everywhere else in
/// this module.
fn authority_from_lock(workspace_dir: &Path) -> compose::Authority {
    drift::read_declarations(workspace_dir)
        .unwrap_or_default()
        .assembly
        .iter()
        .find(|row| row.fact == "authority")
        .and_then(|row| row.value.as_deref())
        .and_then(|value| match value {
            "surface" => Some(compose::Authority::Surface),
            "shared" => Some(compose::Authority::Shared),
            _ => None,
        })
        .unwrap_or_default()
}

/// Ask `install`'s one question interactively (`specs/architecture/20-surface.md`,
/// "install — the front door"): read a line from stdin, `y`/`yes` (case-insensitive)
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

/// Load the author-declared layer for a `temper_toml` path, folding a gitignored
/// `temper-local.toml` beside it over the committed layer when present
/// (`specs/architecture/40-composition.md`). Discovered in the *same* directory as the
/// committed file, so both `check` (project root) and the session-start gate
/// (harness root) find it beside the file they already read. Absent local ⇒ the
/// committed layer (or bare floor) is returned verbatim.
fn load_layer(temper_toml: &Path) -> miette::Result<Option<compose::AuthorLayer>> {
    let committed = compose::AuthorLayer::load(temper_toml)?;

    let local_path = temper_toml.parent().map_or_else(
        || PathBuf::from(TEMPER_LOCAL_TOML),
        |dir| dir.join(TEMPER_LOCAL_TOML),
    );
    let Some(local) = compose::AuthorLayer::load(&local_path)? else {
        return Ok(committed);
    };

    Ok(Some(match committed {
        Some(base) => base.fold_local(local),
        None => local,
    }))
}

/// The session-start reporter's gate over a harness root (`specs/architecture/20-surface.md`,
/// "CLI surface" — session-start is a reporter of `check`): surface-present ⇒ gate the
/// authored `.temper/` itself; surfaceless ⇒ gate the harness root directly — the
/// discovery walk finds its members straight off disk, against the kind's embedded
/// `governs` (the built-in lock).
///
/// The surface-present branch never re-imports: a fresh import discards recognition (the
/// authored `satisfies` links), so every filled requirement would read unfilled — the
/// false positive on clean input the surface-present clause forbids (law 3).
fn session_start_diagnostics(harness_path: &Path) -> miette::Result<Vec<check::Diagnostic>> {
    let authored = harness_path.join(TEMPER_DIR);
    let temper_toml = harness_path.join(TEMPER_TOML);
    if authored.is_dir() && temper_toml.is_file() {
        gate(&authored, &temper_toml)
    } else {
        gate(harness_path, &temper_toml)
    }
}

/// Produce the merged diagnostic set for a surface `workspace` against the active
/// by-kind contracts — the shared gate behind both `check` and the session-start
/// reporter (`specs/architecture/10-contracts.md`, both greens).
fn gate(workspace: &Path, temper_toml: &Path) -> miette::Result<Vec<check::Diagnostic>> {
    // Absent `temper.toml` ⇒ `None` and the by-kind floor runs verbatim; present ⇒ its
    // per-kind package bindings/clause overrides layer over the floor per kind below
    // (`specs/architecture/40-composition.md`).
    let layer = load_layer(temper_toml)?;
    validate_inplace_kinds(layer.as_ref())?;

    // The assembly's own declared facts — requirements, edges, and the reachability
    // opt-in — ride the lock's declaration rows (`specs/architecture/40-composition.md`,
    // "Decision: one authored assembly, no configuration dialect"): `import` is the one
    // (transitional) producer (`drift::Declarations::write_into`), this is the gate's one
    // read of it, replacing the retired `temper.toml` `[requirement.*]`/
    // `[[kind.*.relationships]]` reads and the `roster.toml`/`bindings.toml` assembly-fact
    // artifacts.
    let declarations = drift::read_declarations(workspace)?;
    let assembly_requirements: BTreeMap<String, compose::Requirement> = declarations
        .requirements
        .iter()
        .map(|row| (row.name.clone(), requirement_from_row(row)))
        .collect();
    let assembly_edges = edges_from_declarations(&declarations);
    let assembly_reachability = reachability_from_declarations(&declarations);

    // The harness root an in-place member's `source` path resolves against — the directory
    // the manifest lives in (the CWD for a two-step `check`, the harness path for the
    // one-shot gate).
    let harness_root = temper_toml
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));

    // The embedded std-lib a by-name `package` binding resolves against
    // (`specs/architecture/10-contracts.md`). Packages **compose**: a satisfier is
    // checked by its kind's bound package *and* any package a requirement names.
    // Held for the guarded roster/graph tier below.
    let builtins = builtin::contracts()?;
    let package_resolver = compose::PackageResolver::new(builtins);

    // The generic two-greens over EVERY embedded built-in kind, keyed by qualified
    // identity (`specs/architecture/20-surface.md`, "Artifact kinds & package binding"): each
    // kind's members — resolved by [`kind_features`] straight off harness disk, shared
    // with `explain` (READ-EDGE-UNIFY) so a read cannot disagree with the gate about
    // which members exist — are dispatched to its floor package (⊕ author layer) and
    // validated, so a discovered CLAUDE.md/AGENTS.md memory member fires its `memory`
    // clauses exactly as a skill/rule does — no longer silently skipped by a hardcoded
    // skill/rule pair. Floors bind by QUALIFIED identity, never the bare name: the two
    // `memory` providers share the bare `memory` by design (86d5b70), so a bare resolve
    // would be ambiguous. SCOPE: only this validation path generalizes — the
    // roster/graph tier below stays skill/rule/custom (no memory member publishes a
    // requirement today; folding more built-ins into the requirement corpus is the
    // separate `(builtin-workspace-qualified-key)` fork), so `skill`/`rule` are
    // captured out of the dispatch below into `skill_features`/`rule_features`.
    let mut diagnostics = Vec::new();
    // Per-kind checked-member counts, keyed by qualified identity — carried out of
    // the dispatch loop for the advisory coverage note below (WEDGE-COVERAGE-NOTE),
    // so "checked N members" is stated rather than left as bare silence.
    let mut member_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut skill_features: Vec<extract::Features> = Vec::new();
    let mut rule_features: Vec<extract::Features> = Vec::new();
    for kind in builtin_kind::definitions()?.values() {
        let qualified = kind.qualified_name();
        let package = builtin::floor_package(&qualified).ok_or_else(|| {
            miette::miette!("built-in kind `{qualified}` ships no floor package binding")
        })?;
        // Two greens (`specs/architecture/10-contracts.md`): admissibility — the contract validated
        // against the definition before it is trusted to judge — then conformance.
        let contract = compose::effective(layer.as_ref(), &kind.name, builtin_floor(package)?)?;

        let features = kind_features(kind, harness_root, workspace, layer.as_ref(), &declarations)?;

        diagnostics.extend(engine::admissibility(&contract));
        diagnostics.extend(engine::validate(&contract, &features));
        member_counts.insert(qualified, features.len());
        match kind.name.as_str() {
            "skill" => skill_features = features,
            "rule" => rule_features = features,
            _ => {}
        }
    }

    // The `paths-match` reachability input and the directive backing-set share one repo
    // file-set (`specs/architecture/45-governance.md`, "The world is a node"): every file under
    // the harness root, over-collected so an extra file can only suppress a finding, never
    // forge one (law 3). Computed once on the FLOOR and read by both the directive classing
    // below and the reachability predicate under the assembly tier.
    // `Path::new("temper.toml").parent()` is `Some("")`, not `None` — the two-step
    // `check` route passes a bare relative `temper.toml`, so an unfiltered `.parent()`
    // yields the empty path and `repo_file_set` walks nothing (WalkDir on "" yields only
    // an Err, skipped ⇒ empty set), forging a `directive-unbacked` on every real import
    // (law 3). Treat an empty parent as the current dir.
    let base_dir = temper_toml
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let repo_files = repo_file_set(base_dir);

    // Directive-target classing on the FLOOR tier (`specs/architecture/15-kinds.md`, "Directives";
    // WEDGE-FACT-FLOOR): an unbacked `@import` is a **pure fact** about the importing member —
    // the silent-context-loss failure class made author-time — so it surfaces with zero
    // config, no `temper.toml` required. Over the built-in kinds' members (empty custom slice;
    // a custom kind's directives ride the assembly tier below, which re-classes the full corpus
    // for reachability), the unbacked findings extend as a **non-gating advisory**: the fact is
    // stated, the run never fails on it alone. The graph-scope escalation — reachability closing
    // over the member-class edges — stays assembly-gated (WEDGE ruling 2026-07-03: an unbacked
    // import is a pure fact, not a graph-scope opinion like reachability).
    diagnostics.extend(
        graph::classify_directives(
            &collect_directive_members(harness_root, workspace, layer.as_ref(), &declarations)?,
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
    // quantified over a requirement's satisfier set (`specs/architecture/10-contracts.md`).
    // Guarded on the layer, so the floor-only path adds nothing here or below.
    if let Some(layer) = layer.as_ref() {
        // Custom kinds retire with the KIND.md file format (`specs/architecture/15-kinds.md`,
        // "Decision: field typing lives in the SDK — there is no kind file format"): a
        // project's own kind is SDK-authored, and no SDK path exists in the engine yet,
        // so every `[kind.<name>]` registration that isn't a built-in layer contributes
        // no members and no edges. The corpus/edge plumbing below stays generic — ready
        // for that future SDK path, not hardwired to the empty case.
        let custom_kinds: Vec<CustomKindEntry> = Vec::new();
        let edges = assembly_edges.clone();

        // The by-kind corpus every set-scope and graph predicate ranges over,
        // assembled through the same helper the read arm uses.
        let by_kind = assemble_by_kind(&skill_features, &rule_features, &custom_kinds);

        // Every opt-in-capable member's features (built-in kinds *and* each custom
        // kind's members) — the stream coverage ranges over *and* the one the gate
        // gathers member-published requirements from below.
        let all_features: Vec<extract::Features> = skill_features
            .iter()
            .chain(rule_features.iter())
            .chain(custom_kinds.iter().flat_map(|(_, _, features)| features))
            .cloned()
            .collect();

        // The one requirement namespace (`specs/architecture/10-contracts.md`, "Decision: a
        // requirement's publisher is any authored surface document"): the assembly's
        // `[requirement.*]` unioned with every member's published `[requirement.*]`.
        // A name published by two surfaces is one obligation, not a shadow — the
        // collision is an admissibility finding and the first publisher keeps the slot,
        // so the roster/coverage passes below judge one coherent namespace.
        let (requirements, collisions) =
            union_published_requirements(&assembly_requirements, &all_features);
        diagnostics.extend(collisions);

        // Admissibility before conformance here too: each requirement's own
        // definition is validated before the roster is trusted to judge the harness
        // (`specs/architecture/10-contracts.md`).
        diagnostics.extend(roster::admissibility(
            &requirements,
            &by_kind,
            &package_resolver,
            base_dir,
        ));

        // The set-scope predicates: each requirement's `count` / `unique` /
        // `membership` gate over its satisfier set (`specs/architecture/45-governance.md`).
        diagnostics.extend(roster::check(&requirements, &by_kind, &package_resolver));

        // The `conforms-to` half: each requirement's satisfiers validated against
        // its bound `package`'s contract, retagged under `requirement.conforms-to`.
        // A non-resolving package is admissibility's finding above, skipped here
        // rather than double-reported.
        diagnostics.extend(roster::conformance(
            &requirements,
            &by_kind,
            &package_resolver,
        ));

        // The graph scope: build the reference graph over the declared edges (a
        // reference is a kind capability, `specs/architecture/15-kinds.md`) and check route
        // resolution — a declared reference must resolve to a real artifact of the
        // target kind (`specs/architecture/45-governance.md`). Admissibility before conformance:
        // an edge naming no reference field or targeting an unmodeled kind is
        // reported once and skipped by the route check.
        diagnostics.extend(graph::admissibility(&edges, &by_kind));
        diagnostics.extend(graph::check(&edges, &by_kind));

        // `acyclic` (`specs/architecture/45-governance.md`): the resolved graph must contain no
        // cycle — a circular import loads nothing, so every finding is a true
        // positive. Always-on over the whole edge set, like route resolution above.
        diagnostics.extend(graph::acyclic(&edges, &by_kind));

        // `degree` (`specs/architecture/45-governance.md`): a requirement declares an in/out
        // edge-count bound every satisfier's degree must fall inside, so it takes
        // the requirements *and* the edges, reusing the arc resolution
        // `acyclic`/`check` assemble. Opt-in per requirement.
        diagnostics.extend(graph::degree(&assembly_requirements, &edges, &by_kind));

        // Directive-target classing over the **full** corpus (`specs/architecture/15-kinds.md`,
        // "Directives"): built-in *and* custom members, so a custom kind's `@import` at a
        // built-in member resolves to a member→member edge the reachability closure below
        // consumes (`repo_files` and the file-set come from the floor above). Only the
        // member-class **edges** are read here — the unbacked findings already surfaced on the
        // floor as a non-gating advisory (WEDGE-FACT-FLOOR), so they are not re-extended: an
        // assembly's power over a directive is the graph-scope reachability escalation, not the
        // unbacked fact, which is the same fact with or without an assembly.
        let directive_members =
            collect_directive_members(harness_root, workspace, Some(layer), &declarations)?;
        let directive_classing = graph::classify_directives(&directive_members, &repo_files);

        // `reachable` (`specs/architecture/45-governance.md`, "The world is a node"): a member whose
        // kind declares an activation is dead when the world→member edge is provably so
        // (a blank description-trigger, a zero-match paths glob). Assembly-scope and
        // opt-in like `degree` — it runs only when the assembly declares `[reachability]`,
        // at its declared severity (resolved `reachability-gate-mechanism` option b), so
        // a deliberate work-in-progress dead edge stays the author's call.
        if let Some(reachability) = assembly_reachability {
            // The world's inbound edge into each in-scope kind is that kind's declared
            // activation: the built-in kinds' via `builtin_kind::definitions()`, each
            // custom kind's via its own `CustomKind.activation`. A kind declaring none
            // contributes no edge and no member is subject.
            let builtin_defs = builtin_kind::definitions()?;
            let mut activations: BTreeMap<&str, kind::Activation> = BTreeMap::new();
            for def in builtin_defs.values() {
                if let Some(activation) = &def.activation {
                    // Key by the kind's bare `name`, not `definitions()`'s qualified map
                    // key — `by_kind` (the members this edge reaches) is bare-keyed, so
                    // the activation edge must join it under the same bare name.
                    activations.insert(def.name.as_str(), activation.clone());
                }
            }
            for (name, custom, _features) in &custom_kinds {
                if let Some(activation) = &custom.activation {
                    activations.insert(name, activation.clone());
                }
            }

            diagnostics.extend(graph::reachable(
                &activations,
                &by_kind,
                &repo_files,
                &directive_classing.edges,
                engine::severity_of(reachability.severity),
            ));
        }

        // The requirement-coverage tier (`specs/architecture/10-contracts.md`): every `required`
        // requirement must have a resolving home (≥1 artifact opting in via
        // `satisfies`) and every authored `satisfies` must resolve to a declared
        // requirement. Kind-blind: it ranges over every opt-in-capable artifact —
        // built-in kinds *and* each custom kind's members — so temper's own `spec`
        // corpus can opt in exactly as a skill does (`specs/architecture/15-kinds.md`). The
        // requirement set is the *unioned* namespace, so a member-published obligation
        // is gated here exactly as an assembly-published one.
        diagnostics.extend(coverage::check(&requirements, &all_features));

        // The custom-kind conformance tier: each registered custom kind runs the
        // same two greens the built-in kinds do (`specs/architecture/15-kinds.md`), but through
        // its own authored extractor (features computed above) and its bound package
        // rather than inline clauses.
        for (name, _custom, features) in &custom_kinds {
            // Resolves by name through the same order every binding uses, defaulting
            // to the kind's own name when the registration binds none.
            let package_name = layer.kind_package(name).unwrap_or(*name);
            match package_resolver.resolve(package_name) {
                Some(contract) => {
                    diagnostics.extend(engine::admissibility(&contract));
                    diagnostics.extend(engine::validate(&contract, features));
                }
                None => diagnostics.push(check::Diagnostic::error(
                    format!("{name}.package"),
                    *name,
                    format!(
                        "custom kind `{name}` binds unknown package `{package_name}` (not a built-in package)"
                    ),
                )),
            }
        }
    }

    // The install self-verify (`specs/architecture/50-distribution.md`): temper checking its
    // *own* gate is wired. Advisory (warn) only — a not-yet-installed gate nudges
    // without failing the run, and the session-start reporter ignores warn
    // severity. Read relative to the `temper.toml` parent (the CWD for `check`, the
    // harness path for session-start).
    let root = temper_toml
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    diagnostics.extend(install::gate_installed(root));

    // The wedge's advisory coverage note (`specs/architecture/50-distribution.md`,
    // "Fail-loud delivery — the invariant"): state which built-in kinds checked how
    // many members, and name the known Claude Code surfaces present on disk that no
    // kind governs, so the gate's silence about an unmodeled surface never reads as
    // "checked". Warn-only over the embedded built-in kind set — it leaves the run's
    // exit code and the session-start verdict unchanged.
    diagnostics.extend(coverage_note::check(
        root,
        &builtin_kind::definitions()?,
        &member_counts,
    ));

    // The fail-loud coherence tripwire (`specs/architecture/50-distribution.md`, "Fail-loud
    // delivery — the invariant"): the committed assembly declares members/requirements but
    // the gate resolved none of them and the lock carries no declaration rows either — the
    // harness-root `temper check .` case the wave-end confirmation caught (checked 0
    // members, exit 0). `declared` never looks past the committed layer's own tables, so a
    // correctly-rooted check (≥1 resolved member) and a genuinely empty harness (no
    // `temper.toml` declaring anything) both stay silent.
    let declared = layer.as_ref().is_some_and(|layer| {
        !layer.inplace_members().is_empty() || !layer.requirements().is_empty()
    });
    let resolved_members: usize = member_counts.values().sum();
    let declarations_empty = declarations.kinds.is_empty()
        && declarations.clauses.is_empty()
        && declarations.requirements.is_empty()
        && declarations.assembly.is_empty()
        && declarations.satisfies.is_empty();
    diagnostics.extend(check::empty_assembly_incoherence(
        root,
        declared,
        resolved_members,
        declarations_empty,
    ));

    // The freshness fact (`specs/architecture/20-surface.md`, "Drift — two freshness facts"):
    // a committed projection whose bytes no longer match the lock's emit fingerprint is
    // `config.stale`. Read off the surface `workspace`'s lock (where the members were
    // imported and the fingerprints recorded), advisory so a hand-edited or un-re-emitted
    // projection is surfaced without failing the run — the `shared`-authority nudge.
    diagnostics.extend(drift::config_stale(workspace));

    Ok(diagnostics)
}

/// Every in-place member's declared `kind` resolves to a built-in, checked upfront —
/// in-place carriage is built-in-kind only (a custom kind's units are authored
/// `.temper/` artifacts), so a member naming an unresolved kind is a hard, loud error,
/// never a silent drop from every kind's per-kind dispatch below (a member whose
/// `kind` matches none of `resolve_kind_units`'s per-kind filters would otherwise fall
/// straight through to that kind's own governs-driven harness walk, unreported —
/// `.claude/rules/collaboration.md`, "a silent skip reads as done").
///
/// # Errors
///
/// Returns an error naming the first in-place member whose `kind` resolves to no
/// built-in.
fn validate_inplace_kinds(layer: Option<&compose::AuthorLayer>) -> miette::Result<()> {
    let Some(layer) = layer else {
        return Ok(());
    };
    let builtins = builtin_kind::definitions()?;
    for member in layer.inplace_members() {
        if !builtins.values().any(|kind| kind.name == member.kind) {
            return Err(miette::miette!(
                "in-place member `{}` names non-built-in kind `{}` (in-place carriage is built-in-kind only)",
                member.name,
                member.kind
            ));
        }
    }
    Ok(())
}

/// This kind's effective `governs` locus (`specs/architecture/20-surface.md`, "The lock
/// and drift — one vocabulary"): the committed lock's own kind-fact row when the lock
/// declares one for it — matched by bare name **and** provider, so the two `memory`
/// providers sharing the bare name never cross-resolve each other's locus
/// (`specs/architecture/15-kinds.md`, "Decision: kind identity carries a provider
/// axis") — or the kind's own embedded `governs` when it doesn't: the **built-in
/// lock**, the same declaration shape the engine carries compiled-in for an unadopted
/// harness.
fn effective_governs(kind: &CustomKind, declarations: &drift::Declarations) -> kind::Governs {
    let provider = kind
        .qualified
        .as_deref()
        .and_then(|qualified| qualified.rsplit_once('.'))
        .map(|(provider, _)| provider);
    declarations
        .kinds
        .iter()
        .find(|row| row.name == kind.name && row.provider.as_deref() == provider)
        .map(|row| kind::Governs {
            root: row.governs_root.clone(),
            glob: row.governs_glob.clone(),
        })
        .unwrap_or_else(|| kind.governs.clone())
}

/// The already-projected surface document's own authored `satisfies`/published
/// requirements, when one exists at this member's carriage locus
/// (`<workspace>/<kind's surface subdir>/<id>/<member document>`) — the one home a
/// document/module-carried member's fill edges are ever authored at, never mined from
/// the raw harness file (`specs/architecture/20-surface.md`, "The member document").
/// `None` when the member has never been projected (arrives unrecognized, "install —
/// the front door").
///
/// # Errors
///
/// Returns an error if the surface document exists but is malformed.
fn surface_overlay(
    workspace: &Path,
    kind: &CustomKind,
    id: &str,
) -> miette::Result<Option<frontmatter::Member>> {
    let dir = workspace.join(kind.surface_subdir()).join(id);
    let doc = kind.member_document();
    if !dir.join(&doc).is_file() {
        return Ok(None);
    }
    Ok(Some(frontmatter::Member::from_surface(&dir, &doc)?))
}

/// A kind's members, resolved live off disk — the one corpus both `gate` and `explain`
/// range over (READ-EDGE-UNIFY, `specs/architecture/20-surface.md`, "The lock and
/// drift"). In-place carriage (a `source`-bearing `temper.toml` `[[member]]` table)
/// reads its own declared members from their named source, grafting `satisfies`/
/// published requirements from the assembly's declaration rather than mining them from
/// the file — the harness format is not temper's to annotate. Otherwise every member
/// is discovered by walking this kind's [`effective_governs`] locus, read straight off
/// harness disk so the corpus can never drift from a stale copy; its own `satisfies`/
/// published requirements — authored only on its projected surface document — are
/// grafted from [`surface_overlay`] when one exists.
///
/// # Errors
///
/// Returns an error if a source or surface file is unreadable or malformed, or a
/// governed directory cannot be enumerated.
fn resolve_kind_units(
    kind: &CustomKind,
    harness_root: &Path,
    workspace: &Path,
    layer: Option<&compose::AuthorLayer>,
    declarations: &drift::Declarations,
) -> miette::Result<Vec<Unit>> {
    let inplace: Vec<&compose::InPlaceMember> = layer
        .map(|layer| {
            layer
                .inplace_members()
                .iter()
                .filter(|member| member.kind == kind.name)
                .collect()
        })
        .unwrap_or_default();

    let mut units = if inplace.is_empty() {
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
                published_requirements: Vec::new(),
            };
            if let Some(surface) = surface_overlay(workspace, kind, &unit.id)? {
                unit.satisfies = surface
                    .satisfies
                    .iter()
                    .map(|clause| clause.requirement.clone())
                    .collect();
                unit.satisfies_clauses = surface.satisfies;
                unit.published_requirements = surface.published_requirements;
            }
            units.push(unit);
        }
        units
    } else {
        inplace
            .into_iter()
            .map(|member| inplace_unit(harness_root, member))
            .collect::<miette::Result<Vec<_>>>()?
    };

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
    workspace: &Path,
    layer: Option<&compose::AuthorLayer>,
    declarations: &drift::Declarations,
) -> miette::Result<Vec<extract::Features>> {
    Ok(
        resolve_kind_units(kind, harness_root, workspace, layer, declarations)?
            .iter()
            .map(|unit| builtin_kind::features(kind, unit))
            .collect(),
    )
}

/// Live-extract an in-place member's raw [`Unit`] from its committed landscape file
/// (`specs/architecture/20-surface.md`, "In-place"): read the raw harness file at
/// `<harness_root>/<source>`, parse its frontmatter + body through the generic adapter
/// — the file is its own committed source, re-read every check, so it cannot drift.
/// The join edges are the manifest's declaration (the harness file carries no temper
/// annotation), so the member's `satisfies`/published requirements are grafted from
/// the assembly rather than mined from the file.
///
/// In-place carriage is built-in-kind only (a custom kind's units are authored
/// `.temper/` artifacts), so a member naming a non-built-in kind is a hard error,
/// never a silent skip.
///
/// # Errors
///
/// Returns an error if the kind is not a built-in, or the landscape file is unreadable
/// or malformed.
fn inplace_unit(harness_root: &Path, member: &compose::InPlaceMember) -> miette::Result<Unit> {
    let path = harness_root.join(&member.source);
    // Route to the built-in kind by bare name, then by which owns the source glob — so the
    // two `memory` providers (`CLAUDE.md` vs `AGENTS.md`) sharing the bare `memory` resolve to
    // the right one rather than colliding on an ambiguous bare lookup (`specs/architecture/15-kinds.md`).
    let builtins = builtin_kind::definitions()?;
    let kind = builtins
        .values()
        .filter(|kind| kind.name == member.kind)
        .find(|kind| kind.owns_source(&path))
        .or_else(|| builtins.values().find(|kind| kind.name == member.kind))
        .ok_or_else(|| {
            miette::miette!(
                "in-place member `{}` names non-built-in kind `{}` (in-place carriage is built-in-kind only)",
                member.name,
                member.kind
            )
        })?;
    let source = frontmatter::Member::from_source(kind, &path)?;
    // The id is the manifest's recorded name (the surface identity), not re-derived; the
    // extractor reads frontmatter/body/placement off the raw harness file.
    Ok(Unit {
        id: member.name.clone(),
        frontmatter: source.fields.iter().cloned().collect(),
        body: source.body.clone(),
        source_path: source.provenance.source_path.clone(),
        // The join edges are the assembly's declaration, not a fact mined from the file.
        satisfies: member.satisfies.clone(),
        satisfies_clauses: Vec::new(),
        published_requirements: member.published.clone(),
    })
}

/// Every file under `root`, as repo-relative slash-separated paths — the
/// `paths-match` reachability input (`specs/architecture/45-governance.md`, "The world is a node").
/// A superset is sound (a glob matching an extra file only suppresses a finding); a
/// *missing* file is not (it could forge a dead-edge false positive, law 3), so nothing
/// is excluded and an unreadable entry is skipped rather than aborting the gate. Paths
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

/// A registered custom kind as the corpus construction carries it: its name (borrowed
/// from the assembly layer), its loaded [`CustomKind`] definition, and its computed
/// member [`Features`](extract::Features). Named so the shared corpus helpers keep a
/// legible signature (`clippy::type_complexity`).
type CustomKindEntry<'a> = (&'a str, CustomKind, Vec<extract::Features>);

/// Assemble the by-kind [`Features`](extract::Features) corpus every set-scope and
/// graph predicate ranges over: the built-in kinds plus each custom kind's features
/// (empty today — custom kinds retire with the `KIND.md` file format), keyed by kind
/// name. Borrows every slice, so the caller holds the owned feature vecs for the
/// map's lifetime.
fn assemble_by_kind<'a>(
    skill_features: &'a [extract::Features],
    rule_features: &'a [extract::Features],
    custom_kinds: &'a [CustomKindEntry<'a>],
) -> BTreeMap<&'a str, &'a [extract::Features]> {
    let mut by_kind: BTreeMap<&str, &[extract::Features]> =
        BTreeMap::from([("skill", skill_features), ("rule", rule_features)]);
    for (name, _custom, features) in custom_kinds {
        by_kind.insert(*name, features.as_slice());
    }
    by_kind
}

/// Pair each member with the provenance `source_path` the directive classing joins on
/// (`specs/architecture/15-kinds.md`, "Directives"): the decidable [`Features`](extract::Features)
/// view drops the full path, so it is read off the units the features were extracted
/// from. Every member is carried — a directive may point at a member that imports
/// nothing — with its `directives` occurrences (empty for a kind composing no
/// `directives` primitive).
///
/// Ranges over **every** embedded built-in kind's members via
/// [`builtin_kind::definitions`] — not a hardcoded skill/rule pair — so a discovered
/// `CLAUDE.md` memory member's `at-import` targets reach [`graph::classify_directives`]
/// and an unbacked `@path` draws its finding (DIRECTIVE-MEMBERS-ALL-KINDS, the same
/// generalization CHECK-MEMBERS-ALL-KINDS made for clause dispatch). Each kind's
/// members are resolved through [`resolve_kind_units`] — the same live, governs-driven
/// read the gate's own dispatch uses — and keyed by the bare `kind.name`, the keying
/// `by_kind`/`classify_directives` join on. Custom kinds retire with the KIND.md file
/// format (`specs/architecture/15-kinds.md`) and contribute no members.
///
/// # Errors
///
/// As [`resolve_kind_units`].
fn collect_directive_members(
    harness_root: &Path,
    workspace: &Path,
    layer: Option<&compose::AuthorLayer>,
    declarations: &drift::Declarations,
) -> miette::Result<Vec<graph::DirectiveMember>> {
    let mut members = Vec::new();
    for kind in builtin_kind::definitions()?.values() {
        for unit in resolve_kind_units(kind, harness_root, workspace, layer, declarations)? {
            let feature = builtin_kind::features(kind, &unit);
            members.push(graph::DirectiveMember {
                kind: kind.name.clone(),
                id: feature.id.clone(),
                source_path: unit.source_path.clone(),
                directives: feature.directives.clone(),
            });
        }
    }
    Ok(members)
}

/// Union the assembly's published `[requirement.*]` roster with every member's
/// published `[requirement.*]` into the single requirement namespace the gate judges
/// (`specs/architecture/10-contracts.md`, "Decision: a requirement's publisher is any authored
/// surface document"). `satisfies` fills whichever surface published the demand, so
/// one namespace is the whole point.
///
/// A name published by two surfaces — assembly ⊕ member, or two members — is **one
/// obligation, never a shadow**: the collision is reported as an admissibility finding
/// (returned separately, so the caller folds it into the gate) and the **first
/// publisher keeps the slot**. The assembly roster seeds the map, then members are
/// walked in the corpus's stable order, so the winner and the finding set are
/// deterministic across runs. Every member-published requirement carries only the
/// four facets a member header publishes (`means`/`kind`/`package`/`required`); the
/// richer set-scope facets stay assembly-only and default here.
fn union_published_requirements(
    assembly: &BTreeMap<String, compose::Requirement>,
    members: &[extract::Features],
) -> (
    BTreeMap<String, compose::Requirement>,
    Vec<check::Diagnostic>,
) {
    let mut requirements = assembly.clone();
    let mut diagnostics = Vec::new();
    for features in members {
        for published in &features.published_requirements {
            if requirements.contains_key(&published.name) {
                diagnostics.push(check::Diagnostic::error(
                    REQUIREMENT_COLLISION_RULE,
                    &published.name,
                    format!(
                        "requirement `{}` is published by more than one surface (member `{}` re-declares a name already published); a requirement lives in one namespace and is never shadowed — rename or drop one publisher",
                        published.name, features.id
                    ),
                ));
            } else {
                requirements.insert(published.name.clone(), to_requirement(published));
            }
        }
    }
    (requirements, diagnostics)
}

/// Lift a member-published [`document::PublishedRequirement`] into the shared
/// [`compose::Requirement`] the roster and coverage passes range over — the four
/// published facets carried across, the assembly-only set-scope facets defaulted.
/// The demand is the same concept whichever surface authored it, so it joins one type.
fn to_requirement(published: &document::PublishedRequirement) -> compose::Requirement {
    compose::Requirement {
        name: published.name.clone(),
        means: published.means.clone(),
        kind: published.kind.clone(),
        package: published.package.clone(),
        required: published.required,
        count: None,
        unique: Vec::new(),
        membership: None,
        degree: None,
        verified_by: None,
    }
}

/// Lift the lock's [`drift::RequirementRow`] — the whole requirement shape `import`
/// wrote (`specs/architecture/40-composition.md`) — into the [`compose::Requirement`]
/// the roster/coverage/graph tiers already take, the mirror of `import`'s own
/// `requirement_row`. The row carries no `means` (`import` never emits it — `specs/intent/00-intent.md`
/// law 3, "`temper` never interprets `means`" — no gate reads it), so it defaults here too.
fn requirement_from_row(row: &drift::RequirementRow) -> compose::Requirement {
    compose::Requirement {
        name: row.name.clone(),
        means: None,
        kind: row.kind.clone(),
        package: row.package.clone(),
        required: row.required,
        count: row.count.map(|count| compose::CountBound {
            min: count.min,
            max: count.max,
        }),
        unique: row.unique.clone(),
        membership: row
            .membership
            .as_ref()
            .map(|membership| compose::Membership {
                field: membership.field.clone(),
                source: membership.source.clone(),
                source_kind: membership.source_kind.clone(),
                source_feature: membership.source_feature.clone(),
                source_package: membership.source_package.clone(),
            }),
        degree: row.degree.as_ref().map(|degree| compose::DegreeBound {
            incoming: degree.incoming.map(|bound| compose::EdgeBound {
                min: bound.min,
                max: bound.max,
            }),
            outgoing: degree.outgoing.map(|bound| compose::EdgeBound {
                min: bound.min,
                max: bound.max,
            }),
        }),
        verified_by: row.verified_by.clone(),
    }
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

/// The assembly's `[reachability]` opt-in off the lock's `assembly` fact family —
/// the `fact = "reachability"` row's severity, when present and its `value` names a
/// recognized severity label. Absent or malformed ⇒ `None` (no opt-in), matching
/// `read_declarations`'s "absent evidence forges no finding" tolerance.
fn reachability_from_declarations(
    declarations: &drift::Declarations,
) -> Option<compose::Reachability> {
    declarations
        .assembly
        .iter()
        .find(|fact| fact.fact == "reachability")
        .and_then(|fact| fact.value.as_deref())
        .and_then(|label| match label {
            "required" => Some(contract::Severity::Required),
            "advisory" => Some(contract::Severity::Advisory),
            _ => None,
        })
        .map(|severity| compose::Reachability { severity })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    /// The directive-backing set reads **raw disk**, never ignore-filtered: whether an
    /// `@import` target is backed is a fact about the filesystem the harness loads
    /// regardless of `.gitignore`, and law 3 fixes the safe direction — an extra backing
    /// file only *suppresses* a finding, while pruning one could *forge* an unbacked
    /// finding on a target that exists (`specs/architecture/20-surface.md`, "the backing
    /// set reads raw disk"). This is the counterpart to discovery, which *does* prune —
    /// two sets, two rules, never merged.
    #[test]
    fn repo_file_set_stays_raw_disk_including_gitignored_targets() {
        let root = std::env::temp_dir().join(format!("temper-backing-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
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

        let _ = fs::remove_dir_all(&root);
    }
}
