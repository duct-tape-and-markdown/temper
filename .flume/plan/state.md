# Plan state

- Spec derived through: b8396d4
- Audited through: 3593ab6
- Residue swept through: 3593ab6
- This tick: POST-SHIP RECONCILIATION of `d88ed75..3593ab6` — inbox empty,
  `.flume/refactor/` empty (README only), spec delta empty, so the window was
  the first live input. One build commit, both motions ran; the spec cursor is
  copied forward verbatim.
  **Audit — nothing to drop, the consolidation verified on disk.**
  CHECK-HARNESS-ONE-HOME was already dropped at 3593ab6, so the drop motion
  was pre-paid. Read the files, not the log: 3bd0f80's claim holds where it
  was scoped — one `check_harness` at `tests/common/mod.rs:178`, the six
  byte-identical copies gone (`rg 'fn check_harness' tests/` finds the home
  plus three test names and two thin `check_in` adapters, no second runner),
  `memory_gate`'s status-dropping variant gone with that file's last `BIN`,
  and `CheckRun.stdout` (152) real — so `cli.rs:170`'s which-stream assertion
  keeps full strength rather than being quietly downgraded to "either stream",
  exactly as the commit body claimed.
  **Sweep — one find, and it is the shipped entry's own remainder.** The
  commit body says "every caller reaches it". Eight `temper check` runs still
  hand-roll `Command::new(BIN)`: four in `cli.rs` itself (210, 248, 267, 286 —
  outside the entry's named scope but inside the file it consolidated), two in
  `emit.rs` (1483, 1495), one in `reporters.rs` (172, that file's only `BIN`
  use), one in `session_start.rs` (49). All four files already `mod common`,
  and every one is expressible as `common::check_in(cwd, args, reporter)` —
  its first parameter IS the cwd, so even `cli.rs:248`'s deliberate
  cwd-not-root spelling folds at full strength. Same class engineering.md
  names ("One job, one home"; test scaffolding lives in one home), no pending
  entry consolidating it ⇒ filed as **CHECK-RUNNER-REMAINDER**, pickable. One
  runner is honestly NOT foldable and the entry says so rather than
  overclaiming: `session_start.rs:360` drives `SESSION_START_COMMAND`'s own
  token stream, where `check` arrives as data inside `args`, not as the verb a
  runner prepends.
  **Reclassified the `kinds/`+`packages/` accepted debt — this board had it
  wrong.** It was filed as narration staleness riding "whichever entry next
  reconciles its file", and four entries had opened `session_start.rs` and
  left it. Reading the fixture rather than the record: 121/140 sit inside
  `stray_custom_kind_shaped_fixtures_never_disturb_a_clean_session_start`
  (111), whose *subject* is that retired-format files are inert — the
  vocabulary IS the assertion, not a comment beside it. So the rider's
  condition can never arrive: no hygiene pass reconciles it, and the live
  question (does temper still want this test at all?) is a value call no build
  tick may invent. Re-recorded as a question awaiting a human, not a rider
  awaiting a carrier — and CHECK-RUNNER-REMAINDER is scoped to leave 121/140
  standing, correctly, under that reading.
  The other three accepted-debt riders re-verified unmoved and restamped to
  3593ab6; the window touched none of their files (`git diff d88ed75..3593ab6
  -- src/ sdk/` is empty — the window is test-only) and no queued entry
  carries them. Both parks re-tested on disk: `MAX_IMPORT_HOPS` still 5 at 65
  under a cite claiming five; four era tags, no version tag, crate 0.1.0 vs
  npm 0.0.7, `release.yml`'s darwin + channel-3 deferral verbatim at 7-9.
- Queue: 3 entries — 1 pickable (CHECK-RUNNER-REMAINDER, scoped at 3593ab6),
  2 parked on human acts. No file appears in two entries — checked
  mechanically; the pickable entry is tests-only and disjoint from both parks.

Plan continues: no — every input below post-ship reconciliation is dry (inbox
empty, no captures, spec delta empty) and this tick reconciled the window to
HEAD. Build takes over: CHECK-RUNNER-REMAINDER is pickable.

**One thing for a human, unchanged and not the loop's:** decision 0030 is
still a hole — `specs/decisions/` runs 0023…0029, 0031, and 0030 (`review is
the price of softening`) is orphaned at d6381b4, reverted by this phase's own
`continuation marker is honest` gate firing on a human `specs:` commit.
Recoverable via `git show d6381b4`; John's alone to restore; the misfire is
filed at
`.flume/friction/plan-continuation-gate-reverts-human-specs-commits.md`.

**Waiting on a ruling:** `(clause-vocabulary-holds)` is still the fork board's
largest — four shipped contracts hold decidable, documented rules the algebra
cannot spell, and the corpus sanctions only "undecidable" as a reason a clause
is absent. Unmoved this window (test-only). Nothing is broken; what it costs
is that the gate's reach is thinner than `specs/builtins.md`'s "Strictest
documented profile" stance reads.
