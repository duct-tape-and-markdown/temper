## Invariant served

`specs/process/engineering.md`, "An export earns its consumer" (175-177):

> An SDK root export, a `pub` item, a widened `pub(crate)` — each needs
> a caller outside its own module (a test counts). Zero-consumer
> surface is fileable against this section.

This wording counts only in-repo callers, so it has no carve-out for a
symbol re-exported from the package's own public entry point
(`sdk/src/index.ts`). Observed cost at 14719f2: the posture sweep filed
EMIT-NEEDS-ZERO-CONSUMER-EXPORTS-PRUNE against four symbols —
`ResolveOptions`, `edgePlacements`, `renderedExtents` (sdk/src/emit.ts),
`capability` (sdk/src/needs.ts) — as zero-consumer, and a prepared branch
(`flume/emit-needs-zero-consumer-exports-prune`, commit 63cbefe)
un-exported all four accordingly. Two of them — `ResolveOptions`
(re-exported at `sdk/src/index.ts:91`) and `capability`/`Capability`
(re-exported at `sdk/src/index.ts:21-22`) — are public API of a
week-old, pre-adoption library: their only "zero-consumer" fact is that
no downstream caller lives in this repo yet, because there is no
downstream repo yet. John ruled by hand that those two must stay
exported; only the genuinely-internal `edgePlacements`/`renderedExtents`
(never re-exported from `index.ts`) get un-exported. The branch at
63cbefe must not merge as-is. Same shape as the NUL-byte blind spot
already on record: a "no consumer" verdict that actually means "the
consumer is external."

## Diff

```diff
--- a/specs/process/engineering.md
+++ b/specs/process/engineering.md
@@ -173,8 +173,11 @@ scaffolding outlives its scaffold and becomes API someone must
 excavate later. Grep-verifiable, so the sweep holds it mechanically:
 
-- An SDK root export, a `pub` item, a widened `pub(crate)` — each needs
-  a caller outside its own module (a test counts). Zero-consumer
-  surface is fileable against this section.
+- An SDK root export, a `pub` item, a widened `pub(crate)` — each needs
+  a caller outside its own module (a test counts), **or a re-export
+  from the package's public entry point** (`sdk/src/index.ts`) — the
+  entry point is itself the consuming edge for a symbol shipped as
+  public API; a pre-adoption library's downstream callers are real even
+  when none live in this repo yet. Zero-consumer surface is fileable
+  against this section only when neither condition holds.
 - A seam opened for a planned second consumer carries that consumer's
   name at the export site; when the plan dies, the export dies with it.
```

## Expected settling

The next posture sweep over `sdk/src/index.ts`'s re-exports stops
flagging root-re-exported symbols as zero-consumer residue; a sweep
finding against such a symbol instead argues the re-export itself is
wrong (the symbol shouldn't be public API), not that the symbol lacks a
caller. No repeat of a prepared branch un-exporting public API alongside
a genuinely-internal helper in the same entry.
