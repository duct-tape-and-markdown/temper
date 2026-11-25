# 0099 — a nested subkind is chosen by its host

- **Date:** 2026-11-25 · **Status:** accepted

## Context

0094 places a subkind's documents by locus: a subkind governs a locus
within its base's. A nested kind governs no path. A supporting doc's
place comes from its skill's template, and a handler's from its hook,
and an embedded entry's from its host's admission. So 0094 cannot say
when a nested member is the subkind. The SDK holds two nested built-ins,
`supporting-doc` and `handler`, and consumers hold their own (Cascade's
`invariant`). Cascade's `standard()` needs exactly this: its topic docs
(when, outcome, steps, check) are supporting docs with a typed surface.
John, 2026-11-25: the gap is "maybe an opportunity".

## Decision

**A nested member's kind is declared by its host's template or admission,
as a file's is by its position** (`representation.md`, "kind"). To extend
a nested kind, an extended host names the subkind as its child:
`extend(skill, { name: "procedure", template: topic })`, where `topic`
extends `supporting-doc`. Selections of the base bind the subkind (0094),
and its fields live where the base's format admits them.

## Rejected

- **A path pattern for nested subkinds.** It is a second spelling of a
  placement the host template already owns.
- **Admission alone for every nested case.** It would leave nested-file
  children, which a template places, unextendable.

## Consequences

Cascade's `standard()` lowers whole: `standard` extends `rule` with an
embedded invariant subkind, `procedure` extends `skill` with `topic`
children, and a typed edge `serves: procedure → standard` links them.
The router's index lines render from each topic's `when` (0093).
