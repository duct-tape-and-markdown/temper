# 0013 — one declared format, two one-way faces

- **Date:** 2026-07-07 · **Status:** accepted

## Context

The format layer was ratified (`model/representation.md`) without the rule
that makes its composed extractor well-defined; a cited research pass over
the bidirectional-transformation literature checked layout-as-data before
building it.

## Decision

Sound, on the condition the literature is unanimous about: refuse
bidirectionality. Direction attaches to the artifact — a source is read and
never written; a projection is written and never read for meaning — so the
declaration's two faces are independent one-way functions, never a round
trip. One declaration exists for DRY alone: writer and reader cannot skew.

- **Render is injective** — distinct values never project to the same
  bytes. Bought by **admissibility, checked when the kind is declared**:
  adjacent slots split by constant text neither can absorb, alternatives
  disjoint, prose slot terminal or fenced — decidable for regular slot
  languages (Boomerang, POPL 2008, doi:10.1145/1328438.1328487, retrieved
  2026-07-07).
- **Reads are stable** — the leniency a source-read forgives (whitespace,
  quoting, key order) is declared on the format, never inferred (FliPpr,
  ESOP 2013, doi:10.1007/978-3-642-37036-6_6), and equivalent spellings
  extract identical fields (quotient lenses, ICFP 2008,
  doi:10.1145/1411204.1411257; both retrieved 2026-07-07). Leniency binds
  source-reads only; projections are canonical and never re-read.

A template holds no logic and no derived values — computation is an
engine-computed field on the kind. A structured sublanguage (frontmatter,
JSON) is a slot naming a schema, parsed by a real parser, never syntax
spelled as constant text. Extraction is total-with-errors: a deviating
source yields typed output plus a located finding, never an opaque
failure (PADS calculus, POPL 2006, doi:10.1145/1111037.1111039, retrieved
2026-07-07).

## Rejected

Lens machinery — put-back, merge alignment, format-preserving edits of
projections: it exists only where one file is both read and written, and
no such file exists here. Logic or computed segments in templates (the
JSP-scriptlet graveyard; Mustache and Django drew the hard line and
survived). Inferred leniency (accretes by bug report; declared is
auditable). Read-time ambiguity checking (Augeas ships its check
optional-and-slow; unchecked lenses are its documented worst failure).

## Consequences

Admissibility gains the template rules; the kind row's `format` recuts
from a label to declared template data, and the engine composes renderer
and extractor from it; extraction failures carry source positions.
