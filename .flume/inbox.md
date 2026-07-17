<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- Field defect (centercode, migrating onto the shipped 0025 surface;
  session-verified on disk): a corpus `admit` over a host **wipes the host
  kind's declared file-template layer** — `templatesFor`
  (`sdk/src/declarations.ts:194`) returns admit rows *instead of*
  `facts.templates` whenever any embedded kind is admitted, and admit rows
  carry no `path` column, so the file layer is unspellable in the
  replacement. Consequence: composing `blocks()` bodies over `skill` kills
  the shipped `supporting-doc` layer — the engine refuses via
  `nested_file_path` ("its host kind `skill` templates no file layer"),
  whose own words call the pattern the host's declared fact. The corpus
  never sanctions the wipe: builtins.md's admission sentence overrides a
  *layer's child kind*; COMPOSED-BODY-ADMISSION admits the embedded grain.
  Fix direction (derivable, no new intent): admission overrides the
  embedded grain only and **joins** path-carrying template entries;
  replacing a file layer's kind stays a separate, layer-addressed act no
  admission currently spells. Consumer reproduced end-to-end and reverted
  to per-skill `at`-locus doc kinds. observed at d359782

- Field defect (same migration; session-verified): `mention-reachable`
  ranges over **mention rows only** — `src/main.rs:1114` passes
  `&mention_edges` where `degree` (:1108) receives `&edges` *and*
  `&mention_edges` — so a corpus on 0025's respelled edge-field citations
  gets zero reachability coverage while its bound clause
  (`rule.mention-reachable.paths`) runs green: fail-open silence, the
  invariant-6 shape. Field-verified both halves at the consumer: three
  literally-uncovered consult edges, no findings; the same edges DO count
  for `degree`. Fix direction (derivable): one resolved edge enumeration
  under every graph judge — contract.md already defines a mention as a
  field edge carrying a rendering claim, and READ-EDGE-UNIFY's bar is the
  precedent. observed at d359782

- Field defect (session-verified): emit's embedded edge-leaf resolution
  refuses the bare address `EdgeField.to`'s doc promises — "a one-element
  set resolves a bare address within its one kind" (`sdk/src/kind.ts`,
  decision 0029) vs `edgeTargetFacts`'s flat `options.members.get(address)`
  (`sdk/src/emit.ts:253`) over a `kind:name`-keyed table: leaf
  `harness-meta` on `to: ["skill"]` dies "resolves to no composed member".
  0029 ratified bare-for-singleton, so the lookup fixes toward the doc:
  qualify a bare address within a one-kind set before the get. observed
  at d359782

- Field demand, not a defect (centercode; needs a human ruling — a
  vocabulary addition is a language change, never derived): no shipped
  predicate ranges over an embedded value's **rendered extent**, so
  advisory posture budgets are unsayable (orientation ~12 rendered lines,
  directive ~4, step ~3 — declared, then withdrawn). The want: a
  value-extent predicate (lines/characters of the resolved render),
  each-grain, decidable. First real-consumer vocabulary demand on record;
  parked for the design session, do not derive. observed at d359782