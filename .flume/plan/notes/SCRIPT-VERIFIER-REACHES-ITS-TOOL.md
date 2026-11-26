# Script verifier reaches its tool: engine needed no change

`script(path, { tool })` now names the tool that runs a script verifier. The
SDK's `runsRows` emits a `root/requirement/<name>` runs row for it, the same
row a telemetry tap's tool rides. The engine's `graph::root_runs_edges`
already lifted every requirement runner generically
(A-TAP-REACHES-ITS-TOOL-BY-AN-EDGE), so `src/graph.rs` is untouched and
`src/drift.rs` only gets a doc-comment line.

The entry predicted a verifier-tool column at drift.rs:7253
(`verifier_table`). I left it out on purpose. The tap keeps its tool off
the lock's verifier row and on the runs row alone, and a second carrier would
be a duplicate surface. If plan wants the tool on the verifier row (say, for
explain's "Held by script" line), that is a separate decision.
