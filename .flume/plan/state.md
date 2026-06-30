# Plan state

- **Phase:** reconcile. Verified on disk this tick: GOV-RANGE **shipped** —
  `Predicate::Range` is parsed in `contract.rs` and decided in `engine.rs`, with
  the `min > max` inadmissibility case. GOV-COUNT **not** shipped — `compose.rs`
  `Role` carries `required: bool` only (no `count`), and `roster.rs` keeps the
  single-filler zero/one/many arms (no set-scope `count` band, no `min>max` roster
  admissibility). `contracts/` holds `rule.toml` + `skill.anthropic.toml` only;
  `spec.toml` still untracked/absent; `extract::spec_features` is on disk.
- **Last shipped:** GOV-RANGE (`498589d`). Tree clean, inbox empty, no new fork.
- **Queue:** GOV-COUNT now **`open`, pickable** — its blocker GOV-RANGE shipped, so
  the dangling `blockedBy` resolved to `open`; it is the first set-scope roster
  predicate (`count {min,max}`, compose.rs/roster.rs) and the only pickable entry,
  so `open` is parallel-safe. SPEC-KIND-GATE **`parked`** on a human committing the
  untracked `contracts/spec.toml`.
- **Unfiled frontier:** roster membership/typed-ref/unique (filed one at a time —
  next after GOV-COUNT ships); graph degree/acyclic await an edge-extraction +
  graph foundation (`45` "The harness is a graph too").

Plan continues: no — GOV-RANGE shipped, GOV-COUNT flipped to pickable, queue
reconciled and accurate, inbox empty. Hand to build.
