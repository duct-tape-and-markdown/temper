# gateHooks() satisfies keyed by event

Shipped: `GateHooksInit.satisfies` is a `GateHooksSatisfies` interface keyed
`SessionStart` / `PreToolUse` / `PostToolUse`; each entry becomes that gate
hook's `satisfies` alone, and `defaultPatterns({ gates })` carries it through.

Observed: a hook member carries no `event` field; its event lives only in the
name's prefix (`PreToolUse:Write|Edit|MultiEdit`), so the test reads it there.
The interface spells the three events out rather than mapping `GateHookEvent`,
because the root_exports test requires every named type to be exported and
`GateHookEvent` is not on the patterns subpath or the root.
