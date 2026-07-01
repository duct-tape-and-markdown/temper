//! `temper` CLI entry point.
//!
//! Thin command dispatch over the [`temper`] library. The subcommands mirror the
//! surface in `specs/20-surface.md` ("CLI surface"): `import` scans a harness
//! into the typed config surface, `check` runs **both greens** of
//! `specs/10-contracts.md` — *admissibility* (each built-in contract is itself
//! valid against the definition) and *conformance* (each artifact satisfies its
//! contract) — and exits non-zero when either an inadmissible contract or a
//! `required`-severity conformance clause is violated. All logic lives in the
//! library — `main` only parses args, projects the workspace into the engine's
//! [`Features`] view, runs the generic contract engine (`specs/10-contracts.md`),
//! and maps the result to an exit code.
//!
//! [`Features`]: temper::extract::Features

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use miette::IntoDiagnostic;
use temper::check::{self, Severity, Workspace};
use temper::compose;
use temper::contract::Contract;
use temper::drift;
use temper::engine;
use temper::extract;
use temper::graph;
use temper::import;
use temper::kind::{KindError, Unit};
use temper::reporter;
use temper::roster;
use temper::schema;

/// The surface workspace default for `--into` / the `check` argument: a `.temper`
/// directory under the current working directory (`specs/20-surface.md`).
const DEFAULT_WORKSPACE: &str = "./.temper";

/// The optional author-declared contract layer, discovered at the project root —
/// the invocation directory, beside the harness it governs (`specs/40-composition.md`,
/// "The author-declared contract — `temper.toml`"). Absent ⇒ the by-kind floor
/// runs unchanged.
const TEMPER_TOML: &str = "temper.toml";

/// The built-in Anthropic skill contract — the curated "std-lib" default
/// (`contracts/skill.anthropic.toml`), embedded at build time so `check` has a
/// contract to validate against without any on-disk configuration.
const BUILTIN_SKILL_CONTRACT: &str = include_str!("../contracts/skill.anthropic.toml");

/// The built-in rule contract — the curated default for the `rule` artifact kind
/// (`contracts/rule.toml`), embedded beside the skill one so `check` validates
/// each artifact against the contract for *its* kind without any on-disk config
/// (`specs/20-surface.md`, "contract selection is by artifact kind").
const BUILTIN_RULE_CONTRACT: &str = include_str!("../contracts/rule.toml");

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
        /// The harness to scan: a `skills/*/SKILL.md` tree, or a bare skill dir.
        harness_path: PathBuf,
        /// Where to write the surface workspace (defaults to `./.temper`).
        #[arg(long, default_value = DEFAULT_WORKSPACE)]
        into: PathBuf,
    },
    /// Lint the config surface against the active contract.
    Check {
        /// The surface workspace to lint (defaults to `./.temper`).
        workspace: Option<PathBuf>,
        /// Also fail the run on `advisory` (warn-severity) violations, not just
        /// `required` ones — the strict CI policy from `specs/10-contracts.md`.
        #[arg(long)]
        deny_advisories: bool,
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
    /// drift direction (`specs/20-surface.md`, the hard core). Drifted and added
    /// sources are pulled into the surface tree and the lock is refreshed; an
    /// in-sync harness is a no-op. A reconcile, not a gate — it exits zero.
    ReAdd {
        /// The harness to re-scan for drifted / added on-disk sources.
        harness_path: PathBuf,
        /// The surface workspace to reconcile into (defaults to `./.temper`). Its
        /// lock is refreshed to the current source bytes.
        #[arg(long, default_value = DEFAULT_WORKSPACE)]
        into: PathBuf,
    },
    /// The advisory session-start gate (`specs/50-distribution.md`, "Decision:
    /// the session-start gate is advisory, not blocking"): check a harness in one
    /// shot and emit the `claude-session-start` reporter payload on stdout for a
    /// Claude Code `SessionStart` hook. This is the one-shot import-internally
    /// path — not the two-step import-then-check of the author workflow — so it
    /// takes a *harness* path, not a surface workspace. Advisory: it always exits
    /// zero, never blocking the session; a failing contract routes through the
    /// human via the reporter's notify-and-approve verdict.
    SessionStart {
        /// The harness to check: the same tree `import` scans (a `skills/*` tree,
        /// a bare skill dir, `.claude/rules/*`, plus any `temper.toml` kinds).
        harness_path: PathBuf,
    },
}

