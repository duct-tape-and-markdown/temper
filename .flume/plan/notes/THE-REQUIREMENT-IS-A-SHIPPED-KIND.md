# The requirement kind ships SDK-side; the lock still carries rows, not members

Shipped: `requirement` is an embedded root-hosted kind (`sdk/src/builtins.ts`), `requirement(…)` returns a member of it, and `extend(requirement, …)` mints subkinds (`kind.ts` `extendsKind` walks the chain by identity). `harness()` draws every requirement-lineage member out of `members` into `require`, and each requirement map accepts a literal or a requirement member (`requirementOf`). The built-in lock gains the kind row plus its contract marker, the bundle gains `kind-requirement.md`, and `coverage.checked` counts requirement rows as `requirement (N embedded)`.

Open seams for plan:
- A requirement member reaches the lock only as its `[[declaration.requirement]]` row, so the subkind's name and its own leaves (a consumer's `tell`) never reach explain, the reference or check. The "read uniformly" half of the inbox (`explain`, domain topics 0093) is undone. Carrying them needs a column on `RequirementRow` or requirement members as `nested_member` rows. That is a lock-shape decision.
- `extend` refuses a declared plain field on an embedded base (`placedBody`: no `region`, no `render`). A subkind's extra field therefore rides its TS surface undeclared. 0094/0099 name no placement for an embedded kind's added leaf.
- `kind-requirement.md` says "no clause holds its members", but the fill default rides each row's `required` column. The reference generator has no words for a per-member default.
- `expect: [{ kind: requirement, clauses }]` binds over a population the engine does not model, so those clauses judge nothing and nobody is told. It may need refusing.
