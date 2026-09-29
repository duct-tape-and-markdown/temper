<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->








- observed at 3e808020 — **A render hook that reads a deferred edge's facts gets a
  raw TypeError, not a named refusal.** EMBEDDED-EDGE-TARGET-DEFERS-TO-CHECK
  (25890939) shipped the ruled deferral: an unresolved at-locus address carries
  no facts, so `targets[field]` is absent. A kind's `render` that spells
  `targets[field].path` then throws `TypeError: Cannot read properties of
  undefined` at the emit boundary — loud, but unnamed, and pointing at the
  author's template rather than at the reference (intent.md, loud or nothing).
  Ruling (John 2026-09-29, on the session's recommendation; plan's objection in
  a84a7de0 is what raised it): **reading a deferred edge's facts is a named emit
  refusal.** It is not the rejected render-selection rule — a render that never
  reads the field still emits and defers to `check`, as ruled; only a read of
  facts that do not exist refuses, naming the member, the field, the authored
  address, and that its target is judged at `check`. The seam is already there:
  `recordingView` (sdk/src/emit.ts) wraps `targets` in a Proxy for placement;
  the view handed to `render` answers a get on a deferred field with the
  refusal instead of `undefined`. A partial fact (name/address/kind from the
  address, no path) stays rejected — it reopens the closed fact set. Test: a
  render reading `targets.<deferred>.path` refuses by name; one that ignores the
  field emits clean. Pin that `format-places-edges` still omits a deferred field.
- observed at 3b397826 — **The engine stamp has no example exemption.** The
  LOCK-NAMES-THE-ENGINE-THAT-WROTE-IT capture (3b397826,
  `.flume/refactor/build-lock-engine-stamp-fence.md`) proposes leaving the
  shipped example's committed lock naming no engine, to avoid churn on each
  release. Session stance, 2026-09-29: no. 0069 says the lock names the engine
  that wrote it; an adopter's lock takes that same diff on every upgrade, and
  the example is the adopter shape we ship — exempting it is a per-instance
  branch on shared behavior (`engineering.md`, "The fix lands at the
  mechanism") and hides exactly the churn adopters will see. The example lock
  re-emits with its engine line like any other; a release bump regenerates it.
  The re-scope adds `tests/it/builtin_lock_frozen.rs` (slice the derived side
  from `[[declaration.kind]]` as the capture says) to the entry's files.
