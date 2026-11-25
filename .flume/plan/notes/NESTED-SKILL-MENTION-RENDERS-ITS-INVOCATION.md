# Nested skill mention renders its invocation

`mentionOf` now displays `/apps/web:deploy` for `skill:apps:web:deploy`
(address unchanged). The rule is keyed on kind facts, not the skill name:
an `at` locus with a `**/` glob, a directory unit shape, and a user-invoked
registration channel. A member with no `:` in its name keeps its bare name.

Observed asymmetry worth a plan/spec look: a root skill still displays bare
`deploy` (no leading `/`), while a nested one displays `/apps/web:deploy`
with the slash the spec's bullet spells. A nested command
(`command:frontend:component`) still displays `frontend:component`, not
`/frontend:component`. If the display rule is meant to be "the invocation
spelling" for every user-invoked member, both of those are follow-ups; the
entry named only the nested skill, so this tick did not widen it.
