# Plan state

- **Phase:** reconcile + inbox drain. HEAD 83f74f3.
- **Last shipped (trunk):** MEMBER-PUBLISHED-REQUIREMENTS (9fe2740 build,
  83f74f3 flume) — a member header's `[requirement.*]` now unions into the one
  requirement namespace coverage judges. Its blocker chain (REDD-CUSTOM-KINDS,
  REFERENCES-RETIRE) is all on trunk.
- **This tick:** audited engine vs corpus on disk — the predicate algebra,
  governance set/graph predicates (count/membership/unique/degree/acyclic),
  `range`, `apply`/`re-add`/`schema`/`bundle`/`install`/reporters, `verified_by`
  resolution, and member-published requirements are all shipped; no TODOs; the
  three shipped downstreams (CONTRACTS-RETIRE — `contracts/` gone; READ-EDGE-UNIFY;
  MEMBER-PUBLISHED-REQUIREMENTS) are truly on disk, not just in the log. Filed one
  new `open` gap — READ-CUSTOM-SATISFIERS: the read family's `satisfies`/rationale
  walk covers skill⊕rule only (src/read.rs:28), silently under-reporting a
  custom-kind member that fills a requirement. AGENT-KIND (deferred), PACKAGING-CHANNELS
  (parked) re-verified accurate. Inbox empty; open-questions unchanged
  ((edge-representation-unify) the one live OPEN fork, no dependent).
- **Pickable now (1 `open`):** READ-CUSTOM-SATISFIERS — sole open entry,
  parallel-safe. Deferred: AGENT-KIND. Parked: PACKAGING-CHANNELS.

Plan continues: no — queue reconciled, inbox empty, one `open` entry pickable;
building drains it.
