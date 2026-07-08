# Plan state

- Spec derived through: 474835b
- Audited through: a112dbe
- Residue swept through: 91fb466
- This tick: Residue sweep 7ff3f03..HEAD (job 4) — clean. Only substantive
  commit is 81b6ab4 (AGENT-KIND); it was self-contained: dead `.claude/agents`
  `KNOWN_SURFACES` row removed (rows now settings.json/.mcp.json only), both
  `coverage_note` test doubles rewired to a `widget` kind, `declarations.ts`
  `identityField` loss fixed + byte-verified (builtin_lock_frozen). `genre` hits
  in nested_member.rs / sdk emit tests are removal-assertion guards, not living
  residue. Session-start `+++` kinds/packages fixture debt unchanged (path
  untouched this range) — kept-asymmetry record stays accurate. Cursor → HEAD.
- Queue: 1 — PACKAGING-CHANNELS (parked on human release creds + engine-binary
  workflow). Unchanged this tick.

Plan continues: yes — quiet closing pass (all four jobs above now current; one
closing pass owed before hand-off: queue disjoint, gate reason re-tested,
state re-derived).
