# Plan state

- Spec derived through: f67303c
- Audited through: 9223917
- Residue swept through: 9223917
- This tick: INBOX drain of the two remaining PR #20 notes — the inbox is now
  empty. Both re-verified against HEAD before routing. (1) 0019 decision-record
  renumber: two records wear 0019 (`0019-content-is-a-declared-kind-fact.md`,
  `0019-loud-or-nothing.md`) — confirmed both present on disk; 0020's only bare
  "0019" cite (line 7, "typed the heading tree by declaring positions") points
  to 0019-content, and a full `grep -rn 0019 specs/` finds no other cross-ref.
  This is a `specs/` renumber = ratification territory OUTSIDE build's fence
  (`BUILD_WRITABLE_PATHS` has no `specs/**`; build never writes decisions), so
  it is NOT a build-queueable pending entry and NOT a design fork — a session
  `docs:` task. Determination (commit body): renumber `0019-loud-or-nothing.md`
  → `0023` (next free slot after 0022); 0019-content keeps its number so 0020's
  lone cite stays valid; no other cross-ref moves. Drained to the commit body,
  no entry. (2) Pack-kind field trial → two registered open forks: gap (a)
  single-kind directory-slice path derivation (`member_projection_path`
  drift.rs:513 substitutes first `*` only — confirmed on disk) →
  `(directory-sliced-governance)`; gap (b) frontmatterless projections carry no
  managed-by banner (install.rs:152 note is frontmatter-borne; corpus mandates
  no banner — `grep specs/` for managed-by/banner is empty) →
  `(frontmatterless-managed-by-banner)`. Neither is a spec violation nor
  buildable-as-specified — both are undecided model/product questions, hence
  forks. `(layout-kind-heterogeneous-corpus)`'s stale "still inbox-queued"
  parenthetical repointed at `(directory-sliced-governance)`. Cursors unmoved
  (inbox job, no spec/src window). Pending queue untouched — still disjoint.
- Queue: CHECK-ARG-HALF-GATE (open) + GLOB-VALIDITY-PREDICATE (open, disjoint)
  pickable; LAYOUT-OVERLAY-CHECK-GAP blockedBy CHECK-ARG (shared main.rs);
  SATISFIES-LABEL-QUALIFY + LOCK-SPELLING-REAP + EMIT-INTO-REROOT-REAP all
  dependsOnForks `(lock-upgrade-migration-posture)` (+ their gates);
  PACKAGING-CHANNELS-REMAINDER parked. No queued entry rests on either new fork.

Plan continues: no — inbox drained empty; spec cursor f67303c is specs/ HEAD
(no delta); reconcile window 9223917..HEAD touches no src/tests/sdk
(git diff --stat confirms empty). No live input remains. Pickable entries
exist (CHECK-ARG-HALF-GATE, GLOB-VALIDITY-PREDICATE) — build takes over.
