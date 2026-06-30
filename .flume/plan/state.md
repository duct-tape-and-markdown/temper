# Plan state

- **Phase:** reconcile. Verified on disk: `contract.rs`'s only `Range` is the
  `allowed_chars` charset-range error (`a-z`), not the numeric predicate;
  `engine.rs` has no `Range` arm in `decide`/`inadmissibilities`. Governance
  (`45`) unbuilt. `contracts/` holds `rule.toml` + `skill.anthropic.toml`;
  `spec.toml` still absent.
- **Last shipped:** DRIFT-DIFF (c63d238 / 22b3425). No `build:` since.
- **In flight:** nothing; tree clean. Inbox empty; no new fork.
- **Queue (unchanged, both entries accurate):** GOV-RANGE (`open`, pickable —
  numeric `range {min,max}` over contract.rs + engine.rs, `45` "Also in scope").
  SPEC-KIND-GATE (`parked` on a human committing untracked `contracts/spec.toml`).

Plan continues: no — GOV-RANGE is `open` and immediately pickable; hand to build.
