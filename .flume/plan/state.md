# Plan state

- **Phase:** reconcile. Verified on disk this tick: no numeric `Range` predicate
  in `contract.rs`/`engine.rs` (the only `Range` is the charset `InvalidRange`;
  `inadmissibilities` carries no `min>max` numeric case); `compose.rs` `Role`
  holds `required: bool` only and `roster.rs` keeps the single-filler
  zero/one/many arms — no set-scope `count` band, no `min>max` roster
  admissibility. `contracts/` holds `rule.toml` + `skill.anthropic.toml`;
  `spec.toml` absent; `main.rs` `Check` arm runs no spec contract.
- **Last shipped:** DRIFT-DIFF. No `build:` since; tree clean, inbox empty, no
  new fork. Queue is unchanged from the last `plan:` commit (`80ff752`).
- **Queue:** GOV-RANGE (`open`, pickable — artifact-scope `range {min,max}`,
  contract.rs/engine.rs). GOV-COUNT (`blockedBy` GOV-RANGE — first set-scope
  roster predicate `count {min,max}`, compose.rs/roster.rs; file-disjoint from
  GOV-RANGE, but the human chore `7d58711` serialization is kept deliberately).
  SPEC-KIND-GATE (`parked` on a human committing the untracked `contracts/spec.toml`).
- **Unfiled frontier:** roster membership/typed-ref/unique (filed one at a time —
  next after GOV-COUNT ships); graph degree/acyclic await an edge-extraction +
  graph foundation (`45` "The harness is a graph too").

Plan continues: no — queue reconciled and accurate, GOV-RANGE pickable, inbox
empty. Hand to build.
