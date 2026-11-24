# 0077 — a layout is its own noun

- **Date:** 2026-11-24 · **Status:** accepted

## Context

An adopter's 0.0.23 report (GH #62, #63, #66) found three places where a
layout kind is read on a path of its own and loses what a plain kind
keeps: frontmatter is never parsed once a kind declares a layout
(`compose.rs`'s layout branch returns before the frontmatter read),
clauses on a collection's entries never fire, and the guard judges layout
documents against provenance rows they never carry. The layout lived as
one clause of `representation.md`'s "kind" bullet, about 25 lines long,
and nothing there said how a layout composes with the kind's format or
locus. Invariant 6 already entails the answer: a read that drops the
frontmatter represents less than it was given. The corpus stated the
noun's rules but not where they stop, so each reader decided for itself. Ruled by John 2026-11-24 on the
session's recommendation.

## Decision

**The layout gets its own section in `representation.md`**, beside kind
and locus, and states its edges:

- A layout structures the body and nothing else. The format's other
  slots, frontmatter among them, read exactly as on a `file`-content
  kind, and the document's identity follows its kind's locus like any
  file member's.
- A collection entry is a member in every respect: selected, judged and
  addressed like any member of its kind.

The rule that two kinds never share a `governs` glob moves to the file
locus, where position is declared. The file stays within its 150-line
budget, cut by compression only: no rule is dropped.

## Rejected

- **Fix the three defects with no corpus change.** Each fix is derivable,
  but a noun whose edges are unstated breaks again at its next reader. A
  shared concept is one type (`process/engineering.md`), and the corpus
  side of that is one section per noun.
- **A layout as a format variant.** A format renders and reads a whole
  artifact. A layout types the body slot inside one. Merging them puts
  frontmatter and headings under one grammar that neither needs.

## Consequences

Plan derives the entries from the section: the layout read keeps the
format's frontmatter read and the kind's unit-shape identity (#63), the
embedded-member judge covers collection entries (#62), and the guard's
locus set routes layout documents the way `check` does (#66, per 0079).
