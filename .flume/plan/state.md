# Plan state

- **Phase:** reconcile. Verified on disk: **APPLY-WRITEBACK shipped** — `drift.rs`
  carries the three-state write direction (`apply` + `project_one`, patch-not-re-emit,
  the Applied/Unchanged/Conflicted merge), wired as `Command::Apply` (`main.rs:334`);
  the lock's `last_applied` fingerprint is written at import and reconciled on apply.
  The custom-kind (KIND-*) tier is also on disk — `import` discovers custom kinds from
  `temper.toml` and `check` runs each through its own composed extractor + contract.
- **Last shipped:** APPLY-WRITEBACK (`4a7db52`). Queue was empty; APPLY dropped as shipped.
- **Filed / pickable (1):** **RE-ADD** (`open`) — `temper re-add`, the third drift
  direction (on-disk→surface), reusing `diff`'s classification + `import`'s writers over
  the built-in kinds. Serialized behind it: **SESSION-START-GATE** (`blockedBy RE-ADD`) —
  the advisory session-start reporter/verb; both touch `main.rs`, so one at a time.
- **Frontier:** `bundle`/`install`, the SARIF + GitHub-annotation reporters, and more
  built-in harness kinds (agent/hook/command/MCP/settings/plugin) are the unbuilt areas —
  each touches `main.rs` (and most `import.rs`), so all serialize on that shared surface,
  not filed `open` now. Spec-kind `references-resolve` waits on `(reference-id-normalization)`
  (custom-kind features never join `by_kind` at `main.rs:168`, so spec edges find no sources);
  `decisions-name-alternatives` waits on `(decision-marker-predicate)`.
- **Inbox:** empty (nothing to drain). Open questions unchanged (no fork resolved this tick).

Plan continues: no — queue reconciled, APPLY-WRITEBACK dropped as shipped, RE-ADD filed and pickable with SESSION-START-GATE serialized behind it, inbox empty; hand to build.
