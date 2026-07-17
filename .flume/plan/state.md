# Plan state

- Spec derived through: 6d2cca6
- Audited through: 0349821
- Residue swept through: 0349821
- This tick: RECONCILE b85df4a..HEAD (3 build commits). Audit: the three
  shipped tags (6667265 MENTION-REACHABLE-EDGE-UNIFY, 631bc83 ADMISSION-
  JOINS-FILE-TEMPLATE, 8955a17 EMBEDDED-LEAF-BARE-ADDRESS) already absent
  from the queue — nothing to drop. Verified on disk: `mention_reachable`
  (graph.rs:348) folds `edges`+`mention_edges` into one set (364), no
  orphaned mention-only path; `templatesFor` joins admitted rows onto file
  layers; emit lifts a bare leaf on a singleton `to`. No residue. Re-tested
  the two parked gates: 6667265 touched graph.rs + tests/graph.rs, shifting
  IMPORT-HOP-CAP-CITE's later addresses (const 59 + doc 55-58 unmoved above
  the edit; live_members 621→630, BFS 648→657, cites 519→528 / 618-620→626-629
  / 643-646→652-655; test 1349→1407) — updated, park holds (nothing ruled hop
  semantics). PACKAGING park holds: window left .github/ untouched, crate
  0.1.0, no version tag. Both cursors advance to HEAD.
- Queue: 6 entries — EXTENT-PREDICATE **pickable** (gate:open); SETTINGS-LOCAL-KIND
  + VERIFIER-TYPED (both blockedBy EXTENT, mutually disjoint); TELEMETRY-HOOK-PROJECTION
  (blockedBy VERIFIER-TYPED — chain EXTENT→VERIFIER-TYPED→this, disjoint from
  SETTINGS-LOCAL); + 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER).
  DAG-disjoint. 0037's [1]/[3]/[4] fork-parked on `(tap-log-format)`, not queued.

Plan continues: no — inbox empty, spec delta drained (cursor 6d2cca6, no
newer specs/ commit), and b85df4a..HEAD reconciled (both cursors at HEAD).
EXTENT-PREDICATE is pickable; build takes over.
