# Plan state

- Spec derived through: 663e03f
- Audited through: 60faee0
- Residue swept through: 60faee0
- Posture swept through: foundation done — model next
- This tick: POSTURE SWEEP. Jobs 1-3 re-confirmed quiet (inbox and
  refactor-captures empty at start; no specs/ commits past 663e03f —
  `git log 663e03f..HEAD -- specs/` empty; no src/tests/sdk commits past
  60faee0 — `git log 60faee0..HEAD -- src/ tests/ sdk/` empty). Posture
  sweep was mid-rotation under a stale, pre-architecture.md per-module
  roster ("drift done, cli-engine next" — dated before 663e03f rewrote
  architecture.md's codemap into the 7 named subsystems). Per the
  posture-sweep rule ("the pages are the authority as they read this
  tick... nothing is swept from a remembered list"), restarted the
  rotation clean against the current codemap's roster order and opened
  at its first subsystem: foundation (check.rs, extract.rs, hash.rs,
  address.rs, tap.rs, json_splice.rs, 2849 lines total).
  Read every file against every section of engineering.md. hash.rs,
  address.rs, tap.rs, json_splice.rs, check.rs: clean — no unwrap/expect
  on real paths, no vacuous Result, exports all resolve to an
  outside-module caller (a test counts).
  extract.rs: two findings.
  (1) Mechanical — `enablement_member_fields`/`hook_member_fields` are
  `pub(crate)` with no caller outside their own module (only used by
  `manifest_members` in the same file), unlike their write-face siblings
  `enablement_entry_value`/`hook_matcher_group` which ARE called from
  `crate::drift` and unlike `entry_fields`, the third same-role decomposer,
  already plain `fn`. Filed directly: EXTRACT-PRIVATE-COLLECTION-DECODERS
  (engineering.md, "An export earns its consumer").
  (2) Design question — `nested_members_from_rows`/`embedded_member_from_row`
  take `crate::drift::NestedMemberRow` (pipeline), and `manifest_members`/
  `enablement_member_fields` reference `crate::kind::CollectionKeyPath`/
  `ENABLEMENT_FIELD` (model): real internal dependencies from a module
  architecture.md's own invariant declares dependency-free ("foundation
  depends on nothing internal... extract import[s] no sibling" — false on
  disk at HEAD). Same shape as the two edges the page already declares in
  tension with itself. Not mechanical (which direction resolves it — move
  the row/constant down, move the function up, or widen the invariant text
  — is a design call, same as the other two edges), so filed as a
  `.flume/refactor/` capture per the posture-sweep rule's routing, not
  invented into a pending entry: `.flume/refactor/plan-extract-foundation-boundary.md`.
- Queue: 6 pending — 3 pickable OPEN (DISCOVERY-INFALLIBLE-RESULT-COLLAPSE,
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION, EXTRACT-PRIVATE-COLLECTION-DECODERS;
  disjoint files, no serialization needed), 1 blockedBy
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION (DRIFT-EMIT-LOCK-PARSE-HOIST; shares
  src/drift.rs, correctly serialized), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks: unchanged
  this tick — (multi-harness-projection), (lazy-grounds), (drift-install-edge),
  (frontmatter-builtin-kind-edge). One live refactor capture filed this
  tick: plan-extract-foundation-boundary.md.

Plan continues: yes — the refactor capture this tick filed makes job 1 live
next tick (ahead of posture sweep's continuation to `model`, the roster's
next subsystem).
