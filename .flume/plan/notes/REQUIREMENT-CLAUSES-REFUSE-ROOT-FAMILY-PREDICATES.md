# Requirement clauses refuse root-family predicates

Shipped: `roster::admissibility` (c) refuses any clause whose predicate
`admissibility::judged_binding` classifies as root-family, through the new
`admissibility::root_family_binding` (no fifth per-predicate match).

Observation: `resolves` is refused here too. Its `RootOrRequirement` binding
admits the `requirement` kind's contract, not one requirement's own opt-in
selection, so a requirement clause carrying it would judge nothing. If a
later spec intends `resolves` on a requirement's clauses to mean the
coverage tier's dangling-link judge, that is a spec ruling, not this fence.
