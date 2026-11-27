# Libraries before hand-rolls

- Where a crate in the sanctioned set (CLAUDE.md, "Tech stack") carries the
  mechanic — directly or transitively — adopt it; a hand-rolled
  reimplementation of a solved mechanic is the same residue class.
  Expanding the sanctioned set is a human call, proposed through the inbox.
- The exception is pinned semantics: where the corpus fixes exact behavior
  (byte-fidelity round-trips, charset mechanics), the implementation
  follows the corpus, and a library is adopted only where its semantics
  match the pinned contract.

**Tell:** a hand-rolled glob, walk, parse or diff where a sanctioned crate
carries the mechanic. **Held by:** review; the sanctioned set in CLAUDE.md.
