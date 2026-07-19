# Plan state

- Spec derived through: b645125 — unchanged, not this tick's job.
- Audited through: 2bfcccd — unchanged; 2bfcccd..HEAD touches no src/tests/sdk (only the prior plan commit 8baf50a landed since, verified this tick).
- Residue swept through: 2bfcccd — unchanged, same reasoning.
- Posture swept through: verbs next (mid-rotation) — pipeline neighborhood swept this tick: read.rs read whole (the only pipeline module touched since 03f19a2; drift.rs/import.rs/builtin_lock.rs/placement.rs still covered, 0 touches since then).
- This tick: POSTURE SWEEP, pipeline neighborhood. Fresh cycle armed by the prior tick's build window (1adf7f0..HEAD touched src/read.rs via ee69164 and src/main.rs via e25f62f) — pipeline (read.rs) sorts before verbs (main.rs) in the fixed rotation order, so pipeline swept first. Found dead-plumbed params (`impact_one_impl`'s unused `_member_index`, `context_member_one_impl`'s unused `_by_kind`) and three broken intra-doc links (`impact_one`/`context_member`/`context_member_one`, all deleted by ee69164) — residue from that same consolidation commit. Filed READ-DEAD-INDEX-PARAM-STALE-DOC-PRUNE (open, no blockers). Rotation advances to verbs (main.rs needs a fresh full read next tick — touched by e25f62f since its last full read at d01c0a2 — foundation/model/formats/judges/provider stay skipped, untouched this cycle).
- Queue: 3 pending, 1 open, 0 blockedBy, 2 parked. Refactor: 0 live. Friction: 0 live. Inbox: 0 notes.

Plan continues: yes — the rotation's frontier still holds verbs (main.rs), so it is not closed; next tick sweeps verbs directly rather than idling.
