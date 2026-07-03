<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- HUMAN RULING (John, 2026-07-03, relayed by the interactive session): on
  `(scripted-altitude-reconcile)`, ask (b) is DELEGATED TO PLAN — derive the
  floor-migration wave's serialized sequence yourself and file it as
  `blockedBy`-chained pending entries (the floor deltas: manifest as the sole
  gate-read + lock `source_hash`/`emit_hash`, `config.stale`, `import`→`init`
  in-place on-ramp, `apply`→`emit` with double-emit determinism, `re-add`
  retirement + three-state deletion, emit-owned install placements — pure
  Rust, no npm anywhere in it). Ask (a) (the TypeScript SDK / authoring-face
  npm scaffolding) stays PARKED on John — do not file altitude-rung entries
  onto it; the fork stays open for that rung only. Sequence per your own
  entanglement analysis; the build loop runs the chain immediately after
  filing.
