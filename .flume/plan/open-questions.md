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

- `(workspace-scope)` — Does the config surface target a per-project `.claude/`,
  a managed mirror of the global `~/.claude`, or both? `import`/`check` sidestep
  this by importing from an explicit path argument, but `apply` write-back needs
  it decided. See `specs/20-surface.md`.

- `(yaml-writeback)` — Source frontmatter is YAML; the surface header is TOML. On
  write-back, re-emit YAML (normalizing — no comment-preserving YAML editor
  exists in Rust) or patch only changed fields? Leaning patch-only. Blocks
  anything in the `apply` path, not `import`/`check`. See `specs/20-surface.md`.

- `(surface-authority)` — Is the surface the source of truth (with `re-add` for
  drift) or a lens over canonical on-disk files? MVP treats it as source of
  truth; revisit if direct-harness editing proves the common path. See `specs/20-surface.md`.

- `(field-type-lattice)` — The field-primitive algebra (`10-contracts.md`, "The
  primitive algebra (decidable only)") lists `type` alongside
  `required`/`optional`/`pattern`, but the corpus never declares the **type
  vocabulary** the predicate ranges over (string? integer? boolean? list?) nor how
  a YAML/JSON scalar's source type maps onto it. The extractor also stringifies
  every scalar today (`extract.rs:265`), so a *sound* `type` check additionally
  needs the projection to preserve the source scalar type. Implementing requires
  inventing the lattice — an intent gap, human to author. Blocks only the `type`
  primitive; nothing shipped depends on it. See `specs/10-contracts.md`.

- `(harness-contract-provisioning)` — The harness-contract instance
  (`10-contracts.md`, "Roles and matching" + "`verified_by` — where behavior
  goes") binds author-named roles to artifacts. Unlike the per-kind artifact
  contracts (built-in defaults, `(contract-selection)` RESOLVED), a harness
  contract is *specific to one harness* and cannot be a built-in — so `check` must
  load an author-declared contract from a place the corpus never names (an
  `author.toml` field? a `contracts/` convention?). Also under-specified: what
  "the verifier is declared and *wired*" decides (the path exists? a referential
  resolve?). Blocks the entire role/`verified_by` layer; the artifact algebra
  ships without it. See `specs/10-contracts.md`.
