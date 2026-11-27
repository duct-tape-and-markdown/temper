# An excluding relocation replaces its built-in

Shipped: `relocationInvariants` (sdk/src/declarations.ts) now erases an `at`
locus's `exclude`, and `isRelocationOf` adds `keepsExclusionsOf`: the base's
exclusions must be a subset of the relocation's, so a value excluding less
than its base is still a third kind and refuses.

Observed, not changed: a relocation that *moves* its locus drops the base
exclusions the new template no longer reaches (`relocatedLocus` in
sdk/src/kind.ts). The subset check refuses that relocation beside its base,
as before this tick. No built-in hits it today (memory's moved relocations
keep all four exclusions), but a memory relocation to a root its exclusions
fall outside would collide with a member built from built-in `memory`. The
tighter check is "every base exclusion the relocation's templates still
reach is kept", which needs kind.ts's `templateMatches` /
`templateReachesBeneath` exported. Worth an entry if the spec's "a relocated
built-in keeps each exclusion its new template still reaches" is meant to
make that pair replace rather than refuse.
