# 0045 — guidance decouples from the clause

- **Date:** 2026-07-20 · **Status:** accepted

## Context

The extent ruling (0035 amendment) and invariant 8 established the shape:
temper gates the decidable floor and delivers a kind's intent to the author
as guidance, because in a primarily AI-authored harness the author is the
verifier for what no predicate can decide. But guidance today is
**clause-bound** — `contract.md` defines it as "the atom of every authored
check," delivered only where a predicate fires. So a kind's authoring intent
that is *not* a check — "leave `version` unset while iterating," "the
`description` is trigger language, keep it terse" — has nowhere to live but a
doc comment: `sdk/src/builtins.ts` literally carries a block headed
"Authoring notes the clauses cannot carry," invisible to the authoring agent.
Invariant 8 names the channel; nothing carries it. That is the gap.

## Decision

**Guidance decouples from the clause.** The one guidance concept — advisory
intent, optionally cited — attaches to a **clause** (as today: it teaches at
the moment of failure, and rides as `schema` hover at authoring), or directly
to a **kind** or a **field**, with no predicate: pure authoring counsel,
delivered at authoring time (`schema` hover, `explain`), teaching before there
is anything to fail. It never gates — a field's guidance is not a length
check, and a kind's brevity intent carried as guidance is the author's
counsel, not the gate's (invariant 5). The built-in kinds' stranded authoring
notes promote into it.

One channel, two attachment points, one delivery: on a clause guidance
teaches at the moment of failure; on a kind or field it counsels at the moment
of authoring.

## Rejected

- **A guidance-only clause** (a predicate-less clause that carries only
  guidance): overloads the clause — the atom of a *check* — with a non-check,
  and forces every reader to ask "does this one gate?" Guidance that never
  gates is not a clause.
- **Leaving the notes in doc comments** (the status quo): invisible to the
  authoring agent — the remote-narration-rots failure the harness itself
  gates. Invariant 8 requires delivery, not a comment.
- **Gating the notes as `warn`-severity clauses**: hardens advice into the
  linter temper sits downstream of (invariant 5). A 201-character description
  is not a violation; making it one is the devolution the positioning forbids.

## Consequences

`contract.md` generalizes guidance from clause-atom to a channel attachable to
clause, kind, or field. The SDK gains an advisory `guidance` on `kind()` and
on field declarations (no severity, no predicate); the engine delivers it
through `schema` (hover) and `explain`. The built-in kinds' JSDoc authoring
notes move into the channel — the `skill` description note, the
`plugin-manifest` `version` note, and their kin. Plan derives the entries.
