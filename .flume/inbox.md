<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->
- `(tap-log-format)` ruled 07-17 (interactive, John): option (c)
  sharpened — the log is the engine's own record (the lock / emit-payload
  category), never a member, no declared format; the format vocabulary
  stays three. `pipeline.md` "Telemetry" amended and 0037 amended in
  place (5b6b6f2). Slice [3] rescopes from "local-locus log kind" to
  the record's versioned shape in engine code — JSONL, O_APPEND
  single-line records, a per-line shape version, the reader tolerating
  unknown lines out loud (the 0024 posture by analogy). Slices [1] (tap
  verb) and [4] (field strand) unblock unchanged: the strand's member
  join rides the lock; only the parse is bespoke. observed at 5b6b6f2
