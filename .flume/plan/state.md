# Plan state

- Spec derived through: a53eee4
- Audited through: 6d6ae89
- Residue swept through: 3c6f50b
- This tick: Ship audit. Commits past 9bf90bc touching src/tests/sdk: only
  18d3406 (build: embedded member leaves accept a Text template). Verified
  on disk, not just the log: kind.ts's `EmbeddedMemberValue`/
  `EmbeddedMemberCollectionEntry.leaves` accept `string | Text`; emit.ts's
  renderMemberToml and declarations.ts's mentionRows/nested-member-row
  resolution both route through prose.ts's `resolveLeaf`, matching the
  commit's claimed shared-address-set consolidation. `pnpm --dir sdk test`
  55/55 green; `cargo test --test nested_member` 7/7 green. pending.json
  already had EMBEDDED-LEAF-TEXT retired by the flume dispatcher's own
  6d6ae89 commit — no entry rewrite needed there. Re-tested PACKAGING-
  CHANNELS' parked reason: package.json is still `temper-flume-harness`
  (private), `.github/workflows/` still holds only `temper.yml` — still
  parked, still accurate; refreshed its re-verification stamp to 6d6ae89.
  Re-tested the three "kept on purpose" debts' conditions against 18d3406's
  actual diff (declarations.ts, emit.ts, kind.ts, prose.ts, emit.test.ts,
  tests/nested_member.rs only): the kinds/packages-retirement debt
  (tests/session_start.rs `+++` fixtures; sdk/src/builtins.ts:308,348,385
  citations) and the `overlay_builtin_kind` stale-comment debt
  (tests/coverage.rs:336-338) are both untouched, both still accurate. The
  pre-corpus-reorg citation debt's prediction ("EMBEDDED-LEAF-TEXT already
  opens kind.ts; its exit clause fires there") did NOT hold — 18d3406
  touched kind.ts but never reached the 8 citation lines; corrected the note
  (also fixed a pre-existing hit-count transposition between kind.ts (8) and
  contract.ts (12), verified against disk and `git show 3c6f50b`) — the debt
  still rides whichever entry opens kind.ts next.
- Queue: PACKAGING-CHANNELS only (still parked on human release creds + the
  engine-binary workflow). No open buildable entries.

Plan continues: yes — residue sweep is next live: `Residue swept through`
(3c6f50b) trails HEAD (6d6ae89), and inbox/spec-delta/ship-audit are all
quiet as of this tick.
