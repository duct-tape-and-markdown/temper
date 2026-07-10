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
  `sdk/src/builtins.ts:308,348,385` still doc-comment-cites three deleted
  `packages/{rule,memory}.anthropic|memory.agents-md/PACKAGE.md` files (a
  fourth, `skill.anthropic`, was already cut by `dfba26f`) — untouched since
  `706139a` (2026-07-07). NB the exit clause fires on *reconciliation*, not
  on the file being opened: f36c192 opened builtins.ts and left all three
  cites as unchanged context (verified at ship audit b8f0746). Both
  re-verified on disk at residue sweep HEAD 3d13eb4 (session_start.rs `+++`
  fixtures at lines 128/133/146; the three builtins.ts cites at 308/348/385
  — the window's one code commit, cd1ca29, the fields-only-kind shape,
  touched neither file). Verify both at the next residue sweep.

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
  precedent the rider is undischarged; re-verified on disk (lines
  4/16/106/108) at reconcile HEAD 3d13eb4.

- **`src/extract.rs`'s floor-mention deferral comment is resolved-to-never.**
  The `EmbeddedMember` doc (extract.rs:196-198) still says floor-leaf
  interpolation "stays deferred until a floor mention syntax is separately
  ratified" — 0020 ratified the opposite: a declaration types a position,
  never a pattern within prose, so no floor mention syntax will ever exist
  and the plain `String` leaf is the permanent shape, not a middle. Behavior
  is correct; only the comment names a replacement that will never come.
  Rides whichever entry next opens `src/extract.rs` (0020's own exit
  clause), never standalone. Found routing 0020 at HEAD a0fccaf;
  re-verified on disk (extract.rs:196-198) at sweep HEAD 3d13eb4.

- **Pre-recut vocabulary survives in prose-layer doc comments.** 0001's
  retirement map (law → invariant/spine rule, posture → retired, decisions
  renamed `NNNN-*.md`) still narrates `sdk/src/prose.ts` ("law 5" at
  5/83/184, "law 8" at 10, "posture N" at 68/98/100/127/176, pre-recut
  decision cites `` `15-kinds.md` ``:68 and `` `20-surface.md` ``:139 —
  neither file exists), `sdk/src/kind.ts:252` ("posture 3"), and
  `src/extract.rs:1153/1188` ("law 5"). Doc-comment staleness only —
  behavior and symbols correct; note a8562b5 wrote prose.ts line 10 *fresh*
  in the retired vocabulary, so the narration self-propagates by imitation
  until scrubbed. Rides whichever entry next opens each file (no queued
  entry opens any), never standalone. (`src/kind.rs:1079`'s `15-kinds.md`
  is fixture body text inside a test, not a cite — excluded.) Found at
  residue sweep HEAD c2a8cae. MANIFEST-KIND-MODEL (cd1ca29) opened
  `sdk/src/kind.ts` and shifted its "posture 3" line 225→252 while leaving
  the narration; `prose.ts`/`extract.rs` untouched in the window. Re-verified
  on disk (all lines) at reconcile HEAD 3d13eb4. PROSE-SENTINEL-ESCAPE respelled the two slot sentinels as
  unicode escape sequences (050ef2b), so prose.ts is now NUL-free — grep
  reads it as text without `-a`, and the sweep-mechanics NB retired with
  it. That entry opened prose.ts (lines 56/64) yet left these doc comments
  as unchanged context, so — per the reconciliation-not-opening precedent
  above — the rider is undischarged, still riding whichever entry next
  reconciles the comment lines.

- **Cargo.toml's schemars dep comment is doubly stale.** It cites
  `src/schema/interchange.rs` (the module is `src/schema.rs`; no `schema/`
  dir exists) and assigns ts-rs the interchange-TS role the seam bindings
  superseded (ts-rs's live job is the `sdk/src/generated/` seam home,
  36a7662; `src/schema.rs` is schemars-only). Comment staleness — rides
  whichever entry next opens `Cargo.toml`, never a standalone entry. Found
  at residue sweep HEAD a932bb0; re-verified on disk (lines 42-43) at sweep
  HEAD 3d13eb4.

- **4144b20's retirement of `compose::effective` left two one-line
  comment stragglers in files that commit itself opened.**
  `src/compose.rs:233` ("Unlike `effective`, …") cites the retired symbol
  by name inside `default_contract_from_rows`'s test doc comment;
  `src/contract.rs:459` ("when `target` (above) names a field for layering
  purposes") keeps the retired severity-flip layer's vocabulary one hunk
  from the corrected `Predicate::target` doc — the layer is gone, so
  `target`'s live job is array-surgery identity, never layering. Behavior
  and symbols correct; doc-comment staleness only. Each rides whichever
  entry next opens its file (no queued entry opens either), never
  standalone. Found at residue sweep HEAD d029d4b; re-verified on disk
  (compose.rs:233, contract.rs:459) at sweep HEAD 3d13eb4.

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
