# `author` — a typed maintenance surface for the Claude Code harness

> Status: draft spec (v0.1). Grounded in research on the real CC schemas, the Rust
> config-format/round-trip landscape, and prior art, conducted 2026-06-29.

## 1. Thesis

The Claude Code "harness" — skills, commands, subagents, hooks, MCP/LSP servers,
`CLAUDE.md` rules, plugin & marketplace manifests, settings — has grown into a real
codebase, but it is maintained like a pile of loose files: heterogeneous formats
(`.md`+frontmatter, JSON), scattered across `~/.claude` and plugin dirs, with no type
system, no validation, no cross-artifact view, and no composition story.

`author` treats the harness as a **typed codebase you compile**. It imports the whole
harness into a single typed, validated **config surface**, lets a human reorganize and
de-duplicate it by hand, lints it against the documented schemas + best practices,
optionally composes artifacts into a publishable plugin/marketplace, and writes changes
back to disk with drift-aware, dry-runnable `apply`.

### Positioning

The adjacent tool, **rulesync**, makes a harness *portable* (one source → 30 assistants).
It is a cross-tool transformer: lowest-common-denominator surface, no validation-first
stance, no compose-to-plugin step, TypeScript.

> **`author` makes your harness *good*.** Quality, composition, maintenance — a
> Claude-Code-native object model, not a portability layer.

These are different axes; `author` could even consume rulesync's surface later rather
than compete with it.

## 2. The gap (prior art, mid-2026)

