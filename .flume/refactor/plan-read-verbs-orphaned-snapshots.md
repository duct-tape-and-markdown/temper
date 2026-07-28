## Surface

`tests/read_verbs.rs` has zero `insta`/`assert_snapshot!` usage anywhere in
the file (`rg -n "assert_snapshot|insta" tests/read_verbs.rs` — no hits; its
tests use plain `assert!`, e.g. `a_requirement_target_walks_the_reverse_roster`
at line 139). Yet `tests/snapshots/` carries eight `read_verbs__*.snap`
files, none of whose name stems appear in the file's current test function
names or macro calls (`rg` each stem — zero hits):
`read_verbs__impact_dev_standards.snap`, `read_verbs__impact_lint_runner.snap`,
`read_verbs__impact_sole_publisher.snap`,
`read_verbs__requirements_custom_satisfier.snap`,
`read_verbs__requirements_engineering_standards.snap`,
`read_verbs__requirements_member_published_detail.snap`.

(Two siblings, `read_verbs__requirements_roster.snap` and
`read_verbs__requirements_member_published.snap`, are the same orphan class
but already scoped into pending entry READ-ROSTER-OVERVIEW-RETIRE this tick —
their content is the dead `roster_overview` branch's exact output shape, so
they ride that entry. Not re-listed here.)

git blame traces the module to `b9a411af` ("build: add the why and
requirements read verbs"), where insta-snapshot assertions were the original
form. The file was since rewritten to plain `assert!` checks without
deleting the now-unreferenced fixtures.

Two readings, and only a human/design pass over the six remaining
requirement_detail/impact test cases can tell which: (a) the plain-assert
rewrite was a deliberate, sufficient replacement and the six files are inert
residue to delete, or (b) the rewrite silently dropped snapshot rigor
(full-output drift coverage) that should be restored — a worse problem than
dead files, since it reads as passing.

## Observed at

db94a7f6

## Suggested consolidation

`rg` each of the six name stems against `tests/read_verbs.rs`'s current
`impact`/`requirement_detail` test cases to find their live counterparts,
then either restore `insta::assert_snapshot!` there (if coverage regressed)
or retire the six files (if plain asserts are judged sufficient) — a
verification call, not mechanical deletion like the roster_overview pair.
