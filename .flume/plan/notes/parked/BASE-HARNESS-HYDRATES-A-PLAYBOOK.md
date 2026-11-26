# BASE-HARNESS-HYDRATES-A-PLAYBOOK parked: no playbook shape to hydrate

The entry's gate, `blockedBy: STARTER-SHIPS-THE-PLAYBOOK`, reads as clear
only because that entry left the queue by retirement, not by shipping:
9194fcde deleted `.flume/plan/pending/STARTER-SHIPS-THE-PLAYBOOK.json` "for
re-derivation from the narrowed text", and no successor entry exists in
pending.

The tree confirms nothing shipped: `sdk/src/starter.ts` exports
`reference`, `gateHooks`, `typecheck` and `starter()` only; no `playbook`
constructor exists anywhere under `sdk/src` (rg "playbook" hits only a
requirement name in sdk/test/refusals.test.ts). The acceptance
("examples/base-harness composes a playbook") therefore cannot turn green
without building the SDK shape itself, which is the retired entry's work
(trigger/invocation/topics axes, router renderer, refusals for execution
and preloading), not this one's. Building it here would invent the shape
under an example-only entry and pre-empt the re-derivation 9194fcde asked
for.

What plan should do: re-derive the playbook SDK entry from
specs/distribution.md "What ships — three channels" (as amended by
9194fcde and 0111), then re-gate this entry on the new tag. This entry's
own scope stays sound once that lands: replace the hand-wired
rule_docsDiscipline + skill_verifySummary pair in
examples/base-harness/.temper/harness.ts with one hydrated playbook,
retire their .temper/ sources and .claude/ projections, re-emit lock.toml.
Nothing in this tick was changed beyond this note.
