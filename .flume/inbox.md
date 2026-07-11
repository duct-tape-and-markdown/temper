<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->
- Manifest write drops the assembly's residual settings silently: authoring
  three `hook(...)` members plus `harness({ settings: { autoMemoryEnabled,
  permissions, worktree } })` and running emit produced a settings.json
  containing ONLY the hooks segment — every residue key gone, no refusal, no
  finding. Per 0021 the represented manifest regenerates "declared order
  then residue"; the residue half never crosses the seam (assembly.settings
  has no seam family the way registrations now do). Invariant 6: emit must
  carry authored settings or refuse loud, never shed them. Probe: dogfood
  attempt stashed on main ("dogfood: hooks authored as .temper members");
  apply it, `temper emit --dry-run`, compare. observed at fd3d2ec

- The manifest write face's hook shape contradicts the read face and the
  documented schema: emit wrote `hooks.<Event> = {command, matcher, type}`
  (flat object) where code.claude.com/docs/en/hooks documents — and this
  repo's live settings.json uses, and the READ face correctly parses —
  `hooks.<Event> = [{matcher, hooks: [{type, command}]}]` (array of matcher
  groups). read(file) → members → write(members) does not round-trip, and
  Claude Code would silently ignore the emitted flat shape — the exact
  failure class the tool exists to catch. Same probe as above.
  observed at fd3d2ec
