# Plan state

- Spec derived through: b8396d4
- Audited through: ec2b848
- Residue swept through: ec2b848
- This tick: POST-SHIP RECONCILIATION of `8036900..ec2b848` — inbox empty,
  `.flume/refactor/` empty (README only), spec delta empty, so the window was
  the first live input. One build commit (f9aa32b), both motions ran; the spec
  cursor is copied forward verbatim.
  **Audit — CHECK-FINDINGS-ONE-HOME shipped and verified on disk; already
  dropped from the queue by its own ship commit.** Read the files, not the
  log. Its acceptance holds *mechanically*: `rg 'starts_with("::")'` over
  `src/` and `tests/` returns exactly one hit — `CheckRun::findings`
  (`tests/common/mod.rs:157`) — so all five copies of the github finding-line
  filter are gone and the home is a method on the type that owns the bytes.
  `check_harness` (193) composes it at 199 rather than open-coding it, and
  `tests/bundle.rs:236` — the sixth copy, `check_harness` re-spelled
  byte-for-byte — now calls `check_harness` and uses both halves it returns.
  The two deliberate non-folds are real compositions, not survivors:
  `gate_fail_loud::check_in` (29) and `memory_gate::check_two_step` (76) each
  call `common::check_in(…, Some("github")).findings()` at their own arg
  shape. The commit body's honesty about the pre-existing `check_in` name
  shadowing checks out — `gate_fail_loud.rs:29` and
  `lock_declaration_rows.rs:1625` both still shadow `common::check_in` with
  different signatures, named as out of scope rather than folded silently.
  **Sweep — one find, and it is the same miss one layer further down.** The
  window homed the finding *filter*; the `--harness <root>` one-shot **run**
  is still re-spelled at eight sites (`common/mod.rs:196`, `cli.rs:96`,
  `requirement_roster.rs:45`, `marketplace_kind.rs:200`,
  `plugin_manifest_kind.rs` 142/199, `json_document_format.rs` 218/254). Two
  of them — `cli::run_check_harness` (95) and
  `requirement_roster::check_harness_in` (44) — are private per-file wrappers
  for one job, byte-equivalent in the `&[]` case: exactly what engineering.md
  names ("shared fixtures and builders live in one home (`tests/common`),
  never copy-pasted per file"). `common::check_harness` *is* that home but
  hardwires `Some("github")` and projects to `(findings, ok)`, so the callers
  wanting the `CheckRun` back cannot reach it and the arg vector regrew
  against `check_in` instead — generalize the near-duplicate, the ladder's
  third rung. No pending entry consolidates it ⇒ filed as
  **CHECK-HARNESS-ARGS-ONE-HOME**, pickable. The pattern is now three deep and
  worth stating plainly: 3bd0f80 homed the harness shape, a9a21a9 the spawn,
  f9aa32b the finding filter — each fold homed one layer and left the layer
  beneath it copied. The entry names its own scope bound (the home is this ONE
  arg shape, never a helper per arg permutation) and the two sites that must
  NOT fold (`emit.rs:1485` runs from `CARGO_MANIFEST_DIR` because
  cwd-independence is its subject; `cli.rs:212` is the both-routes usage
  error), so build does not rediscover them one failure at a time.
  **Riders re-verified, and one corrected against disk.** `git diff
  8036900..ec2b848 -- src/ sdk/` is empty, so every `src/`/`sdk/` cite is
  unmoved by construction; all are restamped to ec2b848. The correction is in
  the `(clause-vocabulary-holds)` record: it cited
  `the_rules_below_the_top_level_are_not_gateable_today` at
  `tests/marketplace_kind.rs:267`, and the fn has been at **252** since at
  least 3593ab6 — the record was wrong when written, not drifted. Re-read on
  disk, not carried forward. Both parks re-tested: `MAX_IMPORT_HOPS` still 5
  at 65 under a cite claiming five; four era tags and no version tag, crate
  0.1.0 vs npm 0.0.7, `release.yml`'s darwin + channel-3 deferral verbatim at
  7-9. The `session_start.rs:122/141` fixture question is unmoved (the window
  never opened the file) and stays a question awaiting a human.
- Queue: 3 entries — 1 pickable (CHECK-HARNESS-ARGS-ONE-HOME, scoped at
  ec2b848), 2 parked on human acts. No file appears in two entries — checked
  mechanically; the pickable entry is tests-only and disjoint from both parks.

Plan continues: no — every input below post-ship reconciliation is dry (inbox
empty, no captures, spec delta empty) and this tick reconciled the window to
HEAD. Build takes over: CHECK-HARNESS-ARGS-ONE-HOME is pickable.

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
