# Plan state

- Spec derived through: 474835b
- Audited through: 50e7094
- Residue swept through: 3e666a5
- This tick: Residue sweep e32ed53..HEAD (job 4). Sweep clean — COMMAND-KIND
  (efd6caa) shipped its own residue in-commit (the `command` KNOWN_SURFACES row
  removed); the two `genre` hits (nested_member.rs:193, emit.test.ts:386) are
  retirement-guard tests, not residue; the session_start.rs `+++`
  `.temper/kinds|packages` fixtures stay tracked accepted debt (rides next
  session-start touch, untouched by COMMAND-KIND). Discovered + fixed a scope
  gap: AGENT-KIND's files omitted the agent-becomes-built-in ripple that
  graduates `.claude/agents` — added src/coverage_note.rs (dead KNOWN_SURFACES
  row), tests/coverage_note.rs (still-flagged + lock_agent_kind doubles break),
  src/builtin_lock.rs (name/floor asserts); parallel to COMMAND-KIND. Re-stamped
  scoped 3e666a5. Residue cursor → HEAD (3e666a5).
- Queue: 2 — AGENT-KIND (open, pickable; 13 edits + 1 new, all resolve, disjoint
  from PACKAGING) → PACKAGING-CHANNELS (parked on human release creds +
  engine-binary workflow). One open entry; PACKAGING file-disjoint.

Plan continues: yes — quiet closing pass (job 5): all cursors current, one
disjointness/gate-reason re-derive remains before handing off to build.
