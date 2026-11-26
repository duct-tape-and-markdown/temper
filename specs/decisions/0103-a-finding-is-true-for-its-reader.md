# 0103 — a finding is true for its reader

- **Date:** 2026-11-25 · **Status:** accepted

## Context

Three instances of one class:

- `supporting-doc.degree` told unrepresented authors to add a reference that
  temper could never count (since fixed by claiming the typed reference);
- the bundled harness reference told plugin users to "re-run temper emit",
  in an install that has no program (`bundled-reference-managed-note`);
- the worktrees finding gave no remedy.

Each statement was true in one mode and false in the reader's.

## Decision

**A finding or a managed note states only what holds in the mode it is
read in, with a remedy that works there** (`contract.md`, "Read verbs").
The modes are a represented harness, an unrepresented one, and an
installed plugin. What a mode cannot decide collapses to one coverage note
that names what would decide it.

For the instances: the bundle strips the managed note, because 0093's "same
value" means the same content. The worktrees finding quotes the docs'
gitignore advice.

## Rejected

- **One wording for every mode.** It is false somewhere by construction.

## Consequences

Every new finding states the mode it applies in, and plan files a mode
mismatch as an entry under this record, not as a question.