fn main() -> miette::Result<ExitCode> {
    match Cli::parse().command {
        Command::Import { harness_path, into } => {
            import::run(&harness_path, &into)?;
            Ok(ExitCode::SUCCESS)
        }
        Command::Check {
            workspace,
            deny_advisories,
        } => {
            let workspace = workspace.unwrap_or_else(|| PathBuf::from(DEFAULT_WORKSPACE));
            let diagnostics = gate(&workspace, Path::new(TEMPER_TOML))?;

            print!("{}", check::render(&diagnostics));

            // A `required` violation always fails the run; `--deny-advisories`
            // additionally promotes `advisory` (warn) violations to blocking.
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
            // The keystroke placement of the one gate (`specs/50-distribution.md`,
            // "The gate at keystroke"): emit the *active* contract per kind — the
            // same by-kind floor ⊕ optional `temper.toml` layer `check` gates
            // against — as an editor JSON Schema. Validation channel only; the
            // docs/hover channel has no source to project (see `schema.rs`).
            let layer = compose::AuthorLayer::load(Path::new(TEMPER_TOML))?;

            // The modeled by-kind floors, paired with the kind name the layer keys
            // on — the identical floors `check` composes above.
            let floors: Vec<(&str, Contract)> = vec![
                (
                    "skill",
                    Contract::parse(BUILTIN_SKILL_CONTRACT, Path::new("skill.anthropic.toml"))?,
                ),
                (
                    "rule",
                    Contract::parse(BUILTIN_RULE_CONTRACT, Path::new("rule.toml"))?,
                ),
            ];

            let json = match kind.as_deref() {
                // `--kind <k>`: emit just that kind's schema. An unknown kind is a
                // hard error, never a silent empty schema — the caller named a kind
                // `temper` does not model.
                Some(requested) => {
                    let floor = floors
                        .into_iter()
                        .find_map(|(name, floor)| (name == requested).then_some((name, floor)));
                    let (name, floor) = floor.ok_or_else(|| {
                        miette::miette!("unknown kind `{requested}` (temper models: skill, rule)")
                    })?;
                    let contract = compose::effective(layer.as_ref(), name, floor)?;
                    schema::emit(&contract)
                }
                // No `--kind`: a JSON object mapping each modeled kind to its schema.
                None => {
                    let mut map = serde_json::Map::new();
                    for (name, floor) in floors {
                        let contract = compose::effective(layer.as_ref(), name, floor)?;
                        map.insert(name.to_string(), schema::emit(&contract));
                    }
                    serde_json::Value::Object(map)
                }
            };

            println!("{}", serde_json::to_string_pretty(&json).into_diagnostic()?);
            Ok(ExitCode::SUCCESS)
        }
        Command::Diff { harness_path, into } => {
            // Drift is a report, not a gate (`specs/20-surface.md`, CLI surface):
            // load the surface, compare it against the live harness, print the
            // four-state report, and exit zero regardless of what it finds. The
            // engine writes nothing — this is the read-only on-disk-vs-baseline
            // slice; `apply`/`re-add` own write-back.
            let ws = Workspace::load(&into)?;
            let report = drift::diff(&ws, &harness_path)?;
            print!("{}", drift::render(&report));
            Ok(ExitCode::SUCCESS)
        }
        Command::Apply { into, dry_run } => {
            // The write direction (`specs/20-surface.md`, "Drift / apply"): load the
            // surface + its lock, project it back onto the harness sources patching
            // only changed fields, and print the applied/unchanged/conflicted report.
            // `--dry-run` reports every outcome but writes nothing. Apply targets the
            // recorded provenance path per artifact — the destination is the source it
            // came from, so no harness root is re-supplied here (unlike `diff`, whose
            // harness arg drives its on-disk *rescan* for the "added" axis).
            let ws = Workspace::load(&into)?;
            let report = drift::apply(&ws, &into, drift::ApplyOptions { dry_run })?;
            if dry_run {
                println!("dry run — no files written");
            }
            print!("{}", drift::render_apply(&report));
            Ok(ExitCode::SUCCESS)
        }
        Command::ReAdd { harness_path, into } => {
            // The on-disk -> surface reconcile (`specs/20-surface.md`, "the surface
            // is the source of truth" — `re-add` keeps direct on-disk editing
            // first-class). Load the surface + its lock, pull every drifted / added
            // harness source back into the surface, refresh the lock, and print the
            // reconciled/added/unchanged report. A reconcile, not a gate: exit zero
            // regardless. Unlike `apply`, this re-scans the live harness (like
            // `diff`), so it takes the harness path as well as the surface.
            let ws = Workspace::load(&into)?;
            let report = drift::re_add(&ws, &into, &harness_path)?;
            print!("{}", drift::render_readd(&report));
            Ok(ExitCode::SUCCESS)
        }
        Command::SessionStart { harness_path } => {
            // The advisory session-start gate (`specs/50-distribution.md`,
            // "Decision: the session-start gate is advisory, not blocking"): a
            // one-shot check over a *harness* path. Import it internally into a
            // throwaway scratch surface, run the same by-kind gate `check` runs,
            // and emit the `claude-session-start` reporter payload on stdout for a
            // Claude Code `SessionStart` hook.
            //
            // Import-internally, not the author's two-step import-then-check: the
            // surface is scratch, and the author layer (the harness's `temper.toml`
            // custom kinds/roles) is read from the harness itself, not the process
            // CWD — so the gate judges the harness under check.
            //
            // Advisory: the run *always* exits zero. `SessionStart` cannot block,
            // and a failing contract routes through the human via the reporter's
            // notify-and-approve verdict, never a hard denial.
            let scratch = scratch_surface()?;
            import::run(&harness_path, &scratch)?;
            let diagnostics = gate(&scratch, &harness_path.join(TEMPER_TOML))?;
            // Best-effort cleanup of the scratch surface: a leftover temp dir must
            // never fail the advisory gate, so a removal error is swallowed.
            let _ = fs::remove_dir_all(&scratch);

            println!("{}", reporter::session_start(&diagnostics));
            Ok(ExitCode::SUCCESS)
        }
    }
}

