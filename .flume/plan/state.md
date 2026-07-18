# Plan state

- Spec derived through: 663e03f
- Audited through: 60faee0
- Residue swept through: 60faee0
- Posture swept through: foundation done — model next
- This tick: INBOX. Job 1 was live: `.flume/refactor/` held one capture,
  `plan-extract-foundation-boundary.md` (observed 04610b1, filed last tick
  during the foundation posture sweep). Re-verified at HEAD 5b4701c before
  draining: `git log 04610b1..HEAD -- src/extract.rs src/drift.rs
  src/kind.rs specs/process/architecture.md` empty, so every cited
  line/symbol is unmoved — confirmed on disk (grep): `nested_members_from_rows`/
  `embedded_member_from_row` (764, 778) take `crate::drift::NestedMemberRow`
  as a real parameter type, `manifest_members` (1015) branches on
  `crate::kind::CollectionKeyPath` (1022, 1032), `enablement_member_fields`
  (1075) reads `crate::kind::ENABLEMENT_FIELD` (1077, 1095) — both real
  edges from `extract.rs` (foundation) into `drift.rs` (pipeline) and
  `kind.rs` (model), contradicting architecture.md's "foundation depends on
  nothing internal" invariant as applied to `extract`. The capture's own
  text names the fix as a design call (move the two functions down to
  `drift.rs` vs. move `manifest_members`'s dependencies elsewhere vs. amend
  the invariant's text) — same shape as the two tension edges
  architecture.md already declares against itself. Per collaboration.md
  ("never invent intent absent from its source") and the precedent those
  two edges set (routed to open-questions, not forced into a pending entry
  inventing a direction), routed this the same way: filed
  `(extract-foundation-edge)` in open-questions.md, deleted the capture.
  No pending.json change this tick.
- Queue: unchanged — 6 pending, 3 pickable OPEN
  (DISCOVERY-INFALLIBLE-RESULT-COLLAPSE, DRIFT-LOCK-ROW-WALK-CONSOLIDATION,
  EXTRACT-PRIVATE-COLLECTION-DECODERS; disjoint files), 1 blockedBy
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION (DRIFT-EMIT-LOCK-PARSE-HOIST), 2 parked
  on human action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open
  forks: (multi-harness-projection), (lazy-grounds), (drift-install-edge),
  (frontmatter-builtin-kind-edge) unchanged; (extract-foundation-edge) new
  this tick. No live refactor captures remain (directory holds only
  README.md); inbox empty.

Plan continues: yes — posture sweep resumes at `model`, the roster's next
subsystem (mid-rotation per `Posture swept through:`).
