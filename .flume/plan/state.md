# Plan state

- **Phase:** reconcile. Verified on disk this tick: GOV-TYPED-REFERENCE
  **shipped** — `compose.rs` `Membership.source_contract` + `parse_conforms_to`
  and `roster.rs` `out_of_set` draw the S₂ set only from sources conforming to
  contract C (reusing `RoleContract::resolve` + `engine::validate`), with the
  admissibility guard that a non-resolving `conforms_to` is a reported finding.
  The set scope (count · membership · unique · typed-reference) is now complete.
- **Last shipped:** GOV-TYPED-REFERENCE (`81e2f18`). Tree clean, inbox empty.
- **Queue:** GOV-GRAPH **`open`, pickable** — the graph-scope foundation
  (`45-governance.md`, "The harness is a graph too"): parse a declared edge field
  in `temper.toml`, assemble the directed harness graph over the corpus, flag any
  route resolving to no artifact (the referential primitive over a declared
  syntax). Only pickable entry ⇒ parallel-safe. SPEC-KIND-GATE **`parked`** —
  `contracts/spec.toml` still untracked/absent; a human must commit it.
- **Frontier:** the graph predicates `degree` / `acyclic` (`45-governance.md`)
  follow GOV-GRAPH as next-tick entries once the foundation lands on disk. The
  `spec` references-resolve clause remains downstream of the spec.toml commit.

Plan continues: no — set scope confirmed complete, GOV-GRAPH filed as the one
pickable foundation entry, SPEC-KIND-GATE still parked, inbox empty. Hand to build.
