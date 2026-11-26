# NO-LOCK-GUARD-CALLS-NOTHING-A-PROJECTION

Shipped: `install::guard` now allows every write when `targets` is `None`
(no lock); the `.claude/` substring fallback and `is_claude_path` are gone.
Tests: the flipped no-lock test is
`a_no_lock_write_to_a_skill_draws_no_projection_warning`; the pin is
`with_a_lock_a_write_to_a_lock_named_projection_is_still_guarded`. The
no-lock tail of the governed-locus test in `tests/it/install.rs` flipped too
(it asserted the fallback bound `.claude/rules/stray.md`).

Not built: the optional "at most one session-scoped note that the harness
is unrepresented". The summary allowed zero, and the guard is per-call and
holds no session state, so a once-per-session note would need a session key
(the tap log, or a SessionStart surface) that the entry names nowhere. If
that note is wanted, it belongs on a session-open edge, not on
`PreToolUse`. That takes its own entry.
