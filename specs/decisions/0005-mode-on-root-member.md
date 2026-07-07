# 0005 — enforcement mode is a root-member field

- **Date:** 2026-07-07 · **Status:** accepted

## Context

Enforcement mode is author-declared per placement, but the model named no
home for the declaration: the SDK emits a hardcoded authority fact the
corpus never coined, and the root member carried no field to source a mode
from.

## Decision

The enforcement mode is a field on the root member — harness-wide
declarations are root-member fields — overridable per member. Paths follow
members; policy is never addressed by path.

## Rejected

A settings-residual field (the residue is for genuinely unschematized keys,
never for model concepts). Per-path overrides (members are the unit; a path
is where a member serializes, not an addressing scheme for policy). A
per-projection mode on emit-owned members only (the mode also governs
placements with no projection, e.g. the terminal gate).

## Consequences

The hardcoded authority emit retires; the root member kind gains the mode
field; `temper guard` reads its declared mode from the lock. Nothing gates
on it today — low-stakes until guard blocks per mode.
