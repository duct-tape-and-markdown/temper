//! `temper` CLI entry point.
//!
//! A thin `clap` dispatch over the [`temper`] library: parse args, run the
//! generic contract engine, map the result to an exit code. Every subcommand
//! mirrors the CLI surface of `specs/architecture/20-surface.md`; all logic lives in the
//! library so `tests/` can drive it.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use miette::IntoDiagnostic;
use temper::assembly_artifacts;
use temper::builtin;
use temper::builtin_kind;
use temper::bundle;
use temper::check::{self, Severity, Workspace};
use temper::compose;
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
use temper::kind::{self, CustomKind, KindError, Unit};
use temper::read;
use temper::reporter;
use temper::roster;
use temper::schema;

/// The surface workspace default for `--into` / the `check` argument
/// (`specs/architecture/20-surface.md`): a `.temper` directory under the cwd.
const DEFAULT_WORKSPACE: &str = "./.temper";

/// The authored surface directory beside a harness's `temper.toml`, holding its
/// custom-kind definitions (`kinds/<name>/KIND.md`) and packages
/// (`packages/<name>/PACKAGE.md`) — `specs/architecture/40-composition.md`. On the one-shot
/// gate paths (session-start, `check --harness`) it is the authored root handed
/// to [`gate`], distinct from the throwaway scratch surface the members land in.
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
    /// The on-ramp (`specs/architecture/20-surface.md`): scan an existing harness into a
    /// config skeleton over its members **in place** — a manifest naming each landscape
    /// file, zero file moves, no copy tree. `--lift <member>` migrates one member into a
    /// richer carriage (in-place → document → module).
    Init {
        /// The harness to scan: a project root (its `.claude/skills/`, `.claude/rules/`).
        /// Defaults to the current directory.
        #[arg(default_value = ".")]
        harness_path: PathBuf,
        /// Migrate one member into a richer carriage (in-place → document → module) instead
        /// of scanning: lift the named in-place member into document carriage
        /// (`specs/architecture/20-surface.md`).
        #[arg(long, value_name = "MEMBER")]
        lift: Option<String>,
    },
    /// Project a harness into the document-carriage surface workspace (+ provenance
    /// lock) — the retained copy-tree projection `emit`/`diff` and the document-carried
    /// gate ride (`specs/architecture/15-kinds.md`, the generic frontmatter adapter). The
    /// on-ramp is `init` (members in place, no copy tree); this is the deliberate
    /// document-carriage materialization a member/harness climbs into.
    Import {
        /// The harness to scan: a project root (its `.claude/skills/`, `.claude/rules/`),
        /// or a bare skill dir.
        harness_path: PathBuf,
        /// Where to write the surface workspace (defaults to `./.temper`).
        #[arg(long, default_value = DEFAULT_WORKSPACE)]
        into: PathBuf,
    },
    /// Lint the config surface against the active contract.
    Check {
        /// The surface workspace to lint (defaults to `./.temper`).
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
        /// whichever is chosen.
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
    /// Report on-disk drift of a harness against the surface's import baseline.
    Diff {
        /// The harness to re-scan and compare against the import baseline.
        harness_path: PathBuf,
        /// The surface workspace holding the baseline (defaults to `./.temper`).
        #[arg(long, default_value = DEFAULT_WORKSPACE)]
        into: PathBuf,
    },
    /// Compile the authoring face: re-emit each projection **whole** from the
    /// surface, byte-deterministically and double-emit verified
    /// (`specs/architecture/20-surface.md`, law 5). Each artifact is regenerated full-file —
    /// byte-stable and idempotent — and written back to the source path `import`
    /// recorded; a direct edit to the projection is drift routed to the authored
    /// source, never something emit merges around.
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
    /// The advisory session-start gate (`specs/architecture/50-distribution.md`): check a
    /// harness in one shot and emit the `claude-session-start` reporter payload on
    /// stdout for a Claude Code `SessionStart` hook. Takes a *harness* path, not a
    /// surface workspace, and always exits zero — a failing contract routes
    /// through the human via the reporter's notify-and-approve verdict.
    SessionStart {
        /// The harness to check: the same tree `import` scans (a project root with
        /// `.claude/skills/*` + `.claude/rules/*`, or a bare skill dir, plus any
        /// `temper.toml` kinds).
        harness_path: PathBuf,
    },
    /// Project temper's own gate wiring into the harness (`specs/architecture/50-distribution.md`):
    /// the `SessionStart` hook into `.claude/settings.json`, the CI job into
    /// `.github/`, and the schema modeline into each artifact's frontmatter — all
    /// under the three-state drift engine, so re-running is idempotent and re-places
    /// anything a human deleted. `check` then verifies its own gate stays installed.
    Install {
        /// The project root to wire the gate into (defaults to the current
        /// directory, beside the `.claude/` and `.github/` the placements land in).
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Compute and report every placement without writing a single byte.
        #[arg(long)]
        dry_run: bool,
    },
    /// Compose the imported surface into a publishable Claude Code plugin +
    /// `marketplace.json` (`specs/architecture/50-distribution.md`): the operate-the-gate skill,
    /// the `SessionStart` hook, and the shipped built-in packages embedded.
    /// Deterministic and byte-faithful where it carries prose, so re-running
    /// reproduces an identical tree.
    Bundle {
        /// The imported surface workspace to compose from (defaults to `./.temper`).
        #[arg(default_value = DEFAULT_WORKSPACE)]
        path: PathBuf,
        /// Where to write the plugin tree (defaults to `./plugin`).
        #[arg(long, default_value = "./plugin")]
        out: PathBuf,
    },
    /// Read (`specs/architecture/20-surface.md`): narrate everything that holds a member in
    /// place — the requirements it `satisfies` (each with its rationale), the
    /// package its kind binds, and its declared edges in and out. The forward walk
    /// of the requirement↔`satisfies` edge; never gates, always exits zero.
    Why {
        /// The member (a skill or rule name) to walk the edge forward from.
        member: String,
    },
    /// Read (`specs/architecture/20-surface.md`): narrate the requirement roster — each
    /// requirement with its satisfier set and coverage state; with a `<name>`,
    /// that one's satisfiers and the blast radius a removal would strand. The
    /// reverse walk of the requirement↔`satisfies` edge; never gates, exits zero.
    Requirements {
        /// A single requirement to walk in reverse; omitted ⇒ the whole roster.
        name: Option<String>,
    },
    /// Read (`specs/architecture/20-surface.md`): narrate a member's **blast radius** — what
    /// strands if it is removed or renamed: the requirements it alone fills (left
    /// unfilled), the `satisfies` onto demands it alone publishes (left dangling), the
    /// `@import` edges pointing at it (left unbacked), and the members reachable only
    /// through it (gone dead). Accepts a **leaf address** (`member/genre/key/field-path`)
    /// too, reporting the leaf's citations separately from fallout. The deterministic tier-1
    /// traversal over the graph `check` carries; never gates, exits zero.
    Impact {
        /// A member name (a skill, rule, or custom-kind member) or a leaf address
        /// (`member/genre/key/field-path`) to trace at member or leaf grain.
        member: String,
    },
    /// Read (`specs/architecture/20-surface.md`): emit a member's or leaf's **declared
    /// neighborhood** — its genre slot, its siblings, the members that cite it, and the
    /// requirements its member satisfies — the pre-edit context bundle for the primary
    /// author. Accepts a member name or a leaf address (`member/genre/key/field-path`);
    /// every leaf-grain answer discloses its mixed-posture coverage. Reads the manifest
    /// alone (offline, tier-1); never gates, exits zero.
    Context {
        /// A member name or a leaf address (`member/genre/key/field-path`) to emit the
        /// neighborhood of.
        address: String,
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
}

fn main() -> miette::Result<ExitCode> {
    match Cli::parse().command {
        Command::Init { harness_path, lift } => {
            // The on-ramp writes the manifest over members IN PLACE — no `.temper/` copy
            // tree (`specs/architecture/20-surface.md`, "Decision: `init` is the on-ramp"). `--lift`
            // migrates one member into a richer carriage instead of re-scanning.
            match lift {
                Some(member) => import::lift(&harness_path, &member)?,
                None => import::init(&harness_path)?,
            }
            Ok(ExitCode::SUCCESS)
        }
        Command::Import { harness_path, into } => {
            import::run(&harness_path, &into)?;
            // The document-carriage projection serializes its manifest beside the
            // workspace (`specs/architecture/20-surface.md`, "Topology"); the one-shot gate paths
            // (`check --harness`, session-start) import into a scratch surface and skip it,
            // so a lint never mutates the harness.
            import::emit_manifest(&harness_path, &into)?;
            Ok(ExitCode::SUCCESS)
        }
        Command::Check {
            workspace,
            harness,
            deny_advisories,
            reporter,
        } => {
            // Two ways into the same gate (`specs/architecture/20-surface.md`). `--harness` is
            // the one-shot wedge: import into a throwaway scratch surface, gate
            // against the harness's own `temper.toml` as session-start does, tear
            // the scratch down. Without it, the two-step path gates an
            // already-imported surface. Same diagnostic shape ⇒ shared render below.
            let diagnostics = match harness {
                Some(harness) => {
                    let scratch = scratch_surface()?;
                    import::run(&harness, &scratch)?;
                    // Members land in the scratch, but the authored kinds/packages
                    // live beside the harness's `temper.toml` — resolve them from
                    // the harness's own `.temper/`, not the scratch.
                    let diagnostics = gate(
                        &scratch,
                        &harness.join(TEMPER_DIR),
                        &harness.join(TEMPER_TOML),
                    )?;
                    // A leftover scratch dir must never fail the run.
                    let _ = fs::remove_dir_all(&scratch);
                    diagnostics
                }
                None => {
                    let workspace = workspace.unwrap_or_else(|| PathBuf::from(DEFAULT_WORKSPACE));
                    // Two-step: the surface *is* the authored `.temper/`, so both
                    // members and authored kinds/packages resolve from it.
                    gate(&workspace, &workspace, Path::new(TEMPER_TOML))?
                }
            };

            match reporter {
                Reporter::Terminal => print!("{}", check::render(&diagnostics)),
                Reporter::Github => print!("{}", reporter::github(&diagnostics)),
                Reporter::Sarif => println!("{}", reporter::sarif(&diagnostics)),
            }

            // `--deny-advisories` promotes `advisory` (warn) violations to blocking
            // on top of the always-blocking `required` ones.
            let advisory_blocks = deny_advisories
                && diagnostics
                    .iter()
                    .any(|diagnostic| diagnostic.severity == Severity::Warn);
            Ok(if check::any_error(&diagnostics) || advisory_blocks {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            })
        }
        Command::Schema { kind } => {
            // The keystroke placement of the gate (`specs/architecture/50-distribution.md`):
            // emit the *active* contract per kind — the same floor ⊕ `temper.toml`
            // layer `check` gates against — as an editor JSON Schema.
            let layer = load_layer(Path::new(TEMPER_TOML))?;

            // A bound project package resolves from the default surface's
            // `.temper/packages/`: schema takes no workspace, so it reads the
            // default rather than a caller-supplied one.
            let packages_dir = Path::new(DEFAULT_WORKSPACE).join("packages");

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
                    let contract = compose::effective(layer.as_ref(), &name, floor, &packages_dir)?;
                    schema::emit(&contract)
                }
                None => {
                    let mut map = serde_json::Map::new();
                    for (name, floor) in floors {
                        let contract =
                            compose::effective(layer.as_ref(), &name, floor, &packages_dir)?;
                        map.insert(name, schema::emit(&contract));
                    }
                    serde_json::Value::Object(map)
                }
            };

            println!("{}", serde_json::to_string_pretty(&json).into_diagnostic()?);
            Ok(ExitCode::SUCCESS)
        }
        Command::Diff { harness_path, into } => {
            // Read-only (`specs/architecture/20-surface.md`): compare the surface against the
            // live harness and print the report — the engine writes nothing;
            // `emit` owns write-back. Every custom kind the harness's
            // assembly registers is scanned at its `governs` locus beside the
            // built-ins, so a hand-edited `specs/*.md` shows as drift.
            let ws = Workspace::load(&into)?;
            let custom_kinds = load_custom_kinds(&harness_path)?;
            let report = drift::diff(&ws, &into, &harness_path, &custom_kinds)?;
            print!("{}", drift::render(&report));
            Ok(ExitCode::SUCCESS)
        }
        Command::Emit {
            into,
            frozen,
            dry_run,
        } => {
            // The write direction (`specs/architecture/20-surface.md`, law 5): re-emit each
            // projection whole onto its recorded provenance path — the source it came
            // from, so no harness root is re-supplied here (unlike `diff`, whose harness
            // arg drives its rescan for the "added" axis).
            let ws = Workspace::load(&into)?;
            let report = drift::emit(&ws, &into, drift::EmitOptions { dry_run, frozen })?;
            if dry_run {
                println!("dry run — no files written");
            }
            print!("{}", drift::render_emit(&report));
            Ok(ExitCode::SUCCESS)
        }
        Command::SessionStart { harness_path } => {
            let authored = harness_path.join(TEMPER_DIR);
            let temper_toml = harness_path.join(TEMPER_TOML);
            let diagnostics = if authored.is_dir() && temper_toml.is_file() {
                // Surface-present: gate the surface itself (two-step path), never a
                // fresh import. A fresh import discards recognition (the authored
                // `satisfies` links), so every filled requirement would read
                // unfilled — the false positive on clean input the spec's
                // surface-present clause forbids (law 3).
                gate(&authored, &authored, &temper_toml)?
            } else {
                // Surfaceless fallback: import the raw harness into a scratch
                // surface. Members land in the scratch; the authored layer is read
                // from the harness itself, not the process CWD.
                let scratch = scratch_surface()?;
                import::run(&harness_path, &scratch)?;
                let diagnostics = gate(&scratch, &authored, &temper_toml)?;
                // A leftover temp dir must never fail the advisory gate.
                let _ = fs::remove_dir_all(&scratch);
                diagnostics
            };

            println!("{}", reporter::session_start(&diagnostics));
            Ok(ExitCode::SUCCESS)
        }
        Command::Install { path, dry_run } => {
            let report = install::run(&path, dry_run)?;
            if dry_run {
                println!("dry run — no files written");
            }
            print!("{}", install::render(&report));
            Ok(ExitCode::SUCCESS)
        }
        Command::Bundle { path, out } => {
            let report = bundle::run(&path, &out)?;
            print!("{}", bundle::render(&report));
            Ok(ExitCode::SUCCESS)
        }
        Command::Why { member } => {
            let workspace = PathBuf::from(DEFAULT_WORKSPACE);
            let ws = Workspace::load(&workspace)?;
            let layer = load_layer(Path::new(TEMPER_TOML))?;

            // Build the *same* by-kind corpus + edge set the `check` gate derives
            // (READ-EDGE-UNIFY: one source of truth), so `why`'s edge narration
            // ranges over the exact resolved set `graph::check`/`acyclic`/`degree`
            // do — never a private re-derivation.
            // The by-kind feature corpus reads each built-in kind's surface members
            // through the same generic `Unit` loader the gate uses (no IR→Unit adapter
            // on the check path); the typed `ws` survives for the read family below.
            let skill_units = check::surface_units(&workspace, "skills", "SKILL.md")?;
            let rule_units = check::surface_units(&workspace, "rules", "RULE.md")?;
            let skill_features: Vec<extract::Features> = skill_units
                .iter()
                .map(builtin_kind::skill_features)
                .collect::<Result<_, _>>()?;
            let rule_features: Vec<extract::Features> = rule_units
                .iter()
                .map(builtin_kind::rule_features)
                .collect::<Result<_, _>>()?;
            let kinds_dir = workspace.join("kinds");
            let (custom_kinds, edges, custom_members) = match layer.as_ref() {
                Some(layer) => {
                    let (custom_kinds, edges) =
                        custom_kinds_and_edges(&workspace, layer, &kinds_dir)?;
                    // The forward `satisfies` walk needs each custom member's rationale,
                    // which the feature view above drops — so load the members whole
                    // (READ-CUSTOM-SATISFIERS), beside the edge set the walk shares.
                    let members = custom_members(&workspace, layer, &kinds_dir)?;
                    (custom_kinds, edges, members)
                }
                // No `temper.toml` ⇒ no declared relationships, so no edges, and no
                // registered custom kinds, so no custom members.
                None => (Vec::new(), Vec::new(), Vec::new()),
            };
            let by_kind = assemble_by_kind(&skill_features, &rule_features, &custom_kinds);

            // The forward walk narrates a `satisfies` join over the **composed**
            // requirement namespace the gate judges (assembly ∪ member-published,
            // READ-VERBS-PUBLISHED-DEMANDS), so a member-published demand reads as
            // filled — never the false "This link dangles" over a green graph.
            let roster = composed_roster(
                layer.as_ref(),
                &skill_features,
                &rule_features,
                &custom_kinds,
            );

            print!(
                "{}",
                read::why(
                    &ws,
                    layer.as_ref(),
                    &custom_members,
                    &roster,
                    &by_kind,
                    &edges,
                    &member
                )
            );
            Ok(ExitCode::SUCCESS)
        }
        Command::Requirements { name } => {
            let workspace = PathBuf::from(DEFAULT_WORKSPACE);
            let ws = Workspace::load(&workspace)?;
            let layer = load_layer(Path::new(TEMPER_TOML))?;

            // The reverse walk ranges over the composed requirement namespace the gate
            // judges — assembly ∪ member-published (READ-VERBS-PUBLISHED-DEMANDS) — so a
            // member-published obligation appears in the roster exactly as `check`
            // counts it. That needs the same member feature stream the union reads:
            // the built-in kinds' features (through the shared `Unit` loader) plus each
            // registered custom kind's, which both publish and satisfy requirements.
            let skill_units = check::surface_units(&workspace, "skills", "SKILL.md")?;
            let rule_units = check::surface_units(&workspace, "rules", "RULE.md")?;
            let skill_features: Vec<extract::Features> = skill_units
                .iter()
                .map(builtin_kind::skill_features)
                .collect::<Result<_, _>>()?;
            let rule_features: Vec<extract::Features> = rule_units
                .iter()
                .map(builtin_kind::rule_features)
                .collect::<Result<_, _>>()?;
            let kinds_dir = workspace.join("kinds");
            // Custom kinds carry both the members' published requirements (folded into
            // the composed roster) and their satisfier rows (READ-CUSTOM-SATISFIERS).
            let (custom_kinds, custom_members) = match layer.as_ref() {
                Some(layer) => {
                    let (custom_kinds, _edges) =
                        custom_kinds_and_edges(&workspace, layer, &kinds_dir)?;
                    let members = custom_members(&workspace, layer, &kinds_dir)?;
                    (custom_kinds, members)
                }
                None => (Vec::new(), Vec::new()),
            };
            let roster = composed_roster(
                layer.as_ref(),
                &skill_features,
                &rule_features,
                &custom_kinds,
            );

            print!(
                "{}",
                read::requirements(&ws, &custom_members, &roster, name.as_deref())
            );
            Ok(ExitCode::SUCCESS)
        }
        Command::Impact { member } => {
            let workspace = PathBuf::from(DEFAULT_WORKSPACE);
            let layer = load_layer(Path::new(TEMPER_TOML))?;

            // The blast radius reads the same graph inputs the gate's predicates range
            // over (READ-EDGE-UNIFY): the by-kind feature corpus, the composed roster,
            // the observed directive edges, and each kind's activation — so `impact`
            // cannot disagree with a green `check`.
            let skill_units = check::surface_units(&workspace, "skills", "SKILL.md")?;
            let rule_units = check::surface_units(&workspace, "rules", "RULE.md")?;
            let skill_features: Vec<extract::Features> = skill_units
                .iter()
                .map(builtin_kind::skill_features)
                .collect::<Result<_, _>>()?;
            let rule_features: Vec<extract::Features> = rule_units
                .iter()
                .map(builtin_kind::rule_features)
                .collect::<Result<_, _>>()?;
            let kinds_dir = workspace.join("kinds");
            let custom_kinds = match layer.as_ref() {
                Some(layer) => custom_kinds_and_edges(&workspace, layer, &kinds_dir)?.0,
                None => Vec::new(),
            };
            let by_kind = assemble_by_kind(&skill_features, &rule_features, &custom_kinds);
            let roster = composed_roster(
                layer.as_ref(),
                &skill_features,
                &rule_features,
                &custom_kinds,
            );
            // The assembly's own roster, kept distinct from the composed one so `impact`
            // can tell a demand a member alone publishes from one the assembly carries.
            let empty_roster = BTreeMap::new();
            let assembly = layer
                .as_ref()
                .map_or(&empty_roster, compose::AuthorLayer::requirements);

            // The world→member activation edges and the observed `@import` directive
            // edges the reachability strand closes over — the same derivation the gate's
            // `reachable` runs (`specs/architecture/45-governance.md`). Keyed by bare kind name,
            // the keying `by_kind` and the directive classing join on.
            let builtin_defs = builtin_kind::definitions()?;
            let mut activations: BTreeMap<&str, kind::Activation> = BTreeMap::new();
            for def in builtin_defs.values() {
                if let Some(activation) = &def.activation {
                    activations.insert(def.name.as_str(), activation.clone());
                }
            }
            for (name, custom, _features) in &custom_kinds {
                if let Some(activation) = &custom.activation {
                    activations.insert(name, activation.clone());
                }
            }

            let base_dir = Path::new(".");
            let repo_files = repo_file_set(base_dir);
            let directive_members = collect_directive_members(&workspace, &custom_kinds)?;
            let directive_edges = graph::classify_directives(&directive_members, &repo_files).edges;

            // A leaf address (`member/genre/key/field-path`) dispatches to leaf grain,
            // reading each member's serialized genre values off `by_kind`. Citations are
            // the declared one-way edges naming a leaf; the floor carries no producer yet —
            // floor leaves carry no mentions (`specs/architecture/20-surface.md`, "Genre values"), so
            // the set is empty until the altitude serializes them, and the leaf-grain report
            // names zero citers today.
            let citations: Vec<read::Citation> = Vec::new();

            print!(
                "{}",
                read::impact(
                    assembly,
                    &roster,
                    &by_kind,
                    &activations,
                    &repo_files,
                    &directive_edges,
                    &citations,
                    &member,
                )
            );
            Ok(ExitCode::SUCCESS)
        }
        Command::Context { address } => {
            let workspace = PathBuf::from(DEFAULT_WORKSPACE);
            let layer = load_layer(Path::new(TEMPER_TOML))?;

            // The neighborhood reads the same by-kind feature corpus `check` computes
            // (READ-EDGE-UNIFY): each member's serialized genre values (the leaf surface),
            // `satisfies`, and its genre slots — no runtime, just the manifest. Mirrors the
            // `Impact` dispatch's corpus assembly; the blast-radius-only inputs (assembly
            // roster, activations, directive edges) are not read at neighborhood grain.
            let skill_units = check::surface_units(&workspace, "skills", "SKILL.md")?;
            let rule_units = check::surface_units(&workspace, "rules", "RULE.md")?;
            let skill_features: Vec<extract::Features> = skill_units
                .iter()
                .map(builtin_kind::skill_features)
                .collect::<Result<_, _>>()?;
            let rule_features: Vec<extract::Features> = rule_units
                .iter()
                .map(builtin_kind::rule_features)
                .collect::<Result<_, _>>()?;
            let kinds_dir = workspace.join("kinds");
            let custom_kinds = match layer.as_ref() {
                Some(layer) => custom_kinds_and_edges(&workspace, layer, &kinds_dir)?.0,
                None => Vec::new(),
            };
            let by_kind = assemble_by_kind(&skill_features, &rule_features, &custom_kinds);

            // Citations are the declared one-way edges naming a leaf; the floor carries no
            // producer yet — floor leaves carry no mentions (`specs/architecture/20-surface.md`,
            // "Genre values") — so the set is empty and the neighborhood names zero citers today.
            let citations: Vec<read::Citation> = Vec::new();

            print!("{}", read::context(&by_kind, &citations, &address));
            Ok(ExitCode::SUCCESS)
        }
    }
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

