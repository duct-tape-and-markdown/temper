# Plan state

- Spec derived through: a53eee4
- Audited through: 80697f8
- Residue swept through: a3f9f1f
- This tick: Ship audit. Commits past f000b97 touching `src/`/`sdk/`:
  7a3dfff (build: preserve authored order in nested-member collections) and
  80697f8 (chore(flume): ship NESTED-MEMBER-COLLECTIONS-ORDERED). Verified
  on disk, not just the log: `git show --stat` on both, then read
  `sdk/src/kind.ts` — `EmbeddedMemberValue.collections` is now `Readonly<
  Record<string, readonly EmbeddedMemberCollectionEntry[]>>`, each entry an
  ordered `{key, leaves}` — matching the shipped summary exactly. Ran the
  full gate set live: `cargo test` (all 7 suites green), `pnpm --dir sdk
  test` (50/50 green), `tsc --noEmit` (clean) — the ship is sound, not just
  claimed. `pending.json` already had the shipped entry removed by build's
  own ship commit (80697f8) — nothing to drop this tick.
  Re-tested the one stale gate this ship changes: EMBEDDED-KIND-RENDER-HOOK
  was `blockedBy NESTED-MEMBER-COLLECTIONS-ORDERED`; its blocker is now
  shipped and gate-green, so it flips to `open`. Its cited file/line refs
  (`kind.ts` `kind()` ~155, `emit.ts` `renderMemberFence`/`resolveBody`
  ~93-122) still resolve post-ship — re-grepped, unchanged. EMBEDDED-LEAF-TEXT
  stays `blockedBy EMBEDDED-KIND-RENDER-HOOK` (that slice hasn't shipped);
  its own `tests[]` cite to `tests/nested_member.rs`
  `leaf_addresses_are_structural_member_kind_key_child_path` had drifted
  from `~94` to `~118` (the shipped rework added lines ahead of it in the
  same file) — corrected in place, entry otherwise unchanged.
  PACKAGING-CHANNELS untouched: unrelated to this ship, no re-verification
  triggered. Spec-delta and residue-sweep cursors copied forward verbatim —
  neither job serviced this tick.
- Queue: EMBEDDED-KIND-RENDER-HOOK (open, next) → EMBEDDED-LEAF-TEXT
  (blockedBy); PACKAGING-CHANNELS (parked) unchanged. Disjoint: the two
  remaining 0018 entries share `sdk/src/kind.ts`/`emit.ts` but are
  serialized by `blockedBy`, never both `open`; PACKAGING-CHANNELS shares
  nothing with them.

Plan continues: yes — residue sweep is next live: `Residue swept through`
(a3f9f1f) trails HEAD (80697f8), and inbox/spec-delta/ship-audit are all
quiet as of this tick.
