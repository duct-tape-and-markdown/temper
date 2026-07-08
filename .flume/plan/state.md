# Plan state

- Spec derived through: f87cc0c
- Audited through: d6c37a3
- Residue swept through: da9245c
- This tick: Ship audit (job 3). `git log 37a92f0..HEAD -- src/ tests/ sdk/`
  showed four commits: db34aea (SDK-vendoring scaffolding into
  tests/common), 4c3e473 (glob engines onto globset), e071af2 (clause-row
  decoders onto contract::predicate_from_row), 84ed106 (encodeSeam). Verified
  each on disk (not from the log): `tests/common::vendor_sdk` exists with
  the cited unix-symlink/Windows-junction split; `kind::compile_glob` exists
  and every prior caller resolves through it; `contract::predicate_from_row`
  exists and both call sites (builtin.rs, main.rs) use it; `encodeSeam`
  exists and backs both declarationsToJson and emit's seam. `cargo
  test`/`clippy -D warnings`/`fmt --check` all green (229 unit+integration
  tests). The build's own commit (d6c37a3) had already dropped the four
  shipped tags from pending.json; re-tested the two stale `blockedBy` gates
  those shippings named: GLOB-ENGINE-CONSOLIDATE unblocks
  PATH-NORMALIZER-CONSOLIDATE and PLURAL-HELPER-CONSOLIDATE,
  WINDOWS-VENDOR-SYMLINK-JUNCTION unblocks TEST-SCAFFOLDING-CONSOLIDATE —
  all three rewritten with corrected line numbers (glob's consolidation
  shifted graph.rs/coverage_note.rs; the windows entry shifted
  tests/install.rs/emit.rs). Discovered the three now-open entries pairwise
  share files (TEST-SCAFFOLDING-CONSOLIDATE touches import.rs/bundle.rs/
  coverage_note.rs/install.rs, each also claimed by one of the other two) —
  serialized into one chain (PATH-NORMALIZER-CONSOLIDATE open ->
  PLURAL-HELPER-CONSOLIDATE -> TEST-SCAFFOLDING-CONSOLIDATE ->
  TEST-FIXTURE-HELPERS-CONSOLIDATE, unchanged) since the schema's single
  `blockedBy` tag can't express "wait for both." Also found
  tests/common/mod.rs now exists on disk (WINDOWS-VENDOR-SYMLINK-JUNCTION
  created it) so it moved `new`->`edit` in both TEST-SCAFFOLDING-CONSOLIDATE
  and TEST-FIXTURE-HELPERS-CONSOLIDATE — left as `new` would fail the fence
  gate. And found tests/emit.rs, tests/install.rs, tests/builtin_lock_frozen.rs
  still each carry a local `tmpdir` the windows entry didn't touch (it only
  handled their sdk_root/vendor_sdk half) — added to TEST-SCAFFOLDING-CONSOLIDATE
  to close the 28-copy count exactly. PACKAGING-CHANNELS' parked condition
  re-checked: no `.github/workflows/release.yml`, root package.json still
  the private flume manifest, sdk still 0.0.5 — unchanged. Cursor advances
  to HEAD (d6c37a3).
- Queue: PATH-NORMALIZER-CONSOLIDATE (open) — PLURAL-HELPER-CONSOLIDATE
  (blockedBy path-normalizer) — TEST-SCAFFOLDING-CONSOLIDATE (blockedBy
  plural-helper) — TEST-FIXTURE-HELPERS-CONSOLIDATE (blockedBy
  test-scaffolding) — PACKAGING-CHANNELS (parked, condition unchanged).

Plan continues: yes — residue sweep (job 4) is next: inbox, spec delta, and
ship audit are all current, but `Residue swept through` (da9245c) still
trails HEAD.
