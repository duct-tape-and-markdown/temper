# Plan state

- **Phase:** reconcile. The whole requirements/satisfies/coverage chain is drained
  and verified on disk (`coverage.rs` present + wired at `main.rs:442`, both
  directions). The core engine is essentially spec-complete: contracts + admissibility,
  the kind system (custom-kind declaration, extraction algebra, `src/spec.rs` retired
  to `temper.toml` data), governance (count/membership/typed-ref/unique/role, degree/
  acyclic/declared-edges, range), requirements coverage, drift (diff/apply/re-add),
  and distribution's shipped half (schema validation channel, GitHub/SARIF/session-start
  reporters, the session-start hook).
- **Last shipped:** REQUIREMENT-COVERAGE (`0fdf80d`).
- **In flight:** none.
- **Pickable now (2, parallel-safe):** **INSTALL-GATE-WIRING** (`open`, install/main/lib)
  and **SCHEMA-DOCS-CHANNEL** (`open`, contract/compose/schema/contracts) — disjoint file
  sets. **BUNDLE-PLUGIN** is `blockedBy` INSTALL (shares main.rs+lib.rs). **PACKAGING-CHANNELS**
  parked (needs human release creds). **AGENT-KIND** deferred.
- **Blocked frontier (open questions, not fileable):** custom-kind graph wiring
  (`reference-id-normalization`) and the spec contract's decision-marker clause
  (`decision-marker-predicate`) — both await a human language/resolution decision.
- **Inbox:** empty. Open questions unchanged (no fork resolved this tick).

Plan continues: no — queue reconciled, two disjoint `open` entries pickable, inbox empty; hand to build.
