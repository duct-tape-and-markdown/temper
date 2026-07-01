# Plan state

- **Phase:** reconcile. INSTALL-GATE-WIRING and SCHEMA-DOCS-CHANNEL both verified
  shipped on disk (`src/install.rs` + `Install` arm at `main.rs:328`; `src/schema.rs`
  two-channel emit with `Clause::guidance`) — removed from the queue. The engine is
  spec-complete through distribution's shipped half; the frontier is the plugin verb,
  three inbox findings, and the two spec Decisions ratified in e72c23c.
- **Last shipped:** INSTALL-GATE-WIRING + SCHEMA-DOCS-CHANNEL (`fbec06f`).
- **In flight:** none.
- **Pickable now (4, parallel-safe, disjoint file sets):** **REPRESENTATION-PRESERVE**
  (import/skill/rule/readd — inbox data-loss must-fix, still live on disk),
  **BUNDLE-PLUGIN** (bundle.rs/lib/main — unblocked, sole main.rs editor),
  **UNKNOWN-KEY-REJECT** (compose/contract — spec e72c23c), **COVERAGE-HARDEN**
  (coverage — dangling-loop dedup + doc + fixtures still needed). No two share a file.
- **Serialized:** **FILLED-BY-ROLE** blockedBy UNKNOWN-KEY-REJECT (shares compose.rs;
  also touches main.rs↔BUNDLE and coverage.rs↔COVERAGE-HARDEN, so must not be open
  during the wave; revive next tick once the wave lands). **COVERAGE-CUSTOM-KIND**
  deferred (kind::Unit satisfies follow-on). **PACKAGING-CHANNELS** parked (human
  release creds). **AGENT-KIND** deferred.
- **Inbox:** drained. MUST-FIX→REPRESENTATION-PRESERVE; HARDEN→COVERAGE-HARDEN;
  DECISION(i)→FILLED-BY-ROLE; DECISION(iii)→UNKNOWN-KEY-REJECT; DECISION(ii)
  (kind-blindness) already satisfied in coverage.rs — accepted, no work; DEFER(1)→
  COVERAGE-CUSTOM-KIND (deferred); DEFER(2) read verbs→`(read-verbs)` open question;
  DOGFOOD (root `[requirement.*]`) is human territory — sequence after
  REPRESENTATION-PRESERVE lands.
- **Blocked frontier (open questions, not fileable):** `(read-verbs)` (CLI-surface
  gap), `(reference-id-normalization)`, `(decision-marker-predicate)` — all await a
  human decision.

Plan continues: no — queue reconciled, four disjoint `open` entries pickable, inbox drained; hand to build.
