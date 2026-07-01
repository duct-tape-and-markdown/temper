//! `temper` — a typed maintenance surface for the Claude Code harness.
//!
//! `temper` imports the whole Claude Code harness (skills, commands, agents,
//! hooks, MCP/LSP servers, `CLAUDE.md` rules, plugin & marketplace manifests,
//! settings) into a single typed, validated config surface, lets a human
//! reorganize it, lints it against the documented schemas + best practices,
//! composes artifacts into publishable bundles, and writes changes back to disk
//! with drift-aware, dry-runnable `apply`.
//!
//! See the evergreen `specs/` corpus for the full design — `specs/00-intent.md`
//! is the north star, continuously reconciled against this code (there is no
//! frozen release line). This crate is built tick-by-tick by the flume harness
//! in `.flume/`; modules below are filled in per pending entry.

/// Crate version, surfaced by the CLI's `--version`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod check;
pub mod compose;
pub mod contract;
pub mod drift;
pub mod engine;
pub mod extract;
pub mod graph;
pub mod import;
pub mod kind;
pub mod roster;
pub mod rule;
pub mod schema;
pub mod skill;
