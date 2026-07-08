## Symptom

Implementing CI-DOCUMENTED-TWO-LINE-JOB (rewrite `.github/workflows/temper.yml`
onto `check` + `emit --frozen` + byte-compare) requires validating the new job
actually goes green (the entry's own acceptance bar). Running the real
sequence (`cargo run -- check`, `cargo run -- emit --frozen`, `git diff
--exit-code`) against this repo's own committed `.temper/` surface fails: the
committed `.temper/lock.toml`'s `CLAUDE` memory row carries a stale
`source_hash`/`emit_hash` (`354721e5…`) that does not match `./CLAUDE.md`'s
actual bytes (`sha256sum` gives `5fb34b5f…`) — confirmed the file is
byte-identical back through `b29d7dc` (the commit that wrote both), so the
lock was already wrong the moment it was committed, not a later regression.
Separately, `registration` rows are still the pre-0013-recut scalar form
(`"always"`) where the current engine now writes an array (`["always"]"`,
landed in `1076bf3`) — expected staleness from a schema change nobody
re-emitted `.temper/` against.

## Cost this tick

~30 minutes of investigation (git blame across the SDK version-skew window,
hashing the file at two commits, reading `sdk/src/prose.ts`/`hash.rs` to rule
out a code regression) to confirm this is pre-existing drift in a path build
cannot touch (`.temper/**`, `CLAUDE.md` are outside the writable-paths list —
human `chore(harness):` territory per `CLAUDE.md`'s own rule). The corrected
CI job is implemented and correct, but will fail on its first real run until
someone re-runs `temper emit` over `.temper/` and commits the refreshed lock
— a chore(harness) commit, not a build tick.

## Suggested fix

Route to `.flume/inbox.md` as product/dogfood work (per `CLAUDE.md`: "Friction
the dogfood surfaces routes to `.flume/inbox.md`, never into hand-patched
`src/`"): re-run `temper emit --into .temper` at repo root and commit the
refreshed `.temper/lock.toml` under `chore(harness):`, so the new CI gate this
entry ships actually goes green. Do this before or alongside merging this
entry's PR — until then the new (correct) workflow will show red on the
byte-compare step, not because the CI wiring is wrong but because the
dogfood surface it checks was never fully reconciled after the 0013
registration recut.
