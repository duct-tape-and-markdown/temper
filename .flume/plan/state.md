# Plan state

- **Phase:** derived-lock chain re-scoped. FIRST-PARTY-MODULE-COMPLETE +
  EMIT-INTO-PATH-DOUBLING shipped (ff937e7); the four floors now export from
  the SDK (sdk/src/builtins.ts), so the built-in lock can be derived. Empty
  spec delta; inbox empty.
- **Last shipped:** FIRST-PARTY-MODULE-COMPLETE (3ad2737) — skill/rule/
  memoryAnthropic/memoryAgentsMd floors carried into `@dtmd/temper/claude-code`,
  each clause cited; and EMIT-INTO-PATH-DOUBLING (3804b41) — canonicalize the
  SDK entry before the node spawn.
- **Re-scope this tick:** the single D3 (BUILTIN-LOCK-DERIVED) understated its
  blast radius — retiring builtin.rs/builtin_kind.rs touches ~8 src + ~11 test
  files (grepped: install/bundle/check/read/frontmatter/import/contract/main +
  11 tests), well past one gate-sized commit. Split into a foundation
  (additive: derive + embed + parse the built-in lock) and a row-driven
  consumer (reimplement the two mirrors as lock *projections*, keeping their
  APIs so the ~19 consumers stay green — the spec demands no hand-written
  mirror, not a file deletion). CURATED-TREES-RETIRE promoted open: its dep
  (FIRST-PARTY-MODULE-COMPLETE) shipped and its files (bundle.rs + test/snap)
  are disjoint from the whole chain.
- **Queue — 7 entries, 2 open (disjoint):**
  (1) BUILTIN-LOCK-DERIVED — **open**, foundation. builtin_lock.toml (from a
  memberless module emit) + builtin_lock.rs (parse at startup) + lib.rs pub
  mod. Additive.
  (2) CURATED-TREES-RETIRE — **open**, disjoint. Drop bundle.rs's
  CURATED_PACKAGES embeds.
  Then serialized: BUILTIN-LOCK-ROW-DRIVEN (blockedBy DERIVED; reimplements
  builtin/builtin_kind as lock projections, row-label identity, closes
  `(builtin-workspace-qualified-key)`) → {BUILTIN-LOCK-FROZEN-LANE (blockedBy
  DERIVED, parallel leaf, new test only), CHECK-LOCK-KIND-ROWS (blockedBy
  ROW-DRIVEN; cascade custom-kind-rows fix)}. COMMENT-STOCK-SWEEP — deferred
  (whole-tree solo). PACKAGING-CHANNELS — parked (release creds + engine-binary
  workflow + USPTO screen).
- **What's next:** build picks BUILTIN-LOCK-DERIVED + CURATED-TREES-RETIRE in
  parallel; the chain unblocks link by link. Queued **human** chores: the
  physical packages/** + kinds/** deletion (out of fence) once ROW-DRIVEN +
  CURATED land; PACKAGING-CHANNELS release setup.

Plan continues: no — queue reconciled (D3 re-scoped into a tractable chain,
CURATED promoted open), inbox empty, two disjoint open entries ready. Building
is how the chain drains.
