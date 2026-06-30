# Plan state

- **Phase:** contract-engine cutover — `engine::validate` is shipped and the
  bundled skill template is authored; remaining work pins the template, wires
  `check` onto the engine, and retires the dead heuristic registry. **Blocked on
  a newly-surfaced fork** (see below).
- **Last shipped:** `contracts/skill.anthropic.toml` authored as human territory
  (6e0af11); before it LIBDOC-EVERGREEN + DOCS-EVERGREEN scrubbed the stale
  `RELEASE-v0.1` crate-doc cites.
- **In flight:** nothing.
- **Blocker surfaced:** the shipped template carries no top-level `name`, which
  `Contract::parse` requires (`src/contract.rs` → `MissingName`), so it will not
  load — `include_str!` in CHECK-CUTOVER and the SKILL-CONTRACT-TEMPLATE test
  both error at parse. Filed as `(contract-name-field)`; both entries now declare
  `dependsOnForks: ["contract-name-field"]`. Inbox drained (its note —
  reconcile SKILL-CONTRACT-TEMPLATE to test-only since the data file is
  human-authored/embedded-not-written — is applied).
- **Next:** human resolves `(contract-name-field)` — add `name = "skill"` to the
  template (human edit to `contracts/`), or relax `Contract.name` to optional (a
  code entry to file once chosen). Then SKILL-CONTRACT-TEMPLATE → CHECK-CUTOVER →
  RETIRE-HEURISTICS; afterward reconcile the then-callerless `check::Rule`/`check::run`.

Plan continues: no — queue reconciled and the inbox drained; the only actionable
next step is the human `(contract-name-field)` decision, not more plan work.
Nothing is pickable until that fork resolves (the `open` entry is fork-held), so
re-planning would just re-emit this queue.