/// Load every custom kind a harness's assembly registers, mirroring the discovery
/// [`import::run`] uses: the `temper.toml` beside the harness declares the roster,
/// and each definition lives in `<harness>/.temper/kinds/<name>/KIND.md`
/// (`specs/architecture/40-composition.md`). The drift engine scans each returned kind's `governs`
/// locus, so `diff` reports custom-kind body drift exactly as `import` projects
/// them. Absent a `temper.toml`, an empty list — the built-in kinds drift alone.
fn load_custom_kinds(harness: &Path) -> miette::Result<Vec<CustomKind>> {
    let Some(layer) = compose::AuthorLayer::load(&harness.join(TEMPER_TOML))? else {
        return Ok(Vec::new());
    };
    let kinds_dir = harness.join(TEMPER_DIR).join("kinds");
    let mut kinds = Vec::new();
    for name in layer.registered_kinds() {
        // A bare `[kind.<name>]` resolving to a built-in is a contract layer, not a
        // registration (`specs/architecture/40-composition.md`), so it declares no `governs` locus.
        // Routed through provider resolution (`specs/architecture/15-kinds.md`): a bare name resolves
        // to its unique provider-qualified kind, and a two-provider collision is a load error.
        if builtin_kind::definition(name)?.is_some() {
            continue;
        }
        kinds.push(CustomKind::load(&kinds_dir, name)?);
    }
    Ok(kinds)
}

