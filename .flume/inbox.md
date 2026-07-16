<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- sdk export gap: `ResolvedEmbeddedMemberValue` — the `render` hook's
  parameter type — is exported from neither the package root nor the
  claude-code subpath, so a consumer factoring posture kinds through a shared
  helper (rather than inlining every hook) must recover it as
  `Parameters<NonNullable<KindDefinition<object>["render"]>>[0]`. Export it
  beside `EmbeddedMemberValue` (and its `ResolvedEmbeddedMemberCollectionEntry`
  companion). observed at 1baeedd

- field (centercode): the posture vocabulary landed end to end on the shipped
  surface — five consumer embedded kinds with render hooks, every body in the
  corpus recomposed as posture-typed blocks; emit and check green, the
  required incoming-degree clauses hold over leaf-lifted cite mentions, and
  the lock carries per-host `templates` rows plus leaf-addressed mention rows
  (`connectdb/consult/sql-procedures/cite` → `skill:sql-procedures`). Three
  hand-rolled shims in the consumer program mark the 0025 reconciliation's
  demand, in priority order as felt: (1) a `citePath()` helper deriving
  citation displays from the target's locus facts — format-placed derived
  target facts would delete it; (2) `withinHosts` lists rebuilt per posture —
  the host-agnostic admission declaration replaces them; (3) posture budgets
  (orientation/directive/step line caps, the `count({of: directive})` rule
  budget) are inexpressible — no clause ranges over embedded members, so the
  posture kinds sit in `expect` as bare in-play bindings. observed at 1baeedd
