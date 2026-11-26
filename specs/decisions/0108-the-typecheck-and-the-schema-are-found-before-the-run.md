# 0108 — the typecheck and the schema are found before the run

- **Date:** 2026-11-26 · **Status:** accepted

## Context

Two plan questions about the same problem: a fact the harness needs before
anything runs has nowhere to live.

- `typecheck-found-before-the-run`: emit is to typecheck "before running
  the program", but the declaration lives in the program's payload.
- `unrepresented-schema-home`: an unrepresented harness gets the schema
  modeline, but declining representation creates no `.temper/` to hold a
  schema file.

Ruled by the session under John's direction to triage as before.

## Decision

- **The typecheck is the one the last lock records** (`pipeline.md`,
  "Emit"). Emit already reads the prior lock for its fingerprints and
  reaping, so the declaration has one spelling. A first emit, with no lock
  yet, runs it right after the program.
- **The unrepresented modeline points at the schema the engine's npm
  package ships for its own version** (`distribution.md`, "Keystroke"),
  through a version-pinned CDN URL. Nothing beyond the modeline is written
  to the tree, and the only host is the registry temper already publishes
  to.

## Rejected

- **A typecheck location declared outside the program.** That is a second
  spelling of one fact.
- **A committed schema path in an unrepresented tree.** It writes temper
  files into a harness that declined representation.
- **Amending the modeline away.** The keystroke placement is intent.

## Consequences

The schema JSON joins the published npm package. A changed typecheck
declaration takes effect from the emit after the one that records it.
