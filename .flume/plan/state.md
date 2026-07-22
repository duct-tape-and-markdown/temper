# Plan state

- Spec derived through: c2c7365 — routed in full, 0 new entries; see commit body.
- Audited through: 01ba369 — window 99b88a7..01ba369: two release commits
  (da77c60, b6dc0a3), version-bump/lockfile only, no src/ or sdk/src/
  ripple; verified clean, see commit body.
- Residue swept through: 01ba369 — same window: no retirement/demolition/
  stale vocabulary, no code touched at all; 0 filed.
- Posture swept through: mid-rotation, at src/graph.rs — filed
  GRAPH-IMPORT-HOP-CAP-TO-PROVIDER-FACE (now shipped); src/hash.rs next in
  the c9d11d5 re-arm rotation's frontier.
- This tick: POST-SHIP RECONCILIATION over 99b88a7..01ba369 (da77c60,
  b6dc0a3) — pure release-version chores. Re-tested PACKAGING-CHANNELS-REMAINDER's
  parked reason: the lockstep clause ("wait for a v0.1 tag") resolved
  differently — da77c60 walked Cargo.toml 0.1.0→0.0.11 to match npm's
  cadence directly, tag v0.0.11→da77c60 fired release.yml green
  (b6dc0a3's body). Rewrote the entry, scope narrows to darwin + channel 3;
  see commit body.
- Queue: 8 pending — 5 open, 1 blockedBy (HOOK-COMMAND-FAILS-LOUD-ON-MISSING-TEMPER
  behind GATE-INSTALLED-NAMES-FILES-SUPPRESS-UNADOPTED, both touching
  install.rs), 1 parked (PACKAGING-CHANNELS-REMAINDER, rewritten narrower
  this tick), 1 deferred (GUIDANCE-FIELD-DECLARATION-CHANNEL). Open forks:
  2, unchanged. Friction: 0. Amendments: 0. Inbox: 0.

Plan continues: after-build — inbox/spec-delta/reconciliation all quiet;
the only live job left is the posture sweep (mid-rotation, src/hash.rs
next), and 5 open entries are pickable now — ready work ships first, the
sweep resumes when the wave hands back.
