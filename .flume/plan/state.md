# Plan state

- Spec derived through: a0fccaf
- Audited through: 008a995
- Residue swept through: 9e32fa8
- This tick: Residue sweep — ca1e413 swept; one gap filed
  (UNTEMPLATED-NESTED-MEMBER-LOUD, open: no SDK check ties a `blocks()`
  value's kind to any host's `templates` — `templatesFor` reads only
  `withinHosts` — so an orphaned nested_member row emits legitimately and
  `embedded_features_by_kind`'s get_mut silently drops it from the by-kind
  corpus while the host-address read carries it; emit refusal + admissibility
  backstop, no file overlap with queued entries). Judged clean: the two
  NestedMemberRow lifts are two jobs (EmbeddedMember host view vs Features
  edge-target view), and the get_mut drop's declared-absence semantics are
  loud at every reference once the new entry's admissibility half lands. One
  new riding comment debt (graph.rs:491 incident narration, rides the new
  entry); seven riding debts re-verified on disk, stamps advanced.
- Queue: BUILTIN-CONTRACT-ARRAY-SURGERY (open); LOCK-ROW-REJECT-LOUD
  (blockedBy BUILTIN-CONTRACT-ARRAY-SURGERY); UNTEMPLATED-NESTED-MEMBER-LOUD
  (open); PACKAGING-CHANNELS (parked).

Plan continues: yes — quiet pass (all inputs current; close the cycle, hand
off to build).
