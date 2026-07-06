# Open questions

Product/architecture forks not yet settled. Each is keyed with a `(slug)` so a
pending entry can declare `dependsOnForks: ["slug"]` and be held until resolved.

**Lifecycle (the anti-accumulation rule, John 07-06): this file holds OPEN
forks only.** Resolution = encode the ruling (corpus Decision, or the resolving
commit body) and **delete the record** — git history is the archive; "kept as
the decision record" is retired as a category. Reconciliation evidence (DATUMs)
goes in the plan commit body, never appended here. Rationale: this file is
inlined whole into every plan prompt — every dead line is a per-tick context
tax (it hit 1,280 lines / ~38k tokens before the 07-06 drain).

## Open forks

- `(authority-home)` — OPEN (registered 2026-07-06, John's inbox routing). The
  SDK's `compileDeclarations` emits `{ fact: "authority", value: "shared" }`
  unconditionally (`sdk/src/declarations.ts:151`), but "shared" is
  corpus-uncoined: the surface-authority vocabulary is note/warn/block
  (`20-surface.md`) and `Harness` carries no authority field to source a
  posture from. Where does the authored posture live in the four-field
  assembly? Candidates: a `settings` residual entry, a per-projection posture
  on the member whose path is emit-owned, or a harness-level default with
  per-path overrides. Needs John. Load-bearing the moment `temper guard`
  blocks per posture; nothing gates today.

- `(hook-kind-locus)` — OPEN. Hooks have no standalone file surface — they are
  JSON entries inside `.claude/settings.json`, so no path locus selects one.
  Is a hook a sub-member extracted from the settings member, a distinct locus,
  or a facet the settings floor checks via clauses over the `hooks` key?
  Needs John. Held until the locus model is decided; related:
  `(json-projection-format)`.

- `(json-projection-format)` — OPEN, foundation-gated. The JSON-manifest
  built-in kinds (settings, MCP, plugin/marketplace) need a generic JSON
  adapter (a peer to `src/frontmatter.rs`) reading nested-key fields into the
  generic extraction path; a kind's on-disk shape is the `layout` fact
  (`15-kinds.md`), declared as an SDK value. Real engine work, gated on the
  SDK-primary foundation (the derived-lock chain), not on a vocabulary change.

- `(edge-representation-unify)` — OPEN residual. The gate's graph reads
  `routes_to` as an extracted feature (verified 07-06: live in `src/graph.rs`),
  NOT as the flattened projection of a requirement/satisfies join — so a
  surface-authored join still reaches no graph edge. `[edge.*]`/`EdgeClause`
  is retired (shipped). What remains: wiring join→`routes_to` flattening (the
  emit face derives the one-way pointer; a pointer with no join behind it is
  drift). Spec sanctions it (`45-governance.md`, coupling-is-a-join Decision);
  awaits a human decision to file.

- `(builtin-workspace-qualified-key)` — OPEN, closes with the derived-lock
  chain (D3). `check::Workspace` keys built-ins by bare kind name and
  hardcodes `skills()`/`rules()` accessors (verified 07-06, `src/check.rs:97`);
  two same-bare-name providers collide under one map key. The derived-lock
  rebuild makes the kind map row-driven, which is this fork's fix — do not
  file separately.

- `(local-overrides)` — OPEN. The committed-plus-gitignored personal-override
  layer has no stated spelling in the one-value assembly model. Candidates: a
  local harness module composed by convention, or an engine-side severity
  overlay. Blocks nothing until someone needs a personal override.

- `(genre-fence-format)` — HOLD (resolved-deferred, John 07-04): the fence
  grammar is designed by its first consumer. The consumer landed — cascade
  volunteered as the genre-adoption pilot (07-06); the workshop (John +
  session) designs the grammar against 3–4 real cascade Decision fixtures;
  acceptance is the byte-stable posture-2 ⇄ posture-3 round-trip. Not a
  pending entry until the workshop rules the grammar.

- `(eval-capability)` — OPEN, strategic, parked past launch. Harness evals:
  every requirement carries `means` and every `satisfies` a rationale, so the
  graph gives eval selection for free (blast radius → which evals re-run). If
  ever built: a `verifiedBy` verifier type and/or tier 2 made concrete —
  probabilistic, NEVER tier 1 or the hard gate (law 3). Do not let it near
  the launch wedge.

- `(multi-harness-projection)` — OPEN, strategic. One member projecting to N
  harnesses (`.claude/rules/` and `.cursor/rules/` from one document) —
  rulesync's portability as an architecture side effect. Open questions:
  per-harness capability mismatch, which harness is authoritative, whether a
  lossy projection is a verdict or an error. The read face of foreign formats
  is decided (`50-distribution.md`, migrate-with-a-fix); only the write face
  is open. No dependents.

## Kept on purpose — deliberate asymmetries (re-read every tick)

Every asymmetry below is a **choice with a condition**, not a fact. When its
condition arrives, it is the next break. If work touches one, surface it.

- **Floor auto-adoption** (a bare harness gets built-in kinds checked with no
  assembly declaration) — kept for the zero-config front door; post
  derived-lock it reads as "the built-in lock's contents", data not code.
- **Format implementations are engine code** (the generic frontmatter adapter)
  — kept because an external format's mechanics are temper's to implement
  once; the selection is declared. Grows only by deliberate addition.
- **`kinds/` + `packages/` are curated, fence-excluded** — condition arrived,
  retirement filed: BUILTIN-LOCK-DERIVED + CURATED-TREES-RETIRE remove the
  code refs (citation trail moves to clause `cite` fields per
  `10-contracts.md`); the physical tree deletion is a human `chore(harness):`
  commit after (the trees are out of build's fence). This line dies on the
  reconcile that sees both land.
- **`.flume/` is ungoverned by temper** — the machine that builds temper is
  not yet under its gate; a candidate landscape once the custom-kind story
  proves end to end.
- **`docs/` is candidate intent, not intent** — human territory,
  fence-excluded; plan never reads a horizon entry as intent.
