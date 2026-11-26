# Contract — edge · clause · selection · well-formedness

Layer 2 of the model: declared relationships and checks across members. This
layer is the gate — representation without it is a dotfiles manager.

## edge

A typed, directed relationship between two members. One noun, declared at any
of three loci:

- a **field** — a reference carried as a typed property of the member that
  speaks it, at any grain. The kind declares the field's target as a
  non-empty **set of kinds**; resolution is by identity within the target
  kind. A one-element set resolves a bare name within its one kind; a
  multi-element set requires the kind-qualified address always — resolution
  depends on the written text, never the member population,
- an **import directive** — a reference the target format itself executes (a
  memory file's `@path` import), resolved by path; a file an import reaches
  carries the directives its format executes, whatever its kind; its path
  follows the documented grammar, a backslash-escaped space part of the path
  and trailing punctuation kept literal,
- a **satisfies entry** — an edge whose target is a requirement member.

A field-declared edge may also claim a **rendered position** in its member's
prose — the **mention**: a rendering claim, never a fourth locus; its target
may be a member or a leaf. One resolution path and one degree semantics hold
whatever an edge claims about rendering. A member's reference set is a
**derived view** — the union of the edges its fields and embedded members
declare, and its **containment** of each embedded member its body composes
(one field per admitted kind, derived) — never a second authored list.

Every edge resolves into one enumeration the gate and every read verb share,
so narration never disagrees with the verdict. Edges are declared, never
mined from prose. Path-resolved edges resolve against raw disk, never the
ignore-filtered discovery view: an extra file can only suppress a finding, a
pruned one can forge one (discovery is the opposite, `adoption.md`). A
target the runtime answers from outside the corpus is never dangling: a name
the provider documents as built in (a **provided** name) resolves, and any
other name the tree cannot show (user scope, a plugin) is undecidable here
and collapses to one coverage note naming the scopes temper does not read.
An edge carries no obligation of its own: the clauses that range over it
decide whether it counts; the shipped `mention-reachable` advisory judges
only declared edges, never derived containment. What an edge **means** is
its **intent**, never checked: an edge field's guidance (its field counsel,
"clause" below), stated once, or a satisfies entry's requirement prose.
Delivered at both ends with the requirements and their satisfiers, it lets
an agent growing the contract set reason over what is contracted.

## clause

One **predicate** plus the **severity** its author declared, with optional
**guidance** and **cite** — the atom of every authored check. The tool never
decides error-versus-warning; the clause does, and the guidance channel is
how the gate teaches at the moment of failure.

**Guidance is not the clause's alone.** The same channel — advisory intent,
optionally cited — attaches directly to a **kind** or a **field**, carrying
no predicate: counsel teaching before anything can fail (invariant 8). A
corpus kind's counsel, its fields' with it, projects as a path-scoped rule
over the kind's locus (its admitting hosts', for an embedded kind), which
the runtime loads on a session's first touch of a member's document;
`schema` hover and `explain` serve the human. It never gates: advice
hardened into a numeric rule is the linter temper sits downstream of
(invariant 5). An embedded kind's counsel is **the kind's fact, stated
once**: one counsel in every body that admits it (`representation.md`,
"nesting"), never restated per host.

The predicate vocabulary is **closed**: the enum in code is the authority,
an unknown one refuses at load, never skipped; adding one is a language
change, and the corpus does not enumerate it.

A clause's **id**, its compiled label in the lock (`pipeline.md`, "The
lock"), is deterministic, human-legible, printed by every finding and by
`explain`, and never opaque to the author; every surface that names a clause
keys it `clause`. The shipped **dial** kind consumes it: a temper-owned,
local-locus TOML document (`.temper/dial.toml`) naming a clause by its id
and declaring this machine's read severity. The dial's schema is the
envelope — severity is its only verb, deletion is unspellable, a dialed
clause still reports; dial semantics and the block-mode bound live in
`authoring.md`, "Layers".

A clause binds to a **selection** and evaluates at one of two grains:

- **each** — the predicate holds of every selected item,
- **whole** — the predicate holds of the selection as a set (cardinality,
  uniqueness, membership).

A clause may carry a **guard**: one predicate from the scalar-decidable set
over an addressable field, conditioning a body of ordinary clauses. The body
binds where the guard holds and is silent where it does not, the guarded
field's absence included (absence is `required`'s to indict). Guard and body
share one address binding: each element the guard's path locates is judged
independently, and a finding names the concrete address it fired at. Guards
do not nest.

Some predicates need whole-graph context — a degree bound, a reachability
test. That is cost, not a category: **reachability** and **drift** are
clauses in the root member's default contract, advisory by default —
severities the author declares, never the tool.

## selection

The set a contract binds to. Selectors are declared, decidable expressions:

- **by kind** — every member of a kind: the universal binding,
- **by opt-in** — the members whose satisfies edge targets a requirement:
  the existential binding,
- **by incidence** — the edges at a member, filtered by direction and a
  field set; a bound ranges over the union, an unfiltered one over every
  edge but containment and satisfies, relations the selections already
  judge, each counted only where a filter names it. A selection over the
  requirement kind judges like any kind's.

A selection picks members or edges; the set predicates are one algebra over
selections, and the quantifier is the clause's grain — no separate
universal/existential machinery. Selectors are atomic and do not compose:
narrowing is an each-grain clause over a selection, never a second
selector, and a member outside it is a finding, never a silent exclusion.

## requirement — a shipped kind, not a primitive

A requirement is a member of a built-in kind (embedded locus). Its template:

- **identity** — the name satisfies edges target,
- **prose** — the authored intent the requirement exists to carry; never
  interpreted,
- a **verifier** — the typed delegate that judges the behavioral remainder,
  three shipped species: a **script** (a path-resolved reference to the
  test or CI job that executes the judgment), a **telemetry** declaration
  (named, documented harness events the emitted tap records, judged by
  reading the field record), or a **review** (the reader whose judgment
  holds it, named in prose: the author as it writes, a reviewer at the
  diff). The gate checks the declaration resolves and never runs, records
  into, or aggregates over it: no verifier's verdict enters `check`'s exit
  code. A requirement declaring none is **unheld**, and the read verbs and
  the emitted reference say so; making it a finding is a clause the author
  attaches, never a default (intent, invariant 1),
- **attached clauses** over its opt-in selection. "This must be filled" is
  the shipped default: a whole-grain cardinality clause (at least one
  satisfier) at required severity — overridable, so an advisory requirement
  is expressible.

The requirement's prose and verifier are the model's declared boundary
with the undecidable: the two slots that stay human.

## well-formedness

The only fixed checks — preconditions of judging, never opinions:

- **admissibility** — the contract is coherent before it judges anything:
  predicates in the vocabulary, no vacuous clause, no unfillable selection,
  no dangling verifier. Every finding is an error.
- **acyclicity** — the import relation is well-founded; a cyclic graph makes
  evaluation itself ill-defined.

If anyone could ever want to dial a check's severity, it is a clause.

## Read verbs

`explain` is the one read verb: it narrates a member, requirement, kind, or
leaf; its impact strand reports the deterministic set of members that break
if one is removed, and its field strand narrates the local telemetry record —
counts and denominators joined to members through the same lock declarations
the gate reads: evidence narrated, never judged. Every reading is a
projection over the same resolved edges the gate uses; it never gates.

**A finding is true for its reader.** A finding or a managed note states only
what holds in the mode it is read in (a represented harness, an
unrepresented one, an installed plugin), with a remedy that works there.
What a mode cannot decide collapses to one coverage note naming what would
decide it, never a per-member finding whose remedy cannot work.
