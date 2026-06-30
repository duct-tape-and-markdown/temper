# Plan state

- **Phase:** contract-engine pivot — replace the heuristic `all_rules()` registry
  with a generic validator over an author-declared contract (`10-contracts.md`
  "Decision: kill the heuristic rule registry"). 7 entries queued; none built.
- **Last shipped:** slice-1 vertical — `import`/`check` CLI, `Skill` IR,
  `author.toml` roll-up, the `all_rules()` heuristic registry, acceptance snapshots.
- **In flight:** nothing. Verified on disk this tick: `src/` = {check, import, lib,
  main, rules, skill}; no `contract`/`extract`/`engine`, no `contracts/`;
  `main.rs:57` still calls `rules::all_rules()`; `main.rs:4,20`+`lib.rs:10` still
  cite the absent `SPEC.md`/`spec/RELEASE-v0.1.md`; `rules.rs` + fixtures still hold
  the 5 rejected heuristics; `cargo check` clean. Corpus unchanged; all 7 `per`
  cites resolve; inbox empty; no forks moved.
- **Next:** build ships CONTRACT-MODEL + SKILL-EXTRACTOR → CONTRACT-ENGINE →
  SKILL-CONTRACT-TEMPLATE → CHECK-CUTOVER → RETIRE-HEURISTICS top-down
  (LIBDOC-EVERGREEN anytime — disjoint `lib.rs` doc region). The roles layer and
  declared-model/dependency-graph work get planned once the engine ships and the
  `(model-declaration-format)` intent gap is authored.

Plan continues: no — queue is reconciled and faithful to an unchanged corpus, and
three `open` entries (CONTRACT-MODEL, SKILL-EXTRACTOR, LIBDOC-EVERGREEN) are
pickable; building drains the queue.
