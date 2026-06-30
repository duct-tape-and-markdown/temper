# Plan state

- **Phase:** contract-engine pivot — replacing the heuristic rule registry with the generic validator (`30-landscapes.md` build order, step 1). 6 entries queued; pending was empty.
- **Last shipped:** the slice-1 vertical (`import`/`check` CLI, `Skill` IR, `author.toml` roll-up, acceptance) — verified on disk: `cargo test` green, `clippy -D warnings` clean.
- **In flight:** nothing.
- **This tick:** reconciled `src/` against the rewritten evergreen corpus (commits 2240636, f9dd05c). The corpus now *rejects* `src/rules.rs`'s `all_rules()` registry (`10-contracts.md` "Decision: kill the heuristic rule registry") — the stale state calling slice-1 "complete, nothing plannable" was wrong. Filed the registry→engine replacement as a disjoint chain: CONTRACT-MODEL + SKILL-EXTRACTOR → CONTRACT-ENGINE → SKILL-CONTRACT-TEMPLATE → CHECK-CUTOVER → RETIRE-HEURISTICS. Decidable members (name format/length/deny, forbidden_keys, name-matches-dir, body max_lines) survive as template clauses; the five heuristics (trigger, anti-trigger, third-person, prose-grep companion-refs, refs-depth) are dropped per the Decision. Three new forks opened: (regex-crate), (contract-selection), (skill-ref-syntax). Inbox empty.
- **Next:** after the engine lands — the harness-contract layer (`10-contracts.md` "Roles and matching" + `verified_by`), then `30-landscapes.md` build-order step 2 (declared model + dependency graph / blast radius). Step 2's model-declaration *format* is unsettled and must be authored before it can be filed.

Plan continues: yes — only the artifact-contract engine (step 1) is filed; the harness-contract/roles layer and the step-2 declared-model + dependency-graph work remain to plan once the engine ships and the model format is settled.
