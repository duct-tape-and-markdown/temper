# Plan state

- Spec derived through: f87cc0c
- Audited through: dd7517a
- Residue swept through: fba4e32
- This tick: Ship audit (job 3). 33874ac (build) + dd7517a (chore ship) landed
  past the 2c853af cursor. Verified on disk: gate() and explain() both now
  route through builtin_features_by_kind + assemble_by_kind over every
  builtin_kind::definitions() entry (main.rs:400-438, 890-919) — no more
  hardcoded skill/rule pair; tests/requirement_roster.rs retargeted to test
  a genuinely unmodeled kind; `cargo test` all-green. dd7517a's ship commit
  already removed the entry from pending.json — no rewrite needed. Re-tested
  PACKAGING-CHANNELS's parked reason at dd7517a: still no
  .github/workflows/release.yml (only temper.yml); root package.json still
  the private temper-flume-harness manifest; sdk/package.json @dtmd/temper
  still at 0.0.5 — reason holds, restamped.
- Queue: PACKAGING-CHANNELS parked, sole entry.

Plan continues: yes — residue sweep (job 4) is next; the residue cursor
(fba4e32) still trails HEAD (dd7517a, two src/tests-touching commits ahead).
