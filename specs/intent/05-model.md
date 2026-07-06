# The domain model — temper's own concepts

The concept vocabulary of temper itself and how the pieces relate. Each concept is
detailed in its **owning spec**; this file owns the **relationships** between them.
Reflexively, it is the model temper's own spec kinds declare for this corpus
(`15-kinds.md`) — the compiler describing itself in its own types.

## The spine: two algebras, two walls

temper is built from two **closed algebras** — fixed vocabularies deliberately *too
weak to lie* (`10-contracts.md`):

- the **extraction algebra** bounds what may be **read**: the SDK's typed field and
  feature constructors, lowered onto four generic mechanics — frontmatter, sections,
  fenced blocks, directives (`15-kinds.md`);
- the **predicate algebra** bounds what may be **required**: the closed clause
  vocabulary, judged at three scopes (`10-contracts.md`, `45-governance.md`).

They are a **pipeline, not a mirror**: extraction reads members into features, and
clauses are judged over (features, graph). Extraction is the **soundness boundary** —
a clause is sound only if its feature is deterministically extractable.

The cleave: what you **declare** is a **kind** — a typed constructor whose runtime
residue is five facts. What you **require** is **clause data attached in the
assembly** by two quantifiers — `expect` (universal: every member of a kind owes
these) and `require` (existential: the harness must contain a fill). A kind and the
`expect` clauses over it are a **type**; both algebras reach the author twice — the
**keystroke wall** (tsc over the SDK's plain interfaces) and the **conformance wall**
(the gate over committed artifacts plus the lock).

```
        SDK  (the only authoring surface — implements no semantics)
  members · kinds · genres · clauses · harness()     every type ERASES at the seam
        │
        ├── emit (total · byte-reproducible · refusing) ──► committed artifacts + LOCK
        │                                                    (the committed seam)
        ▼                                                          │
   plain data (in-flight, internal, version-pinned)                ▼
        ENGINE  (the only implementation: emit · lock · gate · explain)
        extract ─► assemble (one graph + the WORLD node) ─► judge (node · node-set · edge)
```

## The six nouns

**harness, member, kind, clause, requirement, prose** — the vocabulary every
placement speaks (`50-distribution.md`, no synonyms). The table adds the supporting
concepts they derive.

| Concept | Is | Owning spec |
|---|---|---|
| **harness** | the assembly value — `harness()`: members · expect · require · settings · reachability, one typed value erasing to lock rows; no configuration dialect | `40-composition.md` |
| **member** | the unit of authorship and the only source: name · prose? · satisfies? · requires? · needs? · typed flat fields; lives at file grain or block grain | `20-surface.md` |
| **kind** | a class of member: a typed constructor (plain interface, tsc-checked) plus five facts of runtime residue — label, locus, layout, registration, edge fields | `15-kinds.md` |
| **clause** | the atom of a contract: predicate · severity · guidance · cite — an SDK value that erases to compiled data; every clause decidable | `10-contracts.md` |
| **requirement** | a named obligation on the harness: means · kind · required · count? · unique? · membership? · degree? · verifiedBy?; filled only by a member's opt-in `satisfies` (the join) | `10-contracts.md` |
| **prose** | a typed field, never a wrapper: `file()` \| `` text`…` `` \| `blocks(…)`; the author's words byte-untouched (law 5); carries the two reference intents | `20-surface.md` |
| genre | a kind at the block locus: members are typed fenced blocks inside host documents — full member surface, cross-kind fills, registration inherited through the host | `15-kinds.md` |
| `expect` / `require` | the two quantifiers attaching all clause data in the assembly: universal (shape of every member of a kind) and existential (the harness contains a fill) | `40-composition.md` |
| floor | a shared contract as nothing but an exported clause array: adoption is an import, override is a spread; shipped and project floors are the same type | `10-contracts.md` |
| `satisfies` | the member-side end of the fill join: a string key naming a requirement, resolved at emit; dangling ⇒ finding | `40-composition.md` |
| `needs` | the member's declared capabilities; emit derives the settings permission list from their union — a permission is never authored twice | `20-surface.md` |
| registration | the declared member↔world edge — the kind's per-spelling fact (trigger, path scope, event, connection) over the member's data; generalizes activation | `15-kinds.md` |
| the world | the ungoverned exterior (harness runtime, repo) as one distinguished node — the other endpoint of every registration; reachability is closure from it | `45-governance.md` |
| mention / embed | the two prose reference intents: `${x}` names (an edge, obligation-free, citation never fallout); `${embed(x)}` pulls content (lock-fingerprinted); both resolution-checked | `20-surface.md` |
| directive | format-executed body syntax (the `@path` import) — admitted per-kind iff the format authority documents execution; yields observed edges | `15-kinds.md` |
| posture | the three equal authoring forms — plain prose, embedded genres, fully composed; 2 ⇄ 3 round-trips byte-stable; `init`'s lift is the move at scale | `15-kinds.md` |
| SDK | the only authoring surface: types, constructors, one JSON pipe; implements no semantics; pins its engine | `20-surface.md` |
| engine | the only implementation — emit, lock, gate, explain; kind- and schema-blind; checks with no language runtime (the stranger gate carries an embedded default program) | `20-surface.md` / `50-distribution.md` |
| emit | the total compile: every artifact a projection of members (settings and `.mcp.json` many-to-one); refuses on declare-side failures; double-emit verified | `20-surface.md` |
| the lock | tool-written, never composed: provenance rows, emit fingerprints, declaration rows — the committed seam beside the artifacts; drift is one comparison, disk vs lock | `20-surface.md` |
| `init` | the one-time lift onto the surface — per member, byte-stable on content; drift routes to the authored source, never a reverse parse | `20-surface.md` |
| guidance / cite | the clause's two data channels: teaching prose that cannot gate (no path from prose to predicate), and the external-fact source (URL + retrieved date) | `10-contracts.md` |
| severity | `required` \| `advisory`, author-declared per clause; the delivery posture at each placement is likewise declared, never baked | `10-contracts.md` / `50-distribution.md` |
| scope | what one compiled predicate ranges over: **node** (one member), **node-set** (a satisfier set or kind population), **edge** (the one graph) | `45-governance.md` |
| conformance / admissibility | the two greens: the harness satisfies its attached clauses; the declarations satisfy the fixed, engine-owned definition — one stage, before any conformance pass | `10-contracts.md` |
| verifier (`verifiedBy`) | wired delegation for the behavioral remainder: temper checks the judge resolves and is wired, never runs it | `10-contracts.md` |
| landscape | a corpus of artifacts (≥1 kinds) governed by the assembly; a new landscape is more kinds, never engine code | `30-landscapes.md` |
| cross-landscape seam | a checked relation between two landscapes' entities (spec ⟷ code) | `30-landscapes.md` |
| provider module | a provider is a module: the first party as the SDK's `claude-code` subpath face (`@dtmd/temper/claude-code`, `50-distribution.md` Decision), a third party as its own package; kind identity travels **by import, never by string** — collision is impossible | `15-kinds.md` |
| `explain` | the one read verb: every read question is one graph walked from a different corner; projections, never gates; discloses coverage | `20-surface.md` |

Dissolved by the six-noun re-cut (2026-07-04; pre-states: the `manifest-era`,
`bound-prose-era`, `mirror-era` tags): the **package** noun (an exported clause
array needs no carrier artifact, identity rules, or layering machinery); the
**manifest as the gate's corpus** (the committed seam is artifacts + lock; CI's
`emit --frozen` byte-compare is the integrity check); hand-authored **kind/package
grammar files**; the **artifact/member role split** (the member *is* the unit —
"artifact" now names only the emitted on-disk output); the **carriage gradient**
(three equal postures, no adoption ladder).

