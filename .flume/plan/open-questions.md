# Open questions

Product/architecture forks not yet settled. Each is keyed with a `(slug)` so a
pending entry can declare `dependsOnForks: ["slug"]` and be held until resolved.
Mark a line `RESOLVED` (and record the decision) to unblock its dependents.

None of these block the queued contract-engine chain: it ships the in-crate
decidable subset and embeds the bundled skill template as the default. The forks
gate *extensions* (a full regex predicate, a contract-selection override, a
declared skill-reference clause) and the later `apply` write-back path.

- `(regex-crate)` — The primitive algebra lists `pattern` (regex), but `regex` is
  not in the sanctioned crate set and the codebase deliberately avoids it. Add
  `regex` to the sanctioned set for a real `pattern` primitive, or restrict to
  in-crate decidable predicates (the `allowed_chars` charset covers the skill
  template's `[a-z0-9-]`)? Blocks only the full `pattern` clause, not the engine.
  See `specs/10-contracts.md`.

- `(contract-selection)` — `check [<workspace>]` validates "against the active
  contract", but the corpus doesn't pin how `check` *locates* it: a CLI flag, a
  `contracts/` directory convention, or a field in `author.toml`? CHECK-CUTOVER
  defaults to the embedded built-in skill template; the override/selection
  mechanism waits on this. See `specs/20-surface.md`.

- `(skill-ref-syntax)` — The rejected `companion-refs` rule grepped prose, which
  is unsound (`10-contracts.md` referential clause: admissible *only* over a
  precisely declared reference syntax). Should a decidable referential clause for
  skills exist, and if so what reference syntax does the author declare (an
  explicit `@path`, a fenced block)? Until declared, no companion-ref check
  ships. See `specs/10-contracts.md`.

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
