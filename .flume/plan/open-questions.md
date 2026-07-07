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

- `(json-projection-format)` — OPEN. The JSON-manifest built-in kinds
  (settings, MCP, plugin/marketplace) need a generic JSON adapter (a peer to
  `src/frontmatter.rs`) reading nested-key fields into the generic extraction
  path; a kind's on-disk shape is the `layout` fact (`15-kinds.md`), declared
  as an SDK value. The SDK-primary foundation (the derived-lock chain) has now
  **shipped**, so this is unblocked engine work — but the adapter and the
  `layout`-fact spelling are an open design fork needing John before it can be
  filed as a pending entry.

- `(edge-representation-unify)` — OPEN residual. The gate's graph reads
  `routes_to` as an extracted feature (verified 07-06: live in `src/graph.rs`),
  NOT as the flattened projection of a requirement/satisfies join — so a
  surface-authored join still reaches no graph edge. `[edge.*]`/`EdgeClause`
  is retired (shipped). What remains: wiring join→`routes_to` flattening (the
  emit face derives the one-way pointer; a pointer with no join behind it is
  drift). Spec sanctions it (`45-governance.md`, coupling-is-a-join Decision);
  awaits a human decision to file.

- `(agents-md-builtin-kind)` — OPEN (registered 2026-07-06). The engine's
  hand-written std-lib ships an `agents-md.memory` built-in kind (glob
  `**/AGENTS.md`), but the SDK module and the derived built-in lock export only
  the `CLAUDE.md` `memory` kind (`15-kinds.md`: memory locus is `**/CLAUDE.md`;
  AGENTS.md is foreign/bridged — `30-landscapes.md`, `50-distribution.md`
  migrate-with-a-fix). BUILTIN-KIND-FLATTEN drops the engine's agents-md.memory
  to match the lock, removing direct AGENTS.md coverage. Open: should temper
  ship an AGENTS.md built-in kind — and if so under a **distinct label** (never
  a provider-qualified `memory`, which the identity-by-import Decision rejects)?
  A feature addition, not a chain blocker — the chain proceeds on the
  spec-faithful default (drop it).

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
  retirement draining: CURATED-TREES-RETIRE, BUILTIN-KIND-FLATTEN,
  BUILTIN-FLOOR-LOCK-PROJECTION, and CHECK-LOCK-KIND-ROWS **shipped** — the
  last cut main.rs's KIND.md/tree comments on contact (grep: zero
  `kinds/`/`packages/` refs in main.rs). The remaining src refs are
  comment-only and ride COMMENT-STOCK-SWEEP (open): builtin_kind.rs's
  `kinds/KIND.md`, bundle.rs's `packages/`, read.rs's `[edge.*]` lines.
  Not in the sweep: tests/session_start.rs writes `+++`-format
  `kinds/spec/KIND.md` + `packages/spec/PACKAGE.md` **fixtures** (live test
  code, asserting stray old-format files are ignored) — a behavioral question,
  accepted debt for now. Citation trail moves to clause `cite` fields
  (`10-contracts.md`). The physical `kinds/`+`packages/` tree deletion is a
  human `chore(harness):` commit — actionable, out of build's fence. This line
  dies when the sweep lands.
- **`.flume/` is ungoverned by temper** — the machine that builds temper is
  not yet under its gate; a candidate landscape once the custom-kind story
  proves end to end.
- **`docs/` is candidate intent, not intent** — human territory,
  fence-excluded; plan never reads a horizon entry as intent.
