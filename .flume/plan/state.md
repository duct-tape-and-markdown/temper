# Plan state

- **Phase:** reconcile. HEAD 61fa275.
- **Last shipped:** MEMORY-COLLISION-SCOPE (build e18e206, chore 61fa275) — the
  memory wave's slice 1: bare-name kind collision is scoped to the binding/lookup
  caller, so two providers can co-embed one bare `memory` without eager-resolution
  turning cargo test red. Verified on disk: `builtin_kind.rs` carries the
  qualified-identity keying + `CustomKind::resolve_bare` (unique-or-collision).
- **This tick:** reconcile-only. Confirmed on disk that slice 1 shipped and is
  correctly gone from the queue, and that IMPORT-BUILTIN-SCAN-GENERIC has **not**
  shipped — `import.rs:123-124` still hardcodes `import_frontmatter_kind(.., "skill")`
  / `(.., "rule")` and `:135` still reads the `BUILTIN_KINDS` literal. Re-verified
  every downstream cite: `builtin_kind::definitions()` (builtin_kind.rs:92),
  `Workspace { skills, rules }` (check.rs:34-38) + `load()` (:50-62), `BUILTIN_KINDS
  = ["skill","rule"]` (kind.rs:30) — all accurate, no entry rewritten. Refreshed the
  lead entry's stale "open beside slice 1" note. Inbox empty; no new corpus↔src gap.
- **Operational note (accepted, not queued):** the 17 `requirement.dangling`
  session-start findings are a **stale installed binary** (`~/.cargo/bin/temper`
  predates the member-published-requirements union) — the freshly-built binary's
  `check` is clean. Fix is `cargo install --path .`, not spec/build work.
- **Pickable now:** IMPORT-BUILTIN-SCAN-GENERIC (`open`, sole pickable — build's
  next tick). Its `blockedBy` chain unlocks as it ships: CHECK-WORKSPACE-KIND-MAP +
  DECLARED-FRONTMATTER-ADAPTER-CUSTOM (both blockedBy it, then disjoint/parallel),
  then RECURSIVE-GOVERNS-PLACEMENT-ID (blockedBy DECLARED). Parked: MEMORY-KIND,
  PACKAGING-CHANNELS, COMMUNITY-DOCS (human action). Deferred: EXTRACTION-VOCAB-GAPS,
  AGENT-KIND (no consumer). OPEN forks stay human-to-settle.

Plan continues: no — queue reconciled against disk, inbox empty, one `open` entry
pickable. Hand to build; building is how the wave drains.
</content>
