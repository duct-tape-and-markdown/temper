+++
[provenance]
source_path = "./specs/architecture/45-governance.md"
import_hash = "0f4646b63ee3ab1cd99139191b00a6aee8d4cbf39d49471b0e88bc6171a041b8"
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

One-way edges are real, but they are **boundary phenomena** — they exist only
where temper's world touches the world it does not govern, in exactly two
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
unilateral, but with zero consumers today they are vocabulary without a
requirement; readmit only when a real consumer arrives, and as data the
obligation graph ignores.

## The world is a node — reachability is a predicate

The relation graph gains one distinguished node: **the world** — the harness
runtime and repo temper observes but does not govern. Activation facts
(`15-kinds.md`) are its edges into members; flattened pointers are members'
edges out. This makes *dead artifacts* decidable:

- **reachable** — the activation edge a member's kind declares is live. A skill
  whose `description` is empty has a dead description-trigger edge (the harness
  has nothing to load); a rule whose `paths` globs match zero files in the repo
  activates never. Each is an exact fact at check time — whether it gates, and
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
