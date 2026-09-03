# Plan state

- Spec derived through: 13455d4a — unchanged, copied forward (not this tick's job).
- Audited through: 53852595 — unchanged; no src/tests/sdk commits land past it (`git log 53852595..HEAD -- src/ tests/ sdk/` empty).
- Residue swept through: 53852595 — unchanged, same empty window.
- Posture swept through: 22f8064c — unchanged, mid-rotation continues. Frozen frontier (armed at 7d695577, fixed regardless of later HEAD movement per the posture-sweep rule): src/read.rs (covered), src/telemetry.rs (covered), tests/read_verbs.rs (covered), src/admissibility.rs (covered), src/gate.rs (covered), src/graph.rs (covered), tests/graph.rs (covered), src/tap.rs, tests/tap.rs, tests/hook_kind.rs, sdk/src/builtins.ts, sdk/src/declarations.ts, tests/emit.rs, src/install.rs, tests/install.rs, src/compose.rs, src/drift.rs, src/glob.rs, src/placement.rs.
- This tick: inbox job — drained the one live refactor capture (`plan-graph-glob-witness-hand-roll.md`, observed at e8c160c3) into pending entry GRAPH-GLOB-WITNESS-HAND-ROLL-DOC-NOTE (open). Re-verified at HEAD: no src/graph.rs or src/glob.rs commits since e8c160c3, claim unchanged; checked globset 0.4.18/0.4.20's public `Glob` API and confirmed it exposes no parsed brace/char-class AST (only the original glob string and a compiled regex string) — so the fix is the doc-comment "pinned semantics" path the capture itself proposed, not an open design fork. Capture file deleted.
- Queue: 7 pending — 2 open, 3 parked, 2 deferred. Open forks: 5 (unchanged). Friction: 0. Amendments: 0. Inbox: 0. Refactor: 0 (drained this tick).

Plan continues: after-build — EMIT-BANNER-OWNERSHIP-MOVE and GRAPH-GLOB-WITNESS-HAND-ROLL-DOC-NOTE are both pickable now; the posture sweep's open rotation resumes once the wave hands back.
