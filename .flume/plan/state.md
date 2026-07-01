# Plan state

- **Phase:** reconcile. Wave 1 (CHECK-CLEANUP, KIND-ENTITIES-RECONCILE,
  BODY-HASH-DROP, FEATURES-COMPANIONS-DROP) verified shipped on disk (e65e872) —
  their four target seams are gone/reconciled. Wave 2's blockers are all satisfied,
  so its three entries flip `blockedBy` → `open`. Inbox empty; no new corpus↔src gap
  to file (the KIND-* frontier stays human-gated on open questions).
- **Last shipped:** CHECK-CLEANUP / KIND-ENTITIES-RECONCILE / BODY-HASH-DROP /
  FEATURES-COMPANIONS-DROP (`e65e872`).
- **In flight:** none.
- **Pickable now (3, mutually disjoint files):** DEPENDENCY-EXISTS-FENCE
  (`contract.rs`/`engine.rs`), SHA256-HOIST (new `hash.rs` + `lib.rs`/`skill.rs`/
  `rule.rs`/`import.rs`/`drift.rs`), MATCHSELECTOR-ROLE-DROP (`compose.rs`/`roster.rs`/
  `tests/requirement_roster.rs`). No path overlap — parallel-safe.
- **Carried:** COVERAGE-CUSTOM-KIND deferred (priority), PACKAGING-CHANNELS parked
  (human release creds), AGENT-KIND deferred (reframe).
- **Blocked frontier (open questions, unchanged):** `(read-verbs)`,
  `(reference-id-normalization)`, `(decision-marker-predicate)` — await a human decision.

Plan continues: no — queue reconciled, 3 disjoint `open` entries pickable, inbox drained; hand to build.
