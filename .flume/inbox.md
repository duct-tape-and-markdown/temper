<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- The builtins update ritual is due, under its own spec authority — not
  derived from any horizon doc. `specs/builtins.md` "The clauses live in
  code" sanctions the ritual ("when upstream docs move the update ritual is
  to walk the clauses and re-check their citations") and "The coverage bar"
  sanctions growth by documented capability. Upstream moved (docs verified
  2026-07-15; working reference `docs/market-formats.md` §"Claude Code deep
  audit", source URLs there — re-fetch raw at build time, per clause):
  (a) re-check every `sdk/src/builtins.ts` cite dated 07-07..07-10 against
  the current pages; (b) re-verify `DOCUMENTED_HOOK_EVENTS` against the
  current documented event set (~30); (c) documented-capability coverage
  candidates, each decidable: skill `forbiddenKeys` review against the grown
  frontmatter, agent `tools`-must-resolve (documented loud failure
  v2.1.208+), rule glob-validity (unparseable `[` matches nothing). The
  prior note's routing (d4b9fa8) derived only the recut items; this is the
  remainder under the ritual's own per. observed at 3b8d162
