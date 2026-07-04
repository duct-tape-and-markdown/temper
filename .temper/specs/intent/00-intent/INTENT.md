+++
[requirement.the-wedge]
means = "the zero-config `check --harness` entry point that lints an unconfigured harness with no assembly must be given an architecture home"
kind = "architecture"
required = true

[requirement.gate-severity]
means = "law 1 (gate, don't lint) — a finding carries a blocking-or-advisory severity that is declared, not baked — must be realized by an architecture mechanism"
kind = "architecture"
required = true

[requirement.dependency-graph]
means = "the claim that removing a load-bearing entity surfaces its blast radius — a decidable graph over the corpus — must have an architecture realization"
kind = "architecture"
required = true

[requirement.behavioral-contract]
means = "the fidelity seam distinguishing tier-1 structural fidelity from tier-3 prose surplus must be given an architecture home"
kind = "architecture"
required = true

[requirement.drift-engine]
means = "the drift/projection engine — 'core, not optional': emit determinism, projection freshness against the lock, and hand-edits surfaced as findings routed to the authored source — must be given an architecture home"
kind = "architecture"
required = true

[provenance]
source_path = "./specs/intent/00-intent.md"
source_hash = "8f401011bbe3e24636485a822d2faffff925f240c84448b7e330dc5132f9cb5c"
+++
# Intent — the north star

`temper` is a **type system for agentic harnesses.** It treats the harness — the
Claude Code (and, in time, any agent's) customization layer: skills, commands,
agents, hooks, MCP/LSP servers, `CLAUDE.md` rules, plugin & marketplace
manifests, settings — as a typed codebase you compile, not a pile of loose files.

This file is the why and the cross-cutting law. Everything under `specs/` is a
contract spec governed by it; orientation files (`90-spec-system.md`) are not.

## The thesis: the Rust of agentic harnesses

Rust relocates failure from runtime (the 2am production incident) to author-time
(the red squiggle), and earns it by checking your code against the **types you
declared** — not against some built-in notion of "good code." `temper` does the
same for harnesses: it moves harness failure — a skill that under-triggers, a
rule that silently doesn't load, a hook command that doesn't resolve, drift you
never noticed — from *silent agent misbehavior at runtime* to a *finding at
author-time*, and it earns soundness the same way Rust does: **by checking the
harness against a contract the author declares, never against the tool's taste.**

The agentic 2am page: the agent quietly did the wrong thing because the harness
was malformed, and you learned it from the output, not a squiggle. `temper`
front-loads that to author-time.

## The law

These bind every part of the tool and every change to it.

1. **Gate, don't lint.** `temper` is a gate you pass, not a linter you maybe run.
   Where blocking is cheap — CI, the author's terminal, the keystroke schema — a
   failing contract **hard-fails**. At **session start**, where a hard block would
   be hostile to a live session, the gate is **advisory**: the harness checks
   itself and *surfaces* the verdict, instructing the agent to notify the user and
   get approval before continuing. Failure is never *silently* passed; the
   enforcement **posture at each placement is author-declared** (`10-contracts.md`),
   not baked. "Complete" means *fills its declared contract*.

2. **Determinism comes from the author's declaration, not the tool's opinion.**
   `temper` ships no built-in judgment about what a good skill/rule/harness is.
   It gives the author a way to **declare a contract** and checks conformance.
   The author writes the types; `temper` is the type checker. Built-in "best
   practices" exist only as **packages** — reusable, bindable data (clauses +
   guidance), adopted by choice, overridable, and *project-authorable as peers* —
   never as hardcoded checks. (`10-contracts.md`.)

3. **Decidable clauses only — the immune system.** A check enters `temper` *iff*
   it is expressible as a decidable contract clause over the fixed primitive
   algebra. No fuzzy heuristic, no "just this once" escape hatch, no arbitrary
   code in a contract. A property that cannot be reduced to a decidable predicate
   is **behavior**, and behavior is delegated (`verified_by`), never guessed. The
   moment `temper` guesses, it produces false positives, and a gate that cries
   wolf gets disabled. This rule is what keeps the project out of the heuristic
   swamp it was started to escape.

