## Surface
`import.rs`'s `READ_DIRS` thread-local counter (`src/import.rs:118-126`, doc:
"re-walks the tree — a scan reintroducing a `fs::read_dir` bumps it off
zero") and its getter `read_dir_count()` (131-134) are never incremented
anywhere in the crate — `git log -S READ_DIRS -- src/import.rs` shows the
counter and the `fs::read_dir` call site it was meant to guard were removed
together in commit `0bc0ee9`, and no call site was ever wired to bump it.
`tests/check_cost.rs:196-199` asserts `read_dirs == 0` as proof "the
per-kind glob scan opens no directory of its own" — but since the counter
is structurally unconstructable to nonzero, the assertion holds vacuously
regardless of whether the invariant is real; a regression reintroducing
`fs::read_dir` inside `collect_glob` (`import.rs:419`) would need to
separately remember to increment `READ_DIRS`, defeating the point of an
automatic count-pin. Contrasts with the file's own live pins `WALKS`
(98-107, incremented for real at 485) and `kind.rs`'s `GLOB_COMPILES`.

## Observed at
7ac498a

## Suggested consolidation
Either wire `READ_DIRS` into every `fs::read_dir` call reachable from
discovery (verify none currently exist outside `ignore::Walk`'s own
internals, which this counter cannot see anyway) so the pin is live, or
delete the counter, its getter, and the `check_cost.rs` assertion as a pin
with nothing left to guard — a design call on whether the invariant is
still worth a decidable pin at all (engineering.md, "A green verdict is
proven non-vacuous").
