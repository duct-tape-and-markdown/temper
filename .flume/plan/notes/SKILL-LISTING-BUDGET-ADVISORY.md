# Skill listing budget advisory: what it does not count

Shipped a whole-grain `listing` predicate (src/contract.rs, judged by
`engine::over_listing`) and bound it in `skillDefaultContract` at 8,000
characters per listing, 1,536 per entry, withheld by
`disable-model-invocation`. Docs re-verified 2026-11-26: skills
#skill-descriptions-are-cut-short gives no number for "1% of the window".
The env-vars row for SLASH_COMMAND_TOOL_CHAR_BUDGET gives "a fallback of
8,000 characters". That fallback is the bound used.

Gaps a later plan tick should weigh:

- **command is not judged.** The entry said "skill (and command)". A kind's
  whole-grain clause judges only its own kind's selection, so a clause on
  `commandDefaultContract` would budget commands apart from skills. The
  listing is one budget across both kinds, so that clause would miss the
  joint overrun. Closing this needs a selection spanning both kinds, or a
  root-bound form of the clause. Neither shape is in the spec today.
  `commandDefaultContract` stays empty.
- **Only this harness's skills count.** Plugin, bundled and personal
  skills share the runtime listing. The guidance says the real headroom is
  smaller than the advisory shows.
- **`paths`-gated skills are counted.** Their listing entry is withheld
  until the gate opens, so the clause counts them conservatively, as
  though every gate were open.
- **The undeclared-description fallback is not counted.** The runtime
  lists a skill's first body line when it has no `description`. The
  required `description` clause already refuses such a skill.
- **`explain` does not yet report per-member cost.** The spec's Default
  contracts section asks for it. This entry did not cover it.

Row encoding: `fields` carries the listed fields, `gate` the withholding
field, and `bound = { max, entry }`. `BoundRow` gained `entry` and its
binding was re-blessed. The `field` column is the synthesized label segment
(`description+when_to_use`).
