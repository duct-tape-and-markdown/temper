# Plan state

- Spec derived through: b645125 — unchanged, not this tick's job.
- Audited through: 9d3aed0 — unchanged; 9d3aed0..HEAD touches no src/tests/sdk (verified this tick).
- Residue swept through: 9d3aed0 — unchanged, same reasoning.
- Posture swept through: pipeline next (mid-rotation) — formats read whole this tick, see commit body.
- This tick: POSTURE SWEEP, formats neighborhood (frontmatter.rs/document.rs/json_manifest.rs/toml_document.rs) — quiet-on-clean, rotation advances to pipeline, see commit body.
- Queue: 2 pending, 0 open, 0 blockedBy, 2 parked. Refactor: 0 live. Friction: 0 live. Inbox: 0 notes.

Plan continues: no — jobs 1-3 are quiet (verified this tick) and the queue's 2 entries are both parked (0 pickable); posture sweep (pipeline) is the next live input once forced-waked.
