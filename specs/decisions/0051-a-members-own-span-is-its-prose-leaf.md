# 0051 — a member's own span is its `prose` leaf

- **Date:** 2026-09-08 · **Status:** accepted

## Context

The `(layout-own-span-leaf)` fork. A layout collection member's own span
— the text directly under its heading, before any child heading — had no
leaf to land under: composed leaves are author-named, and the only
reserved name in play, `prose`, is a member-level value in the SDK
(`MemberInit.prose`), not a leaf key. Without one the span reaches no
row, no leaf address, and no clause. `layout_prose` (f07d4f84) showed a
second shape: a row keyed by position, reserving no name. Ruled by John
2026-09-08: reserve the leaf; the name is the session's.

## Decision

**Every nested member's own span is its `prose` leaf**, addressed
`<host-address>/<kind>/<key>/prose`, cut at the first child heading. The
name is reserved: it is the word `representation.md` and the SDK already
use for a member's verbatim authored words, so the leaf key names the
same thing the member-level value names. A child heading whose slug is
`prose` collides with the reserved leaf and is refused loud at read, the
way a coincident address is a malformed lock (0049). Leaves stay one
family: the own span is a `NestedMemberRow.leaves` entry like any
author-named leaf, so leaf predicates, leaf addresses, and `explain`'s
narration range over one source.

## Rejected

- **A position-keyed row, the `layout_prose` shape**: reserves no name,
  but leaves would live in two families and every leaf consumer would
  range over both.
- **An author-chosen name per kind**: one thing under n names; the
  address grammar wants one spelling for one thing.
- **`body`, `own`, `summary`**: each is a plausible sub-heading in a
  real document; `prose` is already reserved at member grain and is the
  rarer heading.

## Consequences

`representation.md`'s leaf sentence names the reserved leaf.
LAYOUT-COLLECTION-MEMBER-OWN-SPAN-LEAF unblocks: the read cuts the own
span, lands it as the `prose` leaf, and refuses a `prose` child heading;
the SDK reserves the key. The `(layout-own-span-leaf)` record deletes.
