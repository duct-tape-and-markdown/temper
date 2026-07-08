# Plan state

- Spec derived through: 474835b
- Audited through: c44ba6b
- Residue swept through: 8e09363
- This tick: Residue sweep (job 4), 886dc64..HEAD. Filed GENRE-SNAPSHOT-RESIDUE
  — the genre→member recut (0012/EMBEDDED-FENCE) left snapshot residue: one
  orphan `read_verbs__impact_leaf_grain.snap` (dead — assertion_line 623 >
  read_verbs.rs's 377 lines, retired 'genre' coverage body, unreferenced; live
  leaf-grain test asserts via `.contains`), plus 4 `display_rule__*.snap` with
  stale `render_genre` expression metadata (comment-class, rid here). `git grep
  genre` in src/sdk/src is clean; live genre strings in nested_member.rs:193 &
  emit.test.ts:386 are sanctioned retirement guards, kept. 0013 (Format label)
  / 0015 (manifest machinery) residue stays fork-routed by
  (format-template-spelling) / (manifest-authoring-surface), unchanged. Cursor
  886dc64 → 8e09363 (HEAD).
- Queue: 5 — REGISTRATION-CHANNELS (open) + GENRE-SNAPSHOT-RESIDUE (open,
  disjoint: tests/snapshots only) both pickable; COMMAND-KIND → AGENT-KIND
  blocked chain (shared builtin_kind.rs/builtins.ts/builtin_lock.toml);
  PACKAGING-CHANNELS parked.

Plan continues: no — all cursors current (inbox empty; no spec delta past
474835b; ship audit c44ba6b..HEAD is plan-only; residue swept to HEAD). Two
pickable open entries exist — build takes over.
