<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->








- observed at eea2e134 (friction drain: this repo's PostToolUse hook ran the
  retired command after 5820ad7b, and `check` said nothing) — an author who
  claims a gate event with a `hook` member and spells temper's own command
  by hand has no gate on that copy. `gate_outcome` answers
  `SupersededByMember` and `gate_installed` skips it, which is right for a
  deliberate author command but silent for a stale spelling of temper's
  own. Two shapes: the SDK exports the gate commands (`SESSION_START_COMMAND`,
  `GUARD_COMMAND`) the way it already exports `TAP_COMMAND`, so a member
  imports rather than copies them; or `check` notes a superseding member
  whose command matches a retired spelling of a temper gate command. The
  first removes the class; the second only catches it after a release.
