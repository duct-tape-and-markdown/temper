# Plan state

- Spec derived through: cd7135b
- Audited through: d4cd903
- Residue swept through: a723c3e
- This tick: Residue sweep (job 4), range 3e19f4c..a723c3e. Grepped `floor`,
  `posture`, `own_path`, `temper.toml`/`TEMPER-TOML-ZERO` across
  src/tests/sdk, filtering doc comments (`///`/`//!`) and test function
  names — both are the explicit exception 706139a/ad109b4 already carved
  out (comment-only mentions ride whichever entry next opens that file).
  Found one live non-comment straggler: `src/bundle.rs`'s
  `PLUGIN_DESCRIPTION` const ships "...std-lib floors embedded." in
  plugin.json/marketplace.json (channel 3) — filed as
  BUNDLE-DESCRIPTION-DROPS-FLOOR-VOCAB. `posture`/`own_path`/`temper.toml`
  residue is clean (comment/test-name only, or intentional
  stray-config-is-inert regression tests). The `kinds/`+`packages/`
  session_start.rs fixture debt (open-questions.md "Kept on purpose") is
  unchanged — re-verified present, still rides its own future touch, no
  action needed. Cursor advances to a723c3e (HEAD).
- Queue: BUNDLE-DESCRIPTION-DROPS-FLOOR-VOCAB (open, pickable) ahead of
  PACKAGING-CHANNELS (parked on human release creds + engine-binary
  workflow).

Plan continues: no — inbox empty, spec delta empty, ship-audit cursor
current, residue-sweep cursor now at HEAD; queue has one pickable entry,
hand off to build.
