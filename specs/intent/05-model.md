# The domain model — temper's own concepts

The concept vocabulary of temper itself and how the pieces relate. Each concept is
detailed in its **owning spec**; this file owns the **relationships** between them.
Reflexively, it is the model temper's own `spec` kind would declare for this corpus
(`15-kinds.md`) — the compiler describing itself in its own types.

## The spine: two algebras, one type

temper is built from two **closed algebras** — fixed vocabularies deliberately *too
weak to lie* (`10-contracts.md`):

- the **extraction algebra** bounds what may be **read** (`15-kinds.md`);
- the **predicate algebra** bounds what may be **required** (`10-contracts.md`).

Each composes into one of two declarations: a **kind** is a declared *extraction*
(how to read a class of artifact); a **contract** is a declared set of *clauses*
(what an artifact must satisfy). They are a **pipeline, not a mirror**: extraction
reads an artifact into *features*, and a contract's predicates are checked over those
features. So extraction is the **soundness boundary** — the predicate algebra can
only require what extraction can decidably read.

A kind and its **artifact-scope contract** together are a **type** — the narrowest,
sharpest pairing. Widen what the contract quantifies over and it stops being a type
(the three arities below).

The two algebras are the **declare-side** and the **require-side** of one cleave, and
they land in distinct homes. What you *declare* — a **kind** (extraction + entities +
relationships) — is the schema-ish structure. What you *require* — a **contract**
(clauses) — is carried by a **package** bound to that kind. The **assembly**
(`temper.toml`) composes the two: it binds packages to kinds and declares the roster +
relationships that must hold across the corpus. A per-artifact type-check is one clause
in a package, not the whole of it (`00-intent.md` positioning).

```
   EXTRACTION algebra                  PREDICATE algebra
   (what may be READ —                 (what may be REQUIRED)
    the soundness boundary)
        │ composes                          │ composes
        ▼                                    ▼
      KIND ── reads artifact into ─► features ─ checked by ─► CONTRACT
                                                                 │
   kind + its artifact-scope contract = a TYPE  ◄────conformance─┘

   AUTHORING FACE ── emit (checked) ──► MANIFEST + LANDSCAPE (.claude/, specs/)
   (the typed library: members,        the inert serialization the gate reads,
    kinds, packages, the ASSEMBLY      beside the projected corpus of ARTIFACTs,
    as typed values; the floor         governed by the assembly via
    hand-writes the manifest and       REQUIREMENTS (roster) · ENTITIES (graph)
    keeps members in place)                     │
        ▲ init (one-time lift)                  │
        └───────────────────────────────  a PACKAGE binds a kind's contract (require-side)
```

## The concepts

The vocabulary; the **owning spec** carries the detail.

