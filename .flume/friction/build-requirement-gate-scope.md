## Symptom
REQUIREMENT-GATE's `files.edit` named `src/check.rs`/`src/engine.rs`, but the
count/unique/membership/kind clause evaluator it describes as missing was
already fully implemented and wired into `check` (`src/roster.rs` +
`src/graph.rs::degree`, called from `gate()` in `src/main.rs`, landed by an
earlier `REQUIREMENT-CLAUSES-RECUT` tick). `tests/requirement_roster.rs`
already had 19 green tests proving the two-step `check` path fires on a
locked `count`/`unique`/`membership`/`kind` violation. The real gap, found
only by reproducing the field report's exact probe end-to-end, was narrower:
`check --harness <path>` (the one-shot wedge) read declarations off
`<path>/lock.toml` instead of `<path>/.temper/lock.toml`, so an
already-emitted harness's locked requirement/satisfies rows were silently
never read in one-shot mode — `session_start`'s reporter already had the
correct authored-`.temper`-detection branch the plain `--harness` dispatch
lacked.

## Cost this tick
~30 minutes reading roster.rs/graph.rs/drift.rs/main.rs and running the
existing test suite before concluding the described gap was already closed,
then writing a throwaway repro test to isolate the actual defect (a one-line
dispatch bug in `main.rs`, nowhere in the entry's named files).

## Suggested fix
Before scoping an entry whose summary claims "X is never evaluated," `plan`
should grep for the claimed-missing evaluator (here, `roster::check`'s
`Predicate::Count` arm already existed) and run the cited acceptance probe
against the *current* tree, not just the field report's original repro
environment — the gap may have narrowed or moved since the report was filed.
