# 0053 — the kind row carries its leaf set

- **Date:** 2026-09-08 · **Status:** accepted

## Context

The `(kind-declared-leaf-schema)` fork (GH #47). `explain kind:<x>` on a
kind with no member yet — the adopter's actual moment — can teach nothing
about the leaves its embedded children carry: no kind declares a field
set, `corpus_leaves` unions what existing members carry, and the lock's
kind row has no leaf column to render from. Two shapes were open: the
kind declares its children's leaf names, a new authored surface; or the
SDK lowers `keyof T` off the embedded value type at emit, so the row
carries a derived fact and no author writes it twice. Ruled by John
2026-09-08, after the clarification that temper ships the column and the
rendering while the corpus supplies the key set through its typed
constructor.

## Decision

**The lock's kind row carries the kind's leaf set as a derived column**,
lowered at emit from `keyof T` when the SDK knows the embedded value type,
absent otherwise. The column is optional with a serde default, so no
committed lock re-spells (0024). `explain kind:<x>` renders the set in
the hosted-kinds strand in place of "no member of it is in this surface
yet", so an adopter reads what a child carries before authoring one. The
type is the declaration; the row records what only the compiler knew.

## Rejected

- **A kind-declared leaf schema**: a second place to be wrong, competing
  with the member type it would duplicate — the cost `engineering.md`'s
  derived-state rule names.
- **Leaving it to `corpus_leaves`**: teaches only after the first member
  exists, which is after the moment the verb serves.

## Consequences

`representation.md` is unchanged: the leaf set is a fact of the kind row,
not a new noun. Plan derives the entry: the column on `KindFactRow`, its
lowering in `kindFactRow` from the typed constructor, and the `explain`
strand. The `(kind-declared-leaf-schema)` record deletes.
