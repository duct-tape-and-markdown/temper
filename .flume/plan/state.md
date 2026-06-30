# Plan state

- **Phase:** contract-engine pivot — replacing the heuristic rule registry with
  the generic validator (`10-contracts.md` "Decision: kill the heuristic rule
  registry"; `30-landscapes.md` build-order step 1). 6 entries queued.
- **Last shipped:** the slice-1 vertical (`import`/`check` CLI, `Skill` IR,
  `author.toml` roll-up, acceptance snapshots). Verified on disk this tick:
  `src/` is still {check, import, lib, main, rules, skill}; no contract/extract/
  engine module, no `contracts/`; `main.rs` still calls `rules::all_rules()`;
  `cargo check` clean.
- **In flight:** nothing. The chain filed last tick (`c7458b8`) is wholly
  unbuilt — no `build:` commit has landed against it — so this tick is a
  reconciliation no-op on pending: every entry's `per` cite still resolves and
  stays faithful to an unchanged corpus.
- **This tick:** confirmed nothing shipped and the spec is unchanged → kept the
  CONTRACT-MODEL + SKILL-EXTRACTOR → CONTRACT-ENGINE → SKILL-CONTRACT-TEMPLATE →
  CHECK-CUTOVER → RETIRE-HEURISTICS chain intact. Inbox empty. Opened one fork —
  `(model-declaration-format)` — because step-2's declared-model *format* is
  unauthored intent that gates the dependency-graph work.
- **Next:** once the engine chain ships — the harness-contract/roles layer
  (`10-contracts.md` "Roles and matching" + `verified_by`), then build-order
  step 2 (declared model + dependency graph / blast radius), blocked on
  `(model-declaration-format)` being authored into the spec.

Plan continues: yes — step-1 (the artifact-contract engine) is filed but wholly
unbuilt; the harness-contract/roles layer and the step-2 declared-model +
dependency-graph work remain to plan once the engine ships and the model format
is authored.