| Concept | Is | Owning spec |
|---|---|---|
| extraction algebra | closed vocabulary of deterministic extractors (the soundness boundary) | `15-kinds.md` |
| predicate algebra | closed vocabulary of decidable predicates | `10-contracts.md` |
| **kind** | the declare-side: a class of artifact = a declared extraction (+ optional entities/relationships) | `15-kinds.md` |
| **contract** | the require-side: clauses over a kind's features (never a synonym for `temper.toml` or a package) | `10-contracts.md` |
| clause | one predicate application; carries severity + fact/opinion | `10` / `15` |
| **package** | a reusable, bindable bundle of a kind's contract (decidable clauses) + guidance (prose, never gates), two separate channels; itself a kind, checked by the definition; shipped *or* project-authored (non-privileged) | `10-contracts.md` |
| **assembly** | the layered binding declaration: binds packages to kinds, declares requirements (the roster) + relationships-that-must-exist — authored on the face or hand-written at the floor, serialized as the manifest | `40-composition.md` |
| **authoring face** | the typed module library — members, kinds, packages, the assembly as typed values; composition is an import; `emit` compiles it | `20-surface.md` |
| **manifest** | the inert, gate-read serialization (`temper.toml` + the lock); every check, read verb, and placement consumes it offline, no runtime | `20-surface.md` |
| emit | the checked compile: authoring face → projection + manifest; byte-reproducible or it is a finding | `20-surface.md` |
| **artifact** | an instance of a kind (typed fields, content-faithful body, `extra` unknown-key catch-all, companions, provenance) | `20-surface.md` |
| **member** | the role an instance artifact plays vs a governing package/kind; carried one of three ways — **module-carried** (a typed value on the face), **document-carried** (the floor's header-over-body document), **in-place** (the landscape file itself is the member) — one feature shape however carried | `20-surface.md` |
| **mention** | an authored prose interpolation of a declared value: a one-way, resolution-checked, obligation-free edge; opt-in per word, forever (law 8) | `45-governance.md` |
| **landscape** | a corpus of artifacts (≥1 kinds) governed by the assembly | `30-landscapes.md` |
| **surface** | the authored source of truth (the face at the altitude; manifest + in-place members at the floor); compiles to the landscape | `20-surface.md` |
| projection | the on-disk landscape `emit` writes from the surface | `20-surface.md` |
| **requirement** | the demand-side end of a fill edge, published by the assembly or a member's header: a named obligation (optional `means`, typing `kind`/`package`, multiplicity), joined to a member by `satisfies` — the sole binding, no name-`match` (the contract never guesses); coverage-gated, never judged | `10-contracts.md` |
| **satisfies** | the member-side end of the same edge: an artifact's opt-in declaration (in its representation) that it fills a requirement — the sole, decidable fill | `10-contracts.md` |
| **join** | the two-sided relationship mechanism: a demand and a claim that must agree — requirement/satisfies are its ends; every intra-world edge is one (one-way edges exist only at the governance boundary) | `45-governance.md` |
| **world** | the ungoverned exterior (harness runtime, repo) as one distinguished graph node; touches members only via boundary edges — activation in (`15-kinds.md`), flattened pointers out | `45-governance.md` |
| entity / relationship | nodes / edges of a kind's dependency graph — declared in headers and structured fields, never mined from prose (law 8); member-to-member edges are joins | `15` / `45` |
| cross-landscape seam | a checked relation between two landscapes' entities (spec ⟷ code) | `30-landscapes.md` |
| verifier (`verified_by`) | external check for the undecidable remainder | `10-contracts.md` |
| behavioral contract | the prose surplus beyond the declared model (tier-3, human) | `00` / `30` |
| **provider** | the authority that defines a format a kind mirrors — a tool or a standard; the kind-identity axis (`<provider>.<name>`) | `15-kinds.md` |
| harness | the external, evolving **runtime** that consumes a landscape — the world activation edges come from; many harnesses may consume one provider's format | `15` / `45` / `50` |
| provenance | source path + import hash; the per-artifact drift anchor | `20-surface.md` |
| diagnostic | a clause violation the engine reports | `10-contracts.md` |

## The relationships

- a **kind** *reads* an **artifact** into **features**; the **contract** a **package**
  carries *validates* those features (conformance).
- an **artifact** is an *instance of* a **kind** and *lives in* a **landscape**; the
  ones a package checks play the **member** role.
- a **landscape** is governed by the **assembly** (`temper.toml`) layered over the
  by-kind built-in floor — which **binds** a **package** to each kind (dispatching its
  artifact-scope contract per member), declares **requirements** (the roster) and, when
  a kind declares entities, the **relationships-that-must-exist** (the graph).
- an author *adopts / extends / forks* a **package** and binds it in the assembly, and
  declares **custom kinds** as authored artifacts under `.temper/` (`40-composition.md`).
- a **surface** is the authored source — at the altitude, the **authoring face**
  (the typed library); at the floor, a hand-written manifest over **in-place**
  members. **emit** compiles the face to the landscape on disk plus the
  **manifest**; `init` lifts an existing harness up once; a hand-edit to the
  projection is a drift finding routed to the authored source, never
  reverse-parsed (`20-surface.md`).
- a **mention** interpolates a declared value into authored prose — a declared
  one-way edge the gate resolution-checks and the obligation graph ignores
  (`45-governance.md`); the module graph of the face and the join graph carry
  the same intent at two grains.
- landscapes reference each other: a declared **entity** in one may require a
  corresponding entity/symbol in another — the **cross-landscape seam** (spec ⟷
  code), checked both directions (`30-landscapes.md`).
- a **verifier** is named by a `verified_by` clause on a **requirement**; temper checks
  it **resolves and is wired**, never runs it (`10-contracts.md`).
- two checks: **conformance** = a landscape satisfies the assembly (an artifact
  conforms to its bound package / a roster is filled + wired / a model resolves);
  **admissibility** = the assembly and each package ⊨ the definition (the algebras +
  structural rules). Two greens (`00`).

## Contract scope is quantification arity — one engine, a widening lens

The three instances of `10-contracts.md` are the same engine widening only in *what
each predicate ranges over* — its **arity** — never in the landscape it runs in.
Only the narrowest (over one artifact) is a *type* (a kind's dual); the wider arities
are the same engine, not types. **Arity is not a landscape:** harness, spec, and code
(`30-landscapes.md`) each draw on whichever arities their contract needs — a harness
is itself a graph (`45-governance.md`) and uses all three; no landscape *owns* a
scope. `type` / `interface` / `schema` are how each arity *reads*, not what it *is*.

| Scope (arity) | predicate ranges over | reads as |
|---|---|---|
| **artifact** | one matched artifact's features | a **type** |
| **set** (a requirement's fillers) | the *set* of artifacts filling a requirement — count, membership, unique | an **interface / trait** |
| **graph** (the model) | a relation graph of declared edges — degree, acyclic | a **schema / ontology** |

## The two cross-cutting axes

**Ownership — built-in vs custom.** *Who* owns a kind / package: **built-in**
(temper-maintained — the interface to known harnesses) or **custom** (author-defined
— project kinds and packages). Not a separate mechanism; it cuts across kinds
(harness vs project) and packages (shipped vs project-authored, `base ∪ custom`) —
the *same* non-privilege for both (`15-kinds.md`).

**Verdict tier — which check resolves where.** Orthogonal to severity: **tier-1**
structural decides via a **clause** (decidable → the hard gate); **tier-2** judged
decides via a **judge** (advisory, deferred); **tier-3** intent delegates to a
**verifier** over the **behavioral contract** (human). A clause is always tier-1;
its severity (required / advisory) is the separate axis. (`00-intent.md`.)

## Reflexive

Every concept is an instance of itself: this corpus is a **landscape**; its files are
**member artifacts** of the three placement-classed **kinds** (`intent` /
`architecture` / `process`, `specs/process/90-spec-system.md`); checked by the
**contract** each class's **package** carries; and the **entities + relationships**
of this model are what those kinds would declare for
temper's own corpus. The engine governs the harness that builds it (`00-intent.md`
self-hosting) — and, here, the model that defines it.
