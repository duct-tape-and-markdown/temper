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

- `(nested-field-addressing)` — OPEN, model question, registered 07-16 from
  MARKETPLACE-KIND's shipped corner cut (c74aab9; the build's own capture at
  `.flume/friction/build-clause-algebra-cannot-address-nested-fields.md`).
  A clause addresses a field by **top-level key** — every `Predicate` carries a
  flat `field: String`, resolved `Features::field` → `fields.get(name)` — and
  `extract::json_to_feature` (`src/extract.rs:916-926`) flattens on the way in:
  an object becomes an opaque `FeatureValue::Map` (inner keys discarded), an
  array becomes a `List` of stringified elements. So `marketplace.json`'s
  documented, **decidable** rules have no clause: `owner.name` required, each
  `plugins[]` entry's `name`+`source`, the `source` union. A catalog Claude Code
  refuses outright passes `temper check` clean; the shipped slice is
  `required("owner")`/`required("plugins")`, with the hold named in
  `marketplaceDefaultContract`'s header (794) and pinned by
  `the_rules_below_the_top_level_are_not_gateable_today`.
  **The fork is the corpus's, not an entry's.** `specs/builtins.md` ("Default
  contracts") sanctions exactly one reason a clause is absent — "Undecidable
  properties are deliberately absent" — plus the almost-empty default contract
  as "the honest encoding, not a gap". Neither names **decidable but
  unexpressible**, and three shipped contracts now carry that hold
  (`skill` 882-885, `plugin-manifest` 608-613, `marketplace` 794). Conflating
  the two is what wants ruling, because the answers differ in kind: a
  field-path spelling may round-trip with no schema delta (`field` is a
  `String` on both faces), but un-flattening `FeatureValue` touches the `type`
  predicate's kind-preservation contract and every consumer — and whether a
  discriminated-union check (`source`'s value selects the required fields)
  enters the vocabulary at all is a language change `specs/model/contract.md`
  ("clause") reserves to a corpus decision, never a build tick's to invent.
  Nothing is broken by leaving it open — the holds are named and pinned.
  BUNDLE-EMIT-THROUGH-KINDS is **not** blocked by it (it consumes whatever the
  contract gates), but its "passes its kind's contract" bar stays thin for
  `marketplace.json` until this rules.

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
  dir by `temper bundle` on every run? Re-verified at 18f219d: no
  `.claude-plugin/` exists here, so both kinds govern globs this repo
  matches with zero members (honest, the `supporting-doc (0)` precedent), and
  nothing is broken by leaving this open. Distinct from
  BUNDLE-EMIT-THROUGH-KINDS, which routes `bundle`'s writers through the kinds
  without any committed manifest. **The blocked-in-fact clause is discharged:
  all three 0031 kinds now ship** — `installed-plugin` (9f22de2),
  `plugin-manifest` (c68f625), `marketplace` (c74aab9), each verified in
  `all_kinds()` on disk (`src/builtin_kind.rs:374-382`). Nothing but the ruling
  holds it now. `.claude/` is human `chore(harness):` territory (CLAUDE.md,
  "The two harnesses"), so this lands as a human act or not at all. No
  dependents.

- `(closed-surface-predicate)` — OPEN, model question, registered 07-16 from
  the drained refactor capture (`build-type-predicate-cannot-cross-the-lock`,
  observed 024ba9b, re-verified at 18f219d). `plugin-manifest`'s documented
  `--strict` bar — an unrecognized top-level field is an error — needs an
  **allow-list over a closed key set**, and the shipped algebra cannot express
  it: `forbidden_keys` is a deny-list, and the complement of a finite set over
  an open key space is not one. `Predicate::Optional` (`src/contract.rs:89` —
  the record said 88; re-read on disk at 18f219d and unmoved across the window,
  so the old number was wrong when written) already records a key as "part of
  the declared schema" but is `Outcome::Holds` unconditionally
  (`src/engine.rs:621`) — nothing consumes the record, so the rows exist and
  mean nothing. The fork: whether a closed surface is opt-in per contract,
  derived from the `optional` rows emit already writes, or a new predicate.
  Each answer either grows or newly reads the closed vocabulary, and
  `specs/model/contract.md` ("clause") makes that a deliberate language
  change — a corpus decision, never a build tick's to invent. Nothing is broken
  by leaving it open: `sdk/src/builtins.ts` (608-613, the first hold bullet of
  `pluginManifestDefaultContract`'s header) names it, the honest "almost-empty
  default contract" posture `specs/builtins.md` sanctions, and
  TYPE-CLAUSE-CONSUMER is scoped to leave that bullet standing.
  **The sibling is discharged:** TYPE-PREDICATE-ROUND-TRIPS — the *wiring* gap
  the same capture named, filed not forked — shipped at c7bd4f3.
  Kin to `(nested-field-addressing)`: both are decidable-but-unexpressible
  holds, and a corpus ruling that names the category may settle the pair
  together. No dependents.

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
  **One debt survives**, accepted, riding the next entry that **reconciles**
  its file (never merely opens it — the precedent below), never a standalone
  entry: `tests/session_start.rs:121/140` still writes `+++`-format
  `.temper/kinds/spec/KIND.md` + `.temper/packages/spec/PACKAGE.md` fixtures —
  live test code asserting stray old-format files are ignored. Two entries
  (664a522, CHECK-ARG-HALF-GATE 4256274) have opened the file and left them;
  no queued entry opens it, so it waits. Re-verified on disk at reconcile HEAD
  18f219d: both dead trees still spelled (121/140), in a file that window never
  touched either and no queued entry edits.
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
  never on the file being opened). Rides whichever entry next reconciles the
  comment lines — no queued entry opens `prose.ts` — never standalone. All ten
  lines re-verified on disk at reconcile HEAD 18f219d (unmoved; `prose.ts`
  untouched in that window too and edited by no queued entry — still no
  carrier). The
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
  42) at reconcile HEAD 18f219d — no queued entry edits `Cargo.toml`, so it
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
