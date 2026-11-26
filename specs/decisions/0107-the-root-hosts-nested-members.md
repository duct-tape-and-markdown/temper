# 0107 — the root hosts nested members

- **Date:** 2026-11-26 · **Status:** accepted

## Context

The `root-hosted-member-identity` question. `representation.md` makes
the harness itself a member, the root of the forest, but the nested
address grammar admits no root host. The SDK spells a tool
`root/tool/<name>` in one place and `tool:<name>` in another. One engine
reader of the grammar admits `root/<kind>/<key>` and another refuses it.
A requirement reaches the lock only as a flat row, so a requirement
subkind's fields never reach `check` or `explain`. That makes
`expect: [{ kind: requirement, clauses }]` a vacuous verdict nobody is
told of. Ruled by the session from the model's own root-member claim,
under John's standing direction.

## Decision

**The root's address is `root`, and a member it hosts is a nested member**
(`representation.md`, "The root member"): `root/<kind>/<key>`. Its leaves
reach the lock and the read verbs like any nested member's, and its
containment is an edge from the root. One grammar reads it everywhere.

## Rejected

- **Flat `<kind>:<name>` identity for root-hosted members.** It
  contradicts the root being a member, and it keeps a subkind's fields
  out of the read verbs.
- **Refusing `expect` over `requirement` until a fix lands.** That is a
  stopgap, not a model.

## Consequences

Containment from the root is visible to `check`, so the root's out-degree
becomes judgeable. The SDK's two tool addresses fold into one. A
requirement subkind's leaves reach `explain`.
