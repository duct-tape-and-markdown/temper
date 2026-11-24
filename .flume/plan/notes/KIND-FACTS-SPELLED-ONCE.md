# KIND-FACTS-SPELLED-ONCE: shipped, nested-file included

- Dropped `registration` from the nested-file variant too, not only embedded:
  it carried the same unbound `readonly []`, so the one-spelling rule covers it
  the same way. Both variants now type it `registration?: never`, and spelling it
  is a tsc error (`builtins.test.ts`). `declarations.ts` lowers `?? []`.
- `at`-locus kinds that declare `registration: []` (settings, mcp, dial, test
  fixtures) keep it: there the empty set is a real declaration, not derived.
- `LayoutRegion`/`Layout` are now generic over the kind's surface `T`; a field
  `slot` is `keyof T & string`. Because `keyof` reads contravariantly, `kind()`
  widens `KindFacts<T>` to the erased `KindFacts` with one commented assertion.
  A `relocate()` with `governs` re-states `registration: relocated.registration ?? []`
  so the `at` branch type-checks (only `at` bases reach it).
- Fixtures with a field slot (`gauntlet.rs`, `emit.rs`, `emit.test.ts`) now type
  `kind<{ intent?: string }>`. Rust fixture programs are type-stripped, never
  tsc'd, so this keeps them honest without the gate enforcing it.
