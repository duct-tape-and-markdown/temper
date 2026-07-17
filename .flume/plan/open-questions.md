# Open questions

Product/architecture forks not yet settled. Each is keyed with a `(slug)` so a
pending entry can declare `dependsOnForks: ["slug"]` and be held until resolved.

**Lifecycle (the anti-accumulation rule, John 07-06): this file holds OPEN
forks only.** Resolution = encode the ruling (corpus Decision, or the resolving
commit body) and **delete the record** — git history is the archive; "kept as
the decision record" is retired as a category. Reconciliation evidence (DATUMs)
goes in the plan commit body, never appended here. Rationale: this file is
inlined whole into every plan prompt — every dead line is a per-tick context
tax.

## Open forks

- `(layer-delivery-format)` — OPEN, kernel question, **the corpus's, not an
  entry's**. Registered 07-16, deriving decision 0030 (3c1a58c). **This is not
  the deferred admission 0030 overruled** — admission is ruled and settled;
  what is unruled is one question the record does not answer, and all four
  derivations its Consequences name rest on it: **what artifact does `--layer`
  name, and how does an uncommitted layer carry its dialed clauses to the
  engine?** 0030 needs a layer to carry clauses (softening IS a dialed
  severity), to be **parsed by check** ("A layer that fails to parse fails the
  check, fail-closed"), and to compose "over the locked declarations at check
  time". A clause is a declaration row — so an uncommitted layer's clauses are
  declarations the gate reads that are **not in the lock**, and three kernel
  sentences box in every reading (all re-read on disk at f70cd03):
  - `model/pipeline.md`:123-125 ("The lock") — "emit is its sole producer —
    no verb compiles anything else into declaration rows, and the gate reads
    declarations from nowhere but the lock."
  - `model/pipeline.md`:147 — the gate consumes "committed artifacts plus the
    lock and nothing else: offline, no language runtime."
  - `model/pipeline.md`:116-117 ("Emit") — the payload "is internal … not a
    designed public interchange".
  No reading of the disk settles it; three survive, each shipping a different
  artifact:
  1. **A second, uncommitted lock-shaped file emit writes** at a
     natively-local path, joined over the lock at check. One declaration
     format, one producer — "no verb compiles anything else into declaration
     rows" survives verbatim and only "nowhere but the lock" widens to a lock
     stack; 0030's check-side-only rule already half-sanctions it (emit
     projects uncommitted members to natively-local targets). **This record's
     recommendation.** Its cost, stated: an uncommitted lock is a receipt for
     bytes no review saw, and drift's story over it is undefined — what
     fingerprint does a local projection compare against?
  2. **A narrow authored override format** the engine parses directly, emit
     uninvolved. Keeps emit committed-only; mints a second home for the clause
     vocabulary outside the SDK's types — the duplicate-surface disease
     (`process/engineering.md`, "One job, one home").
  3. **The emit payload as check's input.** Promotes the internal payload to a
     consumed interface (against 116-117) and puts Node at check time (against
     147).
  Whichever is ruled amends a kernel sentence in `model/pipeline.md` — a
  deliberate ceremony with a decision record (`process/spec-system.md`,
  "Change ceremony"), never a build tick's to invent nor plan's to guess.
  Nothing is broken by leaving it open: no layer surface exists at all (`rg`
  for `--layer`/`layers`/`LayerSlot` over `src/main.rs`, `sdk/src/assembly.ts`,
  `sdk/src/builtins.ts` is empty at f70cd03), so today's gate is exactly the
  committed harness's — what every placement already assumes. What it costs is
  the capability 0030 ruled important. **Dependents: all four of 0030's
  derivations** — the stack and envelope in engine and SDK, the claude-code
  face's cited precedence declaration, `--layer` on check, the announcement
  line. None is filed until this rules. Rider for whichever entry encodes the
  slots: 0030 requires the claude-code precedence cite (user < project < local
  < invocation; settings docs, retrieved 2026-07-16) **re-fetched raw at encode
  time** per `builtins.md`.

