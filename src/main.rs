//! `author` CLI entry point.
//!
//! Thin command dispatch over the [`author`] library. The subcommands mirror the
//! slice-1 surface in `spec/RELEASE-v0.1.md` ("Surface"): `import` scans a harness
//! into the typed config surface, `check` lints that surface and exits non-zero
//! when any `error`-severity diagnostic fires. All logic lives in the library —
//! `main` only parses args, registers the rule set, and maps the result to an
//! exit code (the one place rule *registration* lives, keeping the engine
//! disjoint from the rules it runs).

use std::path::PathBuf;
use std::process::ExitCode;

use author::check::{self, Workspace};
use author::import;
use author::rules;
use clap::{Parser, Subcommand};

/// The surface workspace default for `--into` / the `check` argument: a `.author`
/// directory under the current working directory (`spec/RELEASE-v0.1.md`).
const DEFAULT_WORKSPACE: &str = "./.author";

/// A typed maintenance surface for the Claude Code harness.
#[derive(Parser)]
#[command(name = "author", version, about, long_about = None)]
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
        /// Where to write the surface workspace (defaults to `./.author`).
        #[arg(long, default_value = DEFAULT_WORKSPACE)]
        into: PathBuf,
    },
    /// Lint the config surface against schemas + best practices.
    Check {
        /// The surface workspace to lint (defaults to `./.author`).
        workspace: Option<PathBuf>,
    },
}

fn main() -> miette::Result<ExitCode> {
    match Cli::parse().command {
        Command::Import { harness_path, into } => {
            import::run(&harness_path, &into)?;
            Ok(ExitCode::SUCCESS)
        }
        Command::Check { workspace } => {
            let workspace = workspace.unwrap_or_else(|| PathBuf::from(DEFAULT_WORKSPACE));
            let ws = Workspace::load(&workspace)?;
            let diagnostics = check::run(&ws, &rules::all_rules());
            print!("{}", check::render(&diagnostics));
            // Any error-severity finding fails the run; warn-only is clean.
            Ok(if check::any_error(&diagnostics) {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            })
        }
    }
}
