Drift repair is the flow that runs when disk and lock disagree: a governed
source moved, or a committed projection was hand-touched. Either way the
finding names the owning member, the side that moved, and the remedy — the
flow is the remedy carried out.

## Trigger

`temper check` (at a gate placement: terminal, CI, session start) reports a
drift finding against the lock.

## Participants

- gate
- corpus

## Steps

1. Read the finding: it names the member that owns the bytes and which side
   moved.
2. A moved source is re-emitted (`temper emit`), refreshing its provenance
   row. A hand-touched projection is never merged: the edit is re-made on
   the owning source (the `.temper/` module or the layout document), then
   re-emitted.
3. `temper check` confirms the pair is clean. Nothing is reconciled
   silently; a drifted ownerless file stays a finding until an author rules
   on it.
