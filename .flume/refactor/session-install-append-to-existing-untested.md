## Surface

Coverage gap, not a confirmed bug — found via a one-off `cargo llvm-cov`
run, not the usual duplication scan. `install.rs`'s "settings.json already
carries a `SessionStart`/`PreToolUse` entry, append to it" branch
(uncovered lines ~861-893) has no exercising fixture, and the gap traces
straight through to its callee: `json_splice.rs`'s `append_element` and
`insert_member` (uncovered lines 209-236) — specifically their
`Some(...)` arms (append-after-existing-element/member), while their
`None` arms (insert-fresh) are fully covered. Every current install
fixture constructs a settings.json with no prior hooks, so the
insert-fresh path is well-tested and the append-to-existing path is not
exercised at all, on either side of the call.

## Observed at

a32a45f

## Suggested fix

Not a consolidation — add a fixture: a settings.json pre-populated with a
sibling hook entry (e.g. an existing `PreToolUse` command from another
tool), run `install`, assert the new hook is appended correctly (comma,
indent, ordering) without disturbing the existing entry. Covers both
`install.rs`'s branch selection and `json_splice.rs`'s append-after-
existing logic in one test. Real-world scenario this exercises: a second
`temper install` run, or installing onto a repo where hooks were already
configured by something else.
