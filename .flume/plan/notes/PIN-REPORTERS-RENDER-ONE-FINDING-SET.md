# PIN-REPORTERS-RENDER-ONE-FINDING-SET

The claim holds. Pinned in tests/it/reporters.rs: one error/warn/note set goes
through terminal, github, sarif and session-start, and each output is parsed back
to (severity, rule, artifact, message) and compared. Terminal (miette graphical)
is prose, so it is checked per field (each rule appears exactly once) rather than
parsed as structured output.