| Category | Examples | What they cover | What they miss |
|---|---|---|---|
| Plugin installers / registries | official `claude plugin`, ccpi, claude-plugins.dev | distribution/install | no import / lint / compose / write-back |
| Single-artifact linters | skillcheck, agent-skill-linter, claude-code-json-schema | validate ONE artifact type | not harness-wide, no surface, no apply |
| Scaffolders | skill-creator, plugin-dev | generate new artifacts | no import or round-trip |
| Cross-tool sync (closest) | **rulesync** (TS, 1.2k★), ruler | import → unified surface → write-back | LCD surface, validation not first-class, **no plugin/marketplace bundling** (their issue #329, "considering"), not CC-native |
| Rust | claude-list, claude-plugin-validate | read-only list / manifest validate | no surface, no compose, no apply |

Nobody unifies **import + lint + compose + bundle, harness-wide**, and nobody does the
compose-to-plugin step at all. The Rust corner is empty.

## 3. What the surface must hold (CC artifact catalog)

Two populations — this drives the whole design.

**Prose-dominant** (small typed frontmatter + large freeform markdown body):

| Artifact | On disk | Frontmatter (key fields) |
|---|---|---|
| Skill | `skills/<name>/SKILL.md` (+ companion `.md`, `scripts/`) | `name`, `description`*, `version`, `license` |
| Command (legacy) | `commands/<name>.md` | `description`, `argument-hint`, `allowed-tools`, `model`, `disable-model-invocation` |
| Agent / subagent | `agents/<name>.md` | `name`, `description`, `tools`, `model`, `color` |
| Memory / rules | `CLAUDE.md` (project & subdir), `~/.claude/CLAUDE.md` | none (pure body) |

**Structured JSON** (no prose; cross-references + `${VAR}` interpolation):

| Artifact | On disk | Shape |
|---|---|---|
| Plugin manifest | `.claude-plugin/plugin.json` | `name`*, `description`, `version`, `author{}`, `mcpServers{}`, `lspServers{}`, `skills[]`, `keywords[]`, … |
| Marketplace manifest | `.claude-plugin/marketplace.json` | `name`*, `owner{}`*, `plugins[]`* (each with polymorphic `source`: local-path / git-subdir / url / github) |
| MCP servers | `.mcp.json` (or `mcpServers` in plugin.json) | per-server: `command`+`args` \| `type:http`+`url`+`headers` \| `type:sse`+`url` |
| LSP servers | `.lsp.json` | per-language: `command`, `args`, `extensionToLanguage{}`, `startupTimeout` |
| Hooks | `hooks/hooks.json` (or `hooks` in settings.json) | event → groups → `{type:command, command, timeout}`; events: `PreToolUse`, `PostToolUse`, `SessionStart`, `Stop`, `UserPromptSubmit` |
| Settings | `~/.claude/settings.json`, `.claude/settings.local.json` | `env`, `statusLine`, `enabledPlugins`, `extraKnownMarketplaces`, `permissions`, `hooks`, misc flags |

\* required. Interpolation tokens: `${CLAUDE_PLUGIN_ROOT}`, `${ENV_VAR}` — the surface can
resolve & validate these.

Cross-references to model as first-class edges: plugin→skills, marketplace→plugin source,
hooks→scripts, mcp→env vars, settings→plugins/marketplaces.

## 4. The config surface (decision)

**Topology: structured-index + markdown-sidecars.** Per-artifact files in a
human-meaningful tree; prose bodies stay as real `.md`; a thin roll-up index powers
cross-artifact views and composition. Chosen over (a) inline-everything single file —
fragile on prose, coarse diffs, no rename detection — and (b) raw source-tree mirror —
poor at the consolidation/reorg workflows that are the whole point.

```
<workspace>/                  # e.g. .author/ in a project, or a managed ~/.claude clone
  author.toml                 # roll-up INDEX: every artifact, its kind, source path,
                              #   import-time content hash, and bundle membership
  author.lock                 # last-applied fingerprints (3-way drift state)
  skills/
    coordinate/
      meta.toml               # typed frontmatter, format-preserving (toml_edit)
      SKILL.md                # body — byte-faithful sidecar, untouched on apply
      PLAYBOOK.md             # companion body
  agents/<name>/{meta.toml, AGENT.md}
  commands/<name>/{meta.toml, COMMAND.md}
  memory/<scope>.md           # CLAUDE.md projections
  plugins/<name>.toml         # structured manifests → TOML (toml_edit)
  marketplaces/<name>.toml
  mcp/<name>.toml
  hooks/<name>.toml
  settings.toml
```

Why this wins (from the round-trip research):
- Bodies round-trip **byte-for-byte** (only the small structured header is ever rewritten).
- Per-file granularity → real `git mv`, small reviewable diffs, conflict-free parallel edits, per-file 3-way merge against drift.
- The index is a compact, greppable table (+ body content-hashes) → cross-artifact
  duplication/inconsistency views without loading every body.
- Surface granularity == source granularity → cheap, low-drift re-apply.

**Format for structured parts: TOML via `toml_edit`** (format-preserving, Cargo-grade,
strongest Rust schema tooling). KDL was the elegant runner-up (nicer raw strings) but has
stale schema tooling. Config languages, JSON, and YAML are disqualified for round-trip
(YAML especially: `serde_yaml` archived, `serde_yml` unsound per RUSTSEC-2025-0068).

## 5. Architecture

```
        import                lint/compose (human + tool)            apply
on-disk ───────► typed IR ───► validate · cross-view · bundle ───► on-disk
harness  parse   + index  emit   diagnostics      plugin/mkt   3-way-merge
         + hash  + lock                                         + dry-run
```

### IR

```
enum Artifact { Skill, Command, Agent, Memory,         // prose-dominant
                PluginManifest, MarketplaceManifest,   // structured
                McpServer, LspServer, Hook, Settings }
```

Each artifact carries: typed attributes, raw body (prose kinds), and **provenance**
(`source_path`, `import_hash`) — provenance is what makes drift detection and write-back
possible and is the real engineering cost, independent of format.

### Drift / apply engine (the moat)

Three states, never two: **desired** (edited surface) / **last-applied fingerprint**
(`author.lock`) / **real on-disk**. Rules borrowed from chezmoi/terraform/kustomize:

1. Compare semantically (parsed nodes), not on whitespace.
2. On-disk ≠ last-applied ⇒ **drift**: surface to the user, offer
   `[diff · overwrite · skip · re-add]`, never silently overwrite.
3. Format-preserving write: mutate only changed nodes, reprint the rest verbatim
   (`toml_edit` for headers/manifests; sidecar `.md` bodies are copied byte-faithful).
4. Apply a minimal field-scoped patch onto the current on-disk file, not a stale full copy.
5. Idempotent: a converged `apply` is a no-op. `apply` always offers `--dry-run`/diff first.
6. Bidirectional `re-add` (on-disk → surface) is a first-class direction, not an afterthought.

### Lint engine

```
trait Rule { fn check(&self, a: &Artifact, ws: &Workspace) -> Vec<Diagnostic>; }
```

Per-artifact rules (schema): frontmatter validity, required fields, manifest field rules,
hook event/command shape, polymorphic `source` correctness, `${VAR}`/path resolution,
dangling companion-file refs.

Cross-artifact rules (the differentiator): duplicated description fragments, skill
descriptions missing trigger/anti-trigger, token-budget overruns, orphaned artifacts
(referenced by no plugin), MCP env vars undeclared in settings, name collisions.

Diagnostics rendered with `miette`/`ariadne` (source spans, fix-its) — also the most
fun Rust-learning surface.

## 6. CLI verbs

| Verb | Does |
|---|---|
| `author init` | create a workspace targeting a project's `.claude/` or `~/.claude` |
| `author import` | scan harness → typed surface + `author.lock` (idempotent) |
| `author check` | lint: per-artifact + cross-artifact diagnostics |
| `author diff` | dry-run: what `apply` would change + any on-disk drift |
| `author apply` | write surface → harness (3-way merge, drift-aware) |
| `author re-add` | pull on-disk drift back into the surface |
| `author bundle <name>` | compose selected artifacts → publishable plugin + `marketplace.json` |

## 7. Rust stack

- CLI: `clap` (derive)
- Structured round-trip: `toml_edit` (format-preserving) + `serde`/`toml` for read models
- Frontmatter: `gray_matter` (single-maintainer — simple enough to vendor if it stalls)
- Markdown body analysis: `comrak` or `pulldown-cmark`
- Walk: `ignore`/`walkdir`
- Validation: `garde` and/or `schemars` + JSON-Schema for the JSON artifacts
- Diagnostics UX: `miette` (+ `thiserror`)
- Tests: `insta` snapshots (golden import/apply fixtures), `assert_cmd` for CLI

## 8. MVP slices (ship in order)

1. **`import` + `check` for Skills only.** Proves the IR, the sidecar topology, the lint
   engine, and the diagnostics UX against your real `coordinate` skill. Immediately useful.
2. **`diff` + `apply` for Skills** — the three-state drift engine end-to-end on one kind.
   This is the hard, differentiating core; prove it small.
3. **Widen the IR** to the structured artifacts (settings, hooks, plugin.json) — exercises
   `toml_edit` round-trip and cross-reference rules.
4. **`bundle`** — compose into a valid plugin + `marketplace.json`. The step no competitor does.

## 9. Open questions

- **Workspace location & scope:** per-project `.author/` vs a single managed mirror of
  `~/.claude` vs both? (Skills/settings are user-global; plugins/marketplace are repo-shipped.)
- **Source frontmatter is YAML; surface headers are TOML.** Read YAML with `serde-saphyr`;
  on write-back, do we re-emit YAML (normalizing, since no comment-preserving YAML editor
  exists in Rust) or only patch changed fields? Leaning: patch-only to minimize drift.
- **Is the surface the source of truth, or a lens?** MVP treats it as source-of-truth with
  `re-add` for drift; revisit if direct-harness-editing turns out to be the common path.
- **Scope of "best practice" rules** — start with a small high-confidence set; grow.
