# Plan state

- **Phase:** reconcile. Verified on disk: **SCHEMA-EMIT shipped** — `src/schema.rs`
  emits the active per-kind contract as an editor JSON Schema, wired as `Command::Schema`
  (`main.rs:84`, dispatch `main.rs:261`) over the same by-kind floor ⊕ `temper.toml` layer
  `check` gates against (validation channel; docs/hover deferred).
- **Last shipped:** SCHEMA-EMIT (`5e79663`). Queue held one blocked entry; SCHEMA-EMIT's
  ship cleared the gate.
- **In flight / pickable (1):** **APPLY-WRITEBACK** (`open`) — `temper apply`, the three-state
  write-back engine (`drift.rs` today has read-only `diff` only; the lock `RollupEntry` carries
  `import_hash`+`body_hash`, no last-applied fingerprint yet). All gating forks RESOLVED.
- **Frontier:** `re-add` (on-disk→surface, sibling of apply) and `bundle`/`install`/reporters
  are the unbuilt distribution/write-back areas; more built-in harness kinds
  (agent/hook/command/MCP/settings/plugin) remain adapters to add — all touch `import.rs`+`main.rs`,
  so each collides with APPLY's blast radius and is serialized behind it, not filed `open` now.
  Spec-kind `references-resolve` waits on `(reference-id-normalization)`; `decisions-name-alternatives`
  waits on `(decision-marker-predicate)`.
- **Inbox:** empty (nothing to drain). Open questions unchanged (no fork resolved this tick).

Plan continues: no — queue reconciled, SCHEMA-EMIT dropped as shipped, APPLY-WRITEBACK unblocked and pickable, inbox empty; hand to build.
