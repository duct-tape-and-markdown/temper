# Plan state

- **Phase:** contract-engine pivot — the generic `engine::validate` now replaces
  the heuristic `all_rules()` registry; remaining work wires it into `check` and
  retires the dead registry (`10-contracts.md` "Decision: kill the heuristic rule
  registry").
- **Last shipped:** CONTRACT-ENGINE — `src/engine.rs` (`validate`): evaluates a
  `contract::Contract`'s clauses over extracted `Features`, maps declared severity
  → diagnostic severity, and returns an honest `Indeterminate` for predicates
  whose backing feature the projection doesn't carry (no fabricated pass).
- **In flight:** nothing. Verified on disk: `engine.rs` shipped + `lib.rs` has
  `pub mod engine;`; no `contracts/` dir; `main.rs:57` still calls
  `rules::all_rules()`; `rules.rs`/`tests/rules.rs`/5 dropped-heuristic fixtures +
  the rules snapshot all present; stale `spec/RELEASE-v0.1.md`/`SPEC.md` citations
  linger in lib.rs, main.rs, import.rs, check.rs, skill.rs, cli.rs, acceptance.rs,
  rules.rs; `cargo check` clean. Repointed CHECK-CUTOVER's satisfied gate
  (CONTRACT-ENGINE shipped → `blockedBy SKILL-CONTRACT-TEMPLATE`, its include_str!
  target) and filed DOCS-EVERGREEN for the doc drift no behavioral entry covers.
- **Next:** build ships SKILL-CONTRACT-TEMPLATE (open) → CHECK-CUTOVER →
  RETIRE-HEURISTICS; LIBDOC-EVERGREEN + DOCS-EVERGREEN are open pure-doc, pickable
  anytime. After the cutover lands: reconcile the then-callerless `check::Rule`/
  `check::run`, and plan the roles + declared-model/dependency-graph layer once
  the `(model-declaration-format)` and `(contract-selection)` forks resolve.

Plan continues: no — queue reconciled against an unchanged corpus, one satisfied
gate repointed and one doc-drift entry filed; three `open` entries are pickable,
so building drains the queue.
