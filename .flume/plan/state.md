# Plan state

- Spec derived through: abe5d5d
- Audited through: b745415
- Residue swept through: b745415
- This tick: DRAINED the inbox — four 07-16 notes (filed 7c1a50a, 3fdebb8).
  Two entries, one fork, one already-routed duplicate; the file is back to its
  header. Every note diffed forward from its `observed at 8b43293` stamp over
  the three code commits since (5c3a618, 2722ca2, 28ad61f): all premises
  unmoved, both mechanisms **narrower than reported** and scoped to the
  verified gap, per job 1.
  **The embedded-edge regression is one list reused for two jobs.** Not
  "0025's degree semantics hasn't landed": `declarations.ts:696` computes
  `unitKindsInPlay(kindsInPlay(harness))` once and spends it on BOTH the kind
  fact rows (705 — correct, an embedded kind takes no row) and
  `assemblyFactRows` (708 — wrong), so an embedded kind's `edgeFields` write
  no assembly `edge` fact and the edge never exists to resolve. Reader half:
  `graph.rs:792-820` matches bare ids while an embedded leaf authors a full
  `kind:name` address (`kind.ts:370`). Filed EMBEDDED-EDGE-DEGREE-SEAM.
  **The silent clause is vacuity, not a missing judge** — the report's
  diagnosis is wrong and the entry says so. 5c34ced's dispatch DOES reach
  composed nested members (`main.rs:898`); the clause is judged against a body
  `embedded_member_features` zeroed at the lift (`main.rs:1611-1613`), so
  `max_lines` reads `0 <= 2` and passes. That is contract.md's "no vacuous
  clause", so the fence is admissibility-side: EMBEDDED-CLAUSE-BODY-VACUITY-FENCE.
  **Both entries indict 5c34ced's fence test as the reason neither was caught:**
  `gate_fail_loud.rs:279` HAND-WRITES the embedded edge fact, pinning the engine
  and never the emit seam — so `format-places-edges` is itself unreachable
  through real emit today, and the first entry restores it.
  **The export note is a duplicate, drained not re-filed:** SDK-ROOT-EXPORT-CLOSURE
  (5b6d17b) already names both `ResolvedEmbeddedMemberValue` and
  `EdgeTargetFacts` in its own `files[].description`. Nothing re-scoped.
  **One fork registered:** `(edge-field-target-openness)` — `EdgeField.to` is
  required (`kind.ts:43-47`) yet emit resolves address-based and `to`-blind
  (`emit.ts:248-274`) while corpus and engine both say "identity within the
  target kind". `to`'s arity is a model-file change, so it is ruled, never
  derived; plan's position (widen `to` to a declared set) rejects both
  spellings the report offered and carries the objection that must answer it.
  Closing checklist: `per` cites verified in-file (contract.md "edge",
  "well-formedness"); every symbol/line claim re-verified on disk; field
  lengths validated. Queue disjoint — EMBEDDED-EDGE-DEGREE-SEAM serializes
  behind SDK-ROOT-EXPORT-CLOSURE (shares `sdk/test/emit.test.ts`, the one home
  where `declarations.assembly` is asserted); the fence entry is file-disjoint
  from all six and ships `open`. No built-in declares `edgeFields`, so
  `builtin_lock.toml` does not move and neither entry collides with the
  built-in chain. Cursors copied forward verbatim — this tick derived no spec
  and audited no window.
- Queue: 2 pickable (SDK-ROOT-EXPORT-CLOSURE, EMBEDDED-CLAUSE-BODY-VACUITY-FENCE);
  four gated. **Two gates are stale and untested by design** — this tick was
  the inbox's; SKILL-NESTED-REFERENCE-DOCS and SUPPORTING-DOC-REACH-CLAUSE name
  NESTED-FILE-DISCOVERY / PREDICATE-SELECTION-ALGEBRA, both shipped at 420da04.
  The refs gate does not read `blockedBy`, so nothing reverts and no build wave
  can pick a stale-gated entry. PACKAGING-CHANNELS-REMAINDER parked (Apple
  notarizing + v0.1 tag).

Plan continues: yes — post-ship reconciliation of the b745415..420da04 window
(5c3a618, 2722ca2, 28ad61f: PLACEMENT-KEY-NUL-DELIMITER, NESTED-FILE-DISCOVERY,
PREDICATE-SELECTION-ALGEBRA), both code cursors trailing it. The audit owes the
two stale gates above a re-test and the work behind them a derivation.