/// Produce the merged diagnostic set for a surface `workspace` against the active
/// by-kind contracts — the shared gate logic behind both `check` and the
/// session-start reporter (`specs/10-contracts.md`, both greens; `specs/20-surface.md`,
/// "contract selection is by artifact kind"). Extracted verbatim from `check` so
/// the one-shot session-start path runs byte-identical checks; `check`'s own
/// behaviour is unchanged.
///
/// `temper_toml` is the author-declared layer's path: `check` passes the project
/// root's `temper.toml`, while the one-shot session-start gate passes the
/// harness's own, so the roster/graph/custom-kind tiers resolve relative to the
/// harness under check rather than the process CWD. Absent that file the layer is
/// `None` and the by-kind floor runs verbatim.
fn gate(workspace: &Path, temper_toml: &Path) -> miette::Result<Vec<check::Diagnostic>> {
    let ws = Workspace::load(workspace)?;

    // The optional author-declared layer beside the harness. Absent ⇒ `None` and
    // the floor runs verbatim (every existing test's path); present ⇒ it layers
    // over the by-kind floor per kind below (`specs/40-composition.md`, the
    // `temper.toml` Decision).
    let layer = compose::AuthorLayer::load(temper_toml)?;

    // Dispatch by artifact kind: each kind's features are validated against the
    // *effective* contract for its kind — the embedded floor with the author layer
    // applied — and the findings are merged into one diagnostic set
    // (`specs/20-surface.md`, "contract selection is by artifact kind"). The
    // generic engine holds no per-kind opinion — each contract carries its own
    // clauses, so a mixed harness (skills *and* rules) is judged correctly in one
    // run.
    let skill_features: Vec<extract::Features> =
        ws.skills.iter().map(extract::skill_features).collect();
    let skill_floor = Contract::parse(BUILTIN_SKILL_CONTRACT, Path::new("skill.anthropic.toml"))?;
    let skill_contract = compose::effective(layer.as_ref(), "skill", skill_floor)?;

    let rule_features: Vec<extract::Features> =
        ws.rules.iter().map(extract::rule_features).collect();
    let rule_floor = Contract::parse(BUILTIN_RULE_CONTRACT, Path::new("rule.toml"))?;
    let rule_contract = compose::effective(layer.as_ref(), "rule", rule_floor)?;

    // Two greens, not one (`specs/10-contracts.md`, both-greens finish line).
    // **Admissibility** first: each built-in contract is itself validated against
    // the definition before it is trusted to judge a harness — an inadmissible
    // contract fails the run exactly as a `required` conformance violation does.
    // **Conformance** second: each artifact is checked against the contract for
    // its kind. Both sets of findings merge into one rendered diagnostic stream.
    let mut diagnostics = engine::admissibility(&skill_contract);
    diagnostics.extend(engine::admissibility(&rule_contract));
    diagnostics.extend(engine::validate(&skill_contract, &skill_features));
    diagnostics.extend(engine::validate(&rule_contract, &rule_features));

    // The harness-contract tier: run role match-selection over the parsed roster,
    // gating each `required` single-filler role on being filled by exactly one
    // artifact of its kind (`specs/10-contracts.md`, "Roles and matching"). Absent
    // `temper.toml` ⇒ no layer ⇒ this adds nothing, so the floor-only path stays
    // byte-for-byte unchanged.
    if let Some(layer) = layer.as_ref() {
        let by_kind: std::collections::BTreeMap<&str, &[extract::Features]> =
            std::collections::BTreeMap::from([
                ("skill", skill_features.as_slice()),
                ("rule", rule_features.as_slice()),
            ]);
        let base_dir = temper_toml.parent().unwrap_or_else(|| Path::new("."));

        // Admissibility before conformance, here too: each role's own definition is
        // validated against the definition — its `match` selector resolves, a
        // `required` role's artifact kind is satisfiable, its contract resolves and
        // is itself admissible, and any `verified_by` resolves — before the roster
        // is trusted to judge the harness (`specs/10-contracts.md`, "Decision: the
        // contract is itself checked — admissibility").
        diagnostics.extend(roster::admissibility(layer.roles(), &by_kind, base_dir));

        // Selection: each `required` single-filler role is filled by exactly one
        // artifact of its kind (`specs/10-contracts.md`, "Roles and matching").
        diagnostics.extend(roster::check(layer.roles(), &by_kind, base_dir));

        // The `conforms-to` half of the same tier: each role's selected filler(s)
        // are validated against the role's resolved contract — its inline clauses,
        // or a template path taken relative to the `temper.toml` directory — with
        // findings retagged under `role.conforms-to` (`specs/10-contracts.md`, the
        // `role` primitive). A non-resolving template is admissibility's finding
        // above, skipped here rather than double-reported.
        diagnostics.extend(roster::conformance(layer.roles(), &by_kind, base_dir));

        // The graph scope: build the harness reference graph over the edges
        // declared as each kind's `[[kind.<name>.relationships]]` — a reference is a
        // kind capability, not a standalone construct (`specs/15-kinds.md`, "The
        // entity graph is a kind capability") — and check route resolution: a
        // declared reference (`routes_to: standards`) must resolve to a real
        // artifact of the target kind (`specs/45-governance.md`, "The harness is a
        // graph too"). Admissibility before conformance, here too: an edge that
        // names no reference field or targets an unmodeled kind is reported once and
        // skipped by the route check. Absent `temper.toml` ⇒ no layer ⇒ no
        // relationships ⇒ this adds nothing, so the floor-only path stays
        // byte-for-byte unchanged.
        diagnostics.extend(graph::admissibility(layer.edges(), &by_kind));
        diagnostics.extend(graph::check(layer.edges(), &by_kind));

        // The graph-scope `acyclic` predicate (`specs/45-governance.md`, "The graph
        // scope (the model)"): the resolved reference graph must contain no cycle —
        // a circular import loads nothing, so every finding is a true positive.
        // Intrinsic to the declared edges, so always-on over `layer.edges()` like
        // route resolution above; no `temper.toml` ⇒ no edges ⇒ this adds nothing,
        // so the floor-only path is unchanged.
        diagnostics.extend(graph::acyclic(layer.edges(), &by_kind));

        // The graph-scope `degree` predicate (`specs/45-governance.md`, "The graph
        // scope (the model)"; the worked example "self-registering vs routed"): a
        // role declares an in/out edge-count bound and every artifact its `match`
        // selects must have a degree inside it over the resolved reference arcs.
        // Declared at the set scope (on the role) but ranging over the edge graph,
        // so it takes the roles *and* the edges, reusing the arc resolution
        // `acyclic`/`check` assemble. Opt-in, per-role: a roster declaring no bound
        // does no graph work here, so the floor-only path stays byte-for-byte
        // unchanged.
        diagnostics.extend(graph::degree(layer.roles(), layer.edges(), &by_kind));

        // The custom-kind tier: each custom kind the layer declares
        // (`specs/15-kinds.md`, "A kind definition — one composed object") is
        // checked through its **own composed extractor** and **own contract** — the
        // same two greens the built-in kinds run above, but data-driven rather than
        // engine code. For each declared kind, project its imported units into raw
        // markdown units, run the composed extractor over each to yield features,
        // then extend the stream with admissibility over the kind's contract and
        // conformance over those features (`specs/15-kinds.md`, "Worked example:
        // `spec`, temper's own custom kind"). Absent a custom kind ⇒ the loop is
        // empty, so the built-in-only path is byte-for-byte unchanged.
        for (name, custom) in layer.custom_kinds() {
            let units = custom_units(workspace, custom)?;
            let features: Vec<extract::Features> = units
                .iter()
                .map(|unit| custom.extraction.extract(unit))
                .collect();
            let contract = Contract {
                name: name.clone(),
                clauses: custom.clauses.clone(),
            };
            diagnostics.extend(engine::admissibility(&contract));
            diagnostics.extend(engine::validate(&contract, &features));
        }
    }

    Ok(diagnostics)
}

