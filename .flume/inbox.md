<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- Extraction-unification tail (source-not-mechanism, the read path): two
  asymmetries remain with no adapter-face justification. (1) SURFACE READING:
  `Workspace` (src/check.rs) loads `skills`/`rules` through per-kind typed IRs
  while custom kinds load generically via `Unit::from_surface_dir` — the
  surface member document is temper's own format, so checking should read every
  kind through the one generic loader (typed IRs remain for the harness
  adapter faces: import parse, apply emit, drift). `main.rs` per-arm dispatch
  collapses with it. (2) IMPORT DISCOVERY: `discover_skill_dirs` is hardwired
  while custom kinds discover off `governs`; the embedded built-in KIND.md
  definitions now carry `governs`, so discovery keys off the declaration for
  every kind. File both; sequence after APPLY-REEMIT-PROJECTION to avoid
  shared-file conflicts.
