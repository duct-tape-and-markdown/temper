# The requirement is a shipped kind in the corpus and a plain value in the code

`specs/model/contract.md` ("requirement — a shipped kind, not a primitive"): "A requirement is a member of a built-in kind (embedded locus)." `specs/builtins.md` ("Requirement"): "The requirement is itself a shipped kind." The code builds neither. `requirement()` (`sdk/src/contract.ts:542`) returns a `Requirement` value carried in `harness({ require })`. It is not a `KindDefinition`, and `src/` has no requirement kind. Verified at 9ea727d7 while checking whether a consumer can `extend(requirement, …)`. They cannot.

This is an instance of class 0104, one concept as one member kind, and the same move that made `tool` a shipped embedded kind. As a kind, a requirement:
- extends like any kind (0094, 0099), so a consumer adds its own `tell`, `holder` or `area` in its own vocabulary;
- is a member that `explain`, the reference's domain topics (0093) and edges read uniformly;
- keeps its template (prose, verifier, attached clauses) and its default fill clause.

The census covers the `require` map's readers: assembly, the declaration rows, `explain`, the reference's domain topics, and the `satisfies` resolution. Acceptance:
- **new:** `extend(requirement, { name, fields })` mints a subkind whose members fill `satisfies` targets;
- **kept:** `harness({ require: { … } })` still declares requirements;
- **kept:** the shipped fill default still refuses an unfilled required requirement.
