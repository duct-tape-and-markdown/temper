# Base harness — a documentation corpus as a temper program

> Internal working document, pre-intent. **Nothing in this file is
> contract**: ratification follows the horizons ceremony (`docs/horizons.md`,
> entry `(base-harness)`), and the meta-freeze holds every scheduling
> question until v0.1 ships. This is design material for the session that
> takes the bite.

## What this is

The **base harness** is a proposed standalone reference repository: a
starter harness whose documentation corpus is authored as a temper program.
It is the external dogfood — the first governed corpus that is not this
repo — and it exercises the components and the processes *together*: the
doc/spec kinds, the seven verbs, the gate placements, and the
spec → plan → build loop that keeps documentation and code in equilibrium.
Its end state is public-facing: the harness a stranger adopts as a starting
point, and the documentation of how a temper-governed corpus works,
demonstrated by the thing itself. It extends the launch demo's posture
(this repo's spec corpus governing itself — decision 0019) from
self-hosting to a corpus anyone can hold.

Reference point: *Repo-Local Documentation System for Humans & Agents*
(lukewilson2002, gist `cb48062397d8b51954034d94b8c19d6d`, retrieved
2026-07-13). An independently invented, unusually clear articulation of the
target shape: doc kinds (systems, flows, decisions, glossary), structural
contracts over them, a thin routing index, link hygiene, an agent workflow.
The base harness does not implement the gist. It encodes what the gist gets
right and inverts its one founding error.

## The founding inversion: the authority arrow

The conventional docs system — the gist included — is **code-authoritative**:
code is the implementation source of truth, docs describe it, and a per-PR
duty ("update the affected docs, reviewers verify") holds the two together.
That arrow is rational when humans author the code, because code then
embodies decisions nobody wrote down. It fails exactly when the system's own
pitch matters most: once agents author most of the code, code is *output*,
and reconciling docs toward output is reading a projection back for meaning
(`specs/intent.md`, invariant 7, at corpus scale). The agent workflow it
produces — infer intent from code because the docs merely trail it — is the
mining invariant 1 forbids, reintroduced by process.

The base harness is founded on the opposite arrow:

- **The docs are the declared intent.** Systems, flows, decisions, and terms
  are authored, typed members — the spec layer. Code reconciles toward them:
  spec → plan → build, with temper gating the corpus's structure at every
  placement.
- **Feedback returns through declared surfaces.** Reality outranks the
  corpus only via a return path that lands in human territory — an inbox
  capture, an open question, a proposed decision amendment — never by
  editing code first and letting docs chase. Every forward path is
  derivation; every return path is a declaration.
- **The per-PR duty is replaced by a maintainer.** A plan-shaped reconciler
  owns the docs↔code delta as a standing loop. Convention enforced by
  reviewer vigilance fails silently, one forgotten PR at a time — the same
  failure mode temper exists to kill at the structural layer.

## The reading that makes it work: constructed edges, not a lint of types

The pitfall to design against: treating temper as a schema validator over
found markdown — "can the predicate vocabulary express this constraint on
this file." That framing manufactures gaps that do not exist, because it
puts every check at the gate. The corpus is *constructed*: authored as a
typed program (or as layout-kind documents whose declared layout is the
typed surface), with relationships as first-class declared edges. Each
constraint lives at the earliest layer that can express it — the
determinism ladder: push every check to the most deterministic layer that
can hold it.

1. **The keystroke** — `tsc` and the schema modeline. A composed kind's
   field type can be a discriminated union; a constructor can couple fields
   that must move together. Invalid states become unrepresentable before any
   gate runs.
2. **The gate** — edges and clauses. Resolution (nothing dangles), drift
   (disk versus lock, both directions), and the declared clause set over
   selections. The gate's job is the graph, not schema archaeology.
3. **The remainder** — requirement prose, verifier edges, the reconciler
   loop, human rulings. Undecidable agreement is delegated and *named*,
   never guessed at.

Three worked examples, each a would-be "gap" under the lint reading:

- **Supersession is an edge, not a conditional field.** The gist needs
  "`superseded_by` required iff `status: Superseded`" — a cross-field
  implication no lint vocabulary should want. Encoded as intent: succession
  is a declared edge between decision members, and the state is carried by
  a *position*, not a value — a declaration types a position
  (`specs/intent.md`, invariant 1). Leaning: partition the kind by locus
  (`decisions/*.md` active, `decisions/superseded/*.md` superseded); the
  superseded kind's successor edge is unconditionally required, an ordinary
  `required` clause, and a dangling successor is an edge-resolution finding.
  Incidence clauses (`degree`) bound the succession graph. The
  alternative — field-value selection in the clause vocabulary — is a
  language change, parked as an open fork, not designed here.
- **A date is typed where it is authored.** For a composed member the
  constructor holds the shape and the artifact is a projection, gated by
  byte-drift — no pattern predicate needed, because nobody hand-authors the
  file. For a layout-source document the field rides frontmatter under the
  shipped field clauses; positional format beyond `allowed_chars` is a known
  vocabulary bound, honest and small, not a design driver.
- **"Docs and code must agree" decomposes along decidability.** Structural
  agreement — sections present, edges resolve, projections undrifted — is
  the gate's. Semantic agreement is declared where declarable (satisfies
  edges, verifier edges naming the tests that judge behavior) and owned by
  the reconciler for the rest. The residue that stays human — accepting a
  decision — stays human by design.

