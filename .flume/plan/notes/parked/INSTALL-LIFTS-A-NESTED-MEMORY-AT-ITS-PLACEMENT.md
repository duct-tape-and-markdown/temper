# Parked: a nested memory has no emit placement to lift toward

The lift half is trivial and works: `lift_base` (src/install.rs) folding against
`drift::fold_base` for every governed kind (the base `compose.rs`
`governed_fold_base` already reads check-side) lifts `apps/web/CLAUDE.md` and
`apps/api/CLAUDE.md` as distinct `apps-web-CLAUDE` / `apps-api-CLAUDE` modules.

The acceptance's second half cannot turn green from install. The first emit
writes **both** members to the root `CLAUDE.md` (observed: root file holds
apps/web's prose; no collision refusal). Cause in src/drift.rs:
`placement_in_unit` → `splice_name` drops the leading `**/` of `**/CLAUDE.md`
and places every file-shaped name at the fixed `CLAUDE.md`. The file-shape fold
joins placement with `-` (`extract::fold_file_id`), which emit cannot invert:
`apps-web-CLAUDE` could be `apps/web/` or `apps-web/`. Path shape inverts only
because `:` is reserved (0090); directory shape the same way.

Needed decision (spec/builtins, not build): how a nested memory is named so
emit (and the SDK's `projectionPath`, gated by tests/projection_path_seam.rs)
can place it. Candidates: memory joins 0090's `:` grammar (`apps:web:CLAUDE`,
placement-invertible, as path shape does), or a module-declared placement.
Also observed debt: two members deriving one projection path emit with no
refusal; one silently wins. Worth its own entry whatever the ruling is.

Once the ruling exists, the install change is the 10-line `lift_base` edit above
plus the test `install lifts two nested memory files as two members`.