/// The set of every kind the gate can dispatch a member to — the embedded built-in
/// std-lib ∪ each registered custom kind (`specs/architecture/40-composition.md`). The
/// resolution set [`resolve_member_kind`] ranges over to key a member by its bare kind
/// and to decide a kind is unrecognized. A `[kind.<name>]` naming a built-in is a
/// contract layer, not a registration, so it is not re-loaded as a custom kind.
///
/// # Errors
///
/// Propagates a [`KindError`] if an embedded built-in or a registered `KIND.md` fails to
/// parse into an admissible kind definition.
fn known_kinds(
    layer: Option<&compose::AuthorLayer>,
    kinds_dir: &Path,
) -> miette::Result<Vec<CustomKind>> {
    let mut kinds: Vec<CustomKind> = builtin_kind::definitions()?.into_values().collect();
    if let Some(layer) = layer {
        for name in layer.registered_kinds() {
            if builtin_kind::definition(name)?.is_some() {
                continue;
            }
            kinds.push(CustomKind::load(kinds_dir, name)?);
        }
    }
    Ok(kinds)
}

/// Resolve a manifest member's authored `kind` to the **bare** name the dispatch loop
/// keys on (`specs/architecture/15-kinds.md`, "Decision: kind identity carries a provider
/// axis"). The SDK stamps the qualified identity `<provider>.<name>`; the gate is the
/// side that resolves it — strip any provider and accept the bare tail iff some known
/// kind carries that bare name. `None` when none does — an unrecognized kind.
///
/// The bare tail is *not* run through [`CustomKind::resolve_bare`]: the corpus is
/// bare-keyed and the dispatch loop reads bare (`corpus.get(&kind.name)`), so a bare
/// name two providers share (the two `memory` kinds) resolves to that one shared slice
/// by design, never a collision error — the qualification tax is paid only where a
/// binding must name *one* kind, which member keying does not.
fn resolve_member_kind(authored: &str, known: &[CustomKind]) -> Option<String> {
    let bare = authored.rsplit('.').next().unwrap_or(authored);
    known
        .iter()
        .find(|kind| kind.name == bare)
        .map(|kind| kind.name.clone())
}

