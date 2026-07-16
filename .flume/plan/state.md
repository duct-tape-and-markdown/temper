# Plan state

- Spec derived through: abe5d5d
- Audited through: 420da04
- Residue swept through: 420da04
- This tick: RECONCILED the b745415..420da04 window (5c3a618, 2722ca2,
  28ad61f), both motions, on disk — never the log. Nothing dropped: the three
  shipped entries left pending at 420da04 already. **Both stale gates re-tested
  and both moved**, the audit's whole point: SKILL-NESTED-REFERENCE-DOCS is
  **OPEN** — NESTED-FILE-DISCOVERY shipped at 2722ca2, verified at
  `discover_builtin` (dispatches on the absent `governs`, walks each child
  under its host's unit, one scanner still) — and SUPPORTING-DOC-REACH-CLAUSE
  **re-points** to it: PREDICATE-SELECTION-ALGEBRA shipped at 28ad61f
  (`graph::degree` takes `&[Selection]`, so a by-kind degree clause is judged
  and admissible outside a requirement — the exact fence that held it), leaving
  the built-in chain as its one real blocker. Queue is 3 pickable, up from 2.
  **The sweep found one gap and it is not the class it looks like.** 28ad61f
  carried a rider to re-verify graph.rs's two memory cites on the standing
  premise "every fact still holds" — and that premise is FALSE for one:
  `MAX_IMPORT_HOPS = 5` cites a page that reads *"a maximum depth of four
  hops"* (re-fetched live this tick, wording unchanged). Build bumped the
  verified cite (689 → 677) and declined this one rather than flip a live gate
  inside an unrelated entry — the right call, and its friction capture is the
  reason the datum survived. Filed **IMPORT-HOP-CAP-CITE, parked**: not
  citation staleness, so it does not ride — staleness rides *because the fact
  holds*, and here the constant itself may be wrong. Parked, not open, because
  the doc sentence does not decide it: temper's indexing is decidable on disk
  (`live_members` counts one import edge per BFS round from a live seed), but
  whether the doc's "four hops" counts those same edges or a recursion atop the
  first import is not, and both readings survive the sentence. Only a runtime
  probe rules it; guessing forges or suppresses a dead-member finding
  (invariant 2). Plan does not guess an external fact.
  **Rider bookkeeping, three records reconciled:** contract.rs:490 discharged
  (28ad61f rewrote the doc, now `documented_field` at 494) — the *named*
  carrier worked where two unnamed ones failed, so the "name the rider in the
  entry" shape is twice proven and the record now says so; graph.rs left the
  rides-along class entirely (see above), leaving builtin_kind.rs as its one
  surviving surface; compose.rs:233 lives with **no carrier** — no queued entry
  opens the file. One new rider named at its carrier rather than filed here:
  28ad61f retired the `Facet` type and left its vocabulary narrating engine.rs
  at 157/534/1096-97, so EMBEDDED-CLAUSE-BODY-VACUITY-FENCE — which rewrites
  `vacuities`'s signature and that very doc — names it in its own description.
  Closing checklist: queue disjoint over the three open entries
  (SDK-ROOT-EXPORT-CLOSURE sdk-only, EMBEDDED-CLAUSE-BODY-VACUITY-FENCE
  engine/main/gate_fail_loud, SKILL-NESTED-REFERENCE-DOCS builtins/lock — no
  shared path); IMPORT-HOP-CAP-CITE shares graph.rs with
  EMBEDDED-EDGE-DEGREE-SEAM but is parked, so no wave can pick both; every gate
  reason re-tested against disk, not restated; `kind.rs:600` corrected to 602
  in SKILL-NESTED-REFERENCE-DOCS; field lengths validated. Spec cursor copied
  forward verbatim — this tick derived no spec.
- Queue: 3 pickable (SDK-ROOT-EXPORT-CLOSURE, EMBEDDED-CLAUSE-BODY-VACUITY-FENCE,
  SKILL-NESTED-REFERENCE-DOCS); 3 gated behind them (EMBEDDED-EDGE-DEGREE-SEAM,
  SDK-FIXTURE-WIRING-ONE-HOME, SUPPORTING-DOC-REACH-CLAUSE — the built-in tail);
  2 parked on human acts (IMPORT-HOP-CAP-CITE: a hop-depth probe.
  PACKAGING-CHANNELS-REMAINDER: Apple notarizing + the v0.1 tag). No gate is
  stale — every one was tested this tick.

Plan continues: no — the window is reconciled on both motions, the inbox is at
its header, and the spec delta is empty. Build takes over on three pickable
entries. The two parked entries need John, and IMPORT-HOP-CAP-CITE is the one
worth surfacing: a live gate constant whose own cited source contradicts it.
