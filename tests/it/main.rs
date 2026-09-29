//! The one integration-test crate. Cargo compiles this file into a single test
//! binary and every suite below is a module of it, rather than each suite being
//! a target that relinks the shared `common` scaffolding into a copy of its own
//! (specs/process/engineering.md, "One job, one home" — scaffolding lives in one
//! home, and one home in `target/` is what a single crate buys).
//!
//! A suite that is not declared here still compiles as source and runs nothing:
//! its assertions leave the suite's test count without failing anything.
//! [`every_suite_beside_this_file_is_declared_here`] is the pin that refuses
//! that silence.

mod common;

mod acceptance;
mod agent_kind;
mod builtin_contract_matrix;
mod builtin_lock_frozen;
mod bundle;
mod check_cost;
mod cli;
mod closed_keys;
mod command_kind;
mod contract_template;
mod coverage;
mod coverage_note;
mod declared_input;
mod dial_kind;
mod directive_classing;
mod display_rule;
mod emit;
mod extent;
mod extract_equivalence;
mod field_addressing;
mod gate_fail_loud;
mod gauntlet;
mod graph;
mod hook_kind;
mod install;
mod installed_plugin_kind;
mod json_document_format;
mod known_marketplace_kind;
mod layer_join;
mod layout_edge_slot;
mod layout_kind;
mod layout_prose_import;
mod local_locus;
mod lock_declaration_rows;
mod manifest_adapter;
mod manifest_schema_oracle;
mod marketplace_kind;
mod mcp_kind;
mod mcp_server_kind;
mod memory_contract;
mod memory_gate;
mod nested_member;
mod plugin_manifest_kind;
mod projection_path_seam;
mod prose_include;
mod read_verbs;
mod registration_locus;
mod reporters;
mod requirement_roster;
mod root_contract;
mod rule_contract;
mod seam_bindings_current;
mod section_contains;
mod session_start;
mod settings_kind;
mod settings_local_kind;
mod shape_predicate;
mod tap;
mod toml_document;
mod type_predicate;

/// Every `tests/it/*.rs` suite beside this file is declared as a module above.
/// An undeclared suite is a suite that stopped running: nothing compiles it, so
/// no assertion in it can fail, and the only trace is a test count that quietly
/// shrank.
#[test]
fn every_suite_beside_this_file_is_declared_here() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/it");
    let mut on_disk: Vec<String> = std::fs::read_dir(&dir)
        .expect("the directory this file lives in is readable")
        .map(|entry| entry.expect("a readable directory entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "rs"))
        .filter_map(|path| path.file_stem().map(|s| s.to_string_lossy().into_owned()))
        .filter(|stem| stem != "main")
        .collect();
    on_disk.sort();

    // Vacuity pin (engineering.md, "A green verdict is proven non-vacuous"): a
    // mis-resolved directory yields an empty set, which would compare equal to
    // an empty module list and pass over zero suites.
    assert!(
        !on_disk.is_empty(),
        "no suites found beside main.rs in {} — the judged set collapsed to zero",
        dir.display()
    );

    let mut declared: Vec<String> = include_str!("main.rs")
        .lines()
        .filter_map(|line| line.strip_prefix("mod ")?.strip_suffix(';'))
        .map(str::to_owned)
        .filter(|name| name != "common")
        .collect();
    declared.sort();

    assert_eq!(
        declared, on_disk,
        "the module list in tests/it/main.rs and the suites on disk disagree; \
         declare every suite so it runs"
    );
}
