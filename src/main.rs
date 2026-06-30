//! `temper` CLI entry point.
//!
//! Thin command dispatch over the [`temper`] library. The subcommands mirror the
//! surface in `specs/20-surface.md` ("CLI surface"): `import` scans a harness
//! into the typed config surface, `check` validates that surface against the
//! active contract and exits non-zero when a `required`-severity clause is
//! violated. All logic lives in the library — `main` only parses args, projects
//! the workspace into the engine's [`Features`] view, runs the generic contract
//! engine (`specs/10-contracts.md`), and maps the result to an exit code.
//!
//! [`Features`]: temper::extract::Features

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use temper::check::{self, Severity, Workspace};
use temper::contract::Contract;
use temper::engine;
use temper::extract;
use temper::import;

/// The surface workspace default for `--into` / the `check` argument: a `.temper`
/// directory under the current working directory (`specs/20-surface.md`).
const DEFAULT_WORKSPACE: &str = "./.temper";

/// The built-in Anthropic skill contract — the curated "std-lib" default
/// (`contracts/skill.anthropic.toml`), embedded at build time so `check` has a
/// contract to validate against without any on-disk configuration.
const BUILTIN_SKILL_CONTRACT: &str = include_str!("../contracts/skill.anthropic.toml");

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

            // Project every skill into the engine's feature view, then validate
            // the whole set against the built-in contract — the generic engine
            // holds no per-rule opinion; the contract carries the clauses.
            let features: Vec<extract::Features> =
                ws.skills.iter().map(extract::skill_features).collect();
            let contract =
                Contract::parse(BUILTIN_SKILL_CONTRACT, Path::new("skill.anthropic.toml"))?;
            let diagnostics = engine::validate(&contract, &features);
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
    }
}
