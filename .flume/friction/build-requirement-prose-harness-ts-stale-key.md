## Symptom
REQUIREMENT-PROSE-PERSISTS renamed `Requirement.means` to `Requirement.prose`
SDK-and-engine-wide (`sdk/src/contract.ts`, `src/compose.rs`, `src/drift.rs`,
`src/main.rs`, plus rename ripple). `.temper/` is outside the build phase's
writable paths, so `.temper/harness.ts` was left untouched — but it authors
two requirements with the retired key: `requirement({ means: "...", ... })`
around lines 26 and 31. `means` is no longer a field the SDK's `Requirement`
type carries; the harness runs un-type-checked at emit time, so the stray key
rides along silently rather than erroring. Both requirements' authored intent
still never reaches the lock — the exact bug this entry fixes, now for a new
reason (wrong key) instead of the old one (no persistence path at all).

## Cost this tick
Zero rework — caught by grepping for `means` repo-wide after the rename and
recognizing `.temper/harness.ts` is out of scope for a `build:` commit. Cost
is entirely deferred: a human `chore(harness):` pass has to catch this before
the two requirements' prose is actually load-bearing.

## Suggested fix
Rename `means:` to `prose:` in `.temper/harness.ts`'s two `requirement({...})`
calls (~lines 26, 31) and re-run `temper emit`. Longer term: since `.temper/`
authors against the same SDK types `sdk/src/**` exports, a rename entry like
this one could name `.temper/harness.ts` as a required companion edit in its
own `chore(harness):` commit rather than relying on a friction capture to
surface it — but that commit is human territory, never `build`'s to make.
