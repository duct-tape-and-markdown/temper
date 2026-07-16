# Posture recursion — one edge noun, block grain, doctrine in the box

Status: design proposal from the consumer-lane field campaign (centercode,
07-14 → 07-16). The frame was human-ratified in the field 07-16 ("go");
the design lands only through review and the corpus. Supersedes the
piecemeal reading of the four 07-16 inbox notes — this is the one entry
they were fragments of.

## The claim

The harness files content by **posture**: what fires unprompted is a
directive, what is reached on demand is a reference, what executes is a
procedure, what orients is orientation. The field campaign found the same
algorithm one scale down: a skill body's natural grain is a short list of
**posture-typed blocks** (orientation / directive / reference / step), each
a sentence or three, attributes attached to the block that speaks them.

So the design is one move, not four features: **a single posture
vocabulary that recurses** — harness → member (posture picks the kind),
member → block (posture picks the block type). Delivery semantics attach
to posture at every scale: push-posture content is budgeted wherever it
lives, pull-posture content is reachability-checked wherever it lives,
and edges attach to the posture-bearing node that speaks them.

## Element A — one edge noun

A **reference** is a declared edge from a member or block to another
member: intent, not a character position. A **mention** respells as *a
reference that additionally claims a rendered position*. `EdgeField`
(fact 5) is the same noun at kind-definition scope. One resolution path
in the gate; one degree semantics; the dark-mention containment clause
(filed 07-16) ranges over all of it uniformly.

Display is never authored. It derives from the target's kind, and the
derivation is functional, not cosmetic — the rendering is the agent's
retrieval instruction: a skill renders as its invocable name, a
supporting doc as its readable projection path. The engine knows both.

Consumer evidence: every rule and skill body in the centercode corpus is
an inline template literal solely to thread positional mentions; a
hand-rolled `cite()` exists solely to control address spelling and
display. Under this element both delete.

## Element B — named blocks are the mechanism; posture is configuration

Human-ruled 07-16: "postures are a named member, that's it — a member of
the configuration, not the engine." The engine ships exactly one thing
here: the **named block** — an embedded member that carries concise
prose, block-attached edges, and contract bindings (may-appear per the
07-15 regions ruling, budgets, degrees). That mechanism largely exists:
base-harness's `invariant`/`step`/`alternative` are consumer-declared
embedded kinds already. What the engine never learns is a posture word —
`orientation` is a name some configuration declares, exactly as
`dev-standards` is a requirement name the engine resolves without
understanding. Declaring a posture vocabulary is consumer work on
purpose: encoding the semantic structure is the point.

`file()` bodies remain the sanctioned exception for genuine long-form —
supporting docs, the pull tier's stock — because unstructured prose is
unbounded and contract-invisible, where a block has a name to justify
and a budget the gate can hold (human-ruled 07-16: "file-referencing is
a path to prose explosion; prose should be concise and meaningful").

Engine-side, this element therefore shrinks to ergonomics: the authoring
path for "a short list of named blocks with plain prose" must not
require a hand-rolled `span()` (base-harness rolls one today) or
template ceremony. The vocabulary itself moves to Element C.

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
posture vocabulary as declared, extendable configuration**: named block
kinds (orientation / directive / reference / step) a consumer extends or
replaces. Per the 07-16 ruling these are configuration members, so they
live here and never in the engine — and they must be real declared
kinds, not prose describing kinds, or vocabulary-in-config decays back
into every corpus re-deriving the names. Scaffolded by `install`,
extended by consumers. This is also the positioning: rulesync
makes a harness portable, marketplaces distribute it, temper makes it
*correct* — and "correct" is undefined without the economy. The product
is not a gate you configure; it is an opinionated theory of harness
economics, made checkable.

## Rejected alternatives

- **Keep mentions; add `references` as a separate feature.** Three
  coexisting edge mechanisms (mention, EdgeField, references), three
  resolution paths, and consumers choosing per case with no principle.
  The recursion collapses them to one noun with an optional position.
- **Engine-shipped posture kinds (this proposal's own first draft).**
  Rejected by the 07-16 ruling: the engine stays vocabulary-free — it
  resolves names, it never understands them. A baked posture set is the
  std-lib pattern-match from language products; temper's grain is
  vocabulary-in-configuration (the built-in kinds themselves live at a
  provider subpath, not the core). The default vocabulary rides in the
  governance package as config.
- **Body structure via headings/layout only, no typed blocks.** A heading
  types position, not force: it cannot carry an edge, a budget, or a
  posture, so contract-invisible prose blobs persist under a structured
  table of contents.
- **Edges out, prose to adjacent files (the first field reading).**
  Rejected by the 07-16 ruling: file blobs are unbounded and
  structureless; the type system stops pushing back on growth exactly
  where growth happens.
- **Doctrine stays consumer-authored.** Two independent corpora already
  converged on it; leaving it out of the box guarantees drift between
  copies of a truth the product itself defines.
- **Authored display (status quo).** The cosmetic half is noise to a
  robot reader; the functional half (the retrieval instruction) is fully
  derivable from the target's kind and projection path; every authored
  display string is a drift surface.

## Explicitly deferred — the behavioral horizon

`check` proves structure; nothing proves behavior. The field campaign's
highest-value facts (the `skill_listing` herald, `paths`-gate channel
semantics) were unknowable from structure and cost hand-built headless
probes. `Requirement.verifiedBy` — the behavioral remainder — is already
in the model, dormant. The third leg (schema → structure → behavior: probe
fixtures asserting over transcript attachments that a rule fires, a gated
skill is selected, a reference is followed) is a quarter-scale bet, not
part of this fork; it carries its own horizon note.

## Migration sketch

Mentions remain valid — they are positioned references, so existing
corpora keep working; legacy wire rows stay accepted per the lock-upgrade
posture (0024). The SDK grows the `references` framework key beside
`satisfies`, the std-lib block kinds, and the governance package;
consumer migrations are mechanical and mostly deletion (centercode:
`cite.ts` dies, eleven template literals become declared edges plus
typed blocks, the `supportingDocs()` factory collapses into the native
skill-package form).
