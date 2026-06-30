# Plan state

- **Phase:** contract-engine cutover. The `Contract` model, the decidable
  primitive algebra, `engine::validate`, the skill extractor, and the bundled
  Anthropic skill template are all shipped; `(contract-name-field)` is RESOLVED.
- **Last shipped:** plan reconcile refreshing the stale last-shipped pointer and
  handing the engine-cutover chain to build (ea01913).
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
unchanged), inbox empty, open-questions current, and a pickable `open` entry leads
the chain. Reconciliation produced no queue change; re-planning it again is the
failure mode. Build drains the queue.
