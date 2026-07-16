# 0027 — the nested file child composes its path from its host

- **Date:** 2026-07-16 · **Status:** accepted

## Context

0025's amended supporting-docs bullet shipped `supporting-doc` as the child
kind of `skill`'s template, and TEMPLATE-FILE-CHILD-FACT (794678f) landed
the declared fact — child kind plus path pattern. Routing found the half
above it unspellable: locus was binary (a file at a governed glob, or
embedded in the parent's body), and a file-owning nested child fit
neither — while a self-governing child kind's glob would overlap `skill`'s
own under `.claude/skills/`, making position undecidable where the
representation layer requires it decidable.

## Decision

Locus gains its third spelling: a **nested file** member owns a file whose
path composes from its host's unit and the host template's path pattern.
The pattern is the host's declared fact — one home — so the child kind
governs no glob of its own and two kinds still never share one. Both faces
flow from the same fact: a composing program declares a skill's documents
as nested members that own files; discovery classifies an adopted
harness's matching files as the skill's children through the template. The
pattern claims the honest, cited subset of the vendor's shape: markdown
documents — the prose-only kind can hold nothing else — with supporting
files of other types unmodeled and named as such, the `settings.json`
partial-governance posture. `supporting-doc`'s default contract is
almost-empty per the standing bar; the one candidate clause — reachable
from its skill's body, the vendor documenting supporting files as read
when linked — ships advisory with its cite, the runtime consequence riding
as guidance, never as encoded behavior.

## Rejected

- **A self-governing child kind**: its glob overlaps the host's; position
  by content is mining, and no carve-out isn't.
- **Modeling every supporting file type**: the vendor allows any type
  anywhere in the skill directory, but the typed kind is prose-only —
  claiming paths the type cannot hold would overclaim; the honest subset
  plus named partial governance is the standing posture.
- **The pattern as the child's own fact**: two homes for one path fact —
  the host template already owns it (`representation.md`, "kind").

## Consequences

`representation.md`'s locus section goes ternary and `builtins.md`'s skill
bullet names the honest-subset claim — same commit, this record. Code
reconciliation routes through plan: the composition surface, discovery off
the template pattern, and the built-in adoption —
SKILL-NESTED-REFERENCE-DOCS re-enters buildable.
