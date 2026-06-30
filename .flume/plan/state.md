# Plan state

- **Phase:** contract-engine cutover. The `Contract` model, the decidable
  primitive algebra, `engine::validate`, and the bundled skill template are all
  shipped; the `(contract-name-field)` fork is RESOLVED. The four-entry chain to
  relax the model, pin the template, wire `check` onto the engine, and retire the
  heuristic registry is queued and unblocked.
- **Last shipped:** plan reconcile verifying the engine-cutover chain against disk
  and refreshing state (3f6a8f2).
- **In flight:** nothing.
- **Next:** CONTRACT-NAME-OPTIONAL (gate `open`, pickable now) — relax
  `Contract.name` to `Option<String>`, drop `MissingName`, derive a display label
  from the file stem; then SKILL-CONTRACT-TEMPLATE → CHECK-CUTOVER →
  RETIRE-HEURISTICS in order, after which reconcile the then-callerless
  `check::Rule`/`check::run`.

Plan continues: no — queue re-verified against disk this tick (every cited site
unchanged: contract.rs name/MissingName/missing_name test, engine.rs construction
sites, main.rs/lib.rs heuristic wiring, the template + extractor + check surface),
inbox empty, open-questions current, and a pickable `open` entry leads the chain.
Building is how the queue drains.
