# Plan state

- Spec derived through: f87cc0c
- Audited through: dd7517a
- Residue swept through: 8dfceee
- This tick: Residue sweep (job 4). Rechecked fba4e32..8dfceee (33874ac build
  + dd7517a chore-ship, both already confirmed shipped by the prior ship
  audit). 33874ac's rewrite touched only src/main.rs and
  tests/requirement_roster.rs — no duplicate surface left behind, and the two
  retargeted roster tests now exercise a genuinely unmodeled kind (`widget`)
  rather than `command`, verified sane on read. Both standing accepted debts
  reconfirmed live verbatim: tests/session_start.rs still writes `+++`-format
  `.temper/kinds|packages/spec/{KIND,PACKAGE}.md` fixtures; sdk/src/builtins.ts
  still doc-comment-cites three deleted `packages/*/PACKAGE.md` files.
  sdk/dist/ mirrors of both are gitignored build output, not residue. Grepped
  for other retired vocabulary (published_requirements, own-path surface) —
  only expected historical comments remain, no live residue. No new fileable
  gap; pending.json unchanged.
- Queue: PACKAGING-CHANNELS parked, sole entry.

Plan continues: yes — quiet closing pass (job 5) is next; residue cursor now
at HEAD, all four prior inputs current.
