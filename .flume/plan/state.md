# Plan state

- Spec derived through: b8396d4
- Audited through: 8036900
- Residue swept through: 8036900
- This tick: POST-SHIP RECONCILIATION of `3593ab6..8036900` — inbox empty,
  `.flume/refactor/` empty (README only), spec delta empty, so the window was
  the first live input. One build commit (a9a21a9), both motions ran; the spec
  cursor is copied forward verbatim.
  **Audit — CHECK-RUNNER-REMAINDER shipped and verified on disk; dropped.**
  Read the files, not the log. Its acceptance ("exactly one hand-rolled runner
  survives in tests/") holds *mechanically*: 33 `Command::new(BIN)` sites
  remain in `tests/`, and every one drives a different verb — `bundle`,
  `explain`, `install`, `guard`, `emit`, `schema`, `--help`, the retired-verb
  loop. The sole surviving `check` is `session_start.rs:361`, the exception the
  entry named honestly up front (the verb arrives as data inside `args`). All
  eight scoped sites now reach `common::check_in` (`cli.rs` 210/246/264/279,
  `emit.rs` 1485/1496, `reporters.rs:167`, `session_start.rs:49`), and both
  behavior-preservation claims check out: `cli.rs:246` keeps the crate root as
  cwd with the harness as positional arg (full strength, not downgraded), and
  the stdout-specific assertions still read `CheckRun.stdout`. The
  beyond-scope claims are real too — `const BIN` is gone from `reporters.rs`
  and `acceptance.rs`, and `acceptance.rs`'s `check_from` (318) composes the
  home.
  **Sweep — one find, and it is the same miss one layer down.** The commit
  homed the `Command` *spawn*; the github finding-line extraction is still
  copy-pasted five times (`common/mod.rs:190` inside `check_harness`,
  `gate_fail_loud.rs` 34 and 179, `memory_gate.rs:80`, `bundle.rs:240`). Worse,
  `bundle.rs:236-241` is `check_harness(&out)` re-spelled byte-for-byte — a
  sixth copy of the home, regrown one commit after 3bd0f80 consolidated six
  copies of exactly it. Same class engineering.md names ("One job, one home"),
  no pending entry consolidating it ⇒ filed as **CHECK-FINDINGS-ONE-HOME**,
  pickable. The pattern is now legible and worth naming: 3bd0f80 homed the
  harness shape, a9a21a9 homed the spawn, each leaving the layer beneath it
  copied. The entry is scoped to keep what is genuinely *not* duplication —
  `gate_fail_loud`'s local wrapper and `memory_gate`'s `check_two_step` are
  compositions of the home at their own arg shapes, not rivals to it, and the
  pre-existing `check_in` name shadowing is named as out of scope rather than
  folded in silently.
  **Riders re-verified, and one corrected against disk.** The window folded a
  runner at `session_start.rs:49`, which shifted the `kinds/`+`packages/`
  fixture cites two lines (121/140 → 122/141, fn 111 → 113) — the record is
  re-read, not carried forward. That same fold *proves* last tick's
  reclassification rather than merely predicting it: CHECK-RUNNER-REMAINDER
  opened the file and correctly left the fixtures standing, the third entry to
  do so. It stays a question awaiting a human, not a rider awaiting a carrier.
  The other three riders are unmoved and restamped to 8036900 — `git diff
  3593ab6..8036900 -- src/ sdk/` is empty, so every `src/`/`sdk/` cite is
  unmoved by construction. Both parks re-tested on disk: `MAX_IMPORT_HOPS`
  still 5 at 65 under a cite claiming five; four era tags and no version tag,
  crate 0.1.0 vs npm 0.0.7, `release.yml`'s darwin + channel-3 deferral
  verbatim at 7-9.
- Queue: 3 entries — 1 pickable (CHECK-FINDINGS-ONE-HOME, scoped at 8036900),
  2 parked on human acts. No file appears in two entries — checked
  mechanically; the pickable entry is tests-only and disjoint from both parks.

Plan continues: no — every input below post-ship reconciliation is dry (inbox
empty, no captures, spec delta empty) and this tick reconciled the window to
HEAD. Build takes over: CHECK-FINDINGS-ONE-HOME is pickable.

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
is absent. Unmoved this window (tests-only). Nothing is broken; what it costs
is that the gate's reach is thinner than `specs/builtins.md`'s "Strictest
documented profile" stance reads.
