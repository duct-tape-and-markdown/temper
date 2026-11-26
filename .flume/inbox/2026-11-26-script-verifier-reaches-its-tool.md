# A script verifier reaches its tool by an edge (0104)

`specs/builtins.md`, "Tool" (marked *(pending)*): script verifiers reach the tool through edges, as gate hooks, taps and the typecheck already do. Today a script verifier is `{ species: "script", path }` with no tool (`sdk/src/contract.ts:446`, `:471`; `src/drift.rs:7253`). It gains a `tool` reference, the `{ file }` or `{ package, bin }` tool that runs it, and that reference is an edge on the graph, so the tool's advisory `degree` counts it. Acceptance:
- **new:** a requirement whose script verifier names a `{ file }` tool leaves that tool with no `tool.degree` finding;
- **kept:** a verifier without a tool is still accepted.
