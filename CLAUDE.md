<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# temper

## Identity

- **Project:** `temper` — a type system for the documents that program
  agents, the Claude Code harness first (skills, rules, agents, hooks,
  MCP/LSP, `CLAUDE.md` memory, settings, plugin & marketplace manifests).
  The harness is authored as a typed program (`@dtmd/temper`), compiled by a
  deterministic `emit` with drift routed to the authored source, and gated by
  `check` against a declared contract; a kind may type its body's layout, so
  document content sits under the same gate. Seven verbs ship: `check`,
  `explain`, `emit`, `schema`, `guard`, `install`, `bundle`. Positioning:
  `rulesync` makes a harness portable; marketplaces distribute; `temper`
  makes it *correct* — downstream of both.

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

The **recursive dogfood is live**: this harness is represented at `.temper/`
(member modules with module-adjacent prose documents, riding the workspace
SDK via `file:../sdk`; `harness.ts`; the committed `lock.toml`), the
`SessionStart` reporter and the guard are wired in `settings.json`, and the
gate is `temper check .`. `CLAUDE.md` and the `.claude/` member files
are **projections**: edit the owning `.temper/` module or document and
re-run `temper emit` — a direct edit is drift. A fresh clone builds the SDK
once (`pnpm -C sdk install && pnpm -C sdk build`) and installs the
workspace dep (`npm -C .temper install`) before `emit` runs. Friction the
dogfood surfaces routes to `.flume/inbox.md`, never into hand-patched
`src/`.

Maintenance of either harness prefers **subtraction before addition**, and a
surface states the rule, never the incident that taught it — these files are
written to be read by you, and originating context is git's to keep.

## Tech stack

- **Rust**, edition 2024, toolchain 1.96+. `cargo` is the build/test/lint driver.
- Key crates (sanctioned set; see Cargo.toml): `clap`, `miette` + `thiserror`, `serde`,
  `toml_edit` (format-preserving round-trip keystone), `gray_matter`, `walkdir`,
  `ignore` (gitignore-honoring discovery walks — `specs/model/pipeline.md`),
  `globset` (the one glob engine — already inside `ignore`; never hand-roll
  matching), `sha2`, `regex` (the charset mechanics behind
  `allowed_chars` — no author-facing `pattern` clause), `insta` (snapshot tests),
  `tempfile` (dev: test temp dirs).
- **flume** control plane (`.flume/`) runs on Node via `@dtmd/flume` (pnpm).

Conventions live in `.claude/rules/*.md`, auto-loaded by Claude Code at launch
(no import needed): `rust.md` and `sdk.md` are `paths:`-scoped to their trees;
`collaboration.md` loads unconditionally.

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
- `pnpm --dir sdk test` — SDK gate: strict `tsc` + `node --test` (afterMerge gate;
  run it whenever a change touches `sdk/**`).

Manual/periodic **checks** (no pipeline enforcement — a signal for a human to
read, not a bar the pipeline holds; nothing reverts on either):

- `cargo machete --with-metadata` — unused-dependency scan.
- `cargo llvm-cov --summary-only` — coverage, no threshold enforced (2026-07-08's
  pass found `install.rs`'s append-to-an-existing-hook-entry path untested end
  to end).
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
