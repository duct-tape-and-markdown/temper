# Plan state

- **Phase:** reconcile. The 4-entry wave (REPRESENTATION-PRESERVE, BUNDLE-PLUGIN,
  UNKNOWN-KEY-REJECT, COVERAGE-HARDEN) verified shipped on disk (`src/bundle.rs`;
  the `KindUnknownKey`/`RoleUnknownKey`/`RequirementUnknownKey` rejects in
  `compose.rs`; dedup-before-dangling in coverage) — dropped. FILLED-BY-ROLE
  dropped outright: `filled_by` is absent from code and the spec (0784d00) retired
  it. The one live gap is the role→requirement code consolidation the spec
  reframe demands.
- **Last shipped:** REPRESENTATION-PRESERVE + BUNDLE-PLUGIN + UNKNOWN-KEY-REJECT +
  COVERAGE-HARDEN (`6969683`).
- **In flight:** none.
- **Pickable now (1):** **CONSOLIDATE-REQUIREMENT** — fold `Role` + `Requirement`
  into one `Requirement`; retire `[role.*]`; roster + coverage + graph.degree run
  over the one type. One atomic entry (the type change ripples through
  compose/roster/coverage/graph/main at once), sole `open`, no parallel wave.
- **Serialized / carried:** COVERAGE-CUSTOM-KIND deferred (shares coverage.rs —
  rebases onto the unified requirement after CONSOLIDATE lands). PACKAGING-CHANNELS
  parked (human release creds). AGENT-KIND deferred (built-in-kind reframe).
- **Inbox:** drained. MAJOR RECONCILE + CONSOLIDATION → CONSOLIDATE-REQUIREMENT;
  KILL FILLED-BY-ROLE → dropped; FOLD hardening entries → moot (both already
  shipped); PARALLEL-SAFE (REPRESENTATION-PRESERVE, BUNDLE-PLUGIN) → both shipped;
  DEFERRED → the three carried entries unchanged.
- **Blocked frontier (open questions, not fileable):** `(read-verbs)`,
  `(reference-id-normalization)`, `(decision-marker-predicate)` — all await a human
  decision.

Plan continues: no — queue reconciled, one atomic `open` entry pickable, inbox drained; hand to build.
