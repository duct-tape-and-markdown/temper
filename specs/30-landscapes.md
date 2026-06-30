# Landscapes — instances of the engine

A **landscape** is a corpus of authored artifacts governed by one declared
contract (`10-contracts.md`). The harness is one landscape; the spec corpus is
another; code is a third. This file specifies how a landscape is declared and the
two that matter beyond the harness: the spec landscape (a declared model + bound
prose) and the cross-landscape seam.

## Extraction is the soundness boundary

Every landscape needs a per-kind **extractor**: parse a unit into structured
features the engine validates (`20-surface.md` generalizes the skill IR to this).
A contract clause is sound only if its feature is **deterministically
extractable**. For structured artifacts (JSON, frontmatter) that is trivial. For
prose it is the dividing line:

- **Deterministically extractable → may be a clause:** a heading is present, a
  `[link](path)` resolves, a `## Decision` carries a `Rejected:` block, a file is
  ≤ N lines, a file sits under a declared directory, a declared binding marker
  exists.
- **Semantic → never a clause:** "is this fact duplicated," "is the same concept
  named the same way," "does this paragraph mean X." These require interpreting
  meaning and are tier-2/tier-3 (`00-intent.md`), not the hard gate.

The engine is sound because the extractor is deterministic. Garbage extraction
would forge false positives, so extractors admit only surface-decidable features.

## The spec landscape: a declared model + bound prose

A spec is not free prose the engine guesses about. The author **declares the
domain model** — entities, relationships, invariants — as structure, and the
prose specs **bind** to it. This is the hard, valuable part of speccing (the
grain): making the model the prose represents coherent *before* the prose flows.
It is `10-contracts.md`'s spec-contract instance.

What that makes deterministic — properties that are undecidable if *inferred*
from prose become structural once the model is *declared*:

- **DRY / one home** — each entity is declared once; each spec declares the
  entity it `owns`; the contract checks every entity has exactly one owner.
- **Naming consistency** — bindings resolve to declared entity names or they do
  not. (Free prose between bindings is not policed — see the seam below.)
- **Coverage** — every declared entity has an owning spec; a declared
  relationship with no spec is a violation.

The model is **declared, not imposed as a template.** The author writes prose
freely; what they additionally declare is the *model their prose is about* — the
essential modeling work, not ceremony. A spec landscape with a small or no model
pays little; one reconciled against code pays the modeling tax and is repaid in
coherence (`00-intent.md` honest bound: this buys coherence, never correctness —
a contract-clean spec can still model the wrong domain).

## The dependency graph — the sound payoff

Declaring entities + relationships yields a **dependency graph of intent**. Its
tier-1 payoff is sound and needs no LLM: given the graph, removing a load-bearing
entity surfaces its **blast radius** — every spec, binding, and
(via the seam) code symbol that depended on it — deterministically. This is
"fearless refactoring" (`00-intent.md` law 6) with teeth, and it is the standing
value of the whole modeling exercise independent of any judge.

## The fidelity seam — where prose exceeds its model

Prose earns its place by saying *more* than the declared model: rationale,
nuance, edge cases. That surplus is **fidelity** — does this paragraph faithfully
describe the entity it binds? — and it is undecidable, so it is `verified_by`
(tier 3, human). The declared graph shrinks fidelity into atomic, context-local
questions (chunk + its declared neighborhood) that a cheap model could judge once
calibrated per question-class (`00-intent.md` tier 2). That judged shelf is
**deferred — not a now thing** — and is forever advisory, never the hard gate.

## The cross-landscape seam — spec ⟷ code

Landscapes reference each other. A declared spec entity may require a
corresponding code symbol; the contract checks the correspondence resolves both
directions. This is flume's spec↔code equality made a *checked relation* rather
than aspirational prose-vs-types — the structural backbone the spec layer lacked.

## Scope (not sequencing)

The spec owns dependencies and scope; the **order** of work is the plan phase's
to derive from those dependencies + current code state — the spec does not dictate
a build order (depth rule, `90-spec-system.md`). The dependencies are stated
above as facts: the graph derives from the declared model; the seam relates two
landscapes; the judged tier rents the graph.

In scope now: the generic engine + the harness instance (replacing the heuristic
registry, `10-contracts.md` decision), the declared model, and the dependency
graph. **Out of current scope:** the judged fidelity tier (tier-2) — deferred,
advisory, and to be calibrated before it is trusted.
