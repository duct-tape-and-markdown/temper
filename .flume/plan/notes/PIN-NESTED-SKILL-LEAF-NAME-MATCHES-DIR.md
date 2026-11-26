# PIN-NESTED-SKILL-LEAF-NAME-MATCHES-DIR: shipped, property held

The pin `a nested skill leaf name passes skill name matches dir` lands in
`tests/it/nested_member.rs` (mod `nested_skill_leaf_name`). It drives the real
`check --harness` over `apps/web/.claude/skills/deploy/SKILL.md`: `name: deploy`
yields no `skill.name-matches-dir` finding while the member's identity reads
`apps:web:deploy`; a control run with `name: apps-web-deploy` fires the clause, so
the clean run is the leaf matching, not the clause skipped on nested placements.
No defect observed.
