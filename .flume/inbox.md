<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- **Stale `re-add` strings in install's placements** (interactive session,
  07-03, post-wave): the managed-by note and guard-hook message `install`
  places both end "(temper re-add lifts a direct edit back)" — READD-RETIRE
  retired that verb this wave. The strings live in `src/install.rs` (curated
  placement text, in-fence). One small entry: reword to the ratified drift
  model ("a direct edit is drift routed to the authored source — edit the
  `.temper/` surface and re-run `temper emit`"). Re-placing updated strings
  is idempotent under the three-state drift engine, but note the placed
  copies in `.claude/` are outside build's fence — the interactive session
  will re-run `temper install` after the entry ships.
