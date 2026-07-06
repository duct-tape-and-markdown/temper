# Plan state

- **Phase:** drained the SDK-front-door-unblock inbox and reconciled the queue
  against the corpus after the 07-05 publish + `(sdk-package-layout)` ratification.
- **Last shipped:** README-RECONCILE-SURFACE (build 1656abe / chore 1dd6d60) —
  README/AGENTS carry the ratified CLI surface (init / check / schema / emit /
  guard / install / bundle / explain).
- **This tick:** filed **SDK-RECUT-SUBPATH-LAYOUT** (`open`, TS/`sdk/**`) — recut
  to the one-package layout, provider face behind the `@dtmd/temper/claude-code`
  subpath. Filed **GATE-READ-LOCK-DEMOLITION** (`parked`) — the SDK-front-door gate
  is discharged, so the gate rewrite + carriage demolition is unblocked, but its
  ~18-test-file blast radius needs a serialized-chain decomposition ceremony before
  parallel-safe slices are cut. Reconciled PACKAGING-CHANNELS (name resolved to
  `@dtmd/temper`; still parked on release creds + engine-binary workflow). Inbox
  drained. `(inplace-lock-producer)` DATUM records the front-door discharge.
- **In flight:** SDK-RECUT-SUBPATH-LAYOUT is the one pickable `open` entry
  (disjoint from the parked Rust work — different language, no shared files).
- **What's next:** build ships the SDK recut; the interactive session runs the
  demolition decomposition ceremony to cut GATE-READ-LOCK-DEMOLITION into a
  serialized chain (gate rewrite → carriage retire → `init` re-shape). Human hand
  still owns the release creds (PACKAGING-CHANNELS) and the USPTO screen.

Plan continues: no — the queue is reconciled, the inbox is drained, and one
disjoint `open` entry (SDK-RECUT-SUBPATH-LAYOUT) is ready for build. The remaining
gap (the gate/carriage demolition) is filed parked pending a human+session
decomposition ceremony, not further plan work this turn.
