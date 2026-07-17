# Plan state

- Spec derived through: 6d2cca6
- Audited through: b85df4a
- Residue swept through: b85df4a
- This tick: ROUTE 0037's last three slices by surfacing the fork they all
  rest on, completing the decision's routing. [1] tap verb, [3] local-locus
  log kind, [4] explain field strand each need the log's storage FORMAT, and
  decision 0037 (read this tick, 6d2cca6) names none — "the tap **appends**
  event records to a local-locus log" (pipeline.md, Telemetry). temper's
  three formats are whole-file document reads; none is an append log, and
  minting a fourth is a deliberate-addition human call (the kept asymmetry),
  not plan's to invent. Keyed **`(tap-log-format)`** in open-questions with
  its three-reading tension (reuse json-document / mint JSONL / not a
  document kind); the three slices are its dependents and wait. A fork record
  IS routing → 0037 now fully routed (2 entries filed + 3 fork-parked), so
  the cursor advances to 6d2cca6. Consequences checklist in the commit body.
- Queue: 6 entries — EXTENT-PREDICATE **pickable** (gate:open); SETTINGS-LOCAL-KIND
  + VERIFIER-TYPED (both blockedBy EXTENT, mutually disjoint); TELEMETRY-HOOK-PROJECTION
  (blockedBy VERIFIER-TYPED — chain EXTENT→VERIFIER-TYPED→this, disjoint from
  SETTINGS-LOCAL); + 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER).
  DAG-disjoint. 0037's [1]/[3]/[4] fork-parked on `(tap-log-format)`, not queued.

Plan continues: yes — spec delta drained (0037 fully routed), but post-ship
reconcile of b85df4a..HEAD (3 build commits: 6667265, 631bc83, 8955a17) is now
the live input below.
