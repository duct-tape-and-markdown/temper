## Surface

`extract::Features.published_requirements` (`src/extract.rs:328`) is now
populated **unconditionally empty** everywhere it is constructed
(`src/kind.rs`'s `Extraction::extract`, `src/main.rs`'s `resolve_kind_units`),
because `RETIRE-DEAD-OWN-PATH-SURFACE-OVERLAY` removed `kind::Unit`'s own
`published_requirements` field (its sole non-empty source, the retired
own-path surface overlay). Nothing else can ever populate it: a custom-kind
member has no other authoring channel for a `[requirement.*]` demand, and
`main.rs`'s `union_published_requirements` (the one reader that folded it
into the roster) is gone too.

Its one surviving consumer, `read.rs`'s `impact` (`sole_publisher`,
`src/read.rs:~1237-1259`, the "demands it alone publishes" blast-radius
narration), can now never fire that branch for a real harness — dead in
practice, but still live code with its own test coverage
(`tests/read_verbs.rs` builds synthetic `Features` with non-empty
`published_requirements` directly, bypassing `Unit` entirely, so those tests
stay green and mask the fact that no real production path reaches this
branch anymore).

## Observed at

f9cc899

## Suggested consolidation

Either (a) retire `Features.published_requirements`,
`document::PublishedRequirement`'s use in `extract.rs`, and `impact`'s
`sole_publisher`/dangling-demand narration together as a follow-on entry
(citing this same section, since it's the same "no production consumer"
class RETIRE-DEAD-OWN-PATH-SURFACE-OVERLAY cut), or (b) if `impact`'s
narration is meant to stay meaningful for some future re-introduced
publish channel, leave it — but that intent should be stated somewhere
citable, not left implicit.
