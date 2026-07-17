# Engineering shape

How code enters this repository — the form standard the phases build
against. Product intent lives in the model; this page governs shape, and it
exists so consolidation is citable: a `per` into this file is how a refactor
becomes a pending entry.

## One job, one home

- A job the tree already performs is extended where it lives, never
  reimplemented beside itself. Before adding a function, module, or helper,
  search for the existing surface; the priority is delete or subsume, then
  extend the existing surface, then generalize a near-duplicate, then —
  last — add new.
- A new surface beside a near-duplicate names, in the commit body, the
  existing surface considered and why it did not fit.
- A second implementation of one job — two matchers, two normalizers, two
  encoders — is residue, fileable against this section whenever no pending
  entry consolidates it.
- Test scaffolding is a surface too: shared fixtures and builders live in
  one home (`tests/common`), never copy-pasted per file.

## Libraries before hand-rolls

- Where a crate in the sanctioned set (CLAUDE.md, "Tech stack") carries the
  mechanic — directly or transitively — adopt it; a hand-rolled
  reimplementation of a solved mechanic is the same residue class.
  Expanding the sanctioned set is a human call, proposed through the inbox.
- The exception is pinned semantics: where the corpus fixes exact behavior
  (byte-fidelity round-trips, charset mechanics), the implementation
  follows the corpus, and a library is adopted only where its semantics
  match the pinned contract.

## A shared concept is one type

Grounded in the field: every cross-feature defect the first consumer
harvest surfaced was a consumer iterating a **partial enumeration of a
shared concept** after a producer widened it — a parallel vec beside
the canonical edge set, an XOR branch where template layers are a
union, a deletable class the ledger never learned. The treatment is
shape, so it lives here:

- A new fact-shape enters a shared concept as a **variant of the one
  shared type, never a parallel structure**. The engine's shared
  enumerations — edges, members, template layers, lock rows, deletable
  things, discoverable paths — each have exactly one type, and a
  private partial copy of one is residue fileable against this section.
- Consumers of a shared enumeration hold **exhaustive matches**, so a
  new variant refuses to compile until every judge answers it — the
  `Format`/`project_bytes` precedent: the next variant answers the
  match by construction. A `_` arm over a shared concept is a seam
  defect waiting for its field report.
- An entry that widens a shared enumeration names that enumeration's
  **other consumers** in its own files[] — "who else reads this set?"
  is derivation's question, never the consumer's bug report.

## The gauntlet corpus

Single-feature fixtures cannot find composition seams; the field
harvests that surfaced them came from a real corpus using feature
*combinations*. One in-tree kitchen-sink fixture harness holds every
composition the model claims legal — composed bodies over templated
hosts, embedded edges with scopes on both endpoints, partially-declared
manifests, local members under ignore rules — with `check` and `emit`
snapshot-tested over it. A feature entry that adds a composable surface
extends the gauntlet where its feature meets the existing ones: each
addition pays its composition cost once, at ship time, instead of in a
consumer's repo later.
