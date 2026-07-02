//! `temper` CLI entry point.
//!
//! A thin `clap` dispatch over the [`temper`] library: parse args, run the
//! generic contract engine, map the result to an exit code. Every subcommand
//! mirrors the CLI surface of `specs/20-surface.md`; all logic lives in the
//! library so `tests/` can drive it.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use miette::IntoDiagnostic;
use temper::builtin;
use temper::bundle;
use temper::check::{self, Severity, Workspace};
use temper::compose;
use temper::contract::Contract;
use temper::coverage;
use temper::drift;
use temper::engine;
use temper::extract;
use temper::graph;
use temper::import;
use temper::install;
use temper::kind::{self, CustomKind, KindError, Unit};
use temper::read;
use temper::reporter;
use temper::roster;
use temper::schema;

/// The surface workspace default for `--into` / the `check` argument
/// (`specs/20-surface.md`): a `.temper` directory under the cwd.
const DEFAULT_WORKSPACE: &str = "./.temper";

/// The authored surface directory beside a harness's `temper.toml`, holding its
/// custom-kind definitions (`kinds/<name>/KIND.md`) and packages
/// (`packages/<name>/PACKAGE.md`) — `specs/40-composition.md`. On the one-shot
/// gate paths (session-start, `check --harness`) it is the authored root handed
/// to [`gate`], distinct from the throwaway scratch surface the members land in.
const TEMPER_DIR: &str = ".temper";

/// The optional author-declared contract layer, discovered at the project root
/// beside the harness it governs (`specs/40-composition.md`). Absent ⇒ the
/// by-kind floor runs unchanged.
const TEMPER_TOML: &str = "temper.toml";

/// The gitignored personal override layer discovered beside [`TEMPER_TOML`]
/// (`specs/40-composition.md`): `temper.toml` is committed project policy,
/// `temper-local.toml` a developer's personal clause/severity override that
/// layers over it. Absent ⇒ the committed layer (or bare floor) runs unchanged.
const TEMPER_LOCAL_TOML: &str = "temper-local.toml";

/// Resolve a built-in package by name into its floor [`Contract`], failing loud
/// if the build embedded no package of that name (`specs/10-contracts.md`) — a
/// missing floor is a hard error, never a silently empty contract.
fn builtin_floor(name: &str) -> miette::Result<Contract> {
    builtin::contract(name)?
        .ok_or_else(|| miette::miette!("built-in package `{name}` is not embedded in this binary"))
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
    /// Scan the harness into the typed config surface (+ provenance lock).
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
        /// workspace (`specs/20-surface.md`). Conflicts with `workspace`.
        #[arg(long, conflicts_with = "workspace")]
        harness: Option<PathBuf>,
        /// Also fail the run on `advisory` (warn-severity) violations, not just
        /// `required` ones — the strict CI policy (`specs/10-contracts.md`).
        #[arg(long)]
        deny_advisories: bool,
        /// The machine format for the diagnostic set (`specs/50-distribution.md`,
        /// reporters). Presentation only — the exit-code verdict is identical
        /// whichever is chosen.
        #[arg(long, value_enum, default_value_t = Reporter::Terminal)]
        reporter: Reporter,
    },
    /// Emit the active per-kind contract as an editor JSON Schema (the keystroke
    /// gate — `specs/50-distribution.md`, "The gate at keystroke").
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
    /// Project the surface back onto its harness sources, patching only changed
    /// fields over the three-state drift model (`specs/20-surface.md`, the hard
    /// core). Each artifact writes back to the source path `import` recorded.
    Apply {
        /// The surface workspace to project (defaults to `./.temper`). The lock
        /// under it carries the last-applied fingerprints the merge stands on.
        #[arg(long, default_value = DEFAULT_WORKSPACE)]
        into: PathBuf,
        /// Compute and report every outcome without writing a single byte — not
        /// the patched sources, not the updated lock.
        #[arg(long)]
        dry_run: bool,
    },
    /// Reconcile direct on-disk harness edits back into the surface — the third
    /// drift direction (`specs/20-surface.md`). Drifted and added sources are
    /// pulled into the surface tree and the lock refreshed; an in-sync harness is
    /// a no-op. A reconcile, not a gate — it exits zero.
    ReAdd {
        /// The harness to re-scan for drifted / added on-disk sources.
        harness_path: PathBuf,
        /// The surface workspace to reconcile into (defaults to `./.temper`). Its
        /// lock is refreshed to the current source bytes.
        #[arg(long, default_value = DEFAULT_WORKSPACE)]
        into: PathBuf,
    },
    /// The advisory session-start gate (`specs/50-distribution.md`): check a
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
    /// Project temper's own gate wiring into the harness (`specs/50-distribution.md`):
    /// the `SessionStart` hook into `.claude/settings.json`, the CI job into
    /// `.github/`, and the schema modeline into each artifact's frontmatter — all
    /// under the three-state drift engine, so re-running is idempotent and re-adds
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
    /// `marketplace.json` (`specs/50-distribution.md`): the operate-the-gate skill,
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
    /// Read (`specs/20-surface.md`): narrate everything that holds a member in
    /// place — the requirements it `satisfies` (each with its rationale), the
    /// package its kind binds, and its declared edges in and out. The forward walk
    /// of the requirement↔`satisfies` edge; never gates, always exits zero.
    Why {
        /// The member (a skill or rule name) to walk the edge forward from.
        member: String,
    },
    /// Read (`specs/20-surface.md`): narrate the requirement roster — each
    /// requirement with its satisfier set and coverage state; with a `<name>`,
    /// that one's satisfiers and the blast radius a removal would strand. The
    /// reverse walk of the requirement↔`satisfies` edge; never gates, exits zero.
    Requirements {
        /// A single requirement to walk in reverse; omitted ⇒ the whole roster.
        name: Option<String>,
    },
}

