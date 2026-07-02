# Plan state

- **Phase:** reconcile. The surface-language machinery chain is **complete and
  self-joining** — GRAPH-CUSTOM-KIND (8f057c7) shipped, so a custom kind's features
  and `[[relationships]]` now reach `by_kind`/`edges` and the corpus graph (verified
  on disk: main.rs:530-545). Only follow-on machinery remains.
- **Last shipped:** GRAPH-CUSTOM-KIND — custom-kind members joined to the corpus
  reference graph (8f057c7 / 2c75c81).
- **In flight:** none.
- **Pickable now (5, all disjoint / parallel-safe):** COVERAGE-CUSTOM-KIND
  (kind.rs+main.rs — revived, GRAPH unblocked it), PACKAGE-CLAUSE-SOURCE
  (contract.rs — inbox), OFFERING-LICENSE (Cargo.toml+LICENSE-*), OFFERING-COMMUNITY
  (new docs), OFFERING-README (README.md+scripts/). Parked: EMBED-BUILTIN-PACKAGES
  (human authors packages/), PACKAGING-CHANNELS (human release creds). Deferred:
  AGENT-KIND (priority; shares main.rs with COVERAGE on revival).
- **Inbox:** drained (source-key → PACKAGE-CLAUSE-SOURCE; offering tranche →
  OFFERING-{LICENSE,COMMUNITY,README}). **Forks:** none new; only the human-only
  OPEN strategics remain, none gate a pickable head.

Plan continues: no — the queue is reconciled to the corpus, five disjoint `open`
entries are pickable, the inbox is drained, and the fork frontier is clear.
Building drains it from here.
