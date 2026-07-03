# Plan state

- **Phase:** reconcile. HEAD 69bb5a0.
- **Last shipped:** PROVIDER-KEY-PARSE (build c52df4f, chore 889ddb9) — kind.rs
  parses the inert `provider` key, exposes qualified identity (`claude-code.skill`)
  and bare→unique-or-collision resolution (`resolve_bare`, `AmbiguousKind`,
  `BadProvider`). Verified on disk.
- **This tick:** drained the inbox line into a new **open** entry EMBED-NESTED-WALK
  — build.rs's kind embed walk tolerates the nested `kinds/<provider>/<name>/`
  layout (keys qualified) beside the flat one, and builtin_kind lookups route bare
  → unique via `resolve_bare`; buildable now (nested branch dormant over today's
  flat/bare tree; disk confirms build.rs walks `<tree>/<name>/<marker>` flat).
  Rewrote BINDING-QUALIFY: removed the build.rs + builtin_kind.rs edits (now
  EMBED-NESTED-WALK's), reparked on EMBED-NESTED-WALK shipping + the human
  file-move. Other five entries reconciled unchanged (MEMORY-KIND / PACKAGING /
  COMMUNITY parked; EXTRACTION-VOCAB-GAPS / AGENT-KIND deferred, no consumer).
- **In flight / pickable:** EMBED-NESTED-WALK (open) — the one pickable entry.
  Parked: BINDING-QUALIFY, MEMORY-KIND, PACKAGING-CHANNELS, COMMUNITY-DOCS.
  Deferred: EXTRACTION-VOCAB-GAPS, AGENT-KIND.
- **Next:** build picks EMBED-NESTED-WALK. After it ships, the human file-move
  (kinds/skill → kinds/claude-code/skill + provider lines) un-parks BINDING-QUALIFY.

Plan continues: no — queue reconciled, inbox drained, EMBED-NESTED-WALK is open and
pickable; hand to build.
