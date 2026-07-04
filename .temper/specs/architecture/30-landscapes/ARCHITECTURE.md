+++
[satisfies.landscape]
rationale = "30-landscapes owns the `landscape` concept — the engine applied as an instance (harness/spec/code) — as its namesake and only definitional home"

[satisfies.cross-landscape-seam]
rationale = "30-landscapes owns the cross-landscape (spec ⟷ code) seam, including its explicit deferral pending a symbol-extraction primitive; nothing else defines the seam"

[satisfies.behavioral-contract]
rationale = "30-landscapes owns the fidelity seam separating tier-1 structure from tier-3 prose surplus — the sole architecture elaboration of the behavioral-contract concept 00-intent co-names"

[provenance]
source_path = "./specs/architecture/30-landscapes.md"
source_hash = "d185adba2cd81d14eb86d23cd101c2d97482064e7084421bd37f380adae4dbad"
+++
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
just another kind — the **spec corpus** is governed by custom spec kinds; **code**
is a third landscape with its own kind. The **build pipeline's own artifacts**
(`.flume/` — work orders citing spec sections, fork slugs, prompt documents)
are the sanctioned *candidate* third instance in practice: its `pending.json`
entries already declare cross-landscape edges (a `per` citation names a spec
path + section; `dependsOnForks` names open-question slugs), so
citation-resolution over them would be the seam's first live check — each
widening of the pipeline's governance is a fresh human ruling, never an
assumption (the fence, `docs/ledger.md`'s asymmetry list).

## The spec landscape

The **spec corpus** is a landscape governed by custom spec kinds (`15-kinds.md`).
A spec is not free prose the engine guesses about: the kind declares the
**entities, relationships, and invariants** the prose is about — and may declare
a **genre vocabulary** for the prose's own recurring forms (`00-intent.md`, the
genre Decision). Declaring the model makes the otherwise-undecidable structural —
each entity has one declared home (DRY), bindings resolve to declared names (naming
consistency), every declared entity has an owning spec (coverage). Adoption is a
gradient of four rungs, never-climb between every pair — no check may demand a
document climb, and a corpus may sit at different rungs per document forever:

1. **Plain document** — markdown, extracted features. The floor.
2. **Bound prose** — the document plus a declared model; mentions opt-in per
   word (`20-surface.md`).
3. **Embedded genres** — the document remains the authored artifact; the
   author opts specific blocks into genre values (the genre fence,
   `20-surface.md`) where the structure pays.
4. **Dissolved document** — the value tree is the source; the document is
   emitted projection. Prose survives as **leaves**: authored strings inside
   genre values — addressable, law-5-protected one by one.

The model is **declared, not templated**, at every rung; a corpus with a small
model pays little, one reconciled against code pays the modeling tax and is
repaid in coherence (`00-intent.md` honest bound — coherence, never
correctness; a contract-clean spec can still model the wrong domain).

## The dependency graph

The declared entities + relationships yield a **dependency graph of intent** — the
kind capability in `15-kinds.md`, and the substrate the governance predicates act on
(`45-governance.md`). Its tier-1 payoff needs no LLM: removing a load-bearing entity
surfaces its **blast radius** across every spec, binding, and — via the seam below —
code symbol, deterministically. Fearless refactoring (`00-intent.md` law 6) with
teeth, the standing value of the modeling exercise independent of any judge.
Under the genre Decision the graph reaches **inside arguments**: a leaf is a
node, a rejected alternative is a node, and blast radius answers "who cites
this rationale" at the grain the primary author edits at.

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
Genre structure shrinks the surplus further — the anatomy of an argument moves
into the declared model — but the leaves *are* the surplus, and they stay
tier-3: `verified_by`, human, forever.

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
