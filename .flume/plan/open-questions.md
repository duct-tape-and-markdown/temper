# Open questions

Product/architecture forks not yet settled. Each is keyed with a `(slug)` so a
pending entry can declare `dependsOnForks: ["slug"]` and be held until resolved.
Mark a line `RESOLVED` (and record the decision) to unblock its dependents.

None of the open forks block slice 1: `import`/`check` take an explicit
`<harness-path>` and never write back to the harness, so the `apply`-path and
workspace-location decisions stay deferred.

- `(workspace-scope)` — Does the config surface target a per-project `.claude/`,
  a managed mirror of the global `~/.claude`, or both? Slice 1 sidesteps this by
  importing from an explicit path argument, but `apply` write-back needs it
  decided. See SPEC §9.

- `(yaml-writeback)` — Source frontmatter is YAML; the surface header is TOML. On
  write-back, re-emit YAML (normalizing — no comment-preserving YAML editor
  exists in Rust) or patch only changed fields? Leaning patch-only. Blocks
  anything in the `apply` path, not `import`/`check`. See SPEC §9.

- `(surface-authority)` — Is the surface the source of truth (with `re-add` for
  drift) or a lens over canonical on-disk files? MVP treats it as source of
  truth; revisit if direct-harness editing proves the common path. See SPEC §9.
