## Symptom
The v0.8 `pendingGate` builtin (recommended adoption, MIGRATING-0.8 §3)
validates + fence-checks **every** pending entry unconditionally. This
chain's hand-rolled fence preflight deliberately exempts `parked`/`deferred`
entries — plan must be able to park work whose declared paths sit outside
today's build fence while the human decides whether to widen `chain.ts`
(the gate's own failure message names that exact workflow). Under
`pendingGate`, plan could not even *record* such an entry: the commit that
parks it reverts. No `PendingGateOptions` knob filters by pickability, so
the builtin cannot replace this chain's gate without a workflow regression.

## Cost this tick
Small this session — the adoption was evaluated and declined during the
0.6→0.8 migration (`entryFenceGate` kept, now extension-aware). The ongoing
cost is carrying a hand-rolled twin of a builtin whose only delta is one
predicate: every future divergence between the two is ours to notice.

## Suggested fix
Route to the flume repo's inbox: give `PendingGateOptions` a pickability
filter — `pickableOnly: boolean` or `fenceWhen?: (entry) => boolean` — so
chains that fence only pickable entries can adopt the builtin and retire
their forks.