- `(clause-vocabulary-holds)` — OPEN, model question, **the corpus's, not an
  entry's**. Registered 07-16; consolidated 07-16 from the two instance-keyed
  records this loop had filed separately (`(nested-field-addressing)`,
  `(closed-surface-predicate)`), because one ruling settles them and three
  drifting records taxed every tick to say it once.
  `specs/builtins.md` ("Default contracts") sanctions exactly **one** reason a
  clause is absent — "Undecidable properties are deliberately absent" — plus
  the almost-empty default contract as "the honest encoding, not a gap".
  Four shipped holds are neither: they are **decidable, documented, and
  unexpressible**, so a member the harness refuses outright passes
  `temper check` clean. The corpus does not name that category, and its
  "Strictest documented profile" stance is what the holds sit against. The
  ruling wanted: is decidable-but-unexpressible a sanctioned absence at all,
  and what discipline governs closing one — because every close grows the
  **closed** predicate vocabulary, which `specs/model/contract.md` ("clause")
  reserves to a corpus decision, never a build tick's to invent.
  The four instances, each with its own mechanism and its own answer — the
  reason this is one ruling and four derivations, not one fix:
  - **Nested field addressing** (from MARKETPLACE-KIND's corner cut, c74aab9;
    capture `build-clause-algebra-cannot-address-nested-fields.md`). Every
    `Predicate` carries a flat `field: String` (`Features::field` →
    `fields.get(name)`), and `extract::json_to_feature`
    (`src/extract.rs:921`, re-verified 8913b59) flattens on the way in: an
    object becomes an opaque `FeatureValue::Map`, inner keys discarded. So
    `owner.name` required and each `plugins[]` entry's `name`+`source` have no
    clause; `required("owner")`/`required("plugins")` is the shipped slice,
    held in `marketplaceDefaultContract`'s header (806) and pinned by
    `the_rules_below_the_top_level_are_not_gateable_today`
    (`tests/marketplace_kind.rs:252` — this record carried 267, wrong since
    written; re-read on disk at 8913b59, unmoved since 3593ab6). A field-path spelling may round-trip
    with no schema delta (`field` is a `String` on both faces); un-flattening
    `FeatureValue` instead touches the `type` predicate's kind-preservation
    contract and every consumer.
  - **Allow-list over a closed key set** (from capture
    `build-type-predicate-cannot-cross-the-lock`, observed 024ba9b).
    `plugin-manifest`'s `--strict` bar — an unrecognized top-level field is an
    error — needs one, and `forbidden_keys` is a deny-list: the complement of
    a finite set over an open key space is not one. `Predicate::Optional`
    (`src/contract.rs:89`) already records a key as part of the declared
    schema but is `Outcome::Holds` unconditionally (`src/engine.rs:621`) —
    nothing consumes the record. The answer forks three ways: opt-in per
    contract, derived from the `optional` rows emit already writes, or a new
    predicate. Held at `sdk/src/builtins.ts:609`.
  - **Type alternation** — this window's find (swept 18f219d..d88ed75).
    TYPE-CLAUSE-CONSUMER (7e266ad) shipped `type("keywords", "list")`, the
    `type` predicate's first real consumer, and stated the remainder plainly:
    `skills`, `commands`, `agents`, `hooks`, `mcpServers`, `lspServers` are
    documented `string|array` (the last three also `object`), and
    `type(field, kind: ValueType)` (`sdk/src/contract.ts:74`) declares one
    lattice kind, never a union — six documented fields of the strictest
    profile ungated until the algebra carries an alternation. `keywords`'s
    single declared kind is exactly why it was the expressible slice. Held at
    `sdk/src/builtins.ts:612-620`. The SDK's TypeScript types hold the union
    bar for an SDK author; a hand-written manifest is unguarded.
  - **Shape rules** (pre-existing, unkeyed until now): `skill`'s name must not
    start/end with a hyphen or carry consecutive hyphens, and the platform's
    "no XML tags in the description" — both decidable, both absent "pending a
    vocabulary addition (a narrow shape predicate governs additions)"
    (`sdk/src/builtins.ts:890-893`). `allowed_chars` is charset mechanics and
    does not reach a positional rule.
  Nothing is broken by leaving this open — every hold is named in its
  contract's header and the marketplace one is pinned by a test. What it costs
  is honesty of the gate's reach: BUNDLE-EMIT-THROUGH-KINDS (0e7dca2) now
  publishes `marketplace.json` as a member of the kind that types it and
  `check` reads it back off the bundled tree — verified on disk — so the
  "passes its kind's contract" bar is real and, for that file, thin by exactly
  the first instance above. No dependents.

