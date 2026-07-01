# The config surface — compose, import, project, drift

The surface is `temper`'s **composition write surface**: the typed thing the
author *composes* the harness in, the contract engine validates, and `temper`
**projects** into the project. It is the source of truth — `.claude/` and `specs/`
are projected from it, not the reverse (`00-intent.md` law 7; Decision below).
`import` is the on-ramp (an existing harness → surface), `re-add` reconciles direct
edits, and the write direction (`apply`) projects the surface back out. Built and
proven for skills in slice 1; the shape generalizes to every artifact kind and
every landscape (`30-landscapes.md`).

## Topology: structured-index + markdown sidecars

The harness is ~half prose-dominant (skills, agents, `CLAUDE.md` — small typed
header + large body) and ~half structured JSON (manifests, hooks, settings). No
single inlined format serves both, so the surface is a **workspace tree**, not
one file:

- **Prose bodies stay as real `.md`**, byte-faithful and `git mv`-able.
- **Structured headers** are written format-preserving (`toml_edit`).
- **A roll-up index** (`author.toml`) lists every artifact with hashes, powering
  cross-artifact views and contract validation without loading every body.

```
<workspace>/
  author.toml                 # roll-up index (name, source_path, import_hash, body_hash)
  skills/<name>/
    meta.toml                 # typed header + [provenance]
    SKILL.md                  # body, byte-faithful
    <companions…>             # copied byte-for-byte
```

## The IR

One typed value per artifact kind, behind an `Artifact` sum type. Each carries
its typed fields, a byte-faithful body where it has one, an `extra` catch-all
that **preserves unknown frontmatter keys verbatim** (never dropped), companion
paths, and provenance. Skills are modelled and shipped (`src/skill.rs`); the
`disable-model-invocation` field is load-bearing (Pocock's invocation axis) and
must be in the IR.

The IR generalizes to a per-kind **extractor** (`30-landscapes.md`): parse a unit
into the structured features the contract engine validates. For a skill that is
frontmatter + body; for a spec it is headings, bindings, and declared model
elements. Extraction is the soundness boundary — it surfaces only
deterministically-decidable features, never inferred meaning.

## Artifact kinds & contract selection

The kind *system* — the extraction algebra and the built-in/custom split — is
`15-kinds.md`; these are the **built-in harness kinds** the surface ships and how
`check` dispatches them. Each artifact kind has an extractor and a built-in contract. Slice 1 shipped
**skill**; the next kind is **rule** (`.claude/rules/*.md`): frontmatter `paths`
(optional — the real Claude Code scoping key) plus a byte-faithful body. Its
decidable clauses forbid the Cursor keys Claude Code ignores (`description`,
`globs`, `alwaysApply`) — the exact mistake that motivated the project (a rule
authored with `.mdc` frontmatter loads nothing). `import` scans every built-in
kind (`skills/*/SKILL.md`, `.claude/rules/*.md`) plus every custom kind the active
`temper.toml` declares (`40-composition.md`); `check` dispatches each artifact to
the effective contract for its kind. This is the path to self-hosting:
`temper`'s own `.claude/` is rules, so once the rule kind exists, `temper check`
can run on its own house.

### Decision: contract selection is by artifact kind

**Chosen:** `check` maps each artifact to the built-in contract for its kind
(skill → `contracts/skill.anthropic.toml`, rule → `contracts/rule.toml`),
embedded as defaults. **Rejected (for now):** a single active contract, or a CLI
flag to pick one — neither generalizes to a mixed harness (skills *and* rules in
one import). A per-workspace override (a `contracts/` dir convention or an
`author.toml` field) is a later extension, not the default. (Resolves
`(contract-selection)`.)

## Provenance and round-trip discipline (law 5)

- `provenance = { source_path, import_hash }`; `import_hash` is the SHA-256 of
  the original source bytes — the drift anchor, computed at import so the lock is
  complete before write-back exists.
- Bodies and companions are **copied, never re-rendered.** Only the structured
  header is written, via `toml_edit` (preserves comments/order/whitespace). A
  lossy serialize-from-scratch on anything a human edits is forbidden.
- `import` is **idempotent**: re-importing an unchanged harness yields an
  identical workspace (asserted by snapshot).
- The author may **compose** prose in the surface — it is the write surface (law
  7) — but `temper` never **synthesizes** it. Prose is stored and projected
  byte-faithfully whether it arrived by `import` or by authoring; the invariant is
  *authored, never synthesized*, not *structure only*. A composed prose body is as
  byte-faithful on projection as an imported one.

## Drift / apply — three states, never two (the hard core)

Write-back (`apply`) is the differentiating engine and the thing "fearless
refactoring" (law 6) rests on. It tracks three states: **desired** (the edited
surface), the **last-applied fingerprint**, and **real on-disk** — so it can
distinguish "the human edited the surface" from "the world drifted" and merge
rather than clobber. Drift surfaces a choice (diff · overwrite · skip · re-add);
`apply` is idempotent and dry-runnable. `re-add` (on-disk → surface) is a first-
class direction, because humans also edit the harness directly.

### Decision: the surface is the source of truth

**Chosen:** the composition surface is canonical; `.claude/` + `specs/` are a
**projection** of it (`apply`), and direct on-disk edits are reconciled back with
`re-add` (drift, above). **Rejected:** the surface as a read-only *lens* over
canonical on-disk files. The lens framing contradicts law 7 — you cannot *compose*
a harness you only mirror — and strands fearless refactoring (law 6), which needs a
surface the author authors. `re-add` keeps direct-on-disk editing first-class
without demoting the surface. (Resolves `(surface-authority)`.)

### Decision: the workspace is per-project

**Chosen:** the surface targets a **per-project** harness — the `.claude/` and
co-located artifacts of one project, located by the explicit path `import` / `check`
already take. **Rejected (for now):** managing a mirror of the global `~/.claude`,
or both at once. The per-project harness is the unit a contract gates and a session
loads; the global config is a later extension the same engine handles as another
landscape root (`30-landscapes.md`), not a redesign. (Resolves `(workspace-scope)`.)

### Decision: write-back patches changed fields, never re-emits

**Chosen:** `apply` **patches only the fields that changed**, in place — TOML
headers via `toml_edit`, YAML frontmatter by surgical field patch — leaving every
untouched byte (comments, key order, whitespace) exactly as the human wrote it.
**Rejected:** re-emitting a header by serializing it from scratch. No comment-
preserving YAML editor exists in Rust, so a full re-emit would normalize the
frontmatter and clobber human edits — the lossy round-trip law 5 forbids on
anything a human authors. (Resolves `(yaml-writeback)`.)

## CLI surface

- `temper import <harness-path> [--into <workspace>]` — scan → surface + lock.
- `temper check [<workspace>]` — the gate: validate **conformance** (each artifact
  against the contract for its kind, `10-contracts.md`) and **admissibility** (each
  contract against the definition); exit non-zero on a `required`-clause violation
  (`--deny-advisories` to also block on advisory).
- `temper diff` / `apply` / `re-add` — the drift engine (future).
- `temper bundle` — compose into a publishable plugin + `marketplace.json`
  (future; the publish verb — `50-distribution.md`).
- `temper install` — project the gate's wiring (`SessionStart` hook, CI job, schema
  modeline) into the harness, drift-synced (future; `50-distribution.md`).
- `temper schema [--kind <kind>]` — emit the active contract as an editor JSON
  Schema for keystroke validation (future; `50-distribution.md`).

Logic lives in the library; `main` is a thin `clap` dispatch that maps results to
an exit code (`.claude/rules/rust.md`).
