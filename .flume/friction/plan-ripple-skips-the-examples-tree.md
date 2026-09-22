## Symptom

`.flume/ripple.mjs:13` walks four roots — `src`, `tests`, `sdk/src`, `sdk/test`
— so `examples/**` reaches no `<files-ripple>` line, even though `examples/**`
is inside build's writable fence (`.flume/chain.ts`, `BUILD_WRITABLE_PATHS`) and
holds a live in-tree consumer of the SDK's authoring surface.

The cost landed at 0c6a7ae0 (SDK-PROSE-SPAN-CONSTRUCTOR): the tick deleted
`export const span` from `examples/base-harness/.temper/kinds.ts` and did not
re-point the seven modules importing it. Node's ESM linker refuses a named
import of a missing export (`SyntaxError: The requested module '../kinds.ts'
does not provide an export named 'span'`, reproduced on node v22.21), so the
example's program has not loaded since — and nothing noticed, because no gate
runs the example either. Had the ripple analyzer walked `examples`, the seven
paths would have been on the entry's ripple line at derivation.

## Cost this tick

~15 minutes of plan's audit motion to find and prove it, plus one shipped-broken
artifact carried since 0c6a7ae0. The compile-gate half is filed as a pending
entry (EXAMPLE-PROGRAM-IMPORTS-A-DROPPED-SPAN); the blind spot that let it ship
is this capture's.

## Suggested fix

One line — add `examples` to the walk roots:

    for (const d of ["src", "tests", "sdk/src", "sdk/test", "examples"]) { … }

The walker already skips `node_modules`/`dist`/`target` and filters to
`.rs|.ts|.toml|.snap`, so the example's `.temper/*.ts` program files qualify as
they stand. If a dotted directory is skipped anywhere else in the ripple or
sizing tooling, the same blind spot applies — `examples/base-harness/.temper/`
is where the example's whole program lives.
