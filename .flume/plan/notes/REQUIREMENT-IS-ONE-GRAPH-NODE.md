# Requirement is one graph node: shipped, one spec gap on unfiltered degree

`graph::bind_requirement_nodes` folds each declared requirement's reserved
`(requirement, name)` endpoint (mentions, root runs) onto its root-hosted member
node `(kind, root/<kind>/<name>)`. It also adds one `satisfies` arc from each
member to each declared requirement it names. gate and `explain_target` both
call it, so containment, satisfies, mention and runs arcs meet at one node.
`explain root/<kind>/<name>` lists each satisfier under "Edges in". Fill and
dangling verdicts still come from `coverage` by name, unchanged.

Spec gap (needs a human): contract.md "selection" says an unfiltered bound
ranges over "every edge but containment". Counting satisfies arcs unfiltered
made a requirement's `degree(outgoing >= 1)` over its satisfiers vacuous, since
every satisfier has one. That flipped
`graph::containment_incidence::the_same_floor_unfiltered_counts_no_containment_arc`.
`filtered_edges` now holds satisfies out of the unfiltered set the way it holds
out containment: it counts only where a filter names `satisfies`. The spec
sentence should say this, or rule otherwise. mention-reachable skips satisfies
arcs too.

Observed: the `requirement` kind's own `expect` clauses route to coverage /
opt-in, never to a kind selection (`gate::kind_selections` filters
`requirement`), so `degree` in `expect: [{kind: requirement}]` is judged
nowhere. The test binds the subkind (`kind: decision`), which does reach
`degree`. A literal requirement's population degree is still vacuous.

The SDK spells a requirement runner `root/requirement/<name>` even for a
subkind (declarations.ts runsRows), which is the hand-spelled-address debt.
The engine absorbs it through the fold.
