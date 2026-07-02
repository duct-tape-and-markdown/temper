# Plan state

- **Phase:** reconcile. Queue reconciled to the corpus; inbox empty; no new fork.
- **Last shipped (trunk):** OFFERING-README — the front-door README with a regenerable
  check-diagnostic hero (9732bda / c5f051d). HEAD is 9732bda; working tree clean.
- **In flight / anomaly:** COVERAGE-CUSTOM-KIND was attempted and **reverted** by the
  `cargo fmt` afterCommit gate; its residue has since been **cleared** (tree clean, HEAD
  confirms kind.rs:740 `satisfies: Vec::new()`, main.rs:653-658 chains skill⊕rule into
  `all_features` only). Stays `open`; build retries from clean HEAD and MUST `cargo fmt
  --all` before commit.
- **Pickable now (4 disjoint / parallel-safe):** OFFERING-LICENSE (Cargo.toml+LICENSE-*),
  AGENTS-MD (AGENTS.md), CHANGELOG-STUB (CHANGELOG.md) — plus COVERAGE-CUSTOM-KIND
  (kind.rs+main.rs+coverage.rs), disjoint from those three. **Serialized main.rs chain:**
  COVERAGE → SESSION-START-CUSTOM-KIND (blockedBy COVERAGE) → EMBED-BUILTIN-PACKAGES
  (blockedBy SESSION-START) — all edit src/main.rs, ship one at a time. Parked:
  PACKAGING-CHANNELS (human release creds). Deferred: AGENT-KIND (priority).
- **Inbox:** empty (drained prior tick). **Forks:** none new; read-verbs + the KIND-*
  chain remain RESOLVED-but-unfiled decision records, deprioritized behind the queue.

Plan continues: no — the queue is reconciled (COVERAGE/SESSION-START stale facts and
line refs corrected, README drop confirmed), three disjoint `open` docs plus the head of
the serialized main.rs chain are pickable, and the inbox is empty. Building drains it.
