# Plan state

- Spec derived through: 663e03f
- Audited through: 60faee0
- Residue swept through: 60faee0
- Posture swept through: drift done — cli-engine next
- This tick: SPEC DELTA. Routed 663e03f (specs: architecture.md, the
  codemap). Job 1 (inbox/captures) confirmed empty first. The doc's own
  narrative names one actionable slice: "Two edges in today's tree stand
  in tension with this map... each a fileable gap" — `drift → install`
  and `frontmatter → builtin_kind` — but rules its own resolution ("move a
  shared helper... or re-draw this map... the entry's design question").
  Verified both on disk rather than filing the reported shape blind:
  `drift → install` holds as described (`src/drift.rs:30` imports
  `crate::install`, `place` fn 2034 calls `install::placement_lines`,
  doc comment 2603 names `install::matches_projection`).
  `frontmatter → builtin_kind` holds narrower than described: the only
  reference (`src/frontmatter.rs:469,473,477`) sits inside
  `#[cfg(test)] mod tests` (467) — the adapter's fixtures borrowing real
  `Kind` values — production `frontmatter.rs` (1-465) has no such import.
  Since either edge's resolution needs a design call the page itself
  defers ("a better argument"), not a call plan may invent, both route to
  open-questions as `(drift-install-edge)` and
  `(frontmatter-builtin-kind-edge)`, each carrying the verified evidence
  and both resolution paths, not a pending entry. Rest of the doc (flat-
  tree policy, growth rules, the codemap listing itself) is narrative/
  process, nothing else actionable. No pending.json change this tick.
- Queue: 5 pending — 2 pickable OPEN (DISCOVERY-INFALLIBLE-RESULT-COLLAPSE,
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION; disjoint files, no serialization
  needed), 1 blockedBy DRIFT-LOCK-ROW-WALK-CONSOLIDATION
  (DRIFT-EMIT-LOCK-PARSE-HOIST; shares src/drift.rs, correctly serialized),
  2 parked on human action (IMPORT-HOP-CAP-CITE,
  PACKAGING-CHANNELS-REMAINDER). Open forks: (multi-harness-projection),
  (lazy-grounds), (drift-install-edge), (frontmatter-builtin-kind-edge).

Plan continues: yes — posture sweep continues to the roster's next
subsystem, `cli-engine` (main+engine).
