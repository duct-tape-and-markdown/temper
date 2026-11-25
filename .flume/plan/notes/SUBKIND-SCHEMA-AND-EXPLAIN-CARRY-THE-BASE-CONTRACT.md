# Subkind schema/explain carry the base contract

Shipped `compose::kind_contract_with_bases` (bases' inherited clauses, outermost
first, then own) as the seam `schema` and `explain` read; `check` still dispatches
the two halves apart via `inherited_clauses`, so the halves share one source.

`schema` also needed the subkind in its *domain*: a subkind row inheriting
`yaml-frontmatter` from its base declared no format of its own, so the old
row-lift domain dropped it. Added `compose::effective_kinds` (overlay + custom +
`resolve_subkinds`, no discovery walk) and `schema` now filters that set; the
private `yaml_frontmatter_kind_domain` in main.rs is retired.

Observed, not fixed: `explain` never applies joined (layer) clauses to a kind's own
contract while `check` does; explain runs with no layers, so it is inert today,
but the seam passes `&[]` for joined and a layer-aware explain would need it.
