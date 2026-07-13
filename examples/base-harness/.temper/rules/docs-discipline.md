# Docs discipline — the corpus moves first

Path-scoped to `docs/`. These files are declared intent, not description.

- Change enters here first; implementation reconciles toward it. Never edit
  a document to match code that drifted — surface the disagreement and rule
  on it.
- Every document is read under its kind's declared layout. Headings are
  positional structure: add a section only if the kind's layout admits it,
  otherwise the gate refuses the document whole.
- Addresses are edges. A `participants` or `superseded by` entry must
  resolve; a `satisfies` entry names a requirement this document fills.
  Entries are bare member names, resolved within the edge's declared
  target kind.
- Superseding a decision is a move, not an edit: the old document relocates
  to `docs/decisions/superseded/` and gains its successor entry; the new
  decision records the old ruling as a rejected alternative.
- Evidence against a standing ruling enters as a proposed amendment, never
  as a quiet rewrite. Accepted decisions receive corrections, not new
  meanings.
