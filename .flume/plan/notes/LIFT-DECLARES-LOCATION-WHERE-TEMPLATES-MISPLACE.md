# Lift declares a location wherever placement misses the read path

Shipped. `lifted_location` (src/install.rs) now compares the read path against
`drift::whole_placement` — the same `placement_through` emit runs with no
location — instead of gating on a `:` in the id. Path-derived `:` ids still
declare (no template places them whole); named-field members now declare
whenever the field does not spell the stem, and a nested agent
(`.claude/agents/sub/x.md`, `**/*.md`) declares too, since its whole placement
drops the subdirectory.

Observed: the lift resolves the emit-side `KindFactRow` by name off
`builtin_lock::declarations()`, because there is no `CustomKind -> KindFactRow`
projection. That holds while the lift reads only built-in kinds; a lift over
relocated/custom kinds would need the effective row instead.
