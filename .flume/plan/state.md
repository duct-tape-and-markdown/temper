# Plan state

- Spec derived through: a53eee4
- Audited through: 9bf90bc
- Residue swept through: 52b3dcd
- This tick: Ship audit. Commits past the prior cursor (80697f8) touching
  src/tests/sdk: 3c6f50b (build: embedded kind render() hook) and 9bf90bc
  (chore(flume): ship EMBEDDED-KIND-RENDER-HOOK, pending.json-only diff).
  Verified on disk, not the log alone: sdk/src/kind.ts carries
  `KindDefinition.render`/`KindOptions.render`, and `embeddedMemberValue()`
  accepts a `KindDefinition` to carry the hook through; sdk/src/emit.ts's
  `renderMemberFence` calls the hook in place of `renderMemberToml` when
  present, the fence wrapper itself unchanged; the stale
  `parse_embedded_member`/`parse_embedded_info` doc-comment citations
  flagged as residue at plan 52b3dcd are gone from both files, and a fresh
  grep confirms no live citation of either symbol remains anywhere.
  `pnpm --dir sdk test` green, 51/51, including the new render-hook case.
  EMBEDDED-KIND-RENDER-HOOK's pending entry is already drained (9bf90bc).
  Re-tested EMBEDDED-LEAF-TEXT's stale `blockedBy EMBEDDED-KIND-RENDER-HOOK`
  gate per job 3's rule: the blocker shipped, so flipped the gate to `open`
  after re-verifying every symbol its file descriptions cite still resolves
  (`EmbeddedMemberValue.leaves`/`embeddedMemberValue()` in kind.ts,
  `renderMemberToml`/`resolveBody` in emit.ts, `mentionRows()` in
  declarations.ts, `leaf_addresses_are_structural_member_kind_key_child_path`
  at tests/nested_member.rs:118). PACKAGING-CHANNELS untouched — its parked
  reason doesn't depend on this ship.
- Queue: EMBEDDED-LEAF-TEXT (open, next); PACKAGING-CHANNELS (parked).
  Disjoint — EMBEDDED-LEAF-TEXT is now the sole open entry.

Plan continues: yes — residue-swept-through (52b3dcd) still trails HEAD by
one src/sdk-touching commit (3c6f50b) not yet re-swept; the next tick sweeps
residue before the loop can go quiet.
