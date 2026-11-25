# supporting-doc.degree folds to one coverage line on an unrepresented harness

Shipped. `gate.rs` partitions `graph::degree`'s output: on a harness with no
committed lock, findings from the `supporting-doc` by-kind selection's
incoming-bounded `degree` clauses are counted, not reported, and
`coverage_note::check` takes the count as a new `unreached_supporting_docs`
parameter, printing one `coverage.unreached-supporting-docs` note. Matching
is by clause label off the selection, so a redeclared or dialed clause still
folds. The guidance now opens on the decidable fact ("No typed reference
reaches this file") and names `mentionOf` and its `path` form as the remedy.

Observed: the fold is specific to supporting-doc. Any other incoming-degree
clause a represented program might declare is unaffected, but if a future
built-in default adds another reach bound, the same unfillable-selection
argument applies and the fold should generalize: key it on the predicate
(incoming `degree` with no declared edges), not the kind.

Severity: the line is a `note` (disclosure), not a `warn`. Its remedy
(represent the harness) is not a per-doc fix, so `--deny-advisories` should
not promote it.
