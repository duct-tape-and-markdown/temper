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

- `(place-three-state-retire)` — OPEN (registered 2026-07-06). `src/drift.rs`'s
  `place()` implements the "three-state merge model (desired / last-applied /
  real)" with an `ApplyOutcome::Conflicted` branch — the model NAMED in the
  rejected alternative of "Decision: drift routes to the authored source — no
  reverse parse" (`20-surface.md`). All four real callers
  (`install.rs:484/509/533/550`) pass `None` for the baseline, so the
  `Conflicted`/`last_applied` path is dead outside one unit test; the module
  doc frames it as the "two-projectors seam" that "stays until `install` rides
  emit's projection." But `install` has now shipped, and the spec's placement
  staleness is "content-keyed fingerprint, one staleness mechanism everywhere"
  (`50-distribution.md`), which rejects "a second staleness mechanism." Does
  `place()` collapse its three-state merge now — retiring `Conflicted` +
  `last_applied` for content-keyed idempotent placement — or does the seam
  stand until emit rides it? Under-specified whether `place()` should adopt
  content-keyed fingerprinting (it byte-compares today, no fingerprint) or
  simply drop conflict detection. Related: `(authority-home)`. Needs John.

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
  the engine retirement drained (CURATED-TREES-RETIRE, BUILTIN-KIND-FLATTEN,
  BUILTIN-FLOOR-LOCK-PROJECTION, CHECK-LOCK-KIND-ROWS, COMMENT-STOCK-SWEEP,
  RETIRE-TOML-CONTRACT-PARSER, PKG-NOUN-COMMENT-SWEEP — all **shipped**). The
  `activation`→`registration` rename and the package/altitude comment tail
  (PKG-NOUN-COMMENT-SWEEP-II) **shipped** (89df4d5); the current residue tail is
  a THIRD retired noun (`manifest`, = the committed lock) plus corpus-wide stale
  spec-section-title citations — filed this tick as RETIRE-MANIFEST-NOUN and
  REFRESH-STALE-SPEC-CITATIONS. What stays
  a standing asymmetry: tests/session_start.rs writes `+++`-format
  `kinds/spec/KIND.md` + `packages/spec/PACKAGE.md` **fixtures** (live test code
  asserting stray old-format files are ignored) — a behavioral question,
  accepted debt; and the physical `kinds/`+`packages/` tree deletion, a human
  `chore(harness):` commit out of build's fence. Citation trail lives on clause
  `cite` fields (`10-contracts.md`).
- **`.flume/` is ungoverned by temper** — the machine that builds temper is
  not yet under its gate; a candidate landscape once the custom-kind story
  proves end to end.
- **`docs/` is candidate intent, not intent** — human territory,
  fence-excluded; plan never reads a horizon entry as intent.
