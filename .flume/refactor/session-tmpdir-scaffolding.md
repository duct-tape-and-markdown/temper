## Surface

One `fn tmpdir(label: &str) -> PathBuf` builder (counter + pid + label temp
dir), byte-identical modulo the label prefix, copy-pasted ~28 times: 20
integration-test files (tests/{acceptance, agent_kind, builtin_lock_frozen,
bundle, cli, command_kind, coverage, coverage_note, emit,
extract_equivalence, gate_fail_loud, graph, install, lock_declaration_rows,
memory_contract, memory_gate, read_verbs, reporters, requirement_roster,
session_start}.rs) plus 8 `#[cfg(test)]` copies (src/builtin_kind.rs:484,
src/bundle.rs:336, src/frontmatter.rs:567, src/check.rs:346,
src/import.rs:382, src/coverage_note.rs:419, src/install.rs:1544,
src/drift.rs:1792). `fixture()` likewise duplicated in tests/acceptance.rs:75
and tests/extract_equivalence.rs:33. No `tests/common` module exists.

## Observed at

0ccba8d

## Suggested consolidation

`tests/common/mod.rs` as the one home; prefer replacing the hand-rolled
builder with the sanctioned `tempfile` dev-dependency (RAII cleanup
subsumes the counter/pid scheme). The in-src test modules use the same
dev-dep directly.
