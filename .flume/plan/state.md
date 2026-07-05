# Plan state

- **Phase:** demolition-wave draining. HEAD dc9af8b (+ this plan commit). Inbox
  empty; no corpus change since last reconcile.
- **Last shipped:** LOCK-ASSEMBLY-ROWS-COMPLETE (build 6f404d8 / chore dc9af8b) —
  the lock's declaration rows now carry the whole assembly (RequirementRow +
  set-scope facets count/unique/membership/degree, the satisfies join family, and
  authority/reachability/edge assembly facts), read back via `drift::read_declarations`.
- **This tick:** unblocked **MANIFEST-MACHINERY-RETIRE** — its predecessor landed
  the whole assembly on the lock, so the gate-flip-plus-deletion is now the pickable
  head. Verified on disk it is unshipped (gate still reads `AuthorLayer` +
  `assembly_artifacts::load`, main.rs:387/500). Build gates omit the self-check
  (deactivated for the wave), so the zero-row committed lock does not block the flip.
- **In flight:** one pickable `open` head — MANIFEST-MACHINERY-RETIRE. Serial tail:
  KIND-PACKAGE-PARSE-RETIRE → EXPLAIN-UNIFY (terminal). PACKAGING-CHANNELS parked
  on human creds.
- **What's next:** build drains from MANIFEST-MACHINERY-RETIRE; plan unblocks the
  next link per green tick. **Session halves flagged:** (1) temper's committed
  `.temper/lock.toml` carries ZERO declaration rows (verified) — regenerate it via
  `init`/import (the temper.toml producer still fills it) as a `chore(harness)`
  before wave-end re-arms the self-gate. (2) the temper.toml→lock producer survives
  as transitional; retiring it (→ SDK `harness.ts` producing the dogfood lock) is
  the pending fence-side migration the session hand-authors when the producer retires.

Plan continues: no — queue reconciled to the ship, one pickable `open` head exists,
inbox empty. Building drains the queue now.
