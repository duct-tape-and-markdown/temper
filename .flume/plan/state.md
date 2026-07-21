# Plan state

- Spec derived through: 946e303 — unchanged; `git log 946e303..HEAD -- specs/` is empty.
- Audited through: 3f1af5f — unchanged; `git log 3f1af5f..HEAD -- src/ tests/ sdk/` is empty.
- Residue swept through: 3f1af5f — unchanged, same reason.
- Posture swept through: sdk/src/ tree fully covered; src/address.rs, src/admissibility.rs,
  src/builtin.rs, src/builtin_kind.rs covered. src/builtin_lock.rs next in tree order —
  mid-rotation.
- This tick: POSTURE SWEEP of src/builtin_kind.rs + immediate imports (compose, drift,
  extract, kind, tap) — filed TAP-EVENT-DOCUMENTED-NAMES-DEDUP (src/tap.rs); see commit
  body for the verified account.
- Queue: 3 pending — 1 open (TAP-EVENT-DOCUMENTED-NAMES-DEDUP), 1 deferred
  (GUIDANCE-FIELD-DECLARATION-CHANNEL), 1 parked (PACKAGING-CHANNELS-REMAINDER); 0 open
  questions unresolved by this queue. Open forks: 2, unchanged. Friction: 0. Amendments: 1,
  still awaiting ratification. Inbox: 0.

Plan continues: after-build — TAP-EVENT-DOCUMENTED-NAMES-DEDUP is pickable; the posture
rotation is the only other live job and resumes at src/builtin_lock.rs once build hands
back.
