# Plan state

- Spec derived through: 474835b
- Audited through: a112dbe
- Residue swept through: 7ff3f03
- This tick: Ship audit 50e7094..HEAD (job 3). AGENT-KIND shipped (81b6ab4) —
  verified on disk: `claude_code_agent()` in builtin_kind.rs (root
  `.claude/agents`, description-trigger channel), named-field identity
  (`NoId`/`NoNamedFieldId`, `from_source_rooted`) in frontmatter.rs, sdk `agent`
  kind; known-kinds set now complete at agent/command/memory/rule/skill (all
  five spec'd built-ins ship). Tests green. Entry already absent from pending
  (build dropped it) — drop is a no-op. Re-tested PACKAGING park: still true (no
  release.yml, root package.json = private temper-flume-harness, sdk
  @dtmd/temper 0.0.4). No fork rides AGENT-KIND; agents-md fork (memory kind) is
  distinct from the shipped `agent` subagent kind, stays open. Audit cursor →
  a112dbe.
- Queue: 1 — PACKAGING-CHANNELS (parked on human release creds + engine-binary
  workflow). AGENT-KIND shipped & dropped.

Plan continues: yes — residue sweep (cursor 7ff3f03 trails HEAD a112dbe;
81b6ab4/a112dbe not yet swept).
