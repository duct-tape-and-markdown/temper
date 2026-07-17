# Plan state

- Spec derived through: 5f89860
- Audited through: b85df4a
- Residue swept through: b85df4a
- This tick: DERIVE spec delta 5f89860 (decision 0035, "extent joins the
  vocabulary") — a human `specs:` commit that landed as last tick's parent
  while the inbox drain was in flight, ruling the value-extent demand the prior
  tick had just parked as a fork. Routed the decision's Consequences:
  - Bullet 1 ("Plan derives the entry: the predicate in engine and SDK, each-
    and whole-grain over rendered extent, units lines/characters, admissibility
    for the unit; the inbox demand note discharges into it") → **EXTENT-PREDICATE**
    (per contract.md, "clause"). Adds `Predicate::Extent` (contract.rs), both-grain
    eval render-side (engine.rs), the `extent` constructor (contract.ts), clause
    lowering + a `unit` column (declarations.ts, drift.rs `ClauseRow`, generated
    mirror). Inbox note already discharged (drained empty last tick).
  - Bullet 2 ("No shipped default contract adopts extent — the shipped defaults
    stay opinion-free") → verified-as-constraint: the entry adopts extent in no
    default contract; `sdk/src/builtins.ts` and the built-in lock stay untouched.
  **Correction carried in the same tick:** the prior commit (f552ccd) had
  registered `(value-extent-predicate)` as an OPEN fork — 0035 resolves it, so
  a linter had already reverted the record from the working tree; this commit
  lands that removal (resolution = encode + delete).
  **Collision surfaced, not papered over:** 0035 rules render-side and *rejects*
  source-side, but the shipped `max_lines` (source-side body lines, adopted in
  the skill/rule/memory defaults) is exactly that rejected measurement, and 0035
  never names its retirement. Registered `(extent-subsumes-max-lines)` — a
  language change no derived layer invents. EXTENT sits BESIDE `max_lines`,
  serialized behind ADMISSION (shared `declarations.ts`).
- Queue: 6 entries, **3 pickable** (the three field defects, gate:open, disjoint)
  + EXTENT-PREDICATE (blockedBy ADMISSION) + 2 parked. No entry rests on a fork.

Plan continues: no — spec delta routed (cursor at 5f89860, its sole commit),
inbox empty, and b85df4a..f552ccd is spec/.flume-only (nothing to reconcile).
Three pickable defects hand off to build; EXTENT unblocks when ADMISSION ships.
