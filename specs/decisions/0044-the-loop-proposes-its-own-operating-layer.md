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
- **The constitution is out of scope.** Amendments may target the
  derived operating layer: `.flume/prompts/**`, `.flume/PROTOCOL.md`'s
  operational sections, the loop-discipline rules under `.temper/`,
  capture READMEs. They may never target intent (`specs/intent.md`,
  `specs/model/**`, `specs/decisions/**`), the chain's gates, or the
  non-negotiables — the thing doing the settling does not rewrite what
  it settles against.
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
