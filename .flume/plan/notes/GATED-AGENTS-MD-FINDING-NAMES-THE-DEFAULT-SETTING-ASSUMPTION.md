# Gated AGENTS.md findings name the default-setting assumption

Shipped. One deviation from the entry's file prediction: `gate.rs` and
`read.rs` did change. SDK-authored custom kinds can declare absence gates
(`sdk/src/kind.ts` `absenceGatesOf`), and those gates have nothing to do
with Claude Code's instruction-files setting. So the assumption belongs to
built-in gates only: `AbsenceClosure::builtin_reason` adds it,
`locus_loader_claim` uses it on its `custom == false` arms, and
`gate::shut_gates` now takes the overlaid built-in map so the reach judge
picks the matching reason. `reason()` is unchanged for custom kinds and for
explain's gate line.
