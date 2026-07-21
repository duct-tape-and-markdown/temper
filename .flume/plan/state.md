# Plan state

- Spec derived through: 946e303 — unchanged; `git log 946e303..HEAD -- specs/` is empty.
- Audited through: 9329952 — unchanged; `git log 9329952..HEAD -- src/ tests/ sdk/` is empty.
- Residue swept through: 9329952 — same, unchanged.
- Posture swept through: sdk/src/ tree fully covered; src/address.rs, src/admissibility.rs
  covered. src/builtin.rs next in tree order — mid-rotation.
- This tick: POSTURE SWEEP of src/admissibility.rs (+ immediate imports check/compose/
  contract/drift/engine/extract/kind::CustomKind, read for context only). Two findings,
  both verified on disk, filed as ADMISSIBILITY-EFFECTIVE-KIND-SET-DEDUP (per
  engineering.md "One job, one home"): (1) local_locus_admissibility (214-220) and
  registration_locus_admissibility (251-257) duplicate an identical loop building
  Vec<CustomKind> from overlaid_builtin_kinds + custom_rows — governs_collision_diagnostics
  checked and ruled out as a third instance (it reads governs_root/glob off rows directly,
  never constructs CustomKind). (2) the module //! header's "Seven judges" is stale —
  dd96b6e added registration_locus_admissibility as an eighth judge without updating the
  count; folded into the same entry's files[].description rather than a standalone
  ride-only open-questions note, since this tick's entry is exactly "whichever one first
  opens the file." No other lens (libraries-before-hand-rolls, shared-concept exhaustiveness,
  cost hoisting, export-earns-consumer — all nine pub items have a gate.rs caller,
  confirmed via rg -a) found anything on this module.
- Queue: 3 pending — 1 open (ADMISSIBILITY-EFFECTIVE-KIND-SET-DEDUP), 1 deferred
  (GUIDANCE-FIELD-DECLARATION-CHANNEL), 1 parked (PACKAGING-CHANNELS-REMAINDER). Open
  forks: 2, unchanged. Friction: 0. Amendments: 1, still awaiting ratification. Inbox: 0.

Plan continues: after-build — the only remaining live job is the posture rotation
(mid-rotation, src/builtin.rs next) and a pickable entry now exists
(ADMISSIBILITY-EFFECTIVE-KIND-SET-DEDUP); ready work ships first, the sweep resumes at
src/builtin.rs when the wave hands back.
