# Prototype: the centercode harness under posture recursion

This directory is the companion to `../posture-recursion.md`: the full
centercode corpus (1 memory, 11 rules, 13 skills, 2 supporting docs)
recomposed under the settled design. It is a reference artifact, not a
build target: the TypeScript is aspirational and assumes the three engine
gaps the proposal names (consumer-declared content over built-in kinds,
the host-agnostic embeddable locus, citation render templates). Spellings
are illustrative; the shape is the proposal.

What to read for what:

| File | Shows |
|---|---|
| `kinds.ts` | The house posture vocabulary as configuration, including two citation postures with different render templates |
| `memory.ts` | The ambient tier as typed blocks |
| `rules.ts` | All eleven rules; `## Invariants` conventions become countable `directive` values |
| `standards.ts` | The `file()` long-form exception, unchanged on purpose |
| `skills.ts` | Procedures as steps; the intake skill from the proposal's example |
| `harness.ts` | Admission (`content:` per built-in kind), budgets per posture, the contract |
| `projections/` | Two rendered samples; every citation line is derived from an edge |

Layout note: the live program keeps one module per member; the prototype
groups by tier for reviewability.

Things nobody authors here, by design: display text (render templates
derive it from target facts), member-level reference lists (derived from
block edges), and any inventory or wiring table (derived from the member
list).

Two clause spellings in `harness.ts` are as aspirational as the locus:
the `content:` admission binding on a built-in kind (the Element B gap),
and `count({ of: directive, max: 5 })` ranging over a posture type
inside a body, which is the "prose explosion dies by clause" payoff and
only becomes expressible once bodies are typed.
