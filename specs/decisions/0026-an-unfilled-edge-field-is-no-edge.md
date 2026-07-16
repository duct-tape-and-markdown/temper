# 0026 — an unfilled edge field is no edge

- **Date:** 2026-07-16 · **Status:** accepted

## Context

EMBEDDED-FORMAT-TARGET-FACTS (74f4e62) shipped emit's derived-target-facts
mechanics with a refusal on any unfilled edge field of an embedded kind — a
declared edge field became mandatory at every instance, undialable. Plan's
reconciliation flagged the collision: invariant 1 bounds declaration
density, invariant 4 forbids the tool deciding something undeclared is
missing, and the spine rule sends every presence check to a clause. The
corpus's own example already needs the distinction: `superseded-decision`
requires its successor edge by the field's own type, while an ordinary
decision's same-shaped field is legitimately absent.

## Decision

Requiredness is the kind's own field schema — the SDK type parameter is its
spelling (`cite: Member` required, `cite?: Member` optional), already a
kind-declared fact (`representation.md`, "kind"). An unfilled optional edge
field is **no edge**: not incident to any clause, nothing for a format to
place, nothing to refuse. An unfilled required field fails in the author's
program at compose time — authoring failure, never a gate finding. Emit's
refusal retreats to what `pipeline.md` licenses: a reference **filled yet
unresolvable**. The two sibling refusals shipped beside it (target names no
composed member; target owns no projection) stand.

## Rejected

- **Ratify the floor** (every declared edge field mandatory): collides with
  invariant 1's density bound and makes the optional reference — an
  ordinary shape the example already carries — unspellable.
- **Clause reach over embedded leaves**: a real capability, wrong forcing —
  nothing here needs gate-time presence policy, and the machinery (a
  selection over an embedded kind's leaves) can arrive when a corpus wants
  that policy, not to fix a refusal.
- **A new optionality flag on `EdgeField`**: the schema already spells it;
  a second home for the same fact is the one-job-one-home violation.

## Consequences

`pipeline.md`'s "Refusing" bullet gains the boundary sentence — same
commit, this record. Code reconciliation routes through plan: the emit
throw retreats to dangling-only, and the schema's optionality reaches the
declared fact row so the engine sees what the author declared.
