# Emit placement cedes a subkind's locus

Shipped: `member_projection_path` (src/drift.rs) derives the base's ceded
patterns from the payload's kind rows naming it their `base` (no new
KindFactRow column, no compose thread) and refuses a member placed there as
the new `DriftError::SubkindProjection`, naming base, subkind, member, path.
`ExcludedProjection` keeps the exclusion half only.

Observed: the SDK's `projectionPath` (sdk/src/emit.ts) does not mirror this
refusal (nor the exclusion one) — it computes the path and the engine refuses.
`tests/projection_path_seam.rs` pins agreement on admitted paths only, so no
divergence on success; if the SDK is ever meant to refuse at authoring time,
that is a separate entry.

Derivation duplicates the ceded set `compose::bind_subkinds` builds on
`CustomKind`; emit works over raw rows, so the two derive the same pattern
(`kind::locus_pattern` of the subkind's governs) from the same facts.
