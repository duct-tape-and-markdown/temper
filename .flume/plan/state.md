# Plan state

- **Phase:** contract-engine cutover. The `Contract` model, decidable primitive
  algebra, `engine::validate`, the skill extractor, and the bundled Anthropic
  skill template are all shipped; the heuristic registry is still wired into
  `check` and awaits the cutover + retire.
- **Last shipped:** SKILL-CONTRACT-TEMPLATE (14dee1c + 5793d44) — the bundled
  `contracts/skill.anthropic.toml` loads through `Contract` with its surviving
  clause vector asserted (`tests/contract_template.rs`).
- **In flight:** nothing. Working tree clean. On-disk `src/main.rs` still calls
  `rules::all_rules()` and cites the absent `spec/RELEASE-v0.1.md` — both
  targeted by CHECK-CUTOVER; plan does not touch `src/`.
- **Next:** CHECK-CUTOVER (gate flipped `open`, pickable now — its
  SKILL-CONTRACT-TEMPLATE blocker shipped) → RETIRE-HEURISTICS (blockedBy
  CHECK-CUTOVER); after the registry is gone, reconcile the then-callerless
  `check::Rule`/`check::run`.

Plan continues: no — reconciled against disk: SKILL-CONTRACT-TEMPLATE verified
shipped (template + `tests/contract_template.rs` present) and its dependent
CHECK-CUTOVER unblocked to `open`; both queue entries re-verified accurate
against `src/` (cited API present, retire targets present). Inbox empty,
open-questions current. A pickable `open` entry leads the chain — build drains it.
