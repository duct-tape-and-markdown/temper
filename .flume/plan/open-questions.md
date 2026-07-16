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

- `(guidance-climb)` — OPEN, raised routing 0025. The decision's third
  delivery layer — "the opinionated layer is **guidance describing the
  climb** from plain built-ins to the postured shape" — has no home in the
  corpus body, and the two homes that exist both refuse it. The plugin's
  skill "teaches mechanics … It never teaches taste: opinions live in
  default contracts" (`specs/distribution.md`, "What ships — three
  channels"), and a default contract carries opinion only as a **clause's**
  guidance (`specs/builtins.md`, "The clauses live in code") — but the climb
  is not a check, so there is no clause for it to ride. So the ratified layer
  is currently unspellable: a guidance artifact with no shipped channel.
  **What the ruling owes:** which surface carries it, and how it stays
  taste-without-a-gate (invariant 4: temper never decides the harness is
  missing something nobody declared — a climb the tool describes must not
  become a floor the tool implies). 0025 sequences it anyway ("the guidance
  climb and the example recut follow the machinery"), so nothing waits on
  this: the four machinery entries are unblocked, and the fork resolves
  through the inbox before any entry can cite a section for the climb.

- `(nested-file-child)` — OPEN, raised routing cc5a9b3 (0025's amended
  supporting-docs bullet). The ruling ships `supporting-doc` as the child kind
  of `skill`'s template (`specs/builtins.md`, "The shipped kinds"), but a
  **file-locus nested child has no spelling** in either layer. Three
  collisions, each re-verified on disk at d97a704:
  (1) **narrowed by TEMPLATE-FILE-CHILD-FACT (`794678f`), not closed.** A kind
  can now *declare* a file-child layer — `KindFacts.templates`
  (`sdk/src/kind.ts:158`) carries the child kind plus the path pattern — but
  that entry's scope was the declared fact alone: nothing composes or resolves
  such a child. `EmbeddedMemberValue` / `NestedMemberRow`
  (`sdk/src/declarations.ts:438`) still carry a host's *body* children only,
  and nothing discovers a file child off the pattern. So a skill still has no
  surface through which to compose a child that owns its own file;
  (2) `Locus` is binary (`sdk/src/kind.ts:57`) — `{at: root+glob}` or
  `{embedded}`; neither says "my path is my host's template pattern under my
  host's unit";
  (3) if `supporting-doc` self-governs instead, its glob overlaps `skill`'s
  `*/SKILL.md` under `.claude/skills` — and a document's kind is declared by
  its position alone (`specs/model/representation.md`, "kind"), so an overlap
  makes position undecidable.
  **What the ruling owes:** whether a nested file child's path composes from
  the host template's pattern (locus grows a third spelling) or the child
  self-governs (and how the overlap is legal); the composition surface a skill
  declares its documents through; and the pattern itself as an external fact —
  code.claude.com/docs/en/skills, "Add supporting files", documents supporting
  files of any type anywhere in the skill directory while the ruling types only
  prose documents, so which paths the template claims is a cited fact, never a
  guess. Also owed: whether `supporting-doc` carries a default contract
  (`specs/builtins.md`, "Default contracts": each shipped kind does, and an
  almost-empty one is the honest encoding) — a fields-free, prose-only,
  channel-less kind has nothing decidable to check.
  **Nothing waits that need not:** TEMPLATE-FILE-CHILD-FACT shipped
  (`794678f`) — the template's *declared fact*, child kind plus path pattern,
  which is what representation.md states whatever resolves the child. What
  this fork gates is only the half above it: composition, resolution, and the
  built-in adoption, which returns as an entry once the fork rules, through
  the inbox.

- `(mention-gate-containment)` — OPEN. A skill's `paths` removes it from every
  invocation channel until a matching file is read (`specs/builtins.md`, "The
  shipped kinds"; `sdk/src/builtins.ts` `Skill.paths`, verified 2.1.210), so a
  rule→skill mention fires only if the rule's own scope falls inside the target
  skill's gate. That containment — `rule.paths ⊆ skill.paths` over every mention
  edge whose source and target both declare `paths` — is held by nothing, and
  the failure is silent: a probe on 2.1.210 (headless testbed, 2026-07-16) shows
  gate-opening emits a `skill_listing` delta only for the *target's own* paths
  match, so a mention arriving through a rule outside the gate points at an
  inventory absence with no error anywhere — it reads as "the model ignored the
  rule". The consumer hand-derives each pack-skill's `paths` as the union of its
  mentioning rules' paths, which drifts the first mention added without widening
  the gate.
  **Why a fork, not an entry:** the check needs a predicate the closed
  vocabulary does not carry — `degree` is the only mention-edge predicate, and
  nothing compares two members' glob sets (`src/contract.rs:81`). Adding one is
  a deliberate language change (`specs/model/contract.md`, "clause"), ratified
  by a decision before it is built: 0022 (`f67303c`) admitted `glob-valid`
  *before* `46b8cd1` shipped it. Plan does not write intent; the ruling returns
  through the spec delta.
  **The objection to settle first:** glob-set containment is not decidable in
  general (`src/**` vs `src/**/*.ts`), so the buildable spelling is a *literal*
  superset — every glob string in `rule.paths` appears verbatim in
  `skill.paths`. That is decidable, and exact for the union-authoring pattern
  that motivates it, but it false-fires on a semantically-contained narrower
  glob, which invariant 2 ("a gate that cries wolf gets disabled") aims at. So
  the decision owes: the predicate's literal bound stated as its declared
  leniency, and the shipped severity — advisory reads as the honest entry
  (invariant 5), error only as the corpus's declared act.
  **Cost evidence for the gated-vs-ungated side:** gate-opening is *loud*. Same
  probe, transcript-verified: reading a gate-matching file injects two
  attachments in one turn — the path-scoped rule as `nested_memory`, and a
  `skill_listing` with `isInitial: false` carrying only the newly ungated
  skills, name plus full description. An ungated skill's description is day-one
  scenery; a gated skill's arrives as an event at the moment of relevance — so
  the remaining costs of gating reduce to the cold-question hole and this
  containment invariant. (Also observed: the rendered rule strips the managed-by
  frontmatter note, `contentDiffersFromDisk: true` — the provenance marker costs
  the agent zero tokens at fire time.)

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
  `skill_listing` herald, `paths`-gate channel semantics — both feeding
  `(mention-gate-containment)` above) were unknowable from structure and cost
  hand-built headless probes (transcript-verified, 2.1.210). So the fork's
  cost side is now measured, not assumed. `Requirement.verifiedBy` is already
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
  TEMPLATE-FILE-CHILD-FACT shipped that fact (794678f); the built-in adoption that lets
  the factory delete against `skill` + `supporting-doc` waits on
  `(nested-file-child)` above. When both land, the factory deletes against the
  built-in and this record's condition is what a future pack argument must
  clear.

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
  drained and the physical trees were deleted (`chore(harness)` 68f187d). Two
  standing debts survive, both accepted, both riding the next entry that
  **reconciles** their file (never merely opens it — the precedent below),
  never a standalone entry:
  (1) `tests/session_start.rs:128/133/146` still writes `+++`-format
  `.temper/kinds/spec/KIND.md` + `.temper/packages/spec/PACKAGE.md` fixtures —
  live test code asserting stray old-format files are ignored. Two entries
  (664a522, CHECK-ARG-HALF-GATE 4256274) have opened the file and left them.
  (2) `sdk/src/builtins.ts:565/611` doc-comment-cite two deleted
  `packages/{rule,memory}.anthropic/PACKAGE.md` files — untouched since
  706139a (2026-07-07). Nine entries have now opened builtins.ts and left both
  as unchanged context; two sibling cites discharged along the way, each by
  deletion of its host rather than by reconciliation (`skill.anthropic` cut by
  dfba26f, `memory.agents-md` by AGENTS-MD-STDLIB-DROP 955be32 deleting the
  whole `memoryAgentsMdDefaultContract` block).
  Both re-verified on disk at reconcile HEAD d97a704, unmoved; neither file was
  touched in the cac023a..d97a704 window. **No queued entry opens either file**
  — cc5a9b3's split routed `skill`'s nesting template to the built-in
  adoption, which waits on `(nested-file-child)`.

- **Rust engine narration cites lag the SDK clause re-fetch.**
  BUILTINS-CITE-REFRESH (c4b060d) re-fetched every Claude Code source live
  2026-07-15 and bumped the SDK clause cites plus doc-comment dates to match;
  the engine's own reader-side narration cites mirror the same facts at their
  older retrieval dates — `src/builtin_kind.rs` (85 @07-07; 194/222 @07-10;
  63/106 undated skills/sub-agents mentions), `src/extract.rs:774` (@07-02),
  `src/graph.rs` (61/689 @07-02). The build entry's `per` targeted the SDK
  clause-enforcement point (`specs/builtins.md`, "The clauses live in code"),
  not this parallel surface, so it flagged them for routing rather than
  silently bumping. Every fact still holds — the two that moved (memory's
  `./.claude/CLAUDE.md` equal-project location, the Codex AGENTS.md URL
  redirect) are SDK-only cites, absent from these Rust files, and
  builtin_kind.rs's `**/CLAUDE.md` glob already covers the new location — so
  this is date-staleness on correct facts: citation staleness, riding
  whichever entry next opens each file, never standalone, never the queue's
  only new work. Found at reconcile HEAD 794ca2b. The `coverage_note.rs:76`
  rider is **discharged**: WORKSPACE-DIR-ONE-HOME (23c31c4) carried it —
  `SETTINGS_DOC` re-verified live against code.claude.com/docs/en/settings (it
  still documents every surface cited off it) and the retrieval date bumped
  @07-02 → @07-16, verified on disk at coverage_note.rs:70-72. The rest
  (`builtin_kind.rs`, `extract.rs:774`, `graph.rs`) still ride the next entry
  opening each; no queued entry opens any.

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
  495/633/770/1172. Re-verified on disk at reconcile HEAD ff7da32.

- **`src/extract.rs`'s floor-mention deferral comment is resolved-to-never.**
  The `EmbeddedMember` doc (extract.rs:196-198) still says floor-leaf
  interpolation "stays deferred until a floor mention syntax is separately
  ratified" — 0020 ratified the opposite: a declaration types a position,
  never a pattern within prose, so no floor mention syntax will ever exist
  and the plain `String` leaf is the permanent shape, not a middle. Behavior
  is correct; only the comment names a replacement that will never come.
  Rides whichever entry next opens `src/extract.rs` (0020's own exit
  clause), never standalone. Found routing 0020 at HEAD a0fccaf. 3611335,
  then MCP-SERVER-KIND (1ffab8f, hunks at 913/938/1148/1167), then HOOK-SHAPE
  (5fc3e9f, +135 lines: the `hook_matcher_group` write/read pair at 928-1015),
  then SATISFIES-LABEL-QUALIFY (3d08a4a, the `host_address`
  `pub(crate)`→`pub` widen at line 640, net-zero), each opened extract.rs
  but left 196-198 as unchanged context, so — reconciliation-not-opening —
  undischarged; re-verified on disk (extract.rs:196-198, unshifted) at
  reconcile HEAD fd0ba24.

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
  re-verified on disk at reconcile HEAD d97a704 (unmoved; `prose.ts` untouched
  in this window). The `sdk/src/kind.ts:257` "posture 3" half of this record is
  **discharged**: TEMPLATE-FILE-CHILD-FACT (794678f) carried it — 0025 made
  "posture" a consumer-declared member type, not a body-authoring mode number,
  and the cite is gone from the file. (Fixture body text inside tests is a
  separate class, excluded — `src/kind.rs`'s `15-kinds.md` strings and
  `src/extract.rs`'s two `"…law 5"` decision-fixture strings are `.to_string()`
  test data, not cites.)

- **`sdk/test/emit.test.ts:980` cites the retired `renderMemberFence`.**
  EMBED-RENDER-FENCE-FREE (f2d73da) renamed `renderMemberFence` →
  `renderMemberBlock` (an embedded format is writer-only, the fence cosmetic
  — `specs/model/representation.md`, "kind") and opened this test file, but
  left the test comment ("untouched — `renderMemberFence`") naming the gone
  symbol. Behavior correct; comment staleness only. Seven entries have now
  opened the file and left the comment as unchanged context — the rider
  discharges on *reconciliation*, never on the file being opened; the last,
  TEMPLATE-FILE-CHILD-FACT (794678f), shifted it 937 → 980, re-verified on
  disk at reconcile HEAD d97a704. Rides EMBEDDED-FORMAT-TARGET-FACTS, the one
  queued entry reconciling this file's comments; never standalone. Found at
  reconcile HEAD 99a79ec.

- **Cargo.toml's schemars dep comment is doubly stale.** It cites
  `src/schema/interchange.rs` (the module is `src/schema.rs`; no `schema/`
  dir exists) and assigns ts-rs the interchange-TS role the seam bindings
  superseded (ts-rs's live job is the `sdk/src/generated/` seam home,
  36a7662; `src/schema.rs` is schemars-only). Comment staleness — rides
  whichever entry next opens `Cargo.toml`, never a standalone entry. Found
  at residue sweep HEAD a932bb0; re-verified on disk (lines 42-43) at sweep
  HEAD c5df845.

- **4144b20's retirement of `compose::effective` left two one-line
  comment stragglers in files that commit itself opened.**
  `src/compose.rs:233` ("Unlike `effective`, …") cites the retired symbol
  by name inside `default_contract_from_rows`'s test doc comment;
  `src/contract.rs:475` ("when `target` (above) names a field for layering
  purposes") keeps the retired severity-flip layer's vocabulary one hunk
  from the corrected `Predicate::target` doc — the layer is gone, so
  `target`'s live job is array-surgery identity, never layering. Behavior
  and symbols correct; doc-comment staleness only. Each rides whichever
  entry next opens its file — FORMAT-OMITS-EDGE-CLAUSE now opens
  `src/contract.rs` (the new predicate variant); no queued entry opens
  `compose.rs` — never standalone. Found at residue sweep HEAD d029d4b. GLOB-VALIDITY-PREDICATE
  (46b8cd1) then opened `src/contract.rs` (+20: the glob-validity predicate)
  yet left the straggler as unchanged context — undischarged; it shifted +16,
  459 → 475. compose.rs:233 untouched this window. Re-verified on disk
  (compose.rs:233, contract.rs:475) at reconcile HEAD c0bbf3b.

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