- `(eval-capability)` — OPEN, strategic, parked past launch. Harness evals: a
  requirement carries prose intent and a verifier edge
  (`specs/model/contract.md`, "requirement"), and the graph gives eval
  selection for free (impact → which evals re-run). If ever built: a verifier
  type and/or the behavioral remainder made concrete — probabilistic, NEVER a
  well-formedness check or the hard gate (`specs/intent.md`, invariant 2 / "The
  honest bound"). Do not let it near the launch wedge.
  **Field evidence, 07-16:** behavior is the unverified half — `check` proves
  structure, and the consumer campaign's two highest-value facts (the
  `skill_listing` herald, `paths`-gate channel semantics — the evidence that
  ruled 0028) were unknowable from structure and cost hand-built headless
  probes (transcript-verified, 2.1.210). So the fork's cost side is now
  measured, not assumed. `Requirement.verifiedBy` is already
  in the model, dormant. Unchanged: this is a quarter-scale bet, parked past
  launch, and `docs/horizons.md` is where a human carries it — plan does not
  write that page.

- `(plugin-author-dogfood)` — OPEN, human's call, registered 07-16 from
  decision 0031's own Consequences: "Temper's own repo becomes a **candidate**
  plugin-author corpus once the producer kinds exist — the dogfood extends to
  the surface it ships." A candidate is not a requirement, and plan does not
  promote one. The question: does this repo commit a `.claude-plugin/` tree as
  `plugin-manifest` + `marketplace` members of its own `.temper/` harness —
  authored, gated, and emitted — rather than assembled fresh into an output
  dir by `temper bundle` on every run? Re-verified at 8913b59: no
  `.claude-plugin/` exists here, so both kinds govern globs this repo
  matches with zero members (honest, the `supporting-doc (0)` precedent), and
  nothing is broken by leaving this open. Distinct from
  BUNDLE-EMIT-THROUGH-KINDS, which **shipped** (0e7dca2) and routes `bundle`'s
  writers through the kinds into an output dir, with no committed manifest —
  so it moved nothing here. **The blocked-in-fact clause is discharged:
  all three 0031 kinds now ship** — `installed-plugin` (9f22de2),
  `plugin-manifest` (c68f625), `marketplace` (c74aab9), each verified in
  `all_kinds()` on disk (`src/builtin_kind.rs:374-382`). Nothing but the ruling
  holds it now. `.claude/` is human `chore(harness):` territory (CLAUDE.md,
  "The two harnesses"), so this lands as a human act or not at all. No
  dependents.

- `(multi-harness-projection)` — OPEN, strategic. One member projecting to N
  harnesses (`.claude/rules/` and `.cursor/rules/` from one document) —
  rulesync's portability as an architecture side effect (`specs/intent.md`,
  "Positioning"). The engine is corpus-generic (`specs/model/representation.md`,
  "Reach"), but the write face of foreign formats is open: per-harness
  capability mismatch, which harness is authoritative, whether a lossy
  projection is a verdict or an error. Inherits the AGENTS.md kind question
  (ruled 07-15: not a claude-code kind — Claude Code does not read
  AGENTS.md, docs retrieved 2026-07-15; its consumer is this fork's
  cross-tool story). Demand side is no longer zero (07-16 war game,
  simulated): 2/8 personas rate one-member→N projections an adoption-blocker
  and want a **counterpart-drift check** — a fourth open face beside the
  three above. Timing unchanged. No dependents.

