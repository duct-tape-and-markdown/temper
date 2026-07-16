# Contract — edge · clause · selection · well-formedness

Layer 2 of the model: declared relationships and checks across members. This
layer is the gate — representation without it is a dotfiles manager.

## edge

A typed, directed relationship between two members. One noun, authored at any
of four loci:

- a **field** — a frontmatter reference, resolved by identity within the
  target kind,
- an **import directive** — a reference the target format itself executes
  (a memory file's `@path` import), resolved by path,
- a **satisfies entry** — an edge whose target is a requirement member,
- a **mention** — a reference planted on a word of prose; its target may be
  a member or a leaf.

Every edge resolves into one enumeration that the gate and every read verb
share, so narration can never disagree with the verdict. Edges are declared,
never mined from prose. Path-resolved edges resolve against the filesystem
the harness actually reads — raw disk, never the ignore-filtered discovery
view: an extra file in the resolution set can only suppress a finding, while
pruning one can forge a finding. (Member discovery is the opposite case and
honors ignore rules — `pipeline.md`.) An edge carries no obligation of its own: whether an
edge is counted is decided by the clauses that range over it. A mention is
obligation-free by default — no shipped clause counts it; a contract may.

## clause

One **predicate** plus the **severity** its author declared, with optional
**guidance** and **cite** — the atom of every authored check. The tool never
decides error-versus-warning; the clause does, and the guidance channel is
how the gate teaches at the moment of failure.

The predicate vocabulary is **closed**: the enum in code is the authority, an
unknown predicate is rejected at load, never skipped, and adding one is a
deliberate language change. The corpus does not enumerate it (equal
representation, `../process/spec-system.md`).

A clause binds to a **selection** and evaluates at one of two grains:

- **each** — the predicate holds of every selected item,
- **whole** — the predicate holds of the selection as a set (cardinality,
  uniqueness, membership).

Some predicates need whole-graph context to evaluate — a degree bound, a
reachability test. That is evaluation cost, not a category: **reachability is
a clause** in the root member's default contract, on by default and
overridable like any shipped clause, because it carries a dialable severity
and the spine rule sends every dialable check to a contract.

## selection

The set a contract binds to. Selectors are declared, decidable expressions:

- **by kind** — every member of a kind: the universal binding,
- **by opt-in** — the members whose satisfies edge targets a requirement:
  the existential binding,
- **by incidence** — the edges at a member, filtered by field and direction.

A selection picks members or edges, and the set predicates are one algebra
over selections. There is no separate universal/existential machinery: the
quantifier is the clause's grain. Selectors are atomic and do not compose:
narrowing a selection — a requirement whose satisfiers must all be skills —
is an each-grain clause over it, never a second selector. A member outside
the narrowing is a finding, never a silent exclusion.

## requirement — a shipped kind, not a primitive

A requirement is a member of a built-in kind (embedded locus — it lives in
the assembly and the lock). Its template:

- **identity** — the name satisfies edges target,
- **prose** — the authored intent the requirement exists to carry; never
  interpreted,
- a **verifier edge** — a path-resolved reference to the test or eval that
  judges the behavioral remainder; the gate checks the edge resolves, never
  runs the target,
- **attached clauses** over its opt-in selection. "This must be filled" is
  the shipped default: a whole-grain cardinality clause (at least one
  satisfier) at error severity — overridable, so an advisory requirement
  ("warn until something fills this") is expressible.

The requirement's prose and verifier edge are the model's declared boundary
with the undecidable: the two slots that stay human.

## well-formedness

The only fixed checks — preconditions of judging, never opinions:

- **admissibility** — the contract is coherent before it judges anything:
  predicates in the vocabulary, no vacuous clause, no unfillable selection,
  no dangling verifier edge. Every finding is an error.
- **acyclicity** — the import relation is well-founded; a cyclic graph makes
  evaluation itself ill-defined.

The boundary is sharp: if anyone could ever want to dial a check's severity,
it is a clause, not well-formedness.

## Read verbs

`explain` is the one read verb: it narrates a member, requirement, or leaf,
and its impact strand reports the deterministic set of members that break
if one is removed. Every reading is a projection over the same resolved
edges the gate uses; it never gates.
