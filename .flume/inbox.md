<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- 2026-07-02 (human): CHECK-MEMBERS-ALL-KINDS — the memory kinds shipped
  (86d5b70) and the collision UX is verified (bare `[kind.memory]` load-errors
  naming both candidates), but memory members are not clause-checked end to
  end: a 401-line CLAUDE.md fixture under `check --harness` fires NO
  memory.anthropic max_lines advisory (expected: fires; the clause is max=200,
  cited). CHECK-WORKSPACE-KIND-MAP (ef73b49) made check::Workspace a per-kind
  map, but the member-assembly path still hardcodes the skill/rule pair —
  src/read.rs:84-100 builds the graph/check member stream from
  workspace.skills() then workspace.rules() only, and whatever the one-shot
  `check --harness` route threads through inherits that pair. File a slice:
  every embedded kind's discovered members flow through clause/package
  checking and the member stream generically (skill, rule, memory, and the
  next one), on both the workspace and --harness paths. Repro: harness dir
  with a >200-line CLAUDE.md, `check --harness <dir>`, expect one advisory.
