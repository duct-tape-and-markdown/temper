# Plan state

- **Phase:** the in-crate decidable artifact algebra ships for **skill** + **rule**
  and self-host *conformance* is green. The corpus **grew since the last plan tick**
  (b2dbf16): two new spec files (40-composition, 50-distribution) and Decisions that
  RESOLVED four forks — the prior `state.md` (written pre-b2dbf16) wrongly called the
  fork-free queue exhausted.
- **Last shipped:** REQUIRE-SECTIONS (827f5c3). Since then the human authored new
  intent (b2dbf16) and made `plugin/**` a build-writable surface (d422925, still
  empty). Verified on disk: `check` runs **conformance only** — no admissibility pass
  (`main.rs`); `extract.rs` still stringifies every scalar (`json_scalar_string`).
- **In flight:** nothing; tree clean, pending was empty.
- **Next (filed, all fork-free):** ADMISSIBILITY (the second green `check` is missing
  — finish line), then the `type` primitive pair TYPED-EXTRACTION → TYPE-PRIMITIVE
  (`(field-type-lattice)` now RESOLVED).
- **Frontier (now fork-free, unfiled — for the next plan ticks to decompose):** the
  harness-contract layer — `temper.toml` + roles + `verified_by` (40-composition,
  `(harness-contract-provisioning)` RESOLVED both halves); `temper schema`; the
  advisory session-start gate + `claude-session-start` reporter; the plugin/`bundle`
  tree; GitHub/SARIF reporters. **Still fork-blocked:** `apply`/`install`
  (`(yaml-writeback)`, `(workspace-scope)`); the declared model + dependency graph +
  `dependency-exists` + cross-landscape seam (`(model-declaration-format)`); full
  `pattern` (`(regex-crate)`); the skill referential clause (`(skill-ref-syntax)`).

Plan continues: no — the queue is reconciled against the grown corpus and three
fork-free entries are filed; build runs. The harness-contract / schema / distribution
frontier is fork-free but deferred to follow-on plan ticks so each is decomposed into
small disjoint entries once ADMISSIBILITY and the type pair land — filing that whole
chain now would be speculative over-filing.
