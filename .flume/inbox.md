<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- `(agents-md-builtin-kind)` ruled 07-15: no claude-code AGENTS.md kind —
  Claude Code does not read AGENTS.md (documented bridge is `@AGENTS.md`;
  code.claude.com/docs/en/memory, retrieved 2026-07-15), so the kind's
  consumer is the cross-tool story, now inherited by
  `(multi-harness-projection)`. Work: delete the engine std-lib's
  hand-written `agents-md.memory` remnant to match the lock (the fork's own
  spec-faithful default); the SDK's orphaned `memoryAgentsMdDefaultContract`
  goes with it or stays as documented honest encoding — build's call, say
  which in the commit. observed at 37e6844
- `(directory-sliced-governance)` ruled 07-15: a multi-segment glob on a
  flat file kind is a **loud refusal**, never a derived path with a literal
  `*` in it (`member_projection_path`, `src/drift.rs:513` replaces only the
  first star — silent nonsense today). The refusal names the two sanctioned
  shapes: a skill for agent-loaded depth (the centercode pack trial's
  resolution — a pack is a skill), a nesting kind for governed graph
  content (`model/representation.md` "a template per inner layer" — built
  when demand arrives, not now). observed at 37e6844
- `(layout-kind-heterogeneous-corpus)` ruled 07-15 by the representation.md
  amendment: a layout's regions state what may appear, never what must — a
  document conforms with a region empty; floors are clauses; two kinds
  never share a governs glob (position types, content never routes). Work:
  verify the engine's layout reader tolerates an empty region today (test
  it — a prose-only document under a prose+collection layout must read
  green with zero members), align if not, and tests either way.
  observed at 37e6844
