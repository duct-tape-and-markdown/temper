# base-harness

A starter harness whose documentation corpus is a temper program. This file
tells you where to look and how to work; `docs/` holds the intent, and
`src/` reconciles toward it.

## Map

- `docs/systems/` — one document per area of declared behavior; each names
  the `src/` files implementing it (`implemented-by`, an edge the gate
  resolves) and declares its invariants as typed members.
- `docs/flows/` — behavior that crosses systems; each step is a typed
  member carrying an edge to the system it happens in, and the
  participants line is rendered from the steps, never authored twice.
- `docs/decisions/` — accepted rulings, rejected alternatives as typed
  members; `docs/decisions/superseded/` holds replaced rulings, each
  carrying its successor edge.
- `docs/glossary.md` — domain terms as addressable members.
- `src/` — the implementation the corpus governs.

## How to work

- The corpus is authoritative. Read the relevant system and flow documents
  before changing anything they govern; when docs and code disagree, the
  code has the bug or the corpus needs an authored amendment.
- The documents under `docs/` are projections of the harness program —
  glossary.md excepted, which is a layout source edited in place. To change
  one, edit its owning module under `.temper/docs/` (narrative passages,
  invariants, steps, and alternatives are typed values in the module) and
  re-run `temper emit`. A direct edit is drift.
- `CLAUDE.md` and `.claude/` are projections the same way: edit the owning
  `.temper/` module, never these files.
- Structural verdicts come from `temper check`; do not re-derive them by
  reading files. `temper explain <member>` narrates any member's edges,
  coverage, and blast radius.