/// Merge a manifest member slice into `corpus` under its **bare** kind key, resolving
/// the authored (possibly `<provider>.<name>` qualified) kind through `known_kinds`
/// (`specs/architecture/15-kinds.md`). A kind resolving to no built-in or custom definition
/// yields every member of the slice as a loud finding into `unknown` — the
/// collaboration-rule failure a silent `checked 0` would be
/// (`.claude/rules/collaboration.md`, "a silent skip reads as done").
fn place_members(
    authored_kind: &str,
    members: Vec<extract::Features>,
    known_kinds: &[CustomKind],
    corpus: &mut BTreeMap<String, Vec<extract::Features>>,
    unknown: &mut Vec<check::Diagnostic>,
) {
    match resolve_member_kind(authored_kind, known_kinds) {
        Some(bare) => corpus.entry(bare).or_default().extend(members),
        None => unknown.extend(members.into_iter().map(|features| {
            check::Diagnostic::error(
                "manifest.unknown-kind",
                features.id.clone(),
                format!(
                    "member `{}` declares kind `{authored_kind}`, which resolves to no \
                     built-in or custom kind — it would be checked against nothing",
                    features.id
                ),
            )
        })),
    }
}

/// Produce the merged diagnostic set for a surface `workspace` against the active
/// by-kind contracts — the shared gate behind both `check` and the session-start
/// reporter (`specs/architecture/10-contracts.md`, both greens).
///
/// `authored` is the `.temper/` the author's own kinds/packages are read from,
/// kept distinct from `workspace` (the surface imported members are enumerated
/// from). They coincide on the two-step `check` path; on the one-shot path
/// (session-start, `check --harness`) members import into a throwaway scratch
/// (`workspace`) while the authored `KIND.md`/`PACKAGE.md` live beside the
/// harness's `temper_toml` (`authored`), so a custom kind's definition resolves
/// from the harness, not the scratch.
fn gate(
    workspace: &Path,
    authored: &Path,
    temper_toml: &Path,
) -> miette::Result<Vec<check::Diagnostic>> {
    // Absent `temper.toml` ⇒ `None` and the by-kind floor runs verbatim; present
    // ⇒ it layers over the floor per kind below (`specs/architecture/40-composition.md`).
    let mut layer = load_layer(temper_toml)?;

    // The temper-owned assembly-fact artifacts (`roster.toml`/`bindings.toml`) sit beside
    // `temper.toml` (`specs/architecture/20-surface.md`, "the bindings, the roster — are emitted
    // as small committed temper-owned artifacts"). When present, the gate reads its
    // requirement roster + kind bindings from them as the assembly source, so an
    // SDK-emitted members-only manifest resolves its `satisfies` instead of dangling. The
    // manifest layer's own inline roster/bindings, if any, take precedence (merge fills
    // only what it left absent). Located beside the manifest — the CWD for a two-step
    // `check`, the harness path for the one-shot gate.
    let assembly_dir = temper_toml
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    if let Some(artifacts) = assembly_artifacts::load(assembly_dir)? {
        layer
            .get_or_insert_with(|| compose::AuthorLayer::empty(temper_toml))
            .merge_assembly(artifacts.requirements, artifacts.bindings);
    }

    // A bound package resolves against the built-in floor ∪ this directory
    // (`specs/architecture/20-surface.md`); absent a binding the floor runs, so it is never read
    // on the floor-only path. Rooted at `authored`, not `workspace` (see fn doc),
    // so a one-shot gate reads it from the harness, not the scratch.
    let packages_dir = authored.join("packages");

    // A registered custom kind's definition resolves from
    // `<authored>/kinds/<name>/KIND.md` (`specs/architecture/40-composition.md`) — read only
    // when the assembly registers one, so the floor-only path never touches it.
    let kinds_dir = authored.join("kinds");

    // The harness root the in-place members' `source` paths resolve against — the
    // directory the manifest lives in (the CWD for a two-step `check`, the harness path
    // for the one-shot gate). Reused for the file-set/directive tier below.
    let harness_root = temper_toml
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));

    // The member-feature corpus (`specs/architecture/20-surface.md`, "the only thing the gate
    // reads"): a document/module-carried `[[member]]` arrives **pre-extracted** (its baked
    // features ARE the corpus), while an **in-place** member (a `source`-bearing table) is
    // **live-extracted** from its landscape file here — no projection, no drift, the file
    // is its own source. When the manifest carries neither (a floor manifest not yet
    // holding its members — temper's own dogfood, any pre-`init` harness), the gate falls
    // back to extracting the authored sources through the one generic `Unit` loader
    // (`specs/architecture/15-kinds.md`, "A built-in kind is an adapter"). Keyed by bare kind name;
    // the typed `Workspace` still survives for drift/bundle/apply and the read family.
    // The resolution set a pre-extracted member's authored kind resolves against — the
    // embedded built-ins ∪ each registered custom kind (`specs/architecture/15-kinds.md`,
    // "Decision: kind identity carries a provider axis").
    let known_kinds = known_kinds(layer.as_ref(), &kinds_dir)?;

    let (manifest_corpus, unknown_kind_diagnostics) = {
        let mut corpus: BTreeMap<String, Vec<extract::Features>> = BTreeMap::new();
        let mut unknown: Vec<check::Diagnostic> = Vec::new();
        if let Some(layer) = layer.as_ref() {
            // Document/module-carried members arrive pre-extracted, grouped by their
            // authored kind. The SDK stamps the qualified identity `<provider>.<name>`
            // (`specs/architecture/15-kinds.md`), so resolve each to the bare key the dispatch
            // loop reads (`corpus.get(&kind.name)`) before merging — a `claude-code.rule`
            // member must reach the `rule` slice, not sit unread under a qualified key.
            for (authored_kind, members) in layer.member_corpus() {
                place_members(
                    &authored_kind,
                    members,
                    &known_kinds,
                    &mut corpus,
                    &mut unknown,
                );
            }
            // In-place members carry a `source` path; live-extract, then key by their
            // authored kind directly. `live_extract_inplace` has already resolved the
            // built-in by `owns_source` (the join `resolve_bare` cannot make — the two
            // `memory` providers share the bare `memory`, `specs/architecture/15-kinds.md`), so
            // the kind is a validated bare built-in name, not the SDK's qualified stamp.
            for member in layer.inplace_members() {
                let features = live_extract_inplace(harness_root, member)?;
                corpus
                    .entry(member.kind.clone())
                    .or_default()
                    .push(features);
            }
        }
        // Keep each kind's slice name-sorted for a stable diagnostic set — a lifted
        // (pre-extracted) member and the live in-place ones can interleave.
        for slice in corpus.values_mut() {
            slice.sort_by(|a, b| a.id.cmp(&b.id));
        }
        ((!corpus.is_empty()).then_some(corpus), unknown)
    };

    // Each kind's features are validated against its *effective* contract (bound
    // package ⊕ author layer) and merged into one set; the generic engine holds no
    // per-kind opinion, so a mixed harness is judged in one run (`specs/architecture/20-surface.md`).
    let skill_features: Vec<extract::Features> = match &manifest_corpus {
        Some(corpus) => corpus.get("skill").cloned().unwrap_or_default(),
        None => check::surface_units(workspace, "skills", "SKILL.md")?
            .iter()
            .map(builtin_kind::skill_features)
            .collect::<Result<_, _>>()?,
    };

    let rule_features: Vec<extract::Features> = match &manifest_corpus {
        Some(corpus) => corpus.get("rule").cloned().unwrap_or_default(),
        None => check::surface_units(workspace, "rules", "RULE.md")?
            .iter()
            .map(builtin_kind::rule_features)
            .collect::<Result<_, _>>()?,
    };

    // The embedded std-lib a by-name `package` binding resolves against before
    // `.temper/packages/` (`specs/architecture/10-contracts.md`). Packages **compose**: a
    // satisfier is checked by its kind's bound package *and* any package a
    // requirement names. Held for the guarded roster/graph tier below.
    let builtins = builtin::contracts()?;
    let package_resolver = compose::PackageResolver::new(builtins, packages_dir.clone());

    // The generic two-greens over EVERY embedded built-in kind, keyed by qualified
    // identity (`specs/architecture/20-surface.md`, "Artifact kinds & package binding"): each
    // kind's members are dispatched to its floor package (⊕ author layer) and validated,
    // so a discovered CLAUDE.md/AGENTS.md memory member fires its `memory` clauses exactly
    // as a skill/rule does — no longer silently skipped by a hardcoded skill/rule pair.
    // Floors bind by QUALIFIED identity, never the bare name: the two `memory` providers
    // share the bare `memory` by design (86d5b70), so a bare resolve would be ambiguous.
    // SCOPE: only this validation path generalizes — the roster/graph tier below stays
    // skill/rule/custom (no memory member publishes a requirement today; folding more
    // built-ins into the requirement corpus is the separate `(builtin-workspace-qualified-key)`
    // fork), so `skill_features`/`rule_features` above are still read there.
    let mut diagnostics = Vec::new();
    // A manifest member whose authored kind resolved to no known kind is a loud finding
    // (GATE-KIND-RESOLVE) — never a silent `checked 0` (`.claude/rules/collaboration.md`,
    // "a silent skip reads as done").
    diagnostics.extend(unknown_kind_diagnostics);
    // Per-kind checked-member counts, keyed by qualified identity — carried out of
    // the dispatch loop for the advisory coverage note below (WEDGE-COVERAGE-NOTE),
    // so "checked N members" is stated rather than left as bare silence.
    let mut member_counts: BTreeMap<String, usize> = BTreeMap::new();
    for kind in builtin_kind::definitions()?.values() {
        let qualified = kind.qualified_name();
        let package = builtin::floor_package(&qualified).ok_or_else(|| {
            miette::miette!("built-in kind `{qualified}` ships no floor package binding")
        })?;
        // Two greens (`specs/architecture/10-contracts.md`): admissibility — the contract validated
        // against the definition before it is trusted to judge — then conformance.
        let contract = compose::effective(
            layer.as_ref(),
            &kind.name,
            builtin_floor(package)?,
            &packages_dir,
        )?;

        // Manifest mode: the kind's members are pre-extracted under its bare name. Fallback:
        // a discovered member routes to its kind by its source glob — the two `memory`
        // providers share the surface locus (`./MEMORY.md`), so `owns_source` keeps a
        // `CLAUDE.md` member off `agents-md.memory` and an `AGENTS.md` member off
        // `memory.anthropic`; the manifest already carries `member.kind`, so it needs no
        // re-routing.
        let features: Vec<extract::Features> = match &manifest_corpus {
            Some(corpus) => corpus.get(&kind.name).cloned().unwrap_or_default(),
            None => {
                let units = check::surface_units(
                    workspace,
                    kind.surface_subdir(),
                    &kind.member_document(),
                )?;
                units
                    .iter()
                    .filter(|unit| kind.owns_source(&unit.source_path))
                    .map(|unit| builtin_kind::features(kind, unit))
                    .collect()
            }
        };

        diagnostics.extend(engine::admissibility(&contract));
        diagnostics.extend(engine::validate(&contract, &features));
        member_counts.insert(qualified, features.len());
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
        graph::classify_directives(&collect_directive_members(workspace, &[])?, &repo_files)
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
        // The custom-kind corpus + declared edge set, built through the *shared*
        // [`custom_kinds_and_edges`] helper the `why` read also calls, so gate and
        // read derive one identical edge set (READ-EDGE-UNIFY). Owned here so the
        // feature slices outlive the graph tier (which borrows them via `by_kind`)
        // and the conformance loop below.
        let (custom_kinds, edges) = custom_kinds_and_edges(workspace, layer, &kinds_dir)?;

        // In manifest mode a custom kind's members are pre-extracted in the manifest too, so
        // swap the live `.temper/` features for the manifest's — keeping the loaded
        // definitions and declared edges the manifest does not carry (`specs/architecture/20-surface.md`).
        let custom_kinds: Vec<CustomKindEntry> = match &manifest_corpus {
            Some(corpus) => custom_kinds
                .into_iter()
                .map(|(name, def, _features)| {
                    let features = corpus.get(name).cloned().unwrap_or_default();
                    (name, def, features)
                })
                .collect(),
            None => custom_kinds,
        };

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
            union_published_requirements(layer.requirements(), &all_features);
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
        diagnostics.extend(graph::degree(layer.requirements(), &edges, &by_kind));

        // Directive-target classing over the **full** corpus (`specs/architecture/15-kinds.md`,
        // "Directives"): built-in *and* custom members, so a custom kind's `@import` at a
        // built-in member resolves to a member→member edge the reachability closure below
        // consumes (`repo_files` and the file-set come from the floor above). Only the
        // member-class **edges** are read here — the unbacked findings already surfaced on the
        // floor as a non-gating advisory (WEDGE-FACT-FLOOR), so they are not re-extended: an
        // assembly's power over a directive is the graph-scope reachability escalation, not the
        // unbacked fact, which is the same fact with or without an assembly.
        let directive_members = collect_directive_members(workspace, &custom_kinds)?;
        let directive_classing = graph::classify_directives(&directive_members, &repo_files);

        // `reachable` (`specs/architecture/45-governance.md`, "The world is a node"): a member whose
        // kind declares an activation is dead when the world→member edge is provably so
        // (a blank description-trigger, a zero-match paths glob). Assembly-scope and
        // opt-in like `degree` — it runs only when the assembly declares `[reachability]`,
        // at its declared severity (resolved `reachability-gate-mechanism` option b), so
        // a deliberate work-in-progress dead edge stays the author's call.
        if let Some(reachability) = layer.reachability() {
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
            match package_resolver.resolve(package_name)? {
                Some(contract) => {
                    diagnostics.extend(engine::admissibility(&contract));
                    diagnostics.extend(engine::validate(&contract, features));
                }
                None => diagnostics.push(check::Diagnostic::error(
                    format!("{name}.package"),
                    *name,
                    format!(
                        "custom kind `{name}` binds unknown package `{package_name}` (author `.temper/packages/{package_name}/PACKAGE.md`)"
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

    // The freshness fact (`specs/architecture/20-surface.md`, "Drift — two freshness facts"):
    // a committed projection whose bytes no longer match the lock's emit fingerprint is
    // `config.stale`. Read off the surface `workspace`'s lock (where the members were
    // imported and the fingerprints recorded), advisory so a hand-edited or un-re-emitted
    // projection is surfaced without failing the run — the `shared`-authority nudge.
    diagnostics.extend(drift::config_stale(workspace));

    Ok(diagnostics)
}

/// Live-extract an in-place member's [`Features`](extract::Features) from its landscape
/// file (`specs/architecture/20-surface.md`, "In-place — … features are extracted"): read the raw
/// harness file at `<harness_root>/<source>`, parse its frontmatter + body through the
/// generic adapter, and run the member's built-in kind extractor — the same composed
/// extraction the copy-tree read runs, so an in-place member gates identically to a
/// document-carried one. The joins are **declared in the manifest** (the harness file
/// carries no temper annotation), so the member's `satisfies`/published requirements are
/// grafted from the assembly rather than mined from the file. The file is its own source:
/// re-read every check, it cannot drift.
///
/// In-place carriage is built-in-kind only (a custom kind's units are authored `.temper/`
/// artifacts), so a member naming a non-built-in kind is a hard error rather than a silent
/// skip.
///
/// # Errors
///
/// Returns an error if the kind is not a built-in, or the landscape file is unreadable or
/// malformed.
fn live_extract_inplace(
    harness_root: &Path,
    member: &compose::InPlaceMember,
) -> miette::Result<extract::Features> {
    let path = harness_root.join(&member.source);
    // Route to the built-in kind by bare name, then by which owns the source glob — so the
    // two `memory` providers (`CLAUDE.md` vs `AGENTS.md`) that share the bare `memory`
    // resolve to the right one rather than colliding on an ambiguous bare lookup
    // (`specs/architecture/15-kinds.md`, "kind identity carries a provider axis").
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
    let unit = Unit {
        id: member.name.clone(),
        frontmatter: source.fields.iter().cloned().collect(),
        body: source.body.clone(),
        source_path: source.provenance.source_path.clone(),
        satisfies: Vec::new(),
        satisfies_clauses: Vec::new(),
        published_requirements: Vec::new(),
    };
    let mut features = builtin_kind::features(kind, &unit);
    // The join edges are the assembly's declaration, not a fact mined from the file.
    features.satisfies = member.satisfies.clone();
    features.published_requirements = member.published.clone();
    Ok(features)
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

/// Create a fresh throwaway surface directory for the one-shot import — a scratch
/// workspace under the system temp dir, unique to this process. The one-shot gate
/// never persists a surface (unlike `import --into`), so the caller tears it down.
fn scratch_surface() -> miette::Result<PathBuf> {
    let dir = std::env::temp_dir().join(format!("temper-session-start-{}", std::process::id()));
    // A stale directory from a crashed prior run must not poison this import.
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).into_diagnostic()?;
    Ok(dir)
}

/// A registered custom kind as the corpus construction carries it: its name (borrowed
/// from the assembly layer), its loaded [`CustomKind`] definition, and its computed
/// member [`Features`](extract::Features). Named so the shared corpus helpers keep a
/// legible signature (`clippy::type_complexity`).
type CustomKindEntry<'a> = (&'a str, CustomKind, Vec<extract::Features>);

/// Load every registered custom kind (definition + computed features) and assemble
/// the declared edge set — `layer.edges()` plus each custom kind's own
/// `[[relationships]]`. The **one** construction the `check` gate and the `why`
/// read both derive their by-kind corpus + edge set from (READ-EDGE-UNIFY: no
/// private re-derivation).
///
/// A `[kind.<name>]` whose name is a built-in is a contract layer, not a
/// registration, so it is skipped (`specs/architecture/40-composition.md`). The returned names
/// borrow `layer`, so it outlives the corpus.
fn custom_kinds_and_edges<'a>(
    workspace: &Path,
    layer: &'a compose::AuthorLayer,
    kinds_dir: &Path,
) -> miette::Result<(Vec<CustomKindEntry<'a>>, Vec<compose::Edge>)> {
    let mut custom_kinds: Vec<CustomKindEntry> = Vec::new();
    for name in layer.registered_kinds() {
        // Provider resolution splits built-in layers from custom registrations
        // (`specs/architecture/15-kinds.md`): a bare name resolving to an embedded kind is a layer.
        if builtin_kind::definition(name)?.is_some() {
            continue;
        }
        let custom = CustomKind::load(kinds_dir, name)?;
        let units = custom_units(workspace, &custom)?;
        let features: Vec<extract::Features> =
            units.iter().map(|unit| custom.extract(unit)).collect();
        custom_kinds.push((name, custom, features));
    }

    // A built-in kind declares its edges in the assembly (`layer.edges()`), a
    // custom kind in its own `KIND.md` (`specs/architecture/15-kinds.md`).
    let mut edges: Vec<compose::Edge> = layer.edges().to_vec();
    for (_name, custom, _features) in &custom_kinds {
        edges.extend(custom.relationships.iter().cloned());
    }

    Ok((custom_kinds, edges))
}

/// Load every registered custom kind's members as the read family sees them
/// (READ-CUSTOM-SATISFIERS): each member's kind name, its id, and its
/// rationale-carrying `satisfies` clauses ([`kind::Unit::satisfies_clauses`]). The
/// read family (`why`/`requirements`) ranges over custom-kind satisfiers exactly as it
/// does skills/rules, so a custom member filling a requirement is reported rather than
/// silently absent. Loaded off the same units the gate extracts from — via the shared
/// [`custom_units`] loader — but carrying the rationale the decidable feature view
/// drops. A `[kind.<name>]` naming a built-in is a contract layer, not a registration
/// (`specs/architecture/40-composition.md`), so it is skipped.
fn custom_members(
    workspace: &Path,
    layer: &compose::AuthorLayer,
    kinds_dir: &Path,
) -> miette::Result<Vec<read::CustomMember>> {
    let mut members = Vec::new();
    for name in layer.registered_kinds() {
        // Provider resolution splits built-in layers from custom registrations
        // (`specs/architecture/15-kinds.md`), the same test the gate applies.
        if builtin_kind::definition(name)?.is_some() {
            continue;
        }
        let custom = CustomKind::load(kinds_dir, name)?;
        for unit in custom_units(workspace, &custom)? {
            members.push(read::CustomMember {
                kind: name.to_string(),
                id: unit.id,
                satisfies: unit.satisfies_clauses,
            });
        }
    }
    Ok(members)
}

/// Assemble the by-kind [`Features`](extract::Features) corpus every set-scope and
/// graph predicate ranges over: the built-in kinds plus each custom kind's
/// features, keyed by kind name. The counterpart to [`custom_kinds_and_edges`]
/// shared by the gate and the `why` read (READ-EDGE-UNIFY). Borrows every slice, so
/// the caller holds the owned feature vecs for the map's lifetime.
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
/// generalization CHECK-MEMBERS-ALL-KINDS made for clause dispatch). Each kind loads its
/// members through the one generic surface loader, filtered by
/// [`CustomKind::owns_source`] so the two `memory` providers sharing the `./MEMORY.md`
/// locus route to their own carrier, extracts through [`builtin_kind::features`], and
/// keys by the bare `kind.name` — the keying `by_kind`/`classify_directives` join on.
/// Each custom kind's units are re-read in the same sorted order [`custom_units`] loads
/// them, so the zip aligns.
fn collect_directive_members(
    workspace: &Path,
    custom_kinds: &[CustomKindEntry<'_>],
) -> miette::Result<Vec<graph::DirectiveMember>> {
    let mut members = Vec::new();
    for kind in builtin_kind::definitions()?.values() {
        let units =
            check::surface_units(workspace, kind.surface_subdir(), &kind.member_document())?;
        for unit in units
            .iter()
            .filter(|unit| kind.owns_source(&unit.source_path))
        {
            let feature = builtin_kind::features(kind, unit);
            members.push(graph::DirectiveMember {
                kind: kind.name.clone(),
                id: feature.id.clone(),
                source_path: unit.source_path.clone(),
                directives: feature.directives.clone(),
            });
        }
    }
    for (name, custom, features) in custom_kinds {
        for (unit, feature) in custom_units(workspace, custom)?.iter().zip(features) {
            members.push(graph::DirectiveMember {
                kind: (*name).to_string(),
                id: feature.id.clone(),
                source_path: unit.source_path.clone(),
                directives: feature.directives.clone(),
            });
        }
    }
    Ok(members)
}

/// The composed requirement namespace the read family (`why`/`requirements`) ranges
/// over — the assembly's `[requirement.*]` roster unioned with every member's published
/// `[requirement.*]`, the exact namespace the `check` gate judges
/// ([`union_published_requirements`], READ-VERBS-PUBLISHED-DEMANDS). Ranging over it,
/// not the assembly roster alone, is what makes a read agree with a green `check`: a
/// member-published join reads as live, never misreported as dangling. Collisions are
/// the gate's admissibility finding, discarded here — a read never re-reports them
/// (`specs/architecture/20-surface.md`, the read family "never gates"). Absent a `temper.toml` the
/// gate composes no roster, so this is empty too.
fn composed_roster(
    layer: Option<&compose::AuthorLayer>,
    skill_features: &[extract::Features],
    rule_features: &[extract::Features],
    custom_kinds: &[CustomKindEntry<'_>],
) -> BTreeMap<String, compose::Requirement> {
    let Some(layer) = layer else {
        return BTreeMap::new();
    };
    let all_features: Vec<extract::Features> = skill_features
        .iter()
        .chain(rule_features.iter())
        .chain(custom_kinds.iter().flat_map(|(_, _, features)| features))
        .cloned()
        .collect();
    union_published_requirements(layer.requirements(), &all_features).0
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

/// Load a custom `kind`'s units from the surface generically — every surface
/// directory under the workspace at the kind's declared `governs.root`, each
/// reloaded via [`Unit::from_surface_dir`]. Keyed on the declared locus, never the
/// kind name: temper reads its own `specs/` because its `temper.toml` roots a kind
/// there, and a kind rooted anywhere else is read the same way
/// (`specs/architecture/40-composition.md`).
///
/// A surface directory holds the kind's `<KIND>.md` member document, name-sorted
/// for stable output. No directory at the root ⇒ no units, and the contract's
/// admissibility still runs over zero artifacts.
fn custom_units(workspace_dir: &Path, custom: &CustomKind) -> Result<Vec<Unit>, KindError> {
    let root = workspace_dir.join(&custom.governs.root);
    if !root.is_dir() {
        return Ok(Vec::new());
    }

    // The member document a custom unit is written under — the kind name upper-cased
    // (`spec` → `SPEC.md`), the same convention `import` writes (`src/import.rs`).
    let document = format!("{}.md", custom.name.to_uppercase());
    let listing = fs::read_dir(&root).map_err(|source| KindError::Io {
        path: root.clone(),
        source,
    })?;
    let mut dirs = Vec::new();
    for entry in listing {
        let entry = entry.map_err(|source| KindError::Io {
            path: root.clone(),
            source,
        })?;
        let path = entry.path();
        if path.is_dir() && path.join(&document).is_file() {
            dirs.push(path);
        }
    }
    dirs.sort();

    dirs.iter().map(|dir| Unit::from_surface_dir(dir)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

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
