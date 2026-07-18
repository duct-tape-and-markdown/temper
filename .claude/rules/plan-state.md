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
Plan continues: yes — <the next live input> | no — <why quiet>
```

- A cursor you did not advance this tick is copied forward **verbatim** —
  cursor lines must survive every rewrite, or the delta window falls back
  to the last `plan:` commit and silently skips past un-derived work.
- The marker is mechanical: `yes` iff an input below the one serviced is
  still live, `no` otherwise (`^Plan continues:\s*yes\b` is load-bearing —
  the harness regex that re-wakes plan). With `no` and pickable entries,
  build takes over; with `no` and none, the loop hibernates.
- Never re-emit an unchanged queue with unmoved cursors under `yes`.
