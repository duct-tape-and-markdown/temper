# Plan state

- **Phase:** reconcile. HEAD 340aa70.
- **Last shipped:** CUSTOM-UNIT-REPRESENTATION-CARRY + READ-VERBS-PUBLISHED-DEMANDS
  (build 320444a/74409e0, chore 459f140) — the custom-unit trace carry-forward and
  the composed-requirement read range.
- **This tick:** drained `.flume/inbox.md` — the human's WAVE 1 (memory kinds) —
  into **five dependency-ordered engine slices**: MEMORY-COLLISION-SCOPE and
  IMPORT-BUILTIN-SCAN-GENERIC (`open`, disjoint files builtin_kind.rs / import.rs),
  then CHECK-WORKSPACE-KIND-MAP + DECLARED-FRONTMATTER-ADAPTER-CUSTOM (blockedBy
  slice 2), then RECURSIVE-GOVERNS-PLACEMENT-ID (blockedBy slice 4). The three
  import.rs slices **serialize** via `blockedBy` (shared-file merge safety); slice 3
  (check.rs + bundle/drift/read) is disjoint and parallel-safe with slice 4. Every
  cited line re-verified on disk (builtin_kind.rs:85-96/112-119, import.rs
  123-124/135/362-409/295-349/453-484, check.rs:34-38, kind.rs:30/77-83,
  frontmatter.rs:186-196). Reconciled the old MEMORY-KIND: its engine wiring is now
  the wave, so it becomes the **parked** flip-ceremony validation (tests/memory_contract.rs).
  The other four tail entries (EXTRACTION-VOCAB-GAPS, AGENT-KIND, PACKAGING-CHANNELS,
  COMMUNITY-DOCS) reconcile clean, unchanged. Inbox now empty.
- **Operational note (accepted, not queued):** the 17 `requirement.dangling`
  session-start findings are a **stale installed binary** (`~/.cargo/bin/temper`
  predates the member-published-requirements union) — the freshly-built binary's
  `check` is clean. Fix is `cargo install --path .`, not spec/build work.
- **Pickable now:** MEMORY-COLLISION-SCOPE + IMPORT-BUILTIN-SCAN-GENERIC (`open`,
  disjoint — build's next wave). blockedBy chain unlocks as they ship. Parked:
  MEMORY-KIND, PACKAGING-CHANNELS, COMMUNITY-DOCS (human action). Deferred:
  EXTRACTION-VOCAB-GAPS, AGENT-KIND (no consumer). OPEN forks stay human-to-settle.

Plan continues: no — queue reconciled, inbox drained, two disjoint `open` entries
pickable. Hand to build; building is how the wave drains.
