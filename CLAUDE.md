# author

## Identity

- **Project:** `author` — a typed maintenance surface for the Claude Code
  harness. Import the whole harness (skills, commands, agents, hooks, MCP/LSP,
  `CLAUDE.md` rules, plugin & marketplace manifests, settings) into one typed,
  validated config surface; lint it against the documented schemas + best
  practices; compose into publishable bundles; write back with drift-aware
  `apply`. Positioning: `rulesync` makes a harness portable; `author` makes it
  *good*.

## Source of truth

**Read `SPEC.md` (root) first** — the canonical design. `spec/RELEASE-*.md` are
the concrete, testable ship targets; the newest is the active plan target,
earlier ones frozen. `docs/INTENT.md` holds the longer-range invariants. Plan
derives the work breakdown against whatever changed in `spec/`; build executes
one entry at a time against the cited section.

## The recursive dogfood — read this

This repo carries **two distinct harnesses**; do not conflate them:

1. **`.flume/`** — the flume build pipeline (plan → build, gated commits). Human
   territory; the `build` phase never edits it.
2. **`.claude/` + this `CLAUDE.md`** — the Claude Code harness. It is the *product
   domain*: the exact artifact kinds `author` is built to project. It is also
   `author`'s first dogfood fixture and the environment you (and the flume build
   agents) run inside. It is hand-curated to an exemplary standard on purpose —
   when you touch it, hold it to the bar `author` will one day lint. Changes flow
   through human `chore(harness):` commits, never `build:` ticks.

## Tech stack

- **Rust**, edition 2024, toolchain 1.96+. `cargo` is the build/test/lint driver.
- Key crates (sanctioned set, SPEC §7): `clap`, `miette` + `thiserror`, `serde`,
  `toml_edit` (format-preserving round-trip keystone), `gray_matter`, `walkdir`,
  `sha2`, `insta` (snapshot tests).
- **flume** control plane (`.flume/`) runs on Node via `@dtmd/flume` (pnpm).

Conventions live in `.claude/rules/*.md`, auto-loaded by Claude Code at launch
(no import needed): `rust.md` is `paths:`-scoped to Rust files; `collaboration.md`
loads unconditionally.

## Workflow: flume drives the build

Two autonomous phases share one dispatcher. Chain config in `.flume/chain.ts`;
prompts in `.flume/prompts/{plan,build}.md`; conventions in `.flume/PROTOCOL.md`.
Plan derives `.flume/plan/pending.json` from `spec/`; build ships entries to the
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
