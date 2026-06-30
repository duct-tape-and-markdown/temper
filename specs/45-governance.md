# Governance — powering up the wider contract scopes

A contract has three scopes by **arity** (`05-model.md`) — over one artifact, over a
matched **set**, over the **relation graph** — not three landscapes; a single harness
draws on all three (it is itself a graph, below). The per-artifact scope has a rich
predicate set (`10-contracts.md`); the **set** scope (the roster of roles) and the
**graph** scope (the model's entities + edges) have almost none — just `role` and a
descriptive graph. But *governing* an environment is corpus-wide and quantified —
"every agent…", "at most N…", "no cycles" — which lives at exactly those two scopes.
This file gives them their predicates. Every one is a **decidable fact**, never a
guess, so the gate never cries wolf (`00-intent.md` law 3): the wider scopes gain
*quantification*, not *fuzziness*.

## The set scope (the roster) — predicates over a matched set

A role already selects artifacts by a decidable `match` (`10-contracts.md`). Lift
the selector from "fill one slot" to "quantify over the matched set," and the roster
scope gains its predicates:

- **count** — `|artifacts matching S| ∈ [min, max]` ("at most N agents," "exactly
  one planner"). This also makes the cascade harness-economy template — cited in
  `10-contracts.md` but currently inexpressible — real.
- **membership** — `field F of every artifact matching S₁ ∈ { feature G over
  artifacts matching S₂ }` ("every agent's `model` is one of the approved set;" "a
  hook's binary is one the manifest declares").
- **typed reference** — membership where S₂ is "artifacts of kind K conforming to
  contract C": a reference resolves *to the right kind of thing*.
- **unique** — a field is unique across the matched set (today only `name` is).

Each is exact membership/count over deterministically-extracted features of a finite
corpus — decidable, a fact, a true positive every time.

## The graph scope (the model) — predicates over the relation graph

The dependency graph (`15-kinds.md`) is *descriptive*: it yields blast-radius.
Governance makes it *prescriptive*:

- **degree** — in/out edge counts on a node ("self-registering artifact: zero
  incoming"; "routed artifact: at least one incoming").
- **acyclic** — the reference graph has no cycle (a circular import loads nothing).

## The harness is a graph too — and references are declared edges

The graph is not spec-only. The **harness** is a graph: skills and rules pointing at
each other. To govern its shape, temper needs the edges — and an edge is a
**declared field on the surface**, never grepped from prose. A rule routes to a
skill by a structured field (`routes_to: standards`), authored on the composition
surface (`20-surface.md`) **alongside** the prose that says the same thing for the
agent. temper builds the graph from the field; the prose rides along byte-faithful;
`apply` projects both into the project.

So governance does not demand you rewrite your prose — it **adds the structure it
needs and plants it.** This is why the write surface is load-bearing for
governance: a pure linter can only nag about what is on disk; temper can carry in
the structure a contract requires.

### Decision: a reference is a declared edge on the surface, never grepped prose

**Chosen:** an artifact references another through a **declared structured field**
(the reference syntax), authored on the surface and projected alongside any prose;
the graph is built from these edges. **Rejected:** inferring edges by scanning prose
for names or paths — the unsound prose-grep `10-contracts.md`'s referential rule
forbids (it matched filenames in prose and forged false positives). The field is the
truth; the prose is payload. (Resolves `(skill-ref-syntax)`.)

## Worked example — self-registering vs routed

A project distinguishes two activation patterns and wants to enforce them:

- a **self-registering** artifact engages Claude through its own frontmatter + hooks
  and must **not** be pointed at → `degree(incoming) = 0`.
- a **routed** artifact is reached by being pointed at and must be reachable →
  `degree(incoming) ≥ 1`, and every route **resolves**.

The author declares which pattern each role follows; temper checks the harness graph
matches. All three are counts over declared edges — sound.

## Also in scope

- **numeric `range` `{min, max}`** over `integer` / `number` — a value bound is a
  fact, wrongly excluded when the `type` lattice rejected JSON-Schema ranges
  (`10-contracts.md`). A narrow named predicate, the corpus's own escape for a
  genuine need.

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
