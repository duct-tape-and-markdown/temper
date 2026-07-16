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
  collisions, each verified on disk at cc5a9b3:
  (1) the SDK's nesting surface is embedded-only — `EmbeddedMemberValue` /
  `NestedMemberRow` (`sdk/src/declarations.ts:425`) carry a host's *body*
  children; no surface exists for a skill to compose a child that owns its
  own file;
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
  **Nothing waits that need not:** TEMPLATE-FILE-CHILD-FACT is unblocked and
  ships the template's *declared fact* — child kind plus path pattern — which
  is what representation.md states whatever resolves the child. The built-in
  adoption returns as an entry once this fork rules, through the inbox.

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
  TEMPLATE-FILE-CHILD-FACT ships that fact; the built-in adoption that lets
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
  touches their file rather than a standalone one: (1) `tests/session_start.rs`
  still writes `+++`-format `.temper/kinds/spec/KIND.md` +
  `.temper/packages/spec/PACKAGE.md` fixtures — live test code asserting stray
  old-format files are ignored — `664a522` touched the file (retargeting two
  unrelated satisfies-fixture tests) without reconciling this one; (2)
  `sdk/src/builtins.ts:392,432,469` doc-comment-cited three deleted
  `packages/{rule,memory}.anthropic|memory.agents-md/PACKAGE.md` files (a
  fourth, `skill.anthropic`, was already cut by `dfba26f`; the third,
  `memory.agents-md`, was discharged by AGENTS-MD-STDLIB-DROP `955be32`
  deleting the whole `memoryAgentsMdDefaultContract` block that carried it —
  so **two** cites now survive, `rule.anthropic` + `memory.anthropic`) —
  untouched since `706139a` (2026-07-07). NB the exit clause fires on
  *reconciliation*, not
  on the file being opened: f36c192, HOOK-KIND (76aaa83), then MCP-SERVER-KIND
  (1ffab8f, +83 lines shifting the cites 344/384/421→392/432/469), then
  MANIFEST-WRITE-SDK-ERASURE (8cc0561) each opened builtins.ts and left all
  three cites as unchanged context — the predicted "SDK-ERASURE next opens
  builtins.ts and carries them again" came true (its hunks carried the
  fields-only typed fields, not the doc comments; cites unshifted at
  392/432/469). SKILL-PATHS-CHANNEL-GATE (2c26759) then opened builtins.ts
  a fifth time — adding the skill `paths` field's 12-line doc comment above
  the cites (builtins.ts:63) — and once more left all three as unchanged
  context, so per the reconciliation-not-opening precedent the rider is
  undischarged; the cites shifted +12, 392/432/469 → 404/444/481.
  BUILTINS-CITE-REFRESH (c4b060d) then opened builtins.ts a sixth time
  (108 lines: bumping every clause `cite` and doc-comment retrieval date to
  2026-07-15) and once more left all three `packages/…PACKAGE.md` cites as
  unchanged context — undischarged; they shifted +2, 404/444/481 → 406/446/483.
  GROWN-FIELD-SCHEMAS (e76934e) then opened builtins.ts a seventh time
  (+152 lines: the grown Skill/Agent typed-field doc comments, all above the
  cites) and once more left all three as unchanged context — undischarged;
  they shifted +152, 406/446/483 → 558/598/635. (Routing note, 07-16:
  SKILL-NESTED-REFERENCE-DOCS now opens builtins.ts — `skill`'s nesting
  template — so the two survivors ride it, discharged only if it reconciles
  them.)
  GLOB-VALIDITY-PREDICATE (46b8cd1) then opened builtins.ts an eighth time
  (+13 lines: the `paths` glob-validity clause, above the cites) and once
  more left all three as unchanged context — undischarged; they shifted +7,
  558/598/635 → 565/611/648. And CHECK-ARG-HALF-GATE (4256274) opened
  `session_start.rs` (+51: the install-wired-command test) yet left the `+++`
  fixtures as unchanged context — undischarged, unmoved at 128/133/146.
  Then AGENTS-MD-STDLIB-DROP (955be32) opened builtins.ts a ninth time and
  *deleted* the `memoryAgentsMdDefaultContract` block (line ~642, below both
  survivors) — discharging the third cite (`memory.agents-md`, was 648) by
  removing its host, while leaving the two survivors as unchanged context:
  the deletion sat below them, so `rule.anthropic`/`memory.anthropic` stay
  undischarged and *unshifted* at 565/611. Re-verified on disk at reconcile
  HEAD cac023a (builtins.ts survivors at 565/611; session_start.rs `+++`
  fixtures still 128/133/146 — neither file touched in the
  a2f4a1c..cac023a window). Both survivors ride the next entry opening
  builtins.ts — cc5a9b3's split routed `skill`'s nesting template to the
  built-in adoption, which waits on `(nested-file-child)`, so **no queued
  entry opens builtins.ts** today. The `+++` fixtures still ride the next
  entry opening session_start.rs; no queued entry does either.

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

