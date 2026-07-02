# Plan state

- **Phase:** reconcile. HEAD cb4c1a0.
- **Last shipped (trunk):** HEADER-FIELD-EXTRACTION — a custom member's clause
  fields now lift into extracted frontmatter (build 0f49c7d). Wave head is drained.
- **This tick:** reconciled the source-not-mechanism extraction wave against
  `specs/15-kinds.md`. Verified on disk: `skill_features`/`rule_features` still live
  in `src/extract.rs` and are called at `main.rs:405-408` + `541-545`; no
  `tests/extract_equivalence.rs`, no `src/builtin_kind.rs`, no `kinds/` walk in
  `build.rs` — the three downstream entries are unshipped and their cites accurate.
  Sole change: EXTRACT-EQUIVALENCE-PIN's stale `blockedBy HEADER-FIELD-EXTRACTION`
  (shipped, out of queue) flipped to `open`. Inbox empty; no forks moved.
- **Pickable now:** EXTRACT-EQUIVALENCE-PIN (open). EMBED-BUILTIN-KINDS →
  BUILTIN-EXTRACT-GENERIC serialize behind it (each `blockedBy` the prior, no two
  open entries share a file). AGENT-KIND deferred; PACKAGING-CHANNELS /
  COMMUNITY-DOCS parked. Sole live OPEN fork: (edge-representation-unify).

Plan continues: no — EXTRACT-EQUIVALENCE-PIN is pickable; hand to build. The wave
serializes one entry at a time; re-planning would only re-emit the same queue.
