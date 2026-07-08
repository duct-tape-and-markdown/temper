# Plan state

- Spec derived through: cd7135b
- Audited through: d8405d7
- Residue swept through: a723c3e
- This tick: Ship audit d4cd903..d8405d7. Verified
  BUNDLE-DESCRIPTION-DROPS-FLOOR-VOCAB shipped clean on disk: `src/bundle.rs`
  `PLUGIN_DESCRIPTION` now reads "default contracts embedded" (Decision
  0001 / specs/builtins.md vocabulary), `tests/bundle.rs`'s
  `shipped_strings_teach_install_not_the_retired_import_verb` guard extended
  to forbid "floor" across plugin.json/marketplace.json/SKILL.md, and
  `cargo test --test bundle` passes (5/5). The entry was already removed
  from pending.json by its own chore(flume) ship commit — nothing to drop
  here. Re-checked PACKAGING-CHANNELS's parked reason: no
  `.github/workflows/release.yml`, sdk still at 0.0.5, root package.json
  still the private flume manifest — condition unchanged, stays parked.
  Cursor advances to d8405d7 (HEAD).
- Queue: PACKAGING-CHANNELS (parked on human release creds +
  engine-binary workflow) — nothing else pending.

Plan continues: yes — residue-sweep cursor (a723c3e) trails HEAD (d8405d7);
next tick sweeps the range.
