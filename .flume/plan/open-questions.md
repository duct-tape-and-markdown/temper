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

- `(extraction-source-not-mechanism)` — RESOLVED (human ruling this session;
  `specs/15-kinds.md`, "The extraction algebra — the soundness boundary, as data":
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
  (`specs/10-contracts.md` Decision: "a contract is identified by its path/role,
  not an internal name"). The hand-applied chore dropped `MissingName` and made
  `Contract.name` default to the file stem when the data file declares none
  (kept as `String`, not `Option`, since a display label always exists) — the
  curated nameless `contracts/skill.anthropic.toml` now loads as `skill.anthropic`.
  Chain head SKILL-CONTRACT-TEMPLATE is now `open`. Kept as the decision record;
  no dependent still waits on it.

- `(regex-crate)` — RESOLVED (`specs/10-contracts.md` Decision: "`allowed_chars`,
  not a general `pattern` clause"). `regex` was already sanctioned for *solved
  mechanics*; the live decision is to **not** expose an arbitrary `pattern =
  "<regex>"` clause — it is expressive enough to be an unsound proxy. The
  author-facing charset predicate caps at `allowed_chars` (`ranges` + `chars`, e.g.
  `[a-z0-9-]`); a genuine *format* need gets a narrow named predicate, never a
  general regex clause. Kept as the decision record; no dependent still waits.

- `(contract-selection)` — RESOLVED (`specs/20-surface.md` Decision: "contract
  selection is by artifact kind"). `check` maps each artifact to the built-in
  contract for its kind (skill → `contracts/skill.anthropic.toml`, rule →
  `contracts/rule.toml`), embedded as defaults. A per-workspace override is a
  later extension, not the default. Unblocks the rule artifact kind.

- `(skill-ref-syntax)` — RESOLVED (`specs/45-governance.md` Decision: "a reference
  is a declared edge on the surface, never grepped prose"). A reference is a
  **declared structured field** authored on the surface (the reference syntax),
  projected alongside any prose; the graph is built from these edges — never
  inferred by grepping prose for names/paths (the unsound prose-grep
  `10-contracts.md`'s referential rule forbids, the exact `companion-refs`
  unsoundness). So no prose-grep companion-ref check ships; a decidable referential
  clause runs only over a declared edge field. Kept as the decision record; its
  build (edge extraction + the graph) is the graph-scope frontier, downstream of a
  graph foundation.

- `(model-declaration-format)` — RESOLVED + now CARRIED (`specs/40-composition.md`
  "Declaring a custom kind" + its Decision "a custom kind is declared in `temper.toml`,
  extraction and all"). The domain model is **not** a separate declared format: a spec
  is a **custom kind** (`15-kinds.md`) whose entities are declared by the kind's
  extraction and whose relationships are declared edges (`45-governance.md`), declared
  under `[kind.<name>]` in `temper.toml` like any custom kind. `05-model.md` supplies the
  corpus's model *content* in prose; the *mechanism* is the kind system, not a
  `model.toml`. The format the old fork was "forwarded to but never carried" is now the
  concrete `[kind.<name>]` surface, built by the KIND-* chain (KIND-DECLARATION-PARSE …
  KIND-EDGE-RELATIONSHIPS). Kept as the decision record; no dependent still waits.

- `(workspace-scope)` — RESOLVED (`specs/20-surface.md` Decision: "the workspace is
  per-project"). The surface targets a **per-project** harness — the `.claude/` +
  co-located artifacts of one project, located by the explicit path `import`/`check`
  already take. Rejected (for now): mirroring the global `~/.claude`, or both; the
  global config is a later landscape root, not a redesign. Was the last fork gating
  the `apply`/`install` write-back path — now fork-free.

- `(yaml-writeback)` — RESOLVED, then SUPERSEDED (`specs/20-surface.md` Decision:
  "the projection is re-emitted; the surface is patched"). The original resolution
  (patch changed YAML fields surgically, never re-emit) was load-bearing when
  `.claude/` was a peer surface humans hand-curated. Under the surface-language
  model the projection is *generated* output: `apply` re-emits it deterministically
  (nothing of the human's in it to lose — content lives in the surface), and only
  the surface's own TOML is patched format-preserving (`toml_edit`). YAML exists
  only on the generated side. Kept as the decision record.

- `(surface-authority)` — RESOLVED (`specs/20-surface.md` Decision: "the surface is
  the source of truth"). The composition surface is canonical; `.claude/` + `specs/`
  are a projection of it (`apply`), and direct on-disk edits are reconciled back with
  `re-add`. The read-only-lens framing was rejected (it contradicts law 7 and strands
  fearless refactoring). With `(yaml-writeback)` + `(workspace-scope)` now both
  RESOLVED, the `apply` path is fork-free.

- `(field-type-lattice)` — RESOLVED (`specs/10-contracts.md` Decision: "the `type`
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
  *Home/selection* (`specs/40-composition.md` Decision: "the author-declared contract
  lives in `temper.toml`, layered"): an optional `temper.toml` at the project root
  layers over the by-kind built-in floor and holds adoptions, overrides, and the
  harness roster — rejected alternatives: a field in the *generated* `author.toml`,
  or the shipped templates as the author's home. *`verified_by`* (`specs/10-contracts.md`,
  "`verified_by` — where behavior goes"): "wired" is a **referential** clause — the
  named verifier must *resolve* (test target / CI job / path exists) or admissibility
  fails; a string-present check was rejected (a dangling verifier is a silent no-op).
  SHIPPED: the whole role/`verified_by`/`temper.toml` layer is on disk — `compose.rs`
  layers an optional `temper.toml` (adopt / extend / override / severity-flip) over the
  by-kind floor and parses the `[role.*]` roster; `roster.rs` runs selection +
  `conforms-to` + admissibility (including `verified_by` resolving to a real path); all
  wired into `check` in `main.rs`. Kept as the decision record; no dependent still waits.

- `(binary-bootstrap)` — RESOLVED (`specs/50-distribution.md` Decision: "acquisition
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
  analogy), not an authored index. `specs/20-surface.md` now names it `lock.toml`
  ("The surface: a contract over its contents"; the topology diagram), superseding the
  `author.toml`↔`temper.toml` name collision. SHIPPED as RENAME-ROLLUP-LOCK —
  `src/import.rs` writes `lock.toml` (`LOCK_FILENAME`), `src/drift.rs` reads it, and
  zero `author.toml` references remain in `src/`. Kept as the decision record; no
  dependent still waits.

- `(decision-marker-predicate)` — RESOLVED (`specs/10-contracts.md`, structural
  primitives): `section_contains` `{heading, marker}` (every section whose heading
  starts with the declared text carries the declared marker) is now enumerated in
  the predicate vocabulary's home — the deliberate language addition law 3
  requires, authorized by `15-kinds.md`'s worked example and now carried.
  decisions-name-alternatives becomes fileable build work once the spec kind's
  package exists (downstream of the surface-language/package-model machinery).
  SHIPPED as SECTION-CONTAINS-PREDICATE — `Predicate::SectionContains {heading,
  marker}` is parsed in `src/contract.rs` and decided in `src/engine.rs`. Kept as
  the decision record; no dependent still waits.

- `(read-verbs)` — RESOLVED (`specs/20-surface.md` Decision: "the CLI gains a read
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
  yet `specs/20-surface.md` ("The member document") and `specs/45-governance.md` call
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
  DATUM (2026-07-02): `specs/45-governance.md` gained the Decision "coupling
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
  `specs/55-offering.md`); it does NOT touch `.claude/**` or `CLAUDE.md` (the
  chore(harness) boundary holds). CHANGELOG.md: fileable as a root stub with an
  Unreleased section now (the alive-signal, `specs/55-offering.md`); release
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
  **provider** axis (`specs/15-kinds.md` Decision "kind identity carries a
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
  qualification. FILED: PROVIDER-KEY-PARSE (shipped c52df4f); EMBED-NESTED-WALK
  as pending (open, 2026-07-02 inbox drain — build.rs tolerates the nested layout
  + builtin_kind lookups resolve bare→unique, the build-side predecessor to the
  file-move); BINDING-QUALIFY as pending (parked on EMBED-NESTED-WALK + the human
  file-move); the file-move + provider lines are the human follow-up (2), not a
  build entry.
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

- `(project-name)` — RESOLVED, provisionally (`specs/55-offering.md` Decision:
  "the name stays `temper`, carried on scoped registries"). Keep the name; the
  contested registries are routed around: crate `temper-cli` (binary stays
  `temper`), npm scoped/prefixed, own Homebrew tap — the audience installs via
  npm/brew/plugin, never `cargo install`. Costs accepted eyes-open (someone
  else's `temper` crate; search mindshare shared with Temper-the-language).
  Two riders: the keep is **reaffirmed at launch** (the last cheap rename
  moment), and a **USPTO screen / Temper Systems non-objection** is the
  human due-diligence item before launch. PACKAGING-CHANNELS uses the scoped
  names. Kept as the decision record.

- `(kind-artifact-format)` — RESOLVED (`specs/20-surface.md` Decision: "a kind
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
  (`specs/15-kinds.md` Decision: "no body-mined references — the `references`
  primitive is retired"; `specs/00-intent.md` law 8). The earlier resolution
  declared a per-kind reference *normalization* (`strip_suffix = ".md"`,
  `` `15-kinds.md` `` → `15-kinds`) for a backtick-filename syntax **mined from
  the member body**. Law 8 retires the whole body-mining primitive: relationships
  range over **declared structured fields only**, never grepped prose — backtick
  file mentions are typography, permanently. So the shipped `strip_suffix`
  machinery is not wired into the dogfood; it is **removed** (filed
  REFERENCES-RETIRE: drop `Primitive::References` + `strip_suffix` +
  `backtick_filename_refs` + `is_filename_reference` from `src/kind.rs`). The spec
  corpus's real edges are the header `[edge.*]` / `satisfies` declarations of the
  classed corpus (`specs/90-spec-system.md`), never extracted from bodies. Kept as
  the decision record; the fork is closed by the retirement, not by wiring.

- `(default-assembly-as-data)` — OPEN. The built-in floor's zero-config
  adoption (a bare harness gets skill/rule checked with no `temper.toml`) is
  today **engine behavior**, the one deliberate built-in/custom asymmetry the
  unification kept. The proposed rung: the floor becomes an **embedded default
  assembly** — shipped declared data beside `kinds/` and `packages/`, same
  source-not-mechanism move — so "what temper does with zero config" is
  readable, cited, and overridable rather than coded. Recommendation: adopt;
  needs its `specs/40-composition.md` Decision (the floor is data), then a
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
  MEMORY-KIND recipe: **FILED** as pending MEMORY-KIND (parked) 2026-07-02, the
  tick after DECLARED-FRONTMATTER-ADAPTER shipped (fab4f79). The `memory` kind is
  the first pure-data managed kind and the recipe's proof — a curated
  `kinds/memory/KIND.md` + `packages/memory.anthropic/` (humans author, cited to
  code.claude.com/docs/en/memory): markdown, no frontmatter, dual root locus per
  `specs/15-kinds.md`. Parked because those curated files sit outside build's
  fence (a human authors them first) and the no-frontmatter format + `@path` edge
  primitive are deliberate closed-vocab additions to sanction. CORRECTION to the
  earlier recipe framing: `@path` imports are format-EXECUTED reference syntax
  (law 8's carve-out, `00-intent.md`) — their *own* edge extraction primitive, NOT
  EXTRACTION-VOCAB-GAPS's fenced/key-path, so MEMORY-KIND does **not** revive that
  entry (it stays deferred on its own no-consumer terms). Shipping MEMORY-KIND
  unlocks this flip ceremony.
