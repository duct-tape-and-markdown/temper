# Plan state

- **Phase:** reconcile. HEAD 923d0d9.
- **Last shipped:** the four-entry wave MEMORY-KIND / WEDGE-INSTALL-SUMMARY /
  WEDGE-COVERAGE-NOTE / SKILL-VERSION-EXTRACTION-DROP (chore 923d0d9) — memory's
  File-shaped round-trip is pinned, install self-verify is one summary advisory,
  the wedge emits a coverage note so silence ≠ "checked", and the `version`
  example field is re-pinned onto `license`. All four drained from pending.
- **This tick:** drained the inbox DIRECTIVES wave into three dependency-ordered
  entries and filed them (per specs/architecture/15-kinds.md "Directives —
  format-executed body syntax" + 45-governance.md "The world is a node"):
  DIRECTIVES-PRIMITIVE-PARSE (the `directives`/`at-import` extraction primitive,
  `open`), DIRECTIVE-TARGET-CLASSING (member/backed/unbacked target classing,
  `blockedBy` slice 1), REACHABILITY-DIRECTIVE-CLOSURE (reachability closes over
  directive edges, `blockedBy` slice 2). Verified on disk: `Primitive` still lacks
  `directives`; the memory KIND.md prose already carries the `@path` grammar +
  citations (adoption of the `[[extraction]]` line is the human's, post-slice-1).
  Reconciled the four carried entries — none shipped, all still truthful:
  EXTRACTION-VOCAB-GAPS + AGENT-KIND stay `deferred` (no consumer / wrong
  direction), PACKAGING-CHANNELS + COMMUNITY-DOCS stay `parked` (human creds /
  fence-widen). Refreshed EXTRACTION-VOCAB-GAPS's note to cite the DIRECTIVES
  collision (both touch src/kind.rs+extract.rs).
- **Operational note (accepted, not queued):** the session-start
  `requirement.dangling` findings are a **stale installed binary** —
  `cargo install --path .` clears them; a freshly-built `temper check .temper` is
  clean.
- **Pickable now:** DIRECTIVES-PRIMITIVE-PARSE (the one `open` entry; slices 2/3
  serialize behind it via `blockedBy`, sharing src/main.rs+graph.rs). Deferred
  (no consumer): EXTRACTION-VOCAB-GAPS, AGENT-KIND. Parked (human action):
  PACKAGING-CHANNELS, COMMUNITY-DOCS.

Plan continues: no — inbox drained into the serialized DIRECTIVES chain, carried
queue reconciled unchanged, one `open` entry pickable. Hand to build.
