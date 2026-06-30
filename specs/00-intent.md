# Intent — the north star

`author` is a **type system for agentic harnesses.** It treats the harness — the
Claude Code (and, in time, any agent's) customization layer: skills, commands,
agents, hooks, MCP/LSP servers, `CLAUDE.md` rules, plugin & marketplace
manifests, settings — as a typed codebase you compile, not a pile of loose files.

This file is the why and the cross-cutting law. Everything under `specs/` is a
contract spec governed by it; orientation files (`90-spec-system.md`) are not.

## The thesis: the Rust of agentic harnesses

Rust relocates failure from runtime (the 2am production incident) to author-time
(the red squiggle), and earns it by checking your code against the **types you
declared** — not against some built-in notion of "good code." `author` does the
same for harnesses: it moves harness failure — a skill that under-triggers, a
rule that silently doesn't load, a hook command that doesn't resolve, drift you
never noticed — from *silent agent misbehavior at runtime* to a *finding at
author-time*, and it earns soundness the same way Rust does: **by checking the
harness against a contract the author declares, never against the tool's taste.**

The agentic 2am page: the agent quietly did the wrong thing because the harness
was malformed, and you learned it from the output, not a squiggle. `author`
front-loads that to author-time.

## The law

These bind every part of the tool and every change to it.

1. **Gate, don't lint.** `author` is a gate you pass, not a linter you maybe run.
   The end state: *you do not start an agent on a harness that fails its
   contract* — enforced by a `SessionStart` gate (the harness checking itself
   before the agent loads). "Complete" means *fills its declared contract*.

2. **Determinism comes from the author's declaration, not the tool's opinion.**
   `author` ships no built-in judgment about what a good skill/rule/harness is.
   It gives the author a way to **declare a contract** and checks conformance.
   The author writes the types; `author` is the type checker. Built-in "best
   practices" exist only as *contract templates* — data, adopted by choice,
   overridable — never as hardcoded checks. (`10-contracts.md`.)

3. **Decidable clauses only — the immune system.** A check enters `author` *iff*
   it is expressible as a decidable contract clause over the fixed primitive
   algebra. No fuzzy heuristic, no "just this once" escape hatch, no arbitrary
   code in a contract. A property that cannot be reduced to a decidable predicate
   is **behavior**, and behavior is delegated (`verified_by`), never guessed. The
   moment `author` guesses, it produces false positives, and a gate that cries
   wolf gets disabled. This rule is what keeps the project out of the heuristic
   swamp it was started to escape.

4. **`author` validates structure; it never adjudicates intent.** It checks that
   the harness fills the contract the human declared. It never decides the
   harness is "missing" something the human didn't ask for — that is the human's
   to declare. Surface gaps; do not fill them. (Mirrors `collaboration` rule.)

5. **Round-trip is byte-faithful; provenance is load-bearing.** Prose bodies are
   copied, never re-rendered; only structured headers are rewritten (format-
   preserving). Every imported artifact carries `source_path` + `import_hash`.
   This is what makes drift detection and write-back possible. (`20-surface.md`.)

6. **Fearless harness refactoring.** Because conformance is checkable, the author
   can reshape, dedupe, and compose the harness and re-verify — the original
   "work with the config surface, reshape and organize" goal. Empty without the
   drift/round-trip engine, so that engine is core, not optional.

## One engine, every layer an instance

There is one **contract engine** built from a fixed algebra of decidable
primitives (`10-contracts.md`). It knows nothing domain-specific. The harness is
not a special case — it is the *first instance* expressed in those primitives.
The spec corpus is *another instance*. Code (its types checked by its compiler)
is a third. `author` governs a **landscape** — any corpus of authored artifacts —
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
Its standing payoff needs no LLM and is the near build: remove a load-bearing
entity and the graph lights up every spec, binding, and code symbol that depended
on it — the blast radius, deterministically. This is law 6 (fearless refactoring)
made literal, and it is tier-1 sound. (The judged tier rents space *on* this
graph later; build the graph first.)

## Positioning

`rulesync` makes a harness *portable* across assistants. `skills`/marketplaces
*distribute* artifacts. `author` makes a harness *correct* — a Claude-Code-native
(then agent-agnostic) contract system on the maintenance/quality axis. Different
layer; `author` can sit downstream of the others, checking what you installed.

## Honest bound on the analogy

Rust's guarantees are *sound* because conformance-to-declared-types is provable.
"Good harness" is **not** provable, and `author` must never pretend it is. What
*is* provable is conformance to a declared contract — so that is all `author`
asserts. The undecidable remainder (does this skill trigger well? does this tool
work?) is delegated to verifiers the contract requires to be *wired*, and whose
*passing* is checked by execution (tests/CI/eval), never by `author`.

## The proof: self-hosting

`author` is built — right now — by an agentic harness (flume) reading a harness
we are still correcting, because `author` doesn't yet exist to check it. The
classic compiler bootstrap. The finish line: `author check` runs green on its
own `.claude/`, then the next flume loop refuses to run until it does. When
`author` governs its own builder, the thesis stops being a slogan.

## Decision: evergreen spec, not release lines

**Chosen:** one living `specs/` corpus, continuously reconciled against code
(`90-spec-system.md`). **Rejected:** flume-style `RELEASE-vN.md` ship lines —
they frame the work as a sequence of finished targets, but `author` is a design
in motion whose contract model is still deepening; a frozen release target would
lie about that. Plan reconciles code↔specs every tick; there is no "done with a
release," only "code conforms to current intent, or it doesn't."
