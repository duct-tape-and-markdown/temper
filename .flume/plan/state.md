# Plan state

- **Phase:** contract-engine pivot — replace the heuristic `all_rules()` registry
  with a generic validator over an author-declared contract (`10-contracts.md`
  "Decision: kill the heuristic rule registry"). 6 entries queued.
- **Last shipped:** CONTRACT-MODEL — `src/contract.rs` (Contract, Clause, Severity,
  Predicate, Charset, ContractError, load/parse), `pub mod contract;` in lib.rs.
- **In flight:** nothing. Verified on disk this tick: `src/` = {check, contract,
  import, lib, main, rules, skill}; no `extract`/`engine`, no `contracts/` dir;
  `main.rs:57` still calls `rules::all_rules()`; `main.rs:4,20` + `lib.rs` still
  cite the absent `spec/RELEASE-v0.1.md`/`SPEC.md`; `cargo check` clean. Corpus
  unchanged; all 6 `per` cites resolve; inbox empty; no forks moved. Repointed two
  stale `blockedBy: CONTRACT-MODEL` gates now that it shipped (CONTRACT-ENGINE →
  blockedBy SKILL-EXTRACTOR for `extract::Features`; SKILL-CONTRACT-TEMPLATE → open).
- **Next:** build ships SKILL-EXTRACTOR (open) → CONTRACT-ENGINE → CHECK-CUTOVER →
  RETIRE-HEURISTICS; SKILL-CONTRACT-TEMPLATE + LIBDOC-EVERGREEN are open/disjoint and
  pickable anytime. Roles layer + declared-model/dependency-graph work get planned
  once the engine ships and the `(model-declaration-format)` intent gap is authored.

Plan continues: no — queue reconciled against an unchanged corpus, two dangling
gates repointed, and three `open` entries (SKILL-EXTRACTOR, SKILL-CONTRACT-TEMPLATE,
LIBDOC-EVERGREEN) are pickable; building drains the queue.