## Kept on purpose — deliberate asymmetries (re-read every tick)

Every asymmetry below is a **choice with a condition**, not a fact. When its
condition arrives, it is the next break. If work touches one, surface it.

- **A pack is a skill — no skill-package kind** (human-ruled 07-15, 39a4833;
  reaffirmed by 0025's Rejected list, 82c816e: "a separate skill-package or
  nesting kind for supporting docs — the built-in already owns the shape; a
  parallel kind would be the duplicate-surface disease"). The condition is a
  consumer who *cannot* express a pack with the built-in `skill` plus its
  nested reference documents. The 07-16 datum that looked like demand — the
  centercode `supportingDocs()` factory, minting one nested-root kind per
  skill directory — is **routed, not pending**: it was ergonomics standing in
  for a template fact the spec already declares and the SDK lacks.
  TEMPLATE-FILE-CHILD-FACT shipped that fact (794678f), 0027 (abe5d5d)
  resolved `(nested-file-child)`, and SKILL-NESTED-REFERENCE-DOCS **landed**
  (a7a8cc1): `skill` templates one file-child layer at its directory's
  markdown and `supporting-doc` is that layer's kind, verified on disk. So
  the factory now deletes against `skill` + `supporting-doc`, and this
  record's condition — a consumer who *cannot* express a pack with the two —
  is what a future pack argument must clear.

- **Default-contract auto-adoption** (a bare harness gets the built-in kinds
  checked with no assembly declaration) — kept for the zero-config front door;
  the engine embeds a built-in lock, the default contract in declaration shape,
  so a lockless harness is still fully gated (`specs/model/pipeline.md`, "The
  lock"). Data, not code.

- **Format implementations are engine code** (the frontmatter adapter, and
  since 3ed8d2b the `json-document` reader beside it) — kept because an
  external format's mechanics are temper's to implement once; the kind that
  selects them is data (`specs/model/representation.md`, "kind": a kind is
  data, its extractor composed from that data). Grows only by deliberate
  addition, and the inventory's second entry was exactly that.

- **`kinds/` + `packages/` curated trees — RETIRED.** The engine retirement
  drained and the physical trees were deleted (`chore(harness)` 68f187d).
  **One debt survives**, accepted: `tests/session_start.rs:122/141` still
  writes `+++`-format `.temper/kinds/spec/KIND.md` +
  `.temper/packages/spec/PACKAGE.md` fixtures. **Reclassified 07-16 — this
  record had it wrong, and the misfiling is why it never discharged.** It was
  filed as narration staleness riding a reconcile; it is not. Read on disk at
  8913b59: the fixtures sit inside
  `stray_custom_kind_shaped_fixtures_never_disturb_a_clean_session_start`
  (113), whose *subject* is that files in the retired format are inert — the
  vocabulary is the assertion, not a comment beside it. So no hygiene pass can
  "reconcile" it: the live question is whether temper still wants a test
  pinning retired-format inertness at all, and that is a value call
  (subtraction before addition, CLAUDE.md) no build tick may invent. **The
  reclassification is now proven, not predicted:** CHECK-RUNNER-REMAINDER
  shipped (a9a21a9), edited this very file at 49, and left the fixtures
  standing — the third entry to open it and correctly not touch it (after
  664a522 and CHECK-ARG-HALF-GATE 4256274). That fold shifted the cites two
  lines (121/140 → 122/141, fn 111 → 113); the numbers above are re-read on
  disk at 8913b59, not carried forward. Not a rider awaiting a carrier; a
  question awaiting a human.
  **The `sdk/src/builtins.ts` half is discharged.** SKILL-NESTED-REFERENCE-DOCS
  (a7a8cc1) carried it named and cut both doc-comment cites to the deleted
  `packages/{rule,memory}.anthropic/PACKAGE.md` files; `rg` over the file finds
  neither. Nine entries had opened builtins.ts and left them — the same lesson
  the record below spent two entries learning, proven a third time: the rider
  discharges when an entry names it, and not when a file is merely opened.

- **Pre-recut vocabulary survives in `sdk/src/prose.ts`'s doc comments.**
  0001's retirement map (law → invariant/spine rule, posture → retired,
  decisions renamed `NNNN-*.md`) still narrates the file: "law 5" at
  6/141/258, "law 8" at 11, "posture N" at 126/156/161/188/238, and the
  pre-recut decision cites `` `15-kinds.md` ``:126 / `` `20-surface.md` ``:200
  — neither file exists. Doc-comment staleness only; behavior and symbols are
  correct. The narration **self-propagates**: a8562b5 wrote line 10 fresh in
  the retired vocabulary, and PROSE-INTERLEAVE-SDK (6450ba6) rewrote the two
  "posture 3" comments fresh — the very lines this record names — so each
  entry that opens the file without reconciling it deepens the rider. Four
  entries have now opened `prose.ts` and left every narration line as
  unchanged context (the precedent: the rider discharges on *reconciliation*,
  never on the file being opened). **The rider has a carrier as of 07-16:**
  SDK-BLOCKS-FILE-REFUSAL edits `blocks()` (242) — whose own doc comment at
  238 is one of the ten lines — and names the reconciliation in its
  `files.edit` description, so this discharges when that entry ships and not
  before. All ten lines re-verified on disk at c370924 (unmoved: `git log
  8913b59..c370924 -- sdk/src/prose.ts` is empty). The
  `sdk/src/kind.ts:257` "posture 3" half of this record is
  **discharged**: TEMPLATE-FILE-CHILD-FACT (794678f) carried it — 0025 made
  "posture" a consumer-declared member type, not a body-authoring mode number,
  and the cite is gone from the file. (Fixture body text inside tests is a
  separate class, excluded — `src/kind.rs`'s `15-kinds.md` strings and
  `src/extract.rs`'s two `"…law 5"` decision-fixture strings are `.to_string()`
  test data, not cites.)

- **Cargo.toml's schemars dep comment is doubly stale.** It cites
  `src/schema/interchange.rs` (the module is `src/schema.rs`; no `schema/`
  dir exists) and assigns ts-rs the interchange-TS role the seam bindings
  superseded (ts-rs's live job is the `sdk/src/generated/` seam home,
  36a7662; `src/schema.rs` is schemars-only). Comment staleness — rides
  whichever entry next opens `Cargo.toml`, never a standalone entry. Found
  at residue sweep HEAD a932bb0; re-verified on disk (the cite still sits at
  42) at reconcile HEAD 8913b59 — no queued entry edits `Cargo.toml`, so it
  still has no carrier.

- **`.flume/` is ungoverned by temper** — the machine that builds temper is not
  yet under its gate; a candidate governed corpus once the custom-kind story
  proves end to end (`specs/model/representation.md`, "Reach"). Narrowed
  2026-07-09: the existence half of `.flume/prompts/{plan,build}.md`'s two
  `.claude/` pointers (`pending-entry` rule, `capture-friction` skill) is now
  graph-tracked — `harness.ts` declares both as `required` assembly
  requirements, each member `satisfies`-links to its own (a real
  `requires`/`satisfies` edge needs no `.flume/`-side kind; `emit`/`check`
  now refuse if either loses its satisfier). What remains genuinely
  ungoverned: the prompts' prose *spells the identifier* outside any gate —
  a member rename moves the graph edge with it but leaves the prompt's text
  stale-but-harmless (neither trigger mechanism reads the prose). Kept — a
  cosmetic residual, not the drift risk originally logged here.

- **`docs/` is candidate intent, not intent** — human territory,
  fence-excluded; plan never reads a horizon entry as intent.
