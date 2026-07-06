# Plan state

- **Phase:** reconciled the demolition tail after INSTALL-FRONT-DOOR (S6)
  shipped. Found **TEMPER-TOML-ZERO (S7) badly under-scoped** — its stated fence
  (compose.rs + main.rs, acceptance `rg=0`) is a fiction: `temper.toml` is woven
  through **21 files** (9 src + 12 test, 200+ refs), and the gate still composes
  each kind's contract from the temper.toml `[kind.*]` layer (`compose::effective`,
  main.rs:691), never the lock's `ClauseRow` (which `emit` already writes). Split
  it into a serialized sub-chain.
- **Last shipped:** INSTALL-FRONT-DOOR / S6 (build 7102391 / chore 79ce5af).
  Verified on disk: `Command::Install` dispatches `install::discover` +
  `ask_represent` + `install::run` (main.rs:383-405) — install is the front door.
- **This tick:** rewrote S7 into **GATE-CONTRACT-FROM-LOCK** (open) →
  **TEMPER-TOML-ZERO** (blockedBy), truthful blast radii. The head wires the gate's
  effective-contract clauses off the lock (20-surface: "the gate never reads
  declarations from anywhere but the lock" — clauses are a declaration family; no
  fork, so plan derives it); the terminal retires the reader + sweeps every ref to
  zero. Inbox empty. No fork moved.
- **In flight:** one `open` head — GATE-CONTRACT-FROM-LOCK. TEMPER-TOML-ZERO waits
  behind it (both touch main.rs/compose.rs/the same test files — serialized, not
  parallel). PACKAGING-CHANNELS parked on human release setup.
- **What's next:** build drains GATE-CONTRACT-FROM-LOCK; its ship unblocks
  TEMPER-TOML-ZERO on the following reconcile (refresh main.rs/compose.rs line refs
  then). Human owns PACKAGING-CHANNELS release setup, the USPTO name screen, and the
  genre-fence-format workshop (cascade pilot).

Plan continues: no — S6 shipped, S7 is re-scoped into a serialized head+terminal
with truthful fences, the head is `open`, and the inbox is drained. Building is how
the queue drains from here.
