# LOCKLESS-CHECK-RESOLVES-THE-BUILT-IN-EDGES: shipped with 0106's off-tree collapse

Shipped. A lockless harness now reads its assembly edges off the embedded
built-in lock (`drift::edge_declarations`, shared by gate and explain).

The entry also needed 0106 itself, which nothing had built yet: a missed
target whose kind the runtime also loads from scopes temper does not read
now goes into `ResolvedEdgesResult::off_tree`. The gate folds all of them
into one `coverage.off-tree-targets` note naming each reference and the
scopes. That applies to represented harnesses too, not only lockless ones.
Off-tree-ness is a fact about the target kind, so it lives in
`builtin_kind::OFF_TREE_SCOPES`, cited against the docs fetched 2026-11-26:
output-style (user `~/.claude/output-styles`, managed policy, plugins) and
known-marketplace ("declared in user settings").

Four existing tests asserted the pre-0106 dangling verdict, and I flipped
them to the note: the case-mismatched `Terse`, the case-folded
`explanatory`, settings-local `ghost`, and the `graph::resolved_edges`
marketplace test.

For plan:
- The entry's acceptance text ("reports a ... naming no style") still
  reads as a dangling finding. It is satisfied as a note.
- A custom kind's edges into some other target kind still dangle. Only the
  two built-in kinds above are off-tree. If an adopter's custom kind also
  loads from user scope, it has no way to say so (no lock column). That
  needs a decision before it is added.
- Edges from a plugin-provided member, or a user-scope skill, are not
  covered. No shipped edge targets them yet.
- The runner's nextest pre-filter is a case-sensitive regex of the line's
  lowercased words, so a camelCase word in a line (`outputStyle`) must be
  spelled lowercase-joined in the fn name (`outputstyle`). That cost one revert.
