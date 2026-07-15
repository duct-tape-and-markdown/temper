# Plan state

- Spec derived through: f67303c
- Audited through: 9223917
- Residue swept through: 9223917
- This tick: INBOX drain of the 0019-content layout cluster (PR #20 notes
  1/2/3 — the batch state.md named). Note 3(2) verified live on disk at HEAD
  and routed to a pending entry; the other four facets to three open forks +
  one commit-body note. Verification (no src/sdk commit touches these sites
  since f67303c): `overlay_builtin_kind` (main.rs:896-919) lifts governs
  always + templates when non-empty (915-917), never the lock row's `content`
  (`KindFactRow.content`, drift.rs:2056); `resolve_kind_units` dispatches
  layout-vs-frontmatter on the embedded `kind.content` (main.rs:1024, always
  `File` for a built-in), not `overlaid.content`; emit honors the row via
  `content_from_row` (kind.rs:843, drift.rs:749) — so emit reads a
  relocated-built-in layout while check silently falls back to frontmatter
  (confirmed divergence, not merely untested). Routed: note 1 (docs remainder)
  → open fork `(custom-kind-consumer-docs)`; note 2 (member-fence dead text) →
  open fork `(member-fence-dead-text)`; note 3(1) (emit honors relocated
  content — `temper::layout::unadmitted` refuses loud, 0019-loud works
  end-to-end) → verified working, commit-body note, no entry; note 3(2)
  (overlay drops content at check) → LAYOUT-OVERLAY-CHECK-GAP (blockedBy
  CHECK-ARG — shared src/main.rs); note 3(3) (layout binds whole kind vs
  heterogeneous corpus) → open fork `(layout-kind-heterogeneous-corpus)`.
  Pickable stays {CHECK-ARG, GLOB-VALIDITY} — disjoint (main.rs+install.rs vs
  contract.rs+builtins.ts+builtin_lock.toml); the new entry is blockedBy, not
  pickable. Cursors unmoved (inbox job, no spec/src window this tick).
- Queue: CHECK-ARG-HALF-GATE (open) + GLOB-VALIDITY-PREDICATE (open, disjoint)
  pickable; LAYOUT-OVERLAY-CHECK-GAP blockedBy CHECK-ARG (shared main.rs);
  SATISFIES-LABEL-QUALIFY + LOCK-SPELLING-REAP + EMIT-INTO-REROOT-REAP all
  dependsOnForks `(lock-upgrade-migration-posture)` (+ their gates);
  PACKAGING-CHANNELS-REMAINDER parked.

Plan continues: yes — inbox still holds two PR #20 notes: the 0019
decision-record renumber (two records wear 0019) and the pack-kind field trial
(two model gaps: directory-sliced governance + frontmatterless managed-by
banner) — each drained in a later tick as its own coherent batch.
