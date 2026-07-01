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
   and idempotent: the same surface emits the same landscape, every time. Import
   is a one-time *parse into the surface language* — a migration, free to
   normalize framing, never to alter content; after it the surface is the single
   authored home. Every member carries `source_path` + `import_hash` — the drift
   anchor. (`20-surface.md`.)

6. **Fearless harness refactoring.** Because conformance is checkable, the author
   can reshape, dedupe, and compose the harness and re-verify — the original
   "work with the config surface, reshape and organize" goal. Empty without the
   drift/projection engine, so that engine is core, not optional.

7. **Compose everything; gate the decidable.** The write surface is total: the
   author composes the whole harness at `temper`'s altitude — structure *and*
   prose — and `temper` **projects** it into the project. The gate is narrow: it
   blocks only on the decidable tier and on whether each `verified_by` is wired.
   `temper` never synthesizes prose and never adjudicates it — prose is the human
   **behavioral contract** (tier 3, below), enforced by its wired verifier, never
   by the tool's taste. Write-total, check-bounded: the editor writes all your
   code; the type checker only checks its types. (`20-surface.md`.)

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

The **product** is a **declarative configuration model for the agent harness**: the
author composes the whole harness at one altitude — the **assembly** (`temper.toml`)
binding **packages** to **kinds** over the authored **members** in `.temper/` — and
`temper` projects it into the project. The **typed gate** is the *differentiating
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
at `temper`'s altitude across every landscape, project into the project, gate the
result (`20-surface.md`, `40-composition.md`). **Rejected:** the downstream-linter
framing — author `.claude/` by hand elsewhere, `temper` only reads and grades it.
The linter framing makes `temper` optional and external to the work and strands
law 6 (you cannot fearlessly refactor a surface you do not author); the
composition framing makes `temper` the place the harness *lives*. The checker is
one face of that surface, never the whole tool.
