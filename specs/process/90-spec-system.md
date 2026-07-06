# Spec system — how `temper` is specified

Orientation file, not a contract spec. Adapted from cascade's spec system. The
`specs/` corpus is the evergreen source of truth for `temper`'s intent and
contract; code is the truth below the line it draws.

This file is itself the prose ancestor of a **spec-landscape contract**
(`30-landscapes.md`): the conventions below are what a declared spec contract for
`temper`'s own corpus would encode (placement, length, decisions-name-
alternatives, declared demands satisfied). The final dogfood is `temper` checking
`specs/` against that contract — the tool eating the rules it was written under.
The classed structure below is live — three placement-governed kinds, each
attaching its own floor; the declaration authoring (declared entities,
`satisfies`) is its next stage.

## What a spec is

- The source of truth for **intent and contract**. The loop and humans re-read
  them every iteration. If spec and code disagree on intent, the spec wins — fix
  the code, or refine the spec if intent has shifted.
- **Prose.** Paragraphs, lists, tables, decisions. The projected file carries no
  frontmatter, no schema, no template to fill; the declared structure — class,
  entities, `satisfies` — lives on the surface member ("The corpus is
  classed", below), never in the body. (Note the irony and hold it: `temper`
  validates *other* harnesses against declared contracts, but its own *design
  intent* is prose — because intent is the undecidable layer that contracts
  delegate, not encode. The header declares the decidable structure; the body
  stays the undecidable why.)
- **One topic per file**, filename is the topic handle. Target under ~150 lines.

## The corpus is classed — three kinds by placement

The corpus is a **system of classes of information**, and a class is a **kind**
(`15-kinds.md`): each class attaches its own floor, so class-specific demands are
ordinary clauses, and each is governed by **placement** — moving a file into a
class directory is the authored act that classes it, never a filename or shape
convention.

- **`intent`** (`specs/intent/`) — the why and the law. An intent spec
  **declares the entities it defines** in its member header; each declared
  entity is a **demand** — a member-published requirement (`10-contracts.md`,
  "a requirement's publisher") that the concept be given an architecture home.
- **`architecture`** (`specs/architecture/`) — how the model realizes intent.
  An architecture spec **satisfies** declared entities by name
  (`[satisfies.<entity>]` + rationale) — the same opt-in fill edge as every
  requirement.
- **`process`** (`specs/process/`) — how the project runs (this file).

The pairing makes authoring **intentional on each side**: an entity nobody
satisfies is a coverage finding against the declaring intent spec; a `satisfies`
naming no declared entity dangles loudly. Every edge in the corpus graph exists
because two declarations agree — never because prose mentioned a filename (law
8, `00-intent.md`). A backtick cite like `` `15-kinds.md` `` is typography for
the human reader; the graph owes it nothing.

The declarations live on the **surface member** — today the migration-era
`+++` document header under `.temper/specs/`; after the corpus's SDK
migration, its member fields (`15-kinds.md`, worked example); the projection under
`specs/` stays headerless prose, the body untouched (law 5). This is
`45-governance.md`'s move applied to our own corpus: governance does not rewrite
prose — it adds the structure it needs and plants it.

### Decision: classes are kinds, discriminated by placement

