# Plan state

- **Phase:** contract-engine cutover. The `Contract` model, the decidable
  primitive algebra, `engine::validate`, the skill extractor, and the bundled
  Anthropic skill template are all shipped; `(contract-name-field)` is RESOLVED +
  SHIPPED (88246bf — `MissingName` gone, `Contract.name` defaults to the file stem).
- **Last shipped:** CONTRACT-NAME-OPTIONAL — `name` optional/stem-derived,
  `MissingName` removed (88246bf, human chore; drained from inbox this tick).
- **In flight:** nothing. (Working tree carries an uncommitted `M src/main.rs`;
  on-disk `main.rs` still calls `rules::all_rules()` and cites the absent
  `spec/RELEASE-v0.1.md` — both targeted by CHECK-CUTOVER. Plan does not touch `src/`.)
- **Next:** SKILL-CONTRACT-TEMPLATE (gate `open`, pickable now) — load the bundled
  template through `Contract`, assert its surviving clause vector + `skill.anthropic`
  display label; then CHECK-CUTOVER → RETIRE-HEURISTICS in order, after which
  reconcile the then-callerless `check::Rule`/`check::run`.

Plan continues: no — CONTRACT-NAME-OPTIONAL verified shipped on disk (no
`MissingName`, `name` stem-derived) and dropped; inbox drained; chain head
SKILL-CONTRACT-TEMPLATE corrected to the shipped name shape and flipped `open`;
open-questions current. A pickable `open` entry leads the chain — build drains it.
