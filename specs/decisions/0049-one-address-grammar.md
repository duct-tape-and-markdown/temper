# 0049 — one address grammar

- **Date:** 2026-09-05 · **Status:** accepted

## Context

"Address" is used across the model and defined nowhere. Four spellings
grew: findings and lock rows write `kind:name`; the leaf grammar the read
family accepts is host-scoped (`<member>/<kind>/<key>/<child>`); edge
resolution identifies a nested member by `(kind, key)` alone, blind to
its host; `explain` accepts `member:`/`requirement:`/`kind:`/`address:`
qualifiers and rejects the `kind:name` form the engine itself prints. The
probe hit every seam between them: two specs carrying an invariant keyed
alike resolved an edge to one of them silently; a renamed key re-pointed
edges to a stranger; a clause label omitted the argument that told two
clauses apart, so a second clause was a malformed lock instead of a
second clause; `explain kind:<x>` narrated a kind that did not exist.

## Decision

**One address grammar.** A member's identity is spelled as an address:
`<kind>:<name>` for a top-level member; `<host-address>/<kind>/<key>` for
a nested member, and `/<leaf>` beneath it — nesting is model containment,
so the address composes through the host the way a nested file composes
its path. Every row spells it, every finding prints it, and every verb
accepts it; `explain` takes an address or a bare name, and a bare name
resolves exactly as an edge's bare identity does — within one declared
kind when unique, refused as ambiguous otherwise. Resolution is **total**:
an address names exactly one thing or the verb refuses, and two rows
whose addresses coincide are a malformed lock (the rule `pipeline.md`
already states for labels, now stated for members). A compiled label is a
clause's address and carries every argument that distinguishes the clause.

## Rejected

- **Corpus-unique keys as the address** (`kind:key` for a nested member):
  shorter, but it makes the host a convention the author maintains by
  prefixing keys, and a rename across hosts stays a silent re-point.
  Uniqueness is the resolver's bar, not the grammar's.
- **Authored labels** on clauses: a label is compiled, never authored
  (`pipeline.md`); folding the distinguishing argument into the compiled
  label keeps it deterministic and spellable in a dial entry.
- **Keeping `explain`'s qualifier namespace** beside the grammar: two ways
  to spell one member is the defect being retired.

## Consequences

`representation.md`'s member identity bullet defines the address; the
`kind:name` form and the leaf grammar are unchanged, so no committed lock
re-spells. Code: `graph` resolves a nested target through its host, and
admissibility refuses duplicate `(kind, key)` across hosts as coincident
addresses; `explain` accepts the grammar and refuses an undeclared kind
under `kind:`; `clause_label` folds `section_contains`'s marker in; the
SDK's member table indexes nested values under their address so an
embedded edge target resolves at emit as a top-level one does.
