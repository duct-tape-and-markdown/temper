# Plan state

- Spec derived through: edb6ddc4 — contract.md re-cut (7fb49108/c9fbbca8/edb6ddc4, 171→150 lines per 0047's Consequences) routed: pure editorial trim, no ratified-intent change, no entries filed.
- Audited through: 15f50601 — window 41e8847a..15f50601 clean (see commit body).
- Residue swept through: 15f50601 — same window, no findings.
- Posture swept through: 5d930da0 mid-rotation — covered src/read.rs+tests/read_verbs.rs+tests/prose_include.rs (filed READ-VERBS-MEMBER-INDEX-HOIST); frontier remaining: sdk/src/dial.ts.
- This tick: posture sweep neighborhood src/read.rs+tests/read_verbs.rs+tests/prose_include.rs. tests/prose_include.rs and tests/read_verbs.rs read clean (exercise read.rs's public/crate-private surface, no residue of their own). Filed READ-VERBS-MEMBER-INDEX-HOIST: build_member_index (read.rs:136) is built dead in requirements() (1403-1426, unused in both branches — requirement_detail's `_member_index` param, 1477, is never read) and built-then-discarded-then-rebuilt on the leaf-address path of impact()/context() (706-728, 1000-1018, 1026-1097) — verified via full read + grep, no other call site of build_member_index (14 total) shows the same waste.
- Queue: 4 pending — 1 open, 1 parked, 2 deferred. Open forks: 3. Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: after-build — READ-VERBS-MEMBER-INDEX-HOIST is pickable and ships first; posture sweep resumes on sdk/src/dial.ts once the wave hands back.
