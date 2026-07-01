# Open questions

Product/architecture forks not yet settled. Each is keyed with a `(slug)` so a
pending entry can declare `dependsOnForks: ["slug"]` and be held until resolved.
Mark a line `RESOLVED` (and record the decision) to unblock its dependents.

The forks below gate *extensions* (a full regex predicate, a contract-selection
override, a declared skill-reference clause) and the later `apply` write-back
path — not the contract-engine chain, which ships the in-crate decidable subset
and embeds the bundled skill template as the default.

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

- `(yaml-writeback)` — RESOLVED (`specs/20-surface.md` Decision: "write-back patches
  changed fields, never re-emits"). `apply` patches only the changed fields in place
  (TOML via `toml_edit`, YAML frontmatter by surgical field patch), leaving every
  untouched byte exactly as written. Rejected: re-emitting a header from scratch —
  no comment-preserving YAML editor exists in Rust, so a full re-emit is the lossy
  round-trip law 5 forbids. Unblocks the `apply` path.

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

- `(decision-marker-predicate)` — The spec contract's **decisions-name-alternatives**
  clause (`15-kinds.md` worked example, `contracts/spec.toml`: every `## Decision` carries
  a `Rejected` marker) rests on a predicate `10-contracts.md`'s closed *predicate* algebra
  does not enumerate — a "a section matching a heading pattern contains a marker" primitive
  (`section_contains { heading, marker }`). `15-kinds.md` adds the matching *extraction*
  primitive (the `## Decision` block, heading + body) and names the clause, but explicitly
  defers a missing predicate to a "deliberate vocabulary addition (`10-contracts.md`)" —
  and `10-contracts.md`'s algebra (the predicate-vocabulary owner, per DRY/`90-spec-system.md`)
  still lists only `max_lines` / `require_sections` / `must_define` structurally. So the
  predicate is authorized to *exist* but is not yet *enumerated in its home spec*. Law 3:
  adding a predicate is a deliberate language change — human to enumerate `section_contains`
  in `10-contracts.md`'s primitive algebra. Until then decisions-name-alternatives does not
  ship (the `max_lines` spec clause ships without it). Distinct from references-resolve,
  whose predicate *category* `10-contracts.md` already enumerates (referential).

- `(read-verbs)` — The traversal payoff `00-intent.md` promises as prose ("remove a
  load-bearing entity and the graph lights up every … — the blast radius") and the
  inbox's proposed `temper why <artifact>` (forward `satisfies → means` + rationale)
  and `temper requirements` (reverse `requirement → satisfiers` = blast radius) are
  READ verbs over post-coverage/graph data — high payoff, read-only, no engine change.
  But `20-surface.md`'s **CLI surface** enumerates only `import`/`check`/`diff`/`apply`/
  `re-add`/`bundle`/`install`/`schema` — it does *not* list `why`/`requirements`.
  Adding a CLI verb the CLI-surface spec does not name is inventing surface
  (`collaboration` rule). Human to decide whether the CLI surface gains read/traversal
  verbs and exactly what each exposes; until then they are not fileable as build work.

- `(reference-id-normalization)` — The spec kind's **references-resolve** clause
  (`15-kinds.md` worked example) is the graph-scope frontier, but it does not yet run:
  the graph scope (`graph::check`/`acyclic`/`degree`) ranges only over the `by_kind`
  map `main.rs` assembles from `skill`+`rule` — custom-kind features are computed in a
  separate loop and never added, so a `[[kind.spec.relationships]]` edge (`from = "spec"`)
  finds no sources and is inert. Wiring custom-kind features into `by_kind` is clear engine
  work — but it exposes a *soundness* fork: a spec reference is filename-shaped
  (`` `15-kinds.md` `` — the declared syntax, extracted with the extension) while a spec
  artifact's id is the file **stem** (`15-kinds`, per `import::import_custom_unit`). Exact-
  string resolution (what `graph` does for `routes_to`) would dangle *every* spec reference —
  a false positive on clean input, the exact failure law 3 forbids. So resolution needs a
  rule mapping `NN-name.md` → the `NN-name` id, and *which* rule is a real decision: strip a
  trailing `.md` only, strip any single extension, or match `id == value || id + ".md" ==
  value`? A too-loose fallback could **mask** a genuine dangling reference (e.g. collapse
  `standards.md` onto skill `standards`), and masking a true positive is as unsound as
  forging one. Law 3: this is a deliberate resolution-semantics choice a human settles, not a
  phase-invented normalization. Until then references-resolve does not ship (the `spec` kind's
  `max_lines` clause ships without it, as it does today in `temper.toml`). Distinct from
  `(decision-marker-predicate)`, which is a missing *predicate*; this is a missing *resolution
  rule* for a predicate category (`referential`) `10-contracts.md` already enumerates.
