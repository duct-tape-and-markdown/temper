# Plan state

- **Phase:** contract-engine pivot — replace the heuristic `all_rules()` registry
  with a generic validator over an author-declared contract (`10-contracts.md`
  "Decision: kill the heuristic rule registry"). 7 entries queued; none built.
- **Last shipped:** slice-1 vertical — `import`/`check` CLI, `Skill` IR,
  `author.toml` roll-up, the `all_rules()` registry, acceptance snapshots.
- **In flight:** nothing. Verified on disk this tick: `src/` = {check, import,
  lib, main, rules, skill}; no `contract`/`extract`/`engine`, no `contracts/`;
  `main.rs:57` calls `rules::all_rules()`; `main.rs:4,20` + `lib.rs:10` cite the
  absent `SPEC.md`/`spec/RELEASE-v0.1.md`; `rules.rs` still holds the 5 rejected
  heuristics (third-person, has-trigger, has-anti-trigger, companion-refs, refs-
  one-level-deep); all RETIRE-HEURISTICS fixtures present; `cargo check` clean.
  Corpus unchanged; all 7 `per` cites resolve; inbox empty; no forks moved.
- **Next:** ship CONTRACT-MODEL + SKILL-EXTRACTOR → CONTRACT-ENGINE →
  SKILL-CONTRACT-TEMPLATE → CHECK-CUTOVER → RETIRE-HEURISTICS top-down
  (LIBDOC-EVERGREEN ships anytime, disjoint `lib.rs` region). Then plan the
  harness-contract/roles layer (`10-contracts.md` "Roles and matching" +
  `verified_by`) and the declared-model/dependency-graph work — the latter held
  on the `(model-declaration-format)` fork, a human intent gap.

Plan continues: yes — the artifact-contract engine is filed but wholly unbuilt
(6-entry chain + 1 doc scrub), and the roles layer + declared-model/dependency-
graph work still need planning once the engine ships and the model-declaration
format is authored.
