# Plan state

- **Phase:** contract-engine cutover — the generic `engine::validate` is shipped;
  remaining work ships the bundled skill template, wires `check` onto the engine,
  and retires the dead heuristic registry (`10-contracts.md` "Decision: kill the
  heuristic rule registry").
- **Last shipped:** LIBDOC-EVERGREEN + DOCS-EVERGREEN — repointed the stale
  `spec/RELEASE-v0.1.md`/`SPEC.md` crate-doc citations in `lib.rs`, `check.rs`,
  `import.rs`, `skill.rs` at the evergreen `specs/` corpus.
- **In flight:** nothing. Verified on disk: no `contracts/` dir; `main.rs:57`
  still calls `rules::all_rules()`; `rules.rs`/`tests/rules.rs`/the 5 dropped-
  heuristic fixtures + `acceptance__rules_check_diagnostics.snap` all present;
  `lib.rs` still has `pub mod rules`; `Contract::parse`, `engine::validate`,
  `extract::skill_features`, `check::render`/`any_error` all present; stale
  `RELEASE-v0.1` cites remain only in `main.rs`/`cli.rs` (CHECK-CUTOVER scope) and
  `acceptance.rs` (RETIRE-HEURISTICS scope), bundled into those behavioral entries.
  `cargo check` clean.
- **Next:** build ships SKILL-CONTRACT-TEMPLATE (`open`) → CHECK-CUTOVER →
  RETIRE-HEURISTICS. After the cutover lands: reconcile the then-callerless
  `check::Rule`/`check::run`, and plan the roles + declared-model/dependency-graph
  layer once the `(model-declaration-format)` and `(contract-selection)` forks
  resolve.

Plan continues: no — queue reconciled against an unchanged corpus, nothing
shipped from the three entries, inbox empty; SKILL-CONTRACT-TEMPLATE is `open` and
pickable, so building drains the queue.
