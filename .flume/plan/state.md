# Plan state

- **Phase:** reconcile. Queue reconciled to the corpus; inbox drained (three lines
  routed into pending); no new fork.
- **Last shipped (trunk):** OFFERING-COMMUNITY — the launch community surface under
  `.github/` (CONTRIBUTING/SECURITY/issue forms) (bebb6aa / 4e87522). Since the last
  plan tick the std-lib packages were authored (966147d) and the memory kind joined
  the corpus enumeration (950257b). HEAD is bebb6aa.
- **In flight / anomaly:** COVERAGE-CUSTOM-KIND was attempted and **reverted** by the
  `cargo fmt` afterCommit gate — its unformatted diff (src/kind.rs, src/main.rs,
  tests/coverage.rs) sits uncommitted in the working tree, not on trunk (HEAD confirms
  kind.rs:740 `satisfies: Vec::new()`, main.rs:624-629 skill⊕rule only). Stays `open`;
  build retries from a clean worktree and MUST `cargo fmt --all` before commit. Residue
  is not plan-writable — a human may `git checkout -- src tests`.
- **Pickable now (4 disjoint / parallel-safe):** OFFERING-LICENSE (Cargo.toml+LICENSE-*),
  OFFERING-README (README.md+scripts/), AGENTS-MD (AGENTS.md), CHANGELOG-STUB
  (CHANGELOG.md) — plus COVERAGE-CUSTOM-KIND (kind.rs+main.rs+coverage.rs), disjoint
  from those four. **Serialized main.rs chain:** COVERAGE → SESSION-START-CUSTOM-KIND
  (blockedBy COVERAGE) → EMBED-BUILTIN-PACKAGES (blockedBy SESSION-START) — all three
  edit src/main.rs, so they ship one at a time. Parked: PACKAGING-CHANNELS (human
  release creds). Deferred: AGENT-KIND (priority; the newly-enumerated memory kind is
  deprioritized the same way and left unfiled).
- **Inbox:** drained (EMBED un-park → filed; front-door docs → AGENTS-MD + CHANGELOG-STUB;
  the dogfood BUG → SESSION-START-CUSTOM-KIND, high-priority, chained behind COVERAGE).
  **Forks:** none new.

Plan continues: no — the queue is reconciled, four disjoint `open` docs plus the
head of the serialized main.rs chain are pickable, and the inbox is drained. Building
drains the queue from here.
