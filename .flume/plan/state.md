# Plan state

- **Phase:** reconcile. CONSOLIDATE-REQUIREMENT verified shipped on disk (one
  `Requirement`; `[role.*]` retired and rejected loudly in `compose.rs`;
  `MatchSelector::Role` survives as leftover role-marker matching). Inbox drained:
  the COMPLEXITY-AUDIT remediation batch filed as 7 entries (4 open + 3 serialized).
- **Last shipped:** CONSOLIDATE-REQUIREMENT (`19333aa`).
- **In flight:** none.
- **Pickable now (4, disjoint files):** CHECK-CLEANUP (`check.rs`),
  KIND-ENTITIES-RECONCILE (`compose.rs`), BODY-HASH-DROP (`import.rs`/`drift.rs`),
  FEATURES-COMPANIONS-DROP (`extract`/`engine`/`roster`/`coverage`/`graph`/`kind`).
- **Serialized wave 2 (blockedBy, mutually disjoint):** DEPENDENCY-EXISTS-FENCE
  ← FEATURES (engine.rs); SHA256-HOIST ← BODY-HASH-DROP (import/drift);
  MATCHSELECTOR-ROLE-DROP ← KIND-ENTITIES (compose.rs; also shares roster.rs with
  FEATURES, held out of that wave). Carried: COVERAGE-CUSTOM-KIND deferred,
  PACKAGING-CHANNELS parked, AGENT-KIND deferred.
- **Blocked frontier (open questions, unchanged):** `(read-verbs)`,
  `(reference-id-normalization)`, `(decision-marker-predicate)` — await a human decision.

Plan continues: no — queue reconciled, 4 disjoint `open` entries pickable, inbox drained; hand to build.
