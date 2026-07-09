# Plan state

- Spec derived through: a53eee4
- Audited through: 9bf90bc
- Residue swept through: 3c6f50b
- This tick: Residue sweep. Only src/tests/sdk-touching commit since 52b3dcd
  is 3c6f50b (EMBEDDED-KIND-RENDER-HOOK, sdk/src/emit.ts + sdk/src/kind.ts +
  sdk/test/emit.test.ts only). Re-verified both standing debts unaffected
  and still live: tests/session_start.rs's `+++`-format kinds/packages
  fixtures, and sdk/src/builtins.ts:308,348,385's three deleted PACKAGE.md
  citations — neither file touched by 3c6f50b or its two predecessors since
  52b3dcd. Also confirmed 3c6f50b's own commit-body fix (stale
  parse_embedded_member/parse_embedded_info citations) is complete: zero
  remaining hits repo-wide. New finding: sdk/src/kind.ts (12 hits) and
  sdk/src/contract.ts (8 hits) doc-comment-cite four numbered spec files
  (10-contracts.md/15-kinds.md/20-surface.md/40-composition.md) that predate
  0002's corpus-form reorg and no longer exist under those names — vocabulary
  the corpus no longer sanctions, compounded by rust.md/sdk.md's "spec
  citations retired from comments" convention. Per job 4's own exception,
  citation staleness never gets a standalone entry — recorded as a new
  "kept on purpose" debt in open-questions.md, riding the next entry that
  opens each file (kind.ts's exit clause already fires under
  EMBEDDED-LEAF-TEXT, which has it in its edit list; contract.ts awaits its
  own next touch). pending.json unchanged — no entry filed.
- Queue: EMBEDDED-LEAF-TEXT (open, next); PACKAGING-CHANNELS (parked).
  Disjoint — EMBEDDED-LEAF-TEXT is the sole open entry.

Plan continues: yes — every cursor (spec derived, audited, residue swept) now
sits at or past the latest relevant commit and the queue is disjoint; the
next tick is the quiet closing pass (job 5).
