# 0016 — install installs the tool

- **Date:** 2026-07-07 · **Status:** accepted

## Context

The lift shipped an intermediate state: member modules holding identity
plus a `file()` back-reference to the artifact they nominally project —
source and projection the same file, depth "emergent" per member
(`model/pipeline.md`, "Install"). The dogfood's first run (PR #15) found
the state hollow: the typed layer typed nothing, while the extractor
already parses every schema-declared field to gate it — the lift threw
away what it had parsed. The ask: `install` must install temper as the
tool, never an intermediate state.

## Decision

Representing a harness is a **whole conversion** — the one question keeps
its one meaning, and yes installs the tool:

- every field the kind's schema declares hoists into a typed property;
- prose moves module-side, byte-faithful — inline text for short bodies, a
  module-adjacent file for documents;
- every governed artifact becomes a projection, regenerated canonically by
  the first emit — the one reviewable adoption diff — and guard-claimed.

No third state exists. Direction attaches to the artifact without
exception: unrepresented, every artifact is a source; represented, every
governed artifact is a projection. The shallow/deep spectrum, depth
emergence, and the own-path machinery (source==projection detection, the
guard's authored-territory exemption, the lock's `own_path` column) retire
with the intermediate state that needed them.

Costs, stated: file-sourced prose duplicates in git — module source and
committed projection, a two-file diff per prose edit — and adoption churns
each artifact once into canonical form.

## Rejected

The shallow lift (anchors without authoring — an install that does not
install the tool; its keystroke wall guarded an empty room). Per-kind
carve-outs (memory staying own-path for want of fields — a second adoption
state costs more than one uniform rule). A depth selector (there is one
depth; re-running install still converges).

## Consequences

`model/pipeline.md` "Install" recuts; the scaffold hoists fields and
relocates prose; `own_path` retires from drift, guard, and lock; the
dogfood workspace (PR #15) re-converts as the first customer.
