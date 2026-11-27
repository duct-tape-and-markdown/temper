# DUPLICATE-HELPER-FOLDS shipped

All three folds landed; kind.ts untouched (locusPattern was already the home).

- frontmatter.rs: one `opening_delimiter`, the member reader's reading
  (first line `trim_end() == "---"`). `frontmatter_matter` (install/placement)
  now also takes `--- \n`; no test pinned the stricter reading.
- `listItem` in declarations.ts is the one list-item writer; reference.ts imports
  it. Every former `bullet` caller passes non-blank text, so output is unchanged.
- locusLabel/locusGlobs read `locusPattern`. locusGlobs used to also drop an
  empty root; no `at` locus carries `root: ""` (only relocationInvariants'
  erased comparison copy, which never reaches locusGlobs).
