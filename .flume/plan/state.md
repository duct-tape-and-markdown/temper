# Plan state

- Spec derived through: 14a803b
- Audited through: 8978596
- Residue swept through: 8978596
- This tick: SPEC DELTA — derived 0028 (14a803b, `mention-reachable`) into
  **MENTION-REACHABLE-PREDICATE** (the vocabulary word, `per`
  contract.md/"clause") and **MENTION-REACHABLE-RULE-CLAUSE** (the rule
  default contract's adoption + lock re-derive, `per` builtins.md/"The
  clauses live in code"). 0028's Consequences checklist, every bullet: (1)
  "`Predicate` enum grows one variant with its schema surface" → the
  PREDICATE entry, which names `schema.rs:110-124` as an exhaustive match
  that breaks the build; (2) "the `rule` default contract gains the clause
  at advisory with a fresh raw cite" → the RULE-CLAUSE entry, cite re-fetch
  named as a build act per builtins.md; (3) "the frozen built-in lock
  re-derives" → same entry, `src/builtin_lock.toml` regenerated, pinned by
  `tests/builtin_lock_frozen.rs:77`; (4) "contract.md's obligation-free
  mention sentence amended in the same commit" → verified moot on disk,
  contract.md:39 carries it; (5) "the fork record deletes" → verified moot,
  `(mention-gate-containment)` absent from open-questions. **One collision
  surfaced, not encoded.** 0028's "every mention edge whose target member
  registers by `paths-match`" does not select skills on this tree:
  `Registration::PathsMatch` is the `rule` kind's alone
  (builtin_lock.toml:80), while `skill` registration is
  user-invoked+description-trigger (builtin_lock.toml:88, builtins.ts:181)
  and its `paths` is a documented, cited, probe-verified GATE that
  deliberately adds no paths-match entry (builtins.ts:122-131). Read
  literally, the ruling's own rule→skill case never fires. Derived instead
  on 0028's design sentence — the two fields are arguments and the
  predicate hard-codes no kind — with the discrepancy stated in the entry;
  the record is the human's to correct through the inbox. Judge home
  verified: `graph.rs` already holds `declared_globs` (576),
  `dead_registration`'s never-cry-wolf posture (576-577), and the `Degree`
  clause path (245-258) with `mention_edges` in scope. Closing checklist:
  MENTION-REACHABLE-PREDICATE shares `sdk/src/index.ts` with
  SEAM-EXPORTS-RETRACT, so it is `blockedBy` it, not `open` beside it;
  RULE-CLAUSE is `blockedBy` PREDICATE. Both parks re-tested at this HEAD
  and hold verbatim (no version tag, crate 0.1.0 vs npm 0.0.7,
  release.yml:7-9; `MAX_IMPORT_HOPS` reads 5 at graph.rs:62, hop semantics
  unruled). IMPORT-HOP-CAP-CITE shares graph.rs with PREDICATE — no
  conflict while parked, a serialization to set when it unparks. Four forks
  re-read; `(eval-capability)`'s dangling `(mention-gate-containment)`
  cross-ref repointed at 0028. Audit/residue cursors copied forward
  verbatim — this tick reconciled no code window.
- Queue: 1 pickable (SEAM-EXPORTS-RETRACT); 2 blocked in one chain
  (MENTION-REACHABLE-PREDICATE → -RULE-CLAUSE); 2 parked on human acts
  (IMPORT-HOP-CAP-CITE: a hop-depth probe. PACKAGING-CHANNELS-REMAINDER:
  Apple notarizing + the v0.1 tag). No gate is stale — all tested this tick.

Plan continues: yes — the spec delta is still live: f6fe385 (0029, an edge
declares its target set) and a571973 are un-routed past this tick's cursor.
Next tick derives 0029; its Consequences section is that tick's checklist.
