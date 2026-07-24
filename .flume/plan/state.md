# Plan state

- Spec derived through: 20a6f54 — unchanged, no spec/ commits past it.
- Audited through: f194b260 — unchanged, no src/tests/sdk commits past it.
- Residue swept through: f194b260 — unchanged, same.
- Posture swept through: 97d0241 — mid-rotation, unchanged this tick:
  sdk/src/index.ts covered the prior tick, quiet-on-clean. Frontier
  remaining: sdk/src/declarations.ts, src/compose.rs, src/main.rs,
  src/read.rs, src/install.rs.
- This tick: INBOX — a9578eac (chore, landed before the prior plan tick
  8205300a but missed by it — that tick fell straight to posture sweep
  without noticing live inbox content; caught this tick by reading the
  actual file, not the digest) filed #15: `schema --kind`'s flag help
  (src/main.rs:91) still names the retired `(`skill`, `rule`)` two-kind
  fossil after SCHEMA-KIND-DOMAIN-WIDEN widened the real domain. Carried an
  explicit interactive build-ready ruling (2026-07-24); re-verified live on
  disk (no main.rs commits since f0ef148a) before filing.
  Routed → SCHEMA-HELP-KIND-DOMAIN-FOSSIL (open, disjoint from all other
  pending entries' files). Inbox drained to empty.
- Queue: 4 pending — 1 open, 1 parked, 2 deferred. Open forks: 2, unchanged.
  Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: yes — the queue now carries a pickable open entry
(SCHEMA-HELP-KIND-DOMAIN-FOSSIL) for build to take next; posture rotation
still has 5 frontier modules left for a future plan tick once the queue
next runs dry.
