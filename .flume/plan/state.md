# Plan state

- Spec derived through: f67303c
- Audited through: 9223917
- Residue swept through: 9223917
- This tick: INBOX drain of the lock-migration cluster (PR #20 notes 5/6/7),
  the coherent batch note 5 explicitly generates ("the generator for the two
  notes below, plus the standing `--into` reap note above"). All three code
  claims re-verified live on disk at HEAD (no src/sdk/tests commit touches the
  sites since 0aa9e62): drift.rs:501 `to_lock_path` strips backslashes never
  `./`; drift.rs:866 orphan sweep joins raw `row.source_path`; drift.rs:2311
  `SatisfiesRow.member` is a bare id where MentionRow carries `kind:name`;
  declarations.ts:314 `satisfiesRows` pushes `member.name` bare while
  mentionRows keys `${kind}:${name}`; main.rs:1051 `resolve_kind_units` folds
  per kind. Routed: note 5 → open fork `(lock-upgrade-migration-posture)`
  (the one posture its three instances hang off); note 7 → SATISFIES-LABEL-
  QUALIFY (open, dependsOnForks the posture); note 6 → LOCK-SPELLING-REAP
  (blockedBy SATISFIES — shared drift.rs — + dependsOnForks); existing
  EMIT-INTO-REROOT-REAP gained dependsOnForks on the same slug (note 5 names
  it the third instance). The two new entries are fork-gated (non-pickable),
  so pickable stays {CHECK-ARG, GLOB-VALIDITY} — disjoint. Cursors unmoved
  (inbox job, no spec/src window this tick).
- Queue: CHECK-ARG-HALF-GATE (open) + GLOB-VALIDITY-PREDICATE (open, disjoint)
  pickable; SATISFIES-LABEL-QUALIFY + LOCK-SPELLING-REAP + EMIT-INTO-REROOT-
  REAP all dependsOnForks `(lock-upgrade-migration-posture)` (+ their gates);
  PACKAGING-CHANNELS-REMAINDER parked.

Plan continues: yes — inbox still holds five PR #20 notes: the 0019-content
layout cluster (docs-remainder, member-fence fork, layout-probe triple), the
0019 decision-record renumber, and the pack-kind field trial — each a
substantial fork/entry, drained in later ticks as coherent batches.
