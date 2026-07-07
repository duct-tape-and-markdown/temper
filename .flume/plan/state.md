# Plan state

- **Phase:** derived-lock chain draining — final link now open. Inbox empty;
  spec delta empty (no `specs/` commits this window since 4ed47a0).
- **Last shipped (5f68c8c):** BUILTIN-FLOOR-LOCK-PROJECTION — builtin.rs's
  floors now project off the lock's clause rows; the package noun
  (SKILL_PACKAGE/FLOOR_BINDINGS/`<kind>.<source>` names) retired from code.
  Residue sweep (grep): package-noun/reachability-dial/role/EdgeClause symbols
  all zero; the surviving `SDK_PACKAGE` hits are the legitimate npm name.
- **This tick:** projection shipped, so CHECK-LOCK-KIND-ROWS unblocked →
  flipped to **open** and rescoped off an agent trace. It is still real
  (main.rs::gate hardcodes `custom_kinds = Vec::new()` at :631/:416; a `require`
  naming a custom kind fails `requirement.admissibility`), but narrower than
  filed: drift.rs already parses `declarations.kinds`, engine.rs is kind-blind
  (no edit), and gate doesn't route through check.rs's `Workspace`. Scope now =
  main.rs::gate + a `KindFactRow→CustomKind` ctor in kind.rs; engine.rs/check.rs
  dropped from the entry.
- **Queue — 3 entries, 1 open:** CHECK-LOCK-KIND-ROWS (open, next).
  COMMENT-STOCK-SWEEP deferred (whole-tree solo; `files.edit` expanded to the
  real residue set — builtin_kind.rs/builtin.rs/bundle.rs/read.rs, incl.
  read.rs's dangling `READ-EDGE-UNIFY` tag; promoted once the chain ships and
  the queue is otherwise empty). PACKAGING-CHANNELS parked (release creds +
  engine workflow + USPTO).
- **What's next:** build picks CHECK-LOCK-KIND-ROWS — the chain's final link.
  On contact it cuts main.rs's now-false KIND.md comments; when it ships, plan
  promotes COMMENT-STOCK-SWEEP to solo open.

Plan continues: no — queue reconciled (projection's cleared successor flipped
to open and rescoped, sweep files expanded, open-questions asymmetry updated),
inbox empty, delta empty. Building drains the chain's last link.
