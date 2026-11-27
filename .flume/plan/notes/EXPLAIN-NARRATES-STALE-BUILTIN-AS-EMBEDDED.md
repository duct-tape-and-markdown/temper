# Stale built-in rows now fall back to the embedded row's facts

The fix went in at the mechanism, `retire_stale_builtin_rows` (src/compose.rs). A
retired row's format, unit shape, registration, guidance and cite are now taken from
the embedded built-in lock's row of the same name instead of being cleared. No
src/read.rs edit was needed. `builtin_contract` overwrites the embedded guidance with
whatever the kind row carries, so a cleared row produced a contract with no guidance.
Every verb reading `committed_declarations_from_doc` now sees the embedded facts.

Globs: the stale row still carries the lock's locus (`**/CLAUDE.md` only), because
retirement keeps it as a possible authored relocation. The acceptance test passes
without changing that, so no question was raised.

Observation: `builtin_contract` replaces guidance with a kind row's guidance even when
the row carries none. That covers a same-engine relocation row too. If emit ever writes
a built-in relocation row without the embedded guidance, explain would narrate it with
no guidance line. Unverified: I did not check whether emit writes it today.
