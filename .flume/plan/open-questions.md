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

- `(local-overrides)` — OPEN. The committed-plus-gitignored personal-override
  layer has no stated spelling in the assembly model (`specs/model/pipeline.md`,
  "The SDK" — the harness is one composed value). Candidates: a local harness
  module composed by convention, or an engine-side severity overlay. Blocks
  nothing until someone needs a personal override.

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

- `(multi-harness-projection)` — OPEN, strategic. One member projecting to N
  harnesses (`.claude/rules/` and `.cursor/rules/` from one document) —
  rulesync's portability as an architecture side effect (`specs/intent.md`,
  "Positioning"). The engine is corpus-generic (`specs/model/representation.md`,
  "Reach"), but the write face of foreign formats is open: per-harness
  capability mismatch, which harness is authoritative, whether a lossy
  projection is a verdict or an error. Inherits the AGENTS.md kind question
  (ruled 07-15: not a claude-code kind — Claude Code does not read
  AGENTS.md, docs retrieved 2026-07-15; its consumer is this fork's
  cross-tool story). No dependents.

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

- **Format implementations are engine code** (the generic frontmatter adapter)
  — kept because an external format's mechanics are temper's to implement once;
  the kind that selects them is data (`specs/model/representation.md`, "kind":
  a kind is data, its extractor composed from that data). Grows only by
  deliberate addition.

- **`kinds/` + `packages/` curated trees — RETIRED.** The engine retirement
  drained and the physical trees were deleted (`chore(harness)` 68f187d).
  **One debt survives**, accepted, riding the next entry that **reconciles**
  its file (never merely opens it — the precedent below), never a standalone
  entry: `tests/session_start.rs:121/140` still writes `+++`-format
  `.temper/kinds/spec/KIND.md` + `.temper/packages/spec/PACKAGE.md` fixtures —
  live test code asserting stray old-format files are ignored. Two entries
  (664a522, CHECK-ARG-HALF-GATE 4256274) have opened the file and left them;
  no queued entry opens it, so it waits. Re-verified on disk at reconcile HEAD
  8978596: both dead trees still spelled (121/140), in a file the window never
  touched.
  **The `sdk/src/builtins.ts` half is discharged.** SKILL-NESTED-REFERENCE-DOCS
  (a7a8cc1) carried it named and cut both doc-comment cites to the deleted
  `packages/{rule,memory}.anthropic/PACKAGE.md` files; `rg` over the file finds
  neither. Nine entries had opened builtins.ts and left them — the same lesson
  the record below spent two entries learning, proven a third time: the rider
  discharges when an entry names it, and not when a file is merely opened.

- **`src/read.rs`'s read-strand doc comments spell retired CLI verbs.** The
  one-read-verb ruling (39a4833, contract.md/pipeline.md "Read verbs")
  already shipped in code: the CLI has one read verb `explain` (main.rs
  `Command::Explain`, doc "The one read verb"), with `why`/`impact`/`context`/
  `requirements` as internal strands of `read::explain`, and read.rs:192
  already documents the four-spelling→`explain` unification. But the
  individual strand doc comments still spell `temper why <member>`
  (read.rs:270), `temper impact <member>`/`<leaf-address>` (495/633),
  `temper context <address>` (770), and `temper requirements [<name>]` (1172)
  as if each were its own CLI command. Vocabulary staleness only — the verb is
  `explain`, the strands are correctly-named internal functions. Rides
  whichever entry next opens read.rs (no queued entry does), never standalone.
  Found deriving 39a4833 at HEAD be3bd27. MENTION-ROUTE-RESOLVE-AT-CHECK
  (8eb39fb) then opened read.rs (+25 in the `why` region, 298-440:
  route-resolving deferred mentions) yet left every strand doc comment as
  unchanged context — undischarged; the `why` comment at 270 stayed above the
  hunks and unmoved, the four below shifted +25, 470/608/745/1147 →
  495/633/770/1172. Re-verified on disk at reconcile HEAD 8978596.

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
  comment lines — no queued entry opens `prose.ts` — never standalone. Lines
  re-verified on disk at reconcile HEAD 8978596 (unmoved; `prose.ts` untouched
  in this window). The `sdk/src/kind.ts:257` "posture 3" half of this record is
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
  at residue sweep HEAD a932bb0; re-verified on disk (the comment spans 42-45)
  at reconcile HEAD 8978596.

- **4144b20's retirement of `compose::effective` left one surviving one-line
  comment straggler.** `src/compose.rs:233` ("Unlike `effective`, …") cites
  the retired symbol by name inside `default_contract_from_rows`'s test doc
  comment. Behavior and symbols correct; doc-comment staleness only. It rides
  whichever entry next opens `compose.rs`; **no queued entry does, so it has
  no carrier and waits** — never standalone. Found at residue sweep HEAD
  d029d4b; re-verified on disk, unmoved, at reconcile HEAD 8978596.
  **The `src/contract.rs` half is discharged** — 28ad61f rewrote
  `Predicate::target`'s doc (now `documented_field`, contract.rs:494) and the
  retired severity-flip layer's "for layering purposes" vocabulary is gone
  from the file. It discharged on the shape this record spent two entries
  learning: a carrier named in *this file* is not read at build time, and only
  an entry naming the rider in its own `files[].description` discharges one —
  GLOB-VALIDITY-PREDICATE (46b8cd1) and FORMAT-OMITS-EDGE-CLAUSE (13c58ed)
  each opened contract.rs under no such naming and left the straggler, while
  PREDICATE-SELECTION-ALGEBRA carried it named and closed it. Twice proven
  now, with NESTED-FILE-LOCUS/91c288c (the `drift.rs:570` rider): name the
  rider in the entry, or it does not discharge.

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
