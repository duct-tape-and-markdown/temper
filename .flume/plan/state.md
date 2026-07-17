# Plan state

- Spec derived through: a571973
- Audited through: 01337ff
- Residue swept through: 01337ff
- This tick: RECONCILE eb2776d..01337ff — one build commit (83fa520,
  MENTION-REACHABLE-PREDICATE), audited and swept in one pass; the window is
  small enough that the split would have been theatre. **Audit: the predicate
  shipped real, verified on disk, not from the log.** `Predicate::
  MentionReachable { scope_field, gate_field }` lives in `src/contract.rs:277`,
  admitted at `contract.rs:391`, vacuity-fenced in `engine.rs:292-301`, judged
  in `graph.rs:320-410` beside `degree`, with `mentionReachable()` exported at
  `sdk/src/contract.ts:137` and the `gate` column spelled across
  `drift.rs:2463`, the ts-rs binding, and `declarations.ts:85`. **Both blockedBy
  gates open** — MENTION-REACHABLE-RULE-CLAUSE and EDGE-TARGET-SET, disjoint in
  files, so both are pickable side by side. **The audit's one substantive
  finding: what shipped is not what MENTION-REACHABLE-RULE-CLAUSE's summary
  said.** 83fa520 recorded that 0028's literal "target registers by
  `paths-match`" reading selects rules and never skills on this tree, and
  shipped the field-argument reading instead — the trigger is the target's gate
  FIELD carrying globs, hard-coding no kind. The entry said "over `paths` to a
  skill's `paths`"; the adoption is `mentionReachable("paths", "paths")` and
  fires on any mentioned member whose `paths` gates, skill and rule alike. Entry
  re-worded to the shipped reading rather than left encoding the ruling's
  superseded literal one — a summary a build phase would have had to silently
  reinterpret is a gap, not a detail. **Sweep: no new residue.** The window
  named no retirement, and the build's own cut-corner disclosure ("No default
  contract adopts it yet") is the queued RULE-CLAUSE, not a gap. `schema.rs`'s
  silent keyword channel is reasoned, not omitted. Verified the new judge reads
  `ResolvedEdge.to` (a `(kind, id)` tuple), never `compose::Edge.to` — so it
  adds no reader EDGE-TARGET-SET must move, and that entry's file list stands
  unchanged. **Line-cite reconciliation was the tick's real work:** 83fa520
  moved `graph.rs` +142, `tests/graph.rs` +166, `drift.rs` +11, `main.rs` +10,
  `compose.rs` +3, and both SDK sites +1/+2, so every cite in EDGE-TARGET-SET
  and IMPORT-HOP-CAP-CITE was re-derived by reading the regions, not by
  arithmetic. Spec cursor copied forward verbatim: the delta is dry and this
  tick derived no spec.
- Queue: 4 entries, every gate re-tested at this HEAD — 2 pickable
  (MENTION-REACHABLE-RULE-CLAUSE, EDGE-TARGET-SET), disjoint from each other in
  files, so they run beside each other; 2 parked on human acts
  (IMPORT-HOP-CAP-CITE: a hop-depth probe — park holds, nothing ruled the
  semantics, cites re-derived at graph.rs:65. PACKAGING-CHANNELS-REMAINDER:
  Apple notarizing + the v0.1 tag — no release act in the window, park unmoved).
  The queue's one file overlap is EDGE-TARGET-SET × IMPORT-HOP-CAP-CITE on
  `src/graph.rs`, inert while the latter is parked: EDGE-TARGET-SET landing will
  move the hop cites a third time, and re-deriving them is that reconcile's job.

Plan continues: no — the inbox is empty, `.flume/refactor/` holds its README
alone, the spec delta is dry (`git log a571973..HEAD -- specs/` is empty), and
this tick reconciled the window to HEAD, so no input below it is live. Build
takes over: two entries are pickable and disjoint. The one friction capture on
disk (`build-import-hop-cap-cite-disagrees-with-live-docs.md`) is already fully
routed as IMPORT-HOP-CAP-CITE's park reason and is the human's to read.
