# Plan state

- Spec derived through: 087b90a — unchanged; `git log 087b90a..HEAD -- specs/` empty.
- Audited through: 1cf7f11 — unchanged; `git log 1cf7f11..HEAD -- src/ tests/ sdk/` empty (only the prior plan commit sits in that window). Both parks re-tested and hold.
- Residue swept through: 1cf7f11 — unchanged, same empty window.
- Posture swept through: sdk/src/contract.ts (+ its immediate import kind.ts) covered this tick; sdk/src/declarations.ts next in rotation.
- This tick: POSTURE SWEEP, neighborhood sdk/src/contract.ts + kind.ts. Every exported predicate constructor checked for a real caller (builtins.ts/dial.ts/claude-code.ts or a contract.test.ts assertion); `formatPlacesEdges` is the sole zero-consumer export — defined and root-re-exported, never called anywhere in sdk/src or sdk/test. Cross-checked the Rust side (src/contract.rs, tests/contract_template.rs): the predicate is deliberately kept live and author-opt-in there, with its own test — so the fix is the TS side's missing constructor test, not deletion. Filed CONTRACT-FORMAT-PLACES-EDGES-ZERO-CONSUMER (open), per engineering.md "An export earns its consumer". No other posture-page violation found in the neighborhood; generated/index.ts (contract.ts's other immediate import) is the machine-written ts-rs boundary and out of scope by design.
- Queue: 3 pending — 1 open (CONTRACT-FORMAT-PLACES-EDGES-ZERO-CONSUMER), 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks: 2 (multi-harness-projection, lazy-grounds). Friction: 1 live (human's to read). Amendments: 0. Inbox: 0 notes.

Plan continues: after-build — the only remaining live job is the posture rotation (declarations.ts next), and a pickable entry now exists; the sweep resumes once the wave hands back.
