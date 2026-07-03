<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- 2026-07-02 (human): CUSTOM-UNIT-REPRESENTATION-CARRY — the intent↔architecture
  trace graph is authored (7d94abc: 17 member-published requirements on the
  intent surface members, 17 single-homed satisfies on architecture members;
  gate-verified). It carries a durability landmine: `import_custom_unit`
  (src/import.rs, the sole import/re-add path for every custom kind) writes a
  bare provenance-only header with NO carry-forward, unlike
  `import_frontmatter_member`'s carry_representation — so any re-import or
  re-add of a changed spec silently wipes the hand-authored
  [requirement.*]/[satisfies.*] tables. File a slice: custom-unit import/re-add
  must carry the authored representation layer forward exactly as the
  frontmatter path does. Until it ships, 05-model's source owning-spec column
  stays as the reconcile-durable copy — the DRY dedup of that column is
  contingent on this slice, not before it.

- 2026-07-02 (human): READ-VERBS-PUBLISHED-DEMANDS — `temper why` and
  `temper requirements` resolve against the assembly roster only, blind to
  member-published requirements: `requirements` lists 2 of 19, and `why
  45-governance` misreports every trace join as "this link dangles" while
  `check` is clean (coverage composes assembly ∪ member-published; the verbs
  don't). File a slice: the read verbs range over the same composed roster
  coverage gates. The misreport is worse than a gap — the verb states a
  falsehood about a green graph.
