# Plan state

- Spec derived through: 946e303 — unchanged; `git log 946e303..HEAD -- specs/` is empty.
- Audited through: e15b969 — unchanged; no window past it touches src/ tests/ sdk/.
- Residue swept through: e15b969 — same, unchanged.
- Posture swept through: sdk/src/ tree fully covered; src/address.rs, src/admissibility.rs,
  src/builtin.rs, src/builtin_kind.rs, src/builtin_lock.rs, src/bundle.rs covered. src/check.rs
  next in tree order — mid-rotation.
- This tick: POSTURE SWEEP src/bundle.rs — filed WRITE-CREATING-PARENTS-CONSOLIDATE (the
  create-parent-dirs-then-write mechanic reimplemented 4x: bundle.rs's write_text, install.rs's
  write_scaffold_file, and drift.rs's own two internal copies — create_dir_all/write_bytes and
  write_placement).
- Queue: 3 pending — 1 open (WRITE-CREATING-PARENTS-CONSOLIDATE), 1 deferred
  (GUIDANCE-FIELD-DECLARATION-CHANNEL), 1 parked (PACKAGING-CHANNELS-REMAINDER); 0 open
  questions unresolved by this queue. Open forks: 2, unchanged. Friction: 0. Amendments: 1,
  still awaiting ratification. Inbox: 0.

Plan continues: after-build — WRITE-CREATING-PARENTS-CONSOLIDATE is pickable and ships first;
the open posture rotation resumes at src/check.rs once the wave hands back.
