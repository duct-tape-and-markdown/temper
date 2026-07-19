# 0043 — The map declares subsystems; the tree declares membership

**Status:** ratified 2026-07-18 (session delegation, architecture lane)

## Context

`specs/process/architecture.md` enumerated every module under its
subsystem. The enumeration churned with the tree: each module birth or
move forced a spec amendment, authored by hand from behind the fence
that keeps build out of `specs/` — the page tracked the tree instead of
ruling it, in an artifact that promises to be revised rarely. The only
information in those lists the tree cannot answer is the
module→subsystem assignment; the names, and their one-line jobs, are
restatements of `ls src/` and the modules' own `//!` docs.

## Decision

Split the map's content by where each datum changes:

- **The page keeps the durable half**: the subsystem vocabulary — each
  subsystem's name and its one job — the invariants stated as
  absences, the flatness rule, and the growth rules. It names no
  modules.
- **The module keeps the moving half**: every engine module's `//!`
  header carries a `subsystem:` line naming one subsystem from the
  page's vocabulary; every SDK module carries the same in its header
  comment. The assignment travels with the file, inside build's
  writable fence — the entry that creates a module declares its home
  in the module.
- **The roster is computed, never stored**: any consumer that needs
  the module list per subsystem (the posture sweep's rotation, a
  reader) derives it from the headers. A module whose header names no
  subsystem, or one outside the vocabulary, is residue against this
  page.

A live tension edge — a module ruled to move — is queue state, not map
state: the ruling routes to the queue, the header changes when the
move ships, and the page says nothing.

## Consequences

- Header lines land as one mechanical entry (routed at ratification).
- Once headers land: `architecture.md` sheds its module enumerations
  and its SDK codemap list; the posture-sweep rule derives its roster
  from headers instead of the page.
- The decidability ladder's endgame — a check that every module
  declares exactly one known subsystem — is a post-freeze product
  candidate, not part of this decision.

## Rejected alternatives

- **Keep enumerating in the page** — restates tree-derivable state in
  a durable artifact; every module birth needs a human spec commit the
  fence denies to the phase doing the work.
- **An assignment table in a spec or data file** — same churn locus as
  the page; the datum still lives away from the thing it describes.
- **No recorded assignment (infer from imports)** — membership is
  intent, not structure; a grouping that exists nowhere is not
  recoverable from a flat tree, and the sweep loses its roster.
