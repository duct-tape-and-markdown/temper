**Symptom:** Comments across `src/`, `sdk/`, and `tests/` cite a `specs/architecture/*`
tree (`20-surface.md`, `40-composition.md`, `10-contracts.md`, `45-governance.md`,
`00-intent.md`) plus a "law N" invariant numbering. Neither exists any more — the
corpus moved to `specs/model/*.md`, `specs/distribution.md`, `specs/builtins.md`,
`specs/decisions/*`, `specs/intent.md` (unnumbered "Invariants" list, not "laws").
The rename entry's own per-file notes ("`00-intent.md` law 4/invariant 5") repeat
the stale form.

**Cost this tick:** every stale cite I touched needed a `grep` round-trip against
the live `specs/` tree to find (or confirm the absence of) a real replacement
section before writing a new comment — several extra searches across a
MODE-ROOT-MEMBER-FIELD-sized entry just to avoid encoding a second dangling
reference while fixing the first.

**Suggested fix:** a one-time repo-wide sweep (or a `rg` job in a residue-sweep
plan tick) retagging `specs/architecture/*` / `00-intent.md` / "law N" cites to
their current `specs/` homes, so future entries touching these regions don't
re-derive the mapping from scratch.
