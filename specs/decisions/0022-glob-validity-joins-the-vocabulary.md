# 0022 — glob validity joins the predicate vocabulary

- **Date:** 2026-07-15 · **Status:** accepted

## Context

The 2026-07-15 builtins audit surfaced two coverage candidates that reach
past the closed predicate vocabulary (`model/contract.md`, "clause": adding
a predicate is a deliberate language change), registered as the
`(builtins-coverage-predicates)` fork. Both were session-argued and
human-ruled the same day.

## Decision

The vocabulary admits a **glob-validity predicate family**: the globs a
selected field carries must parse under the sanctioned engine (`globset`,
brace-expansion aware — the one glob engine, already inside `ignore`).
First consumers: the `rule` and `skill` default contracts, over their
`paths` fields. The failure it gates is vendor-documented and silent — an
unparseable `[` makes the pattern invalid and it matches nothing, so the
rule never fires and the skill never registers, with no error surfaced
(code.claude.com/docs/en/memory, retrieved 2026-07-15; re-fetch raw when
the clause's own `cite` is encoded, per `builtins.md`). Ordinary clause
mechanics: dialable severity, guidance, cite; nothing about the family is
privileged.

The two skill predicates already deferred in code (name hyphen-position,
no-XML-in-description) remain deferred; they may ride a later wave under
their own design, and this decision is not their warrant.

## Rejected

**`tools-must-resolve`** — a clause requiring an agent's `tools` entries to
name real tools — is rejected, permanently, on invariant 2 (decidable only:
a violation is always a true positive). The tool universe is the running
registry — built-ins ∪ MCP tools ∪ skills — knowable only at session time,
so any static clause false-positives on legitimate entries. The runtime
owns this failure loudly at launch (documented v2.1.208+,
code.claude.com/docs/en/sub-agents, retrieved 2026-07-15). Recorded so the
candidate is not re-proposed by a future audit.

## Consequences

The engine's `Predicate` enum grows one variant with its schema surface;
the `rule` and `skill` default contracts gain the clause with a fresh raw
cite; the frozen built-in lock re-derives. No author-facing pattern clause
is introduced — glob validity checks syntax under a named engine, never
content shape (the `allowed_chars` stance stands).
