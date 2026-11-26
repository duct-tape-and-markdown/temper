# EXPLAIN-IMPACT-READS-THE-GATES-REACHABILITY-INPUTS

Shipped. `explain_target` now takes its registration map from
`gate::registrations_by_kind` (now `pub(crate)`), and
`graph::reachability_orphaned` takes the same `(resolved, directives)` pair
`graph::reachable` takes, both chained by one private `reachability_arcs`.
`read::explain` computes the resolved declared edges from the `edges` it
already received, so its public signature did not change.

Observed, not taken:
- `read::explain`'s Member branch now runs `graph::resolved_edges` twice per
  target: once inside `why_impl` and once for impact. That is cheap at member
  scale, but a later tick could hoist one walk into `explain`, which would mean
  changing `why_impl`'s signature.
- `explain_target` still re-assembles the corpus by hand alongside `gate`
  (contracts, custom rows, by_kind, directive classing). This entry
  unified the reachability inputs only. The rest of that parallel assembly is
  where the next READ-EDGE-UNIFY drift will come from.
- Impact strand (2), "Directive edges left unbacked", still reads directive
  arcs only, which is correct since a declared edge has no backing file.
  Its prose still says "imports"; strand (3) now says "references or imports".
