# Plan state

- Spec derived through: 474835b
- Audited through: 0594038
- Residue swept through: 8e09363
- This tick: Ship audit (job 3), c44ba6b..HEAD. REGISTRATION-CHANNELS shipped
  (registration recut to a channel set — `Registration::UserInvoked` +
  `DescriptionTrigger` as a vec, builtin_kind.rs:30; SDK `registration:[{via:
  "user-invoked"},…]`, builtins.ts:66) → unblocked COMMAND-KIND, gate open.
  Full-rewrote it: refreshed stale line refs (all_kinds :122, enumeration tests
  :307-323, lock carries three kind rows), swapped its wrong `lock_declaration_
  rows.rs` target (golden-row test, blind to the built-in set) for `builtin_lock_
  frozen.rs` (the real byte-compare), flagged `commandFloor = skillFloor minus
  nameMatchesDir` (a command is a lone file), re-stamped scoped-at 0594038.
  GENRE-SNAPSHOT-RESIDUE verified swept (9294a8d deleted the orphan snap +
  regenerated 4 display_rule snaps). PACKAGING-CHANNELS parked reason holds
  (diff touched no package.json/.github). AGENT-KIND stays blockedBy COMMAND-KIND
  (verbatim; line refs re-verify at unblock). Cursor c44ba6b → 0594038 (HEAD).
- Queue: 3 — COMMAND-KIND (open, pickable) → AGENT-KIND (blockedBy COMMAND-KIND)
  chain (shared builtin_kind.rs/builtins.ts/builtin_lock.toml); PACKAGING-CHANNELS
  parked.

Plan continues: yes — residue sweep (job 4): residue cursor 8e09363 trails HEAD
0594038; the registration recut + genre sweep (1076bf3..0594038) are unswept.
