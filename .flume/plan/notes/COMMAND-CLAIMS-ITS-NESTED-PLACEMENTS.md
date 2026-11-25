# COMMAND-CLAIMS-ITS-NESTED-PLACEMENTS

New unit shape `path` (Rust `UnitShape::Path`, SDK `"path"`, lock label `path`):
id = placement under the fold base + stem joined with `:`; placement splits each
`:` back into `/` before the one splice (drift.rs/emit.ts `placement_in_unit`).
`file` keeps its `-` fold; both share `extract::joined_placement_id`.

Beyond the entry's file list (needed, or install regresses):
- `install` lift read every member base-less, so a nested command lifted as its
  bare stem and the first emit would place it flat. `lift_base` now folds a
  `path`-shaped kind against its governs base; `module_stems` names the module
  with `-` for `:` (Windows-reserved), the `name` property keeps the id.
- `tests/it/projection_path_seam.rs` now cites a nested `command:frontend:review`.

Observed debt:
- The same base-less lift bites nested `memory` (`**/CLAUDE.md`, `file` shape):
  every nested CLAUDE.md lifts as `CLAUDE` and ordinals only the module stem,
  so emit would collapse them. Left as-is (not this entry's shape).
- SDK `edgeLookupKey` treats any `:` as kind-qualified, so a bare edge leaf
  `frontend:component` misreads as kind `frontend`; authors must write
  `command:frontend:component`. Same latent shape as hook `Event:matcher` names.
