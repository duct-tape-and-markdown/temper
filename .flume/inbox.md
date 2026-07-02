<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- HEADER-FIELD EXTRACTION for custom kinds: the generic custom-member loader
  (`Unit::from_surface_dir`, src/kind.rs) parses `satisfies` + provenance off
  the `+++` header but skips `[clause.<field>]` tables — frontmatter is left
  empty, so field predicates (`required`, `type`, `allowed_chars`, ...) have
  nothing to range over on a custom member even though the surface language
  defines those tables for every kind (specs/20-surface.md, the header clause
  list). Populate the unit's field locus from the header's clause tables (the
  `value = ...` per field), exactly what the built-in adapters do with their
  hand-written parsers. No new predicates, no new grammar — closing the
  built-in/custom asymmetry so a spec's declared header fields are checked by
  the same path as a rule's `paths`.
