# Plan state

- **Phase:** reconcile. Verified on disk this tick: KIND-EXTRACTION-ALGEBRA
  **shipped** — `src/kind.rs` is the closed extraction algebra as data
  (`Extraction`/`Primitive`: `field`, `headings`, `line_count`, `placement`,
  `references`), `from_table` public for compose to reuse, unknown-primitive a
  load error, full unit tests. Also confirmed still un-shipped: `src/compose.rs`
  carries only the adopt/clause `KindLayer` (no `governs`/`extraction`), and
  `src/import.rs` still hardwires `discover_spec_files`/`import_spec`.
- **Last shipped:** KIND-EXTRACTION-ALGEBRA (`9984208`). Tree clean.
- **In flight / next:** the kind-declaration chain, one linear serialized run
  (shared files compose/import/main/extract/lib/check/drift). **KIND-DECLARATION-PARSE
  now `open`, pickable** (its blocker shipped; the stale `blockedBy` is cleared) →
  KIND-IMPORT-DISCOVERY → KIND-CHECK-SPEC → KIND-RETIRE-BUILTIN-SPEC →
  KIND-EDGE-RELATIONSHIPS, each `blockedBy` its predecessor so exactly one is
  pickable at a time.
- **Frontier:** after the chain lands, `degree`/`acyclic` (`45-governance.md`) read
  the same relationships; the spec kind's decisions-name-alternatives clause waits on
  `(decision-marker-predicate)`, references-resolve on KIND-EDGE-RELATIONSHIPS.
- **Inbox:** empty. Open-questions unchanged.

Plan continues: no — chain head unblocked and pickable, tail serialized, inbox
empty; hand to build.
