# Plan state

- **Phase:** contract-engine cutover — `engine::validate` and the bundled skill
  template are shipped; the `(contract-name-field)` fork is RESOLVED, so the
  chain to wire `check` onto the engine and retire the heuristic registry is now
  fully unblocked.
- **Last shipped:** spec resolution of `(contract-name-field)` — a contract is
  identified by path/role, not an internal name (eea1054); before it the curated
  `contracts/skill.anthropic.toml` was authored as human territory (6e0af11).
- **In flight:** nothing.
- **Next:** CONTRACT-NAME-OPTIONAL (gate `open`, pickable now) relaxes
  `Contract.name` to `Option<String>` and drops `MissingName`, after which
  SKILL-CONTRACT-TEMPLATE → CHECK-CUTOVER → RETIRE-HEURISTICS run in order;
  afterward reconcile the then-callerless `check::Rule`/`check::run`.

Plan continues: no — queue reconciled, inbox drained, and a pickable `open`
entry (CONTRACT-NAME-OPTIONAL) leads the chain. Building is how the queue drains;
re-planning would just re-emit this queue.
