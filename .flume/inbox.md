<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->
- Authoring-model finding from the centercode consumer corpus, human-ruled
  07-16 in the field ("we don't need to explicitly manage the inline
  reference; encode the intent of the prose in a manner temper can track" +
  "prose should be concise and meaningful — file-referencing is a path to
  prose explosion"). Two connected asks. **(a) A reference is member intent,
  not a character position**: today the positional mention is the only edge
  reachable on built-in kinds, so every rule and skill body in the corpus is
  an inline template literal solely to thread interpolations, and the
  consumer hand-rolls a `cite()` helper (address spelling + display) to do
  it. The wanted noun is a declared reference — a framework member key
  beside `satisfies`, or edge fields on the built-ins (the `EdgeField` fact
  already exists; base-harness `implemented-by` proves it) — with mention
  respelled as *a reference that additionally wants a rendered position*.
  Display is never authored: it derives from the target's kind, and that
  derivation is functional, not cosmetic — a skill renders as its invocable
  name, a supporting doc as its readable projection path, because the
  rendering is the agent's retrieval instruction. **(b) The body's grain for
  machinery surfaces is the typed block, not the file**: the consumer's
  hand scaffold for a skill body is a short list of role-typed blocks
  (orientation / intent / reference), each a sentence or three, attributes
  attached to the block that speaks them (a `cite` on the reference block).
  That is posture-3 `blocks()`/layout as the *default* skill/rule body —
  expressible today only via custom embedded kinds plus a hand-rolled
  `span()` (which even base-harness hand-rolls). A `file()` blob stays the
  exception for genuine long-form (supporting docs — the pull tier's
  stock), because unstructured file prose is unbounded and
  contract-invisible, where typed blocks give the gate per-block roles and
  budgets. Consumer evidence for both: every hand-rolled helper in the
  centercode program (`cite.ts`, inline-template rule bodies) exists to
  bridge exactly this gap. observed at f08ffca
- Demand-arrived evidence for native skill-package modeling: the 07-15
  directory-sliced-governance ruling deferred a nesting kind "until demand
  arrives." The centercode corpus now runs a hand-rolled
  `supportingDocs(skillName)` factory minting one nested-root kind per
  skill directory (`.claude/skills/<name>/docs`, glob `*.md`,
  registration-free) — two live kinds (`harness-meta-doc`, `platform-doc`),
  each doc reached by a required incoming-degree clause and a typed mention
  from its skill body, with the previously unmanaged
  `platform/reference.md` brought under management by it. Every consumer
  harness with a skill that carries depth re-derives this factory; the
  vendor pattern it models (a skill packages its reference docs, loaded on
  demand) is documented at code.claude.com/docs/en/skills "Add supporting
  files" (retrieved 2026-07-16). observed at f08ffca
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
