# Plan state

- **Phase:** contract-engine cutover. The generic `Contract` + decidable engine
  (`src/contract.rs`, `src/engine.rs`), the skill extractor
  (`extract::skill_features`, extract.rs:94), and the bundled Anthropic skill
  template (`contracts/skill.anthropic.toml`) are all shipped on disk; the
  rejected heuristic registry is still wired into `check` (main.rs:57) awaiting
  the two-entry cutover chain.
- **Last shipped:** SKILL-CONTRACT-TEMPLATE (14dee1c) — bundled template loads
  through `Contract::parse` (contract.rs:300) with its clause vector asserted.
- **In flight:** nothing committed. Working tree carries only a cosmetic import
  reorder in `src/main.rs` (fmt noise). `check` still calls `rules::all_rules()`;
  main.rs/cli.rs cite the absent `spec/RELEASE-v0.1.md` (main.rs:4/20, cli.rs:1) —
  CHECK-CUTOVER targets; acceptance.rs:1 cites it too, a RETIRE-HEURISTICS target.
  Plan never edits `src/`.
- **Next:** CHECK-CUTOVER (gate `open`, pickable now) → RETIRE-HEURISTICS
  (blockedBy CHECK-CUTOVER); after the registry is gone, reconcile the
  then-callerless `check::Rule`/`check::run`.

Plan continues: no — both entries re-verified against disk (cited APIs
`skill_features`, `validate`, `render`/`any_error`, `Contract::parse` and
`contracts/skill.anthropic.toml` all present; `check` on `rules::all_rules()` at
main.rs:57; every retire target present). Inbox empty, open-questions current.
The queue is unchanged and a pickable `open` entry leads the chain — build drains
it.
