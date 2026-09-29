# 0068 — a corpus kind may take a built-in's name

- **Date:** 2026-09-29 · **Status:** accepted

## Context

The `(builtin-relocation-unnamed)` fork. Three layers judge a corpus kind
that reuses a built-in's name, by three rules — structural, provenance,
and string first-wins — while the evergreen corpus never names the case.
`builtins.md` says two providers cannot collide; it was silent on one
corpus redeclaring one provider's name. Ruled by John 2026-09-29 on the
session's recommendation.

## Decision

**One harness holds one kind per name, and a corpus kind of a built-in's
name replaces the built-in there** — ownership, not privilege.
`representation.md` "kind" and `builtins.md`'s opening state it.
Provenance is the one authoring answer; the row-side structural test
stays the engine's reading of it.

## Rejected

- **Refusing the reuse.** It makes a built-in's name privileged, which
  "ownership, not privilege" rules out.
- **Name qualification now.** No second provider ships; `provider` on
  kind rows answers that day.

## Consequences

`kindsInPlay` admits the corpus kind over the built-in of its name,
never by first-wins.
Two kinds of one name in play stay a malformed lock.
