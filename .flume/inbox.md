<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- 2026-07-02 (human): SCAN-QUALIFIED-IDENTITY (blocks the memory-kind file
  commit; found by placing the four curated files locally). The generic scans
  iterate `builtin_kind::definitions()` (qualified keys) but then RE-RESOLVE
  each kind by its BARE name: `import_frontmatter_kind` calls
  `builtin_kind::definition(name)` at src/import.rs:178 and :222-223, and the
  drift path does the same at src/drift.rs:1030. With `claude-code.memory` +
  `agents-md.memory` co-embedded, that bare `memory` re-resolution throws
  AmbiguousKind and reddens 19 import/drift tests — even though no caller ever
  *referenced* bare memory. The scan already holds the parsed kind (or its
  qualified key) from definitions(); carry THAT through instead of
  re-resolving. Repro: drop any second `memory` KIND.md under kinds/<other>/
  and run cargo test. ALSO in this slice: the enumeration test
  `definitions_carries_exactly_the_two_built_in_kinds`
  (src/builtin_kind.rs:278) hardcodes the two-kind set and breaks on any new
  embed — re-pin it to derive its expectation from the kinds/ product tree
  (embeds == tree, robust to curated additions) rather than a literal list.
  Acceptance: with a synthetic or test-only second bare-name kind co-embedded,
  import/drift/check paths stay green and only a caller binding/referencing
  bare `memory` gets the collision. After it drains, the human commits the
  four held curated files (two memory KIND.md + two PACKAGE.md).
