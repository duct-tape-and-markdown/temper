# REVIEW-IS-A-VERIFIER-SPECIES shipped

`Verifier::Review { prose }` lands on both seam halves: the lock sub-table is
`{ species = "review", prose = "..." }`; roster admits it with nothing to
resolve; SDK `review(prose)` refuses blank (whitespace-only) prose. The SDK
row builder (`sdk/src/declarations.ts` `verifierRow`) also needed the arm:
it was a two-way ternary, now an exhaustive switch.

Observed: no read verb renders a verifier's species today (read.rs only
matches Telemetry for the field strand), so `explain` does not narrate a
review's prose. The spec's "unheld" reading (no verifier declared) is a
separate entry's concern if not already queued. The Rust reader accepts a
blank review prose off a hand-edited lock; the lock is tool-written, so no
refusal was added there.
