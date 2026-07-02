# Plan state

- **Phase:** reconcile. HEAD 76dcb04.
- **Last shipped (trunk):** EXTRACT-EQUIVALENCE-PIN — the built-in skill/rule
  extractor output is pinned as `tests/extract_equivalence.rs` snapshots, the
  equivalence baseline the swap in BUILTIN-EXTRACT-GENERIC must hold byte-identical.
- **This tick:** verified on disk — `kinds/{skill,rule}/KIND.md` authored,
  `build.rs` walks `packages/` only (no `kinds/` walk), no `src/builtin_kind.rs`,
  `skill_features`/`rule_features` still at `extract.rs:238/296` and called in
  `main.rs`. Sole change: EMBED-BUILTIN-KINDS's now-shipped `blockedBy
  EXTRACT-EQUIVALENCE-PIN` flipped to `open`. Inbox empty; no forks moved.
- **Pickable now:** EMBED-BUILTIN-KINDS (open). BUILTIN-EXTRACT-GENERIC serializes
  behind it (`blockedBy`, no two open entries share a file). AGENT-KIND deferred;
  PACKAGING-CHANNELS / COMMUNITY-DOCS parked. Sole live OPEN fork:
  (edge-representation-unify).

Plan continues: no — EMBED-BUILTIN-KINDS is pickable; hand to build. The wave
serializes one entry at a time; re-planning would only re-emit the same queue.
