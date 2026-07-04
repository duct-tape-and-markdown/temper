# Plan state

- **Phase:** demolition-wave filed. HEAD 146a646 (+ this plan commit). Inbox drained.
- **This tick:** John's go-ruling (inbox 2026-07-04, "let's go") UNBLOCKED the
  six-noun engine demolition. Filed it as one **linear serialized chain**
  (build-order per John's own enumeration), because the blast radii are deeply
  entangled — manifest/KIND/PACKAGE references span nearly every src file, so a
  parallel fan-out would conflict-and-revert. Chain: LOCK-DECLARATION-ROWS
  (open head) → GATE-CORPUS-ARTIFACTS-LOCK → SDK-RECUT-CORPUS-FACE → CLI-COLLAPSE
  (+ temper guard) → MANIFEST-MACHINERY-RETIRE → KIND-PACKAGE-PARSE-RETIRE →
  EXPLAIN-UNIFY (terminal, fork-gated). PACKAGING-CHANNELS stays parked on human
  creds. Self-gate deactivated for the wave (chain.ts, John's commit); dogfood
  artifacts go stale by design; session runs the wave-end confirmation + re-arm.
- **In flight:** LOCK-DECLARATION-ROWS is the one pickable `open` entry; the
  rest of the chain is `blockedBy` its predecessor, drained one tick at a time
  (a ship unblocks the next on the following reconcile).
- **Fork-gated tail:** EXPLAIN-UNIFY carries `dependsOnForks:
  ["explain-target-disambiguation"]` — the corpus does not specify how one
  `explain <target>` resolves across member/requirement/address/neighborhood
  (member-vs-requirement name collision). It is a terminal leaf blocking nothing;
  John must rule the target-resolution mechanism before build picks it.
- **What's next:** build drains the chain from LOCK-DECLARATION-ROWS; plan
  re-reconciles per green tick and unblocks the next link. Human halves:
  explain's target resolution; PACKAGING-CHANNELS creds; the session's
  fence-side .claude/specs migration onto the SDK face once emit lands.

Plan continues: no — the demolition wave is filed as a serialized chain with a
pickable open head, inbox drained, state re-derived. Building is how the queue
drains now; the next moves are build (the chain) and John (explain's fork).
