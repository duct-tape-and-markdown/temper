# Plan state

- Spec derived through: a53eee4
- Audited through: 80697f8
- Residue swept through: 52b3dcd
- This tick: Residue sweep (was trailing at a3f9f1f). Two src/tests/sdk
  commits landed since: 5ee0b6d (RETIRE-FOLD-MEMBERS) and 7a3dfff
  (NESTED-MEMBER-COLLECTIONS-ORDERED) — the intervening `chore(flume) ship`
  commits (f000b97, 80697f8) carry no src/tests/sdk diff. Re-verified both
  tracked debts against the actual diffs (not the log): 5ee0b6d touched
  src/builtin_kind.rs, src/drift.rs, src/extract.rs, src/kind.rs,
  src/main.rs, tests/agent_kind.rs, tests/command_kind.rs,
  tests/lock_declaration_rows.rs, tests/nested_member.rs; 7a3dfff touched
  sdk/src/{declarations,emit,index,kind}.ts, sdk/test/emit.test.ts,
  src/builtin_kind.rs, src/display.rs, src/drift.rs, src/extract.rs,
  src/read.rs, tests/display_rule.rs, tests/lock_declaration_rows.rs,
  tests/nested_member.rs, tests/read_verbs.rs — neither touches
  tests/session_start.rs, sdk/src/builtins.ts, or tests/coverage.rs, so all
  three `Kept on purpose` debts stay true; open-questions.md timestamps
  advanced to 52b3dcd.
  New residue found and routed (not standalone — rides the already-open
  entry): RETIRE-FOLD-MEMBERS deleted `CustomKind::fold_members` and its
  `parse_embedded_info`/`parse_embedded_member` helpers outright (grepped
  src/ — gone), but sdk/src/kind.ts:187 and sdk/src/emit.ts:68,90 still cite
  `parse_embedded_member`/`parse_embedded_info` as the mechanism reading a
  rendered fence back into leaves/members — stale against pipeline.md
  "Emit" (facts are lock declaration rows, never mined from rendering).
  Both lines live inside files EMBEDDED-KIND-RENDER-HOOK (open, next)
  already schedules to edit, so per the comment-staleness exception this
  rides that entry rather than filing standalone; annotated its two
  files.edit[] descriptions so build corrects both comments in passing.
  No other residue found: fold_members/parse_embedded* grep is clean
  elsewhere (only an accurate "retired" comment in tests/nested_member.rs
  and unrelated sdk/dist build output, gitignored); the ordered-collections
  Record<string, entry[]> shape from 7a3dfff is internally consistent
  end-to-end; display.rs's BTreeMap/fold module doc is still accurate to
  current code (EmbeddedMember.leaves is still a BTreeMap; "fold" still
  names the lock-row fold `builtin_kind::features` now performs, not the
  retired one).
- Queue: EMBEDDED-KIND-RENDER-HOOK (open, next — now also carries the
  stale-citation fix) → EMBEDDED-LEAF-TEXT (blockedBy); PACKAGING-CHANNELS
  (parked) unchanged. Disjoint: the two 0018 entries share
  sdk/src/kind.ts/emit.ts but are serialized by blockedBy, never both open;
  PACKAGING-CHANNELS shares nothing with them.

Plan continues: yes — quiet closing pass is next: inbox, spec-delta, and
ship-audit are all current as of this tick (nothing past 80697f8 has
touched src/tests/sdk besides this residue sweep's own bookkeeping commit),
and residue is now swept through HEAD.
