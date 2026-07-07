<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- GENRE-FOLD residual (friction-drained 07-07): nested-member templates do
  not restore from the lock — `KindFactRow` (src/drift.rs) has no templates
  column, `from_kind_fact_row` (src/kind.rs) leaves `templates` empty with a
  comment naming the gap, and the SDK's `kindFactRow()`
  (sdk/src/declarations.ts) filters embedded-template kinds out of the
  composed kind facts. File the schema-delta entry: lock column + SDK emit +
  engine restore, per specs/model/representation.md (kind: nesting template)
  and specs/model/pipeline.md (the lock's declaration rows).
- One-time cite retag (friction-drained 07-07): the kernel recut invalidated
  the whole old cite namespace at once — comments across src/tests/sdk still
  point at specs/architecture/*, specs/intent/00-intent.md, and "law N"
  numbering. The ride-only rule prices scattered staleness, not a corpus-wide
  invalidation: authorize ONE mechanical retag entry (old path/form → current
  home, comment-only, no behavior change), serialized behind any open
  src-touching entry. After it ships, the riding rule resumes as the only
  channel.
