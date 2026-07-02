# Plan state

- **Phase:** reconcile. HEAD 2259667.
- **Last shipped:** REACHABILITY (build 50c5a00). Since then, human `chore` commits
  only: 2259667 added the `paths-match` absence fact to `specs/15-kinds.md` and the
  skill kind's `activation = { via = "description-trigger", field = "description" }`
  line to `kinds/skill/KIND.md` (verified on disk). No `build:` tick since.
- **This tick:** drained the inbox's post-REACHABILITY soundness note into new
  **PATHS-MATCH-ABSENCE** (`open`) — `dead_activation` (graph.rs:375) reads an absent
  `paths` field as a dead edge, but the cited fact says absence ⇒ unconditional
  loading, so a wired pass would false-positive every unscoped rule (law 3); the fix
  is one branch (mark dead only when globs are present) + a fixture. Updated
  **REACHABILITY-WIRE**'s stale "kinds declare no activation" facts (skill now does;
  rule's line waits on this fix) and sequenced it after PATHS-MATCH-ABSENCE. Carried
  the four other entries unchanged (anchors re-confirmed: BUILTIN_KINDS kind.rs:30 =
  ["skill","rule"]; builtin.rs SKILL/RULE_PACKAGE 32/37). Inbox now empty.
- **In flight / pickable:** **PATHS-MATCH-ABSENCE** is `open` and pickable — a pure
  src/graph.rs + tests/graph.rs soundness fix, disjoint from every other entry.
- **Next:** build ships PATHS-MATCH-ABSENCE. Its landing (plus a human resolving
  `reachability-gate-mechanism` and authoring the rule `paths-match` activation line)
  is what un-parks REACHABILITY-WIRE. The remaining five entries stay human-gated
  (curated kinds/packages data, fence-widen, release creds, an unresolved fork).

Plan continues: no — queue reconciled, inbox drained, PATHS-MATCH-ABSENCE is a
pickable `open` entry; hand to build.
