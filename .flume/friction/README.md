# Friction — the agent→human harness feedback channel

The mirror of `.flume/inbox.md`: inbox carries human notes to plan; this
directory carries agent-experienced friction to the humans who maintain the
two harnesses. An autonomous phase that hits real friction files ONE file
here; humans drain it out of band (session-open sweep), implement or route
the fix, and delete the file. Git is the archive.

## The bar

Capture is **exceptional, never a duty** — most ticks file nothing. A capture
must name friction you actually hit THIS tick, with its actual cost. Never
speculative improvement, never flattery, never a substitute for finishing the
job. Check the directory first: if your friction is already filed, don't
re-file it.

What qualifies — the test is *recurrence × cost*, something an agent likely
hits every session where a simple capture would spare the next one:

- a pitfall the harness could have warned about (you burned time discovering
  a fact a rule/CLAUDE.md line could state),
- a disproportionately lengthy process (tests too slow → suggest lightening
  them; a gate that mostly waits),
- missing operational knowledge (a command, a location, a convention you had
  to reverse-engineer),
- a gate or fence firing on a false positive.

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
