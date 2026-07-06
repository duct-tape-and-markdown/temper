# Plan state

- **Phase:** derived-lock chain filed. The 07-06 in-session ceremony (John +
  session) landed in `inbox.md`; this tick drained it into a serialized
  5-link chain and the residue sweep found one orphan (`contract/`).
- **Last shipped:** PKG-NOUN-EXPLAIN-DIAGNOSTIC (build 54a4e9a / chore
  48b875c) — the package noun's last diagnostic residue; the package-noun
  retirement is complete on disk.
- **In flight — 8 entries, 2 open & disjoint:**
  (1) REQUIREMENT-CLAUSES-ALGEBRA — open. Grow the closed clause `Predicate`
  algebra with the set-/edge-scope predicates (facet-only today:
  contract.rs `Predicate` :93 has no Count/Membership/Degree; they live as
  `Requirement` facets in compose.rs :64-94). Additive.
  (2) CONTRACT-CORPUS-RETIRE — open, disjoint. `contract/` is orphaned
  (verified: `rg contract/ src tests sdk/src` empty) — two-writer-era residue
  (20-surface). Pure deletion.
  Then the serialized derived-lock chain, each blockedBy the prior:
  REQUIREMENT-CLAUSES-RECUT → FIRST-PARTY-MODULE-COMPLETE →
  BUILTIN-LOCK-DERIVED → {BUILTIN-LOCK-FROZEN-LANE, CURATED-TREES-RETIRE}
  (the last two are parallel-safe leaves behind D3).
  And PACKAGING-CHANNELS — parked (human release creds + engine-binary
  workflow + USPTO screen).
- **What's next:** build the two open entries (parallel-safe); the chain
  unblocks link by link. Queued **human** chores, not build work: the
  physical `packages/**` + `kinds/**` tree deletion (out of build's fence)
  once BUILTIN-LOCK-DERIVED + CURATED-TREES-RETIRE remove the code refs; the
  PACKAGING-CHANNELS release setup. OPEN forks in open-questions.md still
  await John or the SDK-primary foundation.

Plan continues: no — queue reconciled, inbox drained, two disjoint open
entries ready. Building is how the chain drains; re-planning the same queue
would be spin.
