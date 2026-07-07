# temper

## Identity

- **Project:** `temper` — a typed maintenance surface for the Claude Code
  harness. Import the whole harness (skills, commands, agents, hooks, MCP/LSP,
  `CLAUDE.md` rules, plugin & marketplace manifests, settings) into one typed,
  validated config surface; lint it against the documented schemas + best
  practices; compose into publishable bundles; write back with drift-aware
  `apply`. Positioning: `rulesync` makes a harness portable; `temper` makes it
  *good*.

## Source of truth

**Read `specs/intent.md` first**, then the model (`specs/model/`) — the source
of truth for intent and contract (`specs/process/spec-system.md` says how the
corpus works; history lives in `specs/decisions/` and git tags, never in the
body text). The corpus is evergreen with a stable kernel: plan reconciles code
against it every tick; build executes one entry at a time against its cited
spec section. Intent is human-authored, never written by a phase.

## The two harnesses — read this

This repo carries **two distinct harnesses**; do not conflate them:

1. **`.flume/`** — the flume build pipeline (plan → build, gated commits). Human
   territory; the `build` phase never edits it.
2. **`.claude/` + this `CLAUDE.md`** — the Claude Code harness. It is the *product
   domain*: the exact artifact kinds `temper` is built to project, and the
   environment you (and the flume build agents) run inside. It is hand-curated
   to an exemplary standard on purpose — when you touch it, hold it to the bar
   `temper` lints. Changes flow through human `chore(harness):` commits, never
   `build:` ticks.

The **recursive dogfood** — temper gating its host repo's own harness
(`.temper/` workspace, `temper.toml` assembly, self-check gate, session-start
hook) — is **deactivated** (John's ruling, 2026-07-04: cumbersome; it cost two
self-gate reverts, a per-wave deactivation ceremony, and a producer fork).
Validation lives in `tests/` fixtures. A real dogfood returns when the
SDK-primary authoring path (`harness.ts` → emit) is the product's own front
door.

## Tech stack

- **Rust**, edition 2024, toolchain 1.96+. `cargo` is the build/test/lint driver.
- Key crates (sanctioned set; see Cargo.toml): `clap`, `miette` + `thiserror`, `serde`,
  `toml_edit` (format-preserving round-trip keystone), `gray_matter`, `walkdir`,
  `ignore` (gitignore-honoring discovery walks — `specs/model/pipeline.md`),
  `sha2`, `regex` (the charset mechanics behind
  `allowed_chars` — no author-facing `pattern` clause), `insta` (snapshot tests).
- **flume** control plane (`.flume/`) runs on Node via `@dtmd/flume` (pnpm).

Conventions live in `.claude/rules/*.md`, auto-loaded by Claude Code at launch
(no import needed): `rust.md` is `paths:`-scoped to Rust files; `collaboration.md`
loads unconditionally.

## Workflow: flume drives the build

Two autonomous phases share one dispatcher. Chain config in `.flume/chain.ts`;
prompts in `.flume/prompts/{plan,build}.md`; conventions in `.flume/PROTOCOL.md`.
Plan reconciles `.flume/plan/pending.json` against `specs/`; build ships entries to the
trunk one validated commit at a time. State is on disk; each tick is a fresh
`claude -p`. Loops are autonomous — no slash command invokes them.

## Common commands

- `cargo build` — compile.
- `cargo test` — tests (prefer `insta` snapshots for parse/lint output).
- `cargo clippy --all-targets -- -D warnings` — the lint bar (afterMerge gate).
- `cargo fmt --all --check` — formatting (afterCommit gate); `cargo fmt --all` to fix.
- `pnpm exec flume status` — baton state.
- `pnpm exec flume render plan` — preview the next plan prompt (no agent call).
- `pnpm exec flume tick` / `loop` — run the pipeline.

## Quality standard

Engineering: **correct, clear, well-tested.** This tool is I/O-bound over tiny
files — there is no performance pressure, so never trade clarity for a micro-opt.
Gates (`cargo fmt`, `cargo clippy -D warnings`, `cargo test`) validate every tick.

## Non-negotiables

- Build commits per pending entry to the trunk after green validation.
- NEVER force-push, amend pushed commits, or `--no-verify`.
- NEVER modify files when asked to investigate — investigate and report.
- Search the codebase before implementing — don't assume not implemented.
- **Never silently fill a spec/product gap — challenge it** (see the
  auto-loaded `collaboration` rule).

## Session ledger

`docs/ledger.md` is the cross-session parking lot — parked threads, queued
human halves, standing flume-loop discipline. It is deliberately **not
imported**: read it at session open to pick the session's one focus, and
again only when parked work resumes. Interactive sessions maintain it; no
autonomous phase reads or writes it.
