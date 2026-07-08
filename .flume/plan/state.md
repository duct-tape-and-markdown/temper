# Plan state

- Spec derived through: cd7135b
- Audited through: d7d0912
- Residue swept through: fe0c502
- This tick: Ship audit (job 3). One commit past the prior cursor touched
  src/tests/sdk: ad109b4 (build: retire posture vocabulary for enforcement
  mode). Verified on disk — the rename landed exactly in its three scoped
  files (src/install.rs, src/drift.rs, tests/install.rs); remaining
  `posture` hits repo-wide are the entry's own documented exclusions (CI
  posture, mixed-posture corpus, SDK's unrelated "delivery posture") —
  cargo test green. Its shipment cleared RETIRE-OWN-PATH-MACHINERY's
  blockedBy: reconfirmed 4ed4027 (INSTALL-WHOLE-CONVERSION) makes scaffold
  structurally incapable of emitting own_path=true, and current
  pipeline.md Install text names no lifted-but-untyped state, so 0016's
  premise still holds; own_path remains live verbatim across all five
  files the entry cites. Flipped its gate blockedBy → open. Re-checked
  PACKAGING-CHANNELS' parked reason: unchanged (only temper.yml exists,
  root package.json still the private flume manifest, sdk/package.json
  still 0.0.5) — reason holds verbatim.
- Queue: RETIRE-OWN-PATH-MACHINERY now open and pickable; PACKAGING-CHANNELS
  parked.

Plan continues: yes — residue cursor (fe0c502) trails HEAD (d7d0912): the
next tick's job 4 sweeps src/tests/sdk for corpus-retired vocabulary since
that commit before handoff.
