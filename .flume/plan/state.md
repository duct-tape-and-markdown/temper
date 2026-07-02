# Plan state

- **Phase:** reconcile. Queue is reconciled to the corpus; the inbox is empty.
- **Last shipped (trunk):** PACKAGE-CLAUSE-SOURCE — per-clause `source` citation
  parsed + preserved (6c51057 / 2bf7529). HEAD is 6c51057.
- **In flight / anomaly:** COVERAGE-CUSTOM-KIND was **attempted and reverted** by
  the `cargo fmt` afterCommit gate — its unformatted diff (src/kind.rs, src/main.rs,
  tests/coverage.rs) sits **uncommitted in the working tree, not on trunk**. The
  entry stays `open`; build retries from a clean fresh worktree (unaffected by the
  residue) and must run `cargo fmt --all` before commit. The residue is not plan-
  writable to clean — a human may `git checkout -- src tests` to clear it.
- **Pickable now (4, all disjoint / parallel-safe):** COVERAGE-CUSTOM-KIND
  (kind.rs+main.rs+tests/coverage.rs), OFFERING-LICENSE (Cargo.toml+LICENSE-*),
  OFFERING-COMMUNITY (new docs+.github/), OFFERING-README (README.md+scripts/). No
  path is shared across the four. Parked: EMBED-BUILTIN-PACKAGES (human authors
  packages/*/PACKAGE.md), PACKAGING-CHANNELS (human release creds). Deferred:
  AGENT-KIND (priority; shares main.rs with COVERAGE on revival).
- **Inbox:** empty. **Forks:** filed `(launch-front-door-docs)` — AGENTS.md +
  CHANGELOG launch docs, held on the AGENTS.md⟷CLAUDE.md canonicality question
  (human territory). No new fork gates a pickable head.

Plan continues: no — the queue is reconciled, four disjoint `open` entries are
pickable, the inbox is empty, and the only new gap is surfaced as an open question.
Building drains the queue from here.
