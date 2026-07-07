# Plan state

- Spec derived through: 813ca61
- Audited through: 813ca61
- Residue swept through: 813ca61
- This tick: spec delta — probed the next recut slice (the requirement arc,
  0001's "Consequences") and hit its design fork. Registered open question
  `(requirement-satisfier-kind)`: `roster.rs` computes the satisfier set
  kind-typed, `coverage.rs` kind-blind (0001's named live bug); contract.md's
  opt-in selector reads kind-blind, but retiring `requirement.kind` drops a
  capability — needs John, not a mechanical derivation. Filed NO entry: the arc
  (embedded locus → requirement-as-member → required→cardinality + satisfier
  unification) is foundation- and fork-blocked, not one contained slice. Cursor
  HOLDS at 813ca61 — the recut stays far from fully derived (embedded locus,
  one edge enumeration, requirement-as-member + prose-persistence, kinds for
  hooks/permissions/MCP; be8e1bf enforcement-mode rides `(authority-home)`).
- Queue: 2 entries — VACATE-KIND-NOUN (open, pickable); PACKAGING-CHANNELS
  (parked). Disjoint file sets. The new fork blocks the un-filed requirement
  arc, neither pending entry.

Plan continues: yes — spec delta is now fork-parked (its next arc waits on
`(requirement-satisfier-kind)`; the rest waits on `(authority-home)` /
`(json-projection-format)`), so the next live input is the residue sweep:
retired vocab across src/ (`means`, `required`, `verified_by`, `genre`/
`GenreValue`, `00-intent.md`/`architecture/*` cites) sits past the residue
cursor (813ca61), un-operationalized and not owned by any live entry.
