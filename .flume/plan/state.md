# Plan state

- **Phase:** reconcile. Verified on disk this tick: GOV-GRAPH **shipped** —
  `src/graph.rs` runs route resolution + admissibility over `AuthorLayer::edges`
  (declared reference fields, never prose-grep). Also confirmed on disk: the
  `spec` kind still ships as a **built-in** — `src/spec.rs`, `extract::spec_features`,
  `import::discover_spec_files`/`import_spec`, `Workspace.specs`, and drift's spec
  axis — which `15-kinds.md`'s kind-declaration mechanism (a custom kind is declared
  data, never engine code) now supersedes.
- **Last shipped:** GOV-GRAPH (`dd2ca3a`). Tree clean.
- **Queue:** the kind-declaration reconciliation, filed as one linear chain (shared
  files compose/import/main/extract/lib/check/drift ⇒ *serialized*, not parallel).
  **KIND-EXTRACTION-ALGEBRA `open`, pickable** (new `src/kind.rs` — the extraction
  algebra as data) → KIND-DECLARATION-PARSE → KIND-IMPORT-DISCOVERY → KIND-CHECK-SPEC
  → KIND-RETIRE-BUILTIN-SPEC → KIND-EDGE-RELATIONSHIPS, each `blockedBy` its
  predecessor so exactly one is pickable at a time. SPEC-KIND-GATE **dropped** — its
  embedded-`contracts/spec.toml` shape is superseded, not parked.
- **Frontier:** after the chain lands, `degree`/`acyclic` (`45-governance.md`) read
  the same relationships; the spec kind's decisions-name-alternatives clause waits on
  `(decision-marker-predicate)`, references-resolve on KIND-EDGE-RELATIONSHIPS.
- **Inbox:** drained. Open-questions: `(rollup-index-rename)` added (the `author.toml`
  rename, a human naming decision); `(spec-landscape-kind)` marked superseded;
  `(model-declaration-format)`'s format now carried by `[kind.<name>]`.

Plan continues: no — reconciliation done, SPEC-KIND-GATE dropped, the mechanism
chain filed with one pickable head, inbox drained. Hand to build.
