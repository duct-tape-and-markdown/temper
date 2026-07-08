## Surface

The version-stamped seam encoding — `JSON.stringify({ version: SEAM_VERSION,
... }, null, 2) + "\n"` — hand-inlined twice in the SDK:
sdk/src/declarations.ts:377 `declarationsToJson` and inside `emit`
(sdk/src/emit.ts, ~:305).

## Observed at

0ccba8d

## Suggested consolidation

One `encodeSeam(payload)` helper; both call sites use it, so the version
stamp and trailing-newline convention have one home.
