<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- 2026-07-02 (human, post-REACHABILITY review): soundness gap in `paths-match`
  dead-edge semantics — `dead_activation` (graph.rs) treats an absent/blank
  glob field as zero globs ⇒ dead, but the cited harness fact says absence
  means unconditional loading ("rules without a `paths` field are loaded
  unconditionally", code.claude.com/docs/en/memory, retrieved 2026-07-02) — a
  wired pass would false-positive every unscoped rule (law 3). The spec now
  carries the absence fact (`15-kinds.md`, paths-match bullet, this commit).
  File PATHS-MATCH-ABSENCE: absent/blank field ⇒ live (one branch in
  `dead_activation` + a fixture test proving an unscoped rule passes and a
  present-but-zero-match rule still fires). Sequencing: it must ship BEFORE
  REACHABILITY-WIRE un-parks. Curated lines: kinds/skill/KIND.md now declares
  `activation = { via = "description-trigger", field = "description" }` (this
  commit — one real activation for the wire to gate); the human adds
  kinds/rule/KIND.md's `paths-match` line only after PATHS-MATCH-ABSENCE
  drains.
