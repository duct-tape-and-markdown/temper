# Plan state

- **Phase:** reconcile. SATISFIES-REPRESENTATION shipped, so the last link of the
  requirements chain unblocks. Verified on disk: `satisfies`/`rationale`/
  `[representation]` live on the skill+rule IR and `extract.rs` (`Features.satisfies`);
  `compose.rs` carries `Requirement { means, required }` + `requirements()`;
  `coverage.rs` still absent (REQUIREMENT-COVERAGE unshipped).
- **Last shipped:** SATISFIES-REPRESENTATION (`79b0a44`); queue reconciled since.
- **In flight:** none.
- **Pickable now (1):** **REQUIREMENT-COVERAGE** (`open`) — `check` gates referential
  coverage (every `required` requirement resolves to a `satisfies`; every `satisfies`
  resolves to a requirement). Sole open entry; the only `[requirement.*]` fixtures are
  parse-only unit tests, so its blast radius (coverage/lib/main + tests) is disjoint.
- **Deferred:** **AGENT-KIND** — deprioritized by the reframe; revive on demand.
- **Inbox:** empty. Open questions unchanged (no fork resolved this tick).

Plan continues: no — one gate flipped to `open`, one pickable entry, inbox empty; hand to build.
