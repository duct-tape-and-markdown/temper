//! `temper` — a typed maintenance surface for the Claude Code harness.
//!
//! `temper` imports the whole Claude Code harness (skills, commands, agents,
//! hooks, MCP/LSP servers, `CLAUDE.md` rules, plugin & marketplace manifests,
//! settings) into a single typed, validated config surface, lets a human
//! reorganize it, lints it against the documented schemas + best practices,
//! composes artifacts into publishable bundles, and writes changes back to disk
//! with drift-aware, dry-runnable `emit`.
//!
//! The evergreen `specs/` corpus is the north star, continuously reconciled
//! against this code (there is no
//! frozen release line). This crate is built tick-by-tick by the flume harness
//! in `.flume/`; modules below are filled in per pending entry.

/// Crate version, surfaced by the CLI's `--version`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Directory name of the surface workspace — the authored SDK modules and the
/// committed lock — that `install` scaffolds beside the harness root it governs.
/// `pub`, not `pub(crate)`: the CLI binary is a separate crate consuming this lib.
pub const WORKSPACE_DIR: &str = ".temper";

/// Filename of the generated roll-up index — the contents' state-of-record —
/// written at the workspace root.
pub const LOCK_FILENAME: &str = "lock.toml";

pub mod address;
pub mod admissibility;
pub mod builtin;
pub mod builtin_kind;
pub mod builtin_lock;
pub mod bundle;
pub mod check;
pub mod compose;
pub mod contract;
pub mod coverage;
pub mod coverage_note;
pub mod dial;
pub mod display;
pub mod document;
pub mod drift;
pub mod engine;
pub mod extract;
pub mod frontmatter;
pub mod gate;
pub mod glob;
pub mod graph;
pub(crate) mod hash;
pub mod import;
pub mod install;
pub mod json_manifest;
pub(crate) mod json_splice;
pub mod kind;
pub mod layout;
pub mod path;
pub(crate) mod placement;
pub mod read;
pub mod reporter;
pub mod roster;
pub mod schema;
pub mod tap;
pub mod telemetry;
#[cfg(test)]
pub(crate) mod test_support;
pub mod toml_document;
