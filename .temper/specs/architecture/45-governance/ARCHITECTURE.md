+++
[satisfies.join]
rationale = "45-governance owns the `join` as the sole sanctioned intra-world edge (the 'coupling is a join' Decision); no other spec defines the two-sided demand/claim edge mechanism"

[satisfies.dependency-graph]
rationale = "45-governance owns the graph-scope predicates (degree/acyclic) over the relation graph — those predicates ARE the decidable blast-radius mechanism 00-intent's dependency-graph claim demands"

[satisfies.mention]
rationale = "45-governance owns the mention's edge class — the readmitted one-way annotation edge: declared, resolution-checked, obligation-free, ignored by the obligation graph — in its join taxonomy; 20-surface owns the authoring mechanics"

[provenance]
source_path = "./specs/architecture/45-governance.md"
source_hash = "73dd8798d9b9728f3d4aa7ae066b1eaa6ae4ea6a32e0fb3264c620fba49be3c0"
+++
# Governance — the wider contract scopes

A judge is a compiled predicate over (features, graph), and it runs at one of
three **scopes** — **node** (one member), **node-set** (a matched set of
members), **edge** (the relation graph) — behind one admissibility stage in the
engine's extract → assemble → judge pipeline. Scopes are the judge's
vocabulary, not modules: the same clause algebra, quantified wider. The node
scope has a rich predicate set (`10-contracts.md`); this file gives the two
wider scopes theirs. *Governing* a harness is corpus-wide and quantified —
"every agent…", "at most N…", "nothing unreachable" — which lives at exactly
these two scopes. Every predicate here is a **decidable fact**, never a guess,
so the gate never cries wolf (`00-intent.md` law 3): the wider scopes gain
*quantification*, not *fuzziness*.

## One graph

The judge sees one graph. **Nodes** are the members — at file grain and at
block grain: a genre member inside a host document is a node like any other
(`15-kinds.md`) — plus one distinguished node, **the world** (below).
**Edges** are one enumeration with several sources:

- **declared reference fields** — a member's typed field naming another member;
- **satisfies** — a member's opt-in claim against a published requirement
  (`40-composition.md`);
- **embed and mention directives** — authored prose interpolations
  (`20-surface.md`);
- **registrations** — a member's declared coupling to the world.

All of them are **declared** (law 8): an edge exists because an author put it
on a structured surface, never because prose named a thing. Declaring an edge
and demanding properties of it are opposite sides of the declare/require
cleave (`40-composition.md`): the declaration lives on the member; the demand
— a degree, a count, a reachability severity — lives in the assembly, never
smuggled into the member's own declaration.

### Decision: one reference concept, one dangling-rule family

**Chosen:** a reference is one concept — a declared edge — however it is
spelled: a reference field, a `satisfies`, an embed or mention directive. One
rule family governs resolution: every reference resolves to a node or it is a
**dangling finding**; the finding's debug label speaks the source's vocabulary
(`10-contracts.md`), but the judge is one. **Rejected:** (a) per-source
dangling checkers — a reference-field checker, a satisfies checker, a
directive checker — three implementations of one fact, drifting
independently; (b) inferring edges by scanning prose for names or paths — the
mined edge is a guess wearing tier-1 clothes, false in both directions (law
8); the declaration is the truth, the prose is payload. (Resolves
`(skill-ref-syntax)`.)

Two of the sources are **prose reference intents**, and they are distinct edge
species (`20-surface.md`): an **embed** pulls the target's content into the
referencing prose — today's `CLAUDE.md` `@path` import is the embed's harness
spelling (`15-kinds.md`) — so meaning flows along it; a **mention** names a
member and nothing flows. The species differ in consequence, not in
resolution: both are resolution-checked (a mention can never dangle past the
gate), but a mention is **obligation-free** — it obligates neither end,
coverage never counts it, deleting a mentioned member is never blocked by its
mentions — and `explain` reports mentions as **citation, never fallout**. The
citer is told, which is the entire point. An obligating mention is how
documentation calcifies a model; a paragraph citing a concept must not pin it
against refactoring.

## The node-set scope — predicates over a matched set

A requirement's set is its **satisfier set** — the members that opt in via
`satisfies`, kind-typed; there is no name-pattern selector (a name match is
the contract guessing). A whole-*kind* population demand quantifies over the
kind's member set instead; an intent *subset* quantifies over its opt-in
satisfiers. The predicates spell as fields on the requirement
(`40-composition.md`) and compile to node-set judges:

