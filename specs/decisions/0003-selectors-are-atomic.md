# 0003 — selectors are atomic; narrowing is a clause

- **Date:** 2026-07-07 · **Status:** accepted

## Context

A requirement's satisfier set was computed two ways: roster narrowed it by
the requirement's declared kind, coverage computed it kind-blind — the same
member counted by one checker and silently excluded by the other (0001,
consequences). Fork: does a requirement narrow its opt-in selection by
kind, or is opt-in strictly kind-blind?

## Decision

Opt-in selection is kind-blind, and selectors stay atomic — they do not
compose. Narrowing is an ordinary each-grain clause over the selection
("every satisfier is of kind K"), shipped with the requirement kind's
default contract. A satisfier outside the narrowing is a finding, never a
silent exclusion.

## Rejected

Selection composition (by-opt-in ∩ by-kind) — a second algebra where a
clause already expresses the demand, and a wrong-kind satisfier stays
silently uncounted instead of diagnosed. Dropping the capability — the
narrowing is real; it moves, it does not die.

## Consequences

Coverage, roster, and graph unify on one kind-blind satisfier set;
`requirement.kind` recuts from a selection input into a shipped clause;
wrong-kind satisfiers surface as findings.
