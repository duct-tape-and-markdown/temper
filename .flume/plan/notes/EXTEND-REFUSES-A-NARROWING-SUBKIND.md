# extend types its base surface; callers spell both type arguments

Shipped as named: `extend<B, T extends B>(base: KindDefinition<B, any>, delta)`.
Two things plan should know:

- `T extends B` alone admits a surface that drops an *optional* base field
  (one without `paths` is still assignable to `Rule`). The delta is also
  intersected with `KeepsBaseSurface<B, T>`, a keyof check that resolves to a
  `Remedy` when a base key is missing. It is root-exported (the
  root_exports test requires that).
- TS has no partial inference, so every caller now writes both arguments
  (`extend<Rule, Standard>(rule, …)`). `B` is checked against `base` only
  through `KindDefinition`'s call signature, so a caller who passes a
  too-narrow `B` (e.g. `{}` over a kind whose fields are all optional) weakens
  the check. Making `B` inferred would need a curried form or an invariant
  phantom on `KindDefinition`. That is a design call, so I did not take it.

`relocate` passes its delta through `extend<any, T>` with a cast, because a
relocation adds no plain field and keeps the base surface by construction.

The counsel text changed, so the committed example projection, its lock and
the gauntlet snapshot were re-emitted. The dogfood `.claude/` projects no
`temper-sdk` rule, so nothing drifts there. The type tests sit with the other
extend tests in `sdk/test/builtins.test.ts` instead of contract.test.ts.
