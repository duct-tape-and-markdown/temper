# `hook({ name })` crashes with a raw TypeError before the typecheck can name the rename

Found while drafting the 0.0.26 notes. A 0.0.25 program that still writes `hook({ name: "PostToolUse", … })` or `gateHook({ name })` dies inside the SDK with `TypeError: Cannot read properties of undefined (reading 'indexOf')`. The type's Remedy message ("a hook's lifecycle event is its `event`…") never shows, because a 0.0.25 lock records no typecheck. The program therefore runs before the compiler (0108: the typecheck runs from the last lock, else after the program).

`authoring.md` and the shipped `temper-sdk` guidance say a refusal names the broken input. The hook constructors should refuse at runtime when they get `name` without `event`, with the same words as the Remedy type: write `hook({ event, matcher, hooks })`, or `gateHook({ event, matcher })`.

Acceptance: an SDK test. The behavior is SDK-only, so no `tests[]` line (PROTOCOL, Tests, question 2).
