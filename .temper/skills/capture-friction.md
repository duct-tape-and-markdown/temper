# Capture friction or structural debt

Two sibling channels, one skill: `.flume/friction/` carries agent→human
harness feedback; `.flume/refactor/` carries agent→plan structural-debt
observations. Same bar, same shape; only the target directory, filename
prefix, and drain audience differ. Check the target directory first — never
re-file an already-filed capture.

**The bar, both channels: exceptional, never a duty.** Most ticks/sessions
file nothing. A capture names something you actually hit or touched THIS
tick, with its actual cost — never speculative, never a substitute for
finishing the job.

## Friction — `.flume/friction/<phase>-<slug>.md`

What qualifies: a pitfall the harness could have warned about, a
disproportionately lengthy process, missing operational knowledge, or a gate
firing on a false positive — recurrence × cost, something the next
session/tick would likely hit too.

```
## Symptom
What happened, concretely.

## Cost this tick
Minutes, retries, reverted commits — the actual price paid.

## Suggested fix
One idea is enough — a rule, a skill, a CLAUDE.md line, a prompt edit, a gate
change, or an inbox item when the fix is product work.
```

Humans drain it out of band (session-open sweep): implement or route the fix
into whichever harness owns it, then delete the file. Full bar and format
detail: `.flume/friction/README.md`.

## Refactor — `.flume/refactor/<phase>-<slug>.md`

What qualifies: a second implementation of one job, a hand-roll a sanctioned
crate covers, scaffolding copy-pasted once more — the engineering-shape
standard (`specs/process/engineering.md`). Consolidation *inside* your own
entry's surface needs no capture; this channel is for debt you touched or
read but couldn't fix in scope.

```
## Surface
The duplication/hand-roll, concretely: every home, file:line each.

## Observed at
<short-sha> (HEAD when observed) — plan diffs forward from here.

## Suggested consolidation
One idea is enough — the home you'd keep, or the crate that replaces it.
```

`plan`'s inbox job drains it: verify the claim at HEAD, file a pending entry
citing `specs/process/engineering.md`, delete the capture. Full bar and
format detail: `.flume/refactor/README.md`.
