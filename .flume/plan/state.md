# Plan state

- Spec derived through: dff2db2
- Audited through: abec284
- Residue swept through: abec284
- This tick: Drain the inbox — route John's two ruled forks (07-15) into
  pending. Both resolutions arrived through the inbox AND their spec homes in
  dff2db2 (pipeline.md "The SDK" + "Install"); routing the inbox routes that
  delta, so Spec-derived advances to dff2db2. Filed DISCOVERY-NESTED-ROOT-FENCE
  ((discovery-nested-root-fence) → pipeline.md "Install"): live symptom
  confirmed on disk — `temper check .temper` reports memory (2), examples/base-
  harness/CLAUDE.md wrongly collected by memory's `**/CLAUDE.md` root=`.` glob
  because `discoverable_paths` fences only `.git`/`.temper`, not a nested
  `.temper/lock.toml`-bearing dir. Filed PROSE-INTERLEAVE-SDK
  ((prose-interleaving) → pipeline.md "The SDK"): `blocks()` today takes
  `EmbeddedMemberValue[]` only (prose.ts:186); SDK-only widen to interleave
  Text spans (emit.ts renderBody, declarations.ts row derivation) — no Rust
  ripple (payload body is pre-rendered, row families unchanged). Neither fork
  sat in open-questions (records deleted by John per the anti-accumulation
  rule); no open-questions change. Audit/residue cursors copied forward: no
  commit past abec284 touched src/tests/sdk (dff2db2 specs-only, 52631eb
  examples+docs+lock, f008120 plan) — no reconciliation due.
- Queue: DISCOVERY-NESTED-ROOT-FENCE (open), PROSE-INTERLEAVE-SDK (open) —
  disjoint files (src/import.rs vs sdk/src/{prose,emit,declarations}.ts);
  PACKAGING-CHANNELS-REMAINDER (parked, human release actions).

Plan continues: no — inbox drained, spec cursor at HEAD-specs (dff2db2), no
src/tests/sdk window past abec284. Two pickable open entries queued; build
takes over.
