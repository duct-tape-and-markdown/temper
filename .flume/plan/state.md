# Plan state

- **Phase:** reconcile. Verified on disk this tick: GOV-UNIQUE **shipped** —
  `compose.rs` `Role` carries `unique: Vec<String>` (with the `RoleBadUnique`
  parse guard, orthogonal to `count`/`required`), and `roster.rs` `check` decides
  it via `ROLE_UNIQUE_RULE` + `duplicates` (group the matched set by the field's
  scalar, flag each shared value). No `membership` exists anywhere in `src/`.
- **Last shipped:** GOV-UNIQUE (`0dbf402`). Tree clean, inbox empty, no new fork.
- **Queue:** GOV-MEMBERSHIP **`open`, pickable** — the next set-scope predicate
  (`45-governance.md`): a declared field on each matched artifact must lie in the
  feature-set derived from a second selector's matched artifacts (`compose.rs` +
  `roster.rs`). SPEC-KIND-GATE **`parked`** on a human committing the untracked
  `contracts/spec.toml`. The two are disjoint (compose/roster vs `main.rs`), and
  GOV-MEMBERSHIP is the only pickable entry, so `open` is parallel-safe.
- **Unfiled frontier:** `(typed-reference)` — membership where S₂ is "kind K
  conforming-to contract C" — filed next, after GOV-MEMBERSHIP ships. Graph
  degree/acyclic still await an edge-extraction + graph foundation (`45` "The
  harness is a graph too").

Plan continues: no — GOV-UNIQUE confirmed shipped and dropped, GOV-MEMBERSHIP
filed as the one pickable entry, queue reconciled, inbox empty. Hand to build.
