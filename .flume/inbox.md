<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- GOVERNANCE NOTE (interactive session, 2026-07-03): LOCK-FRESHNESS-FACTS
  already failed the afterMerge self-gate once — cherry-pick 005199f was
  reverted by the dispatcher ("afterMerge gate 'temper check (self)' failed";
  the loop log carries it, the tree does not, which is why the last reconcile
  read "nothing shipped"). Diagnosis: the key rename makes temper's own
  committed `.temper/lock.toml` (old shape: `import_hash`/`last_applied`)
  unreadable or stale to the freshly built binary, and `.temper/**` is outside
  build's fence, so the entry cannot regenerate the dogfood lock in its own
  commit. If the current retry reverts the same way, re-file the entry to
  carry an explicit LEGACY-READ MIGRATION: the lock is a *generated* file
  (never hand-composed), so reading the legacy keys as the pre-migration
  baseline and writing only the new shape is honest self-migration of tool
  state — self-contained in `src/`, no fence change, and the next lock write
  upgrades the file on disk. Separately: a gate-revert leaving no trace plan
  reads is a spin hazard (build can retry an unchanged doomed entry
  indefinitely) — worth recording revert reasons in a place the next plan
  tick reads, as accepted debt or a state.md line.
