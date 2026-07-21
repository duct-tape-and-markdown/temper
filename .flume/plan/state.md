# Plan state

- Spec derived through: 946e303 — unchanged; `git log 946e303..HEAD -- specs/` is empty.
- Audited through: e15b969 — reconciled this tick; `git log 3f1af5f..e15b969 -- src/ tests/ sdk/`
  was one commit (9534776).
- Residue swept through: e15b969 — same window, same tick.
- Posture swept through: sdk/src/ tree fully covered; src/address.rs, src/admissibility.rs,
  src/builtin.rs, src/builtin_kind.rs covered. src/builtin_lock.rs next in tree order —
  mid-rotation.
- This tick: POST-SHIP RECONCILIATION over 3f1af5f..e15b969 — audit: verified 9534776 shipped
  TAP-EVENT-DOCUMENTED-NAMES-DEDUP exactly as scoped (one exhaustive `documented_name` match,
  `documented_event_names()` reads through it, both tap.rs tests pass unchanged), entry already
  drained from pending.json by the ship commit itself, `cargo test --lib tap::` green, metrics
  window unremarkable (build 42 turns/289s, clean merge, no revert). Re-tested both stale gates:
  PACKAGING-CHANNELS-REMAINDER's parked reason still holds verbatim (no v0.1 tag, crate 0.1.0,
  release.yml's darwin/channel-3 deferral text unchanged, `git diff a23269d..HEAD -- .github/`
  empty); GUIDANCE-FIELD-DECLARATION-CHANNEL's deferred reason unaffected (window never touches
  sdk/src/kind.ts). Sweep: window's only src/ change is the tap.rs dedup itself; grepped
  TapEvent's wire-name strings repo-wide — telemetry.rs's snake_case mapping and the Claude Code
  hook-vocabulary literals elsewhere are distinct, already-exhaustive concerns, not a second
  copy of tap.rs's fix — no residue found.
- Queue: 2 pending — 0 open, 1 deferred (GUIDANCE-FIELD-DECLARATION-CHANNEL), 1 parked
  (PACKAGING-CHANNELS-REMAINDER); 0 open questions unresolved by this queue. Open forks: 2,
  unchanged. Friction: 0. Amendments: 1, still awaiting ratification. Inbox: 0.

Plan continues: yes — no pickable entry exists (both remaining are parked/deferred), so the
open posture rotation drives itself next tick, resuming at src/builtin_lock.rs.
