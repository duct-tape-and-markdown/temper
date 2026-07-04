//! Interchange schema emission — the manifest IR projected Rust-first into a JSON
//! Schema (2020-12) and a TypeScript type module, checked into `contract/`.
//!
//! Distinct from the editor schema its parent [`crate::schema`] emits: that projects
//! one *contract* into a keystroke squiggle over an artifact's frontmatter; this
//! projects the *manifest IR* — [`ManifestMember`] and the [`Features`] it carries —
//! into the interchange contract both implementations version against
//! (`specs/architecture/50-distribution.md`, "versioned against the interchange schema…
//! both implementations tested against one golden set"). The Rust types are the single
//! source of truth: [`schemars`] derives the JSON Schema, [`ts_rs`] the TypeScript, so
//! the two never hand-drift from the IR the gate actually reads.
//!
//! The generated schema describes the **typed IR** (nested `ManifestMember { kind,
//! features }`, Rust field names, the tagged [`FeatureValue`]), not the flattened TOML
//! wire the byte-parity goldens pin — the goldens are the byte contract, this is the
//! type contract, and both live under `contract/` as one versioned set.

use std::io;
use std::path::Path;

use ts_rs::{Config, TS};

use crate::compose::ManifestMember;
use crate::document::PublishedRequirement;
use crate::extract::{FeatureValue, Features, FencedBlock, GenreValue, Kind, Section};

/// The manifest interchange **JSON Schema** (2020-12), pretty-printed with a trailing
/// newline — the schemars projection of [`ManifestMember`], the IR root every carriage
/// serializes into. Deterministic: schemars orders a struct's properties by declaration
/// and its definitions by name, so a re-emit is byte-identical.
#[must_use]
pub fn manifest_schema_json() -> String {
    let schema = schemars::schema_for!(ManifestMember);
    // `schema_for!` yields a `schemars::Schema` that serializes to the JSON Schema
    // document; pretty-print it so the committed artifact reviews cleanly in a diff.
    let mut out =
        serde_json::to_string_pretty(&schema).expect("a schemars Schema always serializes to JSON");
    out.push('\n');
    out
}

/// The manifest interchange **TypeScript** type module — the ts-rs declarations of the
/// IR type graph, in dependency order (leaves first) so each `type X = …` references
/// only names already declared above it. A single self-contained module: `decl()`
/// emits no `import` lines, so concatenation is the whole file.
#[must_use]
pub fn manifest_types_ts() -> String {
    let cfg = Config::new();
    // Leaves first so every referenced name is declared before its use; `ManifestMember`
    // (the root) last. `decl()` returns a bare `type X = …;`, so `export` is prepended to
    // make the module's types importable.
    let decls = [
        Kind::decl(&cfg),
        FeatureValue::decl(&cfg),
        Section::decl(&cfg),
        FencedBlock::decl(&cfg),
        GenreValue::decl(&cfg),
        PublishedRequirement::decl(&cfg),
        Features::decl(&cfg),
        ManifestMember::decl(&cfg),
    ];
    let mut out = String::from(
        "// Generated Rust-first from the manifest IR (ts-rs). Do not edit by hand —\n\
         // regenerate with `cargo test contract` under `BLESS=1` (tests/contract_fixtures.rs).\n\n",
    );
    for decl in decls {
        out.push_str("export ");
        out.push_str(&decl);
        out.push('\n');
    }
    out
}

/// Where the two interchange artifacts land under a `contract/` directory: the JSON
/// Schema and the TypeScript module, side by side so the versioned set travels together.
pub const SCHEMA_JSON_PATH: &str = "schema/manifest.schema.json";
/// The TypeScript type module's path under `contract/` (see [`SCHEMA_JSON_PATH`]).
pub const TYPES_TS_PATH: &str = "schema/manifest.d.ts";

/// Write both interchange artifacts under `contract_dir` — the emission path the
/// regeneration test drives (and a `temper` verb could wire later). Creates the
/// `schema/` subdirectory, then writes the JSON Schema and the TypeScript module.
///
/// # Errors
///
/// Propagates any I/O error creating the directory or writing either file.
pub fn write_interchange(contract_dir: &Path) -> io::Result<()> {
    let schema_dir = contract_dir.join("schema");
    std::fs::create_dir_all(&schema_dir)?;
    std::fs::write(contract_dir.join(SCHEMA_JSON_PATH), manifest_schema_json())?;
    std::fs::write(contract_dir.join(TYPES_TS_PATH), manifest_types_ts())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_json_schema_is_2020_12_and_deterministic() {
        let json = manifest_schema_json();
        // The Rust-first projection targets JSON Schema 2020-12 (the entry's contract).
        assert!(json.contains("https://json-schema.org/draft/2020-12/schema"));
        // The IR root and a carried leaf both surface, so the whole graph projected.
        assert!(json.contains("ManifestMember"));
        assert!(json.contains("Features"));
        // Re-emit is byte-identical — schemars orders properties/definitions stably.
        assert_eq!(json, manifest_schema_json());
    }

    #[test]
    fn the_ts_module_declares_the_ir_graph_and_is_deterministic() {
        let ts = manifest_types_ts();
        for name in [
            "ManifestMember",
            "Features",
            "FeatureValue",
            "Section",
            "FencedBlock",
            "GenreValue",
            "PublishedRequirement",
            "Kind",
        ] {
            assert!(ts.contains(name), "TS module is missing `{name}`");
        }
        assert_eq!(ts, manifest_types_ts());
    }
}
