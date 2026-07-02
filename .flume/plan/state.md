# Plan state

- **Phase:** reconcile. Queue reconciled to the corpus; inbox drained; one new
  `open` entry filed. HEAD a74b1c9; tree clean.
- **Last shipped (trunk):** the harness-locus spec fix (a74b1c9) + AGENTS.md by hand
  (267599a). Verified on disk: AGENTS.md present; `src/import.rs` still scans skills at
  `<harness>/skills/` (bug unfixed in code — spec corrected, code not). The engine is
  broad against the corpus — import/check/drift/apply/re-add, bundle, install, schema,
  reporters, coverage/graph, the roster set-scope predicates, read verbs, section_contains,
  custom `.temper/kinds/<name>/KIND.md` kinds all shipped.
- **This tick:** drained the inbox BUG into IMPORT-SKILL-LOCUS (`open`) — `import`'s
  skill scan is `<harness>/skills/` but rules are `<harness>/.claude/rules/`, so no root
  captures a standard project; the spec (a74b1c9) already says project-root scans
  `.claude/skills/`. Confirmed AGENTS-MD is done (shipped by hand, off the queue).
  AGENT-KIND/PACKAGING-CHANNELS reconciled — still accurate, carried.
- **Pickable now (1 `open`):** IMPORT-SKILL-LOCUS (wide test blast radius but the sole
  open entry — no parallel conflict). Deferred: AGENT-KIND (priority). Parked:
  PACKAGING-CHANNELS (human release creds). Forks: all RESOLVED/OPEN records, no filed
  dependents.

Plan continues: no — inbox drained, queue reconciled, IMPORT-SKILL-LOCUS is pickable
`open`; building drains it.
