# Plan state

- Spec derived through: a9f7b9e
- Audited through: 5f27db2
- Residue swept through: 5f27db2
- This tick: Post-ship reconciliation, window c5df845..HEAD (MCP-SERVER-KIND,
  1ffab8f/ff84346). AUDIT: verified on disk mcp-server shipped — `check .temper`
  shows `mcp-server (0)` checked and `.mcp.json`'s finding retired (governs now
  spans_whole_manifest). COVERAGE-PARTIAL-GOVERNANCE re-verified live (probe at
  5f27db2 still emits `hook (3)` AND `checks none of its members` in one run) —
  left `open`, accurate. Re-tested MANIFEST-WRITE-SIDE's parked reason: STALE —
  it claimed MCP-SERVER-KIND "has not" shipped, but phase 1 (MANIFEST-ADAPTER-
  READ/HOOK-KIND/MCP-SERVER-KIND) is now fully shipped, so its own designated
  re-derivation fires. Re-scoped the placeholder into a blockedBy chain: SDK-
  ERASURE (open) → EMIT-FACE → {BUNDLE, GUARD, COVERAGE-RETIRE}. SWEEP: MCP
  opened builtins.ts (packages/ cites 344/384/421→392/432/469) + extract.rs
  (floor-mention 196-198 left; law-5 fixtures 1227/1262→1223/1258) — riders
  updated with new lines, undischarged per reconciliation-not-opening; no new
  residue from the well-cited MCP kind. Layout-fact/prose/Cargo.toml/compose
  riders untouched this window (verified). Both cursors → 5f27db2.
- Queue: COVERAGE-PARTIAL-GOVERNANCE (open) + MANIFEST-WRITE-SDK-ERASURE (open,
  file-disjoint) → EMIT-FACE (blockedBy SDK-ERASURE) → BUNDLE/GUARD/COVERAGE-
  RETIRE (blockedBy EMIT-FACE) → PACKAGING-CHANNELS (parked). Two open heads,
  disjoint files; all blockedBy tags resolve.

Plan continues: no — window reconciled to 5f27db2, inbox/spec-delta empty; two
open pickable entries, build takes over.
