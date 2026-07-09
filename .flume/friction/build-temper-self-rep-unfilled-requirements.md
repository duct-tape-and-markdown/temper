## Symptom
SessionStart gate on this repo's own `.temper` self-representation reports 2
blocking findings, unrelated to any assigned entry:
- `requirement.unfilled: friction-capture-procedure` — no artifact declares a
  `satisfies` link naming it
- `requirement.unfilled: pending-entry-discipline` — same, for this requirement

A build-phase tick cannot fix these: `.temper/**` is not in the build phase's
writable paths, and `CLAUDE.md` is explicit that `build` never edits `.temper/`
or `.flume/`. Fixing requires a human `chore(harness):` session.

## Cost this tick
A prior attempt on this same entry exited with no commit solely to surface
this gap and ask for direction. This tick re-surfaces it via friction capture
instead and proceeds with the assigned entry, to avoid stalling unrelated
build work indefinitely on an out-of-scope gate.

## Suggested fix
A `chore(harness):` session should add the missing `satisfies` links in the
`.temper/` module(s) that implement friction capture and pending-entry
discipline, then re-run `temper emit` to clear both findings.