/// The machine format `check` renders its diagnostic set in
/// (`specs/50-distribution.md`, reporters). Every variant reshapes *presentation
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
        Command::Import { harness_path, into } => {
            import::run(&harness_path, &into)?;
            Ok(ExitCode::SUCCESS)
        }
        Command::Check {
            workspace,
            harness,
            deny_advisories,
            reporter,
        } => {
            // Two ways into the same gate (`specs/20-surface.md`). `--harness` is
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
            // The keystroke placement of the gate (`specs/50-distribution.md`):
            // emit the *active* contract per kind — the same floor ⊕ `temper.toml`
            // layer `check` gates against — as an editor JSON Schema.
            let layer = load_layer(Path::new(TEMPER_TOML))?;

            // A bound project package resolves from the default surface's
            // `.temper/packages/`: schema takes no workspace, so it reads the
            // default rather than a caller-supplied one.
            let packages_dir = Path::new(DEFAULT_WORKSPACE).join("packages");

            let floors: Vec<(&str, Contract)> = vec![
                ("skill", builtin_floor(builtin::SKILL_PACKAGE)?),
                ("rule", builtin_floor(builtin::RULE_PACKAGE)?),
            ];

            let json = match kind.as_deref() {
                // An unknown kind is a hard error, never a silent empty schema.
                Some(requested) => {
                    let floor = floors
                        .into_iter()
                        .find_map(|(name, floor)| (name == requested).then_some((name, floor)));
                    let (name, floor) = floor.ok_or_else(|| {
                        miette::miette!("unknown kind `{requested}` (temper models: skill, rule)")
                    })?;
                    let contract = compose::effective(layer.as_ref(), name, floor, &packages_dir)?;
                    schema::emit(&contract)
                }
                None => {
                    let mut map = serde_json::Map::new();
                    for (name, floor) in floors {
                        let contract =
                            compose::effective(layer.as_ref(), name, floor, &packages_dir)?;
                        map.insert(name.to_string(), schema::emit(&contract));
                    }
                    serde_json::Value::Object(map)
                }
            };

            println!("{}", serde_json::to_string_pretty(&json).into_diagnostic()?);
            Ok(ExitCode::SUCCESS)
        }
        Command::Diff { harness_path, into } => {
            // Read-only (`specs/20-surface.md`): compare the surface against the
            // live harness and print the report — the engine writes nothing;
            // `apply`/`re-add` own write-back. Every custom kind the harness's
            // assembly registers is scanned at its `governs` locus beside the
            // built-ins, so a hand-edited `specs/*.md` shows as drift.
            let ws = Workspace::load(&into)?;
            let custom_kinds = load_custom_kinds(&harness_path)?;
            let report = drift::diff(&ws, &into, &harness_path, &custom_kinds)?;
            print!("{}", drift::render(&report));
            Ok(ExitCode::SUCCESS)
        }
        Command::Apply { into, dry_run } => {
            // The write direction (`specs/20-surface.md`): patch only changed
            // fields back onto each artifact's recorded provenance path — the
            // source it came from, so no harness root is re-supplied here (unlike
            // `diff`, whose harness arg drives its rescan for the "added" axis).
            let ws = Workspace::load(&into)?;
            let report = drift::apply(&ws, &into, drift::ApplyOptions { dry_run })?;
            if dry_run {
                println!("dry run — no files written");
            }
            print!("{}", drift::render_apply(&report));
            Ok(ExitCode::SUCCESS)
        }
        Command::ReAdd { harness_path, into } => {
            // The on-disk → surface reconcile (`specs/20-surface.md`): pull every
            // drifted / added harness source back in and refresh the lock. Unlike
            // `apply`, it re-scans the live harness (like `diff`), so it takes the
            // harness path too. The same custom kinds `diff` scans reconcile back
            // through `import`'s generic writer.
            let ws = Workspace::load(&into)?;
            let custom_kinds = load_custom_kinds(&harness_path)?;
            let report = drift::re_add(&ws, &into, &harness_path, &custom_kinds)?;
            print!("{}", drift::render_readd(&report));
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
            let skill_features: Vec<extract::Features> =
                ws.skills.iter().map(extract::skill_features).collect();
            let rule_features: Vec<extract::Features> =
                ws.rules.iter().map(extract::rule_features).collect();
            let kinds_dir = workspace.join("kinds");
            let (custom_kinds, edges) = match layer.as_ref() {
                Some(layer) => custom_kinds_and_edges(&workspace, layer, &kinds_dir)?,
                // No `temper.toml` ⇒ no declared relationships, so no edges.
                None => (Vec::new(), Vec::new()),
            };
            let by_kind = assemble_by_kind(&skill_features, &rule_features, &custom_kinds);

            print!(
                "{}",
                read::why(&ws, layer.as_ref(), &by_kind, &edges, &member)
            );
            Ok(ExitCode::SUCCESS)
        }
        Command::Requirements { name } => {
            let ws = Workspace::load(Path::new(DEFAULT_WORKSPACE))?;
            let layer = load_layer(Path::new(TEMPER_TOML))?;
            print!(
                "{}",
                read::requirements(&ws, layer.as_ref(), name.as_deref())
            );
            Ok(ExitCode::SUCCESS)
        }
    }
}

