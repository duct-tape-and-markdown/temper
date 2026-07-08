# Plan state

- Spec derived through: 474835b
- Audited through: 0594038
- Residue swept through: e32ed53
- This tick: Residue sweep (job 4), 8e09363..HEAD. Two code commits swept:
  1076bf3 (registration recut) + 9294a8d (genre snapshot sweep). Genre snapshot
  residue verified clean — `render_member` everywhere (src/display.rs:38,
  tests/display_rule.rs, all four display_rule snaps' expression metadata), the
  orphan read_verbs__impact_leaf_grain.snap deleted. Registration recut
  internally consistent — `Registration::{UserInvoked,DescriptionTrigger,Always,
  PathsMatch,Event}` channel set across src/kind.rs, builtin_kind.rs, graph.rs,
  install.rs; no old single-scalar residue. The two surviving `genre` mentions
  are deliberate retirement guards (nested_member.rs:193 `genre.`-fence-inert;
  emit.test.ts:386 retired-exports), not residue. Corpus's "five kinds ship"
  vs code's three (skill/rule/memory) is operationalized by pending
  COMMAND-KIND/AGENT-KIND — not a fileable gap. No new entries. Cursor
  8e09363 → e32ed53 (HEAD).
- Queue: 3 — COMMAND-KIND (open, pickable) → AGENT-KIND (blockedBy COMMAND-KIND)
  chain (shared builtin_kind.rs/builtins.ts/builtin_lock.toml); PACKAGING-CHANNELS
  parked. Disjoint (chain serialized; packaging touches package.json/.github).

Plan continues: yes — quiet closing pass (job 5): inbox empty, spec delta empty
(cursor 474835b), ship audit current (no code commits past 0594038), residue
swept to HEAD — all four inputs now current, closing pass remains.
