# Plan state

- **Phase:** reconcile. Verified on disk: **SESSION-START-GATE shipped** —
  `Command::SessionStart` (`main.rs:255`) + `reporter::session_start`
  (`src/reporter.rs`), the one-shot import-internally advisory gate emitting the
  `claude-session-start` payload, over `tests/session_start.rs`. The reporter
  family now has its first member.
- **Last shipped:** SESSION-START-GATE (`e787117`). Queue was empty; state was stale.
- **Filed / pickable (1):** **CHECK-REPORTERS** (`open`) — the SARIF +
  GitHub-annotation reporters on `temper check` via a `--reporter` flag, the
  reporter-family siblings of `session_start`. Verified unshipped: `check` only
  calls `check::render`, no `--reporter` arg; `reporter.rs` has no `sarif`/`github`.
- **Frontier:** `bundle`/`install` and more built-in harness kinds
  (agent/hook/command/MCP/settings/plugin) each also touch `main.rs` (+ `import.rs`
  or a new module), so they serialize on that shared surface behind CHECK-REPORTERS
  — not filed `open` now. Spec-kind `references-resolve` waits on
  `(reference-id-normalization)`; `decisions-name-alternatives` on `(decision-marker-predicate)`.
- **Inbox:** empty (nothing to drain). Open questions unchanged (no fork resolved this tick).

Plan continues: no — queue reconciled, SESSION-START-GATE dropped as shipped,
CHECK-REPORTERS filed `open` and pickable, inbox empty; hand to build.
