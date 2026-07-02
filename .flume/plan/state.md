# Plan state

- **Phase:** reconcile. HEAD 3d2b6a8.
- **Last shipped:** the curated adapter-face lines — `chore(harness)` landed
  `format="yaml-frontmatter"` + unit shape on kinds/{skill,rule}/KIND.md
  (3d2b6a8 un-parked the tick); FORMAT-KEY-PARSE (56732ee) + ADAPTER-EQUIVALENCE-PIN
  (4765d5b) shipped earlier.
- **This tick:** verified on disk both KIND.md files now carry `format`/`unit_shape`
  (kinds/skill directory, kinds/rule file) and the typed `CustomKind.format`/
  `unit_shape` fields + closed-vocab guard are live (kind.rs). Both build prereqs
  are down, so **flipped DECLARED-FRONTMATTER-ADAPTER parked → open**; spot-checked
  its whole blast radius (skill.rs/rule.rs, import_skill/rule, skill_fields/rule_fields,
  Workspace, builtin_kind parity helpers) still on disk. Drained inbox: routed the
  join Decision (`45-governance.md`) as a breadcrumb into `(edge-representation-unify)`
  — doctrine, no engine work.
- **In flight / pickable:** DECLARED-FRONTMATTER-ADAPTER (open, sole pickable) —
  the generic-adapter swap, one atomic green commit.
- **Next:** build ships the adapter; then MEMORY-KIND files (open-questions recipe)
  as the first pure-data managed kind, reviving EXTRACTION-VOCAB-GAPS via its `@path`
  executed-reference. Others stay parked/deferred (human creds, fence, no consumer).

Plan continues: no — one pickable `open` entry; building is how the queue drains.
