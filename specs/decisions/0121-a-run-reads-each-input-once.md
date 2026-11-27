# 0121 — a run reads each input once

- **Date:** 2026-11-27 · **Status:** accepted

## Context

"Cost scale is hoisted" (`engineering.md`) has stated since 07-17 that
whole-input work runs once per run, and every walk it governs carries a
count pin. On 0.0.26 a `check` still builds four tree indexes, scans the
index 60 times (once per kind and locus), builds a fresh glob matcher for
each candidate, and reads each member three times. A shell-edge `guard`
walks once per locus, 11 times on this repository. At 100k files `check`
takes 2.4 s, 90% of it in the per-kind scans. Two of the walks waive the
ignore rules across the whole tree and descend `target/`.

Every site came from a build tick, most after the rule landed. Nothing in
the engine owned one run's inputs, so each new judge read the disk itself
and pinned its own mechanism. One pin asserts exactly one walk per locus
and fails a collapse to one walk. Evidence: inbox
2026-11-27-whole-input-work-repeats-per-kind-and-per-door.

## Decision

- **A run holds one owner per input** (`architecture.md`, "A run reads
  each input once"). Judges read through it, it fills on first use, and
  it reads back the run's own writes.
- **One index, fenced in one place.** A kind that claims ignored
  documents reads them under its own locus.
- **Count pins sit at the owners, per verb** (`engineering.md`). A pin on
  one mechanism's counter stops being a cost pin.

## Rejected

- **Per-mechanism memos and counters.** This is the tree as built: each
  is locally right, and the run's total goes unpinned.
- **A process-wide cache keyed by path.** It hides which run owns a
  write, which `emit` and `install` both make.
- **One walk that indexes ignored entries and filters them.** It reads
  `node_modules/` and `target/` on every run to serve a few local loci.
- **Leaving the rule as it stood and filing entries per site.** The rule
  already held and the sites kept arriving. It named no home for the work
  to be shared through.
- **The strongest objection:** an owner threaded through every judge is
  a wide change while the launch gate is open. It is taken because the
  rule already requires sharing, and this decides where the sharing
  lives. The change lands one verb at a time, `check` first, since it
  runs at every session open.

## Consequences

Plan files entries from the inbox record against both sections. The
per-locus walk pin and the per-mechanism counters retire into the
owners' pins.
