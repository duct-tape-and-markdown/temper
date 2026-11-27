# 0119 — a pattern passes `satisfies` to the member that fills

- **Date:** 2026-11-27 · **Status:** accepted

## Context

BASE-HARNESS-HYDRATES-A-PLAYBOOK added `PlaybookInit.satisfies?: { rule?, skill? }` so the example's requirements fill. The spec named no `satisfies` on a pattern, and `gateHooks()` takes none. Plan raised the gap as a question.

## Decision

**A pattern takes `satisfies` keyed by the surface it emits** (`distribution.md`, the SDK). Examples are a playbook's rule or skill, or a gate hook by its event. The pattern passes each entry to the member that surface becomes. A requirement is filled by members, and a pattern is a constructor of members, so this keying follows from the requirement model.

## Rejected

- **A flat list binding every surface.** A requirement that declares `kind: skill` rightly refuses the playbook's rule.
- **Filling requirements only outside the pattern.** The author would have to reach into a pattern's members, which the pattern exists to hide.

## Consequences

The playbook's per-surface `satisfies` is ratified. `gateHooks()` gains the same keying by event.
