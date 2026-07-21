# Friction — the agent→human harness feedback channel

The mirror of `.flume/inbox.md`: inbox carries human notes to plan; this
directory carries agent-experienced friction to the humans who maintain the
two harnesses. An autonomous phase that hits real friction files ONE file
here; humans drain it out of band (session-open sweep), implement or route
the fix, and delete the file. Git is the archive.

The filing bar (exceptional, never a duty) and what qualifies as friction
live in the `capture-friction` skill; this README is the format and draining
reference.

## Format

One file per capture: `<phase>-<slug>.md` (`plan-…`, `build-…` — unique names
merge cleanly from parallel worktrees; a shared file would conflict and
revert the wave). Terse, three parts:

```
## Symptom
What happened, concretely.

## Cost this tick
Minutes, retries, reverted commits — the actual price paid.

## Suggested fix
One idea is enough. Solution class is the drainer's call: a rule, a skill,
a CLAUDE.md line, a prompt edit, a gate change — or an inbox item when the
fix is product work (e.g. lightening a slow test suite).
```

## Draining (humans/session)

Read, triage, implement or route (fix lands in whichever harness owns it —
`.claude/`+`CLAUDE.md` via `chore(harness):`, `.flume/` via `chore(flume):`,
product work via `.flume/inbox.md`), then DELETE the file. Same
anti-accumulation rule as open-questions: this directory holds live captures
only.
