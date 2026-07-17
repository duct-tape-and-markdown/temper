# Plan state

- Spec derived through: a571973
- Audited through: 53519b9
- Residue swept through: 53519b9
- This tick: RECONCILE 01337ff..53519b9 — two build commits
  (MENTION-REACHABLE-RULE-CLAUSE, EDGE-TARGET-SET), audited and swept in one
  pass. **Audit: both shipped real, verified on disk.**
  `mentionReachable("paths", "paths")` is adopted into `rule`'s default
  contract at `sdk/src/builtins.ts:667` — advisory, guidance carrying the
  literal-containment leniency and both remedies, cite re-fetched 2026-07-16
  with the probe evidence (2.1.211) recorded rather than softened — and it
  matches the field-argument reading last tick re-worded the entry to, not the
  superseded literal one. `Edge.to` is `Vec<String>` at `compose.rs:55`, with
  the empty set refusing at load and the SDK holding the same bar in the type
  (`readonly [string, ...string[]]`, `kind.ts:53`). Both entries dropped by
  their ship commits; nothing left to drop. **Both parks re-tested and both
  hold:** no ruling touched the hop semantics (dry delta, empty inbox), and no
  release act landed — `git tag -l` carries the four era tags alone, crate
  0.1.0 vs npm 0.0.7, `release.yml:7-9` states the darwin + channel-3 deferral
  verbatim. **Sweep's one finding, filed: EXAMPLE-EDGE-TARGET-SET-SPELLING.**
  3db8c25's own disclosure named `examples/base-harness/.temper/kinds.ts` as
  still authoring the pre-set `to` spelling and called it "outside this fence"
  — that half is wrong on disk: `examples/**` sits inside build's writable
  fence (`.flume/chain.ts:223`), so it routes as an ordinary entry rather than
  waiting on a human `docs(example):` act. Both fields (135, 182) are type
  errors against the SDK tuple today; they are latent, not red, because
  **nothing gates the example tree** — `rg` over `tests/`, `src/`, `sdk/`,
  `.github/` finds no reference to `examples/` at all, which is why the
  widening landed green with its own exemplar stale. **One rider discharged,
  and its record is deleted:** EDGE-TARGET-SET carried the `compose::effective`
  straggler named in its `files[].description` and re-worded it off the dead
  symbol — `rg effective src/compose.rs` is now clean, and with the
  `contract.rs` half already closed the whole record dies. The precedent it
  cost three entries to learn holds a fourth time: name the rider in the entry,
  or opening the file discharges nothing. Spec cursor copied forward verbatim:
  the delta is dry and this tick derived no spec.
- Queue: 3 entries — 1 pickable (EXAMPLE-EDGE-TARGET-SET-SPELLING, alone in
  `examples/**` and disjoint from everything queued); 2 parked on human acts
  (IMPORT-HOP-CAP-CITE: a hop-depth probe — park holds, cites re-derived by
  reading at graph.rs:65/525-526/624-625/649-652 and tests/graph.rs:1356, which
  1b9a6af moved when it lifted that file's helpers into `tests/common`.
  PACKAGING-CHANNELS-REMAINDER: Apple notarizing + the v0.1 tag — park unmoved).
  The queue's one file overlap is EDGE-TARGET-SET × IMPORT-HOP-CAP-CITE on
  `src/graph.rs` — **retired this tick**: EDGE-TARGET-SET shipped, so the
  parked entry is the sole claimant of that file and its cites are current.

Plan continues: no — the inbox is empty, `.flume/refactor/` holds its README
alone, the spec delta is dry (`git log a571973..HEAD -- specs/` is empty), and
this tick reconciled the window to HEAD, so no input below it is live. Build
takes over: one entry is pickable. The one friction capture on disk
(`build-import-hop-cap-cite-disagrees-with-live-docs.md`) is already fully
routed as IMPORT-HOP-CAP-CITE's park reason and is the human's to read.
