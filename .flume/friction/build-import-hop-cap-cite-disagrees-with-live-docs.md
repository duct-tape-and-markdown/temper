## Symptom

PREDICATE-SELECTION-ALGEBRA carried a rider: re-verify `src/graph.rs`'s two
`code.claude.com/docs/en/memory` cites (@07-02) against the live docs and bump
the dates, on the premise that "the facts hold". One does not.

- `src/graph.rs:689` (absolute/relative import target resolution) — **verified
  unchanged**, date bumped this tick.
- `src/graph.rs:59-62`, `MAX_IMPORT_HOPS = 5`, cited as "the `at-import` grammar
  is recursion-capped at five hops". The live page (retrieved 2026-07-16) reads:
  *"Imported files can recursively import other files, with a maximum depth of
  four hops."* **Four, not five.**

Left as-is rather than flipped to 4: the two counts may not measure the same
thing (whether the first import is hop 1, or the importing member is hop 0, is
not decidable from the doc sentence), so 5 may be right under temper's own
indexing. Guessing either way edits a live gate's semantics — `reachable`
propagates liveness across the cap, so an off-by-one either suppresses a real
finding (cap too high) or forges one (cap too low).

## Cost this tick

~15 minutes: the fetch, the read, and the judgement call not to flip a gate
constant inside an unrelated entry. No retries, nothing reverted.

## Suggested fix

Inbox item — a human rules on the hop semantics (read the cap's behavior, not
just the sentence) and either bumps the cite alone or lands `5 → 4` with a
reachability test that pins the boundary hop. Until then `MAX_IMPORT_HOPS`
carries a cite that the current docs contradict on its face.
