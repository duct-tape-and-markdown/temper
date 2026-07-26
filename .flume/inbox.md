<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- Auto memory's two documented settings keys sit in the `settings` kind's
  opaque residue, untyped: `autoMemoryEnabled` (bool) and
  `autoMemoryDirectory` (absolute or `~/`-prefixed path; honored at any
  settings scope; project-scope values gated by the workspace trust dialog).
  The `agent` kind has the same gap — subagents declare their own auto memory
  via a frontmatter `memory` field. All three are documented surface, so 0036's
  coverage argument applies at field grain — "the documented keys typed, the
  genuinely unschematized residue opaque and named as such" — and this repo
  already authors `autoMemoryEnabled: false` as residue
  (`.temper/harness.ts`), so the dogfood is the first consumer.
  (code.claude.com/docs/en/memory, code.claude.com/docs/en/sub-agents,
  retrieved 2026-07-26.) observed at 543c9f1
- Same premise, the half that is NOT a gap — recorded so plan does not
  re-derive it: the auto-memory **documents** are out of scope. They live at
  `~/.claude/projects/<project>/memory/` (`MEMORY.md` index plus agent-created
  topic files), outside the consumer's tree, under a `<project>` derivation the
  docs do not specify — so identity cannot be had the way `settings-local` has
  it (a fixed documented path), and deriving it would be mining (invariant 1).
  Projection is already ruled out twice over: 0034 (emit's codomain is the
  committed tree; a read-side member is never an emit input or target) and
  invariant 3 (the file mutates every session, so no byte-reproducible emit).
  The one strongly decidable clause — `MEMORY.md`'s 200-line/25KB load limit,
  past which content is silently dropped — is now enforced in-product at write
  time (v2.1.210/2.1.211), leaving temper only the stale-file case. If this
  reopens, the condition is a consumer who pins `autoMemoryDirectory` into the
  tree, which is what makes discovery possible at all.
  (code.claude.com/docs/en/memory, retrieved 2026-07-26.) observed at 543c9f1