- **count** — a *measurement* over the satisfier set:
  `|satisfiers| ∈ [min, max]` ("at most N planners", "exactly one
  release-tool").
- **unique** — a field is unique across the satisfier set.
- **membership** — a field of every satisfier takes its value from a declared
  **source set** (a feature over another requirement's satisfiers, or over a
  kind's members): "every agent's `model` is one of the approved set". What
  the target must *be* is not membership's job — conformance demands ride as
  inline clauses on the expectation itself (`40-composition.md`).

### Decision: `required` is posture, `count` is measurement

**Chosen:** `required` on a requirement is the **posture declaration** — it
states what the absence of any satisfier *means*: gate-blocking or advisory
(`00-intent.md` laws 1 and 4 in one field). `count` is a **predicate** — a
measurement over the satisfier set. They are different kinds of thing and are
never merged: one answers "how does the gate treat an unfilled demand", the
other answers "how many fills are admissible". **Rejected:** folding them —
`required` as "the `count ≥ 1` shorthand" — which welds an enforcement
posture to a cardinality and leaves no spelling for "advisory, but if filled,
exactly once".

## The edge scope — predicates over the relation graph

The graph is *descriptive* by construction — it yields blast radius (below).
The edge scope makes it *prescriptive*:

- **degree** — in/out edge counts on a node. Two idioms are the documented
  cases: a **self-registering** member engages the harness through its own
  registration and must not be pointed at — `degree(incoming) = 0`; a
  **routed** member is reached by being referenced — `degree(incoming) ≥ 1`.
  General `[min, max]` degree bands are legal in principle but held to the
  entry gate: admitted when a consumer lands, not before.
- **acyclic** — the embed graph has no cycle (a circular import loads
  nothing).
- **reachable** — graph reachability from the world (below).

## The world — registration's other endpoint

The world is the harness runtime and repo temper observes but does not
govern, present in the graph as one distinguished node. **Registration** is
the fact that couples a member to it: a skill registers a description
trigger, a rule registers a path scope (or unconditional), a hook registers
for an event, an MCP server registers a connection — one fact, spelled
per-kind, declared with the kind (`15-kinds.md`). The world is the other
endpoint of every registration edge. These edges are the *harness's*
mechanics, not another member's intent — no member consents to being loaded —
so they are facts the extractor reads, never obligations between members.

**Reachability is graph reachability from the world.** The closure runs over
registration edges and then over **embed** edges — an embedded member
inherits its embedder's liveness conditionally (embedded only by an
invocation-gated skill ⇒ live exactly when the skill is), hop-capped as the
format authority documents (`15-kinds.md`). Mentions do not propagate
liveness: naming is not loading. A **dead registration that no live
embed-chain rescues is the finding**: a skill whose description is empty has
nothing for the harness to load; a rule whose path scope matches zero files
activates never. Each is an exact fact at check time — whether it gates, and
at what severity, is the **assembly's dial**, carried on the clause like any
severity (`40-composition.md`). A dead edge can be deliberate — a
work-in-progress skill with a blank description — so the dial is the
author's, declared where edge-scope demands live, never baked into the engine.

**Blast radius is the same closure inverted**: remove a node and the judge
lights every node whose reachability, reference, or requirement depended on
it. The inversion reaches block grain — a genre member's removal lights the
requirements it satisfied — and `explain` reports it, with mentions listed as
citation, separately from fallout.

## Genre members govern cross-kind — and law 8's line

A genre member is a node with the full member surface (`15-kinds.md`), so the
wider scopes quantify over it like any file: a decision block `satisfies` an
adr requirement; a `count` measures the declared decisions of a corpus; a
dangling reference from inside a block is the same finding at a finer
address. The line that keeps this out of the mining swamp is law 8's bound,
stated precisely:

- **Nothing quantifies over genre adoption in prose.** "This document should
  have declared its decisions as blocks" is inadmissible by definition —
  plain prose is a fully legal member, forever, and a demand over what prose
  *could have* declared rebuilds the mining swamp from the declaration side.
- **Structural demands over typed fields are legal.** Where a kind author
  types a field as a genre collection (`20-surface.md`, the composed
  posture), demands over that field are ordinary structure — the opt-in
  moved to the kind author, and quantifying over what *is* declared is
  exactly what this file exists to do.

## Worked example — self-registering vs routed

A project distinguishes two coupling patterns and wants to enforce them: its
self-registering members must not accumulate hidden routes
(`degree(incoming) = 0`), and its routed members must actually be routed
(`degree(incoming) ≥ 1`, every reference resolving). The author declares
which pattern each member follows; temper checks the graph matches. All of it
is counts over declared edges — sound.

## Held to the entry gate

- **General degree bands** (above) — admitted when a consumer lands.
- **Conditionals** (`if field = X then require Y`) — decidable, but where
  proxies re-enter: an implication can stand in for a judgment. Fenced;
  admitted only when a concrete sound need appears.
- **`verifiedBy` "wired" → "wired and gating"** for surface-resident
  verifiers — open, tracked with the requirement's `verifiedBy`
  (`40-composition.md`).

### Decision: governance predicates are facts, never proxies

**Chosen:** every predicate here is exact and decidable over extracted
features and declared edges — count, unique, membership, degree, acyclicity,
reachability, resolution — so each violation is a true positive and earns the
hard gate. **Rejected:** expressive governance (arbitrary logic, unfenced
conditionals, regex over the corpus) that can encode an unsound proxy. "Too
weak to lie" holds at every scope (`00-intent.md` law 3), not just the node.
