# UNREPRESENTED-HARNESS-NOTES-SETTINGS-SCRIPTS: rebuilt from 4e4383b8 after the pin re-scope

Shipped by re-applying the reverted 4e4383b8 implementation unchanged; the
re-scope (557a0524) moved the negative line to pins[], which removes the
only reason it reverted. `coverage_note::script_note` adds one
`coverage.unrepresented-scripts` note on an unrepresented harness whose
settings.json or settings.local.json holds a `command` hook handler or a
`command` statusLine; it names and counts no script.

The stale park note at notes/parked/UNREPRESENTED-HARNESS-NOTES-SETTINGS-SCRIPTS.md
(7a011565) is left for plan's drain: this commit does not touch it, so the
park signal is not re-raised.
