# 0029 — an edge declares its target set

- **Date:** 2026-07-16 · **Status:** accepted

## Context

The `(edge-field-target-openness)` fork: a field edge's `to` names exactly
one target kind, but centercode's citation posture ranges over
heterogeneous targets — its `reference` points at a rule and at
supporting-doc kinds, so no single `to` is true. The fork's seam half —
three surfaces disagreeing on whether `to` is read at all — closed when
EMBEDDED-EDGE-DEGREE-SEAM (c607fad) routed both engine compare paths
through one normalizer resolving by identity within `to`; what survives is
the arity: one declared kind cannot type a heterogeneous reference, and
the consumer's only outs are a duplicate field per target kind or an
untyped address. Session-argued, human-ruled 2026-07-16.

## Decision

A field edge's `to` widens to a **declared, non-empty set of target
kinds**. Resolution stays "by identity within the target kind"; the
authored address names which member of the declared set. Addressing:

- a **one-element set** is today's behavior exactly — a bare name resolves
  within the single kind, so the migration is mechanical and the engine's
  bare-identity path survives as the degenerate case;
- a **multi-element set** requires the kind-qualified address
  (`kind:name`), always — even when a bare name would happen to be unique
  across the set. Resolution depends on the written text, never on the
  global member population, so a verdict can never flip on an unrelated
  member's addition.

The edge's type stays declared at the kind — the member never decides its
own type — and a corpus wanting narrower keeps its each-grain `kind`
clause over the selection.

## Rejected

- **Optional `to`**: makes an edge's target kind a per-instance property of
  the address rather than a declared contract; with no stated expectation,
  the gate has nothing to hold (the same disease as edges mined from
  prose).
- **One posture per target kind**: centercode would author two same-shaped
  fields to dodge the model's arity gap — a duplicate surface exported to
  the consumer.
- **Bare-name uniqueness inference in multi-sets**: friendlier to type,
  but check verdicts would flip on unrelated edits.
- **Defer for a second consumer**: n=1 is thin for a language change, but
  the set is the conservative generalization — it strictly contains
  today's behavior and forecloses nothing — and a real consumer is blocked
  today. Open citation ("cite anything addressable") stays unserved on
  purpose: a typed reference enumerates its targets, and openness must
  arrive as its own deliberate ruling, never as a side effect.

## Consequences

`model/contract.md`'s "edge" field bullet carries the set and the
addressing rule — same commit, this record. The SDK's `EdgeField.to`
widens to the set type; the engine's shipped normalizer
(`src/graph.rs` `target_identity`) extends over the set — the address's
kind component must land in the declared set, the singleton case behaving
exactly as EMBEDDED-EDGE-DEGREE-SEAM built it, so the migration cost is
the one already-measured home. The fork record deletes with this record's
commit; plan derives the entries.
