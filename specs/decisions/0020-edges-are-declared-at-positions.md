# 0020 — edges are declared at positions, never matched in prose

- **Date:** 2026-07-10 · **Status:** accepted

## Context

The same line has been re-derived three times in one arc: 0019 typed the
heading tree by declaring positions; a mention grammar over prose spans
(backtick conventions resolving to edges, drafted for source documents on a
customer corpus's ask) was argued and walked back; the contract layer
already carries requires/satisfies as member properties. The re-litigation
had a cause: a kind-declared *grammar* superficially resembles a
kind-declared *layout*, and invariant 1's "never mined" did not name the
difference.

## Decision

A declaration may type a structural **position** — a heading, a slot, a
fence — and what an author writes at that position is typed surface. A
declaration may never type a **pattern within prose** — a span shape
matched anywhere; matching is mining. Accordingly, a source-locus member's
edges are declared as member properties at a declared position: a layout
may mark a field section as an **edge slot** — `satisfies` among them —
whose entries are addresses, derived to ordinary edge rows by the same
emit pass that derives everything else, a dangling entry hitting the
existing refusal. Prose spans, backticked paths included, stay prose.

## Rejected

- A kind-declared mention grammar over prose regions (this session's own
  draft): every failure semantics degenerates — mandatory resolution
  outlaws ordinary code formatting, silent non-resolution is a silent
  edge, and severity-graded matching is a linter; the gate does not lint.
- Program-side edge declaration for source members (the assembly naming a
  document's edges): links held away from the author's hand rot — the
  documented failure mode of every external traceability tool.

## Consequences

`intent.md` invariant 1 gains the position/pattern sentence;
`representation.md`'s content bullet names the edge slot; the deferred
"floor mention syntax" in `src/extract.rs` resolves to never — the
deferral comment dies on the next entry that opens the file (exit
clause); code: the edge-slot derivation rides the existing kind
edge-field fact over layout slots. The customer convention is answered:
declare, don't match.
