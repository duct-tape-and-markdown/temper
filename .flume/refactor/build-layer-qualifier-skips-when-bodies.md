## Surface

`compose::qualify_layer_label` (`src/compose.rs:1019`) appends
`@<layer>` to a joined row's own `label` and stops there — a `when` row's
`body` rows keep the labels emit stamped in the layer's own lock. Since
WHEN-BODY-LABEL-OWNED-BY-ITS-HOST those body labels are
`<host-label>.<predicate>.<field>`, so a layer's body clause and the host's
identical one now share one address while their hosts do not, and the
`@`-can-never-collide invariant the qualifier exists to hold
(`src/compose.rs:957`) holds only one level deep.

Consequence, concretely: `clause_collision_diagnostics`
(`src/admissibility.rs:279`) descends into the host's and the requirements'
bodies but must skip the joined arm's — a layer emitted from a program that
also expects `marketplaceDefaultContract` would otherwise be refused for a
collision that is the qualifier's gap, not the corpus's. So a layer's own
body twin stays unrefusable, and a dial reaching a joined body clause reaches
the host's too.

## Observed at

0b587eb1 (HEAD when observed)

## Suggested consolidation

Qualify the whole row, not its head: have `qualify_layer_label` recurse into
`body` (guards do not nest, so one level), then drop the joined-arm carve-out
in `clause_collision_diagnostics` so one walk covers all three sources.
