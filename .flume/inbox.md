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
