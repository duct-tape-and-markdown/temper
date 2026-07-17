# Plan state

- Spec derived through: b8396d4
- Audited through: 18f219d
- Residue swept through: 18f219d
- This tick: REFACTOR DRAIN — `build-check-harness-copies.md` (the tick's first
  live input: inbox empty, but the build window filed a capture, so job 1 beat
  the unreconciled window). Filed **CHECK-HARNESS-ONE-HOME**, deleted the
  capture. All three cursors copied forward verbatim — this tick audited,
  swept, and derived nothing.
  **Claim re-verified at HEAD, not taken on report.** The capture (observed
  2e27bdc) holds intact: all six `check_harness` bodies still byte-identical
  and unmoved (`coverage_note` 99, `hook_kind` 64, `installed_plugin_kind` 61,
  `marketplace_kind` 64, `mcp_server_kind` 59, `plugin_manifest_kind` 55), each
  a thin wrapper over `common::check_in` (156). `git diff 2e27bdc..HEAD` over
  the seven named files touched only `plugin_manifest_kind.rs` (7e266ad, +37;
  fn unmoved at 55).
  **Scoped to the verified gap, which is wider than the reported one.**
  `rg 'fn check_harness' tests/` found two homes the capture missed, and they
  are the worse class: `requirement_roster.rs:44` (`check_harness_in`) and
  `cli.rs:95` (`run_check_harness`) do not wrap the shared home at all — they
  hand-roll the runner over their own `BIN`, reimplementing `check_in`'s body
  (Command, stdout+stderr concat, `CheckRun` construction). `requirement_roster`
  does it in a file already calling `common::check_in` 17 times. So the job has
  **four** implementations, not one-plus-copies. `cli.rs` carries the one honest
  wrinkle — stdout-only where `check_in` merges stderr — named in the entry so
  build states it rather than papering it over.
  **Root cause named in the entry, not left as comment residue.**
  `check_in`'s own doc (`tests/common/mod.rs:152-155`) *licenses* the copying in
  as many words — "Callers that need a different return shape (a `(bool,
  String)` pair, a parsed `Vec<String>` of `::`-prefixed finding lines) adapt
  from [`CheckRun`] at the call site" — naming the two exact shapes that got
  copied six and three times. That sentence is the rule the entry changes, not
  staleness riding along: retire it or the copies regrow.
  **Both parks re-tested on disk and hold**, restamped 9005fc1:
  `git diff 18f219d..HEAD -- src/graph.rs tests/graph.rs` is empty,
  `MAX_IMPORT_HOPS` still 5 at 65 under a cite claiming five, 525/624/649-654
  and `tests/graph.rs:1356` unmoved, nothing ruled the hop semantics; four era
  tags and no version tag, crate 0.1.0 vs npm 0.0.7, release.yml:7-9 states the
  darwin + channel-3 deferral verbatim. 9005fc1 had already dropped the three
  shipped entries, so the drop motion was pre-paid.
- Queue: 3 entries — 1 pickable (CHECK-HARNESS-ONE-HOME, `tests/common/mod.rs`
  + nine suites), 2 parked on human acts. No file appears in two entries —
  checked mechanically; the pickable entry is disjoint from both parks
  (`tests/graph.rs` is untouched by it, `release.yml` unrelated).

Plan continues: yes — post-ship reconciliation of `18f219d..9005fc1` (7e266ad,
6dbeb86, 0e7dca2: the `type` clause's first consumer, the `json-document` body
refusal, bundle's manifests through their kinds). Unreconciled: both cursors
still read 18f219d, and this tick spent its one job on the capture that window
filed. Next tick audits and sweeps it — including two routing pointers this
tick deliberately did not chase: `(closed-surface-predicate)`'s record says
"TYPE-CLAUSE-CONSUMER is scoped to leave that bullet standing" and
`(nested-field-addressing)` leans on BUNDLE-EMIT-THROUGH-KINDS, and **both of
those entries have now shipped** — the conditions need re-testing against disk,
which is the audit motion's job, not this one's.

**One thing for a human, unchanged and not the loop's:** decision 0030 is
still a hole — `specs/decisions/` runs 0023…0029, 0031, and 0030 (`review is
the price of softening`) is orphaned at d6381b4, reverted by this phase's own
`continuation marker is honest` gate firing on a human `specs:` commit.
Recoverable via `git show d6381b4`; John's alone to restore; the misfire is
filed at
`.flume/friction/plan-continuation-gate-reverts-human-specs-commits.md`.