4. **`temper` validates structure; it never adjudicates intent.** It checks that
   the harness fills the contract the human declared. It never decides the
   harness is "missing" something the human didn't ask for — that is the human's
   to declare. Surface gaps; do not fill them. (Mirrors `collaboration` rule.)

5. **Content-faithful authorship; deterministic projection; provenance is
   load-bearing.** `temper` never rewords, synthesizes, or drops authored prose —
   the words are the human's, wherever they sit. Projection is byte-deterministic
   and idempotent: the same surface emits the same landscape, every time — and
   **emit**, the compile from the SDK to the harness artifacts themselves
   (`20-surface.md`), is byte-reproducible and mechanically checked, so
   nondeterminism in authoring code is detected, never trusted. A migration into
   the surface (`init`'s lift) is one-time — free to normalize framing, never to
   alter content; after it the surface is the single authored home. The lock
   carries every member's provenance fingerprints — the drift anchor.
   (`20-surface.md`.)

6. **Fearless harness refactoring.** Because conformance is checkable, the author
   can reshape, dedupe, and compose the harness and re-verify — the original
   "work with the config surface, reshape and organize" goal. Empty without the
   drift/projection engine, so that engine is core, not optional.

7. **Compose everything; gate the decidable.** The write surface is total, and
   the **SDK is that surface**: the author composes the whole harness there —
   structure *and* prose — and `temper` **emits** it into the project. The gate is narrow: it
   blocks only on the decidable tier and on whether each `verified_by` is wired.
   `temper` never synthesizes prose and never adjudicates it — prose is the human
   **behavioral contract** (tier 3, below), enforced by its wired verifier, never
   by the tool's taste. Write-total, check-bounded: the editor writes all your
   code; the type checker only checks its types. (`20-surface.md`.)

8. **The model is declared, never mined.** An entity or relationship exists
   because an author declared it on a structured surface — a header clause, a
   frontmatter field, the assembly, an authored interpolation in prose (a
   **mention**, `20-surface.md` — the author marks the word a reference;
   nothing is grepped), or a syntax the external format itself
   executes (a `CLAUDE.md` `@path` import is the harness's structure, not
   prose) — never because prose merely named a thing. Inter-artifact
   relationships are **authored intent**, not mechanics
   the tool reconstructs from bodies; a mined edge is a guess wearing tier-1
   clothes, false in both directions (an unmarked mention is not an edge; an
   edge is not always mentioned), and law 3's false-positive machine by another
   door. A check may read authored content (a line count, a marker in a
   section); it may never derive model *structure* from it. The bound runs
   both ways: a mention is opt-in per word, forever — plain prose is a fully
   legal member, and a completeness demand over mentions ("this paragraph
   should have modeled its nouns") would rebuild the mining swamp from the
   declaration side; law 4 holds at the finest grain. (`45-governance.md`
   decides the reference case; this is the cross-cutting law it instantiates.)

## One engine, every layer an instance

There is one **contract engine** built from a fixed algebra of decidable
primitives (`10-contracts.md`). It knows nothing domain-specific. The harness is
not a special case — it is the *first instance* expressed in those primitives.
The spec corpus is *another instance*. Code (its types checked by its compiler)
is a third. `temper` governs a **landscape** — any corpus of authored artifacts —
by validating it against a contract declared in the primitives (`30-landscapes.md`).
"One source of truth across all dimensions" = one engine over N landscapes, with
the seams between them (a spec entity ⟷ a code symbol) checkable as cross-
landscape relations. The slice-1 heuristic registry is unrepresentable here:
there is no `if` to hide an opinion in — every opinion is declared data.

## Three verdict tiers — the floor is not flat

A check resolves at one of three tiers; never blur them (blurring is how the
heuristic swamp returns through an expensive door):

1. **Structural / declared — sound, deterministic → the hard gate** (rustc). The
   declared model is coherent; the artifact conforms; references resolve.
2. **Judged fidelity — cheap LLM judge over *atomized* questions, non-
   deterministic → advisory / voted, never the hard gate** (no Rust analogue).
   Unlocked only by the declared dependency graph, which shrinks a global "is
   this faithful?" into local, context-complete atoms a small model can judge
   reliably *once calibrated per question-class*. **Deferred — not a now thing.**
3. **Intent fidelity / prose surplus — undecidable → human** (`verified_by`). The
   meaning prose adds beyond its declared model. Declaration shrinks this to its
   minimum; it never eliminates it.

## The graph — fearless refactoring with teeth

Declaring the model yields a **dependency graph of intent** that is prose today.
Its standing payoff needs no LLM: remove a load-bearing entity and the graph
lights up every spec, binding, and code symbol that depended on it — the blast
radius, deterministically. This is law 6 (fearless refactoring) made literal, and
it is tier-1 sound. (The judged tier rents space *on* this graph; the graph
stands on its own without it.)

## Positioning

The **product** is the **SDK — a typed authoring surface for the agent
harness** (`20-surface.md`): the author composes the whole harness as typed
values — the **assembly** binding **packages** to **kinds** over the authored
**members** — and `temper` **emits** it into the project as the harness files
themselves. The engine underneath is infrastructure: the enforcement core that
makes the surface safe. The **typed gate** is the *differentiating
guarantee* that model uniquely offers: because the surface is declared, malformed
harness config is caught at author-time, not at the 2am page. The declarative model is
why you reach for `temper`; the gate is why you reach for it over a dotfiles manager.
The two ship together — the moat and the motivation in one repo — and neither the
type-system framing above nor this one demotes the other: the type checker is *how* the
guarantee is earned (`05-model.md`).

`rulesync` makes a harness *portable* across assistants. `skills`/marketplaces
*distribute* artifacts. `temper` makes a harness *correct* — a Claude-Code-native
(then agent-agnostic) contract system on the maintenance/quality axis. Different
layer; `temper` can sit downstream of the others, checking what you installed.

The **primary author** of a harness is, increasingly, the agent itself — Claude
maintaining its own skills, rules, and memory files. That is the audience the
surface is designed for, and it sharpens both halves of the product: agents are
demonstrably poor at self-authoring harness artifacts unprompted, so the **gate**
catches the structural failures they most commonly commit, and a package's
**guidance** channel delivers best practice just-in-time at the moment of
authorship — to the author who needs it most and retains it least
(`10-contracts.md`). The human sets the contract; the agent authors under it;
`temper` holds the line between them.

## Honest bound on the analogy

Rust's guarantees are *sound* because conformance-to-declared-types is provable.
"Good harness" is **not** provable, and `temper` must never pretend it is. What
*is* provable is conformance to a declared contract — so that is all `temper`
asserts. The undecidable remainder (does this skill trigger well? does this tool
work?) is delegated to verifiers the contract requires to be *wired*, and whose
*passing* is checked by execution (tests/CI/eval), never by `temper`.

## The proof: self-hosting

`temper` is built — right now — by an agentic harness (flume) reading a harness
we are still correcting, because `temper` doesn't yet exist to check it. The
classic compiler bootstrap. The finish line is two greens on `temper`'s own
surface, projected to its own `.claude/`: the harness **conforms** to its
assembly (the packages it binds) and that assembly and its packages are
**admissible** against the definition (`10-contracts.md`). Then the next flume loop refuses to run until both hold. Consumption is the same
two greens aimed outward — the plugin a stranger installs to gate their harness is
the one that gates ours (`50-distribution.md`); there is no separate external
finish line. When `temper` governs its own builder, the thesis stops being a slogan.

## Decision: evergreen spec, not release lines

**Chosen:** one living `specs/` corpus, continuously reconciled against code
(`90-spec-system.md`). **Rejected:** flume-style `RELEASE-vN.md` ship lines —
they frame the work as a sequence of finished targets, but `temper` is a design
in motion whose contract model is still deepening; a frozen release target would
lie about that. Plan reconciles code↔specs every tick; there is no "done with a
release," only "code conforms to current intent, or it doesn't."

## Decision: a composition write surface, not a downstream linter

**Chosen:** `temper` is the surface the author *composes the harness from* — model
in the SDK across every landscape, emit into the project, gate the
result (`20-surface.md`, `40-composition.md`). **Rejected:** the downstream-linter
framing — author `.claude/` by hand elsewhere, `temper` only reads and grades it.
The linter framing makes `temper` optional and external to the work and strands
law 6 (you cannot fearlessly refactor a surface you do not author); the
composition framing makes `temper` the place the harness *lives*. The checker is
one face of that surface, never the whole tool.

## Decision: the SDK is the authored surface; the engine reads the harness

**Chosen:** the authoring medium is a **typed module library** — the **SDK**
(`20-surface.md`): members, kinds, packages, and the assembly authored as
typed values that **emit** compiles into **the harness artifacts themselves**,
committed beside their source. The member schema survives as the compiler's
**in-memory interchange** and the contract fixtures' schema
(`50-distribution.md`) — never a committed file. All Turing-completeness is
quarantined at authoring time; the engine, CI, and every read verb consume
only committed artifacts and the lock, offline, with no language runtime —
law 3's "no `if` to hide an opinion in" holds where opinions are *checked*,
which is the only place it must. Authoring — and editing a compiled harness,
which is the same act — requires the Node toolchain this audience already
carries; **checking never does**. (Re-cut 2026-07-04: the prior cut's
committed manifest was a compiler intermediate promoted to a product —
self-attested integrity, an unread review surface — and its hand-authorable
"floor" defended an authoring persona absent from this product's audience.
Pre-states: the `mirror-era`, `bound-prose-era`, and `manifest-era` tags.)
**Rejected:** (a) script-as-canonical configuration — the engine executing
author code to learn the contract dissolves decidability, determinism, and
the offline gate in one move; (b) a committed manifest as the gate's corpus —
face↔file integrity is unverifiable without re-emitting, so the honest
integrity check is CI re-emitting `--frozen` and byte-comparing, and the
file adds only a third review surface nobody reads; (c) a maintained
hand-TOML authoring surface beside the SDK — a permanent second surface
(docs, keystroke channel, format-preserving patching, an adoption ladder to
dignify it) serving no one; what stays banned is Node at *check* time.

## Decision: genre-typed prose — the model swallows the document

**Chosen:** a kind may declare a **genre vocabulary**: typed shapes for the
recurring forms of its members' prose — a decision with its rejected
alternatives, a law with its bounds, an honest bound with its unlock —
authored as values whose meaning-carrying words are **prose leaves**:
authored strings law 5 protects one by one. Where a corpus adopts genres
fully, its documents become pure projection — the src→dist move of the
Decision above, applied one level deeper (a bound-prose spec document is
still source and rendered thing at once; the last mirror-era holdout).
Genres are **kind/package data, never engine** (law 2): a project whose
prose argues differently declares its own genres with the same machinery
(`15-kinds.md`). Adoption is opt-in per block, forever — plain prose is a
fully legal member of every genre-bearing kind, and **no check may quantify
over genre completeness** (law 8's opt-in bound, one level down). The
payoff belongs to the primary author (Positioning: the agent):
**proprioception** — leaves are addressable, so `impact` reaches inside
arguments, a leaf's declared neighborhood is assemblable context, and an
edit declares what it is. Prose edits become graph events
(`30-landscapes.md`, `20-surface.md`). (Ratified 2026-07-03 by delegated
ceremony; the pre-state is the `bound-prose-era` tag.) **Rejected:** (a)
hardcoded genre checks — a built-in ontology of argument is the tool's
taste, in the compiler where it is hardest to see; (b) typing rationale
itself — "why" is undecidable, the prose remainder never hits zero (the
honest bound), and genre types structure the *anatomy* of an argument,
never its content; (c) genre-completeness or -density demands — the mining
swamp rebuilt from the declaration side, inadmissible by definition.
