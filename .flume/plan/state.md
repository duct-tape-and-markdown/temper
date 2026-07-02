# Plan state

- **Phase:** reconcile. Queue reconciled to the corpus; inbox empty; no new fork.
- **Last shipped (trunk):** COVERAGE-CUSTOM-KIND + CHANGELOG-STUB (37c7ea2) — custom-kind
  units join coverage via authored `satisfies`, and a root CHANGELOG stub exists. HEAD is
  37c7ea2; working tree clean.
- **In flight / anomaly:** none. Verified on disk: `contracts/{skill.anthropic,rule}.toml`
  still embedded (main.rs:59/65); `packages/{skill.anthropic,rule.anthropic}/PACKAGE.md`
  authored; `Contract::load_package` at contract.rs:586; `Cargo.toml` still `license = "MIT"`;
  no LICENSE-*/AGENTS.md on disk.
- **Pickable now (3 disjoint / parallel-safe):** SESSION-START-CUSTOM-KIND (main.rs +
  session_start.rs — now head of the main.rs chain: its blocker COVERAGE-CUSTOM-KIND
  shipped), OFFERING-LICENSE (Cargo.toml + LICENSE-*), AGENTS-MD (AGENTS.md). **Serialized
  main.rs chain:** SESSION-START → EMBED-BUILTIN-PACKAGES (blockedBy SESSION-START; both
  edit src/main.rs). Parked: PACKAGING-CHANNELS (human release creds). Deferred: AGENT-KIND
  (priority).
- **Inbox:** empty. **Forks:** none new; read-verbs + the KIND-* chain remain
  RESOLVED-but-unfiled decision records, deprioritized behind the queue (read-verbs still
  tail-gated behind EMBED, the last surface-language migration step).

Plan continues: no — the only reconciliation this tick was flipping SESSION-START-CUSTOM-KIND
to `open` now that COVERAGE shipped; three disjoint `open` entries plus the serialized EMBED
tail are pickable and the inbox is empty. Building drains it.
