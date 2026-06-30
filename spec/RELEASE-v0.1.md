# RELEASE v0.1 — Skill import + check

The first vertical slice (SPEC §8, slice 1). Goal: prove the typed IR, the
sidecar topology, the lint engine, and the diagnostics UX against a **real**
skill — `~/.claude/skills/coordinate`. Scope is deliberately one artifact kind
(Skill). No `apply`, no `bundle`, no structured-JSON artifacts yet.

This file is the plan target. `plan` breaks it into pending entries; `build`
ships them. Earlier-frozen release lines: none.

## Surface

Two subcommands (the stubs in `src/main.rs` become real):

- `author import <harness-path> [--into <workspace>]` — scan `<harness-path>`
  for skills and write the typed config surface.
- `author check [<workspace>]` — parse the surface and emit lint diagnostics.

`<harness-path>` defaults to nothing required for slice 1 — it is an explicit
argument, sidestepping `(workspace-scope)`. `--into` defaults to `./.author`.

## The IR (slice 1 subset)

A `Skill` models `~/.claude/skills/<name>/SKILL.md` (SPEC §3):

- `name: String` — from frontmatter; must equal the directory name.
- `description: String` — frontmatter; the trigger text.
- `version: Option<String>`, `license: Option<String>` — optional frontmatter.
- `body: String` — the markdown after frontmatter, byte-faithful.
- `companions: Vec<PathBuf>` — sibling files (e.g. `PLAYBOOK.md`, `scripts/**`).
- `provenance: Provenance { source_path: PathBuf, import_hash: String }` —
  `import_hash` is the SHA-256 of the original `SKILL.md` bytes (drives future
  drift detection; computed now so the lock is complete).

Frontmatter is YAML (read-only in slice 1 — no write-back). Parse with
`gray_matter`. Unknown frontmatter keys are preserved verbatim in the surface,
never dropped.

## Import behavior

- Walk `<harness-path>/skills/*/SKILL.md` (and a bare `<harness-path>` that is
  itself a skill dir). Skip non-skill files.
- For each skill, write to `<workspace>/skills/<name>/`:
  - `meta.toml` — typed frontmatter via `toml_edit` (format-preserving writer),
    plus a `[provenance]` table with `source_path` and `import_hash`.
  - `SKILL.md` body and every companion file, copied byte-for-byte.
- Write `<workspace>/author.toml` — the roll-up index: one `[[skill]]` entry per
  imported skill with `name`, `source_path`, `import_hash`, and a `body_hash`.
- `import` is idempotent: re-importing an unchanged harness yields an identical
  workspace (no spurious diffs).

## Check behavior (the lint engine)

`check` parses the workspace IR and runs rules, each producing zero or more
`Diagnostic { severity, rule, artifact, message, span? }`, rendered with
`miette`. Exit non-zero if any `error`-severity diagnostic fires.

Slice-1 rules. These encode the documented Claude Code skill best practices
(Anthropic's hard mechanics + Matt Pocock's precision heuristics — see Sources);
all are mechanical string/structure checks, no judgment calls:

| Rule id | Severity | Asserts | Source |
| ------- | -------- | ------- | ------ |
| `skill.frontmatter-valid` | error | frontmatter parses; required `name` + `description` present and non-empty | Anthropic spec |
| `skill.name-format` | error | `name` ≤ 64 chars, `[a-z0-9-]` only, not a reserved word (`anthropic`, `claude`) | Anthropic spec |
| `skill.name-matches-dir` | error | `name` equals the containing directory name | CC discovery |
| `skill.description-length` | error | `description` ≤ 1024 chars | Anthropic spec |
| `skill.description-third-person` | warn | description is third person — no `I`/`you can…` (it is injected into the system prompt) | Anthropic |
| `skill.description-has-trigger` | warn | description states *when to use* (a trigger/context), not only what it does | Anthropic + Pocock |
| `skill.description-has-anti-trigger` | warn | description states when **not** to use it (branch precision) | Pocock |
| `skill.body-length` | warn | SKILL.md body < 500 lines (else split via progressive disclosure) | Anthropic |
| `skill.companion-refs-resolve` | error | every companion path referenced in the body exists on disk | — |
| `skill.refs-one-level-deep` | warn | referenced files sit ≤ 1 level from SKILL.md (deeper risks partial `head` reads) | Anthropic |

Cross-artifact rules (the differentiator) are out of scope for slice 1 but the
`Rule` trait must take the whole workspace, not a single artifact, so they slot
in later without a signature change.

**Deferred to a later release** (these need heuristics/judgment, not a slice-1
mechanical check): gerund-naming preference; **no-op detection** (a line the
model already obeys by default — Pocock); **leading-word / Leitwort** reuse;
table-of-contents required for reference files > 100 lines; the pushy-recall vs
prune-precision reconciliation in descriptions. Tracked as open design, not
silently dropped.

### Sources (the lint rules trace to these)

- Anthropic — Skill authoring best practices: <https://platform.claude.com/docs/en/agents-and-tools/agent-skills/best-practices>
- Anthropic — `skill-creator`: <https://github.com/anthropics/skills/blob/main/skills/skill-creator/SKILL.md>
- Matt Pocock — `writing-great-skills`: <https://github.com/mattpocock/skills> (`skills/productivity/writing-great-skills/SKILL.md`)

## Tests / acceptance

- `cargo test` green; `cargo clippy --all-targets -- -D warnings` clean.
- An `insta` snapshot of `import` over a fixture skill (a trimmed copy of
  `coordinate`) — stable across re-runs.
- An `insta` snapshot of `check` diagnostics over fixtures that deliberately
  trip each rule.
- Acceptance for the slice: `author import <fixture> --into <tmp>` then
  `author check <tmp>` reproduces the expected diagnostic set, and re-running
  `import` produces no diff.

## Non-goals (explicitly deferred)

- `apply` / write-back (gated on `(yaml-writeback)`).
- Structured-JSON artifacts (settings, hooks, plugin/marketplace manifests).
- `bundle` / marketplace emission.
- Global `~/.claude` auto-discovery (gated on `(workspace-scope)`).
