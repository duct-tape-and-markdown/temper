# Plan state

- Spec derived through: 53e6f615 — 0046 routed: two Consequences already
  shipped in-commit (verified on disk), the third filed as an entry.
- Audited through: 8d15b725 — window f25ce1ad..8d15b725 audited;
  TELEMETRY-FIELD-STATES-DECLARED-ABSENCE's work verified on disk
  (src/telemetry.rs::field's absent-log branch narrates against
  `has_declared_telemetry`; src/read.rs computes it from the roster's
  `Verifier::Telemetry` requirements), both read_verbs.rs cases green,
  entry already absent from pending.json (dropped on ship). metrics.jsonl:
  clean ship, no revert.
- Residue swept through: 8d15b725 — same window; grepped
  `Verifier::Telemetry` across src/ and tests/ — roster.rs's per-event
  admissibility check and read.rs's new declared-wiring check are distinct
  jobs over the same corpus (per 753526a5's own body), no duplicate surface.
- Posture swept through: 39107255 — mid-rotation, window 39107255..HEAD.
  Frontier: src/telemetry.rs (covered this tick), src/read.rs, src/tap.rs
  (both still open, unread in full).
- This tick: POSTURE SWEEP — telemetry.rs neighborhood; filed
  TELEMETRY-FIELD-REDERIVES-MEMBER-INDEX.
- Queue: 4 pending — 1 open, 1 parked, 2 deferred. Open forks: 3. Friction:
  2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: after-build — TELEMETRY-FIELD-REDERIVES-MEMBER-INDEX is
now pickable, so build takes it; the posture rotation (39107255..HEAD,
src/read.rs and src/tap.rs still in the frontier) resumes when the wave
hands back.
