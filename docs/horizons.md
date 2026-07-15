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
  "Read verbs" (`impact` is specified there and not yet shipped — the binary's
  `explain` carries blast radius today).

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

- `(surface-authority-lock)` — RATIFIED 2026-07-03 ("surface authority is a
  declared posture, never a baked stance"); current home `specs/intent.md`
  invariant 5 and `specs/model/pipeline.md`, "Drift". The **drift re-cut**
  noted in the ratified Decision still rides behind the shipped lock — it
  re-enters here or the workshop when the lock proves the inversion.
