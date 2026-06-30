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

- `(regex-crate)` — The primitive algebra lists `pattern` (regex), but `regex` is
  not in the sanctioned crate set and the codebase deliberately avoids it. Add
  `regex` to the sanctioned set for a real `pattern` primitive, or restrict to
  in-crate decidable predicates (the `allowed_chars` charset covers the skill
  template's `[a-z0-9-]`)? Blocks only the full `pattern` clause, not the engine.
  See `specs/10-contracts.md`.

- `(contract-selection)` — RESOLVED (`specs/20-surface.md` Decision: "contract
  selection is by artifact kind"). `check` maps each artifact to the built-in
  contract for its kind (skill → `contracts/skill.anthropic.toml`, rule →
  `contracts/rule.toml`), embedded as defaults. A per-workspace override is a
  later extension, not the default. Unblocks the rule artifact kind.

- `(skill-ref-syntax)` — The rejected `companion-refs` rule grepped prose, which
  is unsound (`10-contracts.md` referential clause: admissible *only* over a
  precisely declared reference syntax). Should a decidable referential clause for
  skills exist, and if so what reference syntax does the author declare (an
  explicit `@path`, a fenced block)? Until declared, no companion-ref check
  ships. See `specs/10-contracts.md`.

- `(model-declaration-format)` — `30-landscapes.md` ("The spec landscape: a
  declared model + bound prose") says the author *declares the domain model* —
  entities, relationships, invariants — as structure, and the dependency graph /
  blast radius (build-order step 2) derives from it. But the corpus never pins
  the **declaration format**: a dedicated `model.toml`, frontmatter
  `owns:`/`binds:` markers per spec, or something else? The graph and the
  cross-landscape seam (spec ⟷ code) can't be filed until this is authored.
  Intent gap — human to author into the spec, not plan to invent. See
  `specs/30-landscapes.md`.

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
  stringify shortcut is corrected before the primitive ships). Dependents filed:
  TYPED-EXTRACTION → TYPE-PRIMITIVE (pending.json).

- `(harness-contract-provisioning)` — RESOLVED, both halves.
  *Home/selection* (`specs/40-composition.md` Decision: "the author-declared contract
  lives in `temper.toml`, layered"): an optional `temper.toml` at the project root
  layers over the by-kind built-in floor and holds adoptions, overrides, and the
  harness roster — rejected alternatives: a field in the *generated* `author.toml`,
  or the shipped templates as the author's home. *`verified_by`* (`specs/10-contracts.md`,
  "`verified_by` — where behavior goes"): "wired" is a **referential** clause — the
  named verifier must *resolve* (test target / CI job / path exists) or admissibility
  fails; a string-present check was rejected (a dangling verifier is a silent no-op).
  The whole role/`verified_by`/`temper.toml` layer is now fork-free — frontier in
  `state.md`, to be decomposed into entries by a follow-on plan tick.

- `(binary-bootstrap)` — RESOLVED (`specs/50-distribution.md` Decision: "acquisition
  rides the ecosystem's package managers"). Ship the prebuilt binary through npm with
  platform-specific `optionalDependencies` (the common `.claude/`-project route), plus
  standalone release binaries, Homebrew, and `cargo install`, channel auto-detected;
  a single bespoke installer and the assume-globally-PATH'd-binary route were rejected.
  Fail-loud is intrinsic — a missing platform binary is an install error, never a
  silent skip. Kept as the decision record; gates packaging work, not the engine.

- `(spec-landscape-kind)` — RESOLVED (mostly), by the human authoring `specs/15-kinds.md`
  ("Kinds — the extraction algebra and the kind system"). Of the three original fronts:
  **(1)** is the **spec corpus a checked artifact kind, scanned by `import`?** — RESOLVED:
  `15-kinds.md` makes `spec` a *custom* kind (provenance: author-defined, not a harness
  format) governing `specs/*.md`, with a worked-example extraction (ATX headings, `## Decision`
  blocks, backtick-filename refs) and contract (max_lines + two clauses). The `spec`-kind
  build-out is filed: SPEC-KIND-IR → IMPORT → WORKSPACE → GATE (pending.json). **(3)** the
  **referential `references-resolve` clause** — RESOLVED: `15-kinds.md` *declares* the spec
  landscape's reference syntax (the backtick-wrapped `` `NN-name.md` ``), the precondition
  `10-contracts.md`'s referential primitive needed; references-resolve is now fork-free
  build work (state.md frontier). **(2)** the `section_contains` / decisions-name-alternatives
  **predicate** is NOT resolved — carved out as `(decision-marker-predicate)` below.
  `contracts/spec.toml` remains untracked human territory; plan neither writes nor commits
  it (the GATE entry is `parked` until a human commits it).

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
