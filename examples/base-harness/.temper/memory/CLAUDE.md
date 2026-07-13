# base-harness

A starter harness whose documentation corpus is a temper program. This file
tells you where to look and how to work; `docs/` holds the intent.

## Docs map

- `docs/systems/` — one document per area of declared behavior.
- `docs/flows/` — behavior that crosses systems.
- `docs/decisions/` — accepted rulings; `docs/decisions/superseded/` holds
  replaced ones, each carrying its successor edge.
- `docs/glossary.md` — domain terms as addressable members.

## How to work

- The corpus is authoritative. Read the relevant system and flow documents
  before changing anything they govern; when docs and code disagree, the
  code has the bug or the corpus needs an authored amendment.
- Documents under `docs/` are sources: edit them in place, then run
  `temper check` — each is read under its kind's declared layout, so a
  section is structure, not formatting.
- `CLAUDE.md` and `.claude/` are projections: edit the owning module under
  `.temper/` and re-run `temper emit`. A direct edit is drift.
- Structural verdicts come from `temper check`; do not re-derive them by
  reading files. `temper explain <member>` narrates any member's edges,
  coverage, and blast radius.
