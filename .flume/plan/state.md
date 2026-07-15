# Plan state

- Spec derived through: 06e0c2c
- Audited through: fd0ba24
- Residue swept through: fd0ba24
- This tick: reconciled the 226199b..fd0ba24 ship window (three builds:
  SATISFIES-LABEL-QUALIFY 3d08a4a, INSTALL-FRONTMATTERLESS-BANNER aa24e62,
  CUSTOM-KIND-DOCS bd8f31f — all dropped from pending by build). AUDIT: verified
  SATISFIES on disk (qualified the satisfies wire to `kind:name`; `mentionRows`
  builds `${kind}:${name}` at declarations.ts:340); its two blocked dependents
  unblock to `open` — LOCK-SPELLING-REAP and MENTION-DISCOVERY-DEFER. Re-verified
  premises: `to_lock_path` (drift.rs:501) strips backslashes never `./`; the
  orphan sweep still compares raw row `source_path` at drift.rs:866;
  `renderTextBody` still rejects on `mentionable` at emit.ts:164. SWEEP: no
  fileable residue — BANNER folded the memory kind special-case into
  content-keyed routing (consolidation, not a duplicate); two comment-staleness
  riders maintained (extract.rs:196-198 floor-mention, emit.test.ts:904
  renderMemberFence — both opened by SATISFIES, neither reconciled, both
  unshifted).
- Queue: LOCK-SPELLING-REAP (open, drift.rs) → EMIT-INTO (blockedBy LOCK-SPELLING)
  → LOCK-LAYER-DROP (blockedBy EMIT-INTO) — serial drift.rs chain;
  MENTION-DISCOVERY-DEFER (open, emit.ts+declarations.ts+graph.rs); PACKAGING
  parked. Pickable now: LOCK-SPELLING-REAP + MENTION-DISCOVERY-DEFER — disjoint
  (drift.rs / sdk+graph.rs).

Plan continues: no — inbox empty, spec delta routed to HEAD's last specs commit
(06e0c2c), and the 226199b..fd0ba24 ship window is fully reconciled (audit +
sweep). Two `open` entries remain, disjoint — build takes over. NB the
SessionStart reporter shows the `.temper` dogfood gate red (two unfilled
requirements: friction-capture-procedure, pending-entry-discipline); harness
territory, a `chore(harness)` fix outside plan's writable paths.
