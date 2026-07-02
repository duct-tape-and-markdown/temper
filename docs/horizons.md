# Horizons — candidate intent, not yet intent

Product opportunities surfaced in design sessions, parked here until the human
ratifies one as a bite. **Nothing in this file is contract.** It lives in
`docs/`, not `specs/`, deliberately: plan reconciles `specs/` against code every
tick (`specs/90-spec-system.md`), so an un-ratified idea placed there would be
read as intent by the autonomous loop — a derived layer must never receive
intent the human hasn't authored (`00-intent.md` law 4). This file is upstream
of the corpus, not part of it.

Not `.flume/plan/open-questions.md` either: that file holds forks *blocking
already-filed work*. A horizon is pre-intent — no pending entry may cite one.

## The graduation ceremony

Each entry is keyed `(slug)` (the open-questions convention). Taking a bite:

1. The human authors the spec section(s) in an interactive session — new intent
   is never written by a phase (`90-spec-system.md`). The entry's fields are the
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
teaches the agent to operate the gate; the graph verbs (`why`, `requirements`,
`check`, `diff`) are the CLI's, and the agent shells out
(`50-distribution.md`). An **MCP server** carrying the same verbs was proposed
and is **parked**: it duplicates a transport the agent already has, adds a
server surface to version and secure, and the skill-taught CLI keeps one
vocabulary in one mouth. Reopen only with evidence the shell-out path fails
agents in practice (latency, output-parsing errors, permission friction) —
that evidence, not preference, is the bite condition.

## Entries

- `(impact-verb)` — **Blast radius as a verb.** `temper impact <member>`:
  deterministic tier-1 traversal answering "what strands if this member is
  removed or renamed" — the graph payoff `00-intent.md` already promises
  ("remove a load-bearing entity and the graph lights up every spec, binding,
  and code symbol that depended on it"), surfaced as a read verb and as the CI
  comment on harness-touching PRs. *Rents:* the graph (`00-intent.md`), read
  verbs (`20-surface.md`), CI placement (`50-distribution.md`). *Tensions:*
  none identified — pure tier-1. *Leaning:* smallest bite here; likely first.
  *Bite condition:* none beyond ratification; the graph it traverses exists.

- `(vet)` — **Check what you install, before you install it.** A verb aimed at
  the *consumer*: run the gate over a plugin or marketplace artifact pre-install
  (and audit what's already installed). `00-intent.md` positioning already says
  temper "can sit downstream of the others, checking what you installed" — this
  names the verb. *Rents:* the checker, built-in packages, `bundle`'s artifact
  shapes (`50-distribution.md`). *Tensions:* a vet verdict over a *stranger's*
  artifact has no author to teach — the guidance channel's framing may need a
  consumer voice; also risks reading as taste about others' work (law 2) unless
  it stays strictly the decidable tier. *Leaning:* do it; second wedge with a
  distinct audience. *Bite condition:* decide the verb's name and whether it is
  `check --harness` over an unpacked plugin or a distinct surface.

- `(graph-explorer)` — **A read-only rendered harness explorer.** Static HTML
  export of the graph: members by kind, requirement coverage, `satisfies`
  edges, drift state, blast radius on hover. Read-only by law: a GUI *editor*
  would be a second authored home and is ruled out (law 5 — the surface is the
  single authored home). Deterministic projection makes a render safe and
  regenerable; every exported page doubles as the launch demo's hero
  (`55-offering.md`). *Rents:* the graph, deterministic projection (law 5),
  the offering's demo posture. *Tensions:* keep it a *projection of real
  output* — a hand-curated visual is the drift failure as marketing
  (`55-offering.md` decision). *Leaning:* after `(impact-verb)` — the explorer
  renders what impact computes. *Bite condition:* decide the emission form
  (verb flag vs. reporter) so it joins the one-reporter-family model.

- `(lsp)` — **The gate as a language server.** The schema modeline is the
  keystroke placement today; `50-distribution.md` already says "served over LSP
  later." An LSP deepens it beyond frontmatter: hover = package guidance,
  go-to-definition across `satisfies` edges, rename over the harness — law 6
  (fearless refactoring) made interactive instead of batch. *Rents:* schema
  emission, the two-channel split (validation vs. docs — the medium enforces
  law 2), the graph. *Tensions:* rename is a *write* through a new door; it
  must route through the same drift-aware apply path, never a second writer.
  *Leaning:* medium-term; heaviest single build here. *Bite condition:*
  `apply`'s write path stable enough to sit under an interactive client.

- `(package-identity)` — **Versioning and provenance for shared packages.**
  Packages are first-class publishable artifacts (`50-distribution.md`) and
  project-authorable as peers (law 2) — community packages are the network
  effect. Missing is the thin identity layer: a version field, a provenance
  convention, compatibility semantics for a package a stranger binds. No
  bespoke registry pre-traction — ride git + marketplaces — but identity
  designed late forces a breaking migration on every published package.
  *Rents:* `bundle`, the package medium (`10-contracts.md`). *Tensions:*
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

- `(verifier-layer)` — **Scaffolding the `verified_by` side.** The model
  delegates behavior and checks only wiring — right, per law 3, but the
  author's most valuable question ("does this skill actually trigger?")
  currently exits the product. The idea: temper *projects* eval harnesses for
  common verifier shapes (skill-trigger evals over a prompt set, hook smoke
  tests) and reads pass/fail back as evidence — temper still never judges;
  execution does. *Rents:* `verified_by` (`00-intent.md`), projection.
  *Tensions:* the largest scope question in this file — this is adjacent to a
  second product (a test runner for harnesses), and the corpus deliberately
  ends at "wired, not passing." Ratifying it moves that boundary; that is an
  intent-level decision, not a feature. *Leaning:* wants its own design
  session before any spec text. *Bite condition:* an explicit human ruling on
  where temper's responsibility ends — scaffold only, orchestrate, or stay out.

- `(agent-agnostic-import)` — **More foreign formats at the on-ramp.** Each
  import source (beyond the Cursor `.mdc` correction that motivated the tool)
  is both a feature and an acquisition channel, and is what eventually earns
  intent's "(then agent-agnostic)" clause. Every format's layout is an external
  fact: cited per source, at the point of claim, or not encoded. *Rents:*
  import-as-migration (law 5), "migrate, with a fix" (`50-distribution.md`).
  *Tensions:* none structural; per-format cost is citation diligence.
  *Leaning:* demand-driven — add formats when a real migration asks. *Bite
  condition:* a named format with a citable layout and a user who wants in.