**Chosen:** each class of spec is its own custom kind (`intent`, `architecture`,
`process`), governed by its class directory and attaching its own floor — the
demand/satisfy pairing is a clause of the `intent` class's floor, never engine
behavior. **Rejected:** (a) one `spec` kind with a `class` header field — the
predicate vocabulary deliberately carries no clause conditioned on a field's
value, and adding conditionals to fake subtypes would grow the algebra to avoid
declaring a kind; (b) classing by filename convention (the `NN-` prefix) — the
numbering survives as reading order only, and enshrining it in `governs` would be
shape-matching the corpus we happen to have (law 8's slope). Migration is a
deliberate human ceremony, staged: engine support (member-published
requirements, header entity declarations) landed first; the placement reshuffle
followed as a human commit, never a build tick; the declaration authoring —
the entity declarations and their `satisfies` — is the remaining stage.

## Evergreen, not release lines

There are no `RELEASE-vN.md` ship targets (`00-intent.md` decision). `specs/` is
continuously reconciled against code: `plan` re-reads the corpus every tick,
files the gap between intent and `src/` as pending entries, and drops entries
whose work has shipped. "Done" is a moving conformance, not a frozen milestone.
New or changed intent is **human-ratified**, not human-typed by requirement:
drafting is the primary author's work (`00-intent.md`, Positioning — the
agent) at every tier, and the `specs:` ceremony is the authority moment — a
cold read over a reviewable diff, never a formality. No autonomous phase
lands spec text: build proposes (leave the entry, surface the question)
instead of writing — the loop shapes and implements intent, it never invents
it (`00-intent.md` law 4; `.claude/rules/collaboration.md`). As a document's spec
text adopts genre values (`30-landscapes.md`), an autonomous phase MAY gain
**draft rights on its leaves — propose-only**: a leaf edit is
a typed, resolution-checked, impact-visible graph event where a prose edit
is an opaque diff, so the review the ceremony rests on becomes mechanical
rather than heroic. The authority tier never loosens; only the drafting tier
widens, per document, with the structure that makes review possible.
The opt-in bound still holds: draft rights are a consequence a document's
author may collect, never a pressure to adopt. (Ruled 2026-07-03.)

The entities stay distinct, because blurring them is how process drifts
(ruled 2026-07-03): **temper is the product**; **flume is the code path** —
spec diff to plan to build, the only route code changes take; **the
dogfood** — temper applied to this repo's own harness and corpus — is a
**confirmation of a finished version, never a live constraint while the
engine under it is changing**. During an engine wave the self-check gate is
deactivated and the dogfood's committed artifacts are left stale by design;
at wave end one confirmation pass (rebuild, refresh the dogfood artifacts,
`temper check`, one commit) validates the new version and re-arms the gate. **The interactive
session builds specs with the human and governs flume — it does not
hand-execute work the pipeline owns.**

Vocabulary is ratification-grade: new coinage needs the human, and plain
words beat metaphor — the ladder vocabulary (rungs, floors, altitude, the
adoption gradient) was retired 2026-07-04 after its connotations were
repeatedly mistaken for design decisions. (Ruled 2026-07-03/04. The
cooling-period rule that once opened this paragraph was struck 2026-07-06:
ratification is the human's cold read over the diff, whenever it happens —
no mandated sleep.)

## The depth rule — how deep a spec goes

**A spec owns the contract; code owns the mechanism. State a fact in a spec only
if code changing shouldn't be free to change it.**

| Spec owns (WHAT / WHY) | Code owns (HOW) — keep OUT |
| ---------------------- | -------------------------- |
| Intent, positioning, the law | Type/field layout, signatures, internals |
| The named primitives + invariants | Parsing details, algorithms |
| Decisions + rejected alternatives | Anything an implementer can change freely without breaking intent |

Boundary test: if a detail can change as an implementation choice without
violating intent, it belongs to code. (This *is* the contract/mechanism split
the tool itself enforces — `10-contracts.md` — applied reflexively to our specs.)

## DRY — one fact, one home

Each fact lives in the most specific spec that owns it; everywhere else
**references** it. A cross-cutting law is stated once in `00-intent.md` and
referenced, never restated. Duplication invites drift — when one copy changes,
the other lies.

## External facts carry citations

