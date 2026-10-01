## Surface

`LEAF-MENTION-ROW-HEADS-ITS-HOST-ADDRESS` is a complete, green change whose
fence is one file short. The code half is three edits, all in-fence:

- `sdk/src/declarations.ts` — `mentionRows` hands `embeddedLeafMentionRows` the
  `address` it already computed instead of `member.name`; the helper's first
  parameter becomes the host's address.
- `sdk/src/member-address.ts` — `leafAddress`'s and `LeafAddress.member`'s docs
  restate the head as the host's own address, bare id demoted to a legacy-lock
  spelling the reader still accepts.
- `src/read.rs` — `resolve_leaf`'s head comment re-credits its two branches
  (equality now serves a nested host, whose `features.id` *is* its whole
  address, plus a pre-canonical-head lock); behaviour unchanged.

Verified locally with those edits applied: `pnpm --dir sdk test` is 235/236,
the one failure being **out of fence** —

- `sdk/test/emit.test.ts:1408` ("a leaf's mention contributes a mention row
  keyed to the leaf's own structural address"), whose `assert.deepEqual` at
  **1434–1435** pins the bare-name head: `CLAUDE/decision/surface-authority/chosen`
  and `CLAUDE/decision/surface-authority/rejected.baked-projection.because`.
  Both respell to `memory:CLAUDE/…` — the entry's whole point, so this is the
  pin moving with the contract, not a regression.

`cargo test --no-fail-fast` is 841/842, and that one failure is **in** fence:
`emit::emit_program_runs_the_shipped_example_harness`, drift in
`examples/base-harness/.temper/lock.toml` where two rows respell
`summarize/step/{scan,summarize}/in` → `flow:summarize/step/{scan,summarize}/in`.
Regenerable in-tick; nothing else in `src/`, `tests/` or the repo's own
`.temper/lock.toml` (which carries zero mention rows) moves.

So the complete set of files the entry needs is `entry.files` ∪
`{sdk/test/emit.test.ts}`.

## Observed at

0e419c2d (HEAD of `flume/primary/leaf-mention-row-heads-its-host-address`).

## Suggested consolidation

Re-scope the entry with `sdk/test/emit.test.ts` added to `files.edit` —
"the leaf-row pin at 1434 respells to the host's `memberAddress`, the same
head `mention.test.ts`'s new addressing cases read at both grains" — and
`examples/base-harness/.temper/lock.toml` named in `files.edit` rather than
left to the fence's glob, so the re-emit is part of the entry's text. No other
change; the work is otherwise done and proven.
