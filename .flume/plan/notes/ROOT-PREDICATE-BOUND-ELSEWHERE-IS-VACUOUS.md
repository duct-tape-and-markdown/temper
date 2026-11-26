# Root-family predicates refused at kinds that never judge them

Shipped `admissibility::root_predicate_admissibility`, wired in `gate::run` right
after `joined_kind_admissibility` (once `contracts` is complete). The family is
an exhaustive match in `admissibility::judged_binding`: resolves (root, or the
`requirement` kind via `coverage::check`), import-backed, reachable,
locus-declared, fresh, engine-matches (root only).

Not covered, same defect: a requirement's *own* `clauses` (roster opt-in
selection) carrying a root-family predicate. `roster::admissibility` reuses
`engine::inadmissibilities`, which has no locus for "opt-in selection", so the
fence was left to a later entry rather than widened silently. A `when` body
already refuses these (they range over the selection).

Debt: `judged_binding` is a fourth exhaustive per-predicate match beside
engine.rs's `addressed_field`/`feature_read`/`judgeless`/`vacuities`; a
`Predicate`-level classification (e.g. `Predicate::root_bound()`) would let
`engine::root_clause` callers and this fence share one source.
