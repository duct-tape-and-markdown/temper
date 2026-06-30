# The config surface — import, IR, round-trip, drift

The surface is `temper`'s in-memory and on-disk projection of a harness: the
typed thing the contract engine validates and the human reshapes. Built and
proven for skills in slice 1; the shape generalizes to every artifact kind.

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

## Provenance and round-trip discipline (law 5)

- `provenance = { source_path, import_hash }`; `import_hash` is the SHA-256 of
  the original source bytes — the drift anchor, computed at import so the lock is
  complete before write-back exists.
- Bodies and companions are **copied, never re-rendered.** Only the structured
  header is written, via `toml_edit` (preserves comments/order/whitespace). A
  lossy serialize-from-scratch on anything a human edits is forbidden.
- `import` is **idempotent**: re-importing an unchanged harness yields an
  identical workspace (asserted by snapshot).

## Drift / apply — three states, never two (the hard core)

Write-back (`apply`) is the differentiating engine and the thing "fearless
refactoring" (law 6) rests on. It tracks three states: **desired** (the edited
surface), the **last-applied fingerprint**, and **real on-disk** — so it can
distinguish "the human edited the surface" from "the world drifted" and merge
rather than clobber. Drift surfaces a choice (diff · overwrite · skip · re-add);
`apply` is idempotent and dry-runnable. `re-add` (on-disk → surface) is a first-
class direction, because humans also edit the harness directly. (Open: the
`yaml-writeback` and `workspace-scope` forks in `.flume/plan/open-questions.md`.)

## CLI surface

- `temper import <harness-path> [--into <workspace>]` — scan → surface + lock.
- `temper check [<workspace>]` — validate against the active contract; exit
  non-zero on a `required`-clause violation (`--deny-advisories` to also block on
  advisory). The gate.
- `temper diff` / `apply` / `re-add` — the drift engine (future).
- `temper bundle` — compose into a publishable plugin + `marketplace.json`
  (future; the composition verb).

Logic lives in the library; `main` is a thin `clap` dispatch that maps results to
an exit code (`.claude/rules/rust.md`).
