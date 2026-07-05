# Plan state

- **Phase:** drained the inbox's README-drift line into pending as an `open`
  entry; verified the CLI surface on disk against `temper --help`.
- **Last shipped:** DRIFT-DIFF-RETIRE (build b9c8f3c / chore 028bd72) — the
  dead four-state drift differ is gone from src/drift.rs.
- **This tick:** filed **README-RECONCILE-SURFACE** (`open`) — README.md:42/46/77
  and AGENTS.md:28 carry pre-demolition claims (retired `import`/`diff` verbs,
  `import` scan, hand-authored-TOML requirements, three-state drift). Verified
  against the trunk-head binary: the real surface is init / check / schema /
  emit / guard / install / bundle / explain — no `import`, no `diff`. Both files
  sit outside build's fence, disjoint from parked PACKAGING-CHANNELS. Inbox drained.
- **In flight:** README-RECONCILE-SURFACE is pickable now. PACKAGING-CHANNELS
  stays parked (npm scope in motion — John's @dtmd; marketplace/signing creds).
- **What's next:** build ships README-RECONCILE-SURFACE. John's hand for the rest —
  the SDK-primary front door (demolition core, `init` re-shape), the corpus
  shadow of the dogfood deactivation, release creds.

Plan continues: no — the inbox is drained, the queue is reconciled, and
README-RECONCILE-SURFACE is an `open` entry ready to hand to build. Building is
how the queue drains; nothing more to re-plan this turn.
