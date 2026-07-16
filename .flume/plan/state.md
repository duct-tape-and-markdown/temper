# Plan state

- Spec derived through: 06e0c2c
- Audited through: 5ef998b
- Residue swept through: 5ef998b
- This tick: reconciled the fd0ba24..5ef998b ship window (two builds:
  LOCK-SPELLING-REAP e5dbb75, MENTION-DISCOVERY-DEFER ed5bb8e — both dropped
  from pending by build). AUDIT: verified both on disk. LOCK-SPELLING-REAP
  added `normalize_lock_path` (drift.rs:511), routed `to_lock_path` through it,
  and normalizes both reap-join sides at drift.rs:879 (a `./`-prefixed row
  rejoins its live projection); its blocked dependent EMIT-INTO-REROOT-REAP
  unblocks to `open`. MENTION-DISCOVERY-DEFER defers a declared-`at`-kind
  mention to check and corrected the MentionRow doc — standalone, no dependents.
  Re-derived the drift.rs anchors both blocked entries cite (window shifted
  drift.rs +25/+8): reap_or_report_orphan 1312→1326, emit_program 537→547,
  orphan sweep 865-872→875-882; main.rs anchors unshifted. SWEEP: no fileable
  residue — `normalize_lock_path` is the one canonicalizer (consolidation, not
  a duplicate). Two riders in touched files maintained: prose.ts law/posture
  narration (MENTION-DEFER +48 at line 53, lines re-derived) and
  emit.test.ts:904→907 renderMemberFence — both left as unchanged context,
  undischarged.
- Queue: EMIT-INTO-REROOT-REAP (open, drift.rs+main.rs) → LOCK-LAYER-DROP-REFUSE
  (blockedBy EMIT-INTO) — serial drift.rs chain; PACKAGING parked. Pickable now:
  EMIT-INTO-REROOT-REAP only.

Plan continues: no — inbox empty, spec cursor at HEAD's last specs commit
(06e0c2c), and the fd0ba24..5ef998b ship window is fully reconciled (audit +
sweep). One `open` entry pickable (EMIT-INTO) — build takes over. NB the
SessionStart reporter shows the `.temper` dogfood gate red (two unfilled
requirements: friction-capture-procedure, pending-entry-discipline); harness
territory, a `chore(harness)` fix outside plan's writable paths.
