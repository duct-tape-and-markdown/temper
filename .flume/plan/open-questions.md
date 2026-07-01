# Open questions

Product/architecture forks not yet settled. Each is keyed with a `(slug)` so a
pending entry can declare `dependsOnForks: ["slug"]` and be held until resolved.
Mark a line `RESOLVED` (and record the decision) to unblock its dependents.

The forks below gate *extensions* and the code↔spec reconciliation to the
package/assembly/kind model — not the shipped contract engine, which today still
embeds the built-in contracts from `contracts/*.toml` under the retired
vocabulary (`template`, requirement-typing `contract`) pending that migration.

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
  `author.toml`↔`temper.toml` name collision. Filed as RENAME-ROLLUP-LOCK — filename
  plus the docstrings/comments naming it; only `src/import.rs` writes it and nothing
  reads it back outside import's own tests, so no behavior change and no `.temper/`
  topology move (the `<into>` dir is the `.temper/` contents root, so `<into>/lock.toml`
  *is* `.temper/lock.toml`). Kept as the decision record; no dependent still waits.

- `(decision-marker-predicate)` — RESOLVED (`specs/10-contracts.md`, structural
  primitives): `section_contains` `{heading, marker}` (every section whose heading
  starts with the declared text carries the declared marker) is now enumerated in
  the predicate vocabulary's home — the deliberate language addition law 3
  requires, authorized by `15-kinds.md`'s worked example and now carried.
  decisions-name-alternatives becomes fileable build work once the spec kind's
  package exists (downstream of the surface-language/package-model machinery).
  Kept as the decision record.

- `(read-verbs)` — RESOLVED (`specs/20-surface.md` Decision: "the CLI gains a read
  family — `why` and `requirements`"). Two **read-only traversal verbs** over data
  `check` already computes: `temper why <member>` walks the requirement↔`satisfies`
  edge forward (requirements filled + rationale, governing package, edges);
  `temper requirements [<name>]` walks it in reverse (satisfier set, coverage state,
  blast radius of a removal). Projections, never gates — no new engine semantics, no
  non-zero exit on findings. Rejected: `check` flags as a query surface; a general
  `query` verb. Fileable as build work **after** the surface-language migration,
  once coverage + graph data exist to read. Kept as the decision record.

- `(kind-harness-axis)` — OPEN. Kind identity has no harness axis: bare
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

- `(reference-id-normalization)` — RESOLVED (`specs/15-kinds.md` Decision:
  "reference resolution is declared by the kind, never guessed by the engine").
  The kind's declared reference syntax carries its own **normalization** — the
  deterministic mapping from extracted reference value to member id, declared
  data beside the syntax it belongs to. The spec kind's backtick-filename syntax
  declares `strip_suffix = ".md"` (`` `15-kinds.md` `` → `15-kinds`); the engine
  applies exactly the declared rule, then demands an **exact** id match.
  Rejected: an engine-global normalization (a baked-in guess, wrong for the next
  kind's syntax); fallback multi-matching (`id == value || id + ".md" == value` —
  a loose fallback can *mask* a genuine dangling reference, and masking a true
  positive is as unsound as forging one). The engine work this fork was holding —
  wiring custom-kind features into the graph's `by_kind` map so declared edges
  find their sources — is now fileable, with references-resolve downstream of it.
  Kept as the decision record.
