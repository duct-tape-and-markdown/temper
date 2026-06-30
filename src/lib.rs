//! `author` — a typed maintenance surface for the Claude Code harness.
//!
//! `author` imports the whole Claude Code harness (skills, commands, agents,
//! hooks, MCP/LSP servers, `CLAUDE.md` rules, plugin & marketplace manifests,
//! settings) into a single typed, validated config surface, lets a human
//! reorganize it, lints it against the documented schemas + best practices,
//! composes artifacts into publishable bundles, and writes changes back to disk
//! with drift-aware, dry-runnable `apply`.
//!
//! See `SPEC.md` for the full design and `spec/RELEASE-v0.1.md` for the active
//! ship target. This crate is built tick-by-tick by the flume harness in
//! `.flume/`; modules below are filled in per pending entry.

/// Crate version, surfaced by the CLI's `--version`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod check;
pub mod contract;
pub mod import;
pub mod rules;
pub mod skill;
