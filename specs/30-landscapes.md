# Landscapes — instances of the engine

A **landscape** is a corpus of authored artifacts governed by the
**assembly** — which binds a **package** per kind (`10-contracts.md`). The harness is one landscape; the spec corpus is
another; code is a third. This file specifies how a landscape is declared and the
two that matter beyond the harness: the spec landscape (a declared model + bound
prose) and the cross-landscape seam.

## Kinds read landscapes — see `15-kinds.md`

Every landscape is read by a **kind**: an extractor (the soundness boundary) plus a
contract. The extraction algebra, the built-in/custom split, and the entity-graph
capability are the kind system (`15-kinds.md`). A landscape beyond the harness is
just another kind — the **spec corpus** is governed by a custom `spec` kind; **code**
is a third landscape with its own kind.

## The spec landscape

The **spec corpus** is a landscape governed by a custom `spec` kind (`15-kinds.md`).
A spec is not free prose the engine guesses about: the kind declares the
**entities, relationships, and invariants** the prose is about, and the prose
**binds** to them. Declaring the model makes the otherwise-undecidable structural —
each entity has one declared home (DRY), bindings resolve to declared names (naming
consistency), every declared entity has an owning spec (coverage). The model is
**declared, not templated**; a corpus with a small model pays little, one reconciled
against code pays the modeling tax and is repaid in coherence (`00-intent.md` honest
bound — coherence, never correctness; a contract-clean spec can still model the
wrong domain).

## The dependency graph

The declared entities + relationships yield a **dependency graph of intent** — the
kind capability in `15-kinds.md`, and the substrate the governance predicates act on
(`45-governance.md`). Its tier-1 payoff needs no LLM: removing a load-bearing entity
surfaces its **blast radius** across every spec, binding, and — via the seam below —
code symbol, deterministically. Fearless refactoring (`00-intent.md` law 6) with
teeth, the standing value of the modeling exercise independent of any judge.

## The fidelity seam — where prose exceeds its model

Prose earns its place by saying *more* than the declared model: rationale,
nuance, edge cases. That surplus is the **behavioral contract** (`00-intent.md`
law 7) — the human tier-3 layer promising what the structure cannot state. The
question asked of it is **fidelity**: does this paragraph faithfully describe the
entity it binds? Fidelity is undecidable, so the behavioral contract is enforced
by `verified_by` (tier 3, human), never adjudicated by `temper`. The declared graph shrinks fidelity into atomic, context-local
questions (chunk + its declared neighborhood) that a cheap model could judge once
calibrated per question-class (`00-intent.md` tier 2). That judged shelf is
**deferred — not a now thing** — and is forever advisory, never the hard gate.

## The cross-landscape seam — spec ⟷ code

Landscapes reference each other. A declared spec entity may require a
corresponding code symbol; the contract the `spec` **package** carries checks the
correspondence resolves both directions. This is flume's spec↔code equality made a *checked relation* rather
than aspirational prose-vs-types — the structural backbone the spec layer lacked.

**Honest bound — the code half is deferred, and the deferral is structural.** The
extraction algebra reads *documents* — structured fields, markdown structure,
text/file facts (`15-kinds.md`) — and none of its primitives can read a code
symbol. A code kind therefore waits on a **symbol-extraction primitive**: a
deliberate vocabulary addition (`15-kinds.md`'s Decision — never a per-kind
hatch) carrying its own machinery, because syntax-tree parsing is not a variation
on frontmatter. Until that primitive is sanctioned, the seam is *direction, not
surface*: nothing in the current algebra implies it, and no spec clause should
pretend to check it.

## Scope (not sequencing)

The spec owns dependencies and scope; the **order** of work is the plan phase's
to derive from those dependencies + current code state — the spec does not dictate
a build order (depth rule, `90-spec-system.md`). The dependencies are stated
above as facts: the graph derives from the declared model; the seam relates two
landscapes; the judged tier rents the graph.

In scope now: the generic engine + the harness instance (replacing the heuristic
registry, `10-contracts.md` decision), the declared model, and the dependency
graph. **Out of current scope:** the judged fidelity tier (tier-2) — deferred,
advisory, and to be calibrated before it is trusted — and the **code landscape**
(the seam's code half, above) — deferred until a symbol-extraction primitive is
deliberately added to the algebra.
