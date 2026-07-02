+++
[provenance]
source_path = "./specs/90-spec-system.md"
import_hash = "31187625e91d6535f8214ba3804aa80c4c39bb483ffa69db5f11076f77c86f52"
+++
# Spec system — how `temper` is specified

Orientation file, not a contract spec. Adapted from cascade's spec system. The
`specs/` corpus is the evergreen source of truth for `temper`'s intent and
contract; code is the truth below the line it draws.

This file is itself the prose ancestor of a **spec-landscape contract**
(`30-landscapes.md`): the conventions below are what a declared spec contract for
`temper`'s own corpus would encode (placement, length, decisions-name-
alternatives, declared demands satisfied). The final dogfood is `temper` checking
`specs/` against that contract — the tool eating the rules it was written under.
The kind exists (`15-kinds.md`, worked example); the classed structure below is
its next stage.

## What a spec is

- The source of truth for **intent and contract**. The loop and humans re-read
  them every iteration. If spec and code disagree on intent, the spec wins — fix
  the code, or refine the spec if intent has shifted.
- **Prose.** Paragraphs, lists, tables, decisions. The projected file carries no
  frontmatter, no schema, no template to fill; the declared structure — class,
  entities, `satisfies` — lives in the surface member's header ("The corpus is
  classed", below), never in the body. (Note the irony and hold it: `temper`
  validates *other* harnesses against declared contracts, but its own *design
  intent* is prose — because intent is the undecidable layer that contracts
  delegate, not encode. The header declares the decidable structure; the body
  stays the undecidable why.)
- **One topic per file**, filename is the topic handle. Target under ~150 lines.

## The corpus is classed — three kinds by placement

The corpus is a **system of classes of information**, and a class is a **kind**
(`15-kinds.md`): each class binds its own package, so class-specific demands are
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

The declarations live in the **surface member's header** (`.temper/specs/` —
the authored home, already a `+++`-headed member document); the projection under
`specs/` stays headerless prose, the body untouched (law 5). This is
`45-governance.md`'s move applied to our own corpus: governance does not rewrite
prose — it adds the structure it needs and plants it.

### Decision: classes are kinds, discriminated by placement

**Chosen:** each class of spec is its own custom kind (`intent`, `architecture`,
`process`), governed by its class directory and binding its own package — the
demand/satisfy pairing is a clause of the `intent` class's package, never engine
behavior. **Rejected:** (a) one `spec` kind with a `class` header field — the
predicate vocabulary deliberately carries no clause conditioned on a field's
value, and adding conditionals to fake subtypes would grow the algebra to avoid
declaring a kind; (b) classing by filename convention (the `NN-` prefix) — the
numbering was scaffolding, not intent, and enshrining it in `governs` would be
shape-matching the corpus we happen to have (law 8's slope). Migration is a
deliberate human ceremony, staged: engine support (member-published
requirements, header entity declarations) lands first; the placement reshuffle
and header authoring follow as `dogfood:` commits, never build ticks.

## Evergreen, not release lines

There are no `RELEASE-vN.md` ship targets (`00-intent.md` decision). `specs/` is
continuously reconciled against code: `plan` re-reads the corpus every tick,
files the gap between intent and `src/` as pending entries, and drops entries
whose work has shipped. "Done" is a moving conformance, not a frozen milestone.
New or changed intent is authored by the human in interactive sessions, never by
an autonomous phase — the loop shapes and implements intent, it never invents it
(`00-intent.md` law 4; `.claude/rules/collaboration.md`).

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
`collaboration` rule; the same bar `10-contracts.md` sets for built-in package
clauses). An uncited external fact is a guess wearing the corpus's authority —
derived layers implement it faithfully and ship the guess.

## Naming consistency — the one hard rule

Name the same concept the same way in every file and in the code. One concept,
one name. `contract` (the clause-set / require-side *only* — never `temper.toml`,
never the bundle), `package` (the reusable bundle carrying a kind's contract +
guidance — the term that retired `template`), `assembly` (`temper.toml`: bindings +
roster + relationships), `member` (an instance artifact; a role, not a directory),
`requirement` (a named obligation — the retired `role` folded into it), `satisfies`,
`verified_by`, `the definition` (the fixed engine axiom — reserved, never a
package's contents), `kind`, `decidable`, `surface`, `provenance`, `drift` are
load-bearing terms — search before coining a new one. Note the recent migration:
`template` → `package`; `temper.toml is the contract` → `temper.toml is the
assembly`; name-`match` retired (opt-in `satisfies` is the sole fill);
`byte-faithful` narrowed to literal byte-for-byte copies (companions, the
deterministic projection) — authored prose is **`content-faithful`** (law 5:
never reworded, synthesized, or dropped).

## Decisions

Every Decision records what was chosen, what was rejected, and why. A decision
without rejected alternatives is incomplete — future readers can't audit it.

## The corpus

- `00-intent.md` — north star: the thesis, the law, positioning, self-hosting.
- `05-model.md` — the domain model: temper's concepts and how they relate.
- `10-contracts.md` — the contract model, the decidable algebra, packages, admissibility.
- `15-kinds.md` — the kind system: the extraction algebra, built-in vs custom kinds.
- `20-surface.md` — the composition write surface: compose, import, project, drift.
- `30-landscapes.md` — landscapes: engine instances, the spec model, the seams.
- `40-composition.md` — authoring the harness: the assembly (bindings + roster).
- `45-governance.md` — powering up the wider scopes: corpus-wide, fact-only predicates.
- `50-distribution.md` — delivering the gate: plugin, CI, the fail-loud invariant.
- `90-spec-system.md` — this file.
