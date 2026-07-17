# Plan state

- Spec derived through: b8396d4
- Audited through: 8913b59
- Residue swept through: 8913b59
- This tick: POST-SHIP RECONCILIATION of `ec2b848..8913b59` — inbox empty,
  `.flume/refactor/` empty (README only), spec delta empty, so the window was
  the first live input. One build commit (d14f6fe), both motions ran; the spec
  cursor is copied forward verbatim.
  **Audit — CHECK-HARNESS-ARGS-ONE-HOME shipped and verified on disk; already
  dropped from the queue by its own ship commit.** Read the files, not the
  log. `common::check_harness_in` (`tests/common/mod.rs:200`) is the one home
  for the `--harness <root>` arg shape; both private per-file wrappers are
  gone (`rg 'fn run_check_harness|fn check_harness_in' tests/` finds only the
  home), and all eight sites reach it. `check_harness` (206) composes it for
  its github+findings projection rather than re-spelling the vector, so the
  ladder's three rungs now stack: `check_in` (176) spawns, `check_harness_in`
  (200) fixes the arg shape, `check_harness` (206) projects.
  **The acceptance's own enumeration was wrong, and the ship commit says so
  rather than quietly meeting it.** It claimed `rg '"--harness"' tests/` would
  return the home plus two non-instances; disk returns the home plus **three**
  (`emit.rs:1487`, `cli.rs:209`, and `cli.rs:137`). The third is my scoping
  error, not build's: `cli.rs:137` was `run_check_harness(harness,
  &["--deny-advisories"])`, and removing the wrapper's `extra` tail — which
  the entry's own scope bound demanded ("never a helper per arg permutation")
  — necessarily dropped it to `check_in` with an inline vector. The entry's
  intent is met and the work is done; one site spelling a three-element vector
  is not a duplicate. Dropped, with the enumeration miss named.
  **Sweep — the runner ladder has bottomed out, and the find is on a different
  axis.** I looked for the fifth rung and there isn't one: the surviving
  private wrappers (`gate_fail_loud::check_in` 29,
  `lock_declaration_rows::check_in` 1625, `memory_gate::check_two_step` 76)
  each compose `common::check_in` at their own file's arg shape and
  projection — compositions engineering.md sanctions, not second
  implementations. `Some("github")` at 15 sites is an argument, not a
  duplicated job. Filing a fold there would be manufactured work, so it is
  named here and not filed.
  What the sweep did find is the same rule on the **fixture** axis, and it is
  stronger evidence than the runner class ever had: `write_plugin_json` is
  **byte-identical** — body and doc comment — in `plugin_manifest_kind.rs:43`
  and `json_document_format.rs:57`; `write_settings` is byte-equivalent in
  `hook_kind.rs:52` and `installed_plugin_kind.rs:49` (only the local binding
  name differs); `coverage_note::write_settings_json` (61) is a third copy
  with the body hardwired, bypassed by its own two callers at 188/310; and
  `marketplace_kind::write_marketplace_json` (52) is the shape a sixth time.
  The tell is that `tests/common` **already homes this exact job** for two of
  the loci (`write_skill` 139, `write_rule` 280) — the manifest members'
  writers simply grew outside it. Second strand, same home:
  `requirement_roster::write_clauses` (44) re-spells `common::write_lock`'s
  (374) `Payload`/`drift::emit` body verbatim with `declarations` narrowed to
  clauses. No pending entry consolidates either ⇒ filed as
  **TEST-FIXTURE-WRITERS-ONE-HOME**, pickable. The entry names the two
  same-name-different-job pairs that must NOT fold (`write_claude_md`,
  `write_lock`), so build does not rediscover them one failure at a time.
  **Riders and parks re-verified on disk, none carried forward.** `git diff
  ec2b848..8913b59 -- src/ sdk/` is empty, so every `src/`/`sdk/` cite is
  unmoved by construction; all restamped to 8913b59. `MAX_IMPORT_HOPS` still 5
  at 65 under a cite claiming five; four era tags and no version tag, crate
  0.1.0 vs npm 0.0.7, `release.yml:7-9`'s darwin + channel-3 deferral
  verbatim. `marketplace_kind.rs:252` re-read (unmoved — the window edited
  that file at 200 one-for-one). The `session_start.rs:122/141` fixture
  question is unmoved and stays a question awaiting a human.
- Queue: 3 entries — 1 pickable (TEST-FIXTURE-WRITERS-ONE-HOME, scoped at
  8913b59), 2 parked on human acts. No file appears in two entries — checked
  mechanically; the pickable entry is tests-only and disjoint from both parks.

Plan continues: no — every input below post-ship reconciliation is dry (inbox
empty, no captures, spec delta empty) and this tick reconciled the window to
HEAD. Build takes over: TEST-FIXTURE-WRITERS-ONE-HOME is pickable.

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
