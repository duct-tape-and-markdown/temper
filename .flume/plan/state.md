# Plan state

- Spec derived through: a571973
- Audited through: b3a1636
- Residue swept through: b3a1636
- This tick: RECONCILE 53519b9..b3a1636 — the first live input at orientation
  (inbox empty, `.flume/refactor/` its README alone, spec delta dry when the
  prompt rendered). **Audit's one finding, and it is the tick: the ship was a
  phantom.** b3a1636 (`chore(flume): ship EXAMPLE-EDGE-TARGET-SET-SPELLING`)
  dropped the entry from the queue, but **no commit in the window touched
  `examples/`** — `git log -- examples/` is unmoved since f19f49b, and both
  fields still read the retired bare-string spelling on disk
  (`kinds.ts:135` `to: "source"`, `:182` `to: "decision"`; `lock.toml:320/326`
  the same). The window's only build commit (9275f15) spent itself filing a
  friction capture and landed no code, and the ship fired anyway. **Entry
  re-filed, re-scoped at b3a1636**, its two line cites and the SDK tuple
  (`kind.ts:53`) re-read rather than trusted, with one thing added that the
  friction bought: the lock's `files[].description` now states the cwd rule
  (`emit` with cwd = `examples/base-harness`, never `--into` from the repo
  root, which exits 0 while re-basing every `source_path` row — 20 corrupted
  rows beside the 2 owed) and predicts the 2-row diff, so the next build reads
  the fact instead of re-paying for it. **Sweep: dry, and trivially so** —
  `git diff --name-only 53519b9..b3a1636` touches `.flume/` alone; no `src/`,
  `tests/`, or `sdk/` byte moved, so no code-vs-corpus gap could open. All four
  riders re-verified by reading and unmoved, stamps advanced to b3a1636:
  `session_start.rs:121/140`, `read.rs:270/495/633/770/1172`, `prose.ts`
  (6/11/126/141/156/161/188/200/238/258), `Cargo.toml:42-45` — none has a
  carrier; no queued entry opens any of them. **Both parks re-tested on disk,
  both hold:** `MAX_IMPORT_HOPS` reads 5 at `graph.rs:65` under a cite claiming
  five and nothing ruled the hop semantics; `git tag -l` carries the four era
  tags and no version tag, crate 0.1.0 vs npm 0.0.7, `release.yml:7-9` states
  the darwin + channel-3 deferral verbatim. Spec cursor copied forward
  verbatim: this tick derived no spec.
- Queue: 3 entries — 1 pickable (EXAMPLE-EDGE-TARGET-SET-SPELLING, alone in
  `examples/**`, disjoint from everything queued); 2 parked on human acts
  (IMPORT-HOP-CAP-CITE: a hop-depth probe. PACKAGING-CHANNELS-REMAINDER: Apple
  notarizing + the v0.1 tag). No file appears in two entries.

Plan continues: yes — **the spec delta went live mid-tick and is the next
input.** 16ce347 (`specs: a plugin registers by enablement (0031)`) landed on
top of b3a1636 while this tick was reconciling; it sits past the spec cursor
a571973, underived. It resolves `(plugin-surface)` and John's commit body ends
"plan derives the entries": the roster grows seven → ten (`installed-plugin`
registration member, `plugin-manifest`, `marketplace`), consolidating the
hard-coded `serde_json` writers at `src/bundle.rs:185,191`. Its Consequences
section is the next tick's derivation checklist and deserves the whole tick —
this tick declines to half-derive an 88-line decision it did not orient on.
John deleted the `(plugin-surface)` fork record himself in that commit; this
tick left the deletion alone. **Unrelated, and for a human rather than the
loop: decision 0030 is a hole.** `specs/decisions/` runs 0023…0029 then 0031,
and 0030 (`review is the price of softening`, 100 lines + 34 of `pipeline.md`)
is orphaned at d6381b4 — committed, then reverted by this phase's own
`continuation marker is honest` gate, which fired on a human `specs:` commit
because plan's then-current `state.md` said `Plan continues: no`. The work is
recoverable (`git show d6381b4`), is John's alone to restore, and the gate
misfire is filed at
`.flume/friction/plan-continuation-gate-reverts-human-specs-commits.md`.
