## Surface

0051's reserved `prose` leaf is refused on both halves, but only one half is
tested. The read half's refusal (`src/layout.rs`'s `LayoutError::ReservedLeaf`,
raised in `read_collection_member`) is pinned by
`tests/layout_kind.rs::a_sub_heading_slugging_to_the_reserved_leaf_refuses_loud`.
The compose half's (`sdk/src/kind.ts`'s `refuseReservedLeaf`, called from
`embeddedMemberValue` over the value's own leaves and each collection entry's)
ships with no test: `sdk/test/refusals.test.ts` — the file that owns exactly
this class of declare-side refusal — was outside
LAYOUT-COLLECTION-MEMBER-OWN-SPAN-LEAF's fence, which listed `sdk/src/kind.ts`
but no SDK test path. An untested throw is a refusal nothing holds in place.

## Observed at

acf68f12 (HEAD when observed) — the guard lands in this tick's commit.

## Suggested consolidation

One pending entry fenced on `sdk/test/refusals.test.ts` alone: two cases —
a top-level `leaves: { prose: … }` and a collection entry's — each asserting
`embeddedMemberValue` throws naming the reserved key. No `src/` change.
