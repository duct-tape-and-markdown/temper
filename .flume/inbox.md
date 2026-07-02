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

- DIRECTION (human ruling, this session): the built-in/custom split is SOURCE,
  never MECHANISM — end-to-end. The engine reads every member through the one
  generic declared path; a built-in kind/package differs from a custom one only
  in where its definition sources from (embedded product data — packages/ today,
  a kinds/ sibling for the declared KIND definitions — vs project-registered in
  the assembly). This is already the corpus's end state (specs/15-kinds.md:
  "Today extractors are engine code (`src/skill.rs`). The end state is that
  extraction is composed from a closed algebra"; "built-in vs custom is
  ownership, not a privileged mechanism") — the hand-coded `skill_features`/
  `rule_features` in src/extract.rs and the per-kind surface readers are
  transitional scaffolding, now called. Engine code remains sanctioned ONLY at
  the harness adapter faces (parse/emit of the external Claude Code formats).
  SEQUENCING: do not displace HEADER-FIELD EXTRACTION (above) — it ships first
  and IS the generic reader this direction completes. File the unification as a
  derived wave behind it (declared built-in KIND definitions embedded beside
  their packages; generic composed extraction replacing the hand-coded feature
  fns; per-kind surface readers retired into the one loader).

- KIND-EXTRACTION-UNIFY re-scope (the fence revert was correct): `kinds/**` is
  RULED human territory — curated std-lib sources beside `packages/**`,
  citation-bearing, build EMBEDS but never writes (chain.ts NOTE updated). The
  two built-in definitions now EXIST, hand-authored: `kinds/skill/KIND.md`,
  `kinds/rule/KIND.md` — same schema the custom-kind loader already parses.
  Re-file the wave code-only (src/**, build.rs, tests/**), and OPEN WITH AN
  EQUIVALENCE-PINNING ENTRY: insta-snapshot the current `skill_features`/
  `rule_features` output over real fixtures BEFORE any entry swaps the
  implementation, so every later entry diffs against a pinned target. Then:
  embed the kinds/ definitions (build.rs beside the packages embed), drive
  built-in extraction through the generic composed path, retire the hand-coded
  feature fns and per-kind surface readers into the one loader. Adapter faces
  (harness-format parse/emit) stay engine code.
