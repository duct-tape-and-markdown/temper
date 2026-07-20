# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: 5ba7b81 — advanced; window 03bd676..5ba7b81 reconciled.
- Residue swept through: 5ba7b81 — advanced, same window.
- Posture swept through: src/builtin.rs..src/json_splice.rs (prior rotation), plus src/kind.rs, src/layout.rs, src/lib.rs, src/main.rs, src/placement.rs, src/read.rs, src/roster.rs, src/schema.rs, src/tap.rs covered — mid-rotation. Rotation continues to src/telemetry.rs next.
- This tick: POSTURE SWEEP, neighborhood src/tap.rs (+ its immediate import, src/builtin_kind.rs, read for context). Finding: tap.rs's `record_from_payload` (167) reads the Claude Code hook payload's `session_id` field as a bare `"session_id"` string literal — a documented external fact (the same hook contract builtin_kind.rs's `classify_claude_code_hook_payload` already cites: code.claude.com/docs/en/hooks, retrieved 2026-07-17) living outside the provider face module architecture.md designates for such facts ("The provider face is data": builtin/builtin_kind are where a payload schema literal belongs, never elsewhere). Filed TAP-SESSION-ID-PROVIDER-FACE-MOVE, mechanical (a sibling extractor fn in builtin_kind.rs, tap.rs calls it), open — no design call. No other lens fired: TapEvent/TapRecord/TapError/LogReadout each hold one job at one home, no dead plumbing (every TapError variant has a live construction site), no other embedded-literal or cohesion issue on disk.
- Queue: 3 pending, 1 open (TAP-SESSION-ID-PROVIDER-FACE-MOVE, pickable), 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — unaffected by this tick, HEAD unchanged since their last re-check). Refactor: 0 live. Friction: 0 live. Amendments: 0 live. Inbox: 0 notes.

Plan continues: after-build — the posture sweep is mid-rotation (next candidate src/telemetry.rs) but a pickable entry now exists (TAP-SESSION-ID-PROVIDER-FACE-MOVE), so build ships it first and the sweep resumes when the wave hands back.
