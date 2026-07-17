## Symptom

EXAMPLE-EDGE-TARGET-SET-SPELLING needed a re-emit of the nested harness at
`examples/base-harness`. The entry named the bootstrap
(`pnpm -C sdk install && pnpm -C sdk build`, `npm -C … install`) but not the
cwd, and `emit` takes `--into <workspace>` — which reads as "emit that
workspace from anywhere". It isn't.

Run from the repo root:

    temper emit --into examples/base-harness/.temper

exits 0, reports `1 orphan-drift  memory  CLAUDE`, and rewrites **every**
`source_path` row in the example's lock from harness-relative to cwd-relative
(`docs/systems/renderer.md` → `examples/base-harness/docs/systems/renderer.md`)
— 20 corrupted rows alongside the 2 intended ones. Run with cwd =
`examples/base-harness`, the same emit reports `0 orphan-drift` and touches
only the 2 rows the change actually owed.

The failure is silent in the shape that matters: exit 0, and the lock is
tool-written whole, so the corruption arrives looking exactly like a
legitimate canonical rewrite. The `orphan-drift` line is the only tell, and it
reads as a finding about the tree rather than about the invocation. Caught here
only because the entry predicted a 2-row diff and the diff was 24.

## Cost this tick

~5 minutes and one `git checkout` of the lock — small, because the entry
happened to state the expected diff size. Absent that prediction this commits
clean and green: no gate reads `examples/`, and every projection still
verifies, so a wrong-cwd emit corrupts the lock's provenance with nothing
downstream to notice.

Recurrence is structural, not incidental: this entry exists because a model
widening left the exemplar behind, that class will recur on the next widening,
and every instance ends in a re-emit of a nested harness from a root-cwd
session.

## Suggested fix

Inbox item — product work, not a doc line. `emit` already knows the workspace
root (`--into` names it); resolving projection paths against cwd instead is
the bug. Either resolve `source_path` against the `--into` root, or refuse a
`--into` whose root isn't cwd rather than silently re-basing 20 rows.

If it stays a footgun, the cheap stopgap is a CLAUDE.md/rule line next to the
existing `.temper/` bootstrap: run `emit` with cwd set to the harness root,
never via `--into` from elsewhere.
