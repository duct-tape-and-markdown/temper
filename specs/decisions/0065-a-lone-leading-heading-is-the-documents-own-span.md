# 0065 — a lone leading heading is the document's own span

- **Date:** 2026-09-29 · **Status:** accepted

## Context

The `(layout-title-heading-admission)` fork (GH #45(b)). A layout binds
its regions to the document's shallowest heading level, so an
H1-title-over-H2-sections document — markdown's universal shape, and
`intent.md`'s own — had its title admitted into whichever region bound
first, swallowing every later region. The swallow is now loud; whether a
title is admitted at all was unruled. 0051 already makes a nested
member's own span, cut at its first child heading, its `prose` leaf.
Ruled by John 2026-09-29 on the session's recommendation.

## Decision

**A lone leading heading is the document's title.** Its span is the
document's own `prose`, cut at the first child heading, and the layout's
regions bind to its child headings. This is 0051's rule applied one
level up: no fourth primitive.

## Rejected

- **A title primitive.** The model admits exactly three primitives; a
  title is the document's own span, which the model already names.
- **Leaving the swallow loud and unadmitted.** It refuses the most common
  markdown shape to protect a rule nothing asks for.

## Consequences

`Layout::read` binds regions beneath a lone leading heading. A document
with two top-level headings has no title and binds as it does today.
