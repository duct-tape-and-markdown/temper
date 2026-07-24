# Plan state

- Spec derived through: 20a6f54 — unchanged, no spec/ commits past it.
- Audited through: 729b0cad — advanced; window f194b260..729b0cad reconciled.
- Residue swept through: 729b0cad — advanced; same window, clean.
- Posture swept through: 97d0241 — still armed: forward window now also
  carries 29894c04 (src/) and ffa93dfb (sdk/src/) on top of the prior
  reconciled window.
- This tick: POST-SHIP RECONCILIATION — audited+swept f194b260..729b0cad.
  29894c04 (SCHEMA-HELP-KIND-DOMAIN-FOSSIL build) matches: `schema --kind`'s
  doc string now points at the unknown-kind error's live domain instead of
  the retired `skill`/`rule` pair (main.rs:88-93); tests/cli.rs asserts the
  help text no longer contains the hardcoded pair. ffa93dfb
  (TAP-HOOK-FIELD-SCHEMA-PROVIDER-FACE build) matches: `tapHookRegistration`
  (builtins.ts:398-441) now owns the `hooks.<Event>` key-path
  (`HOOK_KEY_PATH`) and the `type`/`command`/`matcher` field triple;
  `tapHookRows` (declarations.ts:788) calls it instead of authoring the
  literals inline. Both entries were already drained from pending.json by
  their own 729b0cad ship commit — nothing left to drop. rg repo-wide:
  the surviving `"hooks.<Event>"` literals (kind.ts:140 union member,
  builtin_lock.toml:83, kind.rs:973) are the kind's own declared address on
  the Rust/lock side, not the fixed erasure-module duplicate — no residue.
  Verified live: cargo test (all crates green), clippy -D warnings clean,
  fmt clean, pnpm sdk test (141/141). No findings.
- Queue: 3 pending — 0 open, 1 parked, 2 deferred. Open forks: 2, unchanged.
  Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: yes — the posture sweep is armed with a widened forward
window and the queue has no pickable entries, so plan (not build) takes
the next tick.
