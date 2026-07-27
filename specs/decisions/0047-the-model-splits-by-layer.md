# 0047 — the model splits by layer

- **Date:** 2026-07-27 · **Status:** accepted

## Context

`pipeline.md` stood at 253 lines against spec-system's ≤150 model-file
budget. A subtraction re-cut (e81baa47) evicted everything the budget's own
diagnosis names — inline decision references, rejection residue, mechanism
code owns — and bottomed out at 210: the remainder is contract, defensible
line by line. So the budget stopped explaining the overrun. The file was a
bundle — its own title named five nouns (SDK · emit · lock · drift ·
install), and Layers, Telemetry, and Read verbs had accreted since —
where spec-system's form was always one file per layer.

## Decision

**The model splits by layer.** Three files: `authoring.md` (The SDK,
Layers — how the harness value is written and stacked), `pipeline.md`
(Emit, The lock, Drift, Telemetry, Read verbs — how the value becomes
files and stays true), `adoption.md` (Install — the one on-ramp). Every
section keeps its name — the addressable grain — so a citation moves file,
never section, and the two most-cited addresses ("Emit", "The lock") plus
"Telemetry" do not move at all. The ≤150 budget binds each file, unchanged:
the bar held; the file was mis-drawn.

## Rejected

- **Amend the budget upward**: a budget that rises the first time it binds
  stops forcing eviction, and `contract.md`'s 171 would be legalized
  without ever earning its own re-cut.
- **Accept the overrun as known debt**: a stated bar that visibly does not
  bind is corpus rot — deferral wearing resolution's name.
- **One file per verb**: shatters the compile core's unity — emit, the
  lock, and drift are one system (one compile, one anchor, one comparison)
  and read as one.

## Consequences

Six citation sites re-address in the same commit: `contract.md` ×2,
`representation.md` ×1, the memory document behind `CLAUDE.md` ×1, and two
code comments (`sdk/src/dial.ts`, `tests/prose_include.rs`) whose spec-path
pointers the comment taxonomy retires outright — cut, not re-pointed.
Decision records keep their original spellings, append-only. `contract.md`
remains over budget and owes the same subtraction pass pipeline.md got
before any further ruling on it.
