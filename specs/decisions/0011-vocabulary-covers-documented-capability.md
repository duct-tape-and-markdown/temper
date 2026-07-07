# 0011 — the vocabulary covers documented surface capability

- **Date:** 2026-07-07 · **Status:** accepted

## Context

The remaining Claude Code surfaces could not be modeled or gated for want
of vocabulary: slash commands had no registration value for user-invoked,
hooks and settings had no kinds, and a decidable "unclaimed entry in
`.claude/`" advisory could not ship (field report 2026-07-07 — a bogus
`.clauignore` sails through).

## Decision

The coverage bar is the surface's own documentation: every capability a
built-in surface documents as real — a user-invoked command, an event hook,
a connection — gets its registration value and, where it is an artifact,
its kind. The vocabulary grows by documented, cited capability, never by
invention.

## Rejected

Minimal vocabulary, extended on demand — it leaves permanent coverage holes
(the unclaimed-entry advisory) and makes each addition a fresh ruling when
the external documentation already settles the question. Inventing values
past the documentation — the mining swamp's front door.

## Consequences

`user-invoked` joins the registration vocabulary; hook/settings kinds file
as derivable work; the unclaimed-entry advisory becomes shippable; each
addition carries its doc citation at the point of claim.
