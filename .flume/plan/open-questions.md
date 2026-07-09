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
  `706139a` (2026-07-07). Both re-verified live at residue sweep HEAD
  37c2411 — the one intervening src/tests/sdk change since a561e70
  (a620938, retyping `Requirement.kind` to `KindDefinition<never>`:
  `sdk/src/contract.ts`, `sdk/test/refusals.test.ts` only) never touched
  the two debt files. Verify both at the next residue sweep.

- **`overlay_builtin_kind` rename (e5daf1d) left one stale comment.**
  `tests/coverage.rs:336-338`'s doc comment on
  `a_kind_row_relocating_a_built_ins_governs_fires_no_collision_diagnostic`
  still names the pre-rename symbol `effective_governs` — e5daf1d
  (BUILTIN-KIND-TEMPLATES-OVERLAY) renamed it to `overlay_builtin_kind` in
  src/main.rs but didn't reach this test file. Comment staleness rides the
  next entry that opens `tests/coverage.rs` (`.claude/rules/rust.md`, "the
  exit clause") — never a standalone entry. Found at residue sweep HEAD
  e6d0311; re-verified still true at HEAD 37c2411 (a620938, the sole
  intervening src/tests/sdk change since a561e70, never touched
  tests/coverage.rs).

- **Pre-corpus-reorg spec-path citations in the SDK — `sdk/src/kind.ts`
  (8 hits) and `sdk/src/contract.ts` (12 hits).** Doc comments cite
  `10-contracts.md`, `15-kinds.md`, `20-surface.md`, `40-composition.md` —
  numbered, aspect-oriented files from before 0002's reorg
  (`specs/decisions/0002-corpus-form.md`, "the prior form... aspect-oriented
  files scattered each noun across five homes"; `specs/model/*.md` replaced
  them). None of the four names exist in the corpus today — vocabulary the
  corpus no longer sanctions (`specs/process/engineering.md` residue class).
  Compounding: `.claude/rules/sdk.md`/`rust.md` ("Style & structure") retire
  spec-path citations from comments as a class regardless of validity — cut
  on contact, never annotate (rust.md, "the exit clause") — so this rides
  the next entry that opens either file rather than a standalone one.
  Prediction falsified at ship audit HEAD 6d6ae89: EMBEDDED-LEAF-TEXT
  (18d3406) did open `kind.ts` (import line, both `EmbeddedMemberValue`/
  `EmbeddedMemberCollectionEntry.leaves` doc comments) but its cited scope
  never reached the 8 citation lines (7,57,86,98,125,164,187,204 — hit-count
  and lines re-verified against disk this tick), so the exit clause did not
  fire; the debt still rides whichever entry opens `kind.ts` next.
  `contract.ts`'s prediction also falsified, same shape, at ship audit HEAD
  a641e03: a620938 (REQUIREMENT-KIND-SDK-TYPE, the sole src/tests/sdk commit
  since 6d6ae89) opened `contract.ts`, but its one-line edit (the
  `Requirement.kind` field's type annotation, line 154) never reached any of
  the 12 citation lines (5,7,17,34,38,48,57,89,124,142,149,160 — re-verified
  against disk this tick), so the exit clause did not fire there either; the
  debt still rides whichever entry opens `contract.ts` next. Original
  hit-counts were transposed between the two files in the prior note;
  corrected here against both current disk and `git show 3c6f50b`. Found at
  residue sweep HEAD 3c6f50b (introduced pre-52b3dcd; 3c6f50b's own diff
  edited two of kind.ts's other doc comments without reaching these).

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
