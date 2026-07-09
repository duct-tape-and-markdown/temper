## Symptom

RETIRE-DEAD-OWN-PATH-SURFACE-OVERLAY's `entry.files`/`entry.tests` named 8 src
files and 1 test file, but the real blast radius of retiring
`Member::to_document`/`from_surface`/`Unit::from_member_document` touched
~9 more test files: `tests/common/mod.rs` (whose `author_satisfies`,
`author_rule_satisfies`, `surface_unit`, `skill_surface_unit` all
round-tripped through the retired mechanism and are shared by 8 other test
files), `tests/bundle.rs`, `tests/extract_equivalence.rs`, `tests/acceptance.rs`,
`tests/agent_kind.rs`, `tests/command_kind.rs`, `tests/graph.rs`,
`tests/lock_declaration_rows.rs`, `tests/requirement_roster.rs` (4 tests whose
underlying "member-published-requirement-via-own-header" capability no
longer exists at all and had to be deleted, not retargeted). Worst: two
**product-behavior regression tests** in `tests/session_start.rs` — guarding
a real field report ("the inbox false positive") — hand-authored a
`.temper/<kind>/<id>/<doc>` `+++` surface document directly and were not
discoverable by grepping `entry.files`/`entry.tests` at all; they only
surfaced as `cargo test` failures after the src-side removal compiled clean.

## Cost this tick

Large fraction of the tick: reading every test file's helper usage, redesigning
`tests/common/mod.rs`'s `author_satisfies` family into a lock-merge primitive
(`merge_lock`), reordering 6 call sites in `graph.rs`/`lock_declaration_rows.rs`
where `author_satisfies` preceded `write_lock` (a full-overwrite that would
have silently wiped the merged `satisfies` rows), and retargeting the two
`session_start.rs` tests to author `[[declaration.satisfies]]` lock rows
instead of a hand-written surface document — a design decision (not just a
call-site swap) that needed independent verification against
SATISFIES-CLAUSES-RATIONALE-FROM-LOCK's intent before applying.

## Suggested fix

When scoping a "retire mechanism X" entry, `rg` for the mechanism's read-side
call sites is not enough — also `rg` test helper files (`tests/common/*.rs`)
for write-side round-trips through the same API, since a shared test helper's
blast radius fans out silently to every file that imports it. A plan-time
grep for the retired functions' *names* (`to_document`, `from_surface`,
`from_member_document`) across `tests/**` — not just `src/**` — would have
caught all 9 files up front instead of via iterative `cargo test` failures.
