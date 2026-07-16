# Plan state

- Spec derived through: a571973
- Audited through: 8978596
- Residue swept through: 8978596
- This tick: SPEC DELTA — derived 0029 (f6fe385, an edge declares its target
  set) into one entry, **EDGE-TARGET-SET** (`per` contract.md/"edge"). One
  entry, not a chain: the `to` widening is a single seam crossing — the SDK
  type, the lock column, and the engine's normalizer must spell one shape or
  the lock is malformed, so splitting SDK from engine would ship a
  half-widened wire. 0029's Consequences, every bullet: (1) "`model/
  contract.md`'s 'edge' field bullet carries the set and the addressing rule
  — same commit" → verified moot on disk, contract.md:11-18 carries both the
  non-empty set and the one-element/multi-element addressing rule; (2) "the
  SDK's `EdgeField.to` widens to the set type" → the entry, naming
  kind.ts:46 and the `readonly [string, ...string[]]` non-empty spelling —
  0029 forecloses the `string | readonly string[]` union that would have
  made the migration free; (3) "the engine's shipped normalizer
  (`src/graph.rs` `target_identity`) extends over the set" → same entry,
  graph.rs:1102 named as the one home with both callers (`check` 123-140,
  `resolved_edges` 803-822) and both admissibility sites (174-180, 1077)
  scoped; (4) "the fork record deletes with this record's commit" → verified
  moot, `(edge-field-target-openness)` absent from open-questions.md. Two
  facts verified on disk the ruling does not carry: the **built-in lock does
  not re-derive** (`fact = "edge"` count is 0 in builtin_lock.toml — no
  shipped kind declares an edge field), and the **legacy read is owed and has
  field evidence** — examples/base-harness/.temper/lock.toml:317-326 carries
  two bare-string `to` rows, so 0024's robust-read obligation (pipeline.md,
  "The lock") binds here exactly as it did for templates (08484b3), the
  shipped shape to follow rather than reinvent. One out-of-fence consequence
  noted, not filed: that example's own kinds.ts (135, 182) needs the same
  mechanical re-spelling, and examples/ is human territory — a
  `docs(example):` commit. Cursor advanced past a571973 too: `git log --
  specs/` proves it drops the two inline decision cites and nothing else
  (0c25b2c touches no specs file despite its subject line), so it carries no
  intent to derive and the delta is drained to HEAD. Closing checklist:
  EDGE-TARGET-SET shares graph.rs/drift.rs/declarations.ts with
  MENTION-REACHABLE-PREDICATE, so it is `blockedBy` it, and rides that chain
  transitively past SEAM-EXPORTS-RETRACT's sdk/test/emit.test.ts overlap; it
  is disjoint from RULE-CLAUSE, so the two run beside each other once
  PREDICATE lands. Both parks re-read and hold at this HEAD, unmoved since
  0c25b2c. Three forks re-read, no record moved. Audit/residue cursors copied
  forward verbatim — this tick reconciled no code window.
- Queue: 1 pickable (SEAM-EXPORTS-RETRACT); 3 blocked in one chain
  (MENTION-REACHABLE-PREDICATE → {-RULE-CLAUSE, EDGE-TARGET-SET}); 2 parked
  on human acts (IMPORT-HOP-CAP-CITE: a hop-depth probe.
  PACKAGING-CHANNELS-REMAINDER: Apple notarizing + the v0.1 tag). No gate is
  stale — all tested this tick.

Plan continues: no — the spec delta is drained to HEAD (0028 and 0029 both
routed, every Consequences bullet resolved), the inbox is empty, and no
commit sits past either code cursor. Build takes the pickable entry.
