# Locus membership has one home

Shipped `kind::locus_admits(pattern, excluded, path)` with `CustomKind::locus_admits(path)` beside it.
`CustomKind::excludes` is gone. Its four inline callers (import's discovery retain, and the guard's
pending-write, layout-write and post-edge site enumerations) now ask the predicate. Emit's
`member_projection_path` and the coverage advisory's `governs` ask it too.

Emit gets a new refusal for an excluded placement, `DriftError::ExcludedProjection`
(`temper::drift::excluded_projection`). It is kept apart from `UngovernedProjection` because the
repair is different: rename the member or drop the exclude, not fix the glob.

Observed, not taken:
- The nested-file placement check (`refuse_ungoverned` at the host-composed path) still matches the
  glob alone. A nested file kind has no `governs`, so there is no `exclude` to read today. If host
  templates ever carry exclusions, that site should read the predicate.
- Import's retain now re-matches the pattern over files the segment walk already matched. That is
  cheap (compile_glob is memoized) and only runs when a kind declares `exclude`. It relies on the
  stated equivalence between the walk and compile_glob.
