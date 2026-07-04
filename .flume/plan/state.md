# Plan state

- **Phase:** demolition-wave draining. HEAD 8648270 (+ this plan commit). Inbox
  empty; no corpus change since last reconcile.
- **Last shipped:** GATE-CORPUS-ARTIFACTS-LOCK (build 495c7da / chore 8648270).
  Verified on disk: `gate` (main.rs:805-845) reads **committed artifacts** — the
  in-place member's landscape file or the surface tree — never the retired
  pre-extracted `[[member]]` manifest corpus; the lock rides beside for freshness
  (`drift::read_declarations`). temper.toml survives only as the author layer
  (roster/bindings), retired later by MANIFEST-MACHINERY-RETIRE.
- **This tick:** GATE-CORPUS-ARTIFACTS-LOCK shipping unblocks its successor —
  **SDK-RECUT-CORPUS-FACE** flips `blockedBy → open`. One-line reconcile; nothing
  else in the chain moved. Inbox empty, no fork resolved.
- **In flight:** SDK-RECUT-CORPUS-FACE is the one pickable `open` head; the chain
  tail stays serialized behind it (it defines the seam + posture/needs model the
  later links carry) — CLI-COLLAPSE (+ temper guard) → MANIFEST-MACHINERY-RETIRE
  → KIND-PACKAGE-PARSE-RETIRE → EXPLAIN-UNIFY (terminal, fork-gated on
  `(explain-target-disambiguation)`). PACKAGING-CHANNELS parked on human creds.
- **What's next:** build drains from SDK-RECUT-CORPUS-FACE (cross-language,
  validated by the afterMerge `sdk test` gate); plan re-reconciles per green tick
  and unblocks the next link. Human halves: explain's target resolution;
  PACKAGING-CHANNELS creds; the session's fence-side migration once emit lands.

Plan continues: no — the queue is reconciled to the shipped GATE entry, a single
pickable `open` head exists, inbox is empty. Building is how the queue drains now.
