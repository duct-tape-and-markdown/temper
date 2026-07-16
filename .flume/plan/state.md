# Plan state

- Spec derived through: abe5d5d
- Audited through: 9862b2e
- Residue swept through: 9862b2e
- This tick: RECONCILED the cc12ed0..9862b2e window — one code commit
  (8b3b4e9, UNFILLED-EDGE-FIELD-NO-EDGE; 9862b2e is its `chore(flume)` ship).
  **Audit — verified on disk, not off the log.** The entry landed: refusal
  retreated to a filled-yet-unresolvable reference, and the obligation now
  ranges over the edges a value fills. The declared deferral 9f3541c handed
  it **closed**: `Features::edge_placements` (`src/extract.rs:345`) is an
  `Option`, the lift (`src/main.rs:1512`) carries apart "no format rendered
  this member" (`None`) from "nothing to place" (`Some`, empty), and
  `decide`'s `FormatPlacesEdges` arm (`src/engine.rs:510-528`) decides both —
  no `Indeterminate`. Invariant 6 is now *pinned*, not excepted by comment:
  the vocabulary sweep (`engine.rs:1087-1104`) asserts no admissible
  predicate reaches conformance undecided, and `FormatPlacesEdges` is in its
  list. `contract_template.rs` (400/416) pins both halves. Entry already
  dropped by the ship commit. Its two extract.rs riders discharged as the
  entry named them — 196-198 now states the leaf's `String` as permanent
  (0020), and the memory-doc cite re-retrieved @07-16; both re-verified on
  disk, so **both records leave open-questions**.
  **Stale gates re-tested — the blocker shipped, so two entries re-gate.**
  NESTED-FILE-LOCUS and PREDICATE-SELECTION-ALGEBRA both named
  UNFILLED-EDGE-FIELD-NO-EDGE. Both **rewritten, not patched**: 8b3b4e9
  rewrote emit.ts/kind.ts/engine.rs/contract_template.rs, so every line cite
  into them was stale (`Locus` 49-59 → 57-59, `Template` 102-118 → 114-119,
  `projectionPath` 114-140 → 128-149, `kind_violation` 557 → 553, the
  defensive arm 541-546 → 537-542, the fence test 254-285 → 262-294).
  NESTED-FILE-LOCUS → **open**; PREDICATE-SELECTION-ALGEBRA → re-gated onto
  the new entry below (it shares contract_template.rs / gate_fail_loud.rs
  with it, so the two cannot both be open).
  **Sweep — the window's residue is the build's own stated cut corner, and it
  is wider than the build could see.** 8b3b4e9 omitted its planned
  `gate_fail_loud.rs` proof and said why: `format-places-edges` ranges only
  over embedded members, and no dispatcher hands the engine one, so the test
  would have passed vacuously. Verified on disk, and the honest scope is
  larger than that one clause: `compileDeclarations`
  (`sdk/src/declarations.ts:681`) fills `kinds` from `atLocusKindsInPlay`
  while emitting a `ClauseRow` for **every** `expect` binding unfiltered, and
  `KindFactRow` carries `governs_root`/`governs_glob` unconditionally — so an
  author binds a clause to an embedded kind today, emit writes it to the
  lock, and neither dispatch loop (`src/main.rs:804`/`834`) ever judges it.
  `embedded_features_by_kind` (1324) reaches only `assemble_by_kind` (1287),
  the *graph* tier. That is a clause silently no-op'ing — invariant 6 — and
  the by-kind universal binding failing to reach members that
  representation.md ("nesting") calls full members. Filed as
  EMBEDDED-KIND-CONFORMANCE-DISPATCH, `per` contract.md "selection", **open**
  and first: it is what makes FORMAT-OMITS-EDGE-CLAUSE's shipped clause
  reachable in the real binary at all. No fork owed — the spec text is
  explicit; code lags it.
  Closing checklist: queue disjoint — the two `open` entries
  (EMBEDDED-KIND-CONFORMANCE-DISPATCH, NESTED-FILE-LOCUS) share no path,
  mechanically checked. NESTED-FILE-DISCOVERY shares `src/main.rs` with the
  new entry; its single `gate` tag names NESTED-FILE-LOCUS, so the second
  blocker is stated in its `notes` — the convention this board already runs
  for SUPPORTING-DOC-REACH-CLAUSE's two-blocker gap, unchanged this tick.
  Fork board: four open, none blocking a queued entry; two rider records
  deleted as discharged. This tick wrote no `src/`, `tests/`, or `sdk/` file.
- Queue: 2 pickable (EMBEDDED-KIND-CONFORMANCE-DISPATCH, NESTED-FILE-LOCUS —
  file-disjoint, so the wave may fan out); five serialized behind them as two
  chains; PACKAGING-CHANNELS-REMAINDER parked (Apple notarizing + v0.1 tag).

Plan continues: no — reconciliation was the last live input and it is done.
The inbox is empty, no refactor capture is live, and the spec delta is empty
(cursor abe5d5d is the last `specs:` commit). Both code cursors now sit at
9862b2e = HEAD. Build takes over with the two pickables.
