# Plan state

- **Phase:** both greens hold locally — the contract engine ships **skill + rule**
  *conformance* and contract *admissibility*; `check` runs both passes (`main.rs`,
  `engine.rs::admissibility`). The next move is the `type` field primitive.
- **Last shipped:** ADMISSIBILITY (ce4ccf1) — `check` now validates each contract
  against the definition (empty-list clauses inadmissible), surfacing the second
  green. Verified on disk: `extract.rs` still stringifies every scalar
  (`json_scalar_string` flattens bool/number/string), so `type` is undecidable until
  the extractor preserves source kind.
- **In flight:** nothing; tree clean.
- **Next (filed, fork-free — `(field-type-lattice)` RESOLVED):** TYPED-EXTRACTION
  (preserve each field's parsed kind in the projection — the precondition), then
  TYPE-PRIMITIVE (add `Predicate::Type` over the closed lattice), which it blocks.
- **Frontier (fork-free, unfiled — for follow-on plan ticks):** the harness-contract
  layer (`temper.toml` + roles + `verified_by`, 40-composition); `temper schema`; the
  advisory session-start gate + `claude-session-start` reporter; the plugin/`bundle`
  tree; GitHub/SARIF reporters. **Still fork-blocked:** `apply`/`install`
  (`(yaml-writeback)`, `(workspace-scope)`); the declared model + dependency graph +
  cross-landscape seam (`(model-declaration-format)`); full `pattern` (`(regex-crate)`);
  the skill referential clause (`(skill-ref-syntax)`).

Plan continues: no — the queue is reconciled (both type-pair entries verified
unshipped and accurate; ADMISSIBILITY already dropped), the inbox is empty, and a
pickable `open` entry (TYPED-EXTRACTION) exists. Build runs; it drains the queue.
The harness-contract / schema / distribution frontier is fork-free but deferred to
follow-on plan ticks so each is decomposed into small disjoint entries once the type
pair lands — filing that chain now would be speculative over-filing.
