# Plan state

- Spec derived through: 048f31f
- Audited through: a641e03
- Residue swept through: a561e70
- This tick: Spec delta. Two specs/ commits landed past the recorded cursor
  (a53eee4) mid-tick, from a concurrent interactive ratification session:
  d0e4fda (retire the cold-read ceremony — spec-system.md's Change ceremony
  reworded: the `specs:` commit stays the session's, human in the loop, but
  the verification duty is the session's own, exercised before landing, not
  a separate human cold read after) and 048f31f (intent gains a sixth
  invariant, "Loud or nothing" — decisions/0019-loud-or-nothing.md). Both
  fully routed, no new entries: d0e4fda's downstream already shipped as
  chore commits in the same window (bb5bb2e reworded
  `.claude/rules/collaboration.md` — and its `.temper/` source plus lock
  row; 26b66f5 reworded `.flume/chain.ts`'s writablePaths comment) —
  grepped `specs/`, `.flume/plan/`, `.flume/prompts/` for "cold read"
  residue: none left. 048f31f's own Decision text keeps the per-surface
  refusal clauses as the binding spec text and declines to retrofit
  existing surfaces ("Patch the instance only" is a rejected alternative);
  its motivating gap (EMBEDDED-KIND-RENDER-HOOK's render hook bypassing
  mention resolution) is the already-queued RENDER-HOOK-LEAF-RESOLUTION
  entry, so nothing new derives from it this tick. Re-confirmed ship audit
  and residue-sweep cursors both still trail HEAD with no new src/tests/sdk
  commits in the window (bb5bb2e/26b66f5/9c3b1c1 touch only
  `.claude`/`.temper`/`.flume`/`docs`), so neither job displaces this one.
  Found mid-tick but NOT filed (one job per tick): `sdk/src/assembly.ts:19`'s
  `ExpectBinding.kind` is still typed `KindDefinition<object>` — the
  identical contravariant-assignability bug a620938 fixed on
  `Requirement.kind` (Decision 0003's stranded SDK-side half). Verified live
  via a scratch tsc check: `expect: [{ kind: skill, clauses: [] }]` fails
  TS2322 ("Property 'description' is missing in type '{}' but required in
  type 'Skill'"), the same shape `Requirement.kind` threw before a620938.
  a620938 only retyped `contract.ts`'s sibling field; `assembly.ts`'s
  `ExpectBinding` was never touched. Ready for next tick's residue sweep to
  file (fix mirrors a620938: retype to `KindDefinition<never>`;
  `declarations.ts` reads only `binding.kind.key`/`.facts.name`, so no
  Rust-side change, same as the precedent).
- Queue: RENDER-HOOK-LEAF-RESOLUTION (open), PACKAGING-CHANNELS (parked on
  human release creds + the engine-binary workflow, unchanged this tick).

Plan continues: yes — residue sweep is next (Residue swept through a561e70
trails HEAD; the window carries 0f5fcd0 and a620938 as before, plus the
newly-routed specs commits and their chore-commit reflections, none
code-bearing, plus the ExpectBinding.kind finding above ready to file).
