# Plan state

- Spec derived through: 20a6f54 — unchanged, no spec/ commits past it.
- Audited through: f194b260 — unchanged, no src/tests/sdk commits past it.
- Residue swept through: f194b260 — unchanged, same.
- Posture swept through: 97d0241 — mid-rotation, window head unchanged:
  sdk/src/declarations.ts covered this tick. Frontier remaining:
  src/compose.rs, src/main.rs, src/read.rs, src/install.rs.
- This tick: POSTURE SWEEP — neighborhood sdk/src/declarations.ts (+
  immediate imports assembly.ts/kind.ts/contract.ts/prose.ts/builtins.ts)
  read against engineering.md + architecture.md. Found one embedded-
  provider-knowledge violation ("The provider face is data"): `tapHookRows`
  (declarations.ts:758-796) hardcodes the Claude Code hook payload's
  `type`/`command`/`matcher` field schema and the `hooks.<Event>` key-path
  (already a literal at builtins.ts:409) as bare literals in the engine-
  generic erasure module, instead of sourcing them from the provider face.
  Filed → TAP-HOOK-FIELD-SCHEMA-PROVIDER-FACE (open, disjoint files from
  every other pending entry).
- Queue: 5 pending — 2 open, 1 parked, 2 deferred. Open forks: 2, unchanged.
  Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: after-build — two pickable open entries
(SCHEMA-HELP-KIND-DOMAIN-FOSSIL, TAP-HOOK-FIELD-SCHEMA-PROVIDER-FACE) ship
first; posture rotation resumes (4 frontier modules left) once the wave
hands back.
