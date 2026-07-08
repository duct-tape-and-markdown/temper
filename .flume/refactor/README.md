# Refactor — the agent→plan structural-debt channel

The sibling of `.flume/friction/`: friction carries harness feedback to
humans; this directory carries code-shape observations to the next `plan`
tick. An agent that notices structural debt it cannot take this tick — a
surface its entry's work duplicates, a hand-roll a sanctioned crate covers,
scaffolding copy-pasted once more — files ONE capture here; plan drains the
directory, verifies each claim at HEAD, files the verified gap as a pending
entry citing `specs/process/engineering.md`, and deletes the capture. Git is
the archive.

## The bar

Capture what you TOUCHED or READ this tick and could not fix in scope —
never a speculative redesign, never adjacent-code window shopping, never a
substitute for finishing the entry. The test is the engineering-shape
standard (`specs/process/engineering.md`): a second implementation of one
job, a hand-roll where the sanctioned set carries the mechanic, per-file
copies of shared scaffolding. Check the directory first; never re-file a
filed capture. Consolidation *inside* your entry's own surface needs no
capture — unifying a duplicate you would otherwise extend is in scope.

## Format

One file per capture: `<phase>-<slug>.md` (`plan-…`, `build-…`,
`session-…` — unique names merge cleanly from parallel worktrees). Terse,
three parts:

```
## Surface
The duplication/hand-roll, concretely: every home, file:line each.

## Observed at
<short-sha> (HEAD when observed) — plan diffs forward from here.

## Suggested consolidation
One idea is enough — the home you'd keep, or the crate that replaces it.
```

## Draining (plan, inbox job)

Verify the claim at HEAD (the tree moved since filing; `observed at` bounds
the diff), file the verified gap as a pending entry with a `per` cite into
`specs/process/engineering.md`, DELETE the capture. A claim that no longer
holds is deleted with a note in the plan commit body. Same anti-accumulation
rule as friction and open-questions: live captures only.