A spec claim about a format the outside world owns — Claude Code's layout, a
frontmatter schema, a registry's behavior — is an **external fact**: it carries
its source (URL, retrieved date) at the point of claim, and it is verified
against current docs when written, never encoded from memory (the
`collaboration` rule; the same bar `10-contracts.md` sets for the shipped
floors' clauses). An uncited external fact is a guess wearing the corpus's authority —
derived layers implement it faithfully and ship the guess.

## Naming consistency — the one hard rule

Name the same concept the same way in every file and in the code. One concept,
one name. The **six nouns** — `harness` (the assembly value, `harness()`),
`member`, `kind`, `clause`, `requirement`, `prose` — are API in every placement
(`50-distribution.md`, no synonyms). Around them: `contract` (clause data
attached in the assembly — never a document, never a file format), `floor` (an
exported clause array — the reusable contract unit), `expect` / `require` (the
two quantifiers), `satisfies`, `needs`, `verifiedBy`, `genre` (a kind at the
block locus), `posture` (the three equal authoring forms), `registration`,
`the world`, `mention` / `embed` (the two reference intents), `emit`,
`install`, `explain`, `the lock`, `the definition` (the fixed engine axiom — reserved,
never authorable), `decidable`, `SDK`, `provenance`, `drift` are load-bearing
terms — search before coining a new one.

The migration trail, compressed (each step's pre-state is a tag): `template`
→ `package` → dissolved into **floors** (exported clause arrays; the package
noun retired 2026-07-04); `temper.toml is the contract` → `temper.toml is the
assembly` → the assembly is the **`harness()` value** and no TOML dialect
exists; name-`match` retired (opt-in `satisfies` is the sole fill);
`byte-faithful` narrowed to literal byte-for-byte copies — authored prose is
**`content-faithful`** (law 5). The `mirror-era` reformulation's coinages were
re-cut 2026-07-04 (`manifest-era` tag → the six-noun model): **authoring
face** → **SDK**; **manifest** retired (the seam's in-flight data is internal
and unnamed; the **lock** is the committed anchor); **carriage** and the
**adoption gradient** (rungs, floors-as-rank, altitude, climb) retired for the
three equal **postures**; `import` → `init`, `re-add` and `apply` retired; the
read verbs (`why` / `requirements` / `impact` / `context`) folded into
**`explain`**; `init` absorbed into **`install`** — the front door, one
question, placements follow the lock — and `temper.toml` retired entirely as
a filename (2026-07-06). This corpus's own migration onto the SDK is a staged human
ceremony ahead (`15-kinds.md`, worked example) — until it lands, these members
stay on the migration-era document headers.

## Decisions

Every Decision records what was chosen, what was rejected, and why. A decision
without rejected alternatives is incomplete — future readers can't audit it.

A Decision that dissolves or renames a standing noun also **names what the
dissolution retires** — the facets, verbs, diagnostics, and curated artifacts
that spoke it. Derived layers demolish only what the corpus names: a
dissolution recorded only as rejected prose leaves the noun alive in the
engine (the package-era residue lesson, 2026-07-06 — the noun died in
`10-contracts.md` while its facet, resolver, and conformance pass lived on
uncited by any pending entry).

## The corpus

Classed by directory; the `NN-` prefixes are reading order, never classing.

- `intent/00-intent.md` — north star: the thesis, the law, positioning, self-hosting.
- `intent/05-model.md` — the domain model: temper's concepts and how they relate.
- `intent/55-offering.md` — the offering: what ships, to whom, under what license.
- `architecture/10-contracts.md` — contracts: clauses, the two quantifiers, requirements, admissibility.
- `architecture/15-kinds.md` — the kind system: constructors + five facts, loci, genres, postures.
- `architecture/20-surface.md` — the SDK: authoring, prose, emit, the seam, init.
- `architecture/30-landscapes.md` — landscapes: engine instances, the spec model, the seams.
- `architecture/40-composition.md` — the assembly: `harness()` and its four fields.
- `architecture/45-governance.md` — powering up the wider scopes: corpus-wide, fact-only predicates.
- `architecture/50-distribution.md` — delivering the gate: plugin, CI, the fail-loud invariant.
- `process/90-spec-system.md` — this file.
