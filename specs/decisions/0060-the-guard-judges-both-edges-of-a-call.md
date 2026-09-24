# 0060 — the guard judges both edges of a call

- **Date:** 2026-09-24 · **Status:** accepted

## Context

The `(post-tool-use-placement)` fork (GH #42 (ii)). `install` wires a
`PostToolUse` hook on `Bash` that no spec section owns: a `build:` commit
added it (cf67f291) to close GH #38's hole, since a shell write carries no
`file_path` and so reaches no guard. The hook runs `check` with the
session-start reporter, which stamps `hookEventName: "SessionStart"`.
Claude Code requires the firing event's name there
(code.claude.com/docs/en/hooks, "JSON output", retrieved 2026-09-24).
Measured this day on Claude Code 2.1.281: every run fails with
`Hook returned incorrect event name: expected 'PostToolUse' but got
'SessionStart'` in the debug log, nothing reaches the model or the user's
UI, and each Bash call still pays a full `check` (0.3 s on this repo). The
placement has never carried a finding. A probe confirmed what does reach
the model: `additionalContext` and a `decision: "block"` `reason`. Ruled by
John 2026-09-24.

## Decision

**The shell hole is the guard's, and the guard closes it.** `temper guard`
runs at both edges of a tool call. After a shell tool it judges what it
binds (projection drift, undeclared documents where `locus-declared` is
bound) against the tree the call left, at the author's declared mode:
`note` defers, `warn` surfaces in-band, `block` refuses the call's result
in-band and names the restore. A pass prints nothing. One command,
`temper guard .`, serves both hooks: the payload's `hook_event_name` tells
the guard which edge it is at.

## Rejected

- **Unwire it; shell writes are CI's.** A shell-written projection stays
  drifted all session, though its writer could restore it with one `emit`.
- **A quiet, findings-only `check` reporter.** That is a second per-call
  surface with its own rules: the whole contract instead of the write
  boundary, no mode, and a tree-scale cost on every `ls`. The per-call
  placement is the guard; a second verb there splits one concept in two.
- **The fork's objection to this ruling**, that `PostToolUse` cannot deny
  and so cannot honour `block`: only one mode is a denial. `warn` and `note`
  carry over unchanged, and `block` degrades to the strongest in-band
  refusal the event offers, which the spec states.

## Consequences

The standing objection: judging the tree after the call cannot tell which
call caused the drift, so drift already present re-fires on every shell call
until it is restored. That repeats a real violation that one `emit`
clears, unlike the pass-time disclosure, which repeated on a clean tree.
`distribution.md` "Per tool call" states both edges. Plan derives the entry:
the guard reads `hook_event_name` and answers in that event's own shape;
install's gate row runs the guard command. An earlier-scaffolded harness
carries the old command, so the upgrade block names the line to change.