## The relationships

- a **kind** declares how its members are read (five facts over generic extraction)
  and where they live (two loci: `at(path)`, `genre(within hosts)`).
- the **assembly** lists members, attaches every demand (`expect` universal,
  `require` existential), holds the settings residual and the reachability dial.
- a **requirement** and a member's **`satisfies`** are two ends of one edge; unfilled
  and dangling are the same diagnostic from opposite sides. A member's own
  **`requires`** publishes demands from where the concept lives.
- the judge sees **one graph**: members (file and block grain) plus the world;
  edges from reference fields, `satisfies`, embeds and mentions, registrations —
  all declared, never mined (law 8).
- **reachability** is closure from the world over registration then embed edges
  (mentions never propagate liveness); **blast radius** is the same closure
  inverted, reported by `explain` with citations separate from fallout.
- **emit** compiles the whole surface to committed artifacts plus the lock;
  **drift** is disk vs lock, remedied at the authored source; CI re-runs
  `emit --frozen` and byte-compares.
- two checks, two greens: **admissibility** (declarations ⊨ the definition — the
  fixed, un-authorable algebra) then **conformance** (harness ⊨ its clauses).

## The two cross-cutting axes

**Ownership — by module.** Built-in kinds and floors ship as a published SDK module
(`@dtmd/temper/claude-code`) plus the engine's embedded default program; custom kinds are
the project's own modules, same constructor, same five facts. No privilege either
way (`15-kinds.md`).

**Verdict tier — which check resolves where.** Orthogonal to severity: **tier-1**
structural decides via a compiled clause (decidable → the hard gate); **tier-2**
judged is deferred; **tier-3** intent delegates to a wired **verifier** over the
behavioral contract (human territory). A clause is always tier-1; its severity is
the separate axis (`00-intent.md`).

## Reflexive

Every concept is an instance of itself: this corpus is a **landscape**; its files
are **members** of the three placement-classed custom kinds (`intent` /
`architecture` / `process`, `90-spec-system.md`); its decisions are **genre**
members-in-waiting (the postures stay equal); the clauses over it ride the
project's own module. The engine governs the harness that builds it
(`00-intent.md` self-hosting) — and, here, the model that defines it.
