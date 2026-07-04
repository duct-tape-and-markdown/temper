# Open questions

Product/architecture forks not yet settled. Each is keyed with a `(slug)` so a
pending entry can declare `dependsOnForks: ["slug"]` and be held until resolved.
Mark a line `RESOLVED` (and record the decision) to unblock its dependents.

The forks below gate *extensions*. The package/assembly/kind migration they once
gated has **shipped** (verified on disk): `build.rs` embeds the built-in std-lib
from `packages/<name>/PACKAGE.md` (not `contracts/*.toml`), `.temper/` is authored,
and custom kinds feed the graph's `by_kind`. The old `contracts/*.toml` mirror is
**deleted** (CONTRACTS-RETIRE shipped — no `contracts/` dir on disk; only stale
path strings remain in comments).

## The six-noun model ceremony (2026-07-04, John + session) — corpus re-cut

The architecture corpus was re-cut in one ceremony to the six-noun model
(record: claude.ai/code/artifact/417f8f03-19d9-4036-a200-30e27b0f78db; the
session ledger carries the pointer). Ratified: types erase at the seam (no
KIND.md/PACKAGE.md grammars, no manifest-as-corpus), the engine is kind- and
schema-blind, registration generalizes activation (the world is its endpoint),
the package noun dissolves into `expect` clause arrays, genres are kinds at the
block locus, three equal authoring postures with a byte-stable fence bridge,
emit is total (members are the only source; settings.json/.mcp.json are
many-to-one projections; permissions derived from member `needs`), one
implementation behind a JSON pipe (SDK implements no semantics). Prior fork
resolutions above stand as history; their spec anchors moved in the re-cut.
Plan must re-reconcile the whole pending queue against the new corpus.
DATUM (2026-07-04, corpus catch-up c74a542): John swept the four un-recut
files to the six-noun vocabulary — 00-intent (law 2 + Positioning speak
floors/`expect`/`require`, the SDK Decision drops the fixtures'-schema
framing, the genre Decision re-cut to kind-at-the-block-locus),
30-landscapes (clause attachment over package binding, the three postures),
90-spec-system, 55-offering (the retired `check --harness` spelling → bare
`temper check`; the graph verbs → `explain`), plus `verified_by` →
`verifiedBy` and `impact` → `explain` corpus-wide, and two-line touches to
20-surface/45-governance. **Reconciled: no law changed, no fork resolved, no
cited intent moved, no pending entry affected** (PACKAGING-CHANNELS cites
50-distribution "Three channels", untouched). Every renamed concept
(`verifiedBy`, `explain`, floors/clauses) is the SDK-primary authoring face
— human-ceremony-gated; the `impact`→`explain` collapse is
`(explain-target-disambiguation)` below, human-disambiguation-gated. Nothing
pickable un-gated. The **residual** catch-up deltas the ceremony's "what's
next" names — 50-distribution's `(engine-language)` pointer (still "the open
fork" at 50-distribution:53) and the `temper guard` Decision
(`(guard-posture)`) — did **not** land here; 50-distribution was untouched,
so both stay human-to-do.
DATUM (2026-07-04, corpus catch-up 146a646): John swept the five ruled forks'
stale "open" wording out of the corpus ("a stale 'open' is itself a latent
reference") — `(engine-language)` now non-normative-not-open
(10-contracts/20-surface/50-distribution), the **`temper guard` Decision**
landed (50-distribution + 20-surface, `(guard-posture)`), `(mention-marker)`
`${address}`-across-postures encoded (15-kinds/20-surface),
`(settings-residual)` narrowed to project scope with the five-scope external
fact (40-composition), `(genre-fence-format)` marked deferred-to-the-pilot
(15-kinds). **Reconciled: no law changed, no fork newly resolved (all five
were already RESOLVED on plan's filed recs), no cited intent moved, no pending
entry affected** — PACKAGING-CHANNELS cites 50-distribution "Three channels"
(line 21, untouched; the diff hit ~50 + ~113-125). The residual both-human-
to-do above is **discharged**: both landed. Each fork's record below carries
its corpus-delta note. Nothing pickable un-gated — `(guard-posture)`'s
blocker (i) clears but (ii) still gates (see its record).
DONE (2026-07-04, post-merge reconcile): **AGENT-KIND, COMMAND-KIND, and
SETTINGS-KIND are RETIRED from pending** — every mechanism they named is
rejected by the re-cut: there is no `KIND.md` (15-kinds "Decision: field
typing lives in the SDK — there is no kind file format"), no `PACKAGE.md`
(a floor is an exported clause array in `@temper/claude-code`,
10-contracts), and no string-name identity / `QUALIFIED_FLOOR_BINDINGS` /
`BUILTIN_KINDS`-const machine (15-kinds "Decision: built-ins are a module,
and identity is an import" rejects "string-name identity with provider
qualification"). John's intent survives (agent/command/settings ARE in
scope, `15-kinds.md` built-in enumeration), but the vehicle changed: adding
a built-in kind now means an SDK value in the `@temper/claude-code` module
plus a compiled default-program entry embedded in the engine binary — a
different, unbuilt foundation, gated on the SDK-primary scaffolding (publish
`@temper/claude-code`, npm scoping — the ledger's "SDK npm scaffolding needs
John"). Re-file only once that foundation lands.

The dominant corpus↔code gap is the **six-noun engine demolition**: the code
is manifest-era (commits + reads a `temper.toml` manifest as the gate's corpus
— 20-surface rejects exactly that, (b); parses `KIND.md`/`PACKAGE.md` grammars
— retired; resolves qualified/bare string identity — retired). It is
frame-scale, deeply entangled (every piece touches builtin.rs / kind.rs /
main.rs / import.rs / drift.rs).
**UNBLOCKED + FILED (2026-07-04, John's go-ruling, inbox "let's go").** The
foundation is no longer human-ceremony-gated: the merged six-noun corpus IS
the ratified foundation, git is the rollback story, the per-tick gates (fmt,
clippy, test, sdk) are the only technical gates, and the dogfood self-gate is
deactivated for the wave. Plan filed the **full** demolition + rebuild (no
longer evidence-gated per slice) as one **linear serialized chain** in John's
build order — the entanglement above is precisely why it is serialized, not a
parallel fan-out: LOCK-DECLARATION-ROWS (open head) → GATE-CORPUS-ARTIFACTS-LOCK
→ SDK-RECUT-CORPUS-FACE → CLI-COLLAPSE (+ temper guard) → MANIFEST-MACHINERY-
RETIRE → KIND-PACKAGE-PARSE-RETIRE → EXPLAIN-UNIFY (terminal, fork-gated).
Build drains it one tick at a time; a ship unblocks the next link on the
following reconcile. Prior landed slice **INSTALL-RETIRE-CI-PLACEMENT** (5bc2ee2
— install.rs no longer projects a CI workflow) stands. The two localized
wrong-today items now ride the chain: the corpus-rejected guard grep
(`(guard-posture)`) is replaced by the `temper guard` subcommand at CLI-COLLAPSE;
the read-family CLI split (`(explain-target-disambiguation)`) is EXPLAIN-UNIFY —
still fork-gated (the one design question the wave cannot reach without John).

New forks from the ceremony:

- `(engine-language)` — RESOLVED (John 07-04, inbox): **keep the engine in
  Rust.** No spec delta — the corpus is language-neutral by design ("the
  engine", never a language as normative fact; 50-distribution "The stranger
  gate": "specs state the property, never the language"). The fork closes on
  the latency evidence (release binary ≈ 0.001 s vs `node -e ''` ≈ 0.026–0.178 s
  vs `npx` ≈ 0.32–0.73 s at the hook placements — binary favored ~25–700×) and
  the entry-gate-applied-to-rewrites argument (a rewrite discards the
  review-certified engine.rs decision table + document.rs byte-proven
  round-trip). CORPUS CATCH-UP DONE (146a646, John's hand): 10-contracts,
  20-surface, and 50-distribution now say "deliberately non-normative
  (`(engine-language)` resolved: Rust today, stated nowhere as contract)"; the
  stale "open fork" wording is gone. No behavior rode it. Kept as the decision
  record; no dependent waits.
- `(mention-marker)` — RESOLVED (John 07-04, inbox): **`${address}` in
  posture-2 markdown too** — one mention spelling across all three postures
  (the no-synonyms rule applied to syntax), byte-stable 2⇄3 for free, **not**
  recognized inside code spans or fenced blocks. The embed spelling stays
  `@path`. Blocks posture-2 mention edges only — gated on the posture-2/SDK
  authoring foundation (the same human-ceremony gate as the demolition); no
  pending entry today. CORPUS DELTA DONE (146a646): 20-surface + 15-kinds now
  state `${address}` in posture-2 markdown, not inside code spans/fences.
  Kept as the decision record.
- `(genre-fence-format)` — RESOLVED (John 07-04, inbox): **deferred to the
  genre-adoption pilot.** Entry gate applied to the fence: no posture-2 author
  exists (zero fences in the corpus; the pilot is parked on this fork), so the
  grammar is designed when its consumer lands, against a working byte-stable
  2⇄3 round-trip, never in the abstract. Frame note (John's catch): D7's "TOML
  genre fence" prior is **manifest-era residue** — the fence body is the
  byte-stable template of a TS value, so the pilot designs the grammar from the
  TS value shape outward, not from a Rust/TOML prior. Constraint survives
  whatever grammar wins: prose leaves render byte-exactly (law 5); the
  acceptance test (byte-stable posture-2 ↔ posture-3) is unchanged. No pending
  entry — the pilot is not itself a pending entry. CORPUS DELTA DONE (146a646):
  15-kinds now marks the fence "deliberately undesigned until its first
  consumer — the genre-adoption pilot", the TOML-fence prior explicitly
  unbinding. Kept as the decision record.
- `(settings-residual)` — RESOLVED (John 07-04, inbox): **focus project
  scope** — emit owns the whole project `.claude/settings.json` (typed
  project-harness facts + passthrough); personal preferences belong to the
  local/user scopes the format authority already layers (managed > CLI > local
  > project > user; permission rules merge across scopes — cite
  code.claude.com/docs/en/settings, retrieved 2026-07-04). Local scope later
  via a gitignored local harness module composed by convention — that is
  `(local-overrides)`' settings answer; user scope further out. NOTE for the
  settings design: installed **plugins** inject skills/hooks/MCP temper does
  not govern — the gating fields (enabledPlugins, marketplace lists) are
  project-harness facts, the rulesync-downstream positioning's eventual reach;
  named, not designed. Feeds the settings kind (retired from pending +
  foundation-gated, six-noun ceremony above); no pending entry today. CORPUS
  DELTA DONE (146a646): 40-composition now narrows the residual to project
  scope with the five-scope external fact cited (managed > CLI > local >
  project > user, code.claude.com/docs/en/settings, retrieved 2026-07-04); the
  exact typed field list stays open with the settings member design. Kept as
  the decision record.
- `(guard-posture)` — RESOLVED (John 07-04, inbox): **a `temper guard`
  subcommand** reading the `PreToolUse` stdin payload in the engine; whether it
  blocks follows the author's declared surface-authority posture
  (note/warn/block), default advisory. The generated-shell grep stays rejected
  (pattern-matching prose is mining, law 8; a generated script is a second
  implementation of the gate's judgment). Keep-Rust latency (~1 ms) is what
  makes the per-tool-call placement viable — `(engine-language)` bears directly.
  NOT YET FILEABLE as a pending entry — blocker (i) NOW DISCHARGED (146a646):
  50-distribution + 20-surface carry the **`temper guard` Decision** (the guard
  IS the subcommand, blocking per the author's declared posture, default
  advisory; the always-exit-0 grep is "the migration-era state the subcommand
  replaces"). But (ii) STILL GATES: the subcommand rests on the surface-authority
  posture model (note/warn/block — member `needs`, the six-noun model the engine
  does not yet carry) and the install/distribution demolition (its locus,
  `install::guard_command`, is entangled demolition territory — INSTALL-RETIRE-CI
  already carved one slice, guard messages ride the same rewrite). A bare
  advisory `temper guard` with no posture is degenerate (it does what the exit-0
  grep already does) and still co-edits the demolition's install.rs, so it is
  **not** a clean disjoint pickable slice today. DISCHARGED (2026-07-04, John's
  go-ruling): blocker (ii)'s sequencing dissolves — the SDK-primary posture model
  and the install/distribution demolition are now the FILED chain, and the guard
  rides it. The `temper guard` subcommand is filed inside **CLI-COLLAPSE** (John's
  step 4, "temper guard lands"), blockedBy SDK-RECUT-CORPUS-FACE (which carries
  the needs/posture model). The exit-0 grep in `install::guard_command`
  (src/install.rs:454) persists as accepted debt only until CLI-COLLAPSE lands.
  Kept as the decision record.
- `(explain-target-disambiguation)` — OPEN (dogfood catch, 2026-07-04). The
  re-cut ratified **one read verb** (`20-surface.md`, "Decision: one read verb
  — `explain`"), explicitly rejecting "(c) a verb per question (why /
  requirements / impact / context) — four spellings of one traversal". The code
  still ships exactly those four `clap` subcommands (`Command::{Why,Requirements,
  Impact,Context}`, src/main.rs:236–266; `read::{why,impact,context,requirements}`).
  Collapsing them into `explain <target>` is not human-foundation-gated (it reads
  the same `check`-computed graph the four verbs already read), but the corpus
  does **not** specify how a single positional `<target>` resolves across the four
  target species — member, requirement, an address at leaf grain, a neighborhood.
  Member-name vs requirement-name collision is the live crux. Plan must not invent
  the disambiguation (collaboration rule). FILED (2026-07-04) as pending
  **EXPLAIN-UNIFY**, `dependsOnForks: ["explain-target-disambiguation"]`,
  `blockedBy KIND-PACKAGE-PARSE-RETIRE` — the terminal leaf of the demolition
  chain, sequenced after the read family's data source is rewritten (check →
  committed-artifacts+lock), blocking nothing. It stays unpickable until John
  rules the target-resolution mechanism; this is the one design question the
  go-ruling'd wave cannot reach on its own. Human to settle it.
- `(local-overrides)` — OPEN (surfaced by the 40-composition re-cut, which
  carries the OPEN: marker). The dialect era's committed-plus-gitignored
  personal-override layer (temper-local.toml) has no stated spelling in the
  one-value assembly model. Candidate directions: a local harness module
  composed by convention, or an engine-side severity overlay; neither is
  ruled. Blocks nothing until someone needs a personal override.

Bugs filed by the ceremony's review (wrong today, independent of demolition
sequencing):
1. read.rs floor-binding fallback narrated any non-rule kind as bound to the
   skill floor (wrong-by-default read output). **SHIPPED** (b20aa5c,
   READ-FLOOR-BINDING-DEFAULT) — `read::bound_package` (src/read.rs:145-168)
   resolves each embedded kind's real floor by its qualified identity through
   `builtin::floor_package`.
2. drift's lock loops iterated the stale 2-entry `kind::BUILTIN_KINDS` const
   while more kinds are embedded; memory rows in the lock were invisible to
   drift's fingerprint map. **SHIPPED** (4b42fe5, DRIFT-KIND-ENUMERATION) —
   drift.rs now enumerates the generated `builtin_kind::BUILTIN_KINDS`
   (`<provider>.<name>`, bare tail). NB the stale `kind::BUILTIN_KINDS =
   ["skill","rule"]` const survives (kind.rs:30), now used only by one trivial
   compose.rs test; it retires with the entangled kind.rs demolition, not as a
   standalone throwaway slice.
3. import.rs:697-704, 1007-1015 — authored-layer carry-forward silently
   degrades to "nothing to carry" on an unreadable prior surface (data-loss
   edge). **HELD** — the ceremony's own note flags it "moot when document
   carriage is demolished, real until then", and the demolition (init-lift,
   `file()` members, no `.temper/` copy tree) deletes this carry-forward
   path outright; fixing code slated for removal is throwaway. Re-file only
   if the demolition slips.

## SDK dogfood-migration seams (surfaced by the 2026-07-04 pilot) — ALL RULED

The dogfood-migration pilot (temper's two live rules authored through the SDK,
`emit`ted, diffed against the committed dogfood) surfaced four seams between the
SDK authoring face and the Rust gate/importer. **Projection parity HOLDS** for
rules (`collaboration` byte-identical); the seams below were the residual gaps.
**John ruled all four in one session ceremony (2026-07-04, each on plan's filed
recommendation).** They split by the file each touches into **two disjoint
serialized chains** — a Rust-gate chain over `src/main.rs` and an SDK chain over
`sdk/src/emit.ts` — cross-language disjoint, so the two chain heads are
parallel-safe `open` entries; each chain's second link is `blockedBy` its head.
Kept below as decision records; no dependent still waits.

- `(sdk-placement-round-through)` — RESOLVED (a) (John 07-04): SDK emit reads the
  committed projection and rounds install's placement lines (managed-by note +
  schema modeline) through its whole-file re-emit, mirroring the Rust
  EMIT-OWNED-PLACEMENTS resolution — the two projectors agree by construction.
  `projectBytes` already accepts a `placements` param, never populated at the
  emit call site. `specs/architecture/20-surface.md`, "Content-faithful,
  deterministically emitted (law 5)". FILED as pending **SDK-PLACEMENT-ROUNDTHROUGH**
  (blockedBy SDK-CARRIAGE-CONVERGE — shares `sdk/src/emit.ts`).

- `(gate-kind-spelling-and-unknown-kind)` — RESOLVED (John 07-04): the gate
  resolves a qualified kind to its **bare** key before manifest-corpus lookup
  (identity is `<provider>.<kind>`, `15-kinds.md`), so a `claude-code.rule`
  member is checked, not silently skipped; AND an unrecognized manifest kind is a
  **loud finding**, never a silent `checked 0` (the collaboration rule's exact
  failure mode). The SDK keeps stamping the qualified identity (`members.ts`
  unchanged); the gate is the side that resolves. `specs/architecture/15-kinds.md`,
  "Decision: kind identity carries a provider axis". FILED as pending
  **GATE-KIND-RESOLVE** (`open` head of the Rust-gate chain).

- `(gate-reads-assembly-artifacts)` — RESOLVED (a) (John 07-04): the gate learns
  to read the two SDK-emitted temper-owned artifacts `roster.toml`/`bindings.toml`
  (`sdk/src/assembly_artifacts.ts`) as the assembly source, so an SDK-emitted
  members-only `temper.toml` stops yielding `requirement.dangling` — the artifacts
  exist by ratified ruling, the gate is the side not yet reading them.
  `specs/architecture/20-surface.md`, "CLI surface" (the engine reads the committed
  harness + the temper-owned artifacts + the lock). FILED as pending
  **GATE-READS-ASSEMBLY** (blockedBy GATE-KIND-RESOLVE — shares `src/main.rs`).

- `(module-carriage-manifest-shape)` — RESOLVED converge (John 07-04): module
  carriage serializes **per-heading exactly like the importer** — the SDK's
  `toManifestMember` sectionizes per heading (heading line split out) rather than
  one whole-body section with the H1 inside ("three carriages, one feature shape";
  every consumer stays carriage-blind). The `line_count` +1 (trailing-newline
  `split("\n")` artifact) is a plain bug fileable **within** this shape.
  `specs/architecture/20-surface.md`, "Decision: three carriages, one feature
  shape". FILED as pending **SDK-CARRIAGE-CONVERGE** (`open` head of the SDK chain).

- `(scripted-altitude-reconcile)` — OPEN for the **altitude rung only**; the
  **floor wave has fully SHIPPED** (2026-07-03, ask (b) discharged and all seven
  serialized links landed — see tail). The **scripted-altitude** corpus re-cut
  landed as new intent (`specs:` 32ea84d, ratified 2026-07-03; pre-state =
  `mirror-era` tag) and the code's **floor has now migrated to it**; only the
  altitude rung is unmigrated. The reformulation is decided (`00-intent.md` Decision "the
  authoring face is a typed library; the gate reads inert data"; `20-surface.md`
  throughout). What remains OPEN is the **altitude rung's human prerequisite**
  (ask (a), the TS SDK); the floor sequencing (ask (b)) is now settled. The
  corpus↔code deltas:
  (1) **`import` → `init`** — the on-ramp becomes scan → config skeleton over
  members **in place**, zero file moves (today `import` copies members into
  `.temper/`; `20-surface.md` "Decision: `init` is the on-ramp, and adoption is a
  gradient"). (2) **`apply` → `emit`** — compile the authoring face, byte-
  reproducible + double-emit-verified (`20-surface.md` "Content-faithful,
  deterministically emitted"). (3) **`re-add` RETIRED** + the three-state merge
  (desired/last-applied/real) **deleted** — drift collapses to **one authored
  direction + two freshness facts**: `config.stale` (committed manifest/projection
  vs lock's `source_hash`+`emit_hash`) and projection hand-edits routed to the
  authored source (`20-surface.md` "Decision: `re-add` is retired", "Drift — one
  direction, two freshness facts"). (4) **manifest** — `temper.toml` becomes the
  **only** artifact the gate reads (today `check` ranges over the imported
  `.temper/` surface, and the lock carries `last_applied` not `source_hash`/
  `emit_hash`). (5) **three carriages** (module / document / in-place), one
  feature shape, every consumer carriage-blind (`20-surface.md` "Member carriage").
  (6) **mentions** — authored interpolation edges, one-way resolution-checked
  obligation-free, opt-in per word (law 8's never-climb bound). Already SHIPPED:
  `impact` (main.rs:546, read::impact — graduated from horizons in the re-cut).
  Why it is a fork, not a wave of `open` entries: the floor pieces (1)-(4) all
  rewrite the **same entangled files together** — drift.rs (three-state → config.stale),
  import.rs (copy → in-place skeleton, shared write path), main.rs (verb rename),
  install.rs (guard messages + the schema-modeline placement), frontmatter.rs/
  compose.rs — so they are **not parallel-safe**; and the floor's `config.stale`
  depends on the emit/manifest model existing, so there is no clean standalone
  head. The **altitude rung** (the TypeScript authoring face — `defineHarness`/
  `defineKind`/module members, `temper.config.ts`, mentions) needs **human npm/SDK
  scaffolding** (like PACKAGING-CHANNELS; the session ledger's residual "SDK npm
  scaffolding needs John"), and the spec corpus's own module migration is a
  **staged human ceremony** (`15-kinds.md` worked example). FOLDS IN the ledger's
  "two projectors, one file" residual: `apply`/`emit` re-emitting a rule drops
  `install`'s schema modeline + reflows the YAML — under the re-cut *emit owns the
  projection whole*, so install's placement lines must be **emit-owned** (part of
  the emit reconception, not a separate stamp) — filed as the tail entry
  EMIT-OWNED-PLACEMENTS, now **SHIPPED** (see tail): emit reads the committed
  projection before re-emitting and rounds install's placement lines through, so
  the two projectors agree by construction. The interim "do not run bare
  `apply`/`emit` on rules" discipline is **RETIRED**.
  **ASK (b) DISCHARGED (2026-07-03):** John delegated the floor-wave sequencing
  to plan (inbox ruling); this tick FILED it as the serialized `blockedBy` chain
  **READD-RETIRE → LOCK-FRESHNESS-FACTS → EMIT-VERB → MANIFEST-EMIT →
  MANIFEST-GATE-READ → INIT-ONRAMP → EMIT-OWNED-PLACEMENTS** (all pure Rust, no
  npm). A strict linear chain, not a fan-out: the entanglement map showed every
  floor delta touches `drift.rs`/`main.rs` and the lock shape is shared across
  import/drift, so no slice is parallel-safe — each entry is `blockedBy` the
  prior, the head `open`, build runs them one tick at a time (a ship unblocks the
  next on the following plan reconcile). **PROGRESS (2026-07-03):** two chain
  links have now **shipped** — READD-RETIRE (`build` caa30c4 / `chore` 3804f1c)
  and LOCK-FRESHNESS-FACTS (`build` fa8067d / `chore` 2ef5c4d, carrying the
  legacy-lock self-migration; the earlier afterMerge revert did not recur). A
  **third** shipped — EMIT-VERB (`build` 44df9a8 / `chore` f534fe9):
  `apply`→`emit`, whole re-emit, double-emit verified, three-state merge dropped.
  A **fourth** has now shipped — MANIFEST-EMIT (`build` adbd18a / `chore`
  cdd26ca): import serializes the generated-canonical manifest (`[[member]]`
  feature tables) beside `.temper/`, the floor patched format-preserving; the
  read side stays inert until the gate consumes it. A **fifth** has now
  shipped — MANIFEST-GATE-READ (`build` 70eb831 / `chore` f185d2f): the
  committed `temper.toml` manifest's `[[member]]` tables ARE the gate's corpus
  (main.rs:700, keyed by bare kind), carrying the inbox-recommended legacy
  fallback — a manifest with zero `[[member]]` tables (temper's own pre-`emit`
  dogfood) falls back to live `check::surface_units` extraction, so the gate
  never reads an empty corpus and the self-gate red does not recur. A
  **sixth** has now shipped — INIT-ONRAMP (`build` f7a830b / `chore` eff71a1):
  `import`→`init`, scan into a `temper.toml` manifest over members **in place**,
  zero `.temper/` copy tree (`main.rs:273`, `Command::Init` → `import::init`).
  The **seventh and last** has now shipped — EMIT-OWNED-PLACEMENTS (`build`
  caf4cab / `chore` b31e787): emit reads the committed projection before its
  whole-file re-emit and rounds install's placement lines through (`install::
  placement_lines` drift.rs:538, `project_bytes(…, placements)` drift.rs:589),
  install exposes the single predicate deciding which lines are its own, so the
  modeline/managed-by note survive re-emit and `gate_installed` stops re-nudging.
  **THE FLOOR CHAIN HAS FULLY DRAINED** — all seven links shipped; the code has
  reconciled to the scripted-altitude corpus's floor. ACCEPTED DEBT NOW
  DISCHARGED (2026-07-03; `chore(harness)` 4bd4813, the wave-end confirmation
  pass): temper's own `temper.toml` was regenerated via `emit` and now carries
  17 `[[member]]` tables (verified on disk), with `.temper/lock.toml` re-stamped
  — so the manifest-read path is now exercised against the live dogfood, not only
  fixtures. The self-check gate was re-armed the same day (118af33). These floor
  entries were NOT
  `dependsOnForks`-gated on this fork: the human unblocked the floor explicitly.
  **ALTITUDE RUNG — ask (a) DISCHARGED (2026-07-03):** John delegated the
  scaffold and it landed (`sdk:` a13ad53; `sdk/**` entered build's fence,
  7a0f56f). The scaffold (`sdk/`: emit.ts + members/genres/assembly/manifest,
  package.json `temper-sdk` private, node --test) names its own bounds, each
  failing loud if reached: byte-parity with the Rust `toml_edit` output,
  projection writing, lock stamping, `fromFile` resolution, mention
  resolution-checking. Confirmed this tick: its manifest schema
  (`[[member]]`/`[[member.section]]`/`[[member.genre]]` with
  `[member.genre.leaves]`/`[member.genre.collections.*]`) **matches** compose.rs
  (`genre_to_table`/`write_member_table`) and tests/genre_leaf.rs — no
  schema-shape mismatch; the byte-parity slice reconciles exact spelling only.
  The follow-on slices were FILED as the serialized altitude wave
  **SDK-EMIT-BYTE-PARITY → SDK-BODY-RESOLUTION → SDK-PROJECTION-LOCK** (they
  share `emit.ts`, so serialized, not parallel), gated by `(sdk-build-gate)`
  below (RESOLVED — the human wired the `sdk test` gate, 118af33). **THE
  ALTITUDE WAVE HAS NOW FULLY DRAINED** — all three links shipped (verified on
  disk this tick): SDK-EMIT-BYTE-PARITY (`build` 272b4f4), SDK-BODY-RESOLUTION
  (`build` 9641d9d — `emit.ts` `resolveBody` reads `fromFile` assets and
  resolution-checks mentions against the declared `mentionable` address set),
  and SDK-PROJECTION-LOCK (`build` 8796ce8 / `chore` 47cc021 — `sdk/src/
  project.ts` writes the byte-faithful `.claude/**` projection, `sdk/src/
  lock.ts` stamps `source_hash`/`emit_hash`, both exported from `index.ts`). The
  TypeScript authoring face now emits manifest + projection + lock in one
  deterministic `emit`. What remains for the SDK is **not engine work**: publish
  name/scope is parked on PACKAGING-CHANNELS, and the dogfood's migration onto
  the authoring face is the ledger's TS-primary reformulation — awaiting John's
  ruling, not a pending entry. Design record:
  claude.ai/code/artifact/3b82d365-492d-4900-ad41-e00feb755a07.

- `(sdk-build-gate)` — RESOLVED (2026-07-03; `chore(flume)` 118af33). `sdk/**`
  is TypeScript inside a cargo-gated pipeline, so without a TS gate a build tick
  shipping an SDK slice would pass every gate *trivially* (no Rust changed) while
  the SDK's own compiler and tests never run — the "silent gap reads as done" the
  collaboration rule forbids. The human wired the fix into `.flume/chain.ts`: a
  `shellGate` named `sdk test` running `pnpm --dir sdk test` (tsc + node --test)
  at **afterMerge** (serial on the trunk, where `sdk/node_modules` exists), added
  to build's `gates` array (`[fmtGate, clippyGate, testGate, selfCheckGate,
  sdkGate]`, verified on disk). So build now validates TS slices. The altitude
  wave un-parks — SDK-EMIT-BYTE-PARITY is now `open`, its successors `blockedBy`
  in series. Kept as the decision record; no dependent still waits.

- `(display-rule-emit-face)` — RESOLVED (John's ruling, 2026-07-03; inbox
  drained this tick): **the display rule ships as its own narrow engine
  slice**, NOT riding the custom-kind emit face. The corpus carries "a genre
  value is rendered by one corpus-wide **display rule** per genre — emit-owned,
  byte-deterministic" (`specs/architecture/20-surface.md`, "Content-faithful,
  deterministically emitted (law 5)", the "display rule owns connective tissue"
  bullet). Scope of the ruled slice: a per-genre projection formatter over the
  manifest's genre tables (`[[member.genre]]` — leaves + keyed collections)
  rendering the fenced block in place — headings, Chosen/Rejected labels,
  ordering, anchors; **connective tissue ONLY**, every meaning-carrying word
  tracing to an authored leaf (reuse the mention display rule as the pattern);
  byte-deterministic, double-emit verified. The **island risk is accepted**:
  the rule's contract is pinned by spec, so the future custom-kind emit face
  must reproduce its bytes when it absorbs it. Filed this tick as pending
  **DISPLAY-RULE-GENRE** (`open`, `src/display.rs` + `tests/display_rule.rs` +
  the `lib.rs` `pub mod` line — disjoint from the sdk/ altitude entries). NB
  emit today projects only skill/rule bodies (`src/drift.rs:459-477`); there is
  no custom-kind projection, so the slice delivers the spec-pinned **formatter**
  as a standalone island (no drift.rs wiring), not an end-to-end custom-kind
  emit. Unblocks the genre-adoption pilot (45-governance or 15-kinds, a staged
  human ceremony — the pilot itself is NOT a pending entry). Kept as the
  decision record.

- `(extraction-source-not-mechanism)` — RESOLVED (human ruling this session;
  `specs/architecture/15-kinds.md`, "The extraction algebra — the soundness boundary, as data":
  "Today extractors are engine code … The end state is that extraction is composed
  from a closed algebra"). The built-in/custom split is **source, never
  mechanism**: every member's *extraction* runs through the one generic composed
  path, a built-in differing only in that its KIND definition sources from embedded
  product data (`kinds/<name>/KIND.md`) rather than the project's `.temper/kinds/`.
  The hand-coded `skill_features`/`rule_features` and the per-kind surface readers
  are transitional scaffolding, retired by the serialized wave
  HEADER-FIELD-EXTRACTION → EXTRACT-EQUIVALENCE-PIN → EMBED-BUILTIN-KINDS →
  BUILTIN-EXTRACT-GENERIC (equivalence pinned by snapshot before any swap). Engine
  code stays sanctioned ONLY at the harness adapter faces (parse/emit of the
  external Claude Code format, and the IR→`Unit` read face). Kept as the decision
  record; the wave is its build-out.

- `(package-surface-sequencing)` — RESOLVED: **machinery first, dogfood after.**
  The code reconciles to the model **against test fixtures**; temper's own
  `.temper/` surface stays parked until the machinery it would be authored in
  exists, then un-parks as a *validation* step (the dogfood proves the reconciled
  code, it is not a prerequisite tangled into it). Same order one rung up: temper's
  own `specs/` corpus migrates onto the surface (as `.temper/specs/` projecting to
  `specs/`) only after the surface language ships — chicken before egg, machinery
  before self-application. NB the model this reconciles *to* has deepened since the
  fork was filed: the surface is now the **surface-language** model — a member is
  **one authored document** (TOML-fenced clause-module header over the body, no
  `meta.toml`+body split), a **package** is one `PACKAGE.md` in the same medium
  (clauses in the header, guidance colocated), `import` is a one-time **migration**
  with incremental recognition, and `apply` **re-emits the projection
  deterministically** (the surgical-YAML-patch rule is superseded) — see the revised
  `20-surface.md`, `15-kinds.md` (the two-faced adapter), `10-contracts.md`
  (Packages). Plan reconciles the queue against *that* corpus, deriving the wave
  shape from dependencies as usual; the embedding mechanism for the shipped std-lib
  packages (`include_dir`/`build.rs` — a sanctioned-crate addition when reached)
  lands when temper's own `.temper/packages/` exist to embed, and the embedded
  `contracts/*.toml` floor persists only until then. CARRIED: the plan tick after
  resolution decomposed the migration into the serialized chain MATCH-ERADICATE →
  SURFACE-DOCUMENT-FORMAT → PACKAGE-DOCUMENT → PACKAGE-BINDING →
  REQUIREMENT-PACKAGE-TYPING → MEMBER-DOCUMENT-IMPORT → KIND-AUTHORED-ARTIFACT,
  with EMBED-BUILTIN-PACKAGES parked at the end as the dogfood/validation step.
  SHIPPED: that whole chain has landed — `packages/` is the embedded std-lib home
  (`build.rs`/`src/builtin.rs`), `.temper/` is authored, custom kinds resolve in the
  graph. Only the dead-`contracts/` deletion remains (CONTRACTS-RETIRE). No dependent
  waits; kept as the decision record.

- `(contract-name-field)` — RESOLVED + SHIPPED (88246bf). Option B
  (`specs/architecture/10-contracts.md` Decision: "a contract is identified by its path/role,
  not an internal name"). The hand-applied chore dropped `MissingName` and made
  `Contract.name` default to the file stem when the data file declares none
  (kept as `String`, not `Option`, since a display label always exists) — the
  curated nameless `contracts/skill.anthropic.toml` now loads as `skill.anthropic`.
  Chain head SKILL-CONTRACT-TEMPLATE is now `open`. Kept as the decision record;
  no dependent still waits on it.

- `(regex-crate)` — RESOLVED (`specs/architecture/10-contracts.md` Decision: "`allowed_chars`,
  not a general `pattern` clause"). `regex` was already sanctioned for *solved
  mechanics*; the live decision is to **not** expose an arbitrary `pattern =
  "<regex>"` clause — it is expressive enough to be an unsound proxy. The
  author-facing charset predicate caps at `allowed_chars` (`ranges` + `chars`, e.g.
  `[a-z0-9-]`); a genuine *format* need gets a narrow named predicate, never a
  general regex clause. Kept as the decision record; no dependent still waits.

- `(contract-selection)` — RESOLVED (`specs/architecture/20-surface.md` Decision: "contract
  selection is by artifact kind"). `check` maps each artifact to the built-in
  contract for its kind (skill → `contracts/skill.anthropic.toml`, rule →
  `contracts/rule.toml`), embedded as defaults. A per-workspace override is a
  later extension, not the default. Unblocks the rule artifact kind.

- `(skill-ref-syntax)` — RESOLVED (`specs/architecture/45-governance.md` Decision: "a reference
  is a declared edge on the surface, never grepped prose"). A reference is a
  **declared structured field** authored on the surface (the reference syntax),
  projected alongside any prose; the graph is built from these edges — never
  inferred by grepping prose for names/paths (the unsound prose-grep
  `10-contracts.md`'s referential rule forbids, the exact `companion-refs`
  unsoundness). So no prose-grep companion-ref check ships; a decidable referential
  clause runs only over a declared edge field. Kept as the decision record; its
  build (edge extraction + the graph) is the graph-scope frontier, downstream of a
  graph foundation.

- `(model-declaration-format)` — RESOLVED + now CARRIED (`specs/architecture/40-composition.md`
  "Declaring a custom kind" + its Decision "a custom kind is declared in `temper.toml`,
  extraction and all"). The domain model is **not** a separate declared format: a spec
  is a **custom kind** (`15-kinds.md`) whose entities are declared by the kind's
  extraction and whose relationships are declared edges (`45-governance.md`), declared
  under `[kind.<name>]` in `temper.toml` like any custom kind. `05-model.md` supplies the
  corpus's model *content* in prose; the *mechanism* is the kind system, not a
  `model.toml`. The format the old fork was "forwarded to but never carried" is now the
  concrete `[kind.<name>]` surface, built by the KIND-* chain (KIND-DECLARATION-PARSE …
  KIND-EDGE-RELATIONSHIPS). Kept as the decision record; no dependent still waits.

- `(workspace-scope)` — RESOLVED (`specs/architecture/20-surface.md` Decision: "the workspace is
  per-project"). The surface targets a **per-project** harness — the `.claude/` +
  co-located artifacts of one project, located by the explicit path `import`/`check`
  already take. Rejected (for now): mirroring the global `~/.claude`, or both; the
  global config is a later landscape root, not a redesign. Was the last fork gating
  the `apply`/`install` write-back path — now fork-free.

- `(yaml-writeback)` — RESOLVED, then SUPERSEDED (`specs/architecture/20-surface.md` Decision:
  "the projection is re-emitted; the surface is patched"). The original resolution
  (patch changed YAML fields surgically, never re-emit) was load-bearing when
  `.claude/` was a peer surface humans hand-curated. Under the surface-language
  model the projection is *generated* output: `apply` re-emits it deterministically
  (nothing of the human's in it to lose — content lives in the surface), and only
  the surface's own TOML is patched format-preserving (`toml_edit`). YAML exists
  only on the generated side. Kept as the decision record.

- `(surface-authority)` — RESOLVED (`specs/architecture/20-surface.md` Decision: "the surface is
  the source of truth"). The composition surface is canonical; `.claude/` + `specs/`
  are a projection of it (`apply`), and direct on-disk edits are reconciled back with
  `re-add`. The read-only-lens framing was rejected (it contradicts law 7 and strands
  fearless refactoring). With `(yaml-writeback)` + `(workspace-scope)` now both
  RESOLVED, the `apply` path is fork-free.

- `(field-type-lattice)` — RESOLVED (`specs/architecture/10-contracts.md` Decision: "the `type`
  vocabulary is a closed scalar/container lattice"). The `type` primitive ranges over
  a fixed closed set — `string`, `integer`, `number`, `boolean`, `list`, `map`,
  `null` — taken from the source scalar's *parsed* type; a richer type language
  (formats, unions, ranges) was rejected as the JSON-Schema unsound-proxy surface.
  Requires the extractor to preserve the source scalar type first (the `extract.rs`
  stringify shortcut is corrected before the primitive ships). SHIPPED: on disk the
  `type` predicate is parsed in `contract.rs` (with the `UnknownType` reject) and
  decided in `engine.rs` over the kind-preserving `FeatureValue` — TYPED-EXTRACTION →
  TYPE-PRIMITIVE both drained. Kept as the decision record; no dependent still waits.

- `(harness-contract-provisioning)` — RESOLVED, both halves.
  *Home/selection* (`specs/architecture/40-composition.md` Decision: "the author-declared contract
  lives in `temper.toml`, layered"): an optional `temper.toml` at the project root
  layers over the by-kind built-in floor and holds adoptions, overrides, and the
  harness roster — rejected alternatives: a field in the *generated* `author.toml`,
  or the shipped templates as the author's home. *`verified_by`* (`specs/architecture/10-contracts.md`,
  "`verified_by` — where behavior goes"): "wired" is a **referential** clause — the
  named verifier must *resolve* (test target / CI job / path exists) or admissibility
  fails; a string-present check was rejected (a dangling verifier is a silent no-op).
  SHIPPED: the whole role/`verified_by`/`temper.toml` layer is on disk — `compose.rs`
  layers an optional `temper.toml` (adopt / extend / override / severity-flip) over the
  by-kind floor and parses the `[role.*]` roster; `roster.rs` runs selection +
  `conforms-to` + admissibility (including `verified_by` resolving to a real path); all
  wired into `check` in `main.rs`. Kept as the decision record; no dependent still waits.

- `(binary-bootstrap)` — RESOLVED (`specs/architecture/50-distribution.md` Decision: "acquisition
  rides the ecosystem's package managers"). Ship the prebuilt binary through npm with
  platform-specific `optionalDependencies` (the common `.claude/`-project route), plus
  standalone release binaries, Homebrew, and `cargo install`, channel auto-detected;
  a single bespoke installer and the assume-globally-PATH'd-binary route were rejected.
  Fail-loud is intrinsic — a missing platform binary is an install error, never a
  silent skip. Kept as the decision record; gates packaging work, not the engine.

- `(spec-landscape-kind)` — RESOLVED, and its *build shape* now SUPERSEDED by the
  kind-declaration mechanism (`15-kinds.md` Decision "a custom kind is declared data,
  never engine code"; `40-composition.md` "Declaring a custom kind"). `spec` is a
  *custom* kind governing `specs/*.md` — but it is declared as **data in temper's own
  `temper.toml`**, not shipped as engine code. The earlier build shape (a hardwired
  `src/spec.rs` extractor, an unconditional `specs/*.md` import scan, an embedded
  `contracts/spec.toml`) is retired: those shipped a custom kind *as a built-in*, which
  breaks "temper ships none of them." The replacement is the KIND-* chain
  (KIND-EXTRACTION-ALGEBRA … KIND-RETIRE-BUILTIN-SPEC), and SPEC-KIND-GATE is dropped.
  The referential `references-resolve` clause is now downstream of KIND-EDGE-RELATIONSHIPS
  (the `[kind.<name>.relationships]` reconcile), not a `contracts/spec.toml` commit. The
  `section_contains` / decisions-name-alternatives **predicate** remains carved out as
  `(decision-marker-predicate)` below. Kept as the decision record; no dependent waits.

- `(rollup-index-rename)` — RESOLVED (inbox decision, spec-confirmed). The generated
  roll-up index is renamed `author.toml` → **`lock.toml`** — the contents' generated
  *state-of-record* (provenance + drift/apply fingerprints), a lock (Cargo.lock
  analogy), not an authored index. `specs/architecture/20-surface.md` now names it `lock.toml`
  ("The surface: a contract over its contents"; the topology diagram), superseding the
  `author.toml`↔`temper.toml` name collision. SHIPPED as RENAME-ROLLUP-LOCK —
  `src/import.rs` writes `lock.toml` (`LOCK_FILENAME`), `src/drift.rs` reads it, and
  zero `author.toml` references remain in `src/`. Kept as the decision record; no
  dependent still waits.

- `(decision-marker-predicate)` — RESOLVED (`specs/architecture/10-contracts.md`, structural
  primitives): `section_contains` `{heading, marker}` (every section whose heading
  starts with the declared text carries the declared marker) is now enumerated in
  the predicate vocabulary's home — the deliberate language addition law 3
  requires, authorized by `15-kinds.md`'s worked example and now carried.
  decisions-name-alternatives becomes fileable build work once the spec kind's
  package exists (downstream of the surface-language/package-model machinery).
  SHIPPED as SECTION-CONTAINS-PREDICATE — `Predicate::SectionContains {heading,
  marker}` is parsed in `src/contract.rs` and decided in `src/engine.rs`. Kept as
  the decision record; no dependent still waits.

- `(read-verbs)` — RESOLVED (`specs/architecture/20-surface.md` Decision: "the CLI gains a read
  family — `why` and `requirements`"). Two **read-only traversal verbs** over data
  `check` already computes: `temper why <member>` walks the requirement↔`satisfies`
  edge forward (requirements filled + rationale, governing package, edges);
  `temper requirements [<name>]` walks it in reverse (satisfier set, coverage state,
  blast radius of a removal). Projections, never gates — no new engine semantics, no
  non-zero exit on findings. Rejected: `check` flags as a query surface; a general
  `query` verb. Fileable as build work **after** the surface-language migration,
  once coverage + graph data exist to read. SHIPPED as READ-VERBS — `src/read.rs`
  carries `why`/`requirements`, wired into `main.rs` clap dispatch. Kept as the
  decision record.

- `(edge-representation-unify)` — OPEN (dogfood catch, 2nd harness). The harness
  carries **two disconnected edge representations**. The surface authors an edge as
  an `[edge.<target>]` clause (`EdgeClause` — read by the read family and emitted by
  projection), while the gate's graph (`src/graph.rs`) reads a structured `routes_to`
  field off extracted `Features`, keyed by `[[kind.<name>.relationships]]`, and
  **never reads `[edge.*]`**. So a surface-authored `[edge.*]` edge does not gate,
  yet `specs/architecture/20-surface.md` ("The member document") and `specs/architecture/45-governance.md` call
  `[edge.<target>]` "the graph's source" and the adapter's projected field its emitted
  face. Unresolved: how the two unify into one edge set — (a) extraction lifts `[edge.*]`
  into the graph's edge features (the adapter's inverse-of-projection read face); (b) the
  graph consumes `EdgeClause`s directly; (c) the structured `routes_to` field is the
  authored form and `[edge.*]` is retired. Impedance: `[edge.*]` is keyed by
  target+relation (member-side), a relationship by field+from/to kinds (kind-side), so
  resolving a target's kind needs both. READ-EDGE-UNIFY **has shipped** — it fixed the
  read↔gate divergence (read now consumes the gate's `resolved_edges` set, never the
  `[edge.*]` clauses). Verified on disk this tick, the residual is a precise
  **surface↔engine** divergence: `[edge.<target>]`/`EdgeClause` is still parsed, stored,
  and round-tripped (`document.rs`, `skill.rs`/`rule.rs`) but feeds **no** graph edge —
  the graph reads only the `routes_to` frontmatter field named by
  `[[kind.<name>.relationships]]`. So `[edge.*]` is **dead surface syntax**, closest to
  option (c) but with the clause orphaned, not retired. THIS fork is the deeper "one edge
  set" question — which representation is canonical and how a surface-authored edge reaches
  the gate's graph. No dependent filed; human to settle the canonical form.
  DATUM (2026-07-02): `specs/architecture/45-governance.md` gained the Decision "coupling
  is a join — a one-way edge never obligates its target" (`05-model.md` names
  `join`). It sharpens this fork's frame: `requirement`/`satisfies` is the sole
  two-sided join (mutual obligation); one-way declared edges (`routes_to` mirrored
  harness mechanics, `supersedes`/citation annotations) survive as
  **resolution-checked but obligation-free**. So whichever representation wins,
  `[edge.*]`/`routes_to` is the one-way class — resolved, never obligating — and
  the join machinery already ships (coverage/roster). Doctrine only; no engine
  work beyond what the classed-corpus entries already cover.
  DATUM (2026-07-02, join-retirement): `45-governance.md`'s revised Decision
  "coupling is a join — one-way edges exist only at the governance boundary"
  now *decides the representation*: member-to-member coupling is the
  requirement/satisfies join (canonical); a one-way pointer in the projection
  is the emit face's **flattening** of that join (derived, never authored); and
  the `[edge.<target>]` surface clause is **retired** (rejected (b)) — option
  (c). All THREE scoped engine consequences have now **SHIPPED** (verified on
  disk): EDGE-CLAUSE-RETIRE (`[edge.*]`/`EdgeClause` gone from src/ — dead in
  document.rs/frontmatter.rs, only comment mentions of "edge" survive),
  ACTIVATION-KEY-PARSE (an inert `activation` key parses in KIND.md headers),
  and REACHABILITY-WIRE (the `reachable` predicate now gates at assembly-declared
  severity). What stays OPEN is the narrower *mechanism*: the graph today reads
  `routes_to` as an extracted feature (`[[kind.relationships]]`), NOT as the
  flattened projection of a join, and joins build only coverage — so a
  surface-authored join still reaches no graph edge. Wiring join→`routes_to`
  flattening (the emit face derives the one-way pointer; a landscape pointer with
  no join behind it is drift) is fileable work the spec now sanctions, but NOT
  filed — the human scoped the revision to those three shipped consequences; the
  join→graph unification is the residual and awaits a human decision to file it.

- `(launch-front-door-docs)` — RESOLVED. **AGENTS.md is a separate,
  build-authorable contributor doc; `CLAUDE.md` stays canonical and untouched.**
  This repo's `CLAUDE.md` is not a generic contributor doc — it is the operating
  harness for the pipeline's own agents (the recursive-dogfood fixture), so the
  ruff/uv "`CLAUDE.md` = `@AGENTS.md`" gutting is rejected here. AGENTS.md is
  fileable build work: contributor-facing (build/test/lint commands, architecture
  map, pointer to the AI-contribution policy in CONTRIBUTING.md per
  `specs/intent/55-offering.md`); it does NOT touch `.claude/**` or `CLAUDE.md` (the
  chore(harness) boundary holds). CHANGELOG.md: fileable as a root stub with an
  Unreleased section now (the alive-signal, `specs/intent/55-offering.md`); release
  cadence is settled at first tag (PACKAGING-CHANNELS). When temper's own surface
  eventually governs the harness, both docs become projections and the
  canonicality question dissolves.

- `(eval-capability)` — OPEN, strategic, parked past launch. The platform is
  unusually pre-positioned to offer **harness evals** — answering the question one
  level up from the wedge: not "did the rule load" (tier 1 answers) but "does the
  rule *do* anything" (nobody answers today). The assembly is already an eval
  spec: every requirement carries `means` (declared intent) and every `satisfies`
  a rationale — each edge is a testable behavioral claim, and the graph gives
  eval *selection* for free (blast radius → which evals a change must re-run).
  The model already holds the seat: an eval runner is a **`verified_by` verifier
  type** (`verified_by = "eval:<case>"` — the "wired → wired and gating" loose
  end, `45-governance.md`) and/or tier 2 made concrete (judged, calibrated,
  forever advisory, `00-intent.md`). Hard constraint if ever built: an eval is
  probabilistic and can NEVER enter tier 1 or the hard gate (law 3) — it arrives
  as a verifier/advisory tier only, no new concepts. Human fork on scope and
  timing; no dependents; do not let it near the launch wedge.

- `(kind-harness-axis)` — RESOLVED (human ruling 2026-07-02, after a cited
  three-agent market sweep — `docs/market-formats.md`): kind identity carries a
  **provider** axis (`specs/architecture/15-kinds.md` Decision "kind identity carries a
  provider axis"). Provider = the authority that defines the format (a tool —
  `claude-code`, `cursor` — or a standard — `agents-md`, `agent-skills`);
  identity is `<provider>.<name>`; bare names resolve iff unique, collision is
  a load error; project kinds stay bare; placement mirrors identity
  (`kinds/claude-code/skill/`); published packages bind qualified. Rejected:
  vendor axis (Copilot's surfaces diverge; Windsurf changed vendors),
  active-profile resolution, mandatory qualification. `05-model.md` now splits
  **provider** (format authority, identity axis) from **harness** (consuming
  runtime, the world). Build-out: PROVIDER-KEY-PARSE first (parser accepts
  `provider` inert + qualified/bare resolution + collision diagnostic), then
  the human moves curated files to `kinds/claude-code/*` and adds provider
  lines (build.rs embed must walk the nested layout), then binding
  qualification. FILED + SHIPPED — the whole build-out has landed (verified on
  disk): PROVIDER-KEY-PARSE (c52df4f), EMBED-NESTED-WALK (c65c2ed), the human
  file-move to `kinds/claude-code/{skill,rule}/` with `provider` lines (3cf756b),
  and BINDING-QUALIFY (fd4d142) — `src/builtin.rs` binds the *qualified* kind
  identity via `builtin_kind::qualified`, the floor tuples resolve through the
  bare→unique-or-collision path (`resolve_bare`), published-binds-qualified.
  This fork is fully built out; no dependent waits. Kept as the decision record.
  Original record: was OPEN. Kind identity has no harness axis: bare
  `skill`/`rule` works while Claude Code is the only harness, but a second
  harness's artifact classes collide — Cursor's "rule" is a different format
  with a different adapter and a different sourced package than Claude Code's.
  The corpus already stubs the axis (`05-model.md`'s harness/profile row;
  `45-governance.md`'s harness-version-pinning loose end) without carrying it.
  Options: harness-scoped kind names (`claude-code.rule`), a `profile`/harness
  field on the kind definition with bare names resolving inside the active
  profile, or defer wholesale. Package naming already scales without it
  (`rule.anthropic` / `rule.cursor`, `10-contracts.md`); only *kind* identity
  is open. No dependent today — becomes load-bearing with the first
  non-Claude-Code adapter family. Human to settle when a second harness is
  actually scheduled, not before.
  DATUM (2026-07-02, unification day): the trees already disagree — packages
  are source-qualified (`packages/rule.anthropic`) but kinds are not
  (`kinds/skill`, not `kinds/skill.anthropic` or a per-harness grouping).
  With declared adapters, a new harness family is a directory of data files,
  so this fork's answer decides that directory's shape; blocking-adjacent the
  moment a second harness (Codex, Cursor) is attempted.

- `(multi-harness-projection)` — OPEN, strategic. The surface language is
  harness-neutral and built-in kinds are two-faced adapters (`15-kinds.md`), so
  one member could in principle project to N harnesses at once — `.claude/rules/`
  *and* `.cursor/rules/` from one authored document: rulesync's portability
  falling out of the architecture as a side effect, while keeping the quality
  story rulesync lacks. But it is a real scope call with lossiness questions
  the mirror-model reframe never had to answer: per-harness capability mismatch
  (Claude Code `paths` scoping has no Cursor equivalent and vice versa), which
  harness is authoritative when capabilities diverge, and whether a lossy
  projection is a verdict or an error. The *read* face of foreign formats is
  already decided (`50-distribution.md`, migrate-with-a-fix); only the write
  face is open. Explicitly out of scope for the package-authoring session and
  the launch wedge; human fork, no dependents filed.

- `(project-name)` — RESOLVED, provisionally (`specs/intent/55-offering.md` Decision:
  "the name stays `temper`, carried on scoped registries"). Keep the name; the
  contested registries are routed around: crate `temper-cli` (binary stays
  `temper`), npm scoped/prefixed, own Homebrew tap — the audience installs via
  npm/brew/plugin, never `cargo install`. Costs accepted eyes-open (someone
  else's `temper` crate; search mindshare shared with Temper-the-language).
  Two riders: the keep is **reaffirmed at launch** (the last cheap rename
  moment), and a **USPTO screen / Temper Systems non-objection** is the
  human due-diligence item before launch. PACKAGING-CHANNELS uses the scoped
  names. Kept as the decision record.

- `(kind-artifact-format)` — RESOLVED (`specs/architecture/20-surface.md` Decision: "a kind
  definition is `KIND.md` — one document, same medium"). A custom kind is authored
  as `.temper/kinds/<name>/KIND.md`, a surface-language document like every other
  artifact: the TOML-fenced header carries the definition (`governs`, composed
  extraction, entities/relationships); the body is the kind's own prose — what the
  artifact class *is*, for the authors of its members (a kind definition is not
  "pure structure" in an authoring medium). Rejected: a bare `kind.toml` (a second
  dialect, strands the prose); overloading another document name. The uppercase
  document-per-directory convention (`SKILL.md`, `PACKAGE.md`, `KIND.md`) names the
  role the directory plays. Un-gates KIND-AUTHORED-ARTIFACT. Kept as the decision
  record.

- `(reference-id-normalization)` — RESOLVED, then **SUPERSEDED** by law 8
  (`specs/architecture/15-kinds.md` Decision: "no body-mined references — the `references`
  primitive is retired"; `specs/intent/00-intent.md` law 8). The earlier resolution
  declared a per-kind reference *normalization* (`strip_suffix = ".md"`,
  `` `15-kinds.md` `` → `15-kinds`) for a backtick-filename syntax **mined from
  the member body**. Law 8 retires the whole body-mining primitive: relationships
  range over **declared structured fields only**, never grepped prose — backtick
  file mentions are typography, permanently. So the shipped `strip_suffix`
  machinery is not wired into the dogfood; it is **removed** (filed
  REFERENCES-RETIRE: drop `Primitive::References` + `strip_suffix` +
  `backtick_filename_refs` + `is_filename_reference` from `src/kind.rs`). The spec
  corpus's real edges are the header `[edge.*]` / `satisfies` declarations of the
  classed corpus (`specs/process/90-spec-system.md`), never extracted from bodies. Kept as
  the decision record; the fork is closed by the retirement, not by wiring.

- `(default-assembly-as-data)` — OPEN. The built-in floor's zero-config
  adoption (a bare harness gets skill/rule checked with no `temper.toml`) is
  today **engine behavior**, the one deliberate built-in/custom asymmetry the
  unification kept. The proposed rung: the floor becomes an **embedded default
  assembly** — shipped declared data beside `kinds/` and `packages/`, same
  source-not-mechanism move — so "what temper does with zero config" is
  readable, cited, and overridable rather than coded. Recommendation: adopt;
  needs its `specs/architecture/40-composition.md` Decision (the floor is data), then a
  small engine wave. Filed 2026-07-02 from the ladder review.

- `(reachability-gate-mechanism)` — RESOLVED (human ruling 2026-07-02): option
  **(b)** — reachability is **assembly-declared, like `degree`**. A graph-scope
  predicate's opt-in and severity are assembly declarations, per the
  declare/require cleave (`45-governance.md`: the set/graph scopes are the
  assembly's; a package clause is artifact-scope). The overpromising spec
  sentence ("the package's clause choice") is corrected in the same commit as
  this resolution; `src/graph.rs`'s `reachable` doc comment (~line 338) carries
  the stale sentence and REACHABILITY-WIRE sweeps it. Rejected: (a) always-on —
  a dead edge can be deliberate (a WIP skill with a blank description), the
  severity dial is real; (c) graph-tier package clauses — extends the package
  vocabulary across the scope line the model deliberately drew. Unblocks
  REACHABILITY-WIRE. Original record: was OPEN (dogfood residual; REACHABILITY
  shipped 50c5a00). The `reachable` graph predicate — world→member
  activation-edge liveness (`45-governance.md`, "The world is a node") — is
  library-proven in `src/graph.rs` (`world()`/`reachable`/`dead_activation`)
  and `tests/graph.rs:460`, but reaches **no gate**: `main.rs` never calls
  `graph::reachable`. The mechanical half is clear (build the map of each
  kind's declared `Activation` + the repo file set, dispatch, extend
  diagnostics beside acyclic/degree at `main.rs:654-666`). The open question is
  *how reachability carries author-declared severity*. The spec says "what
  severity it carries is the package's clause choice, like every other
  predicate here" — yet the three shipped graph-scope predicates each carry it
  differently: `acyclic` is **always-on** (fixed severity, no clause),
  `degree` is **assembly-declared** per requirement (opt-in bound), route
  resolution is always-on. And a reachability *fact* ranges over the world
  node + repo files, not one artifact's features, so it does not fit the
  artifact-scope `Predicate` vocabulary a package clause lives in
  (`src/contract.rs`). Options: (a) always-on like `acyclic` (simplest;
  strands "package's clause choice"); (b) an assembly/kind-level opt-in with a
  bound, like `degree`; (c) extend the package clause vocabulary with a
  graph-tier `reachable` clause plus a mechanism for a package to declare
  severity over a graph fact. Blocks pending **REACHABILITY-WIRE**. The curated
  `kinds/skill|rule` `activation` lines it also needed have **both shipped**
  (skill `description-trigger` 2259667, rule `paths-match` 9f7d176 — verified on
  disk), so the severity mechanism is the *only* remaining blocker. Human to
  settle which mechanism.

- `(builtin-workspace-qualified-key)` — OPEN (dogfood residual, surfaced by the
  memory kinds). The built-in `check::Workspace` map (`src/check.rs` `load_kinds`)
  and `src/read.rs` `members()` key built-in members by **bare** kind name, and
  `members()` hardcodes `skills()`/`rules()`. Two same-bare-name providers
  (`claude-code.memory` + `agents-md.memory`, deliberately colliding on `memory`,
  86d5b70) therefore collide under one `"memory"` map key, and the read family's
  member enumeration never ranges over a memory member at all. CHECK-MEMBERS-ALL-KINDS
  sidesteps this on the **gate** path — it loads per **qualified** kind via the free
  `check::surface_units`, so the max_lines advisory fires without touching the map —
  but `why`/`requirements` will not enumerate a memory satisfier, and a
  two-provider harness loses one member from the `Workspace` map. The fix has a real
  mechanism question: re-keying the map by qualified identity breaks the
  `skills()`/`rules()` accessors (they'd need a bare→qualified resolution, and a
  requirement/edge referencing `kind = "memory"` is itself ambiguous). No live
  consumer today — temper's own `.temper/` carries no memory member until the flip —
  so this is deliberately unfiled. Human to settle the accessor/lookup keying before
  the read family is generalized over all embedded kinds.

## Harness-kind scope-in (surfaced by the 2026-07-04 inbox ruling)

John ruled the intent-named harness surfaces in scope (`00-intent.md` surface
enumeration; `15-kinds.md` built-in enumeration names skill/rule/memory/agent/
hook/command/MCP/settings). The intent stands. **SUPERSEDED MECHANISM
(2026-07-04 re-cut):** the entries this section once tracked — AGENT-KIND,
COMMAND-KIND, SETTINGS-KIND — are RETIRED from pending (see the six-noun
ceremony section above): the KIND.md/PACKAGE.md/string-identity vehicle is
gone, and every built-in kind now arrives as an SDK value in
`@temper/claude-code` plus a compiled default-program entry. The two forks
below were framed against the retired `format`-vocabulary mechanism; both are
reframed under the re-cut and neither gates a pending entry today.

- `(json-projection-format)` — OPEN. The JSON-manifest built-in kinds — `settings`
  (`.claude/settings.json`), MCP (`.mcp.json`), and the plugin/marketplace
  manifests — are JSON with nested keys, but the projection-format vocabulary is
  **closed to one harvested entry, `yaml-frontmatter`** (`15-kinds.md`, "the
  adapter faces are declared": "the first and only harvested entry is
  `yaml-frontmatter`… the vocabulary grows deliberately, never per-kind";
  `src/kind.rs` `Format` has one variant, `parse_format` rejects all others).
  Reading them needs (a) a `json` format **harvested into the vocabulary** — a
  deliberate language addition (law 3), the corpus's to make, not plan's — and
  (b) a **generic JSON adapter** (a peer to `src/frontmatter.rs`) parsing the file
  into a `Unit` with nested-key fields, over the **now-shipped** key-path
  `Primitive::Field` (EXTRACTION-VOCAB-GAPS landed 1b9f33a — `extract::resolve_key_path`,
  src/kind.rs:839). The adapter is **one shared engine entry** (settings/MCP/
  plugin all rest on it), fileable once the format is harvested; it is sanctioned
  under the kept "format implementations are engine code" asymmetry.
  REFRAME (2026-07-04 re-cut): there is no `format` grammar or closed
  projection-format vocabulary to harvest into — a kind's on-disk shape is the
  **layout** fact (five-facts, `15-kinds.md`), declared as an SDK value. A JSON
  manifest kind is one whose layout reads a JSON file with nested-key fields.
  The generic JSON adapter (a peer to `src/frontmatter.rs`, over the shipped
  key-path `Primitive::Field`) is still real engine work, but it is gated on
  the SDK-primary foundation, not on a vocabulary addition. SETTINGS-KIND (the
  entry this once blocked) is retired; the MCP + plugin/marketplace kinds ride
  the same adapter, held here unfiled until the foundation lands.

- `(hook-kind-locus)` — OPEN. John ruled the `hook` kind in scope, but hooks have
  **no standalone file surface** — they are configured as JSON entries *inside*
  `.claude/settings.json` (`src/coverage_note.rs`: "Hooks are not a directory…
  configured inside settings.json"; `15-kinds.md` activation `event(field)`,
  "future hook kind, harvested now because the doc names the mechanic"). The
  `governs = {root, glob}` file-locus model every kind uses cannot select a hook:
  it is a nested array within another kind's member. Unresolved: is a hook a
  **sub-member** extracted from the settings member (a kind whose members are
  nested JSON entries, not files), a **distinct locus**, or a facet the
  `settings.anthropic` package checks via clauses over settings.json's `hooks`
  key? Needs John. No dependent filed — the hook kind is held here until the locus
  model is decided. Related: `(json-projection-format)` (a hook lives inside the
  JSON settings file).

## Kept on purpose — deliberate asymmetries (re-read every tick)

Every asymmetry below is a **choice with a condition**, not a fact. When its
condition arrives, it is the next break — today's declared-adapter wave came
from exactly such a line fossilizing. If work touches one, surface it.

- **Floor auto-adoption** (built-in kinds bind their packages with no assembly
  declaration) — kept for the zero-config front door; challenged by
  `(default-assembly-as-data)` above.
- **Format implementations are engine code** (the generic frontmatter adapter,
  post-swap) — kept because an external format's mechanics are temper's to
  implement once; the *selection* is declared. Grows only by deliberate
  vocabulary addition.
- **Two byte-parity TOML writers, kept in lockstep** (Rust `toml_edit` +
  the SDK emitter porting 0.22.27, 272b4f4) — kept until single-writer-per-format
  lands (rides the TS-primary ceremony, human-gated). STANDING RULE (inbox 07-03,
  web-verified: toml_edit's default output style is version-unstable — a breaking
  change at 0.22.25, "Reduced escaping in strings", plus earlier precedents): a
  `toml_edit` version-bump entry is a **contract event** — it must re-run the SDK
  byte-parity fixtures and reconcile both writers in the *same* entry, never
  bump-and-ship. The byte-parity fixtures (CONTRACT-DIR promoted them to
  `contract/`, shipped 375a1cc — `contract/{manifest,schema}/` goldens) are the
  tripwire.
- **`kinds/` + `packages/` are curated, fence-excluded** — kept because
  built-in definitions encode cited external facts; humans author, build
  embeds.
- **`.temper/**` is human territory** — the dogfood surface is the project's
  declared intent; the loop never writes it.
- **`.flume/` is ungoverned by temper** — the machine that builds temper is
  not yet under its gate; a candidate landscape once the corpus migration
  proves the custom-kind story end to end.
- **`docs/` is candidate intent, not intent** — `docs/horizons.md` parks
  product opportunities upstream of the corpus (its own header states the
  law-4 rationale). Human territory, fence-excluded; no phase writes it, and
  plan never reads a horizon entry as intent or cites one from a pending
  entry — an idea enters the corpus only through the human graduation
  ceremony the file describes.
- **CLAUDE.md bootstrap fence** — transitional until the `memory` kind ships
  and the flip ceremony moves it onto the surface.
  WAVE SHIPPED (2026-07-02): the five dependency-ordered engine slices —
  MEMORY-COLLISION-SCOPE, IMPORT-BUILTIN-SCAN-GENERIC, CHECK-WORKSPACE-KIND-MAP,
  DECLARED-FRONTMATTER-ADAPTER-CUSTOM, RECURSIVE-GOVERNS-PLACEMENT-ID — have all
  landed (RECURSIVE 26e296e; re-verified on disk this tick: `collect_glob`
  recurses `**` any-depth import.rs:358-398, `wholefile_id` folds placement via
  `fold_file_id` import.rs:535 / frontmatter.rs:478-533, `resolve_bare` gives the
  qualified-identity/collision resolution kind.rs:263-284). Engine is done and
  generic. The kinds are two **foreign-provider** carriers (`agents-md.memory`,
  `claude-code.memory`, deliberately colliding on bare `memory`) — curated,
  human-authored, cited to code.claude.com/docs/en/memory, held outside build's
  fence and committed only after the wave. DATUM (2026-07-02): that
  "co-embedding turns cargo test red today" is now precisely diagnosed and
  filed as **SCAN-QUALIFIED-IDENTITY**, which has now **SHIPPED** (build 7ae7240 /
  chore 61e4e6f; verified on disk — import/drift thread the qualified kind, no bare
  re-resolution; the `builtin_kind` enumeration test derives from the `kinds/` tree,
  not a hardcoded pair). The scans no longer re-resolve each kind by its bare name, so
  co-embedding two `memory` carriers no longer throws `AmbiguousKind`. The human file
  commit is no longer engine-blocked — it is the sole remaining prerequisite for the
  flip, and those four curated files are still absent on disk. MEMORY-KIND is the **parked
  flip-ceremony validation** entry (tests/memory_contract.rs), not the engine
  wiring — the wave carries that as generic, data-driven code. CORRECTION carried:
  `@path` imports are NOT in this wave (a later edge primitive if needed); they are
  neither the wave's work nor EXTRACTION-VOCAB-GAPS's fenced/key-path, so that entry
  stays deferred on its own no-consumer terms. Shipping the wave + the curated files
  unlocks the flip.
  DATUM (2026-07-02, precedence gap): a *second* placement attempt surfaced that
  `AmbiguousKind` was not the whole story — a project kind bare-named `memory` is
  *silently preempted* by the co-embedded carriers (the `builtin_names.contains`
  short-circuit at import.rs:146 skips it as a built-in layer), and a memberless
  carrier's empty roll-up section vanishes on round-trip. The Decision now rules it
  (`15-kinds.md`: "an explicit registration owns its bare name outright" — a
  registration shadows unbound embedded carriers). **REGISTERED-KIND-SHADOWS-EMBEDDED
  has now SHIPPED** (build 9fec9ab / chore c75075c; verified on disk this tick):
  `src/import.rs` shadows a registered bare-`memory` kind over its two-plus co-embedded
  carriers (the `builtin_names.contains` short-circuit no longer preempts a registration),
  skips a memberless embedded kind so it writes no empty section, and keys two
  co-discovering carriers' roll-up rows by qualified identity (`resolve_bare`). That was
  the **last engine prerequisite** — the memory engine wave is now fully drained and
  generic; nothing engine-side blocks the flip.
  DATUM (2026-07-02, files landed + a residual engine gap): the four curated memory
  files **are now on disk** (86d5b70: `kinds/{claude-code,agents-md}/memory/KIND.md`
  + `packages/memory.{anthropic,agents-md}/PACKAGE.md`) — the human-file prerequisite
  the whole block above tracked is **met**; every "still absent on disk" line above is
  superseded. But shipping them surfaced a residual engine gap the wave did not close
  (inbox 87a34f8, verified this tick): `check`'s member-assembly still **hardcodes the
  skill/rule pair**, so a discovered CLAUDE.md is imported (projected to
  `CLAUDE/MEMORY.md`) yet never dispatched to its `memory.anthropic` package — a
  >200-line CLAUDE.md fires no advisory. Filed as **CHECK-MEMBERS-ALL-KINDS** (open,
  the tick's one pickable entry). Revised ordering: CHECK-MEMBERS-ALL-KINDS (gate
  dispatches memory members) → MEMORY-KIND validation → flip. The read-family half of
  the hardcoding is carved out as the `(builtin-workspace-qualified-key)` fork above.
  DATUM (2026-07-03): the directive half of the hardcoding also shipped —
  DIRECTIVE-MEMBERS-ALL-KINDS (build 28df927; `collect_directive_members` ranges
  over `builtin_kind::definitions()`, verified src/main.rs:787). And the flip now
  gained a NEW discovery prerequisite: **WALK-IGNORE-DISCIPLINE**, which has now
  **SHIPPED** (build f419987 / chore 9a69e1c; verified on disk this tick: `ignore =
  "0.4"` in Cargo.toml, `collect_glob` walks via `ignore::WalkBuilder` at
  src/import.rs:493). The `**` discovery walk now honors `.gitignore` +
  always-excludes `.git`, so a future `**/CLAUDE.md` glob will not import a vendored
  dep's CLAUDE.md out of node_modules as a member. **The flip gate is now clear** —
  every engine prerequisite has drained; the sole remaining step is the human
  ceremony: flip both memory kinds' `governs` to the any-depth glob (curated embeds,
  gates green before commit), then verify no node_modules/.git members appear on a
  cascade vet.
  DATUM (2026-07-03, flip SHIPPED): the human ceremony landed — `chore` 6954475
  ("the memory kinds govern the whole tree — nested hierarchy live"): both memory
  kinds' `governs` now move to any-depth globs, proven on the adversarial cascade
  fixture (agents-md.memory: 0 members from node_modules' recharts AGENTS.md;
  real surface: 21). The bootstrap fence is **dissolved for memory** — CLAUDE.md
  is a governed member. Sole residual is the trailing-period `@import` in
  cascade's CLAUDE.md (grammar-nuance vs real-dead-import: UNVERIFIED, on the
  session ledger's verify list), not an engine gap.
