# Edge targets and requirement kinds are kind values

Shipped: `EdgeField.to`, a relocation's added edges, `Requirement.kind` and the
`requirement` member's `kind` field now take `KindDefinition`s. Lowering to
names happens at the lock seam (`edgeTargetNames`, `sdk/src/kind.ts`), so edge
rows and the built-in lock are byte-identical (builtin_lock_frozen green).

Observed:
- **Self-edges have no first-class spelling.** A kind value cannot name itself
  inside its own initializer (TDZ). It works today through a plain getter,
  `{ field, get to() { return [self] as const; } }`, because `kind()` and
  `relocate` read `to` lazily. A test pins it (`sdk/test/contract.test.ts`) and
  the self-edge fixture in `tests/it/emit.rs` uses it, but no adopter-facing
  surface documents it. Self-edges are common (service depends-on service,
  decision supersedes decision), so this idiom needs a ruling: keep the getter
  and document it, or add a sanctioned spelling.
- Forward references needed reordering in `sdk/src/builtins.ts`: `handler` now
  follows `toolKind`, and `knownMarketplace` precedes `installedPlugin`.
- Breaking for adopters: a string `to:` entry or a string requirement `kind`
  no longer type-checks. The release cut owes a migration line.
- The base-harness example's lock moved only its `kinds.ts` module hash, and
  the gauntlet snapshot only its `harness.ts` hash.
