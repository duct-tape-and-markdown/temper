<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- Dogfood re-conversion finding (2026-07-07): `install --yes` places the
  managed-by note AFTER writing the lock, so the note-bearing projection
  immediately fails `check`'s fingerprint compare ("does not match the
  lock's emit fingerprint") while `emit --dry-run` reports unchanged (the
  renderer includes the note) — the two verbs disagree on a fresh install's
  own output until a second `emit` re-anchors. Fix shape: fingerprint after
  placements (or place notes before the lock write), so install's first
  output is self-consistent.
