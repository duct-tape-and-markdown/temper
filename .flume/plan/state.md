# Plan state

- **Phase:** reconcile. HEAD fb1dad3.
- **Last shipped:** CHECK-MEMBERS-ALL-KINDS (build e92bf62 / chore fb1dad3) — the
  check gate now dispatches *every* embedded kind's members to its floor package by
  qualified identity, so a discovered CLAUDE.md fires its `memory.anthropic`
  `max_lines` advisory instead of being silently skipped. Verified on disk
  (src/main.rs:655-683; tests/memory_gate.rs green). This unblocked MEMORY-KIND.
- **This tick:** drained the inbox wedge-UX pair + version drop (added 04ec7eb,
  after the last plan tick). Filed three `open` entries — WEDGE-INSTALL-SUMMARY
  (collapse the per-artifact install advisories to one summary),
  WEDGE-COVERAGE-NOTE (advisory coverage note so wedge silence ≠ "checked"),
  SKILL-VERSION-EXTRACTION-DROP (re-pin every `version` example field onto
  `license` so the human can drop the uncited extraction). Flipped MEMORY-KIND to
  `open` and **narrowed** it: memory_gate.rs already proves max_lines fire/silent,
  so the residual gap is the frontmatterless File-shaped import round-trip
  idempotence snapshot (adapter_fidelity covers only a Directory-shaped rule). All
  four open entries are file-disjoint (verified path-by-path) → parallel-safe.
- **Operational note (accepted, not queued):** the session-start
  `requirement.dangling` findings are a **stale installed binary** —
  `cargo install --path .` clears them; a freshly-built `temper check .temper` is
  clean.
- **Pickable now (all open, disjoint):** MEMORY-KIND (tests/memory_contract.rs),
  WEDGE-INSTALL-SUMMARY (install.rs), WEDGE-COVERAGE-NOTE (new module + main.rs),
  SKILL-VERSION-EXTRACTION-DROP (import/frontmatter/engine + tests). Parked (human
  action): PACKAGING-CHANNELS, COMMUNITY-DOCS. Deferred (no consumer):
  EXTRACTION-VOCAB-GAPS, AGENT-KIND.

Plan continues: no — inbox drained, queue reconciled, four disjoint open entries
filed. Hand to build.
