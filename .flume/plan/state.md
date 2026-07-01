# Plan state

- **Phase:** reconcile. Verified on disk this tick: GOV-MEMBERSHIP **shipped** —
  `compose.rs` `Membership` (field / kind / selector / feature, with the
  `RoleBadMembership` parse guard) and `roster.rs` `out_of_set` decides it under
  `ROLE_MEMBERSHIP_RULE` (draw the S₂ feature-set, flag any S₁ filler outside it).
  No `source_contract` / typed-reference anywhere in `src/`.
- **Last shipped:** GOV-MEMBERSHIP (`47db174`). Tree clean, inbox empty, no new fork.
- **Queue:** GOV-TYPED-REFERENCE **`open`, pickable** — the fourth set-scope
  predicate (`45-governance.md`): membership where S₂ is "kind K conforming-to
  contract C", so a reference resolves to the right *kind of thing* (`compose.rs`
  + `roster.rs`, reusing `RoleContract::resolve` + `engine::validate`).
  SPEC-KIND-GATE **`parked`** on a human committing the untracked
  `contracts/spec.toml`. Disjoint (compose/roster vs `main.rs`), and
  GOV-TYPED-REFERENCE is the only pickable entry ⇒ `open` is parallel-safe.
- **Frontier:** graph scope (`degree`, `acyclic`) awaits an edge-extraction +
  graph foundation (`45` "The harness is a graph too") — filed after
  GOV-TYPED-REFERENCE. `(skill-ref-syntax)` and `(regex-crate)` reconciled to
  RESOLVED this tick (their specs already carry the resolving decision).

Plan continues: no — GOV-MEMBERSHIP confirmed shipped, GOV-TYPED-REFERENCE filed
as the one pickable entry, two drifted forks reconciled to RESOLVED, inbox empty.
Hand to build.
