# Plan state

- **Phase:** reconcile. Verified on disk this tick: GOV-COUNT **shipped** —
  `compose.rs` `Role` carries `count: Option<CountBound>` (with the
  required/count mutual-exclusion load error), and `roster.rs` decides the
  set-scope band in `check` (`out_of_band`) plus the `min > max` inverted-bound
  case in `admissibility`. `contracts/` holds `rule.toml` + `skill.anthropic.toml`
  only; `spec.toml` still untracked/absent and `main.rs` carries no
  `BUILTIN_SPEC_CONTRACT`; `extract::spec_features` is on disk.
- **Last shipped:** GOV-COUNT (`e31ab73`). Tree clean, inbox empty, no new fork.
- **Queue:** GOV-UNIQUE **`open`, pickable** — the set-scope `unique` predicate (a
  declared field unique across a role's matched set, `compose.rs`/`roster.rs`),
  generalizing the kind-wide `unique-name` engine predicate to a role's subset.
  It is the only pickable entry, so `open` is parallel-safe. SPEC-KIND-GATE
  **`parked`** on a human committing the untracked `contracts/spec.toml`.
  GOV-UNIQUE (compose/roster) and SPEC-KIND-GATE (main.rs) are disjoint anyway.
- **Unfiled frontier:** set-scope membership + typed-reference (both two-selector —
  filed one at a time, next after GOV-UNIQUE ships); graph degree/acyclic await an
  edge-extraction + graph foundation (`45` "The harness is a graph too").

Plan continues: no — GOV-COUNT confirmed shipped, GOV-UNIQUE filed as the one
pickable entry, queue reconciled and accurate, inbox empty. Hand to build.
