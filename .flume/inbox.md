<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- DECLARED-FRONTMATTER-ADAPTER sequencing correction: the parked entry's
  un-park condition ("human adds format/unit lines, then it un-parks") breaks
  main in the interim — `CustomKind::from_header` REJECTS unknown header keys,
  and the built-in KIND.md parse is compiled-in, so curated lines added before
  the parser accepts them turn every check red. File a small FIRST entry:
  FORMAT-KEY-PARSE (src/kind.rs + tests only) — parse optional `format`
  (string, closed vocabulary, first entry "yaml-frontmatter") and unit-shape
  keys on a kind definition into typed fields, inert until the adapter
  consumes them, absent keys remain valid. After it ships, the humans add the
  two curated lines (surfaced, chore commit), and DECLARED-FRONTMATTER-ADAPTER
  un-parks with its declarations present and main green at every step.

- Post-wave, directional: MEMORY-KIND — the first pure-data managed kind and
  the recipe's proof. `kinds/memory/KIND.md` + `packages/memory.anthropic/`
  (curated, humans author, cited to code.claude.com/docs/en/memory): markdown,
  no frontmatter, dual root locus per specs/15-kinds.md. Its `@path` imports
  are format-EXECUTED reference syntax (law 8's explicit carve-out, specs/
  00-intent.md) — a deliberate extraction-vocabulary addition with a real
  consumer (the EXTRACTION-VOCAB-GAPS revival condition). Unlocks the
  CLAUDE.md flip ceremony. Do not file until DECLARED-FRONTMATTER-ADAPTER
  drains.
