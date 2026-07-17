# 0037 — the verifier is typed: telemetry rides the config surface

- **Date:** 2026-07-17 · **Status:** accepted

## Context

The `(eval-capability)` fork, parked strategic since the era began:
`Requirement.verifiedBy` dormant in the model, the behavioral remainder
delegated whole. Two things moved. The 07-16 field campaign measured the
cost side — the herald and gate-semantics facts that ruled 0028 were
unknowable from structure and took hand-built headless probes. And a
design sitting (07-17) verified the vendor's hook surface meets the
capability more than halfway (code.claude.com/docs/en/hooks, retrieved
2026-07-17): `InstructionsLoaded` fires on memory and rule loads with the
load reason as its matcher — `path_glob_match` is a rule's gate firing,
natively; skills execute as `tool_name: "Skill"` with the skill name in
`tool_input`, and `UserPromptExpansion` matches command names, so the two
invocation channels are separately observable; `PostToolUse` runs
`async`, every payload carries `session_id`, and the engine binary spawns
in ~2ms measured — a session's full event stream costs under a second,
off the critical path. The harness maintainer's unanswerable questions —
dead weight, trigger quality, placement audit — are exactly what field
counts with denominators answer. Session-argued, human-ruled 2026-07-17
("facilitate").

## Decision

The requirement's verifier edge becomes the **typed verifier**, three
species: **script** (today's path-resolved delegate — CI executes the
judgment), **telemetry** (named, documented harness events; the emitted
tap records them to a local-locus log; judgment is reading the field
record), and **probe** (a fixture harness plus a headless session, judged
over the run's own record). Telemetry lands first because it rides
documented vendor surface; the probe stays a documented pattern — the
0028 five-cell probe is its reference implementation — until the
transcript surface is documented or a second consumer types it.

Mechanics as ruled: the tap is dumb (append event identity plus minimal
discriminant, exit silent) and the reader is smart (joins to members
through the lock at read time). Storage is the local locus (0032/0034
semantics verbatim). The privacy bound is the tap's contract: no tool
output, no prompt text, no message bodies, ever. Verdict surfaces are
read verbs — `explain`'s field strand and reports — and no verifier's
result enters `check`'s exit code. Eval selection rides the impact
strand: a member edit names the requirements whose verifiers went stale,
uniformly across all three species.

## Rejected

- **A transcript kind first** (the sitting's own opening design): types an
  undocumented internal format — the weakest citation tier — where hooks
  are documented surface. Kept as the probe species' eventual encoding,
  demand-gated.
- **A runner or aggregator in temper**: spawning sessions, pass rates,
  model spend — the second product, and the honest bound delegates
  execution. temper projects fixtures and judges records; it never runs.
- **A bare `eval:` flag on kinds**: says "measure" without naming what or
  bounding claims — hooks see tool events, so telemetry may claim
  "invoked", "loaded", "reached", never "followed" or "helped"; the
  declaration names its events so the claims-bound is explicit, and the
  corpus binds the instrument, never the shipped kind unasked (spine
  rule).
- **Verdicts in the gate**: field counts are probabilistic evidence;
  invariant 2's cries-wolf line holds them to read verbs.
- **An MCP transport for the eval verbs**: the parked delivery posture
  stands (`docs/horizons.md`); the skill-taught CLI keeps one vocabulary
  in one mouth.

## Consequences

`contract.md`'s requirement bullet respells the verifier typed, its
admissibility line and boundary sentence follow, and "Read verbs" gains
the field strand; `pipeline.md` gains the Telemetry section — same
commit, this record. Encode-time verification owed per clause
(`builtins.md` discipline): `InstructionsLoaded`'s exact payload fields
and same-file re-fire behavior, `UserPromptExpansion`'s payload shape,
the `Skill` tool_input schema — each cite lands with the code that
consumes it, re-verified against live docs. Plan derives the entries: the
tap verb, the telemetry declaration and its hook projection, the
local-locus log kind, the field strand, verifier-type resolution at
check. The `(eval-capability)` fork record deletes with this record's
commit; the ledger's behavioral-horizon carry resolves — the horizon
graduated straight to intent, so `docs/horizons.md` gains no entry.
