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

- `(layout-kind-heterogeneous-corpus)` — OPEN (registered 2026-07-15). A
  layout binds the whole kind (`specs/model/representation.md`, "kind": a
  layout is the kind's content), but a real corpus is heterogeneous — some
  documents in a governed tree mix prose and members, others don't.
  0019-content's own answer ("what does not fit the three primitives is two
  kinds, or it is prose") collides with governance: what **two kinds sharing a
  `governs` glob** means is unspecified (representation.md's "per-kind
  precedence" is the runtime artifact-level merge, not this). First question a
  consumer hits adopting a layout for a built-in kind, or governing any
  heterogeneous tree. Open: can two kinds share a governs glob (and how is a
  document routed to one), or is heterogeneity always "declare N kinds over N
  sub-globs"? (Related but distinct from `(directory-sliced-governance)`, the
  single-kind path-derivation gap.) Resolution routes back through the inbox.

- `(directory-sliced-governance)` — OPEN (registered 2026-07-15). A single
  non-nesting **file**-locus kind cannot govern a directory-sliced corpus:
  `member_projection_path` (`src/drift.rs:513`) derives a projection path by
  `governs_glob.replacen('*', name, 1)` — the glob's *first* `*` only — so a
  glob `*/*.md` yields `<name>/*.md`, a literal `*` left in the path. The
  workaround (one kind per directory, each with a `*`-terminal glob) works but
  proliferates kinds. The corpus (`specs/model/representation.md`, "kind")
  gives a kind one `governs` glob and a per-nested-layer path pattern, but says
  nothing of deriving a leaf file's path through a multi-segment glob. Open:
  does temper admit nested-glob path derivation, or a **declared path template**
  on the kind, or is directory-sliced governance always "declare N kinds over N
  `*`-terminal sub-globs"? (Live: pack-kind field trial, centercode 07-15;
  related but distinct from `(layout-kind-heterogeneous-corpus)`, which is about
  two kinds sharing one glob, not one kind spanning many directories.)
  Resolution routes back through the inbox.

- `(agents-md-builtin-kind)` — OPEN (registered 2026-07-06). The engine's
  hand-written std-lib ships an `agents-md.memory` built-in kind (glob
  `**/AGENTS.md`), but the SDK module and the derived built-in lock export only
  the `CLAUDE.md` `memory` kind (`specs/builtins.md`, "The shipped kinds":
  memory is a `CLAUDE.md`-family file; `AGENTS.md` at root sources `CLAUDE.md` —
  `specs/distribution.md`, "The offering"). The spec-faithful default drops the
  engine's agents-md.memory to match the lock. Open: should temper ship an
  AGENTS.md built-in kind — and if so under a **distinct label** (never a
  provider-qualified `memory`; identity travels by import, not string —
  `specs/builtins.md`)? A feature addition, not a chain blocker.

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

- `(multi-harness-projection)` — OPEN, strategic. One member projecting to N
  harnesses (`.claude/rules/` and `.cursor/rules/` from one document) —
  rulesync's portability as an architecture side effect (`specs/intent.md`,
  "Positioning"). The engine is corpus-generic (`specs/model/representation.md`,
  "Reach"), but the write face of foreign formats is open: per-harness
  capability mismatch, which harness is authoritative, whether a lossy
  projection is a verdict or an error. No dependents.

