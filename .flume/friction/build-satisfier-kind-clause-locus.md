## Symptom

SATISFIER-KIND-CLAUSE's `entry.files` named `src/builtin.rs` for "where the
requirement kind's shipped default contract is constructed" and `entry.tests`
for `tests/lock_declaration_rows.rs` asserted the sourced clause "appears as a
declaration row in the lock." Neither matches anything on disk: `builtin.rs`
only lifts *kind-floor* clause rows (skill/rule/memory), and there is no
built-in `requirement` kind at all — so "the requirement kind's shipped
default contract" reads like a kind that doesn't exist yet. Reconciling this
cost real effort: reading `compose.rs`, `main.rs`'s `requirement_from_row`/
`clause_from_row`, `drift.rs`'s `RequirementRow`/`ClauseRow`, and the SDK's
`contract.ts` to rule out a lock-format or SDK change before committing to
the actual design (a `builtin::kind_narrowing_clause(kind)` helper, called
fresh per `roster::check` run from `Requirement::kind` — never stored as a
lock row or lifted through a `ClauseRow`).

## Cost this tick

~20 minutes of cross-file reading (compose.rs, main.rs, drift.rs, sdk/src/contract.ts)
before ruling out a lock/SDK-side synthesis path that the entry's wording implied
but the codebase had no hook for.

## Suggested fix

When an entry's `files`/`tests` cite a locus or format that doesn't exist yet
(a "requirement kind" with no `CustomKind`, a "declaration row" the lock schema
has no column for), plan should flag it as a derived/synthesized mechanism
explicitly, or route it as an open question — "sources a clause" is ambiguous
between "stored in the lock" and "computed at check time," and only one of those
touches `drift.rs`/the SDK.
