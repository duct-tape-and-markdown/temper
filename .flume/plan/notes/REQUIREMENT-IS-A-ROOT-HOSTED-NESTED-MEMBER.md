# Requirement is a root-hosted nested member: shipped, one seam left two-noded

The SDK now writes a `root/<kind>/<name>` `nested_member` row for every declared
requirement. Its kind is the subkind's own name, or `requirement` for a literal, and
its string fields are its leaves. A requirement subkind is put in play too, so its
kind row carries `base`. The `[[declaration.requirement]]` row stays and still carries
the contract-facing template: fill posture, clauses, verifier, prose.

Engine side:
- `admissibility::ROOT_HOSTED_KINDS` gains `requirement`. Its subkinds are admitted by
  base chain (`root_hosted_kinds` / `requirement_kinds`).
- The embedded dispatch already judges member-grain `expect` clauses over those
  members, so `kind_selections` needed no change.
- `read::resolve_leaf` and `sibling_leaves` read `root/...` leaves off the member's own
  features, so tool leaves resolve now too.
- The requirement narration names its kind, its address and its leaves.
- The coverage line counts a requirement once, under its own kind.

Debt, observed and not taken:
- `graph.rs` keeps two nodes for one requirement: the reserved `(requirement, <name>)`
  node that satisfies/mention arcs target, and the member node
  `(<kind>, root/<kind>/<name>)` that containment from root reaches. The fill pin holds,
  but the two never join. Folding them into one is the spec's "one grammar" end state,
  and it touches node species, `explain`'s requirement/member split and the
  blast-radius walks.
- A subkind's own field (`tell`) is not in its kind row's `leaves` column, which lists
  only the base's declared leaves. That is pre-existing `extend` behavior.
- `builtin_lock.toml` did not move, since it has no requirements.