- **Pre-0019 "layout" fact name in `sdk/src/kind.ts`.** The module doc
  (line 4) and the fact-3 doc comments (lines 16/109/111 — "fact 3, layout"
  = `Format` + `UnitShape`, the projection shape) still spell fact 3
  "layout" — vocabulary now colliding, in the same file, with the
  sanctioned `Layout` content type a538a76 exported (0019: a layout is the
  declared content template — `specs/model/representation.md`, "kind"; one
  name per concept, `specs/process/spec-system.md`). Doc-comment staleness
  only — the symbols themselves (`Format`, `UnitShape`, `Layout`) are
  correctly named. Rides whichever entry next opens `sdk/src/kind.ts` —
  TEMPLATE-FILE-CHILD-FACT is now that entry (`KindFacts` grows the
  nesting-template fact, right at the fact-3 narration), with
  EMBEDDED-FORMAT-TARGET-FACTS behind it — never standalone; the fix renames
  the *fact narration*, never the sanctioned type. Found at residue sweep
  HEAD e9d05f6. MANIFEST-KIND-MODEL (cd1ca29) opened all three regions to add
  the `Fields`/`registration` content shape — writing module-doc line 4
  *fresh* in the retired "layout" vocabulary (self-propagation, again) —
  yet left the fact-3 narration, so per the reconciliation-not-opening
  precedent the rider is undischarged. MANIFEST-WRITE-SDK-ERASURE (8cc0561)
  then opened `sdk/src/kind.ts` again (carrying fields-only typed fields)
  and once more left the fact-3 narration. COMPOSED-BODY-ADMISSION (0b2da21)
  then opened the file a third time and rewrote the `Locus` doc (49-59)
  *fresh* — the host-free admission narration — yet once more left the
  fact-3 narration as unchanged context: undischarged; lines 4/16 unmoved,
  106/108 shifted +3 → 109/111, re-verified on disk at reconcile HEAD
  cac023a.

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

