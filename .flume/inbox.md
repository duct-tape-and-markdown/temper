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
- **The display rule is the rung-3 pilot's blocker** (interactive session,
  07-03): the genre package is landed (5f56fda) and the verbs disclose
  correctly, but the ratified corpus text now carries "a genre value is
  rendered by one corpus-wide display rule per genre — emit-owned"
  (20-surface, the law-5 append) with no code behind it — emit has no
  projection face for custom kinds at all, so a fenced Decision would sit
  in the specs/ projection as raw TOML, failing the migration protocol's
  acceptance test (residual = connective tissue only). The corpus's rung-3
  pilot (staged human ceremony) waits on it. Plan should weigh whether the
  display rule is a standalone entry or rides the spec-kind emit face /
  module-carriage work — don't force it if it's entangled; the pilot is
  patient.

