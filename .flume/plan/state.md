# Plan state

- Spec derived through: 832f015
- Audited through: b85df4a
- Residue swept through: b85df4a
- This tick: DRAIN INBOX — routed centercode's four field reports (all
  `observed at d359782`; window d359782..26c0528 is .flume-only, so every
  cited address carries). Re-verified each on disk before scoping, not from
  the log:
  **Three defects → pickable entries (gate:open), disjoint files.**
  - MENTION-REACHABLE-EDGE-UNIFY (contract.md, "edge"): `mention_reachable`
    (`main.rs`:1114) gets `&mention_edges` alone while `degree` (:1108) gets
    both edge sets — a mention on a field edge missing from that partial set
    gets zero coverage yet the clause runs green. Confirmed on disk; the
    invariant-6 fail-open shape, fixed to the READ-EDGE-UNIFY bar (6e80405).
  - ADMISSION-JOINS-FILE-TEMPLATE (representation.md, "nesting"):
    `templatesFor` (`declarations.ts`:193) returns admit rows *instead of*
    `facts.templates` when any kind is admitted — the doc comment itself
    calls the override "layer-blind". Confirmed; fix joins path-carrying
    templates, no new intent.
  - EMBEDDED-LEAF-BARE-ADDRESS (contract.md, "edge"): `edgeTargetFacts`
    (`emit.ts`:253) does a flat `members.get(address)` over a `kind:name`
    table (`memberTable`:626), so a bare address on a one-element `to` set
    dies wrongly. Confirmed against `EdgeField.to`'s doc (`kind.ts`:55,
    decision 0029); SDK-only qualify-before-get.
  **One demand → fork, not entry.** `(value-extent-predicate)`: a
  render-extent predicate for posture budgets. A vocabulary addition is a
  deliberate language change (contract.md, "clause") — never derived —
  registered as an open fork; resolution returns through the inbox.
  Inbox drained to empty. The two existing parks copied forward verbatim,
  untested this tick (audit is not this tick's job; both held at b85df4a
  under d359782 and the window since is .flume-only).
- Queue: 5 entries, **3 pickable** (the three field defects, gate:open,
  disjoint across `src/main.rs`+`src/graph.rs` / `sdk/src/declarations.ts` /
  `sdk/src/emit.ts`) + 2 parked (hop-semantics probe; Apple notarizing + v0.1
  tag). No entry rests on a fork.

Plan continues: no — inbox drained, spec delta empty (cursor 832f015, no
`specs/` commit past it), and b85df4a..26c0528 is .flume-only (nothing to
reconcile). Three pickable defects now hand off to build.
