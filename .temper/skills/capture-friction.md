# Capture friction, structural debt, or a harness amendment

Three sibling channels, one skill: `.flume/friction/` carries agent→human
harness feedback; `.flume/refactor/` carries agent→plan structural-debt
observations; `.flume/amendments/` carries agent→human apply-ready harness
diffs (0044). Same bar, same shape; only the target directory, filename
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

## Amendment — `.flume/amendments/<phase>-<slug>.md`

What qualifies: a harness change you can ground in a named invariant and a
cost you actually paid, expressed as an **apply-ready unified diff** — a
prompt edit, a rule re-cut, a capture-format fix. A proposal that still
needs drafting is friction, not an amendment. In-scope targets and the
constitution's exclusions: the directory README. Friction's full bar
applies unchanged.

```
## Invariant served
The named invariant or posture, and the observed cost at <short-sha>.

## Diff
One fenced unified diff, apply-ready.

## Expected settling
What the record should show if this works.
```

Humans drain it: ratify with one word (apply verbatim, re-emit projections
if the target was `.temper/`) or decline with the reason in the deleting
commit. Full format detail: `.flume/amendments/README.md`.
