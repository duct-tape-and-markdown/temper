# Posture recursion — typed postures, one edge noun, doctrine in the box

Status: design proposal from the consumer-lane field campaign (centercode,
07-14 → 07-16). The frame was human-ratified in the field 07-16 ("go") and
amended twice the same day by field rulings (postures are configuration,
not engine; a posture is a *type* extending member, host-agnostic). The
design lands only through review and the corpus. Supersedes the piecemeal
reading of the four 07-16 inbox notes — this is the one entry they were
fragments of.

## The claim

The harness files content by **posture**: what fires unprompted is a
directive, what is reached on demand is a reference, what executes is a
procedure, what orients is orientation. The field campaign found the same
algorithm one scale down: a member's body naturally grains into a short
list of **posture-typed blocks**, each a sentence or three, each carrying
the properties its role needs.

So the design is one move, not four features: **a single posture
discipline that recurses** — harness → member (posture picks the kind),
member → block (posture picks the block type) — with the vocabulary
living in configuration at both scales, and delivery semantics
(budgets on push, reachability on pull) bound by the contract wherever a
posture-typed value lives.

## Element A — edges are posture properties; mention is a positioned citation

A declared edge rides as a **typed property of the posture-typed member
that speaks it** — the `EdgeField` fact the model already carries. A
`reference` block's `cite` *is* the edge: declared intent, no character
position, gate-resolved, degree-able. There is no new member-level
`references` framework key (this proposal's second draft asked for one;
rejected below): a member's reference set is a **derived view** — the
union of its blocks' edges — never authored.

**Mention** respells as *a citation that additionally claims a rendered
position*, kept for the rare case where prose genuinely wants an
in-place rendering. One resolution path, one degree semantics; the
dark-mention containment clause (filed 07-16) ranges over all edges
uniformly.

**How a citation renders is the posture type's declaration** —
human-settled 07-16: the citer knows how it speaks; the engine knows
what is true about the target. The posture type carries a citation
template; the engine supplies the closed set of derivable target facts a
template may place — name, address, kind, and the projection path
*relative to the citing member's own projection*, which is what makes
the rendered citation actionable (the agent's retrieval instruction).
The template crosses the seam as **data** — placeholders, never a
function — because the engine is the sole compiler of projections and
Turing-completeness stays quarantined at authoring time. A template can
select and place target facts but cannot fabricate them: a rendered
citation is true by construction. Instance prose never spells a target.

**The moat and its bound.** The trust the product sells is that the
encoded contract is represented by the prose. The declared→rendered
direction holds by construction (above), with one deterministic check
worth having: a cite-bearing type whose template never places its edge
renders a contract the prose doesn't represent. The rendered→declared
direction — could authored prose smuggle a pointer no edge declares? —
is **deliberately not machinery** (human-ruled 07-16: the citation
regime is not expected to extract cited paths from prose). Extraction
would be heuristic guessing inside a deterministic gate, and the corpus
is full of legitimate untracked path-text (currency `file:line` anchors,
code examples in reference stock). That direction is held by the block
grain itself — sentence-scale prose gives a smuggled pointer nowhere to
hide from a human — and by the intake discipline, which is where
judgment lives on purpose.

## Element B — a posture is a house type extending member

Human-ruled 07-16, superseding the same day's "named member" spelling: a
posture is a **consumer-declared type extending member** — `kind<T>`
where `T` is the property shape the posture's use in the kind requires.
`reference` is a member type with prose and a doc edge; `orientation` is
a member type with prose that the contract budgets. The properties do
the work; the name only labels the type. The engine resolves these types
without understanding them, exactly as it resolves requirement names.

**Host-agnostic, ruled 07-16**: a posture type is declared once and
admitted wherever a host kind's layout says it may appear (may-appear,
per the 07-15 regions ruling). Binding hosts at declaration
(`withinHosts` today) would recreate at the type level the coupling the
vocabulary ruling removed at the engine level — `orientation` is the
same type whether it is a block in a skill body or the whole of a
memory.

