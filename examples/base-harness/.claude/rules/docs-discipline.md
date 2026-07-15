---
# temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file.
paths: ["docs/**"]
---
# Docs discipline — the corpus is the program

Path-scoped to `docs/`. Everything here except `glossary.md` is a
projection: the authored home is the owning module under `.temper/docs/`,
and a direct edit is drift.

- To change a document, edit its module (narrative passages, invariants,
  steps, and alternatives are typed values composed in the module), then
  run `temper emit`. The structural discipline — a step's system resolves,
  an `implemented-by` file exists, a superseded ruling names its
  successor, spine coverage holds — is enforced by the program and the
  gate, not by this rule.
- Superseding a decision is the `supersede()` operation in
  `.temper/kinds.ts`: it takes the successor as an import, moves the old
  ruling's record to `docs/decisions/superseded/`, and never edits the
  accepted text into something new.
- `docs/glossary.md` is the one source: edit it in place. Its terms are
  addressable members, so renaming one is a model change, not a
  find-and-replace.
- Evidence against a standing ruling enters as a proposed amendment, never
  as a quiet rewrite. Accepted decisions receive corrections, not new
  meanings.