/// Create a fresh throwaway surface directory for the one-shot session-start
/// import — a scratch workspace under the system temp dir, unique to this process,
/// that the caller removes once it has the diagnostics. Import-internally needs a
/// place to project the harness, but the session-start gate never persists a
/// surface (unlike the author's `import --into`), so it is torn down after use.
fn scratch_surface() -> miette::Result<PathBuf> {
    let dir = std::env::temp_dir().join(format!("temper-session-start-{}", std::process::id()));
    // A stale directory from a crashed prior run must not poison this import.
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).into_diagnostic()?;
    Ok(dir)
}

/// Load a custom `kind`'s units from the surface, generically — every surface
/// directory under the workspace at the kind's declared `governs.root`, each
/// reloaded into a raw [`Unit`] via [`Unit::from_surface_dir`]. Keyed on the
/// declared locus, never the kind name: temper reads its own `specs/` because its
/// `temper.toml` declares a kind rooted there, not because anything is hardwired to
/// `spec` — and a custom kind rooted anywhere else (`docs/adr`, …) is read the same
/// way, not just `specs/` (`specs/40-composition.md`, "Declaring a custom kind").
///
/// A surface directory is one holding a `meta.toml`, mirroring the built-in
/// [`Workspace::load`] enumeration, name-sorted so the diagnostic set is stable
/// across runs. A workspace with no directory at the kind's root contributes no
/// units — its contract's admissibility still runs, over zero artifacts.
fn custom_units(workspace_dir: &Path, kind: &compose::CustomKind) -> Result<Vec<Unit>, KindError> {
    let root = workspace_dir.join(&kind.governs.root);
    if !root.is_dir() {
        return Ok(Vec::new());
    }

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
        if path.is_dir() && path.join("meta.toml").is_file() {
            dirs.push(path);
        }
    }
    dirs.sort();

    dirs.iter().map(|dir| Unit::from_surface_dir(dir)).collect()
}
