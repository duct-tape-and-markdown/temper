# Plan state

- **Phase:** reconcile. HEAD 6ba251d.
- **Last shipped (spec):** the "adapter faces are declared" Decision (6ba251d,
  `specs/15-kinds.md`). Disk audit confirms the read/check path is already fully
  generic (`Unit::from_member_document` + embedded `KIND.md` extraction); the
  per-kind typed IRs `src/skill.rs`/`src/rule.rs` now survive **only** at the
  import/apply/re-add/drift/bundle adapter faces + read narration.
- **This tick:** drained the DECLARED-ADAPTER WAVE inbox line → filed
  **ADAPTER-EQUIVALENCE-PIN** (open, tests-only, pickable now — the byte-fidelity
  baseline the swap must not move) and **DECLARED-FRONTMATTER-ADAPTER** (parked:
  the generic declaration-driven adapter collapse retiring skill.rs/rule.rs).
  Rewrote **AGENT-KIND** to the near-zero-engine shape the new architecture makes
  it (no `src/agent.rs`; curated KIND.md + PACKAGE.md). Kept EXTRACTION-VOCAB-GAPS
  (deferred), PACKAGING-CHANNELS/COMMUNITY-DOCS (parked).
- **Pickable now:** ADAPTER-EQUIVALENCE-PIN (sole open entry, disjoint tests).
- **Human-gated (surfaced, not filed as build work):** add `format =
  "yaml-frontmatter"` + a unit-shape line to the curated `kinds/skill/KIND.md`
  and `kinds/rule/KIND.md` (build's fence excludes `kinds/`) — this un-parks
  DECLARED-FRONTMATTER-ADAPTER. Also still gated: the `(edge-representation-unify)`
  fork, release creds (PACKAGING-CHANNELS), the chain.ts fence-widen + private
  vuln reporting (COMMUNITY-DOCS).

Plan continues: no — queue reconciled, inbox drained, one open entry filed
(ADAPTER-EQUIVALENCE-PIN); the substantive wave is human-gated on the curated
KIND.md declarations. Hand to build.
