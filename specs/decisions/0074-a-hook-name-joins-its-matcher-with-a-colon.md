# 0074 — a hook's name joins its matcher with a colon

- **Date:** 2026-09-29 · **Status:** accepted

## Context

The `(hook-address-matcher-glyph)` fork, raised by plan deriving 0063.
0063 makes a hook's identity its event plus its matcher but names no
spelling, and the spelling is lock-visible: every adopter's lock carries
`hook:<Event>` in its rows, so a later respelling re-emits them all.
Matchers are exact strings, `|`- or `,`-separated lists, or unanchored
JavaScript regular expressions (code.claude.com/docs/en/hooks, retrieved
2026-09-29). Ruled by John 2026-09-29 on plan's and the session's shared
stance.

## Decision

**A hook's name is its event, then `:` and the matcher's authored bytes.**
`hook:PostToolUse:Edit|Write`; a group with no matcher keeps
`hook:PostToolUse`. The address grammar is unchanged: an address splits
at its first `:`, so the name is `PostToolUse:Edit|Write` and the event
is the name's text before its own first `:`, which no documented event
carries. The matcher is spelled verbatim, never normalized: `Edit|Write`
and `Edit, Write` are two groups in `settings.json` and two members here.

## Rejected

- **`/` as the joiner.** It is the nesting separator; a matcher is not a
  nested member, and it would read as one.
- **Normalizing equivalent matchers** (`*`, `""`, list order). Claude
  Code keeps them as distinct groups, and identity follows the authored
  bytes, never their meaning.
- **A new three-part grammar.** The two-part grammar already carries a
  name holding `:`; a third part would be a rule for one kind.

## Consequences

- Both `member_address` homes spell and read the hook name this way;
  the one refusal stays 0063's: a matcher carrying `/` is refused loud.
- Every lock re-emits with the new names, this repo's `.temper/lock.toml`
  included — a human `chore(harness):` re-emit ordered after the engine
  and SDK ship, since build never writes `.temper/`.
