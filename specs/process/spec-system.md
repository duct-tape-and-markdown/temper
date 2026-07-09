# Spec system — the corpus's own form

The corpus is the source of truth for intent and contract; code is the truth
below the line it draws. If they disagree on intent, the corpus wins — fix
the code, or amend the corpus if intent has shifted. The pipeline reconciles
the two every tick.

## Document kinds

Five, by placement:

- **intent** (`intent.md`) — why, the spine rule, the invariants, positioning.
- **model** (`model/*.md`) — the kernel: one file per layer, one section per
  noun. The stable center.
- **facts & delivery** (`builtins.md`, `distribution.md`) — what ships, how
  it is delivered, what gates the release.
- **decision** (`decisions/NNNN-*.md`) — append-only records: date, status
  (accepted / superseded-by), context, choice, rejected alternatives.
- **process** (`process/*.md`) — how the project runs.

## Form rules

- **Thing-oriented.** A noun lives in exactly one section of one model file.
  Changing a noun touches one file plus one decision record.
- **Contained.** A file reads stand-alone; cross-file references are few and
  explicit. Sections are the addressable grain.
- **Present tense; history evicted.** Body text states what is. Every choice,
  rejection, and supersession lives in `decisions/` and git tags — never
  inline. Edges run one way: a decision references the sections it governs;
  body text never references decisions.
- **Equal representation.** The corpus owns intent and the model's shape;
  code owns all mechanism and all instance data — the predicate enum, the
  default contracts' clauses, the external-fact citations. The corpus never
  enumerates what code enumerates.
- **Budgets.** intent ≤ 100 lines; a model file ≤ 150; a decision ≤ 55
  (calibrated by 0001, the largest legitimate record: a re-founding with a
  full retirement map). A file that cannot stay in budget is restating code
  or hoarding history.
- **External facts cited at the point of enforcement.** A claim about a
  format the outside world owns carries its source and retrieval date where
  it is enforced — a clause's `cite` — and is verified against current docs
  when written, never encoded from memory. The corpus states only the rule
  that this must hold.
- **One name per concept.** The kernel nouns are API. Search before coining;
  new coinage is ratification-tier, and plain words beat metaphor.
- **No middles.** Body text admits no state defined by its own transition —
  no "yet", no "until", no residual, no roadmap. A fact that names its own
  replacement belongs to the queue or the fork board, never the corpus; a
  verb or noun means its plain reading; two states, never three.

## Change ceremony

A change to `model/` is deliberate: update the section to the new truth,
append one decision record in the same commit, and tag the pre-state when
the change retires a noun or a mechanism. A `specs:` commit belongs to the
session, where the human is in the loop — never to an autonomous phase; the
session verifies its own draft against corpus and code before it lands, and
that verification is nobody else's to perform after the fact. A dissolution
names what it retires; derived layers demolish only what a decision names.

## Depth rule

The corpus owns WHAT and WHY; code owns HOW. State a fact here only if code
changing should not be free to change it. If an implementer can change a
detail without breaking intent, the detail belongs to code.
