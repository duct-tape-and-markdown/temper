<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->
- 0024 read-robustly gap, field-hit migrating the example: a committed
  lock carrying the legacy `templates = ["name"]` string rows is
  **refused at `check`** (`lock kind declaration row column templates is
  not the expected array of tables`) before `emit` can rewrite it — and
  when the corpus also needs source migration (the retired
  `withinHosts`), `emit` fails too, so the refusal is the only exit. The
  normalization is fully mechanical and already exists SDK-side
  (`declarations.ts` maps `(kind) => ({ kind })`): join-time
  normalization per 0024 — read the string row as `{ kind = s }`,
  rewrite canonically on the next emit, refuse nothing. A stranger
  cloning a yesterday-locked harness against today's engine hits this
  cliff with no migration path visible from the error. observed at
  7947235
