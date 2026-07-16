# Plan state

- Spec derived through: abe5d5d
- Audited through: 8978596
- Residue swept through: 8978596
- This tick: RECONCILED cc8d823..8978596 — audit plus sweep in one tick, the
  window being two build commits wide. **Audit:** both shipped entries verify
  on disk, not merely in the log. LOCK-LEGACY-TEMPLATES-READ (08484b3):
  `templates_from_table` (src/drift.rs:3146) now lifts a bare string to
  `TemplateRow { kind, path: None }` and falls through to the canonical inline
  table, with an element outside both spellings still refusing
  (`RowError::wrong("templates", "array of tables or strings")`) — the 0024
  read-robustly shape exactly: the read normalizes, the committed file is
  untouched, the next emit rewrites the column whole. Both new cases in
  tests/lock_declaration_rows.rs (867-995) pin it, including the
  never-patched-in-place assertion. FIXTURE-WIRING-FOURTH-COPY (9948c03): the
  fourth copy is gone — tests/nested_member.rs:262 is now a single
  `common::wire_sdk_harness("nested-file-locus", NESTED_FILE_PROGRAM)` call.
  Both suites green (13 passed, 0 failed). Both entries were already absent
  from pending, removed by the ship commit 8978596 — a verification, not a
  drop. Both parks re-tested on disk at this HEAD and hold verbatim: no
  version tag (only the four era tags), crate 0.1.0 vs npm 0.0.7,
  release.yml:7-9 still deferring darwin + channel 3; `MAX_IMPORT_HOPS` still
  reads 5 at src/graph.rs:62 under a cite claiming five, and nothing ruled the
  hop semantics.
  **Sweep — no finding.** The consolidation is now closed rather than merely
  advanced: `rg` over tests/ finds every real-SDK fixture site routed through
  `common::wire_sdk_harness` (projection_path_seam.rs:176,
  builtin_lock_frozen.rs:80, nested_member.rs:262, emit.rs via its
  four-caller binding at 1215-1216 plus three direct calls) — no fifth copy.
  install.rs's nine `vendor_sdk` calls re-verified as the justified exception
  on disk, not on the prior tick's word: it never authors a fixture program,
  it scaffolds `harness.ts` through `install::run` and rewrites it after
  (732/879) — a different job. The new read path is one home: 3104 is the
  sole caller of `templates_from_table`, and no sibling column reads a second
  way. `respell_templates_column` (lock_declaration_rows.rs:893) is unique —
  the other `.lines()` helpers across tests/ filter `::`-prefixed finding
  lines, a different job. The corpus's own scale clause (pipeline.md, "The
  lock": a reap wave deleting every live projection, or a re-read dropping a
  whole declared layer, refuses) is built and needs no entry — `--teardown`
  at main.rs:150, both refusals at drift.rs:72-93, enforced at 1044-1066.
  Every named rider re-verified and unmoved, each still carrier-less — the
  window touched three files and none is a rider's home: session_start.rs
  121/140, read.rs 270/495/633/770/1172, prose.ts (law 5 at 6/141/258, law 8
  at 11, posture at 126/156/161/188/238), Cargo.toml 42-45, compose.rs:233 —
  stamps advanced to 8978596.
  Closing checklist: no open entry, so the disjointness gate is vacuous; the
  two parks share no file with each other. No fork resolved or opened — all
  six records re-read and their cites re-stamped, none contradicted by this
  window. Field lengths validated. `Spec derived through:` copied forward
  verbatim — this tick derived no spec; the delta is empty.
- Queue: 0 pickable; 2 parked on human acts (IMPORT-HOP-CAP-CITE: a hop-depth
  probe. PACKAGING-CHANNELS-REMAINDER: Apple notarizing + the v0.1 tag). No
  gate is stale — both were tested this tick.

Plan continues: no — inbox drained and empty, no refactor captures, spec delta
empty at abe5d5d, and cc8d823..8978596 is now reconciled on both motions with
both cursors advanced. Nothing is pickable: the queue's remaining two entries
are parked on acts only John can take. The loop hibernates.