/// Load the author-declared layer for a `temper_toml` path, folding a gitignored
/// `temper-local.toml` beside it over the committed layer when present
/// (`specs/40-composition.md`). Discovered in the *same* directory as the
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
/// (`specs/40-composition.md`). The drift engine scans each returned kind's `governs`
/// locus, so `diff`/`re-add` reconcile custom-kind bodies exactly as `import` projects
/// them. Absent a `temper.toml`, an empty list — the built-in kinds drift alone.
fn load_custom_kinds(harness: &Path) -> miette::Result<Vec<CustomKind>> {
    let Some(layer) = compose::AuthorLayer::load(&harness.join(TEMPER_TOML))? else {
        return Ok(Vec::new());
    };
    let kinds_dir = harness.join(TEMPER_DIR).join("kinds");
    let mut kinds = Vec::new();
    for name in layer.registered_kinds() {
        // A `[kind.<name>]` naming a built-in is a contract layer, not a
        // registration (`specs/40-composition.md`), so it declares no `governs` locus.
        if kind::BUILTIN_KINDS.contains(&name) {
            continue;
        }
        kinds.push(CustomKind::load(&kinds_dir, name)?);
    }
    Ok(kinds)
}

/// Produce the merged diagnostic set for a surface `workspace` against the active
/// by-kind contracts — the shared gate behind both `check` and the session-start
/// reporter (`specs/10-contracts.md`, both greens).
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
    let ws = Workspace::load(workspace)?;

    // Absent `temper.toml` ⇒ `None` and the by-kind floor runs verbatim; present
    // ⇒ it layers over the floor per kind below (`specs/40-composition.md`).
    let layer = load_layer(temper_toml)?;

    // A bound package resolves against the built-in floor ∪ this directory
    // (`specs/20-surface.md`); absent a binding the floor runs, so it is never read
    // on the floor-only path. Rooted at `authored`, not `workspace` (see fn doc),
    // so a one-shot gate reads it from the harness, not the scratch.
    let packages_dir = authored.join("packages");

    // A registered custom kind's definition resolves from
    // `<authored>/kinds/<name>/KIND.md` (`specs/40-composition.md`) — read only
    // when the assembly registers one, so the floor-only path never touches it.
    let kinds_dir = authored.join("kinds");

    // Each kind's features are validated against its *effective* contract (bound
    // package ⊕ author layer) and merged into one set; the generic engine holds no
    // per-kind opinion, so a mixed harness is judged in one run (`specs/20-surface.md`).
    let skill_features: Vec<extract::Features> =
        ws.skills.iter().map(extract::skill_features).collect();

    let rule_features: Vec<extract::Features> =
        ws.rules.iter().map(extract::rule_features).collect();

    // The embedded std-lib a by-name `package` binding resolves against before
    // `.temper/packages/` (`specs/10-contracts.md`). Packages **compose**: a
    // satisfier is checked by its kind's bound package *and* any package a
    // requirement names.
    let builtins = builtin::contracts()?;
    let skill_floor = builtin_floor(builtin::SKILL_PACKAGE)?;
    let rule_floor = builtin_floor(builtin::RULE_PACKAGE)?;
    let package_resolver = compose::PackageResolver::new(builtins, packages_dir.clone());

    let skill_contract = compose::effective(layer.as_ref(), "skill", skill_floor, &packages_dir)?;
    let rule_contract = compose::effective(layer.as_ref(), "rule", rule_floor, &packages_dir)?;

    // Two greens (`specs/10-contracts.md`): **admissibility** first — each contract
    // is validated against the definition before it is trusted to judge a harness,
    // failing the run like a `required` violation — then **conformance**.
    let mut diagnostics = engine::admissibility(&skill_contract);
    diagnostics.extend(engine::admissibility(&rule_contract));
    diagnostics.extend(engine::validate(&skill_contract, &skill_features));
    diagnostics.extend(engine::validate(&rule_contract, &rule_features));

    // The harness-contract tier: set-scope predicates over the parsed roster, each
    // quantified over a requirement's satisfier set (`specs/10-contracts.md`).
    // Guarded on the layer, so the floor-only path adds nothing here or below.
    if let Some(layer) = layer.as_ref() {
        let base_dir = temper_toml.parent().unwrap_or_else(|| Path::new("."));

        // The custom-kind corpus + declared edge set, built through the *shared*
        // [`custom_kinds_and_edges`] helper the `why` read also calls, so gate and
        // read derive one identical edge set (READ-EDGE-UNIFY). Owned here so the
        // feature slices outlive the graph tier (which borrows them via `by_kind`)
        // and the conformance loop below.
        let (custom_kinds, edges) = custom_kinds_and_edges(workspace, layer, &kinds_dir)?;

        // The by-kind corpus every set-scope and graph predicate ranges over,
        // assembled through the same helper the read arm uses.
        let by_kind = assemble_by_kind(&skill_features, &rule_features, &custom_kinds);

        // Admissibility before conformance here too: each requirement's own
        // definition is validated before the roster is trusted to judge the harness
        // (`specs/10-contracts.md`).
        diagnostics.extend(roster::admissibility(
            layer.requirements(),
            &by_kind,
            &package_resolver,
            base_dir,
        ));

        // The set-scope predicates: each requirement's `count` / `unique` /
        // `membership` gate over its satisfier set (`specs/45-governance.md`).
        diagnostics.extend(roster::check(
            layer.requirements(),
            &by_kind,
            &package_resolver,
        ));

        // The `conforms-to` half: each requirement's satisfiers validated against
        // its bound `package`'s contract, retagged under `requirement.conforms-to`.
        // A non-resolving package is admissibility's finding above, skipped here
        // rather than double-reported.
        diagnostics.extend(roster::conformance(
            layer.requirements(),
            &by_kind,
            &package_resolver,
        ));

        // The graph scope: build the reference graph over the declared edges (a
        // reference is a kind capability, `specs/15-kinds.md`) and check route
        // resolution — a declared reference must resolve to a real artifact of the
        // target kind (`specs/45-governance.md`). Admissibility before conformance:
        // an edge naming no reference field or targeting an unmodeled kind is
        // reported once and skipped by the route check.
        diagnostics.extend(graph::admissibility(&edges, &by_kind));
        diagnostics.extend(graph::check(&edges, &by_kind));

        // `acyclic` (`specs/45-governance.md`): the resolved graph must contain no
        // cycle — a circular import loads nothing, so every finding is a true
        // positive. Always-on over the whole edge set, like route resolution above.
        diagnostics.extend(graph::acyclic(&edges, &by_kind));

        // `degree` (`specs/45-governance.md`): a requirement declares an in/out
        // edge-count bound every satisfier's degree must fall inside, so it takes
        // the requirements *and* the edges, reusing the arc resolution
        // `acyclic`/`check` assemble. Opt-in per requirement.
        diagnostics.extend(graph::degree(layer.requirements(), &edges, &by_kind));

        // The requirement-coverage tier (`specs/10-contracts.md`): every `required`
        // requirement must have a resolving home (≥1 artifact opting in via
        // `satisfies`) and every authored `satisfies` must resolve to a declared
        // requirement. Kind-blind: it ranges over every opt-in-capable artifact —
        // built-in kinds *and* each custom kind's members — so temper's own `spec`
        // corpus can opt in exactly as a skill does (`specs/15-kinds.md`).
        let all_features: Vec<extract::Features> = skill_features
            .iter()
            .chain(rule_features.iter())
            .chain(custom_kinds.iter().flat_map(|(_, _, features)| features))
            .cloned()
            .collect();
        diagnostics.extend(coverage::check(layer.requirements(), &all_features));

        // The custom-kind conformance tier: each registered custom kind runs the
        // same two greens the built-in kinds do (`specs/15-kinds.md`), but through
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

    // The install self-verify (`specs/50-distribution.md`): temper checking its
    // *own* gate is wired. Advisory (warn) only — a not-yet-installed gate nudges
    // without failing the run, and the session-start reporter ignores warn
    // severity. Read relative to the `temper.toml` parent (the CWD for `check`, the
    // harness path for session-start).
    let root = temper_toml
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    diagnostics.extend(install::gate_installed(root));

    Ok(diagnostics)
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
/// registration, so it is skipped (`specs/40-composition.md`). The returned names
/// borrow `layer`, so it outlives the corpus.
fn custom_kinds_and_edges<'a>(
    workspace: &Path,
    layer: &'a compose::AuthorLayer,
    kinds_dir: &Path,
) -> miette::Result<(Vec<CustomKindEntry<'a>>, Vec<compose::Edge>)> {
    let mut custom_kinds: Vec<CustomKindEntry> = Vec::new();
    for name in layer.registered_kinds() {
        if kind::BUILTIN_KINDS.contains(&name) {
            continue;
        }
        let custom = CustomKind::load(kinds_dir, name)?;
        let units = custom_units(workspace, &custom)?;
        let features: Vec<extract::Features> = units
            .iter()
            .map(|unit| custom.extraction.extract(unit))
            .collect();
        custom_kinds.push((name, custom, features));
    }

    // A built-in kind declares its edges in the assembly (`layer.edges()`), a
    // custom kind in its own `KIND.md` (`specs/15-kinds.md`).
    let mut edges: Vec<compose::Edge> = layer.edges().to_vec();
    for (_name, custom, _features) in &custom_kinds {
        edges.extend(custom.relationships.iter().cloned());
    }

    Ok((custom_kinds, edges))
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

/// Load a custom `kind`'s units from the surface generically — every surface
/// directory under the workspace at the kind's declared `governs.root`, each
/// reloaded via [`Unit::from_surface_dir`]. Keyed on the declared locus, never the
/// kind name: temper reads its own `specs/` because its `temper.toml` roots a kind
/// there, and a kind rooted anywhere else is read the same way
/// (`specs/40-composition.md`).
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
