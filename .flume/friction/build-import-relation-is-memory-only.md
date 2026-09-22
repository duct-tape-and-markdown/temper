## Symptom

The entry's acceptance oracle names "a `CLAUDE.md` → rule → `CLAUDE.md` `@import`
ring fires one `graph.acyclic` error". It does not, and cannot: `memory` is the
only built-in kind composing the `at-import` directive primitive
(`src/builtin_kind.rs:322`); `rule` composes `Field`/`LineCount`/`Headings`/
`Sections`/`Placement` and no `Directives`. A `@`-line in a rule body is
extracted by nobody, so the rule contributes no arc and the chain never closes.
Verified against the built binary: the three-member chain exits 0; the
equivalent `memory` ↔ `memory` ring (`CLAUDE.md` ↔ `docs/CLAUDE.md`) exits 1
with exactly one `graph.acyclic` error.

Nor can an adopter reach for it: a `Directives` primitive is not expressible in
a lock `KindFactRow` at all (no `directives` spelling anywhere in `src/drift.rs`),
so the import relation is memory-only, full stop — for built-ins and custom
kinds alike.

The entry landed on the relation the spec names; the oracle's middle hop is the
part that was never reachable. Shipped against the reachable half.

## Cost this tick

~10 minutes: reading the extraction primitives to work out why the ruled oracle
would not fire, then re-deriving the honest two-member case. No revert.

## Suggested fix

A question for the corpus, not a code change here: is `@path` import a `memory`
capability or a markdown-body capability? Claude Code documents recursive
imports for memory files (code.claude.com/docs/en/memory, retrieved 2026-09-22)
and is silent on rules/skills — so either the relation is correctly memory-only
and future acceptance oracles should say so, or the `Directives` primitive owes
a lock spelling so an adopter can declare it on a custom kind. Route to
`.flume/inbox.md` if the second.
