# Plan state

- **Phase:** contract-engine pivot — replace the heuristic rule registry with the
  generic validator (`10-contracts.md` "Decision: kill the heuristic rule
  registry"). 7 entries queued; none built.
- **Last shipped:** the slice-1 vertical (`import`/`check` CLI, `Skill` IR,
  `author.toml` roll-up, the `all_rules()` registry, acceptance snapshots).
- **In flight:** nothing. Verified on disk this tick: `src/` is still {check,
  import, lib, main, rules, skill} — no contract/extract/engine, no `contracts/`;
  `main.rs:57` still calls `rules::all_rules()`; `rules.rs` carries all 10 structs
  incl. the five rejected heuristics; every RETIRE-HEURISTICS retire-list path
  exists; `cargo check` clean.
- **This tick:** reconciliation no-op on the 6-entry engine chain — corpus
  unchanged, every `per` cite resolves, `files`/retire-list/code-surface all
  faithful. Filed one new gap: `src/lib.rs`'s crate doc cites the rejected
  `SPEC.md`/`spec/RELEASE-v0.1.md` release line (LIBDOC-EVERGREEN, open, last).
  Inbox empty; no forks resolved.
- **Next:** ship CONTRACT-MODEL + SKILL-EXTRACTOR → CONTRACT-ENGINE →
  SKILL-CONTRACT-TEMPLATE → CHECK-CUTOVER → RETIRE-HEURISTICS top-down; then plan
  the harness-contract/roles layer (`10-contracts.md` "Roles and matching" +
  `verified_by`) and build-order step 2 (declared model + dependency graph),
  the latter blocked on `(model-declaration-format)` being authored into
  `30-landscapes.md`.

Plan continues: yes — the entire artifact-contract engine is filed but wholly
unbuilt, and the roles layer + the declared-model/dependency-graph work remain to
plan once the engine ships and the model-declaration format is authored.
