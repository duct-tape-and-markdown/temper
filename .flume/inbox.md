<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- 2026-07-02 (human): DIRECTIVES wave — three slices, dependency-ordered, per
  the new Decision (specs/architecture/15-kinds.md, "Directives are observed
  structure — execution is the admission test"; 45-governance's observed-
  format-edges cut and reachability closure). Curated memory KIND.md files
  adopt the `[[extraction]]` directive lines ONLY after slice 1 ships (embeds
  must parse — the standing red-interim trap); that adoption is the human's.

- 2026-07-02 (human): DIRECTIVES-PRIMITIVE-PARSE (slice 1): the extraction
  algebra gains the `directives` primitive with a closed per-syntax vocabulary
  — sole member `at-import` (`@path/to/file` occurrences; grammar and
  citations live in the spec section and kinds/claude-code/memory/KIND.md
  prose). Extraction yields the member's directive occurrences (target path
  strings, order-stable) as features; unknown syntax values are load errors
  like every closed vocab. Parse + adapter face + unit tests; no gate wiring
  yet.

- 2026-07-02 (human): DIRECTIVE-TARGET-CLASSING (slice 2, after 1): resolve
  each extracted target at check time against the landscape — provenance
  `source_path` is the member join key; disk is the world's. Three classes
  per the spec: member (edge enters the relation graph), backed world file
  (boundary edge, fine), nothing (UNBACKED POINTER — a finding on the
  importing member; the headline check: "your CLAUDE.md imports
  @docs/conventions.md which doesn't exist"). Relative targets resolve
  against the importing file's directory per the cited docs.

- 2026-07-02 (human): REACHABILITY-DIRECTIVE-CLOSURE (slice 3, after 2):
  `graph::reachable` closes over member-target directive edges — a member is
  reachable iff its own activation is live OR a reachable member imports it
  (importer-liveness inherited conditionally; hop-capped per the format's
  docs). Fixes the known false-positive: a zero-match `paths` rule imported
  by an always-live memory member is live, not dead.
