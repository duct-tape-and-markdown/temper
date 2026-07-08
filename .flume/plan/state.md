# Plan state

- Spec derived through: f87cc0c
- Audited through: f6ec58f
- Residue swept through: 99533af
- This tick: Inbox (job 1). The two Windows field-report findings (filed by
  human 352477d, reverted once at 884a008 for a fence violation, fence
  widened to `.gitattributes` by human d7e35f6) re-verified live on disk
  before scoping: `src/install.rs:1311` `npm_program()` still private,
  `tests/common/mod.rs:58` still spawns bare `Command::new("npm")`; no
  `.gitattributes` exists; 20 `.snap` files and 11 `tests/fixtures/**` files
  confirmed (counts unchanged from the reverted attempt). Filed
  WINDOWS-NPM-SPAWN-PUB (per `specs/process/engineering.md`, "One job, one
  home") and GITATTRIBUTES-LF-PIN (per `specs/model/pipeline.md`, "Emit"),
  both open, disjoint from each other and from PACKAGING-CHANNELS. Inbox
  drained. Re-verified PACKAGING-CHANNELS's parked reason still holds at
  d7e35f6 (no release.yml, package.json still private flume manifest). Jobs
  2-4 confirmed quiet this tick: `git log f87cc0c..HEAD -- specs/`,
  `git log f6ec58f..HEAD -- src/ tests/ sdk/`, and
  `git log 99533af..HEAD -- src/ tests/ sdk/` all empty.
- Queue: WINDOWS-NPM-SPAWN-PUB open, GITATTRIBUTES-LF-PIN open, both
  disjoint from PACKAGING-CHANNELS (parked, unchanged).

Plan continues: no — inbox now empty; jobs 2-4 (spec delta, ship audit,
residue sweep) all confirmed quiet this tick. Build takes over the two open
entries.
