# Plan state

- **Phase:** contract-engine pivot — replace the heuristic rule registry with the
  generic validator over a declared contract (`10-contracts.md` "Decision: kill
  the heuristic rule registry"). 7 entries queued; none built.
- **Last shipped:** the slice-1 vertical (`import`/`check` CLI, `Skill` IR,
  `author.toml` roll-up, the `all_rules()` registry, acceptance snapshots).
- **In flight:** nothing. Verified on disk this tick: `src/` = {check, import,
  lib, main, rules, skill} — no contract/extract/engine, no `contracts/`;
  `main.rs:57` calls `rules::all_rules()`; `lib.rs:10` still cites the rejected
  `SPEC.md`/`spec/RELEASE-v0.1.md`; `rules.rs` holds all 10 structs incl. the five
  rejected heuristics; every RETIRE-HEURISTICS retire path exists; `cargo check` clean.
- **This tick:** reconciliation no-op — corpus unchanged, all 7 `per` cites
  resolve, `files`/retire-lists/code-surface faithful. Inbox empty; no forks resolved.
- **Next:** ship CONTRACT-MODEL + SKILL-EXTRACTOR → CONTRACT-ENGINE →
  SKILL-CONTRACT-TEMPLATE → CHECK-CUTOVER → RETIRE-HEURISTICS top-down (LIBDOC-EVERGREEN
  ships anytime, disjoint lib.rs region). Then plan the harness-contract/roles layer
  (`10-contracts.md` "Roles and matching" + `verified_by`) and the declared-model/
  dependency-graph work, the latter blocked on `(model-declaration-format)` being
  authored into `30-landscapes.md`.

Plan continues: yes — the entire artifact-contract engine is filed but wholly
unbuilt, and the roles layer + the declared-model/dependency-graph work remain to
plan once the engine ships and the model-declaration format is authored.
