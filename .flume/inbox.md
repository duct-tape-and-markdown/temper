<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->


- observed at 6a9a25ec (0.0.16 dogfood, this repo) — `temper install` reports
  `applied  post-tool-use hook  ./.claude/settings.json` on every run and
  the file never changes: `install::run` splices the group, then its own
  `drift::emit_program` (install.rs ~493/518) re-projects `hooks.<Event>`
  from the authored `hook` members and drops it, so the placement is undone
  inside the same run and `check` keeps reporting `install.gate-installed`
  (post-tool-use hook missing). Any harness that authors hook members hits
  this for every synthesized placement — the guard and session-start hooks
  only converge here because `.temper/hooks.ts` mirrors their commands by
  hand (the documented dogfood pattern; now extended to the PostToolUse
  Bash hook). Defect: the "applied" line is a claim install does not hold —
  a silent wrong answer. Remedy candidates: (a) install detects that a hook
  kind claims the `hooks.<Event>` address and reports
  `conflicted: emit-owned — author it as a hook member` naming the SDK
  constant; (b) the SDK exports the three synthesized hook commands so a
  consumer authors them without a hand mirror (removes the byte-identical
  comment in hooks.ts). (a) is the fix; (b) is the consolidation.