- `(impact-read-verb)` — OPEN (registered 2026-07-10). The model names two
  read verbs as peers: `explain` narrates, `impact` reports removal fallout
  (`specs/model/contract.md`, "Read verbs"; `specs/model/pipeline.md`, "Read
  verbs"). The shipped surface unified them: `src/main.rs`'s clap `Command`
  has one read variant, `Explain`, and `src/read.rs:190` calls it "the one
  read verb" — `impact` is an internal strand (`read::impact`) `explain`
  dispatches into, never a CLI verb (READ-EDGE-UNIFY). CLAUDE.md's own
  identity enumerates seven shipped verbs with no `impact`, re-cut by a human
  at 827b2f2 *after* the model text — evidence the unified surface is current
  intent and the model's peer-verb spelling is the stale side. This session's
  position: the fix is a `specs:` amendment (respell contract.md/pipeline.md
  "Read verbs" so `impact`/`why`/`context` read as strands of the one
  `explain` verb, not peers), not a code entry that re-splits a deliberate
  unification. But amending the model is a kernel-section change John owns
  (`specs/process/spec-system.md`, "Change ceremony"), and it collides with
  standing corpus text — surfaced, not encoded either way. Resolution routes
  back through the inbox: amend the model, or rule `impact` ships as a
  distinct verb (then a pending entry, `per` contract.md "Read verbs").

## Kept on purpose — deliberate asymmetries (re-read every tick)

Every asymmetry below is a **choice with a condition**, not a fact. When its
condition arrives, it is the next break. If work touches one, surface it.

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
  `sdk/src/builtins.ts:392,432,469` still doc-comment-cites three deleted
  `packages/{rule,memory}.anthropic|memory.agents-md/PACKAGE.md` files (a
  fourth, `skill.anthropic`, was already cut by `dfba26f`) — untouched since
  `706139a` (2026-07-07). NB the exit clause fires on *reconciliation*, not
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
  they shifted +152, 406/446/483 → 558/598/635.
  GLOB-VALIDITY-PREDICATE (46b8cd1) then opened builtins.ts an eighth time
  (+13 lines: the `paths` glob-validity clause, above the cites) and once
  more left all three as unchanged context — undischarged; they shifted +7,
  558/598/635 → 565/611/648. And CHECK-ARG-HALF-GATE (4256274) opened
  `session_start.rs` (+51: the install-wired-command test) yet left the `+++`
  fixtures as unchanged context — undischarged, unmoved at 128/133/146.
  Re-verified on disk at reconcile HEAD c0bbf3b (builtins.ts cites at
  565/611/648; session_start.rs `+++` fixtures still 128/133/146). Now rides
  the next entry opening builtins.ts — no queued entry does — unless it
  reconciles the cites.

- **Rust engine narration cites lag the SDK clause re-fetch.**
  BUILTINS-CITE-REFRESH (c4b060d) re-fetched every Claude Code source live
  2026-07-15 and bumped the SDK clause cites plus doc-comment dates to match;
  the engine's own reader-side narration cites mirror the same facts at their
  older retrieval dates — `src/builtin_kind.rs` (85 @07-07; 194/222 @07-10;
  63/106 undated skills/sub-agents mentions), `src/coverage_note.rs:76`
  (`SETTINGS_DOC`, @07-02), `src/extract.rs:774` (@07-02), `src/graph.rs`
  (61/689 @07-02). The build entry's `per` targeted the SDK
  clause-enforcement point (`specs/builtins.md`, "The clauses live in code"),
  not this parallel surface, so it flagged them for routing rather than
  silently bumping. Every fact still holds — the two that moved (memory's
  `./.claude/CLAUDE.md` equal-project location, the Codex AGENTS.md URL
  redirect) are SDK-only cites, absent from these Rust files, and
  builtin_kind.rs's `**/CLAUDE.md` glob already covers the new location — so
  this is date-staleness on correct facts: citation staleness, riding
  whichever entry next opens each file, never standalone, never the queue's
  only new work. Found at reconcile HEAD 794ca2b.

- **Pre-0019 "layout" fact name in `sdk/src/kind.ts`.** The module doc
  (line 4) and the fact-3 doc comments (lines 16/106/108 — "fact 3, layout"
  = `Format` + `UnitShape`, the projection shape) still spell fact 3
  "layout" — vocabulary now colliding, in the same file, with the
  sanctioned `Layout` content type a538a76 exported (0019: a layout is the
  declared content template — `specs/model/representation.md`, "kind"; one
  name per concept, `specs/process/spec-system.md`). Doc-comment staleness
  only — the symbols themselves (`Format`, `UnitShape`, `Layout`) are
  correctly named. Rides whichever entry next opens `sdk/src/kind.ts` (no
  queued entry does), never standalone; the fix renames the *fact
  narration*, never the sanctioned type. Found at residue sweep HEAD
  e9d05f6. MANIFEST-KIND-MODEL (cd1ca29) opened all three regions to add
  the `Fields`/`registration` content shape — writing module-doc line 4
  *fresh* in the retired "layout" vocabulary (self-propagation, again) —
  yet left the fact-3 narration, so per the reconciliation-not-opening
  precedent the rider is undischarged. MANIFEST-WRITE-SDK-ERASURE (8cc0561)
  then opened `sdk/src/kind.ts` again (carrying fields-only typed fields)
  and once more left the fact-3 narration; re-verified on disk (lines
  4/16/106/108) at reconcile HEAD f075f8d.

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
  each opened extract.rs but left 196-198 as unchanged context, so —
  reconciliation-not-opening — undischarged; re-verified on disk
  (extract.rs:196-198) at reconcile HEAD 224b880.

- **Pre-recut vocabulary survives in prose-layer doc comments.** 0001's
  retirement map (law → invariant/spine rule, posture → retired, decisions
  renamed `NNNN-*.md`) still narrates `sdk/src/prose.ts` ("law 5" at
  6/93/210, "law 8" at 11, "posture N" at 78/108/113/140/190, pre-recut
  decision cites `` `15-kinds.md` ``:78 and `` `20-surface.md` ``:152 —
  neither file exists) and `sdk/src/kind.ts:254` ("posture 3"). Doc-comment
  staleness only —
  behavior and symbols correct; note a8562b5 wrote prose.ts line 10 *fresh*
  in the retired vocabulary, so the narration self-propagates by imitation
  until scrubbed. Rides whichever entry next opens each file (no queued
  entry opens any), never standalone. (Fixture body text inside tests —
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

- **`sdk/test/emit.test.ts:853` cites the retired `renderMemberFence`.**
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
  disk at reconcile HEAD 9223917.

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
  entry next opens its file (no queued entry opens either), never
  standalone. Found at residue sweep HEAD d029d4b. GLOB-VALIDITY-PREDICATE
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
