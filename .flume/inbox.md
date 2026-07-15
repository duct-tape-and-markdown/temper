<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- The built-in typed field schemas lag the documented frontmatter, against
  `specs/builtins.md`'s own claim that "the documented fields that modulate
  them per member are ordinary declared fields" (registration paragraph) and
  the coverage bar's documented-capability rule. `Skill` carries
  name/description/compatibility/paths today; the current skills page
  documents (re-fetch raw at build, per field, for the cite):
  `when_to_use`, `argument-hint`, `arguments`, `disable-model-invocation`,
  `user-invocable`, `allowed-tools`, `disallowed-tools`, `model`, `effort`,
  `context`, `agent`, `hooks`, `shell`. `Agent` carries name/description
  plus whatever shipped 07-07; the sub-agents page documents `tools`,
  `disallowedTools`, `model`, `permissionMode`, `maxTurns`, `skills`,
  `mcpServers`, `hooks`, `memory`, `background`, `effort`, `isolation`,
  `color`, `initialPrompt`. All optional, typed properties with cites —
  fields only, no new clauses (the clause candidates live in the
  `(builtins-coverage-predicates)` fork; do not fold them in). Working
  reference `docs/market-formats.md` §"Claude Code deep audit" for source
  URLs. observed at 4c428b5
