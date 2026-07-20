---
paths: [".flume/plan/state.md"]
---
# plan-state — the scheduler's ledger

`state.md` is re-derived every tick, ~10 lines:

```
# Plan state
- Spec derived through: <sha>
- Audited through: <sha>
- Residue swept through: <sha>
- Posture swept through: <sha, or "<subsystem> next" mid-rotation>
- This tick: <the one job taken and its outcome>
- Queue: <one line — entries, gates>
Plan continues: yes — <the next live input> | after-build — <what resumes> | no — <why quiet>
```

- **state.md is a ledger, never a narrative — 30 lines is the gated
  cap.** `This tick:` is ONE line (the job taken, its outcome); the
  reasoning, evidence, and DATUMs live in the plan **commit body**,
  written once and never re-churned. A tick that essays into state.md
  re-derives that essay every subsequent tick — the churn is the tax,
  and the gate reverts it.
- A cursor you did not advance this tick is copied forward **verbatim** —
  cursor lines must survive every rewrite, or the delta window falls back
  to the last `plan:` commit and silently skips past un-derived work.
- The marker is mechanical, three values (the harness regex reads all
  three): `yes` iff plan itself has a live job next tick — a queue-shaping
  input below the serviced one (an undrained inbox, an unrouted delta, an
  unreconciled window), **or an open posture rotation with no pickable
  entries** (plan drives the sweep to close itself, one neighborhood a
  tick); `after-build` iff the **only** remaining live job is the posture
  sweep AND pickable entries exist — ready work ships first, the sweep
  resumes when the wave hands back; `no` iff nothing is live. With `no`
  and pickable entries, build takes over; with `no` and none, the loop
  hibernates. **An open rotation is therefore never `no`** — pickable
  makes it `after-build`, no-pickable makes it `yes` — so a multi-tick
  sweep drives itself to an empty frontier and is never left mid-rotation
  for a forced wake. Never `after-build` while a queue-shaping input is
  live — building on a queue the next tick would rewrite is the misroute
  the ordering exists to prevent.
- Never re-emit an unchanged queue with unmoved cursors under `yes`.
