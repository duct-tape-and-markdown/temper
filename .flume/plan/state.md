# Plan state

- **Phase:** reconcile. Verified on disk this tick: no numeric `Range` predicate
  in `contract.rs`/`engine.rs` (the only `Range` is the `allowed_chars`
  `InvalidRange`; `inadmissibilities` carries no `min>max` numeric case);
  `compose.rs` `Role` holds `required: bool` only and `roster.rs` keeps the
  single-filler `[_]/[]/many` arms — no set-scope `count` band, no `min>max`
  roster admissibility. `contracts/` holds `rule.toml` + `skill.anthropic.toml`;
  `spec.toml` absent; `main.rs` `Check` arm runs no spec contract.
- **Last shipped:** DRIFT-DIFF. No `build:` since; tree clean, inbox empty, no
  new fork. Most recent commit is the human chore `7d58711` serializing GOV-COUNT.
- **Queue:** GOV-RANGE (`open`, pickable — artifact-scope `range {min,max}`,
  contract.rs/engine.rs). GOV-COUNT (`blockedBy` GOV-RANGE — first set-scope
  roster predicate `count {min,max}`, compose.rs/roster.rs; serialized by the
  human chore, kept). SPEC-KIND-GATE (`parked` on a human committing the
  untracked `contracts/spec.toml`).
- **Unfiled frontier:** roster membership/typed-ref/unique (filed one at a time —
  next after GOV-COUNT ships); graph degree/acyclic await an edge-extraction +
  graph foundation (`45` "The harness is a graph too").

Plan continues: no — queue reconciled (GOV-RANGE pickable); stale `state.md`
fixed to match the human serialization of GOV-COUNT. Hand to build.
