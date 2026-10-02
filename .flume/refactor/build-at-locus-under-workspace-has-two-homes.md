## Surface

The `.temper/`-rooted-`at`-locus invariant is spelled twice, and the two
spellings now disagree on the `local` commitment class:

- `src/drift.rs:1666-1680` — emit's `DriftError::AtLocusUnderWorkspace`, raised
  per payload member from the kind's `governs_root`/`governs_glob` columns.
- `src/admissibility.rs:535-580` — `at_locus_under_workspace_admissibility`,
  the same invariant over an already-committed lock.

Two independent re-spellings of one predicate: each reads the locus itself,
each carries its own `root_path == workspace || starts_with(".temper/")` test
and its own sentence. The divergence this tick exposed is the class split —
the check judge now spares a `Commitment::Local` locus (discovery waives the
`.temper/` fence for exactly a local kind's own walk, `import::local_governs`
at `src/import.rs:424`; emit projects and locks nothing for such a member), but
emit's copy tests the locus *before* its own
`commitment_from_row(facts)? == Some(Commitment::Local) { continue }` at
`src/drift.rs:1694`, so emit still refuses a payload member whose kind declares
a local locus under the workspace. Latent rather than live: the shipped local
kind (`dial`) authors no payload member, so nothing trips it today.

## Observed at

9906e44f (HEAD when observed)

## Suggested consolidation

One home for the predicate — a `kind`-level `workspace_rooted_locus_fault` beside
`local_locus_fault`/`registration_locus_fault` (`src/kind.rs:552-584`), which both
emit and the judge call — so the class exemption and the sentence are decided once.
Hoisting emit's own test below its local `continue` falls out of that move.
