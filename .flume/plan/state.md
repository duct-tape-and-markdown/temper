# Plan state

- Spec derived through: a0fccaf
- Audited through: 3024687
- Residue swept through: d029d4b
- This tick: Ship audit — LOCK-ROW-REJECT-LOUD verified on disk
  (`LockRowError` + req_/opt_ lift helpers + `family()` attribution,
  drift.rs 2231/2307/2327; `kind_vocab`/`from_kind_fact_row`/
  `content_from_row` vocabulary rejection, kind.rs 547/707/748; fallible
  propagation through main.rs; tests/lock_declaration_rows.rs 29 green,
  run live). Build's declared residual confirmed on disk for the sweep:
  `mode_from_lock` (main.rs:500) `.unwrap_or_default()`s the whole read —
  swallowing the new loud LockRowError — and degrades an out-of-vocabulary
  `mode` value to `warn` (same invariant-6 class, left for the queue per
  b3ffb52's commit body). PACKAGING-CHANNELS' parked reason re-verified
  (still no release.yml, only temper.yml; root package.json still the
  private flume manifest) and re-stamped at 3024687.
- Queue: PACKAGING-CHANNELS (parked) — nothing pickable.

Plan continues: yes — residue sweep: d029d4b trails HEAD; b3ffb52 is an
unswept code commit carrying a declared degrade-to-default residual.
