# EXPLAIN-NAMES-WHAT-HOLDS-A-REQUIREMENT shipped

`read::held_by` (src/read.rs) adds one line under the coverage state in
`explain`'s requirement narration: `Held by review: "<prose>".`,
`Held by script: `<path>`.`, `Held by telemetry over `<E>`, ….`, or
`Unheld — no verifier is declared …`. It is narration only: the tests
check that `coverage::check` gives the same findings with verifiers
present or stripped, and that an unheld requirement raises no finding.

Still open (spec "requirement" section): "the emitted reference say so".
Nothing in emit's generated reference names a verifier or unheld yet.
That needs its own entry if it isn't queued already. The member-side
`narrate_filled` (a member's `satisfies` walk) doesn't name the verifier
either. This entry left that alone because it covers the requirement
target only.
