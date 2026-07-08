# 0012 — the embedded-member fence speaks the kernel noun

- **Date:** 2026-07-07 · **Status:** accepted

## Context

The nested-member write face was unbuilt (emit could not render embedded
members into a host body), the read fold parsed a fence grammar spelled
`genre.<kind> <key>` — a retired noun living in a user-facing format,
against "every placement speaks the kernel nouns" — and ~77 vocabulary
survivors (the SDK's `genre()` constructor and module, the locus tag, doc
comments) outlived the value-shape fold.

## Decision

One grammar, both directions, kernel-spelled: the fence is
`member.<kind> <key>`. Emit renders an embedded member with the same
grammar the read fold parses — a hand-authored mixed host is a source the
fold reads; an SDK-composed host is a projection, write-only as ever. An
embedded member gets no lock rows of its own: it serializes into its
parent's artifact, and the parent's provenance row fingerprints every byte.
The SDK's `genre()` constructor dissolves — a kind with an embedded locus
needs no second constructor — and the `"genre"` locus tag recuts to
`"embedded"`. Pre-1.0 carries no compatibility shim.

## Rejected

Keeping the `genre.` spelling for read-compatibility (CLEAN SLATE; a
retired noun in an emitted artifact re-teaches the vocabulary the corpus
retired). Per-embedded-member lock rows (a redundant copy of a fact the
parent's hash already anchors). A distinct SDK constructor for embedded
kinds (locus is a kind fact, not a species).

## Consequences

The fence recuts `genre.` → `member.` in fold and render; emit gains the
write face (`blocks()` renders embedded members into the host); the ~77
`genre` survivors across src/ and sdk/ recut to kernel nouns —
`genres.ts` dissolves, `Locus` tag `"genre"` → `"embedded"`; the field
report's stale claims re-probe at scoping per the plan rule.
