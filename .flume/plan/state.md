# Plan state

- **Phase:** demolition-wave draining, re-sequenced after the MANIFEST bail.
  HEAD 9dee3c3 (+ this plan commit). Inbox drained (2 items); no corpus change
  since last reconcile.
- **Last shipped:** CLI-COLLAPSE (build da84b69 / chore 9a250ed) — the collapsed
  face init·check·emit·install·bundle·schema·guard.
- **This tick:** MANIFEST-MACHINERY-RETIRE voluntarily bailed (mis-sequenced —
  the lock could not carry the whole assembly, RequirementRow lacks set-scope
  facets + satisfies). Re-scoped per the inbox: inserted predecessor **LOCK-
  ASSEMBLY-ROWS-COMPLETE** (the new open head — enrich the lock rows), and the
  retire is rewritten as a clean gate-flip-plus-deletion behind it. Also resolved
  `(explain-target-disambiguation)` (John: bare target + collision error) →
  EXPLAIN-UNIFY un-forked, mechanism encoded.
- **In flight:** one pickable `open` head — LOCK-ASSEMBLY-ROWS-COMPLETE. Serial
  tail: MANIFEST-MACHINERY-RETIRE → KIND-PACKAGE-PARSE-RETIRE → EXPLAIN-UNIFY
  (terminal). PACKAGING-CHANNELS parked on human creds.
- **What's next:** build drains from LOCK-ASSEMBLY-ROWS-COMPLETE; plan unblocks
  the next link per green tick. **Session halves flagged:** (1) before MANIFEST-
  MACHINERY-RETIRE flips the gate, temper's committed `.temper/lock.toml` carries
  ZERO declaration rows — regenerate it (re-run `init`/import, which the temper.toml
  producer still fills) as a `chore(harness)`; the self-gate stays off for the
  wave. (2) the temper.toml→lock producer survives as transitional; retiring it
  (→ SDK `harness.ts` producing the dogfood lock) is the pending fence-side
  migration — the session hand-authors that SDK assembly when the producer retires.

Plan continues: no — the queue is reconciled to the bail + explain ruling, one
pickable `open` head exists, inbox is drained. Building drains the queue now.
