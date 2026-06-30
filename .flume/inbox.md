<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- `(contract-name-field)` is RESOLVED (option B) — see open-questions.md and the
  new Decision in `specs/10-contracts.md`. Action for plan: (1) file a code entry
  `CONTRACT-NAME-OPTIONAL` (gate `open`) — relax `Contract.name` to `Option<String>`
  in `src/contract.rs`, drop the `MissingName` error path, derive a display label
  from the file stem; update its unit tests. (2) Clear `dependsOnForks:
  ["contract-name-field"]` from SKILL-CONTRACT-TEMPLATE and CHECK-CUTOVER, and set
  CHECK-CUTOVER `blockedBy` the new entry as needed so the order is
  CONTRACT-NAME-OPTIONAL → SKILL-CONTRACT-TEMPLATE → CHECK-CUTOVER → RETIRE-HEURISTICS.
