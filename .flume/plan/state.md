# Plan state

- **Phase:** contract-engine pivot — replace the heuristic rule registry with the
  generic validator (`10-contracts.md` "Decision: kill the heuristic rule
  registry"). 6 entries queued; none built yet.
- **Last shipped:** the slice-1 vertical (`import`/`check` CLI, `Skill` IR,
  `author.toml` roll-up, the `all_rules()` registry, acceptance snapshots).
- **In flight:** nothing. Verified on disk: `src/` is still {check, import, lib,
  main, rules, skill} (no contract/extract/engine, no `contracts/`); `main.rs`
  still calls `rules::all_rules()`; `rules.rs` still carries all 10 structs incl.
  the five rejected heuristics; the RETIRE-HEURISTICS retire-list paths all exist;
  `cargo check` clean.
- **This tick:** reconciliation no-op — corpus unchanged since the pivot, every
  `per` cite resolves, `files`/retire-list/code-surface references all faithful,
  so the CONTRACT-MODEL + SKILL-EXTRACTOR → CONTRACT-ENGINE → SKILL-CONTRACT-
  TEMPLATE → CHECK-CUTOVER → RETIRE-HEURISTICS chain stays intact. Inbox empty;
  no forks resolved.
- **Next:** ship the engine chain top-down; then plan the harness-contract/roles
  layer (`10-contracts.md` "Roles and matching" + `verified_by`) and build-order
  step 2 (declared model + dependency graph / blast radius), the latter blocked on
  `(model-declaration-format)` being authored into `30-landscapes.md`.

Plan continues: yes — the artifact-contract engine is filed but wholly unbuilt,
and the roles layer + step-2 declared-model/dependency-graph work remain to plan
once the engine ships and the model-declaration format is authored.
