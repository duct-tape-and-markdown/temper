# Plan state

- **Phase:** contract-engine cutover. The `Contract` model, decidable primitive
  algebra, `engine::validate`, the skill extractor, and the bundled Anthropic
  skill template are all shipped; the heuristic registry is still wired into
  `check` and awaits CHECK-CUTOVER + RETIRE-HEURISTICS.
- **Last shipped:** SKILL-CONTRACT-TEMPLATE (14dee1c + 5793d44) — the bundled
  `contracts/skill.anthropic.toml` loads through `Contract` with its surviving
  clause vector asserted (`tests/contract_template.rs`).
- **In flight:** nothing committed. Working tree carries only a cosmetic import
  reorder in `src/main.rs` (fmt noise, not a build tick); on disk `check` still
  calls `rules::all_rules()` and main.rs/tests cite the absent
  `spec/RELEASE-v0.1.md` — all CHECK-CUTOVER targets. Plan does not touch `src/`.
- **Next:** CHECK-CUTOVER (gate `open`, pickable now) → RETIRE-HEURISTICS
  (blockedBy CHECK-CUTOVER); after the registry is gone, reconcile the
  then-callerless `check::Rule`/`check::run`.

Plan continues: no — reconciled against disk: both queue entries re-verified
accurate (cited APIs `extract::skill_features`, `engine::validate`,
`check::render`/`any_error`, `Contract::parse`, and `contracts/skill.anthropic.toml`
all present; every retire target present), inbox empty, open-questions current.
A pickable `open` entry leads the chain — build drains it.
