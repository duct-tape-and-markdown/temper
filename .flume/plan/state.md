# Plan state

- **Phase:** contract-engine cutover. The `Contract` model, the decidable
  primitive algebra, `engine::validate`, the skill extractor, and the bundled
  Anthropic skill template are all shipped; `(contract-name-field)` is RESOLVED.
  The four-entry chain to relax the model, pin the template, wire `check` onto the
  engine, and retire the heuristic registry is queued and unblocked.
- **Last shipped:** plan reconcile re-verifying the engine-cutover chain against
  disk and refreshing state (b46aa33).
- **In flight:** nothing. (Working tree carries an uncommitted `M src/main.rs`;
  on-disk `main.rs` still calls `rules::all_rules()` and cites the absent
  `spec/RELEASE-v0.1.md` — both targeted by CHECK-CUTOVER. Plan does not touch
  `src/`.)
- **Next:** CONTRACT-NAME-OPTIONAL (gate `open`, pickable now) — relax
  `Contract.name` to `Option<String>`, drop `MissingName`, derive a display label
  from the file stem; then SKILL-CONTRACT-TEMPLATE → CHECK-CUTOVER →
  RETIRE-HEURISTICS in order, after which reconcile the then-callerless
  `check::Rule`/`check::run`.

Plan continues: no — queue re-verified against disk (every cited site present and
unchanged: contract.rs name/MissingName/missing_name test, engine.rs construction
sites, lib.rs/main.rs heuristic wiring, the shipped template + extractor + check
surface), inbox empty, open-questions current, and a pickable `open` entry leads
the chain. Building is how the queue drains.
