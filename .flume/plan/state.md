# Plan state

- **Phase:** reconcile. HEAD 78195e0.
- **Last shipped:** IMPORT-BUILTIN-SCAN-GENERIC (build aa825bd, chore 78195e0) —
  the memory wave's slice 2: `import::run` drives the built-in scan off
  `builtin_kind::definitions()`, never a `["skill","rule"]` literal, so a third
  embedded kind discovers its members through the one generic frontmatter adapter.
  Verified on disk: import.rs:120-134 iterates `definitions()`, and the custom
  split reads off the same embedded set (`builtin_names`), not a literal.
- **This tick:** reconcile-only. Slice 2 shipped → unlocked its two disjoint
  dependents to `open`: CHECK-WORKSPACE-KIND-MAP (check.rs + bundle/drift/read) and
  DECLARED-FRONTMATTER-ADAPTER-CUSTOM (import.rs). Re-verified every cite on disk —
  `Workspace { skills, rules }` (check.rs:34-38) + `load()` (:50-62); bundle.rs:209
  reads `.skills.len()`/`.rules.len()`; drift.rs:162/176/419/430 and read.rs:89/96
  iterate the two fields; `import_custom_unit` (import.rs:372-409) still derives id
  from `file_stem` and ignores the declared `unit_shape`/`format` — all accurate, no
  entry rewritten. Refreshed the two unlocked entries' stale "blockedBy slice 2"
  notes. Inbox empty; no new corpus↔src gap.
- **Operational note (accepted, not queued):** the 17 `requirement.dangling`
  session-start findings are a **stale installed binary** (`~/.cargo/bin/temper`
  predates the member-published-requirements union) — the freshly-built binary's
  `check` is clean. Fix is `cargo install --path .`, not spec/build work.
- **Pickable now:** CHECK-WORKSPACE-KIND-MAP + DECLARED-FRONTMATTER-ADAPTER-CUSTOM
  (both `open`, disjoint files → parallel-safe). Then RECURSIVE-GOVERNS-PLACEMENT-ID
  (blockedBy DECLARED, shares import.rs). Parked: MEMORY-KIND, PACKAGING-CHANNELS,
  COMMUNITY-DOCS (human action). Deferred: EXTRACTION-VOCAB-GAPS, AGENT-KIND (no
  consumer). OPEN forks stay human-to-settle.

Plan continues: no — queue reconciled against disk, inbox empty, two disjoint
`open` entries pickable. Hand to build; building is how the wave drains.
