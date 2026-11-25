# 0097 — a tap binary compares as resolved

- **Date:** 2026-11-25 · **Status:** accepted

## Context

The `tap-binary-disagreement` question. 0088 makes the binary each
telemetry declaration's own fact, but tap hooks are deduplicated per
event and matcher. As shipped, two declarations sharing a tap refuse
when their binaries differ, and an undeclared binary next to an explicit
`"temper"` counts as a difference. Defaulting in the SDK would spell the
default twice (0078). Ruled by the session under John's standing
direction.

## Decision

**Declarations sharing a tap compare the binary as the engine resolves it**
(`pipeline.md`, "Telemetry"). The default has one spelling, the
engine's, so an undeclared binary and `"temper"` agree. A real
disagreement refuses at emit and names both declarations. Neither binary
wins.

## Rejected

- **One tap per binary.** It records each event twice.
- **The SDK resolves the default.** That is a second spelling of the
  engine's default.

## Consequences

The disagreement check moves to the engine's resolution of the binary,
and the refusal stays for real disagreements.
