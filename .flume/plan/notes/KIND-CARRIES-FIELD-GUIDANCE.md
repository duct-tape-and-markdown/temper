# KIND-CARRIES-FIELD-GUIDANCE

Shipped SDK-only: `KindFacts<T>.fieldGuidance?: FieldGuidance<T>` (exported from the
root), keyed by `keyof T & string`; the guidance rule body is the kind's counsel,
then `## Fields` with one `` - `field`: counsel `` line per entry in authored order
(`guidanceBody`, `sdk/src/declarations.ts`). A kind with only field guidance still
projects a rule; blank counsel is dropped.

Observed:
- A fieldless surface (`Record<never, never>`) resolves `FieldGuidance` to `never`,
  because a mapped type over no keys is `{}` and tsc runs no excess-property check
  against `{}`.
- Field guidance is plain text, uncited, and reaches no lock row, so `schema` hover
  and `explain` (contract.md "clause": "serve the human") don't see it yet. The
  Rust kind-fact row has no column for it. If the human faces should carry it,
  that's a separate entry needing a seam row change.
- Relocation-declared field guidance is out of scope here
  (RELOCATION-DECLARES-GUIDANCE). `KindRelocation` would need the same `keyof T`
  bound.
