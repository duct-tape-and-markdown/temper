# Horizons — candidate intent, not yet intent

Product opportunities surfaced in design sessions, parked here until the human
ratifies one as a bite. **Nothing in this file is contract.** It lives in
`docs/`, not `specs/`, deliberately: plan reconciles `specs/` against code every
tick (`specs/process/spec-system.md`), so an un-ratified idea placed there would be
read as intent by the autonomous loop — a derived layer must never receive
intent the human hasn't authored (`specs/process/spec-system.md`). This file is upstream
of the corpus, not part of it.

Not `.flume/plan/open-questions.md` either: that file holds forks *blocking
already-filed work*. A horizon is pre-intent — no pending entry may cite one.

## The graduation ceremony

Each entry is keyed `(slug)` (the open-questions convention). Taking a bite:

1. The human authors the spec section(s) in an interactive session — new intent
   is never written by a phase (`specs/process/spec-system.md`). The entry's fields are the
   raw material for the spec's Decision: leaning → chosen, parked → rejected.
2. Mark the entry `RATIFIED → <spec file/section>` (or `DROPPED`, with why) and
   delete its body — the spec is now the home; a stale copy here would drift.
3. Plan picks the new intent up on its next tick. No horizon entry ever feeds
   plan directly.

Entry shape: **the opportunity** (one paragraph), **rents** (which existing spec
sections it builds on — an entry renting nothing is probably a second product),
**tensions** (where it rubs against the law), **leaning / parked** (the current
inclination and the alternatives held with reasons), **bite condition** (what
must be true or decided before ratifying).

External-fact discipline applies even here: an entry that asserts how a foreign
system behaves marks it `UNVERIFIED` until cited (`.claude/rules/collaboration.md`).

## Delivery posture — a standing note, not an entry

The corpus's delivery stance is **CLI instructed by skill**: the bundled skill
teaches the agent to operate the gate; the graph verbs (`check`, `explain`)
are the CLI's, and the agent shells out
(`specs/distribution.md`). An **MCP server** carrying the same verbs was proposed
and is **parked**: it duplicates a transport the agent already has, adds a
server surface to version and secure, and the skill-taught CLI keeps one
vocabulary in one mouth. Reopen only with evidence the shell-out path fails
agents in practice (latency, output-parsing errors, permission friction) —
that evidence, not preference, is the bite condition.

## Entries

- `(impact-verb)` — RATIFIED 2026-07-03; current home `specs/model/contract.md`,
  "Read verbs", amended 2026-07-15: `explain` is the one read verb and
  impact is its strand — the shipped unification is the intent, and the
  model's old peer-verb spelling was the stale side.

