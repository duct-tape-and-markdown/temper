<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- **Install's note check is presence-based, not content-based** (interactive
  session, 07-03, post-INSTALL-DRIFT-STRINGS): after the reworded strings
  shipped, `temper install` re-placed the guard hook but reported the
  managed-by note "unchanged" while the placed copy still carried the
  retired `re-add` text — a stale note body never refreshes and
  `gate-installed` never flags it. Small entry: the note's drift state
  should compare expected bytes (the modeline check already does), so a
  string rework re-places on the next install. Interim: the interactive
  session hand-cycled the stale note (delete line, re-run install).

