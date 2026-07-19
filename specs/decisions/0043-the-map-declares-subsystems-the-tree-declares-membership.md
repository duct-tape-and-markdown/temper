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
- **The registry keeps the moving half**: `src/lib.rs` already lists
  every library module as a mandatory `pub mod` line — the one edit a
  module birth cannot skip. Those lines are grouped under one section
  comment per subsystem, and membership is the section a module's
  line sits in. No per-module annotation exists: the assignment rides
  the registry edit build already makes, inside its writable fence.
  The binary `main` is the registry's one absence; the page names it
  in the verbs vocabulary as the standing exception.
- **The roster is computed, never stored**: any consumer that needs
  the module list per subsystem (the posture sweep's rotation, a
  reader) derives it from the registry's sections. A `pub mod` line
  above the first section, or a section name outside the page's
  vocabulary, is residue against this page.
- **The SDK keeps the page's short list for now**: `sdk/src/` has no
  mandatory registry (`index.ts` is curated — an export earns its
  consumer) and its module list is near-static. The page's SDK
  sentence stays until SDK module churn gives this decision's
  mechanism something to attach to.

A live tension edge — a module ruled to move — is queue state, not map
state: the ruling routes to the queue, the header changes when the
move ships, and the page says nothing.

## Consequences

- The registry grouping lands as one mechanical entry (routed at
  ratification).
- Once it lands: `architecture.md` sheds its engine module
  enumeration; the posture-sweep rule derives its roster from the
  registry instead of the page.
- The decidability ladder's endgame — a check that the registry's
  sections match the page's vocabulary — is a post-freeze product
  candidate, not part of this decision.

## Rejected alternatives

- **Keep enumerating in the page** — restates tree-derivable state in
  a durable artifact; every module birth needs a human spec commit the
  fence denies to the phase doing the work.
- **A `subsystem:` line in every module header** — the first cut of
  this decision, withdrawn on review: ~36 scattered metadata comments
  are the comment-sprawl class the narration posture prunes, and a
  new per-file annotation invites more. The registry grouping carries
  the same datum in seven lines on a surface every birth already
  edits.
- **An assignment table in a spec or data file** — same churn locus as
  the page; the datum still lives away from the thing it describes.
- **Directories as membership** — structurally pure but reverses the
  ratified flat-tree section on comment-hygiene grounds; too much
  reversal for the problem's size.
- **No recorded assignment (infer from imports)** — membership is
  intent, not structure; a grouping that exists nowhere is not
  recoverable from a flat tree, and the sweep loses its roster.
