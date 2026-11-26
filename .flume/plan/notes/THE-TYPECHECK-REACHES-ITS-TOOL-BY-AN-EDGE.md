# Typecheck's tool edge rides beside the mention edges, not fold_runs

Shipped as the entry asks: the SDK's `runsRows` appends `{ member = "root",
tool }` when the program declares a typecheck, so the lock carries the edge
as an ordinary `[[declaration.runs]]` row.

Shape differs from `files[]`: `compose::fold_runs` is untouched. The root is
no kind member, so it has no features to fold a `runs` field onto. Instead
`drift::root_runs_edges_from_declarations` lifts root-runner rows into
`ResolvedEdge`s (`graph::root_runs_edges`), and gate and read both extend
their lifted-edge list (`mention_edges`) with them. `degree` and `explain`
then see the same arc. `mention_finding` skips non-`mention` fields, so the
arc never route-checks.

Two observations:
- `root/tool/<name>` does not parse as a nested address. `host_identity`
  rejects the bare `root` host, so `node_from_address` turns it into a
  requirement node. The handler path never hits this because
  `target_identity` keys the node `(tool, address)`, and the root edge now
  does the same. Any future lift of a root-hosted address through
  `node_from_address`, such as a mention targeting a tool, would miss the
  same way.
- The root node is `(root, root)`, a pseudo-kind. Nothing selects it today,
  so its out-degree is never judged.

`src/builtin_lock.toml` is unchanged: the built-in program declares no
typecheck, so it gets no root runs row.
