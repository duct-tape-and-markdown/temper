# SDK type errors name their remedy

Shipped: one `Remedy<"fix">` type in `sdk/src/kind.ts`. No value can inhabit it,
and tsc prints its message wherever a value lands on it. The census, each case
pinned in `sdk/test/refusals.test.ts` (section 10) by compiling a snippet against
the emitted `.d.ts` and matching the diagnostic text:

- typed-never fields: `registration` and `collectionAddress` on embedded or
  nested-file kinds; a typed key inside a `residue` bag.
- moved/removed fields: `collectionAddress.manifest`; `hook()`'s flat `type`,
  `command` and `timeout` (new `HookInit`, via a second `Init` parameter on
  `KindDefinition`).
- retired exports: `GUARD_COMMAND`, `SESSION_START_COMMAND` (claude-code face)
  and `maxLines` (root). These are `export declare const`, so they exist for
  tsc only. A program run without tsc still fails at ESM link, as before.

Observed: `KindFacts` discriminates on the nested `locus.kind`, and TS cannot
narrow a union by a nested key. A single union parameter reported a wrong field
against an arbitrary branch (always `nested-file`). `kind()` now has one overload
per locus, and `relocate` calls the shared `defineKind`. Internal readers go
through `registrationOf` / `collectionAddressOf` instead of the erased union.

For plan: the census came from `docs/MIGRATING-*.md`. A future break that
retires an export or moves a field has no mechanism that adds its `Remedy`
declaration, so the census rots unless the release recipe or a pending entry
owns it. Retired declarations also stay on the face with no expiry rule.
