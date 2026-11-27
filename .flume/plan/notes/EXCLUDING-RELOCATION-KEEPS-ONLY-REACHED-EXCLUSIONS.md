# Relocation provenance check now reads exclusion reach

Shipped: `keepsExclusionsOf` (sdk/src/declarations.ts) requires only the base
exclusions the relocation's own templates still match or reach beneath, using
the same `templateMatches`/`templateReachesBeneath` test `relocatedLocus` uses
to drop them (both now exported from kind.ts), so the subset check and the
relocation's construction agree. Both behaviors are SDK tests in
sdk/test/builtins.test.ts. The refusal case passes on the pre-fix tree too
(the old check refused every dropped exclusion), so it is a pin by nature. Its
refusal reads "two kinds named `memory`", since only the forged value carries
the marker.

Rust `row_relocates_builtin` (src/compose.rs) does not compare exclusions, so
the engine side needed no change.
