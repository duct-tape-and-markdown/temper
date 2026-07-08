# Flume Protocol — author project conventions

Runtime mechanics (baton, gates, handoff, pending schema) live in `chain.ts` and
the `@dtmd/flume` runtime. This file holds the project-side conventions the chain
config doesn't encode.

## The chain

`specs/` (evergreen) → `.flume/plan/` → `src/` (+ `tests/`, `Cargo.toml`, docs) → git log

The `specs/` corpus is the evergreen source of truth (`specs/process/spec-system.md`),
authored only in interactive sessions under explicit direction, never by an
autonomous phase. It is **not** a release line: plan reconciles code against the
living corpus, one job per tick — there is no frozen ship target. `specs/intent.md`
is the north star; `specs/decisions/` is history, outside the phases' read path.

| Layer | Author | Phase | Commit prefix  |
| ----- | ------ | ----- | -------------- |
| spec  | human  | —     | (any)          |
| plan  | plan   | plan  | `plan:`        |
| code  | build  | build | `build:`       |

Harness-authored commits (post-merge ship) use `chore(flume):`.

## Two harnesses live here — don't confuse them

This repo carries **two** harnesses with different owners:

1. **The flume harness** (`.flume/`) — the build pipeline. Authored by humans;
   never edited by the `build` phase, with two deliberate slits:
   `.flume/friction/**` (agent→human harness feedback, humans drain) and
   `.flume/refactor/**` (agent→plan structural-debt captures, plan drains
   into pending entries). Both phases may file into either; see each
   directory's README.
2. **The Claude Code harness** (`.claude/`, `CLAUDE.md`) — the *product domain*:
   the very artifacts `temper` is built to project, and the environment the
   build agents themselves run inside, so it is hand-curated to an exemplary
   standard. The `build` phase never edits it; changes flow through human
   `chore(harness):` commits. The recursive dogfood is live: `CLAUDE.md` and
   the `.claude/` member files are projections of `.temper/` modules — edit
   the owning module and re-run `temper emit`; a direct edit is drift the
   guard blocks.

`writablePaths` in `chain.ts` enforces both exclusions mechanically.

## Rust gate model

Gate placement follows CHAIN-AUTHORING §2, adapted for cargo (compilation is the
expensive step):

- **afterCommit:** `cargo fmt --all --check` only — no compile, safe to run
  N-wide under fanout.
- **afterMerge:** `cargo clippy --all-targets -- -D warnings` and `cargo test` —
  serial on the trunk, no N-wide contention; a failure reverts only the offending
  entry.

No `setupWorktree`: cargo shares its registry cache via `~/.cargo`; only `target/`
is per-worktree (the cold compile kept off the parallel path on purpose).

## Plan dispatch + continuation marker

Plan is iteratively prompted: one tick = one job, dispatched off its stateful
records (inbox → spec delta → ship audit → residue → quiet; the job table
lives in `prompts/plan.md`), with per-input cursors in `state.md`
(`Spec derived through:` / `Audited through:` / `Residue swept through:`).
`state.md` ends with `Plan continues: yes — <reason>` while a later input is
live and the harness re-wakes plan; `Plan continues: no` hands to build or
hibernates. The regex `^Plan continues:\s*yes\b` (in `chain.ts`) is
load-bearing, and the marker's honesty is gated (`planHonestyGate`): a `no`
with an undrained inbox or a trailing spec cursor reverts the tick.

## Disk vs git log

To answer "did X ship?" or "is gate Y green?", read the disk artifact
(`pending.json`, the `src/` file, `cargo` output). Never grep commit messages.
Git log is orientation, not authority.

## Non-negotiables

- Build commits per entry reach the trunk after green validation — **via the
  dispatcher's merge, never directly**. The agent commits only on its own
  worktree branch: no rebasing onto `main`, no pushing a trunk ref, no
  committing from the root checkout. A `main` that moved mid-tick is the
  dispatcher's merge problem, not the agent's — a commit made directly on the
  trunk bypasses the afterMerge gates and is invisible to ship bookkeeping.
- NEVER force-push, amend pushed commits, or `--no-verify`.
- NEVER modify files when asked to investigate — investigate and report.
- Search the codebase before implementing — don't assume not implemented.
- Never silently fill a spec gap — surface it as an open question.
