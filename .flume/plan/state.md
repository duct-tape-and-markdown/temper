# Plan state

- **Phase:** reconcile. Verified on disk this tick: no numeric `Range` predicate
  in `contract.rs`/`engine.rs` (the only `Range` is the `allowed_chars` charset
  `InvalidRange`; `engine.rs` `inadmissibilities` carries no `min>max` numeric
  case); `compose.rs` `Role` holds `required: bool` only and `roster.rs` has the
  single-filler arms — no set-scope `count` band. `contracts/` holds `rule.toml`
  + `skill.anthropic.toml`; `spec.toml` still absent; `main.rs` `Check` arm runs
  no spec contract.
- **Last shipped:** DRIFT-DIFF (c63d238 / 22b3425). No `build:` since.
- **In flight:** nothing; tree clean. Inbox empty; no new fork.
- **Queue:** GOV-RANGE (`open`, pickable — artifact-scope `range {min,max}`,
  contract.rs/engine.rs). GOV-COUNT (`open`, pickable — first set-scope roster
  predicate `count {min,max}`, compose.rs/roster.rs). SPEC-KIND-GATE (`parked` on
  a human committing untracked `contracts/spec.toml`).
- **Unfiled frontier:** roster membership/typed-ref/unique; graph degree/acyclic
  await an edge-extraction + graph foundation (`45` "The harness is a graph too").

Plan continues: no — GOV-RANGE and GOV-COUNT are both `open` and pickable; the
queue reconciled with no change. Hand to build.
