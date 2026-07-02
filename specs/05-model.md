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

   SURFACE ── compose, then project ──► LANDSCAPE (.claude/, specs/, code)
   (authored source of truth:          a corpus of ARTIFACTs (≥1 kinds),
    temper.toml = the ASSEMBLY,        governed by the assembly via
    .temper/ = kinds incl. package)    REQUIREMENTS (roster) · ENTITIES (graph)
        ▲ import / re-add                       │
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
| **assembly** | `temper.toml`: binds packages to kinds, declares requirements (the roster) + relationships-that-must-exist — the extensional set/graph-arity layer | `40-composition.md` |
| **artifact** | an instance of a kind (typed fields, content-faithful body, `extra` unknown-key catch-all, companions, provenance) | `20-surface.md` |
| **member** | the role an instance artifact plays (a `skill`/`rule`/`spec`) vs a governing package/kind; a role, not a directory | `20-surface.md` |
| **landscape** | a corpus of artifacts (≥1 kinds) governed by the assembly | `30-landscapes.md` |
| **surface** | the authored source of truth; projects to the landscape | `20-surface.md` |
| projection | the on-disk landscape `apply` writes from the surface | `20-surface.md` |
| **requirement** | the demand-side end of a fill edge, published by the assembly or a member's header: a named obligation (optional `means`, typing `kind`/`package`, multiplicity), joined to a member by `satisfies` — the sole binding, no name-`match` (the contract never guesses); coverage-gated, never judged | `10-contracts.md` |
| **satisfies** | the member-side end of the same edge: an artifact's opt-in declaration (in its representation) that it fills a requirement — the sole, decidable fill | `10-contracts.md` |
| entity / relationship | nodes / edges of a kind's dependency graph — declared in headers and structured fields, never mined from prose (law 8) | `15` / `45` |
| cross-landscape seam | a checked relation between two landscapes' entities (spec ⟷ code) | `30-landscapes.md` |
| verifier (`verified_by`) | external check for the undecidable remainder | `10-contracts.md` |
| behavioral contract | the prose surplus beyond the declared model (tier-3, human) | `00` / `30` |
| harness | the external, evolving format a built-in kind-group mirrors | `15` / `50` |
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
- a **surface** is the authored source (`temper.toml` + `.temper/`); `apply`
  **projects** it to the landscape on disk; `import` / `re-add` reconcile back (drift,
  `20-surface.md`).
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
**member artifacts** of the custom `spec` **kind**; checked by the **contract** the
`spec` **package** carries; and the **entities + relationships** of this model are
what that kind would declare for
temper's own corpus. The engine governs the harness that builds it (`00-intent.md`
self-hosting) — and, here, the model that defines it.
