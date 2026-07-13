The documentation corpus is this repository's declared intent: the systems,
flows, decisions, and terms under `docs/` are authored members of declared
kinds, and the code and configuration of the repository reconcile toward
them. A document here is never a description trailing the implementation; it
is the specification the implementation answers to.

## Purpose

Hold the repository's intent in typed, addressable, gated form. Each
document is read under its kind's declared layout, so its sections are
model structure: an invariant is a member a contract can bind, a
participant list is a set of edges the gate resolves, and a claim of
requirement coverage is a `satisfies` entry, not a sentence.

## Invariants

### Spec authoritative

Change enters the corpus first. Code follows via reconciliation; a change
that lands in code with no corpus home is drift to surface, never truth to
transcribe backwards.

### Declared never mined

Structure exists because an author declared it on a typed surface: a
heading the layout admits, an address in an edge section, a member in a
collection. No tool derives model structure by pattern-matching prose.

### Return paths are declarations

Evidence against the corpus (field reports, implementation pain) enters
through an authored surface — a proposed decision, an open question —
never by editing downstream artifacts first.

## Satisfies

- documented-spine
