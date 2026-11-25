# Guidance rules now carry contracts and edge intent

Shipped. `guidanceBody` (sdk/src/declarations.ts) adds `## Edges` (outbound edge-field
intent on the declaring kind, inbound `kind`.`field` intent on each target kind) and
`## Requirements` (every requirement a kind's members satisfy or that declares the
kind: identity, declared kind, prose verbatim, satisfier addresses or "No member
satisfies it yet."). An edge field's counsel moved out of `## Fields` into `## Edges`
so it is stated once per rule. An imported built-in now takes a rule when the program
contracts over it, carrying only the program's contracts and inbound intent from
program kinds; shipped field guidance on a shipped source kind is never delivered.

Ripple: examples/base-harness re-emitted (five new temper-guidance-* rules, lock rows).
The repo's own dogfood (`.temper/` -> `.claude/rules/`) will now emit new guidance
rules on its next `temper emit` — human territory, not re-emitted here; expect drift
until a `chore(harness):` commit re-runs it.

Observation: requirement prose is delivered verbatim and the example's prose runs long
(one-line sentences of ~150 chars); no length gate, per 0092.
