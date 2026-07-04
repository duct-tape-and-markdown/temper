# Plan state

- **Phase:** demolition-wave draining. HEAD bef1902 (+ this plan commit). Inbox
  empty; no corpus change since last reconcile; no fork resolved.
- **Last shipped:** SDK-RECUT-CORPUS-FACE (build 9dc9162 / chore bef1902).
  Verified on disk: `sdk/src/` now carries the six-noun face (needs.ts,
  declarations.ts, kind.ts, contract.ts, genres.ts, project.ts, lock.ts) — the
  posture/`needs` model + declaration-row seam the later engine links carry.
- **This tick:** SDK-RECUT shipping unblocks its successor — **CLI-COLLAPSE**
  flips `blockedBy → open`. Verified CLI-COLLAPSE is unshipped: main.rs still
  carries Import/Diff/SessionStart/Why/Requirements/Impact/Context (no Guard, no
  Explain). One-line reconcile; nothing else moved.
- **In flight:** CLI-COLLAPSE (+ `temper guard`) is the one pickable `open`
  head; the tail stays serialized behind it (each defines the seam the next link
  consumes) — MANIFEST-MACHINERY-RETIRE → KIND-PACKAGE-PARSE-RETIRE →
  EXPLAIN-UNIFY (terminal, fork-gated on `(explain-target-disambiguation)`).
  PACKAGING-CHANNELS parked on human creds.
- **What's next:** build drains from CLI-COLLAPSE; plan re-reconciles per green
  tick and unblocks the next link. Human halves: explain's target resolution;
  PACKAGING-CHANNELS creds; the session's fence-side migration once emit lands.

Plan continues: no — the queue is reconciled to the shipped SDK-RECUT entry, a
single pickable `open` head exists, inbox is empty. Building drains the queue now.
