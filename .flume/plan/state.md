# Plan state

- Spec derived through: 3c1a58c
- Audited through: 9409a6c
- Residue swept through: 9409a6c
- This tick: RECONCILE c370924..9409a6c — the three-entry wave shipped, audited
  and swept in one pass; both cursors advance. The spec cursor is copied
  forward verbatim: 3c1a58c was fully routed and `<spec-delta>` is empty.
  **Audit.** All three verified on disk, not off the log. cf270d3:
  `harness_root_of` (`src/drift.rs`:624) floors to `.` and every row is spelled
  through `harness_relative` (1544), the root joined back on only where disk is
  touched (emit_one 1291, emit_manifest 1420, the reap). 42a2dd1: `blocks()`
  (`sdk/src/prose.ts`:246) refuses a `file()` block, discriminating on shape —
  `isFile` (267) tests `moduleUrl`, not the free-form `kind` tag. 9c25b6d:
  `write_mcp_json` (`tests/common/mod.rs`:170) composes `write_sibling` (141)
  and both private copies are gone (`grep` for `fn write_mcp*` outside
  `common/mod.rs` is empty; three callers compose it). cf270d3's commit body
  names the `.` floor as *found while wiring* — read on disk, it was **fixed,
  not deferred**, so no corner is unfiled. **Both parks re-tested and hold**:
  `MAX_IMPORT_HOPS` still reads 5 at `src/graph.rs`:65 under a cite claiming
  five, every cite unmoved (`git log c370924..HEAD -- src/graph.rs
  tests/graph.rs` is empty); `git tag -l` carries only the four era tags, crate
  0.1.0 vs npm 0.0.7, release.yml:7-9 states the darwin + channel-3 deferral
  verbatim.
  **Sweep.** One find, and it is ride-only — no entry: `src/roster.rs`:465's
  doc comment cites `10-contracts.md`, a file 0001 deleted. It is the last of
  the class the retired `prose.ts` record tracked, which 42a2dd1 discharged in
  full (all ten narration lines cut; `rg` finds none). Filed to open-questions
  beside the `Cargo.toml`:42-43 cite — neither has a carrier, since the queue's
  two parked entries open neither file. **The CLAUDE.md fixture axis was again
  NOT filed**: 8913b59 ruled it out by name and 5aa8348 re-affirmed it; the
  writers are different jobs (pad-to-N vs verbatim) sharing one `fs::write`
  line, and install.rs's four are one-liners at varied paths, not a locus
  writer's job. A deliberate exclusion is not re-opened without evidence, and
  none moved. The four shipped manifest loci now each have exactly one home.
- Queue: 2 entries, **0 pickable** — IMPORT-HOP-CAP-CITE and
  PACKAGING-CHANNELS-REMAINDER, both parked on human acts. Disjoint
  (`src/graph.rs`+`tests/graph.rs` vs `.github/workflows/release.yml`).

Plan continues: no — every input is drained and the queue has no pickable
work. Inbox empty, no refactor captures, `<spec-delta>` empty at 3c1a58c, both
reconciliation cursors at HEAD. **The loop hibernates: it cannot self-start.**
Both parked entries need John (the hop-semantics probe; Apple notarizing + the
v0.1 tag), and the two forks that gate real capability — `(layer-delivery-format)`
holding all four of 0030's derivations, `(clause-vocabulary-holds)` holding four
decidable-but-unexpressible gaps — need rulings. Nothing is broken; what is
stalled is new work.
