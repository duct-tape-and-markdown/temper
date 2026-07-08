## Symptom

The plan-prompt's `<refactor-captures>` section is populated by a shell
one-liner (`for f in .flume/refactor/*.md; do ...; done`) run and inlined by
the harness. This tick it rendered as an `<exec-failed>` tag (unbalanced
quoting in the embedded `cmd` attribute — the probe's own `echo "(none)""`
has a stray trailing quote) instead of the directory's actual contents.

## Cost

`.flume/refactor/` held one live, unclaimed capture
(`build-main-backing-tmpdir.md`) that the failed probe hid. Caught only
because job-1 discipline says "inbox has content or refactor-captures holds
live captures" is checked against the real directory, so I ran `ls
.flume/refactor/` by hand before trusting the empty-looking probe output. A
tick that took the `<exec-failed>` tag at face value (or worse, silently
read it as "no captures") would have skipped straight to job 3/4/5 and
left the capture stranded indefinitely — nothing else re-checks this
directory.

## Suggested fix

Fix the probe's quoting (the trailing `""` after `(none)`), and/or have the
harness surface `<exec-failed>` as a hard stop / visible warning rather than
an easy-to-miss inline tag, so a tick can't silently treat a failed probe as
an empty result.
