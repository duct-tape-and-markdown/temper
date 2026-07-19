# 0044 — The loop proposes its own operating layer

**Status:** ratified 2026-07-18 (John)

## Context

The loop's operating layer — phase prompts, discipline rules, capture
formats — is human territory. When it hurts, an autonomous phase files
friction prose and waits for a human to author the fix. That seam made
sense when the loop was young; by now the loop's own record (metrics,
reverts, retries) is often the best evidence for how its harness
should change, and the humans are a transcription step: the phase
describes the fix, the session writes it.

The pin this decision operationalizes: our job in the harness is to
name the invariants, and let the loop settle. The loop's configuration
is itself a settling surface.

## Decision

- **A third capture channel, `.flume/amendments/`**: an autonomous
  phase that can ground a harness change in a named invariant and an
  observed cost files a capture carrying a **concrete diff**, not a
  description. Humans ratify with one word — apply verbatim, or
  decline with the reason in the deleting commit. Ratification is
  review, never authorship.
- **Two amendment classes, one channel** (amended same day — the
  first cut fenced all of `specs/**` and over-shot):
  - *Operating-layer amendments* target the loop's derived plumbing:
    `.flume/prompts/**`, `.flume/PROTOCOL.md`'s operational sections,
    the loop-discipline rules under `.temper/`, capture READMEs.
    Ratification is review — apply verbatim or decline.
  - *Phrase advocacy* targets the process corpus, `specs/process/**`:
    the posture wordings and declared choices the loop tests against
    reality every tick. Merge, split, widen, narrow — grounded in the
    invariant's intent and the record. Ratification here is the
    design argument, not a stamp: the session attacks the proposal
    before encoding (the collaboration rule), and a contested cut
    becomes a Decision like any other.
- **The constitution is out of scope for both.** `specs/intent.md`,
  `specs/model/**`, `specs/decisions/**`, the chain's gates, and the
  non-negotiables — the thing doing the settling does not rewrite
  what it settles against; it argues phrasing, never intent.
- **Staged authority.** This decision grants propose-and-ratify only.
  Auto-landing amendments (apply on file, human revert window) is a
  future amendment to this decision, contingent on the record showing
  ratified proposals are consistently applied unchanged.

## Consequences

- `.flume/amendments/` joins friction and refactor as the third slit
  in the control-plane fence (chain writable paths, both phases).
- The `capture-friction` skill and `.flume/PROTOCOL.md` name the third
  channel; the amendment capture format lives in the directory README.
- Session-open sweep extends to amendments: ratify or decline, then
  delete — git is the archive.

## Rejected alternatives

- **Keep friction-prose only** — the human transcription step adds
  latency and drops fidelity; the phase that paid the cost writes the
  sharpest diff.
- **Auto-land immediately** — a prompt that edits the thing that
  generates it compounds drift fast; the ratify stage is where the
  loop's taste earns the wider grant.
- **Widen refactor captures instead** — refactor targets product
  structure and drains to plan; amendments target the harness and
  drain to humans. Different audience, different bar, different fence.
