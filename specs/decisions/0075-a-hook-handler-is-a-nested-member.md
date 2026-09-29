# 0075 — a hook handler is a nested member

- **Date:** 2026-09-29 · **Status:** accepted

## Context

The `(hook-handler-grain)` fork, raised by plan deriving 0063. 0063 says a
group carries its handlers "as its own array", but six of the seven
shipped `hook` clauses range over one handler's flat fields (`type`,
`command`, `url`), and the clause vocabulary has no per-element
predicate. Where the handler-level contract lives decides every 0063
entry. This amends 0063's array wording. Ruled by John 2026-09-29 on
plan's and the session's shared stance.

## Decision

**Each entry of a group's `hooks` array is a nested, embedded member of
a built-in `handler` kind**, keyed by its position (`…/handler/0`). The
hook member keeps its event and matcher; the handler-level clauses move
to `handler` unchanged. Nested content is a member of its own kind,
never a parallel value shape (`representation.md`, "nesting"), so this
is the model's existing primitive, not a new one. `handler` ships under
`hook` the way `supporting-doc` ships under `skill`, outside the surface
enumeration: a handler is the hook domain's own content, not a new
domain.

Matching handlers run in parallel (code.claude.com/docs/en/hooks,
retrieved 2026-09-29), so position carries no runtime meaning: a reorder
re-keys lock rows and changes nothing Claude Code does.

## Rejected

- **A raw array field with the six clauses retired.** A coverage
  regression, not a simplification.
- **A per-element clause predicate.** Grows the vocabulary for a shape
  nesting already covers.
- **Keying handlers by content** (the command, the URL). An edit becomes
  a delete and a create, and two identical handlers coincide.

## Consequences

- The SDK `hook()` composes handler members; tap synthesis joins an
  authored group at the same (event, matcher) as another handler.
- Address and grain ship together: identity alone refuses a legal
  `settings.json` holding two handlers in one group.
