# Plan state

- Spec derived through: 946e303 — unchanged; `git log 946e303..HEAD -- specs/` is empty.
- Audited through: 638f051 — unchanged, no new src/tests/sdk commits past it.
- Residue swept through: 638f051 — unchanged, same reason.
- Posture swept through: sdk/src/ tree fully covered; src/address.rs now covered too
  (no internal imports, so its neighborhood was itself alone) — mid-rotation.
  src/admissibility.rs next in tree order.
- This tick: POSTURE SWEEP, src/address.rs neighborhood (zero `use crate::` imports —
  leaf module, no immediate-import fan-out). Filed ADDRESS-NORMALIZE-PATH-COHESION-SPLIT
  (open): `normalize_path` (filesystem lexical normalization, 327-345) and `FieldPath`
  (JSON field addressing, the module's whole stated job per its own //! header) are two
  unrelated jobs in one file. normalize_path landed here at dc3a5a4 under architecture.md's
  then-current subsystem framework (53df138's ruling); that framework was cut from
  architecture.md at 0522eff, already behind the spec cursor — the placement's rationale
  is gone, current architecture.md hands cohesion judgment to the posture sweep. All eight
  `normalize_path` call sites (drift.rs x3, compose.rs x2, graph.rs x3, import.rs x3+test)
  and all `FieldPath` consumers (contract.rs, engine.rs, schema.rs, extract.rs) verified
  live — no zero-consumer or dead-plumbing finding otherwise.
- Queue: 3 pending — 1 open (ADDRESS-NORMALIZE-PATH-COHESION-SPLIT), 1 deferred
  (GUIDANCE-FIELD-DECLARATION-CHANNEL), 1 parked (PACKAGING-CHANNELS-REMAINDER). Open
  forks: 2, unchanged. Friction: 0. Amendments: 1, still awaiting ratification. Inbox: 0.

Plan continues: after-build — the posture rotation is the only remaining live job and a
pickable entry now exists (ADDRESS-NORMALIZE-PATH-COHESION-SPLIT), so the wave ships it
before the sweep resumes.
