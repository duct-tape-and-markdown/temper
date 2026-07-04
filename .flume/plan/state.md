# Plan state

- **Phase:** reconcile. HEAD 67be950.
- **Last shipped:** **SDK-EMIT-REFUSALS** (`build` 173f4fa / `chore` 67be950) —
  emit's two declare-side refusals (`refuseBrokenSource`, sdk/src/emit.ts:328):
  a `satisfies` naming no declared requirement (dangling join) and a `required`
  requirement no member fills both throw before a byte is written.
- **This tick:** reconciled the queue against disk. SDK-EMIT-REFUSALS shipped, so
  **SDK-ASSEMBLY-ARTIFACTS flips blockedBy→open** — its blocker drained and the
  emit surface is stable. CONTRACT-DIR stays blockedBy SDK-ASSEMBLY-ARTIFACTS
  (both touch `sdk/test/emit.test.ts`; goldens want emit frozen first). The three
  carried entries hold: EXTRACTION-VOCAB-GAPS + AGENT-KIND deferred (no consumer —
  `Primitive::Field` flat-reads kind.rs:836; no `kinds/claude-code/agent/`),
  PACKAGING-CHANNELS parked (root package.json still the private flume manifest).
  Inbox empty — nothing to drain.
- **Pickable now:** **SDK-ASSEMBLY-ARTIFACTS** — sdk/** only, in-fence, clean
  cite (20-surface "the bindings, the roster — emitted as small temper-owned
  artifacts"). Additive: manifest/projection/lock bytes unchanged.
- **What's next:** build ships SDK-ASSEMBLY-ARTIFACTS, then the next reconcile
  un-blocks CONTRACT-DIR (the last in-fence ratification item). No-demolition
  holds (D6): engine-side retirements stay evidence-gated behind the human
  dogfood migration onto the SDK, never filed.

Plan continues: no — queue reconciled and truthful, inbox empty, and a pickable
`open` head (SDK-ASSEMBLY-ARTIFACTS) exists. Building is how the queue drains;
hand to build.
