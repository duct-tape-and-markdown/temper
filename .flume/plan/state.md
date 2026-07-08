# Plan state

- Spec derived through: cd7135b
- Audited through: d4cd903
- Residue swept through: 3e19f4c
- This tick: Ship audit (job 3). Commits past d7d0912 touching src/tests/sdk:
  1251112 (build: RETIRE-OWN-PATH-MACHINERY) + d4cd903 (chore ship, drops the
  entry from pending.json). Verified on disk, not trusted: `rg own_path`
  across src/tests/sdk now resolves only inside historical comments
  (install.rs, tests/install.rs narrating the retired lift) and
  `specs/decisions/0016/0017` — no live column/detection/exemption code
  remains; `cargo build` + `cargo test --lib` (229 passed) + `cargo test
  --test install --test emit` (19 passed) all green. Re-tested
  PACKAGING-CHANNELS's parked facts, unchanged: no
  `.github/workflows/release.yml` (only temper.yml), root package.json still
  `temper-flume-harness`/private, sdk/package.json still `@dtmd/temper`
  0.0.5. No open-question rides own-path. Cursor advances to d4cd903 (HEAD).
- Queue: PACKAGING-CHANNELS parked on human release creds + engine-binary
  workflow — the only pending entry.

Plan continues: yes — residue sweep cursor (3e19f4c) trails HEAD (d4cd903);
commits 1251112/d4cd903 are unswept residue-sweep input for the next tick.
