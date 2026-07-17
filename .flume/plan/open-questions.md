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

- `(source-union-predicate)` — OPEN, non-blocking, registered 07-16 routing
  0033. 0033 closes four holds; a fifth survives its own wave, and the corpus
  does not rule it. `marketplaceDefaultContract`'s header
  (`sdk/src/builtins.ts`:813-816, read at 15b7a3a) names the `source` union as
  needing "both of the above **plus a discriminated-union predicate**": the
  relative-path form's leading `./`, the four object forms' `source`
  discriminator and their required fields. 0033's widening 2 makes
  `plugins[*].source` **addressable** — `required` and `type` over it become
  spellable — and stops there; which of the five documented forms a value is,
  and whether that form's own required fields are filled, no predicate
  decides. So after FIELD-ADDRESSING-RFC-9535-SUBSET ships, the bullet stays
  as a hold rather than discharging, which is admissible precisely because it
  already names its closing widening (`builtins.md`, "Default contracts": a
  hold with no named closing widening is not a hold). The question is whether
  that fifth widening is ever ratified — a vocabulary addition is a deliberate
  language change (`model/contract.md`, "clause"), never plan's to derive and
  never build's to invent. Nothing is broken by leaving it open: the union is
  guarded for an SDK author by the `MarketplaceSource` type today, and what is
  unguarded is the hand-written catalog. No dependents.

- `(settings-local-kind)` — OPEN, human's call, registered 07-16 from 0032's
  own Consequences: "The claude-code face's `settings.local.json` is the first
  candidate local-locus layout kind beyond the dial itself." A candidate is not
  a requirement, and plan does not promote one — the `(plugin-author-dogfood)`
  precedent. The question: does the claude-code face ship a
  `settings.local.json` kind? The "can it" half is now **built, not merely
  ruled** — 0034's three derivations all ship (bce89b7, 09ef5ea, 6e7b958), so
  a local JSON kind would be gated in place under `json-document` and its
  always-gitignored document actually found by the walk. Ship-or-not is all
  that remains, and it costs no upstream work.
  Nothing is broken by leaving it open: the file is ungoverned today and
  no member declares it. No dependents.

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
  dir by `temper bundle` on every run? Re-verified at 9409a6c: no
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

- **Format implementations are engine code** (the frontmatter adapter, the
  `json-document` reader beside it since 3ed8d2b, and `toml-document` since
  09ef5ea) — kept because an external format's mechanics are temper's to
  implement once; the kind that selects them is data
  (`specs/model/representation.md`, "kind": a kind is data, its extractor
  composed from that data). Grows only by deliberate addition, and each of
  the inventory's two additions was exactly that. The third entry sharpened
  the record rather than straining it: `toml-document` is a **read face with
  no write twin**, so `project_bytes` now returns `Option<String>` over an
  exhaustive `Format` match — a format that cannot be written refuses at the
  writer rather than inheriting a fall-through. The next format answers that
  match by construction, which is what keeps "deliberate" mechanical here.

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

- **Two stale cites, one now carried — ride-only, never an entry.**
  Comment and citation staleness never files a standalone entry; it rides
  whichever entry next opens the file. The `sdk/src/prose.ts` record that sat
  here for four entries is **retired, discharged on disk at 9409a6c**:
  SDK-BLOCKS-FILE-REFUSAL (42a2dd1) carried the rider it was given and cut all
  ten pre-recut narration lines — `rg` over the file for `law N` / `posture N`
  / `15-kinds` / `20-surface` now returns nothing. That closes the class the
  record tracked. Sweeping the same class across `src/` + `sdk/src/` still
  leaves exactly two lines, both re-read on disk at 399d8e3 — but **one now
  has a carrier**, which is the ride-only rule working rather than a gap:
  - `Cargo.toml`:37-39 and 42-45 — **carried, no longer orphaned.** 37-39
    sells `regex` as "the `pattern` primitive", a predicate that does not
    exist and that the corpus refuses outright; 42-45 cites
    `src/schema/interchange.rs` (the module is `src/schema.rs`; no `schema/`
    dir exists) and assigns ts-rs the interchange-TS role the seam bindings
    superseded (ts-rs's live job is the `sdk/src/generated/` seam home,
    36a7662; `src/schema.rs` is schemars-only).
    FIELD-ADDRESSING-RFC-9535-SUBSET adds `serde_json_path` to this file and
    names both riders in its first edit — filed at 6f72a3b, which is what
    retired this record's old "no queued entry opens either file" claim.
    Found at sweep HEAD a932bb0.
  - `src/roster.rs`:470 — **the last orphan of its class**: the doc comment on
    the `membership_roster` test helper cites `` `10-contracts.md` ``, a file
    0001 deleted. The sentence's claim is live — a `target` names a declared
    requirement — so the cite comes out, never gets re-pointed at a surviving
    file. No queued entry opens `src/roster.rs`; it waits for one, and never
    becomes an entry of its own. Cite re-read on disk at 80685db, still 470 —
    but re-read rather than carried, because an orphan's address drifts under
    the ride-only rule (6d145fa moved it 465→470 while this record slept).
  Fixture body text inside tests stays a separate class, excluded — it is
  `.to_string()` test data, not cites: `src/kind.rs`'s `15-kinds.md` strings,
  `src/read.rs`'s `20-surface` member ids, `tests/display_rule.rs`'s "law 5"
  and "law 7" rejected-entry bodies, and `src/extract.rs`'s two `"…law 5"`
  decision-fixture strings.

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