## The kinds

All declared through the SDK — user kinds are the same construct as
built-ins (`specs/model/representation.md`, "kind"). The default posture
(ruled 2026-07-13, deepening the first cut): doc kinds are **composed,
projected members** — the docs tree is a projected collection. Each member
is a typed value in the program, its narrative a module-adjacent markdown
file, its relationships typed fields projected to frontmatter; an edge is
authored from a member value, so a dangling reference fails in the program
before the gate runs, and the discipline that would otherwise live in a
rule is construction. The **layout-content source** posture (document as
authored home, read under its declared layout, never regenerated) is
reserved for prose-first hosts whose body sections are themselves model
structure — the glossary, a spec corpus. The three layout primitives —
prose region, field section, member collection — are that posture's whole
syntax budget; what does not fit is two kinds, or it is prose. A
reference harness demonstrates both faces and says when each is right.

- **`system`** — one area of declared behavior. Field sections for purpose
  and scope; an **invariants member collection** — each invariant an
  addressable nested member a contract can bind and a mention can target;
  an edge field section as the source map (path-resolved entries the gate
  checks against raw disk). The gist's system template's headings become
  the declared layout: the template *is* the kind.
- **`flow`** — cross-system behavior. Participant edges to `system`
  members; step prose; failure-mode sections; the same source-map edge
  section.
- **`decision`** — the ADR, re-founded. Succession edges; kind partition by
  locus for lifecycle (leaning above); rejected alternatives as prose the
  layout addresses. Decisions stay human-owned — the gist gets this right.
- **`term`** — the glossary as one layout host whose member collection
  makes each term an addressable member. Usage links are declared
  **mentions** (obligation-free by default; a contract may count them) —
  never a Title-Case scan, which would be mining.
- **`requirement`** (shipped kind) — the docs corpus publishes requirements;
  members satisfy them; verifier edges name the tests. This is the
  declared half of doc↔code agreement, and the future code-landscape join
  (`docs/horizons.md`, `(code-seam-joins)`) is its natural extension.
- **The routing index** (`AGENTS.md` / `CLAUDE.md`) — a thin composed
  member: where to look and how to work, nothing more. Whether its docs
  map can be a *derived* segment (the graph already knows every member,
  and derived facts are computed, never authored twice) or must stay
  authored prose (emit stamps nothing of its own into a projection) is a
  real fork — see below.
- Alongside all of it, the shipped Claude Code kinds (skills, rules,
  memory, hooks) — the base harness is a *whole* starter, not a docs-only
  corpus, organized by the domain architecture below (third cut).

What the encoding deletes from the gist: the `templates/` directory (the
kind's layout is the template, and the check that fails beats the example
that pleads), the per-PR docs duty (the reconciler owns it), and every
convention that would require reading prose for structure.

## The domain architecture (third cut, ruled in session 2026-07-15)

