<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- `CONTRACT-NAME-OPTIONAL` was hand-applied (human chore) after the loop livelocked
  on it: `Contract.name` now derives from the file stem when absent, `MissingName`
  is removed, the curated template loads. Verified green (build/clippy/55 tests +
  template-load check). Action for plan: DROP `CONTRACT-NAME-OPTIONAL` from pending
  (shipped, verify on disk: `src/contract.rs` has no `MissingName`), and unblock
  the chain — SKILL-CONTRACT-TEMPLATE is now buildable, then CHECK-CUTOVER, then
  RETIRE-HEURISTICS.
