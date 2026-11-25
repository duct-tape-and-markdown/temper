# 0093 — the harness emits its own reference

- **Date:** 2026-11-25 · **Status:** accepted

## Context

Every team building a harness with AI keeps a "how our setup works and
why" document, and it rots. Intent does not survive a conversation. On
2026-11-25 a session nearly designed a second owner for a domain temper
already fills: the bundled `temper` operate skill, a hand-kept
`src/bundle.rs` string only plugin users receive. 0091 and 0092 deliver
one kind's counsel and contracts on touch. Nothing gives the whole
picture to an agent about to change the harness. John, 2026-11-25: the
skill's contents "is a representation of the harness, emitted with it".

## Decision

**The operate skill is an SDK value that a harness emits as its own
member** (`distribution.md`, "What ships", 3):

- its body is the product's mechanics plus a representation of the
  harness, rendered from the declarations at emit: requirements and their
  prose, their owners, each edge field's intent, each kind's counsel;
- its description names the contracted domains, so every session starts
  knowing which are taken;
- it renders only what was declared, and invents nothing (invariant 1).

The skill is a projection. It is committed, read in the diff that changes
a contract, readable without temper, and held true by drift. `bundle`
embeds the same value. The `bundle.rs` literal retires.

## Rejected

- **Assembling the map at invocation** (`` !`temper explain` `` in the
  body). It is never reviewed and needs the binary, a permission and a
  working directory. A failed command aborts the invocation.
- **Keeping the fixed text.** It is a second spelling of the surface, and
  it already rotted.
- **The map in path rules alone.** Those carry one kind's slice on touch,
  and restructuring needs the whole map.

## Consequences

The shape is progressive disclosure, which is the router Cascade's
`standard()` arrived at independently. `SKILL.md` stays as small as
possible: the method, and one index line per kind and per domain that
leads with the moment (`read kind-domain.md when you add or change a
domain`). Each topic file sits flat beside it as a supporting doc.
`kind-<k>.md` carries the kind's counsel, locus, fields, edges in and
out with their intent, clauses and domains. `domain-<r>.md` carries the
requirement's prose, its verifier and its owners. No file lists members,
so a topic grows with declarations, never with documents. Only committed
declarations render; dials, `local` members and joined locks never do.
Any contract change re-renders it. The skill can declare `satisfies`,
and so list itself as its domain's owner.
