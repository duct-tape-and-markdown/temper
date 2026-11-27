# SEVERITY-READS-REQUIRED-OR-ADVISORY: wire levels kept

The entry pointed at reporter.rs:398/:414 (`sarif_level`, `workflow_command`).
Those are SARIF 2.1.0's closed `level` set and GitHub's workflow-command names.
Respelling them would make the output invalid SARIF and give GitHub annotations
it silently drops. The spec's own rule covers this case: where the runtime names
a thing, temper uses the runtime's name in the runtime's sense. So they stay,
and `it__gauntlet__check_diagnostics.snap` (github `::warning` lines) is unchanged.

What shipped instead:
- `check::Severity::word()`: required / advisory / disclosure.
- The terminal render's help line leads with `severity: <word>`.
- Each SARIF result carries `properties.severity`.
- CLI help: `check` reads "Gate", and "(warn-severity)" is gone.

Left for plan:
- The GitHub reporter still shows only GitHub's own words. Putting temper's word
  in `title=` or the body would re-shape a line format about 18 test files parse.
  Call it if wanted.
- The session-start heading "Blocking findings:" is not error/warn, so it stayed.
- The internal `Severity::Error/Warn` and `Diagnostic::error/warn` names are not
  author-facing, so they were not renamed.
- The migration-note line (docs/) is still owed by the release cut.
