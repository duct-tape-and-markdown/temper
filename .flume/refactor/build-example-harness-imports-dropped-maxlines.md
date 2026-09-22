## Surface

`EXAMPLE-PROGRAM-IMPORTS-A-DROPPED-SPAN` is implementable exactly as scoped — the
seven `span` imports re-point, every call site is unchanged, and the entry's test
is the right gate — but it cannot reach green inside its fence. The example
program carries a **second** dropped-symbol import, in the one module the fence
does not list:

- **`examples/base-harness/.temper/harness.ts:1`** — imports `maxLines` from
  `@dtmd/temper`, which no longer exports it. `4bdb7960` ("derive extent, retire
  max_lines into it") replaced the predicate with `extent(unit, bound)` and
  re-pointed the three shipped budgets, but no example consumer. Call site `:90`,
  a `severity: "advisory"` clause on the `system` kind.

This is the entry's own break in a second symbol, not an adjacent one: `harness.ts`
is the emit entry point, so node dies at *its* link error
(`does not provide an export named 'maxLines'`, node v22.21) before any member
module loads. The entry's test — `drift::emit_program` over
`examples/base-harness/.temper`, asserting `Ok` — therefore fails identically
before and after the seven `span` fixes. There is no green subset: the whole test
is blocked, not a rider.

Both breaks are one class, and the already-filed
`.flume/friction/plan-ripple-skips-the-examples-tree.md` names the cause — an SDK
retirement draws no ripple line to an example consumer, so the example program has
been dead since `4bdb7960` (17 Jul) and no gate noticed.

## Verified, out of tree

With both fixes applied locally (`span` from `@dtmd/temper` in the seven modules;
`maxLines(120)` → `extent("lines", 120)`, the same budget re-spelled the way
`4bdb7960` re-spelled the shipped three), against the built SDK vendored at
`examples/base-harness/.temper/node_modules/@dtmd/temper`:

- `node harness.ts` links, composes, and writes the seam JSON.
- `temper emit --into examples/base-harness/.temper --dry-run` reports
  **11 unchanged, 0 emitted** — every projection and the lock are byte-current.
  The re-emit fence the entry carries (`CLAUDE.md`, the two `SKILL.md`s, the four
  `docs/` projections, `lock.toml`) is never exercised: the shipped `span`'s
  dedent is identity over every call site, as the entry predicted.

So the re-scoped entry is the eight source edits plus the test; no projection moves.

## Observed at

4dd4e40b (HEAD when observed)

## Suggested consolidation

Re-scope the entry with `examples/base-harness/.temper/harness.ts` added to
`files.edit` — description: `:1` drops `maxLines` for `extent`, call site `:90`
becomes `clause(extent("lines", 120), …)`, the same 120-line budget under the
predicate that subsumed it (`4bdb7960`); the guidance string wants the same
render-side rewording that commit gave the shipped budgets. Keep the seven `span`
edits and the `tests/emit.rs` case unchanged, and keep the projection paths in the
fence as the re-emit escape hatch even though the dry run says nothing moves.
