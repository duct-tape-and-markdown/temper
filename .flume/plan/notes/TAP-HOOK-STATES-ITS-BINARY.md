# Tap hooks state their binary

Shipped. `telemetry(events, { binary })` (sdk/src/contract.ts) states the
binary; the tap handler carries it as `temper:tap-binary` (null = none) and
the engine renders the anchored command via `install::tap_hook_command`
(src/drift.rs `tap_rendered_handlers`).

The key sits on the handler, not the row as `temper:gate-binary` does,
because a tap can join an authored group as a further handler. The renderer
also runs after the gate render, so a tap that joins a gate group renders
too.

Disagreement (open question tap-binary-disagreement): `tapHookRows` throws
when two declarations sharing an (event, matcher) state different binaries.
It compares the raw facts, so an undeclared binary and an explicit
`"temper"` are refused as a disagreement. Defaulting in the SDK would give
the default a second spelling. The ruling may want to relax this.

Dogfood follow-up (human `chore(harness):`): `.claude/settings.json` still
carries the old `temper tap "$CLAUDE_PROJECT_DIR"`. The next `temper emit`
re-renders it to bare `temper`, which is off this machine's PATH. Before
re-emitting, `.temper/harness.ts` should pass `{ binary }` (the gate hooks'
workspace binary constant) to its `telemetry([...])`.

The verifier's lock row (`verifierRow`) does not carry the binary. Its only
wire home is the tap handler.
