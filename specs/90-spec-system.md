# Spec system — how `author` is specified

Orientation file, not a contract spec. Adapted from cascade's spec system. The
`specs/` corpus is the evergreen source of truth for `author`'s intent and
contract; code is the truth below the line it draws.

This file is itself the prose ancestor of a **spec-landscape contract**
(`30-landscapes.md`): the conventions below are what a declared spec contract for
`author`'s own corpus would encode (placement, length, decisions-name-
alternatives, references resolve). The final dogfood is `author` checking
`specs/` against that contract — the tool eating the rules it was written under.
Until then these stay prose discipline.

## What a spec is

- The source of truth for **intent and contract**. The loop and humans re-read
  them every iteration. If spec and code disagree on intent, the spec wins — fix
  the code, or refine the spec if intent has shifted.
- **Prose.** Paragraphs, lists, tables, decisions. No frontmatter, no schema, no
  template to fill. (Note the irony and hold it: `author` validates *other*
  harnesses against declared contracts, but its own *design intent* is prose —
  because intent is the undecidable layer that contracts delegate, not encode.)
- **One topic per file**, filename is the topic handle. Target under ~150 lines.

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

## Naming consistency — the one hard rule

Name the same concept the same way in every file and in the code. One concept,
one name. `Contract`, `artifact contract`, `harness contract`, `role`,
`verified_by`, `decidable`, `surface`, `provenance`, `drift` are load-bearing
terms — search before coining a new one.

## Decisions

Every Decision records what was chosen, what was rejected, and why. A decision
without rejected alternatives is incomplete — future readers can't audit it.

## The corpus

- `00-intent.md` — north star: the thesis, the law, positioning, self-hosting.
- `10-contracts.md` — the two-layer contract model + the decidable algebra.
- `20-surface.md` — the config surface: import, IR, round-trip, drift, CLI.
- `90-spec-system.md` — this file.
