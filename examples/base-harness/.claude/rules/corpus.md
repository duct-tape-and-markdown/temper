---
# temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file.
paths: ["docs/**","src/**","TODO.md"]
---
# The corpus is the program

`docs/` is declared intent and `src/` reconciles toward it: when the two
disagree, the code has the bug or the corpus needs an authored amendment.

- Everything under `docs/` except `glossary.md` is a projection: the
  authored home is the owning module under `.temper/docs/` (narrative
  passages, invariants, steps, and alternatives are typed values composed in
  the module). Edit the module, then run `temper emit`; a direct edit is
  drift.
- `docs/glossary.md` is the one layout source: edit it in place. Its terms
  are addressable members, so renaming one is a model change, not a
  find-and-replace.
- The structural discipline — a step's system resolves, a system's
  implementing files (its declared inputs) exist and are fingerprinted, a
  superseded ruling names its successor, spine coverage holds — is enforced
  by the program and `temper check .`, not by this rule.
- Accepted decisions receive corrections, not new meanings.

For a procedure in this area, use the `corpus` skill: it routes to one topic per procedure.