The whole-starter expansion is organized by **domain first, mechanism
second**. Provenance: the decomposition of a mature production harness
(John's, 2026-07-15) into five domains whose require block was already
domain-shaped — the base harness is that shape's seed. Standing rulings:
the prescription is example-authored, never shipped (`specs/builtins.md`,
"The domain partition": no vendor baseline exists, so any prescribed
composition is an authored contract); the DRY demonstration is the
centerpiece; the base stays simpler than the mature harness but must point
at it and grow into it organically.

**The five domains, each a requirement in the example's require block:**

- **Conduct** — how any agent behaves: epistemics, verification,
  escalation. Zero project facts; the portable domain.
- **Orientation** — what this project is and where truth lives: the map,
  the routing pointers.
- **Standards** — what correct code and change look like: the invariants
  of development.
- **Operations** — doing work to the project: build, run, verify,
  diagnose.
- **Governance** — the harness maintaining itself: the gate hooks, drift
  discipline, the growth protocol.

**The filing rule** (decidable, stated in the example's memory): pick the
domain, then the delivery tier, and the kind falls out. Domain =
requirement; delivery tier = registration channel (push = `always` /
`paths-match`, pull = `description-trigger` / `user-invoked`, enforcement
= `event`); the kind is the shelf. A `satisfies` edge crosses kinds
freely — a Conduct requirement satisfied by a rule is an authored choice
the graph shows, never a mislabeled member.

**Calibration** (ratified 2026-07-15): all five requirements
declared from day one — the skeleton is complete at the first commit —
with Conduct, Orientation, and Governance `required: true` and Standards /
Operations carrying one exemplary satisfier each, replaced by the
adopter's own. Growth is **additive, never reorganizational**: maturing
is adding satisfiers to slots that already exist, and the vendor's
trigger-driven adoption table (code.claude.com/docs/en/features-overview,
retrieved 2026-07-15) is the growth protocol, stated in the Governance
surface so the harness teaches its operator when to feed it.

**Documentation is not a harness domain.** Orientation routes to truth;
it does not restate it — a pointer can dangle (gate-checkable) but cannot
semantically rot. The docs corpus (the kinds above) is *governed content*
Orientation routes into, admitted per kind by rot exposure: **records**
(decisions) always — claims about the past cannot go stale; **definitions**
(terms) with mention-liveness; **current-state descriptions** (systems,
flows) only with outward falsifiable claims — the second cut's design
mark. Stated limit: edges catch referential rot, never semantic rot —
wiring, not meaning; descriptive prose that cannot be coupled lives
outside the gate without shame.

**The demonstrations** (third cut shipped 549969f): the DRY centerpiece
(one authored program value — the toy's verify command — projected into
the memory's map, the skill's procedure, and the settings manifest, so
cross-surface contradiction is unrepresentable, not linted; verified live,
one constant edit moving three projections in one emit); a `paths`-scoped
skill exercising the channel gate (`specs/builtins.md`, registration); and
the required-domain floor verified falsifiable (removing the conduct
member fails `check` with `requirement.unfilled`). **The skill→script reference edge** landed in two waves: the emit-side
deferral shipped (a mention addressing a declared discovery-locus kind
defers to the gate — the example's verify skill now mentions
`source:main`), and the gate-side verdict is the follow-up finding (check
inherited a pre-deferral assumption that every lock mention row is
emit-resolved; inbox 07-15). The falsifiability claim completes when that
lands. The hook→script edge stays future work (the example's hooks target
the `temper` binary, so no dangling risk exists today).

## What the dogfood must exercise

The point of building it is coverage no unit fixture gives — components and
processes together, in a repo that is not this one:

- **Layout reading end to end** — all three primitives, member collections
  with explicit keys surviving retitling, prose imports fingerprinted and
  refusing when dangling.
- **Edges at all four loci** — fields, import directives, satisfies
  entries, mentions — resolved into the one enumeration `check`, `explain`,
  and `impact` share.
- **Emit's totality both directions** — source-moved and
  projection-touched drift; reaping a byte-identical ownerless projection;
  refusal on a dangling edge before a byte is written.
- **`install` against a foreign corpus** — the yes-path conversion of an
  existing docs tree into member modules, layout documents staying home
  unconverted; re-run convergence.
- **Every placement** — keystroke (schema modeline and `tsc`), session
  start (advisory reporter), CI (`check` plus `emit --frozen`
  byte-compare), terminal, `guard` per tool call.
- **`bundle`** — the base harness as a distributable artifact; whether the
  starter ships as a plugin, a template repo, or both is a fork below.
- **The process half** — the reconciler loop running against the doc
  corpus: a spec delta producing planned work, build reconciling code
  toward the corpus, friction routing to declared capture surfaces. This
  is the part no fixture can test and the reason the dogfood exists.

## Open forks

Keyed here so ratification has its raw material; none is designed in this
file.

- `(second-corpus-scope)` — the model says exactly one governed corpus
  ships and a second is a feature, never a founding assumption
  (`specs/model/representation.md`, "Reach"). The base harness is that
  feature's first consumer. Ratifying it is an intent-level act: what does
  "corpus-generic" promise publicly, and what stays harness-first?
- `(doc-kind-ownership)` — do `system`/`flow`/`decision`/`term` ship as
  first-party kinds (a `docs` subpath beside `claude-code`), or does the
  base harness declare them as project kinds that others copy? Shipping
  them is a taste commitment the spine rule scrutinizes; copying keeps
  ownership with the adopter.
- `(derived-index)` — may a projection carry a derived member index (the
  routing map), or is that tool-authored meaning in violation of the
  verbatim law? The permission list is precedent for manifests; a markdown
  projection is not a manifest.
- `(lifecycle-encoding)` — kind-partition-by-locus versus a field-value
  selector in the clause vocabulary. Leaning: partition; the selector is
  the larger language change and wants its own admissibility argument.
- `(home-and-loop)` — the base harness repo's own control plane: does it
  carry a flume-shaped loop as shipped process documentation, or is the
  reconciler described but bring-your-own? Bears directly on "components
  and processes together."
- `(register)` — this file and the eventual public docs: the public-prose
  rule's exemption list is enumerated and does not include this primer; the
  graduation from internal primer to public documentation needs a declared
  home and register ruling.

## Bite condition

v0.1 shipped (the meta-freeze holds until the tag), then a human ruling on
`(second-corpus-scope)` — the rest of the forks hang off it. The primer's
claims about shipped behavior are checked against `specs/` as of 0019/0021;
when this file and a spec disagree, the spec is right and this file has a
bug.
