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

- `(manifest-authoring-surface)` — OPEN (registered 2026-07-07). 0015 rules
  structured config files (`settings.json`, `.mcp.json`, a plugin's manifests)
  **manifests**: projections of a container member's controlled segment —
  fields, its members' registration facts, and derived aggregates (the
  permission union — already `sdk/src/needs.ts` `permissionUnion`, never a
  member) (`specs/model/representation.md`, "Reach"; `specs/builtins.md`, "The
  named expansion"). Settled *semantics*: a registration member is a fields-only
  kind (no prose, no artifact, no lock rows — 0012) at a **collection address**
  (`mcpServers.*`, `hooks.<Event>` — a kind fact, the manifest's fence);
  read-unrepresented infers members, represented regenerates whole (declared
  order then residue, LF), unrepresented write stays 0008's splice
  (`src/json_splice.rs`); a plugin is both faces (`temper bundle` the bespoke
  instance the general write subsumes); levels are peer forests, temper governs
  the project one. OPEN is the *spelling*: none of the machinery exists — no
  JSON manifest adapter beside `src/frontmatter.rs`, no fields-only kind shape
  beside `Format`/`UnitShape` (`src/kind.rs`), no collection-address kind fact,
  no container-segment emit projection, no SDK API for a hook/mcp-server/plugin
  member. John rules the authoring surface + emit/write architecture before
  the hook/mcp-server/plugin kinds and the canonical-manifest write file as
  build entries (0014's fetch-and-cite pattern per kind, once unblocked).
  On resolution, `src/bundle.rs`'s bespoke serde_json manifest writes
  (~lines 158-291: `plugin.json`, `marketplace.json`, `hooks.json`) convert
  to general-write instances — 0015's named consequence; correct as
  hand-builds only until then (inbox note, 07-09).

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
  `706139a` (2026-07-07). Debt (2)'s riding prediction falsified at ship
  audit HEAD b8f0746: f36c192 (SKILL-CONTRACT-RECITE) opened builtins.ts —
  its cite-restamp hunks bracket all three lines — but left every one as
  unchanged context; all three cites verified still on disk, so the exit
  clause did not fire and the debt rides whichever entry opens the file
  next. Debt (1) untouched (no commit since 5f88258 opened
  tests/session_start.rs). Both re-verified on disk at residue sweep HEAD
  08550e5 (session_start.rs `+++` fixtures at lines 128/133/146; the three
  builtins.ts cites at 308/348/385 — neither file touched since fcdbe52;
  b5b6fb4 touched neither). Verify both at the next residue sweep.

- **Pre-corpus-reorg spec-path citations in `sdk/src/kind.ts` — 8 hits**
  (lines 7,57,86,98,125,166,189,206; re-verified against disk at ship audit
  HEAD b8f0746). Doc comments cite `10-contracts.md`, `15-kinds.md`,
  `20-surface.md`, `40-composition.md` — aspect-oriented files 0002's reorg
  retired; none exist in the corpus today
  (`specs/process/engineering.md` residue class). Compounding:
  `.claude/rules/sdk.md` retires spec-path citations from comments as a
  class regardless of validity — cut on contact, never annotate — so this
  rides the next entry that opens the file, never a standalone one. Three
  riding predictions falsified already (18d3406, 3c6f50b, cb17438 each
  opened `kind.ts` without their scope reaching the citation lines).
  `contract.ts`'s twin 12-hit debt cleared at ship audit HEAD b8f0746
  (36e0556's exit clause fired; evidence in that plan commit body).
  KIND-CONTENT-FACT (filed 07-09, 0019 derivation) opens `kind.ts` and
  names the 8-line cut in its file description — verify the exit clause
  fired at its ship audit, then delete this bullet.

- **Two `src/kind.rs` doc comments are factually false**, both claiming
  inertness a consumer disproves: `unit_shape`'s "Inert alongside
  format" (line ~74 — consumed by `src/frontmatter.rs:175`) and
  `registration`'s "nothing else consumes it yet" (line 82 — consumed by
  `src/main.rs:452-455` → `graph.rs::live_members` since 207e701).
  Comment staleness rides whichever entry next opens `src/kind.rs`
  (rust.md, "the exit clause") — never a standalone entry. From the 0013
  inbox note (observed 9c3b1c1); both verified on disk at f600965.
  KIND-CONTENT-FACT (filed 07-09, 0019 derivation) opens `src/kind.rs`
  and names both fixes in its file description — verify the exit clause
  fired at its ship audit, then delete this bullet.

- **Pre-0019 "every governed artifact" universal in `src/install.rs:18`.**
  The module doc still states the first emit "regenerates every governed
  artifact as a canonical projection" — 0019 recut the universal: every
  *composed kind's* artifact is a projection, a layout kind's document a
  source at either depth (`specs/model/pipeline.md`, "Install"). Comment
  staleness only — no behavior can diverge until a layout kind ships, and
  LAYOUT-READER lands the behavior. Rides the exit clause (rust.md), never
  standalone: LAYOUT-READER (filed 07-09) opens `src/install.rs` and names
  the line-18 fix in its file description — verify at its ship audit, then
  delete this bullet. The `tests/install.rs:15,382` half cleared at ship
  audit HEAD ddae7d4 (1589845's exit clause fired; evidence in that plan
  commit body). Found at residue sweep HEAD ec3d112.

- **Cargo.toml's schemars dep comment is doubly stale.** It cites
  `src/schema/interchange.rs` (the module is `src/schema.rs`; no `schema/`
  dir exists) and assigns ts-rs the interchange-TS role the seam bindings
  superseded (ts-rs's live job is the `sdk/src/generated/` seam home,
  36a7662; `src/schema.rs` is schemars-only). Comment staleness — rides
  whichever entry next opens `Cargo.toml`, never a standalone entry. Found
  at residue sweep HEAD a932bb0.

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
