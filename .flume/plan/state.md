# Plan state

- **Phase:** demolition-wave draining. HEAD 5df6de9 (+ this plan commit). Inbox
  empty; no corpus change since last reconcile.
- **Last shipped:** LOCK-DECLARATION-ROWS (build 957e83b / chore 5df6de9) — the
  lock grew a declaration-row family (`[[declaration.{kind,clause,requirement,
  assembly}]]`) beside provenance + emit fingerprints; `drift::read_declarations`
  reads it back. Verified on disk: the reader exists but is **not yet wired** into
  the gate — `check` still reads `temper.toml`-as-corpus (main.rs:435-508). So the
  next link is real, unshipped work.
- **This tick:** LOCK-DECLARATION-ROWS shipping unblocks its successor —
  **GATE-CORPUS-ARTIFACTS-LOCK** flips `blockedBy → open` (the gate re-points its
  corpus read to the lock's declaration rows + committed `.claude/**` artifacts).
  One-line reconcile; nothing else in the chain moved.
- **In flight:** GATE-CORPUS-ARTIFACTS-LOCK is the one pickable `open` head; the
  chain tail stays serialized behind it — SDK-RECUT-CORPUS-FACE →
  CLI-COLLAPSE (+ temper guard) → MANIFEST-MACHINERY-RETIRE →
  KIND-PACKAGE-PARSE-RETIRE → EXPLAIN-UNIFY (terminal, fork-gated on
  `(explain-target-disambiguation)`). PACKAGING-CHANNELS parked on human creds.
- **What's next:** build drains from GATE-CORPUS-ARTIFACTS-LOCK; plan re-reconciles
  per green tick and unblocks the next link. Human halves: explain's target
  resolution; PACKAGING-CHANNELS creds; the session's fence-side migration once
  emit lands.

Plan continues: no — the queue is reconciled to the shipped LOCK entry, a
pickable `open` head exists, inbox is empty. Building is how the queue drains now.
