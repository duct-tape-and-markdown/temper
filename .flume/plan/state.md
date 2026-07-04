# Plan state

- **Phase:** reconcile. HEAD ebb822c.
- **Last shipped:** MEMORY-PROJECTION-SDK (`build` 1ec14fb / `chore` d6ce2c2) — SDK
  emit projects a module-carried memory to its root `CLAUDE.md`/`AGENTS.md`.
- **This tick:** drained the inbox — John ruled all four SDK dogfood-migration
  seam forks (session ceremony, each on plan's filed recommendation). Marked the
  four forks RESOLVED and filed them as **two disjoint serialized chains**, split
  by the file each touches: a Rust-gate chain over `src/main.rs`
  (GATE-KIND-RESOLVE → GATE-READS-ASSEMBLY) and an SDK chain over
  `sdk/src/emit.ts` (SDK-CARRIAGE-CONVERGE → SDK-PLACEMENT-ROUNDTHROUGH). The two
  chain heads are cross-language disjoint (`src/**`+`tests/**` vs `sdk/**`), so
  both are `open` and parallel-safe; each chain's second link is `blockedBy` its
  head (shared file). Re-verified the three carried entries hold: EXTRACTION-VOCAB-GAPS
  deferred (`Primitive::Field` flat-reads `unit.frontmatter.get(key)`, kind.rs:836),
  AGENT-KIND deferred (`BUILTIN_KINDS = ["skill","rule"]`, kind.rs:30),
  PACKAGING-CHANNELS parked.
- **Pickable now:** GATE-KIND-RESOLVE and SDK-CARRIAGE-CONVERGE (both `open`,
  disjoint — Rust gate vs SDK emit). Their followers unblock on the next reconcile.
- **What's next:** build ships the two heads in parallel; a green ship of each
  unblocks its serialized follower. After the four seams land, the dogfood's full
  migration onto the SDK is the ledger's TS-primary human ceremony (not a pending
  entry).

Plan continues: no — inbox drained, the four ruled forks filed as two parallel-safe
serialized chains, two `open` heads pickable. Hand to build.
