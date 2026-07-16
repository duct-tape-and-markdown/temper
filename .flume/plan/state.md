# Plan state

- Spec derived through: 06e0c2c
- Audited through: cb5da8d
- Residue swept through: cb5da8d
- This tick: drained the inbox — three 07-15 fork rulings routed into pending,
  all `open` and disjoint. AGENTS-MD-STDLIB-DROP (agents-md remnant is
  SDK-side only — builtins.ts doc+export, claude-code.ts:22 re-export,
  builtins.test.ts, and the src/builtin_lock.toml header comment; NO Rust
  engine remnant, lock declaration rows already spec-faithful). Prior attempt
  was reverted (refs gate) for putting `memoryAgentsMdDefaultContract` in
  `retire` — it is a symbol in surviving files, so refiled as `edit`s.
  LAYOUT-EMPTY-REGION-TOLERATE: verified a REAL gap — `Layout::read`
  (kind.rs) is positional and raises `LayoutError::MissingSection` on an
  empty region; layout_kind.rs:126 asserts that now-wrong behavior,
  contradicting the 39a4833 representation.md amendment (regions state what
  may appear, never what must). FLAT-GLOB-DEPTH-REFUSE: confirmed the
  silent-nonsense path (`member_projection_path`, drift.rs:565
  `replacen('*', name, 1)`) — refuse loudly instead.
- Queue: 3 pickable (AGENTS-MD-STDLIB-DROP, LAYOUT-EMPTY-REGION-TOLERATE,
  FLAT-GLOB-DEPTH-REFUSE — disjoint files: sdk/+lock.toml, kind.rs, drift.rs)
  + PACKAGING-CHANNELS-REMAINDER parked. Spec cursor still at 06e0c2c.

Plan continues: yes — spec delta 39a4833 (four fork rulings) is unrouted; the
spec cursor stays at 06e0c2c and next tick's job 2 advances it (three of the
four rulings are now routed via these entries; the fourth, "one read verb",
is still to derive/verify). NB the SessionStart reporter shows the `.temper`
dogfood gate red (friction-capture-procedure, pending-entry-discipline
unfilled) — harness territory, a `chore(harness)` fix outside plan's writable
paths.
