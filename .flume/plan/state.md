# Plan state

- **Phase:** contract-engine pivot — replace the heuristic `all_rules()` registry
  with a generic validator over an author-declared contract (`10-contracts.md`
  "Decision: kill the heuristic rule registry"). 5 entries queued.
- **Last shipped:** SKILL-EXTRACTOR — `src/extract.rs` (`Features`, `FeatureValue`,
  `skill_features`, name-keyed `field`/`has_field`, `body_lines`, `source_dir`,
  `companions`), the engine-facing surface-decidable view of a parsed skill.
- **In flight:** nothing. Verified on disk this tick: `src/` = {check, contract,
  extract, import, lib, main, rules, skill}; `extract.rs` exports `Features` as
  cited; no `engine`, no `contracts/` dir; `main.rs:57` still calls
  `rules::all_rules()`; `main.rs:5,20` + `lib.rs:10-11` still cite the absent
  `spec/RELEASE-v0.1.md`/`SPEC.md`; all five RETIRE-HEURISTICS fixtures + the
  `acceptance__rules_check_diagnostics.snap` snapshot present; `cargo check` clean.
  Corpus unchanged; all 5 `per` cites resolve; inbox empty; no forks moved.
  Repointed CONTRACT-ENGINE's gate (`blockedBy: SKILL-EXTRACTOR` → `open`) now that
  its `extract::Features` dependency shipped.
- **Next:** build ships CONTRACT-ENGINE (open) → CHECK-CUTOVER → RETIRE-HEURISTICS;
  SKILL-CONTRACT-TEMPLATE + LIBDOC-EVERGREEN are open/disjoint and pickable anytime.
  Roles layer + declared-model/dependency-graph work get planned once the engine
  ships and the `(model-declaration-format)` intent gap is authored.

Plan continues: no — queue reconciled against an unchanged corpus, one satisfied
gate repointed, and three `open` entries (CONTRACT-ENGINE, SKILL-CONTRACT-TEMPLATE,
LIBDOC-EVERGREEN) are pickable; building drains the queue.
