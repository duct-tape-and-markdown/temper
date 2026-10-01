## Surface

The member-address grammar has two homes, and only the engine's now reads
parity. `src/member_address.rs:segment` cuts at every `/` and lets the count
decide the grain (odd member, even leaf), so the engine reads a member address
at any depth. `sdk/src/member-address.ts:segment` (~line 200) still cuts
`parts.slice(3).join("/")` — fixed three segments plus "the remainder", so
`parseNestedAddress` refuses a five-segment member address and
`parseLeafAddress` reads it as a leaf whose child path carries a `/`. Its own
module header states the seam bar it now misses: "a spelling moves on both
sides at once or the seam breaks."

Nothing ships broken today: `memberAddress` host-qualifies with a host address
only (`emit.ts`'s `hostUnit` refuses a deeper host), so no SDK writer spells a
deeper address for either reader to disagree over. The divergence is latent,
and it bites the first corpus that nests two layers.

## Observed at

HEAD of `flume/primary/address-grammar-reads-any-depth` (the parity re-cut
itself) — `sdk/**` is outside that entry's fence, so the TS half could not move
with it.

## Suggested consolidation

Re-cut `sdk/src/member-address.ts`'s `segment` on the same parity rule and
carry the host's own grain into `NestedAddress` as the engine's
`host_identity` does, with `sdk/test/member-address.test.ts` reading both
grains at five and six segments. One grammar, two ends, one rule.