- `(vet)` — **Check what you install, before you install it.** A verb aimed at
  the *consumer*: run the gate over a plugin or marketplace artifact pre-install
  (and audit what's already installed). `specs/intent.md` positioning already says
  temper sits "downstream of both, checking what you installed" — this
  names the verb. *Rents:* the checker, built-in packages, `bundle`'s artifact
  shapes (`specs/distribution.md`). *Tensions:* a vet verdict over a *stranger's*
  artifact has no author to teach — the guidance channel's framing may need a
  consumer voice; also risks reading as taste about others' work (the spine rule) unless
  it stays strictly the decidable tier. *Leaning:* do it; second wedge with a
  distinct audience. *Bite condition:* decide the verb's name and whether it is
  `check --harness` over an unpacked plugin or a distinct surface.

- `(graph-explorer)` — **A read-only rendered harness explorer.** Static HTML
  export of the graph: members by kind, requirement coverage, `satisfies`
  edges, drift state, blast radius on hover. Read-only by law: a GUI *editor*
  would be a second authored home and is ruled out (invariant 7, read or
  written never both — `specs/intent.md`). Deterministic projection makes a
  render safe and regenerable; every exported page doubles as the launch
  demo's hero (`specs/distribution.md`). *Rents:* the graph, deterministic
  projection (invariant 3), the demo posture. *Tensions:* keep it a
  *projection of real output* — a hand-curated visual is the drift failure as
  marketing (`specs/distribution.md`). *Leaning:* after `(impact-verb)` — the explorer
  renders what impact computes. *Bite condition:* decide the emission form
  (verb flag vs. reporter) so it joins the one-reporter-family model.

  *Second strand — the observed graph (2026-07-26, session-argued):*
  `(impact-verb)`'s precondition is met, and the entry above renders only the
  **declared** graph. The strand: shade it by the **observed** one — the tap's
  per-machine log (`src/tap.rs`) read the way `explain`'s field strand already
  reads it (`src/telemetry.rs`). A declared member with no `InstructionsLoaded`
  record across N sessions is precisely `specs/intent.md`'s stated problem —
  "a skill that never triggers, a rule that fails to load" — and precisely what
  invariant 2 keeps out of `check` forever. The render shows the evidence and
  issues no verdict, which is invariant 8's split rendered rather than argued;
  `Verifier::Telemetry` (`src/compose.rs`) already types the same edge and is
  resolved-but-never-run, so the overlay is that verifier's readout, not a new
  authority. *Rents additionally:* the tap, the field strand, the telemetry
  verifier. *Tensions:* (a) the tap is local-locus and uncommitted, so an
  overlay render is **one machine's** evidence — publishing one as the demo
  hero asserts a single developer's session history as the project's, and the
  static entry above wants exactly that hero; (b) a **live localhost UI**, the
  form this arrived in, collides with the kernel's "still offline, still no
  runtime" (0032) and inherits the delivery-posture note's version-and-secure
  half, though not its duplicate-transport half — that objection was about an
  *agent* transport, and a human-facing view duplicates none; (c) the
  longitudinal value ("never loads") needs weeks of history, so the live crawl
  is the better demo and the accumulated view the better product — they are not
  the same feature and should not be costed as one. *Leaning:* static-first —
  regenerate on a `Stop` hook and let the page refresh itself, which is
  live-enough with no server, no port, and no auth question; the live form
  reopens only on evidence the static one is insufficient, per this file's own
  standing reopen condition. *Bite condition:* the emission-form decision above,
  plus a corpus carrying enough tap history for "never loaded" to mean
  something. Wiring is not the obstacle it first appeared: a telemetry verifier
  synthesizes its own tap hooks at emit, so any consumer that declares one is
  already recording. The obstacle is that the consumers which exercise temper
  hardest exercise the *structural* half — the standing ledger note of 07-20
  puts verifiers and the local commitment class in the un-field-tested half —
  so the history this strand reads has to be deliberately started, per consumer,
  before it accrues. This repo's own harness declares `context-arrives` as of
  2026-07-26 and is the first.

- `(lsp)` — **The gate as a language server.** The schema modeline is the
  keystroke placement today; `specs/distribution.md` already says "served over LSP
  later." An LSP deepens it beyond frontmatter: hover = package guidance,
  go-to-definition across `satisfies` edges, rename over the harness —
  fearless refactoring made interactive instead of batch. *Rents:* schema
  emission, the two-channel split (validation vs. docs — the medium enforces
  the spine rule), the graph. *Tensions:* rename is a *write* through a new
  door; it must route through the same drift-aware emit path, never a second
  writer. *Leaning:* medium-term; heaviest single build here. *Bite condition:*
  emit's write path stable enough to sit under an interactive client.

- `(package-identity)` — **Versioning and provenance for shared packages.**
  Packages are first-class publishable artifacts (`specs/distribution.md`) and
  project-authorable as peers (the spine rule) — community packages are the network
  effect. Missing is the thin identity layer: a version field, a provenance
  convention, compatibility semantics for a package a stranger binds. No
  bespoke registry pre-traction — ride git + marketplaces — but identity
  designed late forces a breaking migration on every published package.
  *Rents:* `bundle`, the package medium (`specs/model/contract.md`). *Tensions:*
  version-compatibility checking must stay decidable or stay out. *Leaning:*
  design the identity fields early, ship the ecosystem play later. *Bite
  condition:* first real external consumer of a project-authored package —
  or the decision to seed one.

- `(org-assembly)` — **One contract, many repos.** A platform team declares a
  shared package/assembly; every repo's harness checks against it in CI —
  harness governance at org scale, and the plausible commercial layer over a
  free single-repo gate. Composition of existing parts (a shared package + the
  CI placement); the new questions are distribution of the shared contract and
  reporting across repos. *Rents:* packages, CI placement, reporters.
  *Tensions:* central taste imposed on leaf repos is still law-2-clean only
  because adoption is the org's declared choice — the spec section must say
  where that choice is authored. *Leaning:* hold in view; not a now-thing.
  *Bite condition:* `(package-identity)` ratified first; an org-shaped user.

- `(verifier-layer)` — **Scaffolding the verifier side.** The model
  delegates behavior and checks only wiring — right, per "decidable only"
  (`specs/intent.md`), but the
  author's most valuable question ("does this skill actually trigger?")
  currently exits the product. The idea: temper *projects* eval harnesses for
  common verifier shapes (skill-trigger evals over a prompt set, hook smoke
  tests) and reads pass/fail back as evidence — temper still never judges;
  execution does. *Rents:* the verifier edge (`specs/model/contract.md`,
  "requirement"), projection.
  *Tensions:* the largest scope question in this file — this is adjacent to a
  second product (a test runner for harnesses), and the corpus deliberately
  ends at "wired, not passing." Ratifying it moves that boundary; that is an
  intent-level decision, not a feature. *Leaning:* wants its own design
  session before any spec text. *Bite condition:* an explicit human ruling on
  where temper's responsibility ends — scaffold only, orchestrate, or stay out.
  *Field evidence (2026-07-10):* a consumer's hook substring-matched
  "developer" and injected irrelevant docs every turn — a declared,
  deterministic trigger whose relevance was pure noise; "is this trigger
  signal?" is exactly the question that exits the product today.

- `(agent-agnostic-import)` — **More foreign formats at the on-ramp.** Each
  import source (beyond the Cursor `.mdc` correction that motivated the tool)
  is both a feature and an acquisition channel, and is what eventually earns
  intent's "(then agent-agnostic)" clause. Every format's layout is an external
  fact: cited per source, at the point of claim, or not encoded. *Rents:*
  import-as-migration, "migrate, with a fix" (`specs/distribution.md`).
  *Tensions:* none structural; per-format cost is citation diligence.
  *Leaning:* demand-driven — add formats when a real migration asks. *Bite
  condition:* a named format with a citable layout and a user who wants in.

- `(code-seam-joins)` — **The cross-landscape seam as a join, acknowledged in
  code.** A harness artifact publishes invariant packs (`[requirement.<name>]`
  — a dev-standards skill naming per-module invariants); the code where each
  invariant lives *acknowledges* it with a trace tag in a declared grammar
  (`// satisfies dev-standards.parser-strictness`), and the gate resolves both
  ends: delete the invariant and the tag dangles, gut the code and the demand
  dangles. Blast radius crosses the landscape boundary (`impact src/kind.rs`
  lights up harness artifacts). The tag moves with the code it annotates, so
  neither join end holds a fragile `file:line` coordinate. *Rents:* the join
  doctrine and the cross-landscape seam (both from the retired pre-kernel
  spec cut; the nearest current home is `specs/model/representation.md`,
  "Reach" — this makes "checked both directions" authored rather than
  resolution-shaped), set-scope predicates (one invariant, many code sites =
  a satisfier set; `count`/coverage apply unchanged), and the repo's own
  DO-178C trace-tag convention (`.claude/rules/rust.md`) — the one-way version
  already practiced, waiting for its mechanism. *Tensions:* "declared, never
  mined" (`specs/intent.md`, invariant 1) — a
  name-mention in a comment is prose; this stays legal only as a **deliberate
  tag in a grammar the code kind's extraction declares** (authored to be
  machine-read = a declaration; the code author writing it is the opt-in).
  Births the **code landscape kind** — extraction over source files, the
  largest vocabulary addition yet; this use case is its consumer. Tags are
  comments, paid twice (`rust.md`) — a join half is load-bearing enough to
  keep. *Leaning:* do it, after the corpus migration proves manifest
  authoring at scale; it is what the migration makes credible, never a gate on
  it. *Bite condition:* corpus migration shipped; then a design session for
  the tag grammar + the code kind's extraction shape.
  *Field evidence (2026-09-22, unratified, consumer report):* a consumer
  modeling a legacy web app pinned code at file grain and kept line
  citations as plain data. It asked for range-anchored pins. A range is
  decidable only between tags in this entry's declared grammar, never by
  line number, so sub-file grain lands here (see `(declared-input)` for the
  file-grain half).

