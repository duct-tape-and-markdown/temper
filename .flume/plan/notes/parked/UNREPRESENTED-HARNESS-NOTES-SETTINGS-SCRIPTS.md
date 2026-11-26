# UNREPRESENTED-HARNESS-NOTES-SETTINGS-SCRIPTS: negative tests[] line is green on base by construction

Parked, not built. The prior attempt (4e4383b8) implemented the note and
was reverted by the named-lines gate at afterMerge: tests[] line "an
unrepresented harness with no hook or status line command draws no script
note" already passes at 33f12cf. It must: the base tree emits no script
note anywhere, so an absence assertion holds before the fix. No honest
test of that title can fail on base, and build may not restructure a test
to make it fail. Re-dispatching the entry unchanged reverts again.

Fix for plan: move the negative line from tests[] to pins[] (a property
that already holds and gains its check here), keeping the positive line
"an unrepresented harness whose settings hold a hook command draws the
script note" in tests[] (proven red-on-base last attempt). The existing
pin "the claude worktrees unclaimed entry quotes the docs gitignore advice"
was judged proven and can stay.

The reverted implementation was sound and small (src/coverage_note.rs
+59, src/gate.rs +3, tests/it/coverage_note.rs +76); the next tick can
rebuild it from `git show 4e4383b8`.
