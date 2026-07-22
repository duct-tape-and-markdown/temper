# Plan state

- Spec derived through: c2c7365 — routed in full, 0 new entries; see commit body.
- Audited through: 99b88a7 — window 8dd1436..99b88a7: one src/-touching
  commit (5e9d1fe, GRAPH-IMPORT-HOP-CAP-TO-PROVIDER-FACE's build tick);
  verified clean, see commit body.
- Residue swept through: 99b88a7 — same window: a pure literal relocation
  (pub const + import), no retirement/demolition/stale vocabulary; 0 filed.
- Posture swept through: mid-rotation, at src/graph.rs — filed
  GRAPH-IMPORT-HOP-CAP-TO-PROVIDER-FACE (now shipped); src/hash.rs next in
  the c9d11d5 re-arm rotation's frontier.
- This tick: SPEC DELTA — routed c2c7365 ("command requires no
  frontmatter", specs/builtins.md "The shipped kinds") in full, 0 new
  entries: COMMAND-CONTRACT-EMPTY-NO-SKILL-IMPORT (filed the 9d4ce81 tick,
  before this delta was read) already carries the identical fix against
  the identical bullet; `per` still matches the live heading. Cursor
  advances to c2c7365; see commit body.
- Queue: 8 pending — 5 open, 1 blockedBy (HOOK-COMMAND-FAILS-LOUD-ON-MISSING-TEMPER
  behind GATE-INSTALLED-NAMES-FILES-SUPPRESS-UNADOPTED, both touching
  install.rs), 1 parked (PACKAGING-CHANNELS-REMAINDER), 1 deferred
  (GUIDANCE-FIELD-DECLARATION-CHANNEL) — unchanged, not re-touched this
  tick. Open forks: 2, unchanged. Friction: 0. Amendments: 0. Inbox: 0.

Plan continues: yes — post-ship reconciliation is next: the
Audited/Residue-swept window (99b88a7..HEAD) holds two sdk/-touching
release commits (da77c60, b6dc0a3) not yet reconciled.
