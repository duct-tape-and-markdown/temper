# Plan state

- Spec derived through: abe5d5d
- Audited through: cc12ed0
- Residue swept through: cc12ed0
- This tick: RECONCILED the ca4e866..cc12ed0 window — one code commit
  (9f3541c, INDETERMINATE-NEVER-SILENT; cc12ed0 is its `chore(flume)` ship).
  **Audit — verified on disk, not off the log.** The fence landed as the entry
  named it: `Facet` (`src/engine.rs:112-121`) splits `inadmissibilities`
  (132) into `judgeless` (144-175) + `vacuities` (177+), so the five
  node-set/edge-scope predicates are inadmissible on a kind's per-artifact
  contract and stay admissible on a requirement's — `roster::admissibility`
  passes `Facet::Requirement` at `src/roster.rs:224`, so the half that works
  is intact. The trap the entry flagged (a naive push onto the shared list)
  was avoided. Coverage is real: `gate_fail_loud.rs` (a `count` floor clause
  on `skill` fails the run, error-severity, non-zero exit) and the engine
  test at 1095-1110 pinning that no admissible predicate reaches conformance
  undecided. Entry already dropped from the queue by the ship commit; nothing
  to drop here. **Stale gate re-tested:** UNFILLED-EDGE-FIELD-NO-EDGE's
  blocker shipped → re-gated `blockedBy` → **open**. It is now the queue's one
  pickable.
  **Sweep — the window's one residue, and it was declared, not hidden.**
  9f3541c's commit body states a deferral NOT covered by its acceptance:
  `evaluate` (`src/engine.rs:256-268`) still maps `Outcome::Indeterminate`
  onto the empty violation vec, and `decide`'s `FormatPlacesEdges` arm
  (513-516) produces exactly that whenever `features.edge_placements` is
  empty — the sole `Indeterminate` an admissible run still reaches, so one
  clause still reads as the green pass invariant 6 forbids. The build's
  argument is sound and I did not overturn it: `Features::edge_placements`
  (`src/extract.rs:341`) is a `BTreeMap` that is empty for two different
  reasons — "the kind declares no edge" and "no format rendered this member"
  (a layout host's document is source) — and only `main.rs`'s construction
  (1480-1516) can tell them apart, so a finding forged in the engine would be
  fabricated. It handed the fix to UNFILLED-EDGE-FIELD-NO-EDGE **by name**,
  and that entry did not carry it — a deferral pointed at an entry that does
  not operationalize it is a silent gap on the next tick. Scoped in now:
  `src/engine.rs` and `src/extract.rs` join its `files`, with a
  `gate_fail_loud.rs` test, and the acceptance names the residual. No fork
  owed — whether the formatless member reads as excluded from the selection
  or as a vacuous hold is mechanism (spec-system.md, "Depth rule"): both
  spell the same verdict, no finding.
  **Two entries rewritten, not patched** (9f3541c rewrote 449 lines of
  engine.rs, so every line cite into it was stale): UNFILLED-EDGE-FIELD-NO-EDGE
  (re-gated, main.rs anchors corrected, the deferral scoped) and
  PREDICATE-SELECTION-ALGEBRA (its engine.rs/graph.rs descriptions now cite
  `judgeless`/`Facet`/`kind_violation` at live anchors, and the two fence-test
  surfaces 9f3541c added — `contract_template.rs`'s inverted named-`kind` case,
  `gate_fail_loud.rs`'s new fixture — enter its ripple, since the fence is what
  it retires; the fail-loud bar itself stands on `dependency-exists`,
  judge-less in both facets). Its notes claimed a shared `src/main.rs` with
  UNFILLED-EDGE-FIELD-NO-EDGE — false, the file is not in its `files`; the real
  overlap is engine.rs / contract_template.rs / gate_fail_loud.rs, and the
  `blockedBy` was already correct.
  **Three riders found carriers**, each named in the carrying entry's own
  `files[].description` — the only shape that discharges one (the lesson this
  board records): `extract.rs:196-198`'s resolved-to-never floor-mention
  comment and `extract.rs:774`'s @07-02 cite ride UNFILLED-EDGE-FIELD-NO-EDGE;
  `graph.rs:61/689`'s @07-02 cites ride PREDICATE-SELECTION-ALGEBRA.
  Closing checklist: the queue is disjoint — one chain of five
  (UNFILLED-EDGE-FIELD-NO-EDGE → NESTED-FILE-LOCUS → NESTED-FILE-DISCOVERY →
  SKILL-NESTED-REFERENCE-DOCS → SDK-FIXTURE-WIRING-ONE-HOME), a second forking
  off it (PREDICATE-SELECTION-ALGEBRA → SUPPORTING-DOC-REACH-CLAUSE,
  file-disjoint from the NESTED-FILE chain so the two run in parallel), plus
  the park. Fork board: four open, none blocking a queued entry. The
  SUPPORTING-DOC-REACH-CLAUSE two-blocker gap stands as last tick recorded it
  — one `gate` tag, the second blocker in `notes`; unchanged this tick, and it
  is the queue tail either way. This tick wrote no `src/`, `tests/`, or `sdk/`
  file.
- Queue: 1 pickable (UNFILLED-EDGE-FIELD-NO-EDGE); six serialized behind it as
  two chains forking after it; PACKAGING-CHANNELS-REMAINDER parked (John's
  Apple notarizing + the v0.1 lockstep tag).

Plan continues: no — reconciliation was the last live input and it is done.
The inbox is empty, no refactor capture is live, and the spec delta is empty
(cursor abe5d5d is the last `specs:` commit). Both code cursors now sit at
cc12ed0 = HEAD. Build takes over with UNFILLED-EDGE-FIELD-NO-EDGE.
