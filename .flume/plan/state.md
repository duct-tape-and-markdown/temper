# Plan state

- **Phase:** reconcile. Verified on disk: wave 2 (DEPENDENCY-EXISTS-FENCE,
  SHA256-HOIST, MATCHSELECTOR-ROLE-DROP) shipped at `d78cf65` — the `role:` marker
  is gone from the fill vocabulary, the four sha256 defs are hoisted into `hash.rs`,
  and `dependency-exists` is fenced at admissibility (engine.rs). One new gap filed:
  the spec names a gitignored `temper-local.toml` (`40-composition.md`) but `main.rs`
  loads only `temper.toml`. Everything else is either shipped or fork-blocked. Inbox empty.
- **Last shipped:** DEPENDENCY-EXISTS-FENCE / SHA256-HOIST / MATCHSELECTOR-ROLE-DROP
  (`d78cf65`).
- **In flight:** none.
- **Pickable now (1):** TEMPER-LOCAL-LAYER — `main.rs` + `compose.rs` +
  `tests/temper_toml.rs`. No other `open` entry, so no parallel-conflict risk.
- **Carried:** COVERAGE-CUSTOM-KIND deferred (priority; kind.rs:451-454 gap confirmed),
  PACKAGING-CHANNELS parked (human release creds), AGENT-KIND deferred (reframe).
- **Blocked frontier (open questions, unchanged):** `(reference-id-normalization)` +
  `(decision-marker-predicate)` gate the spec-kind references-resolve / decisions-name
  clauses; `(read-verbs)` gates `why`/`requirements` CLI verbs — all await a human decision.

Plan continues: no — queue reconciled, one disjoint `open` entry pickable, inbox drained; hand to build.
