# Plan state

- **Phase:** reconcile. Queue reconciled to the corpus; inbox drained; tree clean.
  HEAD a74b1c9.
- **Last shipped (trunk):** AGENTS.md shipped by hand (267599a — dropped from the
  queue); the spec's one-`harness_path` correction landed (a74b1c9, human). The
  engine is broad against the corpus — import/check/drift/apply/re-add, bundle,
  install, schema, reporters, coverage/graph, roster set-scope predicates, custom
  `.temper/kinds/`, the read verbs (why/requirements), and section_contains all shipped.
- **This tick:** drained the inbox BUG into **IMPORT-SKILL-LOCUS** (`open`) — code's
  skill scan root (`<harness>/skills/`) disagrees with the now-corrected spec
  (`.claude/skills/`), so no single project-root harness_path captures a standard
  Claude Code project. Verified on disk: `discover_skill_dirs` (src/import.rs) scans
  `<harness>/skills/` + bare `SKILL.md`; `discover_rule_files` scans `.claude/rules/`.
  Wide test ripple (one scan-fn change) scoped into the entry. No other src↔spec gap found.
- **Pickable now (1 `open`):** IMPORT-SKILL-LOCUS (single atomic scan-locus fix; no
  parallel peer). Deferred: AGENT-KIND (priority). Parked: PACKAGING-CHANNELS (human
  release creds). Forks: all RESOLVED/OPEN decision records with no filed dependents.

Plan continues: no — inbox drained, queue reconciled, IMPORT-SKILL-LOCUS is pickable
`open`; building drains it.
