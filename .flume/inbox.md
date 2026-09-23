<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->








## The reserved-`prose` refusal points an embedded value at a remedy it cannot take — observed at 0a4ca236

Observed in the 0.0.19 pre-cut regression over cascade's harness (a throwaway
worktree). `embeddedMemberValue` refuses a leaf named `prose` with "rename
the field, or author the words as the member's prose" (sdk/src/kind.ts,
`refuseReservedLeaf`). But an embedded value has no member-level `prose`:
`embeddedMemberValue`'s init takes `kind`, `key`, `leaves` and
`collections` only. So the second remedy names a surface that does not exist,
and the author's only path is the rename. Ruled by John 09-23: the
reservation stands and adopters rename. The message says so for an embedded
value and offers the member-prose route only where a member has one.
Message-only; no behavior change.

## A source dependency outside the harness root records an absolute path in the committed lock — observed at 4756671c

Consumer report on 0.0.19 (win32), **reproduced on linux** at 4756671c. A harness at
`a/b/.temper` declares `inputs: [input(import.meta.url, "../../../src/x.txt")]`
and one `"../inside.txt"`. `emit` writes
`source_path = "/home/…/input-outside/src/x.txt"` for the first and
`"inside.txt"` for the second. On Windows the absolute form also carries
canonicalize's verbatim prefix (`//?/C:/Users/…`).

**Read:** `harness_relative` (src/drift.rs:2182) strips the canonical harness
root as a prefix and, when the target is not under it, falls back to the
absolute path (`Err(_) => to_lock_path(&target_abs)`, :2195). All three
source-dependency families route through it, so an `include()` or layout import
outside the root has the same fault, not only `input()`.

**Why it is a defect:** the committed lock stops being byte-reproducible across
checkouts (invariant 3). A second checkout, worktree or CI runner fingerprints
the *first* checkout's file or none, and a Windows lock is unreadable on
linux. It makes `input()` unusable for its main case: a model that sits
beside the code it describes, not above it.

**Fix direction:** record the path relative to the harness root with `../`
segments, and resolve it the same way at check. Refuse loudly only when no
relative path exists (a target on another Windows drive). Pin it with a
test that emits the same lock from two checkout paths.

## `membership` over a list-valued field passes silently; `unique` shares the hole — observed at 4756671c

Consumer report (0.0.18 and 0.0.19), **reproduced** at 4756671c with a scratch program.
`membership("emits", "request-entry")` on kind `control`, where the satisfier
`p1` has `emits: "ok-value"`: `c1` with `emits: "bogus-a"` fires
`control.membership.emits` (exit 1). `c1` with `emits: ["bogus-a", "bogus-b"]`
reports nothing (exit 0).

**Read:** `engine::out_of_set` builds both the allowed set and the checked
values through `Selection::values` (src/engine.rs:578), which keeps
`value.as_scalar()?` only, so a list contributes no value to either side and
the clause checks nothing. `duplicates` (`unique`) reads through the same
function.

**What the spec decides:** loud or nothing (intent.md, invariant 6), so a
silent pass is the defect whichever way it is fixed. For `membership` the
reading is per element on both sides: each element of the member's list
must be drawn from the union of the satisfiers' values, lists flattened, and
each out-of-set element is a finding naming it. For `unique` over a list
the reading is ambiguous (unique within one member's list, or across
members), so refuse a `unique` clause over a list-valued field at
admissibility until it is ruled, rather than guess.
