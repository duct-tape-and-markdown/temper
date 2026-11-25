# Nested memory lifts as `apps:web:CLAUDE`; `file` and `path` shapes now coincide

Shipped as plan's architecture: memory stays `unit_shape = "file"`, and the
file-shape fold (`extract::fold_file_id`) joins placement with `:` instead of
`-`. Emit (engine `drift::placement_in_unit`, SDK `placementInUnit`) turns `:`
back into `/` for file shape as it already did for path, plus one new case: a
`**/`-led pattern with a `*`-free leaf (`**/CLAUDE.md`) lands the name's
leading segments as directories. `install::lift_base` folds every governed
kind against `drift::fold_base`, not just path/directory.

Why not make memory `path`-shaped: the dogfood's `.temper/lock.toml` pins
memory's row with `unit_shape = "file"`; a diverging shape is a
`kind.admissibility` collision, so `self_host_check_is_clean_over_tempers_own_surface`
goes red and `.temper/` is outside the build fence.

Observed, for plan:
- `UnitShape::Path` and `UnitShape::File` are now derived and placed
  identically (only the label differs). A fold candidate: retire `path`,
  command's row becomes `file`. Needs a dogfood re-emit only if the dogfood
  ever declares a command row (it does not today).
- Rule identity moved too: a nested `.claude/rules/sub/x.md` is now
  `rule:sub:x` and emits back at `sub/x.md` (before: `sub-x`, emitted at a
  different file, `sub-x.md`). Same for nested file children (`deep:home`).
  No decision names rule's nested grammar; 0096's title ("one identity
  grammar") is the reading taken. Plan should confirm or file a question.
- Adopter break: a lock holding a `-`-folded nested file-shaped id re-keys.
- Still open from the prior park: two members deriving one projection path
  emit with no refusal.
