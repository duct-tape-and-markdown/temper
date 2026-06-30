# Plan state

- **Phase:** contract-engine cutover. The `Contract` model, the decidable
  primitive algebra, `engine::validate`, and the bundled skill template are all
  shipped; the `(contract-name-field)` fork is RESOLVED. The four-entry chain to
  relax the model, pin the template, wire `check` onto the engine, and retire the
  heuristic registry is queued and unblocked.
- **Last shipped:** plan reconcile filing CONTRACT-NAME-OPTIONAL and unblocking
  the engine-cutover chain (e7aeb1b); before it, the spec resolution of
  `(contract-name-field)` — identity is path/role, not an internal name (eea1054).
- **In flight:** nothing.
- **Next:** CONTRACT-NAME-OPTIONAL (gate `open`, pickable now) — relax
  `Contract.name` to `Option<String>`, drop `MissingName`, derive a display label
  from the file stem; then SKILL-CONTRACT-TEMPLATE → CHECK-CUTOVER →
  RETIRE-HEURISTICS in order, after which reconcile the then-callerless
  `check::Rule`/`check::run`.

Plan continues: no — queue reconciled and verified against disk (every cited site
unchanged since e7aeb1b: contract.rs name/MissingName, engine.rs construction,
main.rs/lib.rs heuristic wiring), inbox empty, open-questions current, and a
pickable `open` entry leads the chain. Building is how the queue drains.
