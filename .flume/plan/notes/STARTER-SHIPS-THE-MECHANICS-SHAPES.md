# Starter mechanics shapes: what shipped and what it left

- `@dtmd/temper/starter` exports `gateHooks()`, `typecheck()`, `reference`
  (re-exported, now with a `description` option) and `starter()`. `starter()`
  returns `{ members, typecheck }`, so `harness(starter(opts))` is a whole
  program, and a consumer adds members by spreading.
- `typecheck.project` is the one required option. It is the consumer's path,
  so it has no default. The compiler, the gate tool and the reference
  description are optional and defaulted. A tool both the typecheck and the
  gates use is composed once.
- No counsel-rule shape. The entry summary lists one, but `emit`
  synthesizes the `temper-sdk` counsel rule over every program, and it
  refuses an authored rule of that name. A shape would duplicate it or
  collide with it, so the starter relies on the synthesis. If plan meant
  something else by "counsel rule", that needs a spec decision.
- A hydrated reference description replaces only the trigger. The derived
  "Its contracted domains: ..." sentence still follows it.
- Friction: the reference's `kind-tool` topic reads the lockfile of
  `process.cwd()`. So emitting any harness with a `{ package, bin }` tool and
  a reference throws unless cwd holds a `packageManager`. The starter always
  has both, so its tests chdir into a scratch npm program. A real program
  runs from its own directory, so only tests hit this. Still, the cwd
  coupling is hidden input to a supposedly pure `emit()`.
- Not done (other entries or human work): the patterns (standard, spec
  layout, procedure skill) and their typed options, install scaffolding a
  `starter()` call, `examples/base-harness` folding into the starter, and
  `.temper/harness.ts` composing it (a human `chore(harness):` change).