- **Pre-recut vocabulary survives in prose-layer doc comments.** 0001's
  retirement map (law → invariant/spine rule, posture → retired, decisions
  renamed `NNNN-*.md`) still narrates `sdk/src/prose.ts` ("law 5" at
  6/141/258, "law 8" at 11, "posture N" at 126/156/161/188/238, pre-recut
  decision cites `` `15-kinds.md` ``:126 and `` `20-surface.md` ``:200 —
  neither file exists) and `sdk/src/kind.ts:257` ("posture 3"). Doc-comment
  staleness only —
  behavior and symbols correct; note a8562b5 wrote prose.ts line 10 *fresh*
  in the retired vocabulary, so the narration self-propagates by imitation
  until scrubbed. Rides whichever entry next opens each file —
  TEMPLATE-FILE-CHILD-FACT now opens `sdk/src/kind.ts`; no queued entry
  opens `prose.ts` — never standalone. (Fixture body text inside tests —
  not cites, excluded — is a separate class: `src/kind.rs`'s `15-kinds.md`
  strings, and `src/extract.rs`'s two `"…law 5"` decision-fixture strings,
  which 3611335 shifted 1153/1188→1227/1262, MCP-SERVER-KIND (1ffab8f)
  shifted 1227/1262→1223/1258, then HOOK-SHAPE (5fc3e9f) shifted
  1223/1258→1340/1375 — reclassified out of the doc-comment list
  above on finding them `.to_string()` test data.)
  Found at
  residue sweep HEAD c2a8cae. MANIFEST-WRITE-SDK-ERASURE (8cc0561) opened
  `sdk/src/kind.ts` and shifted its "posture 3" line 252→254 (+2) while
  leaving the narration; `prose.ts` untouched in this window. Re-verified on
  disk at reconcile HEAD f075f8d (`sdk/src/kind.ts:254` "posture 3";
  `sdk/src/prose.ts` unchanged). PROSE-SENTINEL-ESCAPE respelled the two slot sentinels as
  unicode escape sequences (050ef2b), so prose.ts is now NUL-free — grep
  reads it as text without `-a`, and the sweep-mechanics NB retired with
  it. That entry opened prose.ts (lines 56/64) yet left these doc comments
  as unchanged context, so — per the reconciliation-not-opening precedent
  above — the rider is undischarged, still riding whichever entry next
  reconciles the comment lines. PROSE-INTERLEAVE-SDK (6450ba6) then opened
  prose.ts to widen `blocks()`, and *rewrote* the two "posture 3" doc
  comments fresh (self-propagation again, this time the very lines the
  rider names) while leaving the rest as unchanged context — undischarged;
  prose.ts line numbers above re-derived on disk at reconcile HEAD d2496b6
  (`kind.ts:254` unchanged, that file untouched this window).
  MENTION-DISCOVERY-DEFER (ed5bb8e) then opened prose.ts a fourth time
  (+48 at the `Include` region, line 53) and once more left every narration
  line as unchanged context — undischarged; each shifted +48 below line 53,
  re-derived on disk at reconcile HEAD 5ef998b. COMPOSED-BODY-ADMISSION
  (0b2da21) then opened `sdk/src/kind.ts` (the host-free `Locus` at 49-59)
  and once more left the "posture 3" narration as unchanged context —
  undischarged; it shifted +3, 254 → 257, re-verified on disk at reconcile
  HEAD cac023a (`prose.ts` untouched in this window).

- **`sdk/test/emit.test.ts:937` cites the retired `renderMemberFence`.**
  EMBED-RENDER-FENCE-FREE (f2d73da) renamed `renderMemberFence` →
  `renderMemberBlock` (an embedded format is writer-only, the fence cosmetic
  — `specs/model/representation.md`, "kind") and opened this test file, but
  left the test comment at 851-854 ("untouched — `renderMemberFence`")
  naming the gone symbol. Behavior correct; comment staleness only — the
  symbol is now `renderMemberBlock`. Rides whichever entry next reconciles
  the comment (not merely opens the file), never standalone. Found at
  reconcile HEAD 99a79ec. PROSE-INTERLEAVE-SDK (6450ba6) then opened
  `sdk/test/emit.test.ts` to add a composed-body test below (line 907+) and
  left 853 as unchanged context — undischarged; re-verified on disk at 853,
  reconcile HEAD d2496b6. GROWN-FIELD-SCHEMAS (e76934e) then opened the file
  again (+53 lines: a typed-surface emit test) and once more left the comment
  as unchanged context — undischarged; it shifted 853 → 904, re-verified on
  disk at reconcile HEAD 9223917. SATISFIES-LABEL-QUALIFY (3d08a4a) then
  opened `sdk/test/emit.test.ts` a fifth time (the `rule:`-qualified
  satisfies-row assertions at 257-393, all net-zero, above the comment) and
  once more left 904 as unchanged context — undischarged, unshifted;
  re-verified on disk at 904, reconcile HEAD fd0ba24. MENTION-DISCOVERY-DEFER
  (ed5bb8e) then opened the file a sixth time (retargeting the
  unresolved-mention test at 579+, +3 above the comment) and once more left
  it as unchanged context — undischarged; it shifted 904 → 907, re-verified
  on disk at reconcile HEAD 5ef998b. COMPOSED-BODY-ADMISSION (0b2da21) then
  opened the file a seventh time — retargeting the `withinHosts` fixtures
  onto `admit`, the routing note predicted it would carry the comment — and
  once more left it as unchanged context: undischarged; it shifted 907 → 937,
  re-verified on disk at reconcile HEAD cac023a. Rides
  EMBEDDED-FORMAT-TARGET-FACTS, the one queued entry reconciling this file's
  comments; TEMPLATE-FILE-CHILD-FACT opens it first (the template-row
  assertions) and may shift the line without discharging it.

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
