# Emit runs the program twice; second run costs ~0.1 s of ~0.9 s

Measured before the cut (WSL2, debug build, the dogfood `.temper/`): one
`node` run of the program is ~0.09-0.11 s; a whole `temper emit --dry-run`
is ~0.9 s, the declared `tsc` typecheck dominating. The second run adds
roughly 11%, judged acceptable, so the entry shipped rather than parked.

Shape: `emit_program` runs `run_sdk_program` twice and byte-compares the raw
stdout before the version peek, parse, typecheck or any write; a mismatch is
`DriftError::Nondeterministic { path: <harness.ts> }`. The render-side
double-emit in `emit_one` is kept (it is the engine's own check over one
payload). Count pin: `a_deterministic_program_emits_as_today_across_exactly_two_node_runs`
asserts two runs per emit through a program-written counter file.

Observed: `text` template interpolations are mentions, never strings, so a
fixture that stamps the clock has to do it in a plain string leaf (the test
uses a skill `description`). An adopter reaching for `${Date.now()}` in prose
gets a TypeError from `parseHostAddress` deep in the SDK rather than a type
error or a named refusal; the typecheck would catch it under a real `tsc`,
but the runtime crash message is opaque.
