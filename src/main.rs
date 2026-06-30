//! `author` CLI entry point.
//!
//! Thin command dispatch over the [`author`] library. Subcommands mirror the
//! verbs in `SPEC.md §6`; they are stubbed here and implemented tick-by-tick by
//! the flume build phase against `spec/RELEASE-v0.1.md`.

use clap::{Parser, Subcommand};

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
    Import,
    /// Lint the config surface against schemas + best practices.
    Check,
}

fn main() -> miette::Result<()> {
    match Cli::parse().command {
        Command::Import => println!("author import: not yet implemented"),
        Command::Check => println!("author check: not yet implemented"),
    }
    Ok(())
}
