# Plan state

- Spec derived through: b645125 — unchanged, not this tick's job.
- Audited through: 9d3aed0 — unchanged; 9d3aed0..HEAD touches no src/tests/sdk (re-verified this tick).
- Residue swept through: 9d3aed0 — unchanged, same reasoning.
- Posture swept through: verbs next (mid-rotation) — provider read whole this tick.
- This tick: POSTURE SWEEP, provider neighborhood (builtin.rs/builtin_kind.rs/sdk/src/builtins.ts/sdk/src/claude-code.ts read whole) — quiet-on-clean, rotation advances to verbs, see commit body.
- Queue: 3 pending, 1 open, 0 blockedBy, 2 parked. Refactor: 0 live. Friction: 0 live. Inbox: 0 notes.

Plan continues: yes — the posture-sweep rotation is open (verbs next); per posture-sweep.md an open rotation is live input and drives itself to close, never waits on a forced wake.
