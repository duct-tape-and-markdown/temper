# Plan state

- **Phase:** reconcile after DRIFT-DIFF shipped. Verified on disk: `src/drift.rs`
  + the `Diff` arm in `main.rs` give read-only `temper diff` over the import
  baseline. Governance (`45-governance.md`) is entirely unbuilt — `engine.rs` has
  no `range`/`count`/`degree`/`acyclic` predicate.
- **Last shipped:** DRIFT-DIFF (c63d238 / 22b3425).
- **In flight:** nothing; tree clean (`contracts/spec.toml` still absent — never
  committed by a human).
- **The gap reconciled:** DRIFT-DIFF already off-queue (shipped). SPEC-KIND-GATE
  stays `parked` on a human committing `contracts/spec.toml`. Filed GOV-RANGE —
  the numeric `range {min, max}` predicate from `45`'s `Also in scope`, fork-free
  and self-contained (a field-scope clause, no roster/graph machinery), the
  cleanest next governance slice. Inbox empty; no new fork.

Plan continues: no — GOV-RANGE is `open` and immediately pickable (contract.rs +
engine.rs only). SPEC-KIND-GATE waits on a human, not on planning. Hand to build.
