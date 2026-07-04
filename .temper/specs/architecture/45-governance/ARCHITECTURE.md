+++
[satisfies.join]
rationale = "45-governance owns the `join` as the sole sanctioned intra-world edge (the 'coupling is a join' Decision); no other spec defines the two-sided demand/claim edge mechanism"

[satisfies.dependency-graph]
rationale = "45-governance owns the graph-scope predicates (degree/acyclic) over the relation graph — those predicates ARE the decidable blast-radius mechanism 00-intent's dependency-graph claim demands"

[satisfies.mention]
rationale = "45-governance owns the mention's edge class — the readmitted one-way annotation edge: declared, resolution-checked, obligation-free, ignored by the obligation graph — in its join taxonomy; 20-surface owns the authoring mechanics"

[provenance]
source_path = "./specs/architecture/45-governance.md"
source_hash = "f9624d34f8252f287b812e2812b087111715719623ef37d219e019f79f096bf9"
+++
# Governance — powering up the wider contract scopes

A contract has three scopes by **arity** (`05-model.md`) — over one artifact, over a
matched **set**, over the **relation graph** — not three landscapes; a single harness
draws on all three (it is itself a graph, below). The per-artifact scope is a
**package**'s (it says what a member *is*); the **set** scope (the roster of
requirements) and the **graph** scope (the model's entities + edges) are the
**assembly**'s (what the environment *contains and connects*) — the intensional/
extensional line (`40-composition.md`). The artifact scope has a rich predicate set
(`10-contracts.md`); the two wider scopes have almost none — just a requirement's
presence and a descriptive graph. But *governing* an environment is corpus-wide and
quantified — "every agent…", "at most N…", "no cycles" — which lives at exactly those
two assembly scopes. This file gives them their predicates. Every one is a **decidable
fact**, never a guess, so the gate never cries wolf (`00-intent.md` law 3): the wider
scopes gain *quantification*, not *fuzziness*.

## The set scope (the roster) — predicates over a matched set

A requirement's set is its **satisfier set** — the artifacts that opt in via
`satisfies`, kind-typed (`10-contracts.md`); there is no name-`match` selector (a name
pattern is the contract guessing, eradicated). Lift from "fill one slot" to "quantify
over the satisfier set" and the roster scope gains its predicates:

- **count** — `|artifacts satisfying R| ∈ [min, max]` ("at most N planners," "exactly
  one release-tool"). This also makes the cascade harness-economy package — cited in
  `10-contracts.md` but otherwise inexpressible — real. A whole-*kind* population
  constraint quantifies over the requirement's `kind` (every artifact of the kind);
  an intent *subset* quantifies over its opt-in satisfiers.
- **membership** — `field F of every artifact satisfying R₁ ∈ { feature G over
  artifacts satisfying R₂ }` ("every agent's `model` is one of the approved set;" "a
  hook's binary is one the manifest declares"), each set an opt-in satisfier set.
- **typed reference** — membership where R₂'s set is "artifacts of kind K conforming
  to package C": a reference resolves *to the right kind of thing*.
- **unique** — a field is unique across the satisfier set (today only `name` is).

Each is exact membership/count over deterministically-extracted features of a finite
corpus — decidable, a fact, a true positive every time.

## The graph scope (the model) — predicates over the relation graph

The dependency graph (`15-kinds.md`) is *descriptive*: it yields blast-radius.
Governance makes it *prescriptive*:

- **degree** — in/out edge counts on a node ("self-registering artifact: zero
  incoming"; "routed artifact: at least one incoming").
- **acyclic** — the reference graph has no cycle (a circular import loads nothing).

## The harness is a graph too — and references are declared edges

The graph is not spec-only. The **harness** is a graph: skills and rules coupled
to each other. To govern its shape, temper needs the edges — and inside the
world an edge is a **join**, never grepped from prose and never authored
one-sided. A rule that routes to a skill is temper's intent on *both* ends: the
rule publishes the demand (`[requirement.<name>]`), the skill opts in
(`[satisfies.<name>]`), and the gate resolves them (`20-surface.md`) —
**alongside** the prose that says the same thing for the agent. temper builds
the graph from the join; the prose rides along content-faithful; where the
harness must *execute* the coupling, the kind's emit face flattens the join
into the projection's one-way structured field (`15-kinds.md`) — derived from
the join, never authored as a pointer.

So governance does not demand you rewrite your prose — it **adds the structure it
needs and plants it.** This is why the write surface is load-bearing for
governance: a pure linter can only nag about what is on disk; temper can carry in
the structure a contract requires.

Hold one line, because it is where the declare/require cleave (`05-model.md`) is
easiest to blur: **declaring an edge and requiring it are opposite sides.** Each
half of a join is a declared clause on its own member (declare-side); the demand
that the join *agree*, or that a node's *degree* hold, or that the graph be
*acyclic*, is a predicate in the **assembly** (require-side). Same edge, two
sides — so the predicates below live in the assembly, never smuggled into the
member's own declaration.

### Decision: a reference is a declared edge on the surface, never grepped prose

**Chosen:** an artifact references another through **declared structure** —
inside the world, the paired join clauses (the Decision below); at the boundary,
syntax the external format itself executes (law 8's carve-out) — authored on the
surface and projected alongside any prose; the graph is built from these
declarations. **Rejected:** inferring edges by scanning prose
for names or paths — the unsound prose-grep `10-contracts.md`'s referential rule
forbids (it matched filenames in prose and forged false positives). The declaration
is the truth; the prose is payload. (Resolves `(skill-ref-syntax)`.)

### Decision: coupling is a join — one-way edges exist only at the governance boundary

**Chosen:** any relationship between two members of temper's world is a
**join**: two independent declarations — a published demand
(`[requirement.<name>]`) and an opt-in claim (`[satisfies.<name>]`) — that the
gate resolves, true only where both sides agree, a finding on either dangling
half (`10-contracts.md`). The fill edge is the join mechanism and there is no
second one: there is no free-standing edge clause. A member that needs another
member *publishes the demand*; the needed member *opts in* — a skill that
dispatches to a runner publishes `[requirement.lint-runner]`, and the runner
declares `[satisfies.lint-runner]` with its rationale. Delete the runner and
the demand dangles — the graph lights up, because the obligation was declared
on both ends.

One-way edges are real, but they are never **authored** in temper's
vocabulary. They appear in two places. Inside the world's own formats they are
**observed format edges**: structure a format authority documents as executed
— an `@path` import (`15-kinds.md`, "Directives") — which temper extracts as
fact and never authors; an observed edge carries no obligation either way, so
its initiator may be a member (a memory file importing a rule) without
touching the join doctrine — the gate checks it is backed, and reachability
propagates over it (below). At the **governance boundary**, where temper's
world touches the world it does not govern, they run in exactly two
directions:

- **Inbound — activation (world → member).** The harness reaches into the
  landscape by its own mechanics: it loads a skill's `description` into context
  and the body on invocation; it applies a rule when the agent reads files
  matching its `paths` globs; it loads `CLAUDE.md` in full at launch. These
  edges are the *harness's* intent, not an author's — nobody on the surface
  consents to them, and modeling them as joins would forge a signature the
  world never gave. A kind declares them as **activation facts**
  (`15-kinds.md`); the gate checks **reachability** (below), never consent.
- **Outbound — flattening (member → world).** A join whose meaning the harness
  must *execute* is projected by the kind's emit face into the one-way
  structured field the harness format defines. The pointer in the projection is
  **derived from the join**, never authored as one — and a pointer found in the
  landscape with no join behind it is a **drift finding**: un-consented intent
  sitting in the world.

**Rejected:** (a) unilateral member-to-member dependency edges — an obligation
the obligated party never declared is un-consented intent (law 8's social half)
and the invisible 2am dependency in corpus form: you delete the artifact
nothing *appears* to need, and three others dangle; (b) the `[edge.<target>]`
clause as a surface mechanism — it either duplicates a join (then author the
join) or smuggles (a); retired from `20-surface.md`'s header vocabulary; (c)
obligation-free annotation edges (a citation, a `supersedes`) — genuinely
unilateral, but with zero consumers *at the time*: vocabulary without a
requirement, readmitted only when a real consumer arrives, and as data the
obligation graph ignores. **The consumer arrived**: the **mention**
(`20-surface.md`, the 2026-07-03 reformulation) is exactly that readmission —
the Decision below.

### Decision: the mention is the readmitted one-way annotation class

**Chosen:** an authored prose interpolation of a declared value — a
**mention** — is a declared one-way edge of precisely the class rejected-(c)
above reserved: **resolution-checked** (a mention names a declared value or
it does not compile / does not pass the gate — it can never dangle),
**obligation-free** (it obligates neither end: the obligation graph ignores
it, coverage never counts it, deleting a mentioned value is not blocked by
its mentions), and **reported as citation, never fallout** (`impact` lists
mentions separately from join and reachability consequences). It satisfies
law 8 by construction — the author marks the word a reference; nothing is
mined — and its admission changes no join semantics: the join remains the
sole obligating edge. **Rejected:** (a) mentions as obligating edges — a
paragraph citing a concept must not pin that concept against refactoring;
citation-weight obligation is how documentation calcifies a model; (b)
mention edges mined from unmarked prose — the `references` retirement
(`15-kinds.md`) stays fully in force: an unmarked name is typography,
permanently; (c) mention-completeness demands — inadmissible by definition
(`20-surface.md`, the opt-in Decision; law 8's never-climb bound).

The class gains **address grain**, not new semantics: a declared one-way
edge (a mention, a citation) may target a **leaf address** (`20-surface.md`,
genre values) — resolution-checked against the manifest's serialized shape,
obligation-free exactly as above, reported by `impact` as citation at the
leaf it names. Deleting or rewording a cited leaf is never blocked by its
citations; the citer is told, which is the entire point.

## The world is a node — reachability is a predicate

The relation graph gains one distinguished node: **the world** — the harness
runtime and repo temper observes but does not govern. Activation facts
(`15-kinds.md`) are its edges into members; flattened pointers are members'
edges out. This makes *dead artifacts* decidable:

- **reachable** — the activation edge a member's kind declares is live, **or a
  reachable member imports it**: reachability closes over observed directive
  edges (`15-kinds.md`, "Directives"), the target inheriting the importer's
  liveness conditionally (imported only by an invocation-gated skill ⇒ live
  exactly when the skill is), hop-capped as the format documents. A skill
  whose `description` is empty has a dead description-trigger edge (the harness
  has nothing to load); a rule whose `paths` globs match zero files in the repo
  activates never — unless a reachable member imports it. Each is an exact
  fact at check time — whether it gates, and
  at what severity, is the **assembly's declaration**, like `degree`: the graph
  scope is the assembly's, and a package clause is artifact-scope (the
  declare/require cleave above). A dead edge can be deliberate (a
  work-in-progress skill with a blank description), so the dial is the
  author's — declared where graph-scope demands live, never smuggled into a
  member or a package.

## Worked example — self-registering vs routed

A project distinguishes two activation patterns and wants to enforce them:

- a **self-registering** artifact engages Claude through its own frontmatter + hooks
  and must **not** be pointed at → `degree(incoming) = 0`.
- a **routed** artifact is reached by being pointed at and must be reachable →
  `degree(incoming) ≥ 1`, and every route's join **agrees**.

The author declares which pattern each artifact follows; temper checks the harness
graph matches. All three are counts over declared edges — sound.

## Also in scope

- **numeric `range` `{min, max}`** over `integer` / `number` — a value bound is a
  fact, wrongly excluded when the `type` lattice rejected JSON-Schema ranges
  (`10-contracts.md`). A narrow named predicate, the corpus's own escape for a
  genuine need — now enumerated in `10-contracts.md`'s field primitives (the
  vocabulary's home).

## Held back, and loose ends

- **Conditionals** (`if field=X then require Y`) are decidable but are where proxies
  re-enter — an implication can stand in for a judgment. **Fenced** like `pattern`
  (`10-contracts.md`); admit only when a concrete sound need appears.
- Open, related: **harness-version pinning** (a `profile` declaring which format
  version a built-in kind targets, `15-kinds.md`); **`verified_by` "wired" → "wired
  and gating"** for surface-resident verifiers; and whether the flagship "a hook
  command that doesn't resolve" check is tier-1 (over a declared reference) or
  delegated.

### Decision: governance predicates are facts, never proxies

**Chosen:** every predicate here is exact and decidable over extracted features —
count, membership, degree, acyclicity, range — so each violation is a true positive
and earns the hard gate. **Rejected:** expressive governance (arbitrary logic,
unfenced conditionals, regex over the corpus) that can encode an unsound proxy.
"Too weak to lie" holds at every scope (`00-intent.md` law 3), not just the artifact
one.
