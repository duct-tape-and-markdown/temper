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
use temper::check::{self, Severity, Workspace};
use temper::compose;
use temper::contract::Contract;
use temper::drift;
use temper::engine;
use temper::extract;
use temper::graph;
use temper::import;
use temper::kind::{KindError, Unit};
use temper::roster;

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
    /// Report on-disk drift of a harness against the surface's import baseline.
    Diff {
        /// The harness to re-scan and compare against the import baseline.
        harness_path: PathBuf,
        /// The surface workspace holding the baseline (defaults to `./.temper`).
        #[arg(long, default_value = DEFAULT_WORKSPACE)]
        into: PathBuf,
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
            let ws = Workspace::load(&workspace)?;

            // The optional author-declared layer at the project root. Absent ⇒
            // `None` and the floor runs verbatim (every existing test's path);
            // present ⇒ it layers over the by-kind floor per kind below
            // (`specs/40-composition.md`, the `temper.toml` Decision).
            let layer = compose::AuthorLayer::load(Path::new(TEMPER_TOML))?;

            // Dispatch by artifact kind: each kind's features are validated
            // against the *effective* contract for its kind — the embedded floor
            // with the author layer applied — and the findings are merged into one
            // diagnostic set (`specs/20-surface.md`, "contract selection is by
            // artifact kind"). The generic engine holds no per-kind opinion — each
            // contract carries its own clauses, so a mixed harness (skills *and*
            // rules) is judged correctly in one run.
            let skill_features: Vec<extract::Features> =
                ws.skills.iter().map(extract::skill_features).collect();
            let skill_floor =
                Contract::parse(BUILTIN_SKILL_CONTRACT, Path::new("skill.anthropic.toml"))?;
            let skill_contract = compose::effective(layer.as_ref(), "skill", skill_floor)?;

            let rule_features: Vec<extract::Features> =
                ws.rules.iter().map(extract::rule_features).collect();
            let rule_floor = Contract::parse(BUILTIN_RULE_CONTRACT, Path::new("rule.toml"))?;
            let rule_contract = compose::effective(layer.as_ref(), "rule", rule_floor)?;

            // Two greens, not one (`specs/10-contracts.md`, both-greens finish
            // line). **Admissibility** first: each built-in contract is itself
            // validated against the definition before it is trusted to judge a
            // harness — an inadmissible contract fails the run exactly as a
            // `required` conformance violation does. **Conformance** second: each
            // artifact is checked against the contract for its kind. Both sets of
            // findings merge into one rendered diagnostic stream.
            let mut diagnostics = engine::admissibility(&skill_contract);
            diagnostics.extend(engine::admissibility(&rule_contract));
            diagnostics.extend(engine::validate(&skill_contract, &skill_features));
            diagnostics.extend(engine::validate(&rule_contract, &rule_features));

            // The harness-contract tier: run role match-selection over the parsed
            // roster, gating each `required` single-filler role on being filled by
            // exactly one artifact of its kind (`specs/10-contracts.md`, "Roles and
            // matching"). Absent `temper.toml` ⇒ no layer ⇒ this adds nothing, so
            // the floor-only path stays byte-for-byte unchanged.
            if let Some(layer) = layer.as_ref() {
                let by_kind: std::collections::BTreeMap<&str, &[extract::Features]> =
                    std::collections::BTreeMap::from([
                        ("skill", skill_features.as_slice()),
                        ("rule", rule_features.as_slice()),
                    ]);
                let base_dir = Path::new(TEMPER_TOML)
                    .parent()
                    .unwrap_or_else(|| Path::new("."));

                // Admissibility before conformance, here too: each role's own
                // definition is validated against the definition — its `match`
                // selector resolves, a `required` role's artifact kind is
                // satisfiable, its contract resolves and is itself admissible, and
                // any `verified_by` resolves — before the roster is trusted to
                // judge the harness (`specs/10-contracts.md`, "Decision: the
                // contract is itself checked — admissibility").
                diagnostics.extend(roster::admissibility(layer.roles(), &by_kind, base_dir));

                // Selection: each `required` single-filler role is filled by
                // exactly one artifact of its kind (`specs/10-contracts.md`, "Roles
                // and matching").
                diagnostics.extend(roster::check(layer.roles(), &by_kind, base_dir));

                // The `conforms-to` half of the same tier: each role's selected
                // filler(s) are validated against the role's resolved contract —
                // its inline clauses, or a template path taken relative to the
                // `temper.toml` directory — with findings retagged under
                // `role.conforms-to` (`specs/10-contracts.md`, the `role`
                // primitive). A non-resolving template is admissibility's finding
                // above, skipped here rather than double-reported.
                diagnostics.extend(roster::conformance(layer.roles(), &by_kind, base_dir));

                // The graph scope: build the harness reference graph over the
                // declared edges and check route resolution — a declared reference
                // (`routes_to: standards`) must resolve to a real artifact of the
                // target kind (`specs/45-governance.md`, "The harness is a graph
                // too"). Admissibility before conformance, here too: an edge that
                // names no reference field or targets an unmodeled kind is reported
                // once and skipped by the route check. Absent `temper.toml` ⇒ no
                // layer ⇒ no edges ⇒ this adds nothing, so the floor-only path stays
                // byte-for-byte unchanged.
                diagnostics.extend(graph::admissibility(layer.edges(), &by_kind));
                diagnostics.extend(graph::check(layer.edges(), &by_kind));

                // The custom-kind tier: each custom kind the layer declares
                // (`specs/15-kinds.md`, "A kind definition — one composed object")
                // is checked through its **own composed extractor** and **own
                // contract** — the same two greens the built-in kinds run above, but
                // data-driven rather than engine code. For each declared kind,
                // project its imported units into raw markdown units, run the
                // composed extractor over each to yield features, then extend the
                // stream with admissibility over the kind's contract and conformance
                // over those features (`specs/15-kinds.md`, "Worked example: `spec`,
                // temper's own custom kind"). Absent a custom kind ⇒ the loop is
                // empty, so the built-in-only path is byte-for-byte unchanged.
                for (name, custom) in layer.custom_kinds() {
                    let units = custom_units(&workspace, custom)?;
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
    }
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