- `(base-harness)` — **A standalone starter harness whose docs corpus is a
  temper program.** The external dogfood: a reference repo carrying doc/spec
  kinds (`system`, `flow`, `decision`, `term`), the shipped Claude Code
  kinds, and the spec → plan → build loop — components and processes tested
  together, graduating into the public starter plus its documentation. Full
  design material: `docs/base-harness-primer.md` (pre-intent, same standing
  as this file). *Rents:* layout content (0019), `install`'s conversion,
  `bundle`, the requirement machinery, the demo posture
  (`specs/distribution.md`). *Tensions:* the second-corpus clause
  (`specs/model/representation.md`, "Reach" — a second corpus is a feature,
  never a founding assumption); the meta-freeze; the public-prose register
  for its eventual public face. *Leaning:* do it, post-launch — it is the
  natural successor to the spec-corpus demo. *Bite condition:* v0.1 shipped,
  then a human ruling on second-corpus scope; the primer keys the remaining
  forks. *Field evidence (2026-07-13, human-approved):* a first cut is live
  at `examples/base-harness/` — five user-declared doc kinds, all green
  under `emit`/`check`; three product findings routed to `.flume/inbox.md`
  (SDK-phase fill check vs layout fills, `install --yes` re-run preview,
  nested-root discovery fencing). Deepened same day on the human's ruling:
  the docs tree is a **projected collection** — doc members composed, edges
  authored from member values, lifecycle as a typed `supersede()` — with
  the glossary kept as the one layout source; the `(lifecycle-encoding)`
  fork is settled twice over (kind partition + the field's own type).
  *Second cut (2026-07-15, human-ruled in session):* the kinds recomposed
  against the mark "a docs kind earns its type when a typed field can go
  false about the world, and its body composes from declared members" —
  systems contain `invariant` members, flows contain `step` members (each
  step's system an import, its edge a mention row; `participants` deleted
  as a field, rendered from the steps), decisions contain `alternative`
  members, and a `source` kind over a deliberately tiny governed `src/`
  makes `implemented-by` an edge the gate refuses when the file goes
  (verified firing: `graph.route`). Five product findings routed to the
  inbox (destructive emit reap on workspace spelling; the fence wrapper
  vs the model's unconstrained embedded rendering; no prose/blocks
  interleave; no mention adapter; embedded members unmentionable). Held
  back on purpose: a roll-up rendering helper (wait for recurrence) and
  edge fields on embedded kinds (mentions may simply be right). All six
  findings closed same day by the loop (six build commits, 07-15): fence-
  free rendering, the reap fix, embedded mentions, fill deferral, prose
  interleave (dff2db2's ruling; the example's `passage` wrapper deleted,
  projections byte-identical), and the nested-root discovery fence (the
  repo gate no longer counts the example's CLAUDE.md).
  *Third cut (2026-07-15, shipped 549969f):* the whole starter, organized
  by the five-domain architecture (primer §"domain architecture", ruled and
  calibrated in session) — five domain requirements (conduct/orientation/
  governance floored, falsifiability verified red/green), `operations`
  keyed `kind: skill` (the variance fix in the field), the DRY centerpiece
  live (facts.ts constants; one edit moved CLAUDE.md + SKILL.md +
  settings.json in one emit), a `paths`-gated verify skill, and the
  grow-harness governance procedure. Three product findings routed to the
  inbox: the `check .` half-gate (install.rs:88 hardcodes it into every
  adopted harness's reporter), composed mentions unable to target
  discovered members (blocks the script-edge demo), and `emit --into`
  re-root reaping live projections.

- `(field-reach)` — *UNRATIFIED draft (2026-09-22), session-proposed from a
  consumer field report; the human keeps or drops it.* **Reachability over
  declared field edges from declared roots.** Today's `graph.reachable` is
  the runtime-load closure: world → a live registration channel, then
  `@import` directives, capped at `MAX_IMPORT_HOPS` (`src/graph.rs`
  `live_members`). A kind with no registration counts as always live
  (`graph.rs:1006`). So in a user-declared corpus, a member reached only
  through a dead member stays silent. Example: roots A → B → C, plus an
  orphan D → C′. `degree(incoming ≥ 1)` catches D and never C′, because
  one-hop degree can't express transitive closure. The opportunity is an
  opt-in predicate: each selected member lies in the closure of a root
  selection over a declared field set. It is decidable (a graph closure,
  invariant 2) and well-defined over cycles. *Rents:* `contract.md`
  "clause" ("some predicates need whole-graph context … a reachability
  test"). The by-incidence field-set filter (0052), for the edge set the
  closure walks. `membership`'s precedent of a predicate naming a second
  selection (its target requirement) as a parameter, so roots are named
  the same way and selectors stay atomic. *Tensions:* (a) it must never
  merge into the default `reachable` clause. Folding field edges into the
  runtime closure would silence a true dead-registration finding: a skill
  whose trigger is dead but which a live rule `routes_to` still never
  loads. That breaks invariant 6. It needs its own name and rule id
  (`reached-from`, not `reachable`). (b) Invariant 1's density bound. A
  clause demanding every member be field-reachable is fine when the
  author declares it. Shipped in a default contract, it becomes the
  declaration-density demand the invariant forbids, so it stays out of
  every default contract. (c) Invariant 5. If a shipped package ever
  carries it, it enters advisory. (d) It presupposes that field-edge cycles
  are legal. Under today's `graph.acyclic` scope (inbox, "acyclicity fires
  on declared field edges") the cyclic flows it most wants to judge are
  refused before it runs. *Leaning:* do it as a vocabulary addition, not a
  kernel change: one predicate taking `roots` (a requirement name, like
  `membership`'s target) and `via` (a field set, like 0052's filter).
  *Parked:* adding the closure to `degree` as a mode (it overloads a
  local count with a global walk). Recommending the demote-to-plain-field
  workaround (it loses the edge from `explain`'s enumeration). *Bite
  condition:* the acyclicity defect is fixed. 0052's field-set filter has
  shipped. Then ratification. Second-corpus scope is settled for this
  entry (human-ruled 2026-09-22): opt-in, domain-neutral predicates built
  from existing nouns meet `representation.md` "Reach". Only kinds or
  defaults designed for another corpus are held back, so a non-harness
  first driver doesn't block it.

- `(declared-input)` — *UNRATIFIED draft (2026-09-22), session-proposed from
  a consumer field report; the human keeps or drops it.* **A member
  declares a file it rests on, and the lock fingerprints it without
  projecting it.** The only way today to get "the code under this claim
  changed" is to `include()` the file into prose. That splices the bytes
  into the projection (a consumer projected ~210KB of copies for five
  files) just to buy the `import_hash` row that `prose.include-stale`
  compares (`src/drift.rs` `source_dep_stale_from_doc`). The opportunity is
  the same source-dependency row with no splice: a declared input of the
  member. When its bytes move, the finding names the dependent member, the
  file, and the remedy, as `pipeline.md` "Drift" already shapes an
  authored-source freshness fact. *Rents:* `pipeline.md` "Drift" (fact one:
  an authored source differing from its provenance row). The existing
  include and layout-import source-dependency families. Invariant 8: the
  finding routes the author back to re-verify, and the author, not temper,
  judges whether the claim still holds. *Tensions:* (a) a re-emit refreshes
  the hash with no visible diff. With `include()`, the projection diff at
  least shows what changed. With this row, the lock line is the only
  review surface. So a re-emit can bless a claim nobody re-checked, and the
  finding's remedy text must not read as "re-emit and you're done". (b)
  Severity. Source-dependency findings are fixed `warn`, and the dial
  reaches clause labels only (`src/dial.rs:95`). An author who wants this
  to gate can't declare it, which sits badly with the spine rule and
  `pipeline.md`'s "how loudly … is the author's declared severity". That is
  worth its own ruling whether or not this entry lands. (c) A
  **range-anchored** pin. Hashing a line range false-fires on every edit
  above the range (a gate that cries wolf, invariant 2). Locating the range
  by matching content is mining (invariant 1). A range is decidable only
  between declared tags, and that is `(code-seam-joins)`'s code kind, so
  sub-file grain is parked there. (d) This is not `(external-commitment)`.
  That fork (`.flume/plan/open-questions.md`, human-parked 09-03) gives the
  *ground member itself* a committed, never-emitted file locus with a lock
  row and a byte hash. That kernel change to `locus` is also what would
  retire the per-file `locus.undeclared-member` notes a read-only ground
  kind draws today. The fingerprint belongs on the dependent pin because
  the finding must name whose claim is at risk. A hash on the ground member
  names no dependent. *Leaning:* do the input declaration, a vocabulary and
  pipeline addition with no kernel change. Leave the ground-member locus to
  `(external-commitment)`, and cite this report there as field evidence
  when that fork is next argued. *Parked:* a fingerprint on the ground
  member (that is the fork above). Line-number ranges (fragile, and the
  problem `(code-seam-joins)` was built to avoid). Promoting
  `include-stale` to `error` as the fix (that bakes a severity rather than
  letting the author declare one). *Bite condition:* (b) was ruled on
  2026-09-22 (drift findings become dialable), and its mechanism is drafted
  in `docs/proposals/drift-severity-is-a-clause.md`. That lands first. Then
  the two graph defects in `.flume/inbox.md` (acyclicity scope, the
  `degree` field filter), then ratification. Second-corpus scope doesn't
  bind (ruled 2026-09-22, see `(field-reach)`). *Evidence status:* observed
  at 0.0.18 in a copy of the consumer's prototype. The five pin projections
  total 209,554 bytes and copy 208,622 bytes of code. `temper check .`
  draws 8 `locus.undeclared-member` notes, 7 of them on the `source` ground
  kind (coverage: "source (7: 0 declared, 7 undeclared)"), for example
  "document `code/_init.html` sits at the `source` kind's governed locus
  but the lock declares no member for it — `emit` will never maintain it
  and `guard` never bound it, yet Claude Code loads it". Changing one byte
  of `code/_init.html` and re-running `check` without `emit` adds only
  "! prose include target `code/_init.html` (referenced by `pin:_init`) no
  longer matches the lock's fingerprint — the target changed and `emit` has
  not run; re-emit to reconcile". In a minimal repro with no other failing
  clause, the same one-byte edit leaves `check` at exit 0. The remedy text
  is "re-emit to reconcile", which is the reading tension (a) warns
  against.

- `(surface-authority-lock)` — RATIFIED 2026-07-03 ("surface authority is a
  declared posture, never a baked stance"); current home `specs/intent.md`
  invariant 5 and `specs/model/pipeline.md`, "Drift". The **drift re-cut**
  noted in the ratified Decision still rides behind the shipped lock — it
  re-enters here or the workshop when the lock proves the inversion.
