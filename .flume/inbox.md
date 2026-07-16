<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->
- Clause candidate from the centercode packs-are-skills re-cut: the **dark
  mention**. A skill's `paths` removes it from every invocation channel until
  a matching file is read (cited at `sdk/src/builtins.ts` `Skill.paths`,
  verified 2.1.210), so a rule→skill mention is actionable only if the
  rule's own `paths` fall inside the target skill's gate at fire time. That
  containment — `rule.paths ⊆ skill.paths` for every mention edge whose
  source and target both declare paths — is held by nothing today, and the
  failure is perfectly silent: probe on 2.1.210 (headless testbed session,
  2026-07-16) shows gate-opening emits a `skill_listing` delta only for the
  *target's own* paths match, so a mention arriving through a rule outside
  the gate points at an inventory absence with no error anywhere — it reads
  as "the model ignored the rule." The consumer currently hand-derives each
  dev pack-skill's `paths` as the union of its mentioning rules' paths, which
  drifts the first time a mention is added without widening the gate. The
  gate already resolves the mention graph and both `paths` lists; 0022
  admitted glob validity to the predicate vocabulary, and a literal
  glob-superset check over mention edges is the same species. observed at
  f08ffca
- Empirical fact for the market-formats deep-audit digest, next to the
  `paths`-gate cite: gate-opening is **loud**. On 2.1.210 (headless probe in
  the centercode testbed, 2026-07-16, transcript-verified), reading a
  gate-matching file injects two attachments in the same turn: the
  path-scoped rule as `nested_memory`, and a `skill_listing` attachment with
  `isInitial: false` carrying *only* the newly ungated skills, name plus
  full description — an in-narrative delta announcement, not a silent
  listing mutation. Decision-relevant for gated-vs-ungated authoring
  guidance: an ungated skill's description is day-one scenery; a gated
  skill's description arrives as an event at the moment of relevance, so the
  remaining costs of gating reduce to the cold-question hole and the
  containment invariant above. Also observed: the rule's rendered content
  strips the managed-by frontmatter note (`contentDiffersFromDisk: true`) —
  the provenance marker costs the agent zero tokens at fire time. observed
  at f08ffca
