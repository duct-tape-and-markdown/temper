<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- **MANIFEST-GATE-READ self-gate revert — diagnosis (interactive session,
  07-03):** build commit `8d3ef1a` was cherry-picked onto main and reset away
  by the afterMerge gate — the same invisible-revert shape as
  LOCK-FRESHNESS-FACTS. Treat the entry as **attempted and reverted**, not
  unbuilt. Reproduced the red from the reverted commit's own binary: the
  committed `temper.toml` carries **zero `[[member]]` tables** (the dogfood
  manifest predates MANIFEST-EMIT; no `emit` has rerun since), so the
  manifest-read corpus is empty — `coverage.checked` reports 0 members across
  4 built-in kinds, the built-in `rule` kind drops off the roster, and both
  required requirements fail twice over (`requirement.admissibility` +
  `requirement.unfilled`, 4 errors). The lock likewise still carries only
  legacy `import_hash` keys, no `emit_hash`. **Recommended in-fence fix (the
  LOCK-FRESHNESS-FACTS pattern, shipped fa8067d):** a manifest with no
  `[[member]]` tables is a legacy floor manifest — fall back to synthesizing
  the corpus via `import::scratch_manifest` (machinery already present in the
  reverted commit for `--harness`/surfaceless) instead of reading an empty
  corpus. The durable half — regenerating the committed `temper.toml` + lock
  via `emit` — is **outside build's fence** (`temper.toml` is not in
  `BUILD_WRITABLE_PATHS`); route it to the interactive session/human after
  the entry ships, or fold it into INIT-ONRAMP's scope.
