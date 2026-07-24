## Symptom
bc6f944 gated broken intra-doc links via `cargo doc --no-deps` (afterMerge),
but that flag only checks links inside *public* items — most of this
crate's `fn`s/`struct`s are private and slip the gate. A same-day plan-phase
attempt (47cfa9d, reverted for a writable-paths violation: touched
`.flume/chain.ts` + 6 `src/` files, outside plan's fence) tried to extend
the gate with `--document-private-items` and fix the 7 links it surfaced.
Those 7 fixes were correct but never landed — reverted along with the path
violation. Re-probing `cargo doc --no-deps --document-private-items --quiet`
at HEAD confirms all 7 are still broken: src/compose.rs:1129
(`edges_from_declarations`), src/drift.rs:2100 (`read_prior_provenance`,
renamed to `_from_doc`) and :4112 (`crate::main`), src/glob.rs:7
(`CustomKind::owns_source`), src/graph.rs:480 (`DegreeBound`, a removed
type), src/placement.rs:8 (`NOTE_COMMENT`), src/read.rs:918
(`context_member`, renamed `context_member_impl`). The probe also surfaces
an 8th issue 47cfa9d never touched: src/graph.rs:947 links `` [`unreachable`] ``
ambiguously (the sibling fn at graph.rs:822 vs the std macro) — rustdoc
suggests `` [`unreachable()`] `` to disambiguate to the fn. So even a clean
re-application of 47cfa9d's diff would still fail the gate on this one.

## Cost this tick
None direct — this tick didn't attempt the edit, only probed and verified.
But 47cfa9d's verified-correct 7-link fix is stranded in git history
(reverted, not on HEAD) and gets re-derived from scratch unless someone
picks this note up.

## Suggested fix
A chore(harness) commit, same class as bc6f944: (1) add
`--document-private-items` to the `cargo doc` gate args in
`.flume/chain.ts:235`; (2) re-apply 47cfa9d's 7 link fixes (`git show
47cfa9d` — every hunk is still correct at HEAD, none drifted further);
(3) additionally disambiguate src/graph.rs:947's `` [`unreachable`] `` to
`` [`unreachable()`] `` — new since 47cfa9d, first caught by this probe.
Verify with `cargo doc --no-deps --document-private-items --quiet` before
landing.
