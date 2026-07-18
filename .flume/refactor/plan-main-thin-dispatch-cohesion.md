## Surface

`src/main.rs`'s own module doc (1-6) states the file is "a thin `clap`
dispatch over the [`temper`] library: parse args, run the generic contract
engine, map the result to an exit code... all logic lives in the library so
`tests/` can drive it." `.claude/rules/rust.md` ("Style & structure") and
`specs/process/architecture.md`'s verbs codemap ("`main` (CLI dispatch —
thin...)") plus its Invariants section ("`main` carries dispatch only —
corpus assembly and judgment live in the library") declare the same
contract. The file (2762 lines) does not hold it: 47 free functions beyond
`main` itself carry corpus assembly and judgment, not dispatch — the two
verb bodies (`gate` 847-1223, `explain` 513-641), clause/edge construction
from lock rows (`clause_from_row` 2485, `edges_from_declarations` 2502,
`requirement_from_row` 2457, `mention_edges_from_declarations` 2542,
`import_edges_from_lock` 2566), and admissibility judges
(`joined_kind_admissibility` 1738, `satisfies_label_admissibility` 2034,
`nested_member_admissibility` 2074, `local_locus_admissibility` 2341,
`governs_collision_diagnostics` 2394, `clause_collision_diagnostics` 2299,
`kind_collision_diagnostic` 2263) among many others (`resolve_kind_units`
1319, `manifest_units` 1498, `kind_features` 1521, `assemble_lock_family`
1582, `assemble_by_kind` 1862...).

None of this is reachable from `tests/*.rs` (the external integration
suite links only against the `temper` library crate, per `rust.md`: "`main.rs`
is its own binary crate, not a module of the library"). `tests/cli.rs`'s own
doc comment names the one legitimate reason it drives the binary as a
subprocess — "the exit code is observable only across a real process
boundary... `process::ExitCode` is surfaced by `main`, not returned by the
library" — a narrow justification for exit-code assertions, not for the
~2000 lines of judgment logic itself, which is unit-tested (where tested at
all) only via main.rs's own internal `#[cfg(test)] mod tests` (2582, 7
tests) — invisible to `tests/`. Many of these functions are already
independently targeted by chained pending entries for their own internal
redundancy (double-parses, non-exhaustive matches — DRIFT-SOURCE-DEP-PARSE-
HOIST, GATE-KIND-UNITS-DOUBLE-RESOLVE-HOIST, GRAPH-RESOLVED-EDGE-WALK-
CONSOLIDATE, COVERAGE-NOTE-LOCK-PARSE-HOIST...) without any of them
addressing that the functions sit in the wrong crate at all.

## Observed at

285f57b (HEAD when observed) — plan diffs forward from here.

## Suggested consolidation

No single mechanical move fits: each function's right library home differs
(the clause/edge builders look like `compose.rs`/`graph.rs` territory, the
admissibility judges look like `engine.rs`/`graph.rs`, `gate`/`explain`
themselves may want a new pipeline-facing entry point the library exports
and `main.rs` merely calls) — same "which module" judgment call
architecture.md's Growth rules names for a cohesion split ("the codemap
gains a name, the tree gains a file"). This is sized like
READ-VERB-STRAND-COHESION, not like a single hoist/prune entry: plan's next
pass on this capture should scope it as a human-ratified split (or an
explicit affirmation that `main.rs`'s current shape is accepted, named
cost), not a single buildable pending entry.
