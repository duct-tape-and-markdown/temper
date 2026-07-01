# Plan state

- **Phase:** reconcile. REQUIREMENTS-PARSE shipped, so its dependent unblocks; the
  three-entry chain (REQUIREMENTS-PARSE → SATISFIES-REPRESENTATION →
  REQUIREMENT-COVERAGE) advances one link. Verified on disk: `compose.rs` carries
  `Requirement`/`requirements()`/`parse_requirement`; `coverage.rs` absent; no
  `satisfies`/`rationale`/`representation` in skill/rule/extract yet.
- **Last shipped:** REQUIREMENTS-PARSE (`ee3d561`); queue reconciled since.
- **In flight:** none.
- **Pickable now (1):** **SATISFIES-REPRESENTATION** (`open`) — carry
  `satisfies`+`rationale` under a `[representation]` meta.toml table on the
  skill/rule IR + `Features.satisfies`. REQUIREMENT-COVERAGE stays `blockedBy` it.
- **Deferred:** **AGENT-KIND** — deprioritized by the reframe; revive on demand.
- **Inbox:** empty. Open questions unchanged (no fork resolved this tick).

Plan continues: no — one gate flipped to `open`, one pickable entry, inbox empty; hand to build.
