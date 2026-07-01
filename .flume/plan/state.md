# Plan state

- **Phase:** reconcile. Verified on disk: **CHECK-REPORTERS shipped** —
  `reporter::sarif` + `reporter::github` (`src/reporter.rs`) rendered from `check`'s
  one diagnostic source behind the `--reporter` flag (`main.rs`, `Command::Check`),
  over `tests/reporters.rs`. The reporter family is now complete: `session_start` +
  `github` + `sarif`, one contract, every placement.
- **Last shipped:** CHECK-REPORTERS (`9010dfe`). Queue was empty; state was stale.
- **Filed / pickable (1):** **AGENT-KIND** (`open`) — the `agent` built-in harness
  kind (`.claude/agents/*.md`), the next kind after skill + rule: an Agent IR
  mirroring `rule.rs`, an embedded decidable `agent.toml` floor, wired into
  import/check/schema. Verified unshipped: no `src/agent.rs`, no `agents` field on
  `Workspace`, no `agent` in `gate()`'s by-kind dispatch.
- **Frontier:** the remaining built-in kinds (command/hook/MCP/settings/plugin) and
  the `bundle`/`install` verbs each also touch the shared surface
  (`main.rs`/`check.rs`/`extract.rs`/`import.rs` + a new module), so they serialize
  behind AGENT-KIND — not filed `open` now. Spec-kind `references-resolve` waits on
  `(reference-id-normalization)`; `decisions-name-alternatives` on `(decision-marker-predicate)`.
- **Inbox:** empty (nothing to drain). Open questions unchanged (no fork resolved this tick).

Plan continues: no — queue reconciled, CHECK-REPORTERS dropped as shipped,
AGENT-KIND filed `open` and pickable, inbox empty; hand to build.
