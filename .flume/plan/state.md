# Plan state

- Spec derived through: 832f015
- Audited through: 832f015
- Residue swept through: 832f015
- This tick: RECONCILE `399d8e3..832f015` — both motions, one code commit in
  the window (6d145fa, CLAUSE-LABEL-IS-AN-ADDRESS; the three `specs:` commits
  are 0034, already derived at a94cfb5). **Audit:** the ship is verified on
  disk, never from the log — `clause_label` (`src/contract.rs`:84),
  `stamp_clause_labels` (`src/drift.rs`:1269), the `label` column (2658),
  `Clause.label` (`src/contract.rs`:51). The entry was already dropped by its
  own ship commit (6205707), so nothing was left to drop. Every gate re-tested:
  the chain's `blockedBy` tags all resolve, LOCAL-LOCUS-IS-READ-SIDE-ONLY stays
  the one pickable, and both parks hold on re-read — PACKAGING on every clause
  (four era tags only, crate 0.1.0 vs npm 0.0.7, release.yml:7-9 verbatim,
  `.github/` untouched), IMPORT-HOP-CAP-CITE because nothing ruled the hop
  semantics and the constant is still 5. **Sweep:** no residue. The label ship
  has one home — every site calls `contract::clause_label`, no second
  derivation — and `Selector::rule` is gone as its body claims. 0034's
  `layout-only` residue in `src/kind.rs` (50/735/738/746/761) is already
  operationalized by the pickable entry, not a fresh gap. One tension swept and
  cleared, not filed: a pre-label lock now hard-fails at `req_str(table,
  "label")` (`src/drift.rs`:3583), but 0024's owed "robust read" governs
  *spelling* normalization of existing labels, not a new identity column, and
  pre-1.0 carries no back-compat burden (`distribution.md`) with zero such locks
  in the wild. **Cite repair — the window's one real finding:** 6d145fa moved
  cites under two records that *asserted they had not moved*.
  IMPORT-HOP-CAP-CITE's notes claimed `git diff -- src/graph.rs tests/graph.rs`
  is empty and listed five unmoved cites; all false at HEAD (constant 65→59,
  narration 525→519 and 624→618, BFS 654→648, test 1356→1357) — re-read and
  re-stamped. The orphan cite `src/roster.rs`:465→470 likewise. The rest of the
  queue is deliberately NOT re-stamped: `scoped at <sha>` is the designed
  defense against line drift — build diffs that range at pick-up — so a mass
  re-stamp is churn and would falsely claim a re-scope.
- Queue: 12 entries, **1 pickable** — LOCAL-LOCUS-IS-READ-SIDE-ONLY. Nine chain
  behind it (0034's three → join → 0033's four → dial → announcement),
  serialized on shared files; no entry rests on a fork. Two parked on human acts.

Plan continues: no — the window is reconciled on both motions, the spec delta is
empty at 832f015, and the inbox and refactor captures are clear. Build takes over
on the pickable entry.
