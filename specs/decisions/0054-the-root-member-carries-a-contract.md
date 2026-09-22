# 0054 — the root member carries a contract

- **Date:** 2026-09-22 · **Status:** accepted

## Context

`contract.md` puts reachability in the root member's default contract, on
by default, and `pipeline.md` makes drift loudness the author's declared
severity. Neither holds in code. 207e701d (07-06) retired reachability's
assembly switch "into the clause algebra", but no `reachable` predicate
entered the vocabulary, so `graph::reachable` has no caller outside tests:
`check` has not reported a dead registration since. The five disk-vs-lock
findings (`config.stale`, `prose.include-stale`, `layout.import-stale`,
`locus.undeclared-member`, `layout.undeclared-member`) are fixed `warn`
pushes: the dial reaches clause labels only, and enforcement mode acts at
`guard`, never on `check`'s exit, so no placement can make a drifted
content pin fail CI (observed on 0.0.18, a consumer's field report). One
gap underlies both: the SDK gives the root member no clause surface.
`harness()` binds only by kind and by opt-in, with no root kind to bind.
Ruled by John 09-22 ("make drift findings dialable"; mechanism the session's).

## Decision

**The root member carries a contract**, composed like any: `harness()`
takes one, and a shipped root default contract applies when the author
declares none. Adoption is the import, override is composing the array.
Its selection is the whole governed forest. Three predicates join the
vocabulary, each each-grain over that forest: **`reachable`** (a member's
world edge is live, or a live importer reaches it, `graph::reachable`'s
existing rule), **`fresh`** (every lock row a member owns matches disk:
projection fingerprint and each fingerprinted source dependency), and
**`locus-declared`** (every document at a governed locus has a row). The
root default binds all three at `advisory`: today's drift posture, and no
adopter turns red on the upgrade. Any author hardens by composition, and
the dial reaches each by label.

## Rejected

- **One predicate for the whole disk-vs-lock comparison**: it bundles the
  undeclared-document notes a read-only ground kind draws with the stale
  pin the author wants blocking, so hardening one hardens noise.
- **Binding `fresh` per kind by default**: user-declared kinds carry no
  default contract, so their drift would go silent; and a per-kind
  binding beside the root's double-reports one row.
- **Shipping `reachable` at `required`**: its dead criteria are exact,
  but turning adopters red on a check dark for eleven weeks is the
  tool's escalation, not the corpus's (invariant 5).

## Consequences

`contract.md` "clause" and `pipeline.md` "Drift" name the declared
severity. Plan derives: the root contract field and default in the SDK;
the three predicates and their lowering; the five fixed pushes becoming
clause judgments under their labels, a changelog note for the rule ids;
`reachable` wired again beside `degree`.
