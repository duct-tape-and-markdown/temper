//! `temper` — a type system for the documents that program agents, the Claude Code
//! harness first.
//!
//! The harness is authored as a typed program, compiled by a deterministic `emit` with
//! drift routed to the authored source, and gated by `check` against a declared
//! contract. The `specs/` corpus is the source of truth this code is reconciled against.

#![deny(rustdoc::broken_intra_doc_links)]

/// Crate version, surfaced by the CLI's `--version`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Directory name of the surface workspace — the authored SDK modules and the
/// committed lock — that `install` scaffolds beside the harness root it governs.
/// `pub`, not `pub(crate)`: the CLI binary is a separate crate consuming this lib.
pub const WORKSPACE_DIR: &str = ".temper";

/// Default workspace path with `./` prefix — the `emit` workspace default and the
/// path `schema` / `explain` / `bundle` read the committed lock from.
pub fn default_workspace() -> String {
    format!("./{}", WORKSPACE_DIR)
}

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
pub(crate) mod fs_util;
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
pub mod member_address;
pub mod path;
pub mod placement;
pub mod read;
pub mod reporter;
pub mod roster;
pub mod schema;
pub mod tap;
pub mod telemetry;
#[cfg(test)]
pub(crate) mod test_support;
pub mod toml_document;
pub mod tool;
