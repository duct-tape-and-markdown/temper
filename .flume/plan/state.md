# Plan state

- **Phase:** derived-lock chain **complete**; the comment-residue sweep opens.
  Inbox empty; spec delta empty (no `specs/` commits since 4ed47a0 — the last
  three plan ticks only touched `.flume/`).
- **Last shipped (53da730):** CHECK-LOCK-KIND-ROWS — main.rs::gate synthesizes
  custom kinds from the lock's `KindFactRow`s (no more hardcoded
  `custom_kinds = Vec::new()`); the chain's final link. On contact it cut
  main.rs's now-false KIND.md/tree comments (grep: zero `kinds/`/`packages/`
  refs left in main.rs).
- **This tick:** promoted COMMENT-STOCK-SWEEP to **open** (chain shipped, queue
  otherwise empty) and re-scoped it off a fresh whole-tree grep. Kept to the
  four src files whose comments are **false** (retired trees, `[edge.*]`
  construct, package-era narration). Dropped tests/session_start.rs: its
  KIND.md/PACKAGE.md lines narrate **live fixture code** (it writes `+++`
  old-format files to assert session-start ignores stray files) — a behavioral
  test question, not a comment cut. The tree-wide stale-**form** residue (~286
  accurate quoted section titles, ~60 dangling shipped-tag refs) is left to
  rust.md's cut-on-contact exit clause, not a dedicated churn slice.
- **Queue — 2 entries, 1 open:** COMMENT-STOCK-SWEEP (open, solo).
  PACKAGING-CHANNELS parked (release creds + engine-binary workflow + USPTO).
- **What's next:** build picks COMMENT-STOCK-SWEEP. Beyond it, the next engine
  wave is `(json-projection-format)` — its SDK-primary foundation (the
  derived-lock chain) has now shipped, so it is unblocked, but the JSON
  adapter / `layout`-fact spelling is an open fork awaiting John before it can
  be filed. Accepted debt noted in the commit body: tests/session_start.rs's
  retired-format fixtures, and the tree-wide cut-on-contact comment residue.

Plan continues: no — queue reconciled (sweep promoted + re-scoped, open-questions
asymmetry and json fork updated), inbox empty, delta empty, one pickable open
entry. Building drains it.
