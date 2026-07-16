# Plan state

- Spec derived through: abe5d5d
- Audited through: b745415
- Residue swept through: b745415
- This tick: RECONCILED the 9862b2e..b745415 window (5c34ced, 91c288c —
  EMBEDDED-KIND-CONFORMANCE-DISPATCH and NESTED-FILE-LOCUS). Both motions
  ran; both cursors advance.
  **Audit — the two stale gates re-tested and OPENED.** Verified on disk, not
  from the log: the embedded-kind conformance dispatch ships (the third loop
  in `main.rs`; proofs at `gate_fail_loud.rs` 276/310), and
  `CustomKind::nested_file` (kind.rs:600) + the third `member_projection_path`
  branch ship. So NESTED-FILE-DISCOVERY and PREDICATE-SELECTION-ALGEBRA go
  `open` — file-disjoint from each other and from PLACEMENT-KEY-NUL-DELIMITER,
  mechanically checked (the only shared files sit among the three `blockedBy`
  entries, which never run in parallel).
  **The audit found a false file claim and dropped it.** NESTED-FILE-DISCOVERY
  claimed `src/drift.rs` "re-runs the same `governs`-keyed scan" — **it does
  not**: no discovery caller exists in that file; drift diffs the payload's
  owned set against the prior lock's rows. Rescoped to where the hole actually
  is — 91c288c turned it into two *explicit commented arms*
  (`import.rs:95-103`'s `None => Ok(Vec::new())`, `main.rs:1139-1143`'s
  `(_, _, None) => Vec::new()`) — and added the two surfaces the old scope
  missed: `install.rs:316` (the discovery report the `per` section opens with,
  so the entry's adopted-harness claim is true for the verb that adopts) and
  `kind.rs:578-580` (the declared-fact-only sentence it retires).
  **Sweep — one rider discharged, one filed, no standalone entry.** 91c288c
  discharged the `drift.rs:570` "retired `projectionPath`" rider by rewriting
  the doc (675-677); the record is **deleted** from open-questions. New this
  window: `import.rs` 91 and 147-149 both credit drift with re-running the
  scan — false at HEAD. Comment staleness, so it rides NESTED-FILE-DISCOVERY,
  named in that entry's own `files[].description` — the only shape that
  discharges one, now proven rather than proposed.
  Closing checklist: `per` cites and every symbol/line claim re-verified on
  disk; field lengths validated (max notes 494/500). No fork owed — no
  ambiguity surfaced; four open forks, none blocking a queued entry. Spec
  cursor copied forward verbatim (delta empty). Park re-tested: no v0.1 tag,
  crate 0.1.0 vs npm 0.0.7, release.yml:7-9 still defers darwin.
- Queue: 3 pickable and mutually file-disjoint (PLACEMENT-KEY-NUL-DELIMITER,
  NESTED-FILE-DISCOVERY, PREDICATE-SELECTION-ALGEBRA); three gated in two
  chains behind them; PACKAGING-CHANNELS-REMAINDER parked (Apple notarizing
  + v0.1 tag).

Plan continues: no — every input is drained. Inbox empty, no refactor
captures, spec delta empty (cursor at abe5d5d, no `specs/` commit past it),
and both code cursors now sit at b745415, the last commit touching `src/`,
`tests/`, or `sdk/`. Build takes over: three disjoint entries are pickable.
