# Plan state

- Spec derived through: f87cc0c
- Audited through: c93eeed
- Residue swept through: e45d424
- This tick: Ship audit (job 3) — one src/tests commit past the old cursor
  (69ba0fe): e5daf1d shipped BUILTIN-KIND-TEMPLATES-OVERLAY. Verified on disk,
  not just the log: src/kind.rs gained `overlay_templates`/the
  `Fenced`-primitive append, src/main.rs threads `overlay_builtin_kind`
  through `kind_features` (and so `gate`'s built-in dispatch +
  `builtin_features_by_kind`) and `resolve_kind_units`, and
  tests/lock_declaration_rows.rs's new case
  (`a_lock_declared_templates_row_folds_a_builtin_hosts_member_fence`) plus
  the full suite pass (17/17). The chore(flume) ship commit (c93eeed) already
  dropped the entry from pending.json — nothing left to prune there. Re-tested
  the one stale gate in the queue: PACKAGING-CHANNELS's parked reason —
  `.github/workflows/release.yml` still absent (only `temper.yml`, a check
  job), root `package.json` still the private flume manifest — condition
  still holds; restamped its "Re-verified at" to c93eeed.
- Queue: PACKAGING-CHANNELS (parked, package.json + a new release.yml) — the
  only pending entry.

Plan continues: yes — residue sweep (job 4) is next live: `Residue swept
through` (e45d424) trails HEAD (c93eeed), and e5daf1d touched src/kind.rs +
src/main.rs since that sweep.
