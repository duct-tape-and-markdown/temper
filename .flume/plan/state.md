# Plan state

- Spec derived through: a53eee4
- Audited through: f000b97
- Residue swept through: a3f9f1f
- This tick: Inbox job. Routed 6bed388's carried note (0018's remaining
  scope) into three independently-green entries per its own stated
  sequencing: NESTED-MEMBER-COLLECTIONS-ORDERED (open — `collections`
  becomes an ordered `{key, leaves}` list, SDK + Rust + the lock row, per
  `pipeline.md` "The lock"), EMBEDDED-KIND-RENDER-HOOK (blockedBy slice 1 —
  an optional SDK-side render hook, per `representation.md` "kind"'s
  writer-only/unconstrained embedded format), EMBEDDED-LEAF-TEXT (blockedBy
  slice 2 — leaves may carry `Text` + mentions, per `contract.md` "edge").
  All three verified live against HEAD (6bed388) before scoping: grepped
  `EmbeddedMemberValue.collections`/`NestedMemberRow.collections` (still
  keyed maps), grepped for a `render` hook (none beyond unrelated
  `renderText`/`renderMember*`), grepped `Text` in `kind.ts` (absent from
  leaves). The inbox's specific claim of a testbed
  `1-classify`/`2-validate-currency` ordering hack does **not** resolve
  anywhere in `src/`, `tests/`, or `sdk/` at HEAD — dropped from entry scope
  as unverified rather than encoded as fact (noted in
  NESTED-MEMBER-COLLECTIONS-ORDERED's `notes`). Slices 2 and 3 are
  `blockedBy` their predecessor purely on entry-discipline file-overlap
  (all three touch `sdk/src/kind.ts`/`emit.ts`), matching the inbox's own
  1→2→3 order even though slice 3 has no semantic dependency on slice 2.
  Inbox line removed; `.flume/refactor/` re-confirmed empty (README only).
  Spec delta, ship audit, and residue-sweep cursors re-checked and stay
  current — no `specs/` commits past a53eee4, no `src`/`tests`/`sdk`
  commits past f000b97 or a3f9f1f (both empty `git log` ranges).
  open-questions.md untouched — no fork implicated. `cargo check` green.
- Queue: NESTED-MEMBER-COLLECTIONS-ORDERED (open, next) →
  EMBEDDED-KIND-RENDER-HOOK (blockedBy) → EMBEDDED-LEAF-TEXT (blockedBy);
  PACKAGING-CHANNELS (parked) unchanged. Disjoint: the 0018 chain shares
  files across its own three entries but they're serialized by `blockedBy`,
  never both `open`; PACKAGING-CHANNELS shares nothing with them.

Plan continues: no — spec delta, ship audit, and residue sweep are all
current as of HEAD (6bed388), and the inbox is now drained. Build takes
NESTED-MEMBER-COLLECTIONS-ORDERED; the loop hibernates on plan's side until
the inbox, a spec commit, or a src/tests/sdk commit gives it new work.
