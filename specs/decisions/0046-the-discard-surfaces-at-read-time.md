# 0046 — the discard surfaces at read time

- **Date:** 2026-07-26 · **Status:** accepted

## Context

The tap is advisory and always exits zero — including on a failed append,
whose stderr line lands where no one reads (a hook's stderr is effectively
invisible). The field report that drove TAP-WORKTREE-GIT-AWARE
(centercode-platform, 2026-07-25) raised the meta-question: an always-zero-
exit subsystem has no channel to report that its own output is being
discarded. Investigation found two silence layers, not one: the write-time
silence the report named, and a read-time silence it didn't — the field
strand narrates nothing on an absent or empty log, even when the lock
declares the tap wired, and a telemetry verifier is admissibility-checked
but never judged, so no reader anywhere states the gap. Both worktree
defects in this arc were found by a human comparing logs across checkouts
by hand.

## Decision

**The writer stays dumb; the reader narrates absence against declaration.**
The tap's exit-zero totality is correct and keeps: a writer that cannot
write cannot durably report either, so the systemic detector lives on the
read side. Pipeline.md's tolerate-out-loud clause extends to the absence
case: a reader meeting no records where the lock declares tap registrations
states that absence — the declared wiring against the empty log,
expectation and observation, never a verdict and never a gate. The one
narrating surface is `explain`, the log's reader of record. A healthy
fresh clone and a broken tap read identically there, on purpose: the
narration is true and useful in both worlds, and inferring which world is
the reader's job, not the engine's.

## Rejected

- **A writer-side health channel** (marker file, warn-once state): a second
  machine-written surface with its own discard problem — the recursion
  never bottoms out, and its state lives in the same failure domain as the
  log it reports on.
- **A non-zero exit on failed append**: gates the tool call the tap exists
  only to observe — the spec's advisory-never-a-gate line, crossed.
- **Freshness heuristics** (log mtime against lock, "this session should
  have records by now"): a verdict dressed as evidence, racy at session
  open, and wrong the first time a machine idles.
- **A line in check's session-start reporter**: advisory text inside the
  gate surface muddies green-means-the-contract-holds. Revisit on a
  field-reported discard that sat unnoticed because nobody ran `explain`.

## Consequences

`pipeline.md` "Telemetry" carries the absence clause. The engine's field
narration states the absence when the lock carries tap registrations and
the readout is empty — today it returns the empty string. Plan derives the
entry. The fork `(advisory-report-channel)` resolves here.
