## Surface

With the `settings` container now embedded beside `mcp-server`, **every**
`KNOWN_SURFACES` row (`src/builtin_kind.rs:87`) is governed whole by a built-in kind, and
`coverage_note::check` is always called with the full built-in set (`src/gate.rs:662`). So
two branches of `src/coverage_note.rs` are unreachable through the gate:

- `SegmentCoverage::Partial` / `SegmentCoverage::Wholly` (`src/coverage_note.rs:146-167`)
  — `whole` is true for both surfaces under any real invocation, so the
  `coverage.unmodeled-surface` rule never fires in production. `manifest_top_level_keys`
  and the whole `segments`/`Segment` model (`src/builtin_kind.rs:51-67`) exist only to
  decide that verdict.
- the locked-kind suppression `with_locked_kinds` feeds (`src/coverage_note.rs:119`) —
  a locked custom kind cannot change a surface's verdict that a built-in already governs
  whole, and a second document kind at one of those loci is now a
  `kind.governs-collision` (`src/admissibility.rs:522`) rather than a suppression.

Three tests had to withhold a built-in from scope to keep observing either branch
(`tests/coverage_note.rs:102`, `:376`, `tests/check_cost.rs:345`); that withholding is the
tell. The `coverage.unclaimed-entry` strand and the `coverage.checked` disclosure are
unaffected and still live.

## Observed at

2dba7c0c (HEAD when observed).

## Suggested consolidation

Decide whether `coverage.unmodeled-surface` is retired now that the std-lib governs every
surface it names — subtraction would take the rule, `KNOWN_SURFACES`'s `segments` column,
`Segment`, `manifest_top_level_keys` and `with_locked_kinds` with it — or whether the
registry is meant to grow rows for surfaces no built-in governs (a `.claude/` surface the
docs name and no kind models), which would keep it live and is the corpus question.
