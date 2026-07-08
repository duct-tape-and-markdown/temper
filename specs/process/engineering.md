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