The mechanism largely exists: embedded kinds with edge fields are
proven (base-harness's `alternative` is prose plus a `supersede` edge —
structurally identical to `reference`). The **single engine gap**:
built-in kinds do not admit consumer-declared content. A skill or rule
body is `file`-content by definition, and the shipped overlay machinery
lifts a relocated built-in's governs and templates, never its content —
so a consumer cannot today say "my skills' bodies are composed of my
posture types." That one admission — a consumer layout over a built-in
kind's body, ranging over house types — enables the whole model.

`file()` bodies remain the sanctioned exception for genuine long-form —
supporting docs, the pull tier's stock — because unstructured prose is
unbounded and contract-invisible, where a typed block has a name to
justify, properties to fill, and a budget the gate can hold
(human-ruled 07-16: "file-referencing is a path to prose explosion;
prose should be concise and meaningful").

## Element C — the governance package ships in the box

The doctrine the field campaign wrote for centercode (`economy.md`: two
readers, tiers and currencies, the admission test,
restatement-was-enforcement, the filing algorithm) contains zero
centercode facts, and base-harness's `grow-harness` skill independently
converged on the same filing rule. N consumers will otherwise carry N
drifting copies of one truth — the restatement disease at product scale.

Ship it once: a standard governance package — the doctrine document, the
intake skill that routes to it, the contract template (slice
requirements, reached-by degree clauses, tier budgets), and the **default
posture vocabulary as declared, extendable configuration**: posture
types (orientation / directive / reference / step) a consumer extends or
replaces. Per the 07-16 rulings these are configuration members, so they
live here and never in the engine — and they must be real declared
types, not prose describing types, or vocabulary-in-config decays back
into every corpus re-deriving the names. Scaffolded by `install`,
extended by consumers.

This is also the positioning: rulesync makes a harness portable,
marketplaces distribute it, temper makes it *correct* — and "correct" is
undefined without the economy. The product is not a gate you configure;
it is an opinionated theory of harness economics, made checkable.

## Example — the intake skill under this model

Spellings illustrative, not final; the shape is the proposal.

```ts
// kinds.ts — the house posture vocabulary (configuration, not engine).
// Host-agnostic: no withinHosts; admission is the host layout's decision.
export const orientation = kind<{ prose: Prose }>({
  name: "orientation",
  locus: "embeddable",
});
export const directive = kind<{ prose: Prose }>({
  name: "directive",
  locus: "embeddable",
});
export const reference = kind<{ prose: Prose; cite: Member }>({
  name: "reference",
  locus: "embeddable",
  edgeFields: [{ field: "cite" }], // target kind open; resolution corpus-wide
  render: "{prose}: {cite.path}",  // template-as-data over derived target facts
});
```

```ts
// skills/harness-meta.ts — the body is posture-typed values, not a blob.
export const doc_economy = harnessMetaDoc({
  name: "economy",
  prose: file(import.meta.url, "./harness-meta/economy.md"),
});

export const skill_harness_meta = skill({
  name: "harness-meta",
  description: "Intake & maintenance for the harness itself — ...",
  satisfies: ["harness-governance"],
  content: [
    orientation({
      prose: "Run this when the harness itself is the work — new guidance to place, a rule to demote, an audit.",
    }),
    directive({
      prose: "The harness is a projection of the temper program at .temper/: edit the owning module and run temper emit — a direct edit is drift, and the guard refuses it.",
    }),
    reference({
      prose: "The doctrine — what belongs, the filing algorithm, the judgments no clause can hold.",
      cite: doc_economy,
    }),
  ],
});
```

```ts
// harness.ts — admission and semantics are contract, per posture type.
expect: [
  // the builtin's body admits the house postures (the Element B gap)
  { kind: skill, content: [orientation, directive, reference, step] },
  // push posture: budgeted wherever it lives
  { kind: orientation, clauses: [clause(maxLines(3), { severity: "advisory" })] },
  // a rule is a handful of directives, not an essay
  { kind: directive, clauses: [clause(maxLines(4), { severity: "advisory" })] },
]
```

The projection an agent reads is unchanged in kind — frontmatter, then
plain concise prose, the reference line rendered from the edge
(`docs/economy.md`, derived, actionable). What changed is that the gate
now sees the body: it can count a rule's directives, budget its
orientation, and prove every reference resolves and every referenced doc
is reached. Today's `## Invariants` heading convention in the centercode
rules becomes typed structure — three `directive` values the contract
can range over — and prose explosion dies by clause, not by review.

## Rejected alternatives

- **Engine-shipped posture kinds (this proposal's first draft).**
  Rejected by the 07-16 ruling: the engine stays vocabulary-free — it
  resolves names, it never understands them; the built-in kinds
  themselves live at a provider subpath, not the core. The honest cost
  of rejecting it is **interop**: a shared ontology is what would let
  fleet-level tooling reason generically ("this harness is 40%
  directives"; a marketplace lint comparing two harnesses' economies; a
  probe suite asserting "directives fire" without per-corpus mapping).
  Mitigation: tooling keys on the shipped default vocabulary when
  present. Residue, accepted by this ruling: corpora that replace the
  default forgo fleet-level comparison.
- **A member-level `references` framework key (this proposal's second
  draft).** Rejected 07-16: a second declared-edge mechanism beside edge
  fields recreates the three-mechanisms disease inside the proposal
  meant to kill it. Edges are posture properties; the member-level set
  is derived.
- **Host-bound posture types (`withinHosts` at declaration).** Rejected
  07-16: the same type means the same thing everywhere; admission is the
  host layout's decision, not the type's. Binding hosts at declaration
  is the vocabulary coupling reborn one level down.
- **Keep mentions; add references as a separate feature.** Three
  coexisting edge mechanisms, three resolution paths, consumers choosing
  per case with no principle.
- **Body structure via headings/layout only, no typed blocks.** A
  heading types position, not force: it cannot carry an edge, a
  property, or a budget, so contract-invisible prose blobs persist under
  a structured table of contents.
- **Edges out, prose to adjacent files (the first field reading).**
  Rejected by the 07-16 ruling: file blobs are unbounded and
  structureless; the type system stops pushing back on growth exactly
  where growth happens.
- **Doctrine stays consumer-authored.** Two independent corpora already
  converged on it; leaving it out of the box guarantees drift between
  copies of a truth the product itself defines.
- **Authored display (status quo).** The cosmetic half is noise to a
  robot reader; the functional half (the retrieval instruction) is fully
  derivable from the target's facts through the citing type's template;
  every authored display string is a drift surface.
- **A smuggled-citation lint (prose extraction; this proposal's fourth
  draft briefly held it).** Rejected 07-16: extracting cited paths from
  authored prose is heuristic guessing inside a deterministic gate, and
  legitimate untracked path-text is everywhere (currency `file:line`
  anchors, code examples). Collides with loud-or-nothing; the
  rendered→declared direction stays human, held by the block grain.

## Explicitly deferred — the behavioral horizon

`check` proves structure; nothing proves behavior. The field campaign's
highest-value facts (the `skill_listing` herald, `paths`-gate channel
semantics) were unknowable from structure and cost hand-built headless
probes. `Requirement.verifiedBy` — the behavioral remainder — is already
in the model, dormant. The third leg (schema → structure → behavior:
probe fixtures asserting over transcript attachments that a rule fires,
a gated skill is selected, a reference is followed) is a quarter-scale
bet, not part of this fork; it carries its own horizon note.

## Migration sketch

Mentions remain valid — they are positioned citations, so existing
corpora keep working; legacy wire rows stay accepted per the
lock-upgrade posture (0024). The engine grows one admission
(consumer-declared content over built-in kinds) and the host-agnostic
embeddable locus; the governance package grows the default posture
types. Consumer migrations are mechanical and mostly deletion
(centercode: `cite.ts` dies, eleven template-literal bodies become
posture-typed values, the `supportingDocs()` factory collapses into the
native skill-package form, and the hand-maintained `## Invariants`
convention becomes `directive` values the contract can count).
