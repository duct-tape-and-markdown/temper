# Tap synthesis brings the hook kind

Landed in `kindsInPlay` (sdk/src/declarations.ts), not emit.ts: a non-empty
`tapHookRows` admits `hook.facts`. That way the kind-fact row and every other
family derived from kinds in play (assembly rows, guidance rules) see `hook`
the same way they would for an authored hook member. emit.ts needed no change.

Cost: `kindsInPlay` now calls `tapHookRows`, which resolves each telemetry
verifier's tool. Emit already throws on an uncomposed tool, so no new failure
mode, but the tap scan now runs once more per compile. Fine at this scale.
